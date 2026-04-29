use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;

use super::controller::AuditLabModel;
use super::fixtures::design_brief_for;
use super::model::{
    AUDIT_SCHEMA_VERSION, AuditGate, UxAuditRunInput, UxAuditScenario, UxAuditScorecard,
    UxAuditSuite,
};
use super::schema::AuditJsonEnvelope;
use super::score::{batch_gate, scorecard_for};
use crate::shell::uxlab::{LabRunError, LabScenarioRegistry, ViewportClass};

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UxAuditExportResult {
    pub root: String,
    pub files_written: Vec<String>,
    pub gate: AuditGate,
}

pub fn export_current_review_pack(
    model: &AuditLabModel,
    root: &Path,
) -> Result<UxAuditExportResult, LabRunError> {
    let suite = model.suite();
    let state = model.state();
    let scenario = state.selected_scenario(suite);
    let scorecard = scorecard_for(
        suite,
        scenario.firehorse_scenario_id,
        state.viewport,
        model.lab_registry(),
    )?;
    export_review_pack(
        suite,
        scenario,
        state.viewport,
        &scorecard,
        &state.findings,
        model.lab_registry(),
        root,
    )
}

pub fn export_review_pack(
    suite: &UxAuditSuite,
    scenario: &UxAuditScenario,
    viewport: ViewportClass,
    scorecard: &UxAuditScorecard,
    findings: &[super::model::UxAuditFinding],
    lab_registry: &LabScenarioRegistry<'_>,
    root: &Path,
) -> Result<UxAuditExportResult, LabRunError> {
    if !is_audit_export_root(root) {
        return Err(LabRunError::Io(format!(
            "refusing audit export outside docs/firehorse_mockups/ux_audit_lab or target: {}",
            root.display()
        )));
    }

    let capture_dir = root.join("captures");
    let ansi_dir = root.join("ansi");
    fs::create_dir_all(&capture_dir).map_err(|error| LabRunError::Io(error.to_string()))?;
    fs::create_dir_all(&ansi_dir).map_err(|error| LabRunError::Io(error.to_string()))?;

    let stem = export_stem(scenario.firehorse_scenario_id, viewport);
    let mockup =
        lab_registry.render_mockup(suite.id, scenario.firehorse_scenario_id, Some(viewport))?;
    let ansi = lab_registry.render_mockup_terminal_stream(
        suite.id,
        scenario.firehorse_scenario_id,
        Some(viewport),
    )?;
    let brief = design_brief_for(suite, scenario.firehorse_scenario_id, viewport.name())
        .expect("export scenario should have design brief");
    let run = UxAuditRunInput {
        suite_id: suite.id.to_string(),
        scenario_ids: vec![scenario.firehorse_scenario_id.to_string()],
        viewports: vec![viewport.name().to_string()],
        evaluation: vec!["functional".to_string(), "aesthetic".to_string()],
        output_root: Some(root.to_string_lossy().to_string()),
    };

    let files = vec![
        (
            root.join("README.md"),
            review_readme(suite, scenario, viewport, scorecard),
        ),
        (
            root.join("audit_schema.json"),
            json_string("audit_schema", schema_doc())?,
        ),
        (
            root.join("audit_suite.json"),
            json_string("audit_suite", suite)?,
        ),
        (
            root.join(format!("agent_brief_{stem}.json")),
            json_string("audit_design_brief", &brief)?,
        ),
        (root.join("audit_run.json"), json_string("audit_run", &run)?),
        (
            root.join("scorecard.json"),
            json_string("audit_scorecard", scorecard)?,
        ),
        (
            root.join("findings.json"),
            json_string("audit_findings", findings)?,
        ),
        (
            root.join(format!("scenario_{stem}.md")),
            scenario_markdown(suite, scenario, viewport, scorecard, findings),
        ),
        (capture_dir.join(format!("{stem}.txt")), mockup.text),
        (
            ansi_dir.join(format!("{stem}.ansi")),
            String::from_utf8_lossy(&ansi).to_string(),
        ),
    ];

    for (path, _) in &files {
        refuse_overwrite(path)?;
    }

    let mut files_written = Vec::new();
    for (path, contents) in files {
        fs::write(&path, contents).map_err(|error| LabRunError::Io(error.to_string()))?;
        files_written.push(path.to_string_lossy().to_string());
    }

    Ok(UxAuditExportResult {
        root: root.to_string_lossy().to_string(),
        files_written,
        gate: scorecard.gate,
    })
}

pub fn export_suite_review_pack(
    suite: &UxAuditSuite,
    lab_registry: &LabScenarioRegistry<'_>,
    root: &Path,
) -> Result<UxAuditExportResult, LabRunError> {
    if !is_audit_export_root(root) {
        return Err(LabRunError::Io(format!(
            "refusing audit export outside docs/firehorse_mockups/ux_audit_lab or target: {}",
            root.display()
        )));
    }

    fs::create_dir_all(root).map_err(|error| LabRunError::Io(error.to_string()))?;
    let summary_readme = root.join("README.md");
    let suite_json = root.join("audit_suite.json");
    let scorecards_json = root.join("scorecards.json");
    refuse_overwrite(&summary_readme)?;
    refuse_overwrite(&suite_json)?;
    refuse_overwrite(&scorecards_json)?;

    let mut files_written = Vec::new();
    let mut scorecards = Vec::new();
    for scenario in &suite.scenarios {
        let viewport = ViewportClass::parse(scenario.default_viewport).ok_or_else(|| {
            LabRunError::UnknownViewport {
                value: scenario.default_viewport.to_string(),
            }
        })?;
        let scorecard = scorecard_for(
            suite,
            scenario.firehorse_scenario_id,
            viewport,
            lab_registry,
        )?;
        scorecards.push(scorecard.clone());
        let subroot = root.join(export_stem(scenario.firehorse_scenario_id, viewport));
        let result = export_review_pack(
            suite,
            scenario,
            viewport,
            &scorecard,
            &[],
            lab_registry,
            &subroot,
        )?;
        files_written.extend(result.files_written);
    }

    let gate = batch_gate(&scorecards);
    let readme = suite_readme(suite, gate);
    fs::write(&summary_readme, readme).map_err(|error| LabRunError::Io(error.to_string()))?;
    fs::write(&suite_json, json_string("audit_suite", suite)?)
        .map_err(|error| LabRunError::Io(error.to_string()))?;
    fs::write(
        &scorecards_json,
        json_string("audit_scorecards", &scorecards)?,
    )
    .map_err(|error| LabRunError::Io(error.to_string()))?;

    files_written.push(summary_readme.to_string_lossy().to_string());
    files_written.push(suite_json.to_string_lossy().to_string());
    files_written.push(scorecards_json.to_string_lossy().to_string());

    Ok(UxAuditExportResult {
        root: root.to_string_lossy().to_string(),
        files_written,
        gate,
    })
}

pub fn scenario_markdown(
    suite: &UxAuditSuite,
    scenario: &UxAuditScenario,
    viewport: ViewportClass,
    scorecard: &UxAuditScorecard,
    findings: &[super::model::UxAuditFinding],
) -> String {
    let persona = suite
        .find_persona(scenario.persona_id)
        .expect("scenario persona should exist");
    let mut out = String::new();
    out.push_str(&format!(
        "# {} - {}\n\n",
        scenario.title, scenario.firehorse_scenario_id
    ));
    out.push_str(&format!(
        "Suite: `{}`  \nViewport: `{}`  \nGate: `{:?}`\n\n",
        suite.id,
        viewport.name(),
        scorecard.gate
    ));
    out.push_str("## Persona\n\n");
    out.push_str(&format!(
        "- `{}`: {}\n- Pressure: {}\n- Delight target: {}\n\n",
        persona.id, persona.role, persona.job_pressure, persona.delight_target
    ));
    out.push_str("## Journey\n\n");
    for step in &scenario.steps {
        out.push_str(&format!("### {} - {}\n\n", step.id, step.title));
        out.push_str(&format!("Intent: {}\n\n", step.user_intent));
        out.push_str("Expected surfaces:\n");
        for surface in &step.expected_surfaces {
            out.push_str(&format!(
                "- {} -> `{}` -> {} -> {}\n",
                surface.surface,
                surface.projection_path,
                surface.visible_contract,
                surface.owner_workset
            ));
        }
        out.push_str(&format!(
            "\nActions: `{}`\n\n",
            step.expected_actions.join("`, `")
        ));
        out.push_str("Seams:\n");
        for seam in &step.seam_refs {
            out.push_str(&format!(
                "- {} -> {:?} -> {}\n",
                seam.source, seam.status, seam.downstream_workset
            ));
        }
        out.push('\n');
    }
    out.push_str("## Functional Scorecard\n\n");
    for result in &scorecard.functional {
        out.push_str(&format!(
            "- `{}` -> {:?}: {}\n",
            result.criterion_id, result.status, result.rationale
        ));
    }
    out.push_str("\n## Aesthetic Scorecard\n\n");
    for result in &scorecard.aesthetic {
        out.push_str(&format!(
            "- `{}` -> {:?}: {}\n",
            result.criterion_id, result.status, result.rationale
        ));
    }
    out.push_str("\n## Local Findings\n\n");
    let local = findings
        .iter()
        .filter(|finding| finding.scenario_id == scenario.id)
        .collect::<Vec<_>>();
    if local.is_empty() {
        out.push_str("No local findings were marked in this export.\n");
    } else {
        for finding in local {
            out.push_str(&format!(
                "- `{}` -> {:?}: {}\n",
                finding.criterion_id, finding.status, finding.rationale
            ));
        }
    }
    out.push_str("\n## Reproduction\n\n");
    for command in &scorecard.reproduction_commands {
        out.push_str(&format!("- `{command}`\n"));
    }
    out.push_str("\n## Artifacts\n\n");
    for artifact in &scenario.reference_artifacts {
        out.push_str(&format!(
            "- {} | {} | `{}`\n",
            artifact.kind, artifact.title, artifact.path
        ));
    }
    out
}

pub fn is_audit_export_root(path: &Path) -> bool {
    let normalized = path.to_string_lossy().replace('\\', "/");
    normalized.contains("docs/firehorse_mockups/ux_audit_lab")
        || normalized.contains("target/ux_audit_lab")
}

fn review_readme(
    suite: &UxAuditSuite,
    scenario: &UxAuditScenario,
    viewport: ViewportClass,
    scorecard: &UxAuditScorecard,
) -> String {
    format!(
        "# Fire Horse UX Audit Export\n\nSuite: `{}`  \nScenario: `{}`  \nViewport: `{}`  \nGate: `{:?}`\n\nThis is a local evidence pack. It does not create public posts or external issues.\n",
        suite.id,
        scenario.firehorse_scenario_id,
        viewport.name(),
        scorecard.gate
    )
}

fn suite_readme(suite: &UxAuditSuite, gate: AuditGate) -> String {
    format!(
        "# Fire Horse UX Audit Suite Export\n\nSuite: `{}`  \nScenarios: `{}`  \nGate: `{:?}`\n\nEach subdirectory is a local evidence pack for one Fire Horse scenario at its default audit viewport.\n",
        suite.id,
        suite.scenarios.len(),
        gate
    )
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct AuditSchemaDoc {
    schema_version: u32,
    outputs: Vec<&'static str>,
    exit_codes: Vec<&'static str>,
}

fn schema_doc() -> AuditSchemaDoc {
    AuditSchemaDoc {
        schema_version: AUDIT_SCHEMA_VERSION,
        outputs: vec![
            "audit_suite",
            "audit_design_brief",
            "audit_run",
            "audit_scorecard",
            "audit_findings",
        ],
        exit_codes: vec![
            "0 ready",
            "1 concern_or_blocked",
            "2 command_or_input_error",
            "3 render_or_capture_error",
        ],
    }
}

fn json_string<T>(kind: &'static str, value: T) -> Result<String, LabRunError>
where
    T: Serialize,
{
    serde_json::to_string_pretty(&AuditJsonEnvelope::new(kind, value))
        .map_err(|error| LabRunError::Io(error.to_string()))
        .map(|mut value| {
            value.push('\n');
            value
        })
}

fn refuse_overwrite(path: &Path) -> Result<(), LabRunError> {
    if path.exists() {
        Err(LabRunError::Io(format!(
            "refusing to overwrite existing audit export file {}",
            path.display()
        )))
    } else {
        Ok(())
    }
}

fn export_stem(scenario_id: &str, viewport: ViewportClass) -> String {
    format!("{}_{}", scenario_id, viewport.name())
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

#[allow(dead_code)]
fn export_root_from_cli_path(path: &Path) -> PathBuf {
    if path.extension().and_then(|extension| extension.to_str()) == Some("md") {
        path.with_extension("")
    } else {
        path.to_path_buf()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shell::uxlab::audit::fixtures::firehorse_audit_suite;
    use crate::shell::uxlab::audit::score::scorecard_for;

    #[test]
    fn scenario_markdown_includes_review_pack_sections() {
        let suite = firehorse_audit_suite();
        let scenario = suite
            .find_scenario("firehorse-editing-lens-standard")
            .expect("scenario");
        let registry = LabScenarioRegistry::built_in();
        let scorecard = scorecard_for(
            &suite,
            scenario.firehorse_scenario_id,
            ViewportClass::Studio,
            &registry,
        )
        .expect("scorecard");

        let markdown = scenario_markdown(&suite, scenario, ViewportClass::Studio, &scorecard, &[]);

        assert!(markdown.contains("## Persona"));
        assert!(markdown.contains("## Journey"));
        assert!(markdown.contains("## Functional Scorecard"));
        assert!(markdown.contains("## Aesthetic Scorecard"));
        assert!(markdown.contains("HostWorkspaceSession::diagnostics"));
    }

    #[test]
    fn export_root_guard_accepts_audit_lab_paths_only() {
        assert!(is_audit_export_root(Path::new(
            "docs/firehorse_mockups/ux_audit_lab/exports/editing"
        )));
        assert!(is_audit_export_root(Path::new(
            "target/ux_audit_lab/export-smoke"
        )));
        assert!(!is_audit_export_root(Path::new("docs")));
    }
}
