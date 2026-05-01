use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde::Serialize;

use super::export::is_audit_export_root;
use super::model::UxAuditSuite;
use crate::shell::uxlab::{LabCliSelection, LabRunError, ViewportClass, ViewportSize};

const WTD_WORKSPACE_ROOT: &str = "target/ux_audit_lab/wtd_workspaces";
const TARGET_TAB: &str = "design";
const TARGET_PANE: &str = "oxide-uxlab-design-pane";
const WAIT_TIMEOUT: Duration = Duration::from_secs(20);
const POLL_INTERVAL: Duration = Duration::from_millis(200);
const STABLE_WINDOW: Duration = Duration::from_millis(800);

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct WtdDesignPlan {
    pub workspace_name: String,
    pub workspace_path: String,
    pub target: String,
    pub executable: String,
    pub cwd: String,
    pub args: Vec<String>,
    pub viewport: String,
    pub width: u16,
    pub height: u16,
    pub suite_id: String,
    pub scenario_id: Option<String>,
    pub firehorse_scenario_id: Option<String>,
    pub visible_needle: String,
    pub open_command: String,
    pub capture_text_command: String,
    pub capture_vt_command: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct WtdOpenResult {
    pub plan: WtdDesignPlan,
    pub status: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct WtdCaptureResult {
    pub plan: WtdDesignPlan,
    pub root: String,
    pub files_written: Vec<String>,
    pub status: &'static str,
}

pub fn open_design(
    selection: &LabCliSelection,
    suite: &UxAuditSuite,
) -> Result<WtdOpenResult, LabRunError> {
    let plan = build_plan(selection, suite)?;
    write_workspace_file(&plan)?;
    run_wtd_checked(&["open", "--file", &plan.workspace_path, "--recreate"])?;
    wait_for_visible_and_stable(&plan)?;
    Ok(WtdOpenResult {
        plan,
        status: "opened",
    })
}

pub fn capture_design(
    selection: &LabCliSelection,
    suite: &UxAuditSuite,
    root: &Path,
) -> Result<WtdCaptureResult, LabRunError> {
    if !is_audit_export_root(root) {
        return Err(LabRunError::Io(format!(
            "refusing WTD capture outside docs/firehorse_mockups/ux_audit_lab or target/ux_audit_lab: {}",
            root.display()
        )));
    }

    let run_root = root.join(capture_run_directory_name());
    fs::create_dir_all(&run_root).map_err(|error| LabRunError::Io(error.to_string()))?;
    let plan = build_plan_with_workspace_root(selection, suite, &run_root)?;
    write_workspace_file(&plan)?;
    run_wtd_checked(&["open", "--file", &plan.workspace_path, "--recreate"])?;
    let text = wait_for_visible_and_stable(&plan)?;
    let vt = capture_vt(&plan.target)?;

    let stem = capture_stem(&plan);
    let text_path = run_root.join(format!("{stem}.txt"));
    let vt_path = run_root.join(format!("{stem}.vt"));
    let summary_path = run_root.join("wtd_capture.json");

    write_new(&text_path, text.as_bytes())?;
    write_new(&vt_path, &vt)?;

    let result = WtdCaptureResult {
        plan,
        root: run_root.to_string_lossy().to_string(),
        files_written: vec![
            text_path.to_string_lossy().to_string(),
            vt_path.to_string_lossy().to_string(),
            summary_path.to_string_lossy().to_string(),
        ],
        status: "captured",
    };
    let summary =
        serde_json::to_vec_pretty(&result).map_err(|error| LabRunError::Io(error.to_string()))?;
    write_new(&summary_path, &summary)?;
    append_newline(&summary_path)?;
    Ok(result)
}

pub fn build_plan(
    selection: &LabCliSelection,
    suite: &UxAuditSuite,
) -> Result<WtdDesignPlan, LabRunError> {
    build_plan_with_workspace_root(selection, suite, Path::new(WTD_WORKSPACE_ROOT))
}

fn build_plan_with_workspace_root(
    selection: &LabCliSelection,
    suite: &UxAuditSuite,
    workspace_root: &Path,
) -> Result<WtdDesignPlan, LabRunError> {
    let suite_id = selection
        .suite
        .clone()
        .unwrap_or_else(|| suite.id.to_string());
    if suite_id != suite.id {
        return Err(LabRunError::UnknownSuite {
            suite: suite_id,
            available: vec![suite.id.to_string()],
        });
    }

    let selected_scenario = selection
        .scenario
        .as_deref()
        .map(|id| {
            suite
                .find_scenario(id)
                .ok_or_else(|| LabRunError::UnknownScenario {
                    suite: suite.id.to_string(),
                    id: id.to_string(),
                    available: suite
                        .scenarios
                        .iter()
                        .map(|scenario| {
                            format!(
                                "{}/{} -> {}",
                                suite.id, scenario.id, scenario.firehorse_scenario_id
                            )
                        })
                        .collect(),
                })
        })
        .transpose()?;

    let viewport = if let Some(viewport) = selection.viewport {
        viewport
    } else if let Some(scenario) = selected_scenario {
        ViewportClass::parse(scenario.default_viewport).ok_or_else(|| {
            LabRunError::UnknownViewport {
                value: scenario.default_viewport.to_string(),
            }
        })?
    } else {
        ViewportClass::Studio
    };
    let size = viewport.wtd_size();

    let scenario_stem = selected_scenario
        .map(|scenario| sanitize_file_stem(scenario.firehorse_scenario_id))
        .unwrap_or_else(|| "audit-cockpit".to_string());
    let workspace_name = workspace_name(&scenario_stem, viewport.name());
    let workspace_path = workspace_root.join(format!("{workspace_name}.yaml"));
    let target = format!("{workspace_name}/{TARGET_TAB}/{TARGET_PANE}");
    let executable = oxide_uxlab_executable()?;
    let cwd = std::env::current_dir().map_err(|error| LabRunError::Io(error.to_string()))?;

    let args = if let Some(scenario) = selected_scenario {
        vec![
            "--suite".to_string(),
            suite.id.to_string(),
            "--scenario".to_string(),
            scenario.firehorse_scenario_id.to_string(),
            "--viewport".to_string(),
            viewport.name().to_string(),
            "--once".to_string(),
            "--mockup".to_string(),
        ]
    } else {
        vec![
            "--audit".to_string(),
            "--suite".to_string(),
            suite.id.to_string(),
            "--viewport".to_string(),
            viewport.name().to_string(),
        ]
    };

    let visible_needle = if let Some(scenario) = selected_scenario {
        scenario.firehorse_scenario_id.to_string()
    } else {
        "OxIde UX Audit Lab".to_string()
    };

    Ok(WtdDesignPlan {
        workspace_name,
        workspace_path: workspace_path.to_string_lossy().to_string(),
        target: target.clone(),
        executable: executable.to_string_lossy().to_string(),
        cwd: cwd.to_string_lossy().to_string(),
        args,
        viewport: viewport.name().to_string(),
        width: size.width,
        height: size.height,
        suite_id: suite.id.to_string(),
        scenario_id: selected_scenario.map(|scenario| scenario.id.to_string()),
        firehorse_scenario_id: selected_scenario
            .map(|scenario| scenario.firehorse_scenario_id.to_string()),
        visible_needle,
        open_command: format!(
            "wtd open --file {} --recreate",
            shell_quote(&workspace_path.to_string_lossy())
        ),
        capture_text_command: format!("wtd capture {}", shell_quote(&target)),
        capture_vt_command: format!("wtd capture --vt {}", shell_quote(&target)),
    })
}

fn write_workspace_file(plan: &WtdDesignPlan) -> Result<(), LabRunError> {
    let path = PathBuf::from(&plan.workspace_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| LabRunError::Io(error.to_string()))?;
    }
    let yaml = workspace_yaml(plan);
    write_new(&path, yaml.as_bytes())
}

fn workspace_yaml(plan: &WtdDesignPlan) -> String {
    let mut out = String::new();
    out.push_str("version: 1\n");
    out.push_str(&format!("name: {}\n", plan.workspace_name));
    out.push_str("defaults:\n");
    out.push_str("  terminalSize:\n");
    out.push_str(&format!("    cols: {}\n", plan.width));
    out.push_str(&format!("    rows: {}\n", plan.height));
    out.push_str("profiles:\n");
    out.push_str(&format!("  {}:\n", plan.workspace_name));
    out.push_str("    type: custom\n");
    out.push_str(&format!(
        "    executable: '{}'\n",
        yaml_single_quote(&plan.executable)
    ));
    out.push_str(&format!("    cwd: '{}'\n", yaml_single_quote(&plan.cwd)));
    out.push_str("    args:\n");
    for arg in &plan.args {
        out.push_str(&format!("      - '{}'\n", yaml_single_quote(arg)));
    }
    out.push_str("tabs:\n");
    out.push_str(&format!("  - name: {TARGET_TAB}\n"));
    out.push_str("    layout:\n");
    out.push_str("      type: pane\n");
    out.push_str(&format!("      name: {TARGET_PANE}\n"));
    out.push_str("      session:\n");
    out.push_str(&format!("        profile: {}\n", plan.workspace_name));
    out
}

fn wait_for_visible_and_stable(plan: &WtdDesignPlan) -> Result<String, LabRunError> {
    let started = Instant::now();
    let mut last = String::new();
    while started.elapsed() < WAIT_TIMEOUT {
        last = capture_text(&plan.target)?;
        if last.contains(&plan.visible_needle) {
            return wait_for_stable_frame(&plan.target, last);
        }
        thread::sleep(POLL_INTERVAL);
    }

    Err(LabRunError::Io(format!(
        "wtd target {} did not show {:?} within {:?}; last capture:\n{}",
        plan.target, plan.visible_needle, WAIT_TIMEOUT, last
    )))
}

fn wait_for_stable_frame(target: &str, mut previous: String) -> Result<String, LabRunError> {
    let started = Instant::now();
    while started.elapsed() < WAIT_TIMEOUT {
        thread::sleep(STABLE_WINDOW);
        let current = capture_text(target)?;
        if current == previous {
            return Ok(current);
        }
        previous = current;
    }
    Err(LabRunError::Io(format!(
        "wtd target {target} did not stabilize within {WAIT_TIMEOUT:?}"
    )))
}

fn capture_text(target: &str) -> Result<String, LabRunError> {
    let output = run_wtd_checked(&["capture", target])?;
    String::from_utf8(output.stdout).map_err(|error| LabRunError::Io(error.to_string()))
}

fn capture_vt(target: &str) -> Result<Vec<u8>, LabRunError> {
    Ok(run_wtd_checked(&["capture", "--vt", target])?.stdout)
}

fn run_wtd_checked(args: &[&str]) -> Result<Output, LabRunError> {
    let output = Command::new("wtd")
        .args(args)
        .output()
        .map_err(|error| LabRunError::Io(format!("failed to invoke wtd: {error}")))?;
    if output.status.success() {
        Ok(output)
    } else {
        Err(LabRunError::Io(format!(
            "wtd {} failed with status {:?}\nstdout:\n{}\nstderr:\n{}",
            args.join(" "),
            output.status.code(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )))
    }
}

fn write_new(path: &Path, bytes: &[u8]) -> Result<(), LabRunError> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| {
            LabRunError::Io(format!("failed to create {}: {error}", path.display()))
        })?;
    file.write_all(bytes)
        .map_err(|error| LabRunError::Io(error.to_string()))
}

fn append_newline(path: &Path) -> Result<(), LabRunError> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(path)
        .map_err(|error| LabRunError::Io(error.to_string()))?;
    file.write_all(b"\n")
        .map_err(|error| LabRunError::Io(error.to_string()))
}

fn oxide_uxlab_executable() -> Result<PathBuf, LabRunError> {
    let cwd = std::env::current_dir().map_err(|error| LabRunError::Io(error.to_string()))?;
    let release = cwd.join("target").join("release").join(if cfg!(windows) {
        "oxide-uxlab.exe"
    } else {
        "oxide-uxlab"
    });
    if release.exists() {
        return Ok(release);
    }
    std::env::current_exe().map_err(|error| LabRunError::Io(error.to_string()))
}

fn capture_run_directory_name() -> String {
    format!("wtd_{}", unique_suffix())
}

fn workspace_name(stem: &str, viewport: &str) -> String {
    let suffix = short_unique_suffix();
    let prefix = "oxide-wtd";
    let fixed_len = prefix.len() + viewport.len() + suffix.len() + 3;
    let max_stem_len = 64usize.saturating_sub(fixed_len).max(1);
    let short_stem = stem.chars().take(max_stem_len).collect::<String>();
    format!("{prefix}-{short_stem}-{viewport}-{suffix}")
}

fn unique_suffix() -> String {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!(
        "{}_{:09}_{}",
        duration.as_secs(),
        duration.subsec_nanos(),
        std::process::id()
    )
}

fn short_unique_suffix() -> String {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}{}", std::process::id(), duration.as_millis() % 1_000_000)
}

fn capture_stem(plan: &WtdDesignPlan) -> String {
    sanitize_file_stem(&format!(
        "{}_{}",
        plan.firehorse_scenario_id
            .as_deref()
            .unwrap_or("audit-cockpit"),
        plan.viewport
    ))
}

fn sanitize_file_stem(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

fn yaml_single_quote(value: &str) -> String {
    value.replace('\'', "''")
}

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}

#[allow(dead_code)]
fn _size_for_docs(viewport: ViewportClass) -> ViewportSize {
    viewport.wtd_size()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shell::uxlab::audit::fixtures::firehorse_audit_suite;

    #[test]
    fn scenario_plan_opens_release_mockup_in_wtd() {
        let suite = firehorse_audit_suite();
        let mut selection = LabCliSelection::parse(
            [
                "--audit",
                "--suite",
                "firehorse",
                "--scenario",
                "firehorse-editing-lens-standard",
                "--viewport",
                "studio",
                "--wtd-open",
            ]
            .into_iter()
            .map(String::from),
        )
        .expect("selection");
        selection.viewport = Some(ViewportClass::Studio);

        let plan = build_plan_with_workspace_root(
            &selection,
            &suite,
            Path::new("target/ux_audit_lab/test_wtd"),
        )
        .expect("plan");

        assert_eq!(plan.viewport, "studio");
        assert_eq!(plan.width, 190);
        assert_eq!(plan.height, 48);
        assert_eq!(
            plan.firehorse_scenario_id.as_deref(),
            Some("firehorse-editing-lens-standard")
        );
        assert_eq!(plan.visible_needle, "firehorse-editing-lens-standard");
        assert!(plan.args.iter().any(|arg| arg == "--mockup"));
        assert!(!plan.args.iter().any(|arg| arg == "--audit"));
        assert!(plan.capture_vt_command.contains("wtd capture --vt"));
    }

    #[test]
    fn cockpit_plan_uses_audit_cockpit_not_once_mockup() {
        let suite = firehorse_audit_suite();
        let selection = LabCliSelection::parse(
            ["--audit", "--suite", "firehorse", "--wtd-open"]
                .into_iter()
                .map(String::from),
        )
        .expect("selection");

        let plan = build_plan_with_workspace_root(
            &selection,
            &suite,
            Path::new("target/ux_audit_lab/test_wtd"),
        )
        .expect("plan");

        assert_eq!(plan.viewport, "studio");
        assert!(plan.scenario_id.is_none());
        assert!(plan.args.iter().any(|arg| arg == "--audit"));
        assert!(!plan.args.iter().any(|arg| arg == "--once"));
        assert_eq!(plan.visible_needle, "OxIde UX Audit Lab");
    }

    #[test]
    fn generated_workspace_name_fits_wtd_limit() {
        let name = workspace_name(
            "firehorse-editing-lens-standard-with-an-extra-long-descriptive-tail",
            "first-class",
        );

        assert!(name.len() <= 64, "{name}");
        assert!(
            name.chars()
                .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
        );
    }
}
