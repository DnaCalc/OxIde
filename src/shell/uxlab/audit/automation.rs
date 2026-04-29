use super::model::{AuditGate, AuditStatus, EvaluationMode, UxAuditScorecard};
use crate::shell::uxlab::{LabCliSelection, LabRunError, LabRunOutcome};

pub fn requested_evaluation_modes(
    selection: &LabCliSelection,
) -> Result<Vec<EvaluationMode>, LabRunError> {
    let value = selection
        .evaluate
        .as_deref()
        .ok_or(LabRunError::MissingValue { flag: "--evaluate" })?;
    evaluation_modes_from_values([value], "--evaluate")
}

pub fn evaluation_modes_from_values<'a>(
    values: impl IntoIterator<Item = &'a str>,
    flag: &'static str,
) -> Result<Vec<EvaluationMode>, LabRunError> {
    let mut modes = Vec::new();
    for value in values {
        for token in value.split(',') {
            let token = token.trim();
            if token.is_empty() {
                continue;
            }
            match token {
                "all" => {
                    push_mode(&mut modes, EvaluationMode::Functional);
                    push_mode(&mut modes, EvaluationMode::Aesthetic);
                }
                "functional" => push_mode(&mut modes, EvaluationMode::Functional),
                "aesthetic" => push_mode(&mut modes, EvaluationMode::Aesthetic),
                "mixed" => {
                    push_mode(&mut modes, EvaluationMode::Functional);
                    push_mode(&mut modes, EvaluationMode::Aesthetic);
                }
                _ => {
                    return Err(LabRunError::UnknownArgument {
                        value: format!(
                            "{flag} evaluation '{token}' is not one of functional,aesthetic,all"
                        ),
                    });
                }
            }
        }
    }
    if modes.is_empty() {
        Err(LabRunError::MissingValue { flag })
    } else {
        Ok(modes)
    }
}

pub fn filter_scorecard_by_modes(
    mut scorecard: UxAuditScorecard,
    modes: &[EvaluationMode],
) -> UxAuditScorecard {
    if !modes.contains(&EvaluationMode::Functional) {
        scorecard.functional.clear();
    }
    if !modes.contains(&EvaluationMode::Aesthetic) {
        scorecard.aesthetic.clear();
    }
    scorecard.gate = gate_for_scorecard(&scorecard);
    scorecard
}

pub fn outcome_for_gate(gate: AuditGate) -> LabRunOutcome {
    match gate {
        AuditGate::Ready => LabRunOutcome::Success,
        AuditGate::Concern => LabRunOutcome::AuditGateConcern,
        AuditGate::Blocked => LabRunOutcome::AuditGateBlocked,
    }
}

fn push_mode(modes: &mut Vec<EvaluationMode>, mode: EvaluationMode) {
    if !modes.contains(&mode) {
        modes.push(mode);
    }
}

fn gate_for_scorecard(scorecard: &UxAuditScorecard) -> AuditGate {
    let mut has_concern = false;
    for result in scorecard
        .functional
        .iter()
        .chain(scorecard.aesthetic.iter())
    {
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

    #[test]
    fn evaluation_mode_parser_accepts_comma_and_list_forms() {
        let modes =
            evaluation_modes_from_values(["functional,aesthetic", "functional"], "--evaluate")
                .expect("valid modes");

        assert_eq!(
            modes,
            vec![EvaluationMode::Functional, EvaluationMode::Aesthetic]
        );
    }

    #[test]
    fn evaluation_mode_parser_rejects_unknown_tokens() {
        let error = evaluation_modes_from_values(["functional,visual"], "--evaluate")
            .expect_err("visual is not a supported mode");

        assert!(error.to_string().contains("visual"));
    }
}
