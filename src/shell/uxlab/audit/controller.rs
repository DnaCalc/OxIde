use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use ftui::{
    KeyEventKind,
    prelude::{Cmd, Event, Frame, KeyCode, KeyEvent, Model},
};

use super::export;
use super::model::{
    AuditConfidence, AuditStatus, UxAuditCriterion, UxAuditFinding, UxAuditScenario, UxAuditSuite,
};
use super::registry::UxAuditRegistry;
use super::view;
use crate::shell::uxlab::{LabCliSelection, LabRunError, LabScenarioRegistry, ViewportClass};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuditFocusRegion {
    Atlas,
    Stage,
    Dossier,
    Evidence,
}

impl AuditFocusRegion {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Atlas => "Scenario Atlas",
            Self::Stage => "Live Fire Horse Stage",
            Self::Dossier => "Audit Dossier",
            Self::Evidence => "Evidence Rail",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DossierMode {
    Persona,
    Journey,
    Mapping,
    Checklist,
    Findings,
}

impl DossierMode {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Persona => "Persona",
            Self::Journey => "Journey",
            Self::Mapping => "Mapping",
            Self::Checklist => "Checklist",
            Self::Findings => "Findings",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuditRenderMode {
    Mockup,
    Contract,
}

impl AuditRenderMode {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Mockup => "mockup",
            Self::Contract => "contract",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuditLabState {
    pub suite_id: String,
    pub scenario_index: usize,
    pub viewport: ViewportClass,
    pub render_mode: AuditRenderMode,
    pub focus: AuditFocusRegion,
    pub dossier_mode: DossierMode,
    pub selected_step_index: usize,
    pub selected_criterion_index: usize,
    pub selected_artifact_index: usize,
    pub findings: Vec<UxAuditFinding>,
    pub status_line: String,
}

impl AuditLabState {
    pub fn new(selection: &LabCliSelection, suite: &UxAuditSuite) -> Result<Self, LabRunError> {
        let scenario_index = if let Some(requested) = selection.scenario.as_deref() {
            suite
                .scenarios
                .iter()
                .position(|scenario| {
                    scenario.id == requested || scenario.firehorse_scenario_id == requested
                })
                .ok_or_else(|| LabRunError::UnknownScenario {
                    suite: suite.id.to_string(),
                    id: requested.to_string(),
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
                })?
        } else {
            suite
                .scenarios
                .iter()
                .position(|scenario| {
                    scenario.firehorse_scenario_id == "firehorse-editing-lens-standard"
                })
                .unwrap_or_default()
        };

        let scenario = &suite.scenarios[scenario_index];
        let viewport = selection
            .viewport
            .or_else(|| ViewportClass::parse(scenario.default_viewport))
            .unwrap_or(ViewportClass::Studio);

        Ok(Self {
            suite_id: suite.id.to_string(),
            scenario_index,
            viewport,
            render_mode: AuditRenderMode::Mockup,
            focus: AuditFocusRegion::Atlas,
            dossier_mode: DossierMode::Persona,
            selected_step_index: 0,
            selected_criterion_index: 0,
            selected_artifact_index: 0,
            findings: Vec::new(),
            status_line:
                "Tab focus  v viewport  r render  1-5 dossier  p/c/f/d mark  e export  q quit"
                    .to_string(),
        })
    }

    pub fn selected_scenario<'a>(&self, suite: &'a UxAuditSuite) -> &'a UxAuditScenario {
        &suite.scenarios[self
            .scenario_index
            .min(suite.scenarios.len().saturating_sub(1))]
    }

    pub fn selected_criterion<'a>(&self, suite: &'a UxAuditSuite) -> Option<&'a UxAuditCriterion> {
        suite.criteria.get(
            self.selected_criterion_index
                .min(suite.criteria.len().saturating_sub(1)),
        )
    }

    pub fn focus_next(&mut self) {
        self.focus = match self.focus {
            AuditFocusRegion::Atlas => AuditFocusRegion::Stage,
            AuditFocusRegion::Stage => AuditFocusRegion::Dossier,
            AuditFocusRegion::Dossier => AuditFocusRegion::Evidence,
            AuditFocusRegion::Evidence => AuditFocusRegion::Atlas,
        };
        self.status_line = format!("Focus: {}", self.focus.label());
    }

    pub fn focus_previous(&mut self) {
        self.focus = match self.focus {
            AuditFocusRegion::Atlas => AuditFocusRegion::Evidence,
            AuditFocusRegion::Stage => AuditFocusRegion::Atlas,
            AuditFocusRegion::Dossier => AuditFocusRegion::Stage,
            AuditFocusRegion::Evidence => AuditFocusRegion::Dossier,
        };
        self.status_line = format!("Focus: {}", self.focus.label());
    }

    pub fn move_selection(&mut self, delta: isize, suite: &UxAuditSuite) {
        match self.focus {
            AuditFocusRegion::Atlas => {
                self.scenario_index =
                    wrapped_index(self.scenario_index, delta, suite.scenarios.len());
                self.selected_step_index = 0;
                self.selected_artifact_index = 0;
                self.status_line = format!(
                    "Selected {}",
                    self.selected_scenario(suite).firehorse_scenario_id
                );
            }
            AuditFocusRegion::Dossier => match self.dossier_mode {
                DossierMode::Journey | DossierMode::Mapping => {
                    let len = self.selected_scenario(suite).steps.len();
                    self.selected_step_index = wrapped_index(self.selected_step_index, delta, len);
                }
                DossierMode::Checklist | DossierMode::Findings => {
                    self.selected_criterion_index =
                        wrapped_index(self.selected_criterion_index, delta, suite.criteria.len());
                }
                DossierMode::Persona => {}
            },
            AuditFocusRegion::Evidence => {
                let len = self.selected_scenario(suite).reference_artifacts.len();
                self.selected_artifact_index =
                    wrapped_index(self.selected_artifact_index, delta, len);
            }
            AuditFocusRegion::Stage => {}
        }
    }

    pub fn set_dossier_mode(&mut self, mode: DossierMode) {
        self.dossier_mode = mode;
        self.status_line = format!("Dossier: {}", mode.label());
    }

    pub fn cycle_viewport(&mut self) {
        self.viewport = match self.viewport {
            ViewportClass::Studio => ViewportClass::FirstClass,
            ViewportClass::FirstClass => ViewportClass::Standard,
            ViewportClass::Standard => ViewportClass::Compact,
            ViewportClass::Compact => ViewportClass::Studio,
            ViewportClass::Wide => ViewportClass::Studio,
        };
        self.status_line = format!("Viewport: {}", self.viewport.name());
    }

    pub fn toggle_render_mode(&mut self) {
        self.render_mode = match self.render_mode {
            AuditRenderMode::Mockup => AuditRenderMode::Contract,
            AuditRenderMode::Contract => AuditRenderMode::Mockup,
        };
        self.status_line = format!("Render mode: {}", self.render_mode.label());
    }

    pub fn mark_current_criterion(&mut self, suite: &UxAuditSuite, status: AuditStatus) {
        let Some(criterion) = self.selected_criterion(suite) else {
            return;
        };
        let scenario = self.selected_scenario(suite);
        let downstream_owner = if status == AuditStatus::Deferred {
            scenario
                .steps
                .iter()
                .flat_map(|step| step.expected_surfaces.iter())
                .next()
                .map(|surface| surface.owner_workset.to_string())
        } else {
            None
        };
        let confidence = match status {
            AuditStatus::Pass | AuditStatus::Concern => AuditConfidence::StructuredJudgement,
            AuditStatus::Fail | AuditStatus::Deferred => AuditConfidence::ManualRequired,
        };
        let rationale = match status {
            AuditStatus::Pass => "Reviewer marked pass from the audit cockpit.",
            AuditStatus::Concern => "Reviewer marked concern for follow-up design review.",
            AuditStatus::Fail => {
                "Reviewer marked fail; downstream implementation should not consume this as ready."
            }
            AuditStatus::Deferred => {
                "Reviewer deferred this criterion to the named downstream owner."
            }
        };

        let finding = UxAuditFinding {
            scenario_id: scenario.id.to_string(),
            criterion_id: criterion.id.to_string(),
            status,
            confidence,
            rationale: rationale.to_string(),
            downstream_owner,
            evidence: scenario.reference_artifacts.clone(),
        };

        if let Some(existing) = self.findings.iter_mut().find(|existing| {
            existing.scenario_id == finding.scenario_id
                && existing.criterion_id == finding.criterion_id
        }) {
            *existing = finding;
        } else {
            self.findings.push(finding);
        }
        self.status_line = format!("Marked {} as {:?}", criterion.id, status);
    }
}

fn wrapped_index(current: usize, delta: isize, len: usize) -> usize {
    if len == 0 {
        return 0;
    }
    let len = len as isize;
    (current as isize + delta).rem_euclid(len) as usize
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditMsg {
    Quit,
    NextFocus,
    PreviousFocus,
    Move(isize),
    Open,
    CycleViewport,
    ToggleRenderMode,
    SetDossierMode(DossierMode),
    Mark(AuditStatus),
    Export,
    Help,
    Noop,
}

impl From<Event> for AuditMsg {
    fn from(event: Event) -> Self {
        match event {
            Event::Key(key) if !is_actionable_key(key) => Self::Noop,
            Event::Key(key) if key.is_char('q') => Self::Quit,
            Event::Key(key) if matches!(key.code, KeyCode::Tab) => Self::NextFocus,
            Event::Key(key) if matches!(key.code, KeyCode::BackTab) => Self::PreviousFocus,
            Event::Key(key) if matches!(key.code, KeyCode::Up) || key.is_char('k') => {
                Self::Move(-1)
            }
            Event::Key(key) if matches!(key.code, KeyCode::Down) || key.is_char('j') => {
                Self::Move(1)
            }
            Event::Key(key) if matches!(key.code, KeyCode::Enter) => Self::Open,
            Event::Key(key) if key.is_char('v') => Self::CycleViewport,
            Event::Key(key) if key.is_char('r') => Self::ToggleRenderMode,
            Event::Key(key) if key.is_char('1') => Self::SetDossierMode(DossierMode::Persona),
            Event::Key(key) if key.is_char('2') => Self::SetDossierMode(DossierMode::Journey),
            Event::Key(key) if key.is_char('3') => Self::SetDossierMode(DossierMode::Mapping),
            Event::Key(key) if key.is_char('4') => Self::SetDossierMode(DossierMode::Checklist),
            Event::Key(key) if key.is_char('5') => Self::SetDossierMode(DossierMode::Findings),
            Event::Key(key) if key.is_char('p') => Self::Mark(AuditStatus::Pass),
            Event::Key(key) if key.is_char('c') => Self::Mark(AuditStatus::Concern),
            Event::Key(key) if key.is_char('f') => Self::Mark(AuditStatus::Fail),
            Event::Key(key) if key.is_char('d') => Self::Mark(AuditStatus::Deferred),
            Event::Key(key) if key.is_char('e') => Self::Export,
            Event::Key(key) if key.is_char('?') => Self::Help,
            _ => Self::Noop,
        }
    }
}

pub struct AuditLabModel {
    audit_registry: UxAuditRegistry,
    lab_registry: LabScenarioRegistry<'static>,
    state: AuditLabState,
}

impl AuditLabModel {
    pub fn new(selection: &LabCliSelection) -> Result<Self, LabRunError> {
        let audit_registry = UxAuditRegistry::built_in();
        let suite_id = selection.suite.as_deref().unwrap_or("firehorse");
        let suite = audit_registry
            .suite(suite_id)
            .ok_or_else(|| LabRunError::UnknownSuite {
                suite: suite_id.to_string(),
                available: audit_registry
                    .suites()
                    .iter()
                    .flat_map(|suite| suite.scenarios.iter())
                    .map(|scenario| scenario.firehorse_scenario_id.to_string())
                    .collect(),
            })?;
        let state = AuditLabState::new(selection, suite)?;
        Ok(Self {
            audit_registry,
            lab_registry: LabScenarioRegistry::built_in(),
            state,
        })
    }

    pub fn state(&self) -> &AuditLabState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut AuditLabState {
        &mut self.state
    }

    pub fn lab_registry(&self) -> &LabScenarioRegistry<'_> {
        &self.lab_registry
    }

    pub fn suite(&self) -> &UxAuditSuite {
        self.audit_registry
            .suite(&self.state.suite_id)
            .expect("audit model must hold a valid suite")
    }

    pub fn render_text(&self) -> Result<String, LabRunError> {
        view::render_model_text(self)
    }

    fn export_current(&mut self) {
        let scenario = self.state.selected_scenario(self.suite());
        let root = interactive_export_root(scenario.firehorse_scenario_id, self.state.viewport);
        match export::export_current_review_pack(self, &root) {
            Ok(result) => {
                self.state.status_line = format!(
                    "Exported {} files to {}",
                    result.files_written.len(),
                    result.root
                );
            }
            Err(error) => {
                self.state.status_line = format!("Export failed: {error}");
            }
        }
    }
}

impl Model for AuditLabModel {
    type Message = AuditMsg;

    fn update(&mut self, msg: Self::Message) -> Cmd<Self::Message> {
        match msg {
            AuditMsg::Quit => return Cmd::quit(),
            AuditMsg::NextFocus => self.state.focus_next(),
            AuditMsg::PreviousFocus => self.state.focus_previous(),
            AuditMsg::Move(delta) => {
                let suite = self.suite();
                let mut state = self.state.clone();
                state.move_selection(delta, suite);
                self.state = state;
            }
            AuditMsg::Open => {
                self.state.status_line = format!(
                    "Opened {}",
                    self.state.selected_scenario(self.suite()).title
                );
            }
            AuditMsg::CycleViewport => self.state.cycle_viewport(),
            AuditMsg::ToggleRenderMode => self.state.toggle_render_mode(),
            AuditMsg::SetDossierMode(mode) => self.state.set_dossier_mode(mode),
            AuditMsg::Mark(status) => {
                let suite = self.suite();
                let mut state = self.state.clone();
                state.mark_current_criterion(suite, status);
                self.state = state;
            }
            AuditMsg::Export => self.export_current(),
            AuditMsg::Help => {
                self.state.status_line =
                    "Keys: Tab focus | j/k move | v viewport | r render | 1-5 dossier | p/c/f/d mark | e export | q quit"
                        .to_string();
            }
            AuditMsg::Noop => {}
        }
        Cmd::none()
    }

    fn view(&self, frame: &mut Frame) {
        view::render_model(self, frame);
    }
}

fn is_actionable_key(key: KeyEvent) -> bool {
    matches!(key.kind, KeyEventKind::Press)
}

fn interactive_export_root(
    scenario_id: &str,
    viewport: crate::shell::uxlab::ViewportClass,
) -> PathBuf {
    PathBuf::from("docs/firehorse_mockups/ux_audit_lab/exports").join(format!(
        "{}_{}_{}",
        sanitize_export_stem(scenario_id),
        viewport.name(),
        unique_export_suffix()
    ))
}

fn unique_export_suffix() -> String {
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

fn sanitize_export_stem(value: &str) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn model() -> AuditLabModel {
        let selection =
            LabCliSelection::parse(["--audit"].into_iter().map(String::from)).expect("selection");
        AuditLabModel::new(&selection).expect("model")
    }

    #[test]
    fn initial_state_selects_editing_lens_studio_mockup_persona() {
        let model = model();

        assert_eq!(
            model
                .state()
                .selected_scenario(model.suite())
                .firehorse_scenario_id,
            "firehorse-editing-lens-standard"
        );
        assert_eq!(model.state().viewport, ViewportClass::Studio);
        assert_eq!(model.state().render_mode, AuditRenderMode::Mockup);
        assert_eq!(model.state().dossier_mode, DossierMode::Persona);
    }

    #[test]
    fn focus_traversal_visits_four_regions() {
        let mut state = model().state().clone();

        state.focus_next();
        assert_eq!(state.focus, AuditFocusRegion::Stage);
        state.focus_next();
        assert_eq!(state.focus, AuditFocusRegion::Dossier);
        state.focus_next();
        assert_eq!(state.focus, AuditFocusRegion::Evidence);
        state.focus_next();
        assert_eq!(state.focus, AuditFocusRegion::Atlas);
    }

    #[test]
    fn atlas_selection_updates_selected_firehorse_scenario() {
        let model = model();
        let suite = model.suite();
        let mut state = model.state().clone();
        state.move_selection(1, suite);

        assert_ne!(
            state.selected_scenario(suite).firehorse_scenario_id,
            "firehorse-editing-lens-standard"
        );
    }

    #[test]
    fn marking_current_criterion_records_local_finding() {
        let model = model();
        let suite = model.suite();
        let mut state = model.state().clone();
        state.set_dossier_mode(DossierMode::Checklist);
        state.mark_current_criterion(suite, AuditStatus::Concern);

        assert_eq!(state.findings.len(), 1);
        assert_eq!(state.findings[0].status, AuditStatus::Concern);
        assert_eq!(state.findings[0].scenario_id, "audit-editing-lens-pricing");
    }

    #[test]
    fn viewport_and_render_mode_cycle_without_changing_scenario() {
        let mut state = model().state().clone();
        let suite = model().suite().clone();
        let scenario = state.selected_scenario(&suite).firehorse_scenario_id;

        state.cycle_viewport();
        state.toggle_render_mode();

        assert_eq!(
            state.selected_scenario(&suite).firehorse_scenario_id,
            scenario
        );
        assert_eq!(state.viewport, ViewportClass::FirstClass);
        assert_eq!(state.render_mode, AuditRenderMode::Contract);
    }
}
