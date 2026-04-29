use serde::{Deserialize, Serialize};

pub const AUDIT_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UxAuditSuite {
    pub schema_version: u32,
    pub id: &'static str,
    pub title: &'static str,
    pub personas: Vec<UxPersona>,
    pub scenarios: Vec<UxAuditScenario>,
    pub criteria: Vec<UxAuditCriterion>,
    pub rubrics: Vec<UxAuditRubric>,
}

impl UxAuditSuite {
    pub fn find_scenario(&self, id: &str) -> Option<&UxAuditScenario> {
        self.scenarios
            .iter()
            .find(|scenario| scenario.id == id || scenario.firehorse_scenario_id == id)
    }

    pub fn find_persona(&self, id: &str) -> Option<&UxPersona> {
        self.personas.iter().find(|persona| persona.id == id)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UxPersona {
    pub id: &'static str,
    pub title: &'static str,
    pub role: &'static str,
    pub job_pressure: &'static str,
    pub goals: Vec<&'static str>,
    pub constraints: Vec<&'static str>,
    pub delight_target: &'static str,
    pub failure_modes: Vec<&'static str>,
    pub source_refs: Vec<AuditArtifactRef>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UxAuditScenario {
    pub id: &'static str,
    pub firehorse_scenario_id: &'static str,
    pub persona_id: &'static str,
    pub title: &'static str,
    pub intent: &'static str,
    pub default_viewport: &'static str,
    pub steps: Vec<UxJourneyStep>,
    pub reference_artifacts: Vec<AuditArtifactRef>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UxJourneyStep {
    pub id: &'static str,
    pub title: &'static str,
    pub user_intent: &'static str,
    pub expected_surfaces: Vec<UxSurfaceExpectation>,
    pub expected_actions: Vec<&'static str>,
    pub state_refs: Vec<UxStateRef>,
    pub seam_refs: Vec<UxSeamRef>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UxSurfaceExpectation {
    pub surface: &'static str,
    pub projection_path: &'static str,
    pub visible_contract: &'static str,
    pub owner_workset: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UxStateRef {
    pub owner: &'static str,
    pub field: &'static str,
    pub downstream_workset: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UxSeamRef {
    pub source: &'static str,
    pub status: SeamStatus,
    pub downstream_workset: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SeamStatus {
    Real,
    Future,
    Unavailable,
    NotRequired,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UxAuditCriterion {
    pub id: &'static str,
    pub category: AuditCategory,
    pub question: &'static str,
    pub severity_if_failed: AuditSeverity,
    pub evidence_required: &'static str,
    pub evaluation_mode: EvaluationMode,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditCategory {
    PersonaFit,
    JourneyFit,
    CommandClarity,
    StateOwnership,
    SeamHonesty,
    Degradation,
    Hierarchy,
    Density,
    Balance,
    ToneAndColor,
    TerminalCraft,
    ReferenceFidelity,
    TextFit,
    EmotionalFit,
}

impl AuditCategory {
    pub const fn name(self) -> &'static str {
        match self {
            Self::PersonaFit => "persona_fit",
            Self::JourneyFit => "journey_fit",
            Self::CommandClarity => "command_clarity",
            Self::StateOwnership => "state_ownership",
            Self::SeamHonesty => "seam_honesty",
            Self::Degradation => "degradation",
            Self::Hierarchy => "hierarchy",
            Self::Density => "density",
            Self::Balance => "balance",
            Self::ToneAndColor => "tone_and_color",
            Self::TerminalCraft => "terminal_craft",
            Self::ReferenceFidelity => "reference_fidelity",
            Self::TextFit => "text_fit",
            Self::EmotionalFit => "emotional_fit",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditSeverity {
    Concern,
    Fail,
    Blocker,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvaluationMode {
    Functional,
    Aesthetic,
    Mixed,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UxAuditRubric {
    pub id: &'static str,
    pub title: &'static str,
    pub evaluation_mode: EvaluationMode,
    pub criteria: Vec<&'static str>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AuditArtifactRef {
    pub kind: &'static str,
    pub path: &'static str,
    pub title: &'static str,
    pub authority: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UxDesignBrief {
    pub scenario_id: &'static str,
    pub firehorse_scenario_id: &'static str,
    pub viewport: String,
    pub persona_id: &'static str,
    pub design_intent: &'static str,
    pub aesthetic_target: &'static str,
    pub must_preserve: Vec<&'static str>,
    pub likely_files: Vec<&'static str>,
    pub render_commands: Vec<String>,
    pub evaluation_commands: Vec<String>,
    pub reference_artifacts: Vec<AuditArtifactRef>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UxAuditDossier {
    pub suite_id: &'static str,
    pub scenario: UxAuditScenario,
    pub persona: UxPersona,
    pub viewport: String,
    pub functional_criteria: Vec<UxAuditCriterion>,
    pub aesthetic_criteria: Vec<UxAuditCriterion>,
    pub render_commands: Vec<String>,
    pub reference_artifacts: Vec<AuditArtifactRef>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UxAuditMatrixRow {
    pub audit_scenario_id: &'static str,
    pub firehorse_scenario_id: &'static str,
    pub persona_id: &'static str,
    pub default_viewport: &'static str,
    pub criteria: Vec<&'static str>,
    pub downstream_worksets: Vec<&'static str>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct UxAuditRunInput {
    pub suite_id: String,
    pub scenario_ids: Vec<String>,
    pub viewports: Vec<String>,
    pub evaluation: Vec<String>,
    pub output_root: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UxAuditScorecard {
    pub run_id: String,
    pub suite_id: String,
    pub scenario_id: String,
    pub firehorse_scenario_id: String,
    pub viewport: String,
    pub render_mode: &'static str,
    pub functional: Vec<UxCriterionResult>,
    pub aesthetic: Vec<UxCriterionResult>,
    pub objective_preflight: UxObjectivePreflight,
    pub gate: AuditGate,
    pub artifacts: Vec<AuditArtifactRef>,
    pub reproduction_commands: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UxAuditBatchResult {
    pub run: UxAuditRunInput,
    pub scorecards: Vec<UxAuditScorecard>,
    pub gate: AuditGate,
    pub output_root: Option<String>,
    pub files_written: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UxAuditFinding {
    pub scenario_id: String,
    pub criterion_id: String,
    pub status: AuditStatus,
    pub confidence: AuditConfidence,
    pub rationale: String,
    pub downstream_owner: Option<String>,
    pub evidence: Vec<AuditArtifactRef>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UxCriterionResult {
    pub criterion_id: String,
    pub status: AuditStatus,
    pub confidence: AuditConfidence,
    pub rationale: String,
    pub evidence: Vec<AuditArtifactRef>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UxObjectivePreflight {
    pub width: u16,
    pub height: u16,
    pub non_empty_lines: usize,
    pub dense_lines: usize,
    pub max_line_width: usize,
    pub has_ansi_stream: bool,
    pub has_reference_image: bool,
    pub has_terminal_capture: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditStatus {
    Pass,
    Concern,
    Fail,
    Deferred,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditConfidence {
    Objective,
    StructuredJudgement,
    ManualRequired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditGate {
    Ready,
    Concern,
    Blocked,
}
