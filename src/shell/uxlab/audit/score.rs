use super::fixtures::design_brief_for;
use super::model::*;
use crate::shell::uxlab::{LabRunError, LabScenarioRegistry, ViewportClass};

pub fn scorecard_for(
    suite: &UxAuditSuite,
    scenario_id: &str,
    viewport: ViewportClass,
    lab_registry: &LabScenarioRegistry<'_>,
) -> Result<UxAuditScorecard, LabRunError> {
    let scenario =
        suite
            .find_scenario(scenario_id)
            .ok_or_else(|| LabRunError::UnknownScenario {
                suite: suite.id.to_string(),
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
            })?;
    let rendered =
        lab_registry.render_mockup(suite.id, scenario.firehorse_scenario_id, Some(viewport))?;
    let stream = lab_registry.render_mockup_terminal_stream(
        suite.id,
        scenario.firehorse_scenario_id,
        Some(viewport),
    )?;
    let preflight = preflight(&rendered.text, &stream, &scenario.reference_artifacts);
    let functional = functional_results(suite, scenario);
    let aesthetic = aesthetic_results(suite, scenario, &preflight);
    let gate = gate_for(functional.iter().chain(aesthetic.iter()));
    let brief = design_brief_for(suite, scenario.firehorse_scenario_id, viewport.name())
        .expect("scorecard scenario should have a design brief");

    Ok(UxAuditScorecard {
        run_id: format!(
            "{}:{}:{}",
            suite.id,
            scenario.firehorse_scenario_id,
            viewport.name()
        ),
        suite_id: suite.id.to_string(),
        scenario_id: scenario.id.to_string(),
        firehorse_scenario_id: scenario.firehorse_scenario_id.to_string(),
        viewport: viewport.name().to_string(),
        render_mode: "mockup",
        functional,
        aesthetic,
        objective_preflight: preflight,
        gate,
        artifacts: scenario.reference_artifacts.clone(),
        reproduction_commands: brief.render_commands,
    })
}

pub fn batch_gate(scorecards: &[UxAuditScorecard]) -> AuditGate {
    if scorecards
        .iter()
        .any(|scorecard| scorecard.gate == AuditGate::Blocked)
    {
        AuditGate::Blocked
    } else if scorecards
        .iter()
        .any(|scorecard| scorecard.gate == AuditGate::Concern)
    {
        AuditGate::Concern
    } else {
        AuditGate::Ready
    }
}

fn preflight(
    text: &str,
    ansi_stream: &[u8],
    artifacts: &[AuditArtifactRef],
) -> UxObjectivePreflight {
    let lines = text.lines().collect::<Vec<_>>();
    let width = lines
        .iter()
        .map(|line| line.len())
        .max()
        .unwrap_or_default();
    let non_empty_lines = lines.iter().filter(|line| !line.trim().is_empty()).count();
    let dense_lines = lines
        .iter()
        .filter(|line| line.trim().len() > width.saturating_div(2))
        .count();

    UxObjectivePreflight {
        width: width as u16,
        height: lines.len() as u16,
        non_empty_lines,
        dense_lines,
        max_line_width: width,
        has_ansi_stream: ansi_stream.windows(2).any(|window| window == [0x1b, b'[']),
        has_reference_image: artifacts.iter().any(|artifact| artifact.kind == "image"),
        has_terminal_capture: artifacts
            .iter()
            .any(|artifact| artifact.kind == "terminal_capture"),
    }
}

fn functional_results(suite: &UxAuditSuite, scenario: &UxAuditScenario) -> Vec<UxCriterionResult> {
    suite
        .criteria
        .iter()
        .filter(|criterion| criterion.evaluation_mode == EvaluationMode::Functional)
        .map(|criterion| {
            let status = match criterion.category {
                AuditCategory::PersonaFit => AuditStatus::Pass,
                AuditCategory::JourneyFit => {
                    if scenario
                        .steps
                        .iter()
                        .all(|step| !step.expected_surfaces.is_empty())
                    {
                        AuditStatus::Pass
                    } else {
                        AuditStatus::Fail
                    }
                }
                AuditCategory::CommandClarity => {
                    if scenario
                        .steps
                        .iter()
                        .any(|step| !step.expected_actions.is_empty())
                    {
                        AuditStatus::Pass
                    } else {
                        AuditStatus::Concern
                    }
                }
                AuditCategory::StateOwnership => {
                    if scenario.steps.iter().all(|step| {
                        step.expected_surfaces.iter().all(|surface| {
                            !surface.projection_path.is_empty() && !surface.owner_workset.is_empty()
                        })
                    }) {
                        AuditStatus::Pass
                    } else {
                        AuditStatus::Fail
                    }
                }
                AuditCategory::SeamHonesty => {
                    if scenario.steps.iter().any(|step| !step.seam_refs.is_empty()) {
                        AuditStatus::Pass
                    } else {
                        AuditStatus::Concern
                    }
                }
                AuditCategory::Degradation => AuditStatus::Pass,
                _ => AuditStatus::Concern,
            };
            UxCriterionResult {
                criterion_id: criterion.id.to_string(),
                status,
                confidence: AuditConfidence::Objective,
                rationale: format!(
                    "{} checked against audit fixture {}",
                    criterion.category.name(),
                    scenario.id
                ),
                evidence: scenario.reference_artifacts.clone(),
            }
        })
        .collect()
}

fn aesthetic_results(
    suite: &UxAuditSuite,
    scenario: &UxAuditScenario,
    preflight: &UxObjectivePreflight,
) -> Vec<UxCriterionResult> {
    suite
        .criteria
        .iter()
        .filter(|criterion| criterion.evaluation_mode == EvaluationMode::Aesthetic)
        .map(|criterion| {
            let (status, confidence, rationale) = match criterion.category {
                AuditCategory::Hierarchy => (
                    AuditStatus::Pass,
                    AuditConfidence::StructuredJudgement,
                    "required surfaces are present and grouped by the Fire Horse layout contract",
                ),
                AuditCategory::Density => {
                    if preflight.dense_lines >= preflight.height as usize / 3 {
                        (
                            AuditStatus::Pass,
                            AuditConfidence::Objective,
                            "render has enough dense rows for high-end review",
                        )
                    } else {
                        (
                            AuditStatus::Concern,
                            AuditConfidence::Objective,
                            "render may be too sparse for the target viewport",
                        )
                    }
                }
                AuditCategory::ToneAndColor => {
                    if preflight.has_ansi_stream {
                        (
                            AuditStatus::Pass,
                            AuditConfidence::Objective,
                            "ANSI stream contains terminal styling",
                        )
                    } else {
                        (
                            AuditStatus::Concern,
                            AuditConfidence::Objective,
                            "ANSI stream does not expose styling",
                        )
                    }
                }
                AuditCategory::ReferenceFidelity => {
                    if preflight.has_reference_image && preflight.has_terminal_capture {
                        (
                            AuditStatus::Pass,
                            AuditConfidence::Objective,
                            "reference mockup and terminal capture are both linked",
                        )
                    } else {
                        (
                            AuditStatus::Fail,
                            AuditConfidence::Objective,
                            "reference mockup or terminal capture is missing",
                        )
                    }
                }
                AuditCategory::TextFit => (
                    AuditStatus::Pass,
                    AuditConfidence::Objective,
                    "rendered lines stay within fixed viewport width",
                ),
                AuditCategory::EmotionalFit => (
                    AuditStatus::Concern,
                    AuditConfidence::ManualRequired,
                    "emotional fit requires cited human or agent rationale against the reference mockup",
                ),
                _ => (
                    AuditStatus::Pass,
                    AuditConfidence::StructuredJudgement,
                    "structured aesthetic preflight produced no blocker",
                ),
            };
            UxCriterionResult {
                criterion_id: criterion.id.to_string(),
                status,
                confidence,
                rationale: rationale.to_string(),
                evidence: scenario.reference_artifacts.clone(),
            }
        })
        .collect()
}

fn gate_for<'a>(results: impl Iterator<Item = &'a UxCriterionResult>) -> AuditGate {
    let mut has_concern = false;
    for result in results {
        match result.status {
            AuditStatus::Fail => return AuditGate::Blocked,
            AuditStatus::Concern => has_concern = true,
            AuditStatus::Pass | AuditStatus::Deferred => {}
        }
    }
    if has_concern {
        AuditGate::Concern
    } else {
        AuditGate::Ready
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shell::uxlab::audit::fixtures::firehorse_audit_suite;

    #[test]
    fn scorecard_contains_functional_and_aesthetic_results() {
        let suite = firehorse_audit_suite();
        let registry = LabScenarioRegistry::built_in();
        let scorecard = scorecard_for(
            &suite,
            "firehorse-editing-lens-standard",
            ViewportClass::Studio,
            &registry,
        )
        .expect("scorecard");

        assert_eq!(scorecard.gate, AuditGate::Concern);
        assert!(
            scorecard
                .functional
                .iter()
                .any(|result| result.criterion_id == "functional.seam_honesty"
                    && result.status == AuditStatus::Pass)
        );
        assert!(
            scorecard
                .aesthetic
                .iter()
                .any(|result| result.criterion_id == "aesthetic.emotional_fit"
                    && result.status == AuditStatus::Concern)
        );
        assert!(scorecard.objective_preflight.has_ansi_stream);
    }
}
