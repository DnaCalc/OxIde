//! WinTermDriver test harness for OxIde.
//!
//! Drives the `wtd` CLI to launch OxIde in a ConPTY under `wtd-host`, capture
//! the rendered screen, and compare against committed goldens.
//!
//! Owned by W037. See `docs/TESTING_WTD.md` for the development loop.

#![allow(dead_code)]

use std::{
    env,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::{Mutex, MutexGuard, OnceLock},
    thread,
    time::{Duration, Instant},
};

/// Serializes all `Harness` instances across a test binary.
///
/// wtd workspaces are keyed by name; two tests opening the same name with
/// `--recreate` in parallel would tear down each other's panes. Tests can
/// still run as a parallel suite (the guard is only held for the lifetime of
/// one `Harness`), but within each test the workspace is owned exclusively.
fn harness_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// Default capture polling interval.
const POLL_INTERVAL: Duration = Duration::from_millis(200);

/// Default overall timeout for `wait_for_text` and `wait_for_stable_frame`.
const DEFAULT_WAIT_TIMEOUT: Duration = Duration::from_secs(20);

/// Stable-frame detection compares this many consecutive captures.
const STABLE_WINDOW: Duration = Duration::from_millis(800);

/// Whether the caller wants to overwrite goldens instead of asserting them.
pub fn update_goldens() -> bool {
    env::var("UPDATE_GOLDENS")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Path to the OxIde repo root.
pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Path to `tests/wtd/goldens/<workset>/`.
pub fn goldens_dir(workset: &str) -> PathBuf {
    repo_root()
        .join("tests")
        .join("wtd")
        .join("goldens")
        .join(workset)
}

/// WTD launch contract for a non-interactive `oxide-uxlab --once` scenario.
///
/// The workspace YAML still owns the concrete process launch because `wtd`
/// opens panes from workspace files. This struct keeps the suite/id/viewport,
/// workspace target, and expected command shape together so future UX-lab
/// journeys do not each invent their own convention.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LabScenarioJourney {
    pub workspace_yaml: &'static str,
    pub workspace_name: &'static str,
    pub pane_suffix: &'static str,
    pub suite: &'static str,
    pub scenario: &'static str,
    pub viewport: &'static str,
}

impl LabScenarioJourney {
    pub const fn new(
        workspace_yaml: &'static str,
        workspace_name: &'static str,
        pane_suffix: &'static str,
        suite: &'static str,
        scenario: &'static str,
        viewport: &'static str,
    ) -> Self {
        Self {
            workspace_yaml,
            workspace_name,
            pane_suffix,
            suite,
            scenario,
            viewport,
        }
    }

    pub fn oxide_uxlab_args(&self) -> Vec<&'static str> {
        vec![
            "--suite",
            self.suite,
            "--scenario",
            self.scenario,
            "--viewport",
            self.viewport,
            "--once",
        ]
    }

    pub fn oxide_uxlab_command_line(&self) -> String {
        format!("oxide-uxlab.exe {}", self.oxide_uxlab_args().join(" "))
    }
}

/// A running wtd workspace for a single test.
///
/// On `Drop` the workspace is closed so panes do not leak between tests.
/// Holds a process-wide mutex for its lifetime so concurrent tests do not
/// tear down each other's workspace with `--recreate`.
pub struct Harness {
    workspace_name: String,
    _guard: MutexGuard<'static, ()>,
}

impl Harness {
    /// Open the WTD workspace for a non-interactive lab scenario.
    pub fn open_lab_once(journey: &LabScenarioJourney) -> Self {
        Self::open(journey.workspace_yaml, journey.workspace_name)
    }

    /// Open the workspace definition at `yaml_path` (relative to the repo root),
    /// returning a handle whose target strings are addressed as
    /// `<workspace_name>/<tab>/<pane>`.
    pub fn open(yaml_path: impl AsRef<Path>, workspace_name: &str) -> Self {
        let guard = harness_lock();

        let yaml = repo_root().join(yaml_path.as_ref());
        assert!(
            yaml.exists(),
            "workspace definition not found: {}",
            yaml.display()
        );

        // Close any lingering instance first so reruns are deterministic.
        let _ = wtd_command(&["close", workspace_name]);

        let out = wtd_command(&[
            "open",
            "--file",
            yaml.to_str().expect("non-utf8 yaml path"),
            "--recreate",
        ]);
        assert!(
            out.status.success(),
            "wtd open failed: stdout={} stderr={}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );

        Self {
            workspace_name: workspace_name.to_string(),
            _guard: guard,
        }
    }

    /// Capture the visible screen of `target_suffix` (e.g. `"empty/oxide-empty-pane"`).
    pub fn capture_text(&self, target_suffix: &str) -> String {
        let target = self.target(target_suffix);
        let out = wtd_command(&["capture", &target]);
        assert!(
            out.status.success(),
            "wtd capture failed for {target}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8_lossy(&out.stdout).into_owned()
    }

    /// Wait for a lab scenario's visible contracts, stabilize, then capture text.
    pub fn capture_lab_once_text(
        &self,
        journey: &LabScenarioJourney,
        visible_needles: &[&str],
    ) -> String {
        for needle in visible_needles {
            self.wait_for_text(journey.pane_suffix, needle);
        }
        self.wait_for_stable_frame(journey.pane_suffix);
        self.capture_text(journey.pane_suffix)
    }

    /// Send one or more key specs (e.g. `["Ctrl+O"]`, `["Down", "Enter"]`)
    /// to `target_suffix`.
    pub fn send_keys(&self, target_suffix: &str, keys: &[&str]) {
        assert!(!keys.is_empty(), "send_keys requires at least one key spec");
        let target = self.target(target_suffix);
        let mut command = Command::new("wtd");
        command.arg("keys").arg(target);
        for key in keys {
            command.arg(key);
        }
        let out = command.output().unwrap_or_else(|err| {
            panic!("failed to invoke `wtd keys`: {err}");
        });
        assert!(
            out.status.success(),
            "wtd keys failed: stdout={} stderr={}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
    }

    /// Send literal text bytes without an implicit trailing newline.
    pub fn send_text(&self, target_suffix: &str, text: &str) {
        let target = self.target(target_suffix);
        let out = wtd_command(&["send", "--no-newline", &target, text]);
        assert!(
            out.status.success(),
            "wtd send failed: stdout={} stderr={}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
    }

    /// Inject a mouse click into the pane.
    pub fn mouse_click(&self, target_suffix: &str, col: u16, row: u16) {
        let target = self.target(target_suffix);
        let out = wtd_command(&[
            "mouse",
            "--col",
            &col.to_string(),
            "--row",
            &row.to_string(),
            "--button",
            "left",
            &target,
            "click",
        ]);
        assert!(
            out.status.success(),
            "wtd mouse click failed: stdout={} stderr={}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
    }

    /// Capture a replayable VT snapshot of `target_suffix`.
    pub fn capture_vt(&self, target_suffix: &str) -> Vec<u8> {
        let target = self.target(target_suffix);
        let out = wtd_command(&["capture", "--vt", &target]);
        assert!(
            out.status.success(),
            "wtd capture --vt failed for {target}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        out.stdout
    }

    /// Capture the replayable VT snapshot for a stabilized lab scenario pane.
    pub fn capture_lab_once_vt(&self, journey: &LabScenarioJourney) -> Vec<u8> {
        self.capture_vt(journey.pane_suffix)
    }

    /// Poll `target_suffix` until `needle` appears or `DEFAULT_WAIT_TIMEOUT` elapses.
    pub fn wait_for_text(&self, target_suffix: &str, needle: &str) {
        self.wait_for_text_with_timeout(target_suffix, needle, DEFAULT_WAIT_TIMEOUT);
    }

    pub fn wait_for_text_with_timeout(&self, target_suffix: &str, needle: &str, timeout: Duration) {
        let start = Instant::now();
        let mut last = String::new();
        while start.elapsed() < timeout {
            last = self.capture_text(target_suffix);
            if last.contains(needle) {
                return;
            }
            thread::sleep(POLL_INTERVAL);
        }
        panic!(
            "wait_for_text timeout after {:?} waiting for {:?} in {target_suffix}.\n--- last capture ---\n{last}",
            timeout, needle
        );
    }

    /// Wait until two consecutive captures taken `STABLE_WINDOW` apart are
    /// identical, or until `DEFAULT_WAIT_TIMEOUT` elapses.
    pub fn wait_for_stable_frame(&self, target_suffix: &str) {
        let start = Instant::now();
        let mut previous = self.capture_text(target_suffix);
        while start.elapsed() < DEFAULT_WAIT_TIMEOUT {
            thread::sleep(STABLE_WINDOW);
            let current = self.capture_text(target_suffix);
            if current == previous {
                return;
            }
            previous = current;
        }
        panic!(
            "wait_for_stable_frame timeout after {:?} for {target_suffix}.\n--- last capture ---\n{previous}",
            DEFAULT_WAIT_TIMEOUT
        );
    }

    fn target(&self, suffix: &str) -> String {
        format!("{}/{}", self.workspace_name, suffix)
    }
}

impl Drop for Harness {
    fn drop(&mut self) {
        let _ = wtd_command(&["close", &self.workspace_name]);
    }
}

fn wtd_command(args: &[&str]) -> Output {
    Command::new("wtd")
        .args(args)
        .output()
        .unwrap_or_else(|err| {
            panic!(
                "failed to invoke `wtd {}`: {err}. Is wtd on PATH and the host running?",
                args.join(" ")
            )
        })
}

/// Assert that `actual` matches the text golden at `goldens/<workset>/<name>.txt`.
///
/// Set `UPDATE_GOLDENS=1` to overwrite instead of asserting.
pub fn assert_golden_text(workset: &str, name: &str, actual: &str) {
    let path = goldens_dir(workset).join(format!("{name}.txt"));

    if update_goldens() {
        std::fs::create_dir_all(path.parent().unwrap()).expect("create goldens dir");
        std::fs::write(&path, actual).expect("write golden");
        eprintln!("updated golden: {}", path.display());
        return;
    }

    let expected = std::fs::read_to_string(&path).unwrap_or_else(|err| {
        panic!(
            "golden not found at {}: {err}. Run with UPDATE_GOLDENS=1 to create it.",
            path.display()
        )
    });

    if expected != actual {
        let diff = simple_line_diff(&expected, actual);
        panic!(
            "golden mismatch for {name}.txt\ngolden:   {}\ncaptured: (see below)\n\n{diff}",
            path.display()
        );
    }
}

/// Assert that `actual_vt` matches the VT golden at `goldens/<workset>/<name>.vt`.
///
/// VT comparison is byte-exact. Set `UPDATE_GOLDENS=1` to overwrite.
pub fn assert_golden_vt(workset: &str, name: &str, actual_vt: &[u8]) {
    let path = goldens_dir(workset).join(format!("{name}.vt"));

    if update_goldens() {
        std::fs::create_dir_all(path.parent().unwrap()).expect("create goldens dir");
        std::fs::write(&path, actual_vt).expect("write golden");
        eprintln!("updated VT golden: {}", path.display());
        return;
    }

    let expected = std::fs::read(&path).unwrap_or_else(|err| {
        panic!(
            "VT golden not found at {}: {err}. Run with UPDATE_GOLDENS=1 to create it.",
            path.display()
        )
    });

    if expected != actual_vt {
        panic!(
            "VT golden mismatch for {name}.vt at {} (byte-exact).\n\
             Re-inspect via `wtd capture --vt ...` and compare the paired .txt golden \
             for a human-readable view. Run UPDATE_GOLDENS=1 to re-bless after review.",
            path.display()
        );
    }
}

/// Produce a compact line-level diff suitable for test panics.
/// Prefix `-` = expected, `+` = actual, ` ` = matching.
fn simple_line_diff(expected: &str, actual: &str) -> String {
    let expected_lines: Vec<&str> = expected.lines().collect();
    let actual_lines: Vec<&str> = actual.lines().collect();
    let mut out = String::new();
    let max = expected_lines.len().max(actual_lines.len());
    for i in 0..max {
        let e = expected_lines.get(i).copied().unwrap_or("");
        let a = actual_lines.get(i).copied().unwrap_or("");
        if e == a {
            out.push_str("  ");
            out.push_str(e);
        } else {
            out.push_str("- ");
            out.push_str(e);
            out.push('\n');
            out.push_str("+ ");
            out.push_str(a);
        }
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lab_scenario_journey_builds_expected_uxlab_once_command() {
        let journey = LabScenarioJourney::new(
            ".wtd/oxide-uxlab-smoke.yaml",
            "oxide-uxlab-smoke",
            "uxlab-smoke/oxide-uxlab-smoke-pane",
            "lab-smoke",
            "lab-smoke-editing",
            "standard",
        );

        assert_eq!(
            journey.oxide_uxlab_args(),
            vec![
                "--suite",
                "lab-smoke",
                "--scenario",
                "lab-smoke-editing",
                "--viewport",
                "standard",
                "--once",
            ]
        );
        assert_eq!(
            journey.oxide_uxlab_command_line(),
            "oxide-uxlab.exe --suite lab-smoke --scenario lab-smoke-editing --viewport standard --once"
        );
    }
}
