pub mod automation;
pub mod controller;
pub mod export;
pub mod fixtures;
pub mod model;
pub mod registry;
pub mod schema;
pub mod score;
pub mod view;

use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, process};

use serde::Serialize;

use super::{
    LabCliMode, LabCliSelection, LabRunError, LabRunOutcome, LabScenarioRegistry, ViewportClass,
};
use automation::{
    evaluation_modes_from_values, filter_scorecard_by_modes, outcome_for_gate,
    requested_evaluation_modes,
};
use controller::AuditLabModel;
use export::{export_current_review_pack, export_suite_review_pack, is_audit_export_root};
use fixtures::design_brief_for;
use model::{
    AuditGate, EvaluationMode, UxAuditBatchResult, UxAuditCriterion, UxAuditDossier,
    UxAuditRunInput, UxAuditScorecard, UxAuditSuite,
};
use registry::UxAuditRegistry;
use schema::AuditJsonEnvelope;
use score::{batch_gate, scorecard_for};

pub fn run_audit_cli<W>(
    selection: &LabCliSelection,
    lab_registry: &LabScenarioRegistry<'_>,
    mut out: W,
) -> Result<LabRunOutcome, LabRunError>
where
    W: Write,
{
    let audit_registry = UxAuditRegistry::built_in();

    match selection.mode {
        Some(LabCliMode::Batch) => write_batch(selection, &audit_registry, lab_registry, &mut out),
        Some(LabCliMode::Export) => {
            write_export(selection, &audit_registry, lab_registry, &mut out)
        }
        Some(LabCliMode::List) => {
            write_list(selection, &audit_registry, &mut out).map(|_| LabRunOutcome::Success)
        }
        Some(LabCliMode::Matrix) => {
            write_matrix(selection, &audit_registry, &mut out).map(|_| LabRunOutcome::Success)
        }
        Some(LabCliMode::Brief) => {
            write_brief(selection, &audit_registry, &mut out).map(|_| LabRunOutcome::Success)
        }
        Some(LabCliMode::Evaluate) => {
            write_evaluate(selection, &audit_registry, lab_registry, &mut out)
        }
        Some(LabCliMode::Once) => write_dossier(selection, &audit_registry, lab_registry, &mut out)
            .map(|_| LabRunOutcome::Success),
        None => write_cockpit_once(selection, &mut out).map(|_| LabRunOutcome::Success),
    }
}

fn write_list<W>(
    selection: &LabCliSelection,
    audit_registry: &UxAuditRegistry,
    out: &mut W,
) -> Result<(), LabRunError>
where
    W: Write,
{
    let suites = selected_suites(selection, audit_registry)?;
    if selection.json {
        write_json(out, "audit_suite_list", suites)
    } else {
        writeln!(out, "OxIde UX Audit Lab suites")
            .map_err(|error| LabRunError::Io(error.to_string()))?;
        for suite in suites {
            writeln!(
                out,
                "{}/{} | personas {} | scenarios {} | criteria {}",
                suite.id,
                suite.title,
                suite.personas.len(),
                suite.scenarios.len(),
                suite.criteria.len()
            )
            .map_err(|error| LabRunError::Io(error.to_string()))?;
            for scenario in &suite.scenarios {
                writeln!(
                    out,
                    "  {} -> {} | persona {} | {}",
                    scenario.id,
                    scenario.firehorse_scenario_id,
                    scenario.persona_id,
                    scenario.title
                )
                .map_err(|error| LabRunError::Io(error.to_string()))?;
            }
        }
        Ok(())
    }
}

fn write_matrix<W>(
    selection: &LabCliSelection,
    audit_registry: &UxAuditRegistry,
    out: &mut W,
) -> Result<(), LabRunError>
where
    W: Write,
{
    let suite_id = required_suite(selection)?;
    let rows = audit_registry
        .matrix_rows(&suite_id)
        .ok_or_else(|| unknown_audit_suite(&suite_id, audit_registry))?;
    if selection.json {
        write_json(out, "audit_matrix", rows)
    } else {
        writeln!(out, "OxIde UX Audit Lab matrix: {suite_id}")
            .map_err(|error| LabRunError::Io(error.to_string()))?;
        for row in rows {
            writeln!(
                out,
                "{} -> {} | persona {} | viewport {} | owners {}",
                row.audit_scenario_id,
                row.firehorse_scenario_id,
                row.persona_id,
                row.default_viewport,
                row.downstream_worksets.join(",")
            )
            .map_err(|error| LabRunError::Io(error.to_string()))?;
        }
        Ok(())
    }
}

fn write_brief<W>(
    selection: &LabCliSelection,
    audit_registry: &UxAuditRegistry,
    out: &mut W,
) -> Result<(), LabRunError>
where
    W: Write,
{
    let suite_id = required_suite(selection)?;
    let scenario_id = required_scenario(selection)?;
    let suite = audit_registry
        .suite(&suite_id)
        .ok_or_else(|| unknown_audit_suite(&suite_id, audit_registry))?;
    let viewport = viewport_name(selection, suite, &scenario_id)?;
    let brief = design_brief_for(suite, &scenario_id, &viewport)
        .ok_or_else(|| unknown_audit_scenario(&suite_id, &scenario_id, suite, audit_registry))?;

    if selection.json {
        write_json(out, "audit_design_brief", brief)
    } else {
        writeln!(out, "Design brief: {}", brief.scenario_id)
            .map_err(|error| LabRunError::Io(error.to_string()))?;
        writeln!(out, "Fire Horse scenario: {}", brief.firehorse_scenario_id)
            .map_err(|error| LabRunError::Io(error.to_string()))?;
        writeln!(out, "Viewport: {}", brief.viewport)
            .map_err(|error| LabRunError::Io(error.to_string()))?;
        writeln!(out, "Intent: {}", brief.design_intent)
            .map_err(|error| LabRunError::Io(error.to_string()))?;
        writeln!(out, "Preserve: {}", brief.must_preserve.join(" | "))
            .map_err(|error| LabRunError::Io(error.to_string()))?;
        Ok(())
    }
}

fn write_dossier<W>(
    selection: &LabCliSelection,
    audit_registry: &UxAuditRegistry,
    lab_registry: &LabScenarioRegistry<'_>,
    out: &mut W,
) -> Result<(), LabRunError>
where
    W: Write,
{
    let suite_id = required_suite(selection)?;
    let scenario_id = required_scenario(selection)?;
    let suite = audit_registry
        .suite(&suite_id)
        .ok_or_else(|| unknown_audit_suite(&suite_id, audit_registry))?;
    let scenario = suite
        .find_scenario(&scenario_id)
        .ok_or_else(|| unknown_audit_scenario(&suite_id, &scenario_id, suite, audit_registry))?;
    let persona = suite
        .find_persona(scenario.persona_id)
        .expect("audit fixtures must reference valid personas");
    let viewport = viewport_name(selection, suite, &scenario_id)?;

    lab_registry
        .find(&suite_id, scenario.firehorse_scenario_id)
        .ok_or_else(|| LabRunError::UnknownScenario {
            suite: suite_id.clone(),
            id: scenario.firehorse_scenario_id.to_string(),
            available: lab_registry.available_rows(),
        })?;

    let dossier = UxAuditDossier {
        suite_id: suite.id,
        scenario: scenario.clone(),
        persona: persona.clone(),
        viewport: viewport.clone(),
        functional_criteria: criteria_by_mode(suite, EvaluationMode::Functional),
        aesthetic_criteria: criteria_by_mode(suite, EvaluationMode::Aesthetic),
        render_commands: vec![
            format!(
                "target/release/oxide-uxlab.exe --suite {suite_id} --scenario {} --viewport {viewport} --once --mockup",
                scenario.firehorse_scenario_id
            ),
            format!(
                "target/release/oxide-uxlab.exe --suite {suite_id} --scenario {} --viewport {viewport} --once --mockup --ansi",
                scenario.firehorse_scenario_id
            ),
        ],
        reference_artifacts: scenario.reference_artifacts.clone(),
    };

    if selection.json {
        write_json(out, "audit_dossier", dossier)
    } else {
        writeln!(out, "Audit dossier: {}", dossier.scenario.title)
            .map_err(|error| LabRunError::Io(error.to_string()))?;
        writeln!(
            out,
            "scenario: {} -> {}",
            dossier.scenario.id, dossier.scenario.firehorse_scenario_id
        )
        .map_err(|error| LabRunError::Io(error.to_string()))?;
        writeln!(
            out,
            "persona: {} | {}",
            dossier.persona.id, dossier.persona.job_pressure
        )
        .map_err(|error| LabRunError::Io(error.to_string()))?;
        writeln!(out, "viewport: {}", dossier.viewport)
            .map_err(|error| LabRunError::Io(error.to_string()))?;
        writeln!(out, "surfaces:").map_err(|error| LabRunError::Io(error.to_string()))?;
        for step in &dossier.scenario.steps {
            for surface in &step.expected_surfaces {
                writeln!(
                    out,
                    "- {} -> {} -> {}",
                    surface.surface, surface.projection_path, surface.owner_workset
                )
                .map_err(|error| LabRunError::Io(error.to_string()))?;
            }
        }
        writeln!(
            out,
            "commands: {}",
            dossier.scenario.steps[0].expected_actions.join(", ")
        )
        .map_err(|error| LabRunError::Io(error.to_string()))?;
        Ok(())
    }
}

fn write_evaluate<W>(
    selection: &LabCliSelection,
    audit_registry: &UxAuditRegistry,
    lab_registry: &LabScenarioRegistry<'_>,
    out: &mut W,
) -> Result<LabRunOutcome, LabRunError>
where
    W: Write,
{
    let suite_id = required_suite(selection)?;
    let scenario_id = required_scenario(selection)?;
    let suite = audit_registry
        .suite(&suite_id)
        .ok_or_else(|| unknown_audit_suite(&suite_id, audit_registry))?;
    let viewport = viewport_for(selection, suite, &scenario_id)?;
    let modes = requested_evaluation_modes(selection)?;
    let scorecard = filter_scorecard_by_modes(
        scorecard_for(suite, &scenario_id, viewport, lab_registry)?,
        &modes,
    );
    let outcome = outcome_for_gate(scorecard.gate);

    if selection.json {
        write_json(out, "audit_scorecard", scorecard)?;
    } else {
        writeln!(
            out,
            "Audit scorecard: {} {} {:?}",
            scorecard.firehorse_scenario_id, scorecard.viewport, scorecard.gate
        )
        .map_err(|error| LabRunError::Io(error.to_string()))?;
        for result in scorecard
            .functional
            .iter()
            .chain(scorecard.aesthetic.iter())
        {
            writeln!(
                out,
                "- {} {:?}: {}",
                result.criterion_id, result.status, result.rationale
            )
            .map_err(|error| LabRunError::Io(error.to_string()))?;
        }
    }
    Ok(outcome)
}

fn write_batch<W>(
    selection: &LabCliSelection,
    audit_registry: &UxAuditRegistry,
    lab_registry: &LabScenarioRegistry<'_>,
    out: &mut W,
) -> Result<LabRunOutcome, LabRunError>
where
    W: Write,
{
    let path = selection
        .batch
        .as_ref()
        .ok_or(LabRunError::MissingValue { flag: "--batch" })?;
    let raw = fs::read_to_string(path).map_err(|error| LabRunError::Io(error.to_string()))?;
    let run: UxAuditRunInput =
        serde_json::from_str(&raw).map_err(|error| LabRunError::Io(error.to_string()))?;
    let suite = audit_registry
        .suite(&run.suite_id)
        .ok_or_else(|| unknown_audit_suite(&run.suite_id, audit_registry))?;
    let mut scorecards = Vec::new();
    let modes = evaluation_modes_from_values(run.evaluation.iter().map(String::as_str), "--batch")?;

    for scenario_id in &run.scenario_ids {
        for viewport in &run.viewports {
            let viewport =
                ViewportClass::parse(viewport).ok_or_else(|| LabRunError::UnknownViewport {
                    value: viewport.clone(),
                })?;
            scorecards.push(filter_scorecard_by_modes(
                scorecard_for(suite, scenario_id, viewport, lab_registry)?,
                &modes,
            ));
        }
    }

    let gate = batch_gate(&scorecards);
    let outcome = outcome_for_gate(gate);
    let batch_output = write_batch_outputs(&run, &scorecards, gate, suite)?;
    let result = UxAuditBatchResult {
        run,
        scorecards,
        gate,
        output_root: batch_output.root,
        files_written: batch_output.files_written,
    };

    if selection.json {
        write_json(out, "audit_batch", result)
    } else {
        writeln!(
            out,
            "Audit batch: {} scorecards {:?}",
            result.scorecards.len(),
            result.gate
        )
        .map_err(|error| LabRunError::Io(error.to_string()))
    }?;
    Ok(outcome)
}

#[derive(Debug, Default)]
struct BatchOutput {
    root: Option<String>,
    files_written: Vec<String>,
}

fn write_batch_outputs(
    run: &UxAuditRunInput,
    scorecards: &[UxAuditScorecard],
    gate: AuditGate,
    suite: &UxAuditSuite,
) -> Result<BatchOutput, LabRunError> {
    let Some(root) = run.output_root.as_deref() else {
        return Ok(BatchOutput::default());
    };
    let requested_root = PathBuf::from(root);
    if !is_audit_export_root(&requested_root) {
        return Err(LabRunError::Io(format!(
            "refusing audit batch output outside docs/firehorse_mockups/ux_audit_lab or target: {}",
            requested_root.display()
        )));
    }

    let export_root = requested_root
        .join("batch_runs")
        .join(batch_run_directory_name(run));
    let scorecard_dir = export_root.join("scorecards");
    let brief_dir = export_root.join("agent_briefs");
    fs::create_dir_all(&scorecard_dir).map_err(|error| LabRunError::Io(error.to_string()))?;
    fs::create_dir_all(&brief_dir).map_err(|error| LabRunError::Io(error.to_string()))?;

    let mut files = vec![
        (
            export_root.join("README.md"),
            batch_readme(run, scorecards.len(), gate),
        ),
        (
            export_root.join("audit_run.json"),
            json_string("audit_run", run)?,
        ),
        (
            export_root.join("scorecards.json"),
            json_string("audit_scorecards", scorecards)?,
        ),
    ];

    for scorecard in scorecards {
        let stem = batch_scorecard_stem(scorecard);
        files.push((
            scorecard_dir.join(format!("{stem}.json")),
            json_string("audit_scorecard", scorecard)?,
        ));
        let brief = design_brief_for(suite, &scorecard.firehorse_scenario_id, &scorecard.viewport)
            .expect("batch scorecard should have a matching design brief");
        files.push((
            brief_dir.join(format!("{stem}.json")),
            json_string("audit_design_brief", &brief)?,
        ));
    }

    for (path, _) in &files {
        refuse_existing_file(path)?;
    }

    let mut files_written = Vec::new();
    for (path, contents) in files {
        fs::write(&path, contents).map_err(|error| LabRunError::Io(error.to_string()))?;
        files_written.push(path.to_string_lossy().to_string());
    }

    Ok(BatchOutput {
        root: Some(export_root.to_string_lossy().to_string()),
        files_written,
    })
}

fn batch_readme(run: &UxAuditRunInput, scorecard_count: usize, gate: AuditGate) -> String {
    format!(
        "# Fire Horse UX Audit Batch\n\nSuite: `{}`  \nScorecards: `{}`  \nGate: `{:?}`\n\nThis is a local automation evidence pack. It contains parseable scorecards and agent briefs, and it does not create public posts or external issues.\n",
        run.suite_id, scorecard_count, gate
    )
}

fn batch_run_directory_name(run: &UxAuditRunInput) -> String {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!(
        "{}_{}_{:09}_{}",
        sanitize_file_stem(&run.suite_id),
        duration.as_secs(),
        duration.subsec_nanos(),
        process::id()
    )
}

fn batch_scorecard_stem(scorecard: &UxAuditScorecard) -> String {
    sanitize_file_stem(&format!(
        "{}_{}",
        scorecard.firehorse_scenario_id, scorecard.viewport
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

fn refuse_existing_file(path: &Path) -> Result<(), LabRunError> {
    if path.exists() {
        Err(LabRunError::Io(format!(
            "refusing to overwrite existing audit batch file {}",
            path.display()
        )))
    } else {
        Ok(())
    }
}

fn write_export<W>(
    selection: &LabCliSelection,
    audit_registry: &UxAuditRegistry,
    lab_registry: &LabScenarioRegistry<'_>,
    out: &mut W,
) -> Result<LabRunOutcome, LabRunError>
where
    W: Write,
{
    let root = selection
        .export
        .as_ref()
        .map(PathBuf::from)
        .ok_or(LabRunError::MissingValue { flag: "--export" })?;
    let export_root = export_root_from_cli_path(&root);
    let result = if selection.scenario.is_none() {
        let suite_id = selection.suite.as_deref().unwrap_or("firehorse");
        let suite = audit_registry
            .suite(suite_id)
            .ok_or_else(|| unknown_audit_suite(suite_id, audit_registry))?;
        export_suite_review_pack(suite, lab_registry, &export_root)?
    } else {
        let model = AuditLabModel::new(selection)?;
        export_current_review_pack(&model, &export_root)?
    };
    let outcome = outcome_for_gate(result.gate);

    if selection.json {
        write_json(out, "audit_export", result)?;
    } else {
        writeln!(
            out,
            "Audit export: {} files written to {} ({:?})",
            result.files_written.len(),
            result.root,
            result.gate
        )
        .map_err(|error| LabRunError::Io(error.to_string()))?;
        for file in &result.files_written {
            writeln!(out, "- {file}").map_err(|error| LabRunError::Io(error.to_string()))?;
        }
    }
    Ok(outcome)
}

fn write_cockpit_once<W>(selection: &LabCliSelection, out: &mut W) -> Result<(), LabRunError>
where
    W: Write,
{
    let model = AuditLabModel::new(selection)?;
    let text = model.render_text()?;
    write!(out, "{text}").map_err(|error| LabRunError::Io(error.to_string()))
}

fn export_root_from_cli_path(path: &std::path::Path) -> PathBuf {
    if path.extension().and_then(|extension| extension.to_str()) == Some("md") {
        path.with_extension("")
    } else {
        path.to_path_buf()
    }
}

fn criteria_by_mode(suite: &UxAuditSuite, mode: EvaluationMode) -> Vec<UxAuditCriterion> {
    suite
        .criteria
        .iter()
        .filter(|criterion| criterion.evaluation_mode == mode)
        .cloned()
        .collect()
}

fn selected_suites<'a>(
    selection: &LabCliSelection,
    audit_registry: &'a UxAuditRegistry,
) -> Result<Vec<&'a UxAuditSuite>, LabRunError> {
    if let Some(suite) = selection.suite.as_deref() {
        let suite = audit_registry
            .suite(suite)
            .ok_or_else(|| unknown_audit_suite(suite, audit_registry))?;
        Ok(vec![suite])
    } else {
        Ok(audit_registry.suites().iter().collect())
    }
}

fn required_suite(selection: &LabCliSelection) -> Result<String, LabRunError> {
    selection.suite.clone().ok_or(LabRunError::MissingSuite)
}

fn required_scenario(selection: &LabCliSelection) -> Result<String, LabRunError> {
    selection
        .scenario
        .clone()
        .ok_or(LabRunError::MissingScenario)
}

fn viewport_name(
    selection: &LabCliSelection,
    suite: &UxAuditSuite,
    scenario_id: &str,
) -> Result<String, LabRunError> {
    if let Some(viewport) = selection.viewport {
        Ok(viewport.name().to_string())
    } else {
        suite
            .find_scenario(scenario_id)
            .map(|scenario| scenario.default_viewport.to_string())
            .ok_or_else(|| {
                unknown_audit_scenario(suite.id, scenario_id, suite, &UxAuditRegistry::built_in())
            })
    }
}

fn viewport_for(
    selection: &LabCliSelection,
    suite: &UxAuditSuite,
    scenario_id: &str,
) -> Result<ViewportClass, LabRunError> {
    if let Some(viewport) = selection.viewport {
        Ok(viewport)
    } else {
        let name = suite
            .find_scenario(scenario_id)
            .map(|scenario| scenario.default_viewport)
            .ok_or_else(|| {
                unknown_audit_scenario(suite.id, scenario_id, suite, &UxAuditRegistry::built_in())
            })?;
        ViewportClass::parse(name).ok_or_else(|| LabRunError::UnknownViewport {
            value: name.to_string(),
        })
    }
}

fn unknown_audit_suite(suite: &str, audit_registry: &UxAuditRegistry) -> LabRunError {
    LabRunError::UnknownSuite {
        suite: suite.to_string(),
        available: audit_registry
            .suites()
            .iter()
            .flat_map(|suite| {
                suite.scenarios.iter().map(|scenario| {
                    format!(
                        "{}/{} -> {}",
                        suite.id, scenario.id, scenario.firehorse_scenario_id
                    )
                })
            })
            .collect(),
    }
}

fn unknown_audit_scenario(
    suite_id: &str,
    scenario_id: &str,
    suite: &UxAuditSuite,
    _audit_registry: &UxAuditRegistry,
) -> LabRunError {
    LabRunError::UnknownScenario {
        suite: suite_id.to_string(),
        id: scenario_id.to_string(),
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
    }
}

fn write_json<W, T>(out: &mut W, kind: &'static str, data: T) -> Result<(), LabRunError>
where
    W: Write,
    T: Serialize,
{
    serde_json::to_writer_pretty(&mut *out, &AuditJsonEnvelope::new(kind, data))
        .map_err(|error| LabRunError::Io(error.to_string()))?;
    writeln!(out).map_err(|error| LabRunError::Io(error.to_string()))
}

fn json_string<T>(kind: &'static str, data: T) -> Result<String, LabRunError>
where
    T: Serialize,
{
    serde_json::to_string_pretty(&AuditJsonEnvelope::new(kind, data))
        .map_err(|error| LabRunError::Io(error.to_string()))
        .map(|mut value| {
            value.push('\n');
            value
        })
}

#[allow(dead_code)]
fn _viewport_name(viewport: ViewportClass) -> &'static str {
    viewport.name()
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::shell::uxlab::{LabRunOutcome, LabScenarioRegistry};

    #[test]
    fn audit_list_json_is_parseable_and_names_firehorse_suite() {
        let registry = LabScenarioRegistry::built_in();
        let mut output = Vec::new();

        crate::shell::uxlab::run_cli(
            ["--audit", "--suite", "firehorse", "--list", "--json"]
                .into_iter()
                .map(String::from),
            &registry,
            &mut output,
        )
        .expect("audit list json should render");

        let parsed: Value = serde_json::from_slice(&output).expect("parse json");
        assert_eq!(parsed["schema_version"], 1);
        assert_eq!(parsed["kind"], "audit_suite_list");
        assert_eq!(parsed["data"][0]["id"], "firehorse");
    }

    #[test]
    fn audit_brief_json_contains_agent_work_packet() {
        let registry = LabScenarioRegistry::built_in();
        let mut output = Vec::new();

        crate::shell::uxlab::run_cli(
            [
                "--audit",
                "--suite",
                "firehorse",
                "--scenario",
                "firehorse-editing-lens-standard",
                "--viewport",
                "studio",
                "--brief",
                "--json",
            ]
            .into_iter()
            .map(String::from),
            &registry,
            &mut output,
        )
        .expect("audit brief json should render");

        let parsed: Value = serde_json::from_slice(&output).expect("parse json");
        assert_eq!(parsed["kind"], "audit_design_brief");
        assert_eq!(parsed["data"]["scenario_id"], "audit-editing-lens-pricing");
        assert!(
            parsed["data"]["must_preserve"]
                .as_array()
                .expect("must preserve array")
                .iter()
                .any(|value| value.as_str()
                    == Some("source canvas remains the primary visual object"))
        );
    }

    #[test]
    fn audit_once_json_contains_dossier_without_ansi_escape_bytes() {
        let registry = LabScenarioRegistry::built_in();
        let mut output = Vec::new();

        crate::shell::uxlab::run_cli(
            [
                "--audit",
                "--suite",
                "firehorse",
                "--scenario",
                "firehorse-editing-lens-standard",
                "--viewport",
                "studio",
                "--once",
                "--json",
            ]
            .into_iter()
            .map(String::from),
            &registry,
            &mut output,
        )
        .expect("audit dossier json should render");

        assert!(
            !output.windows(2).any(|window| window == [0x1b, b'[']),
            "json dossier must not contain ANSI escapes"
        );
        let parsed: Value = serde_json::from_slice(&output).expect("parse json");
        assert_eq!(parsed["kind"], "audit_dossier");
        assert_eq!(
            parsed["data"]["scenario"]["firehorse_scenario_id"],
            "firehorse-editing-lens-standard"
        );
        assert!(
            parsed["data"]["functional_criteria"]
                .as_array()
                .expect("functional criteria")
                .iter()
                .any(|criterion| criterion["id"] == "functional.seam_honesty")
        );
    }

    #[test]
    fn audit_matrix_json_includes_downstream_owners() {
        let registry = LabScenarioRegistry::built_in();
        let mut output = Vec::new();

        crate::shell::uxlab::run_cli(
            ["--audit", "--suite", "firehorse", "--matrix", "--json"]
                .into_iter()
                .map(String::from),
            &registry,
            &mut output,
        )
        .expect("audit matrix json should render");

        let parsed: Value = serde_json::from_slice(&output).expect("parse json");
        assert_eq!(parsed["kind"], "audit_matrix");
        assert!(
            parsed["data"]
                .as_array()
                .expect("matrix rows")
                .iter()
                .any(|row| row["downstream_worksets"]
                    .as_array()
                    .expect("owners")
                    .iter()
                    .any(|owner| owner == "W060"))
        );
    }

    #[test]
    fn audit_evaluate_json_filters_modes_and_reports_gate() {
        let registry = LabScenarioRegistry::built_in();
        let mut output = Vec::new();

        let outcome = crate::shell::uxlab::run_cli_with_outcome(
            [
                "--audit",
                "--suite",
                "firehorse",
                "--scenario",
                "firehorse-editing-lens-standard",
                "--viewport",
                "studio",
                "--evaluate",
                "functional,aesthetic",
                "--json",
            ]
            .into_iter()
            .map(String::from),
            &registry,
            &mut output,
        )
        .expect("audit scorecard json should render");

        assert_eq!(outcome, LabRunOutcome::AuditGateConcern);
        assert!(
            !output.windows(2).any(|window| window == [0x1b, b'[']),
            "json scorecard must not contain ANSI escapes"
        );
        let parsed: Value = serde_json::from_slice(&output).expect("parse json");
        assert_eq!(parsed["kind"], "audit_scorecard");
        assert_eq!(parsed["data"]["gate"], "concern");
        assert!(
            parsed["data"]["functional"]
                .as_array()
                .expect("functional scorecard")
                .iter()
                .any(|result| result["criterion_id"] == "functional.persona_fit")
        );
        assert!(
            parsed["data"]["aesthetic"]
                .as_array()
                .expect("aesthetic scorecard")
                .iter()
                .any(|result| result["criterion_id"] == "aesthetic.emotional_fit")
        );
    }

    #[test]
    fn audit_evaluate_functional_only_can_pass_without_aesthetic_gate() {
        let registry = LabScenarioRegistry::built_in();
        let mut output = Vec::new();

        let outcome = crate::shell::uxlab::run_cli_with_outcome(
            [
                "--audit",
                "--suite",
                "firehorse",
                "--scenario",
                "firehorse-editing-lens-standard",
                "--viewport",
                "studio",
                "--evaluate",
                "functional",
                "--json",
            ]
            .into_iter()
            .map(String::from),
            &registry,
            &mut output,
        )
        .expect("audit scorecard json should render");

        assert_eq!(outcome, LabRunOutcome::Success);
        let parsed: Value = serde_json::from_slice(&output).expect("parse json");
        assert_eq!(parsed["data"]["gate"], "ready");
        assert!(
            !parsed["data"]["functional"]
                .as_array()
                .expect("functional scorecard")
                .is_empty()
        );
        assert!(
            parsed["data"]["aesthetic"]
                .as_array()
                .expect("aesthetic scorecard")
                .is_empty()
        );
    }

    #[test]
    fn audit_batch_json_runs_fixture_for_all_selected_pairs() {
        let registry = LabScenarioRegistry::built_in();
        let mut output = Vec::new();

        let outcome = crate::shell::uxlab::run_cli_with_outcome(
            [
                "--audit",
                "--batch",
                "docs/firehorse_mockups/ux_audit_lab/agent_run.json",
                "--json",
            ]
            .into_iter()
            .map(String::from),
            &registry,
            &mut output,
        )
        .expect("audit batch json should render");

        assert_eq!(outcome, LabRunOutcome::AuditGateConcern);
        let parsed: Value = serde_json::from_slice(&output).expect("parse json");
        assert_eq!(parsed["kind"], "audit_batch");
        assert_eq!(parsed["data"]["gate"], "concern");
        assert_eq!(
            parsed["data"]["scorecards"]
                .as_array()
                .expect("scorecards")
                .len(),
            14
        );
        assert_eq!(
            parsed["data"]["run"]["output_root"],
            "target/ux_audit_lab/agent_run"
        );
        let output_root = parsed["data"]["output_root"]
            .as_str()
            .expect("batch output root")
            .replace('\\', "/");
        assert!(output_root.contains("target/ux_audit_lab/agent_run/batch_runs"));
        assert!(
            parsed["data"]["files_written"]
                .as_array()
                .expect("files written")
                .iter()
                .any(|value| value
                    .as_str()
                    .is_some_and(|path| path.ends_with("scorecards.json")))
        );
    }
}
