use ftui::layout::Rect;
use ftui::text::WrapMode;
use ftui::widgets::Widget;
use ftui::widgets::block::{Alignment, Block};
use ftui::widgets::borders::Borders;
use ftui::widgets::paragraph::Paragraph;
use ftui::{Frame, GraphemePool, PackedRgba, Style};

use super::controller::{AuditFocusRegion, AuditLabModel, AuditRenderMode, DossierMode};
use super::model::{
    AuditCategory, AuditConfidence, AuditStatus, SeamStatus, UxAuditCriterion, UxAuditFinding,
    UxAuditScenario, UxAuditSuite,
};
use super::score::scorecard_for;
use crate::shell::uxlab::{LabRunError, ViewportClass, frame_to_text};

pub fn render_model_text(model: &AuditLabModel) -> Result<String, LabRunError> {
    let size = model.state().viewport.wtd_size();
    let mut pool = GraphemePool::new();
    let mut frame = Frame::new(size.width, size.height, &mut pool);
    frame.set_cursor(None);
    frame.set_cursor_visible(false);
    render_model(model, &mut frame);
    Ok(frame_to_text(&frame))
}

pub fn render_model(model: &AuditLabModel, frame: &mut Frame<'_>) {
    let root = Rect::new(0, 0, frame.width(), frame.height());
    let theme = AuditTheme::new();
    clear(frame, root, theme.root_style());

    let state = model.state();
    let suite = model.suite();
    let scenario = state.selected_scenario(suite);
    let persona = suite
        .find_persona(scenario.persona_id)
        .expect("audit scenario must reference valid persona");

    let sections = AuditSections::from_root(root, state.viewport);
    render_header(frame, sections.header, model, theme);
    render_panel(
        frame,
        sections.atlas,
        "Scenario Atlas",
        &atlas_body(suite, scenario, state.scenario_index, state.viewport),
        theme,
        theme.cyan,
        state.focus == AuditFocusRegion::Atlas,
    );
    render_panel(
        frame,
        sections.stage,
        "Live Fire Horse Stage",
        &stage_body(model, sections.stage),
        theme,
        theme.ember,
        state.focus == AuditFocusRegion::Stage,
    );
    render_panel(
        frame,
        sections.dossier,
        &format!("Audit Dossier | {}", state.dossier_mode.label()),
        &dossier_body(model, suite, scenario, persona),
        theme,
        theme.gold,
        state.focus == AuditFocusRegion::Dossier,
    );
    render_panel(
        frame,
        sections.evidence,
        "Evidence Rail",
        &evidence_body(model, scenario),
        theme,
        theme.green,
        state.focus == AuditFocusRegion::Evidence,
    );
    render_status(frame, sections.status, model, theme);
}

#[derive(Clone, Copy)]
struct AuditSections {
    header: Rect,
    atlas: Rect,
    stage: Rect,
    dossier: Rect,
    evidence: Rect,
    status: Rect,
}

impl AuditSections {
    fn from_root(root: Rect, viewport: ViewportClass) -> Self {
        let header = Rect::new(root.x, root.y, root.width, 2.min(root.height));
        let status = Rect::new(
            root.x,
            root.y + root.height.saturating_sub(1),
            root.width,
            1.min(root.height),
        );
        let evidence_height = if root.height >= 34 { 5 } else { 4 };
        let evidence = Rect::new(
            root.x,
            status.y.saturating_sub(evidence_height),
            root.width,
            evidence_height,
        );
        let body_y = header.y + header.height;
        let body_height = evidence.y.saturating_sub(body_y);
        let body = Rect::new(root.x, body_y, root.width, body_height);

        if viewport == ViewportClass::Compact || root.width < 118 {
            let stage_height = (body.height.saturating_mul(3) / 5).max(10);
            let stage = Rect::new(body.x, body.y, body.width, stage_height.min(body.height));
            let lower_y = stage.y + stage.height;
            let lower_height = body.y + body.height - lower_y;
            let atlas_width = (body.width / 2).max(28);
            let atlas = Rect::new(body.x, lower_y, atlas_width.min(body.width), lower_height);
            let dossier = Rect::new(
                body.x + atlas.width,
                lower_y,
                body.width.saturating_sub(atlas.width),
                lower_height,
            );
            return Self {
                header,
                atlas,
                stage,
                dossier,
                evidence,
                status,
            };
        }

        let atlas_width = if root.width >= 180 { 34 } else { 30 };
        let dossier_width = if root.width >= 180 { 48 } else { 42 };
        let stage_width = body
            .width
            .saturating_sub(atlas_width)
            .saturating_sub(dossier_width);
        let atlas = Rect::new(body.x, body.y, atlas_width, body.height);
        let stage = Rect::new(body.x + atlas_width, body.y, stage_width, body.height);
        let dossier = Rect::new(
            stage.x + stage.width,
            body.y,
            dossier_width.min(body.width.saturating_sub(atlas_width)),
            body.height,
        );

        Self {
            header,
            atlas,
            stage,
            dossier,
            evidence,
            status,
        }
    }
}

#[derive(Clone, Copy)]
struct AuditTheme {
    root: PackedRgba,
    panel: PackedRgba,
    panel_alt: PackedRgba,
    text: PackedRgba,
    muted: PackedRgba,
    ember: PackedRgba,
    gold: PackedRgba,
    cyan: PackedRgba,
    green: PackedRgba,
}

impl AuditTheme {
    fn new() -> Self {
        Self {
            root: rgb(0x06, 0x0A, 0x0F),
            panel: rgb(0x0D, 0x13, 0x19),
            panel_alt: rgb(0x14, 0x1D, 0x28),
            text: rgb(0xEC, 0xF2, 0xF4),
            muted: rgb(0x8A, 0x96, 0xA3),
            ember: rgb(0xFF, 0x6B, 0x3D),
            gold: rgb(0xFF, 0xC4, 0x5C),
            cyan: rgb(0x58, 0xD9, 0xE6),
            green: rgb(0x74, 0xD9, 0x9F),
        }
    }

    fn root_style(self) -> Style {
        Style::new().bg(self.root).fg(self.text)
    }

    fn panel_style(self, active: bool) -> Style {
        Style::new()
            .bg(if active { self.panel_alt } else { self.panel })
            .fg(self.text)
    }

    fn content_style(self, active: bool) -> Style {
        Style::new()
            .bg(if active { self.panel_alt } else { self.panel })
            .fg(if active { self.text } else { self.muted })
    }

    fn border_style(self, accent: PackedRgba) -> Style {
        Style::new().fg(accent).bg(self.panel_alt).bold()
    }
}

fn rgb(r: u8, g: u8, b: u8) -> PackedRgba {
    PackedRgba::rgb(r, g, b)
}

fn clear(frame: &mut Frame<'_>, area: Rect, style: Style) {
    Paragraph::new("").style(style).render(area, frame);
}

fn render_header(frame: &mut Frame<'_>, area: Rect, model: &AuditLabModel, theme: AuditTheme) {
    let state = model.state();
    let scenario = state.selected_scenario(model.suite());
    let size = state.viewport.wtd_size();
    let body = format!(
        "OxIde UX Audit Lab | {} | {} | {} | {} {}x{}\nfocus {} | evidence local only | public posting disabled",
        scenario.firehorse_scenario_id,
        state.viewport.name(),
        state.render_mode.label(),
        state.dossier_mode.label(),
        size.width,
        size.height,
        state.focus.label()
    );
    Paragraph::new(body)
        .style(Style::new().bg(theme.root).fg(theme.text).bold())
        .render(area, frame);
}

fn render_panel(
    frame: &mut Frame<'_>,
    area: Rect,
    title: &str,
    body: &str,
    theme: AuditTheme,
    accent: PackedRgba,
    active: bool,
) {
    if area.is_empty() {
        return;
    }
    Paragraph::new(body.to_string())
        .style(theme.content_style(active))
        .block(
            Block::new()
                .borders(Borders::ALL)
                .border_style(theme.border_style(accent))
                .style(theme.panel_style(active))
                .title(title)
                .title_alignment(Alignment::Center),
        )
        .wrap(WrapMode::WordChar)
        .render(area, frame);
}

fn render_status(frame: &mut Frame<'_>, area: Rect, model: &AuditLabModel, theme: AuditTheme) {
    Paragraph::new(model.state().status_line.clone())
        .style(Style::new().bg(theme.panel_alt).fg(theme.text).bold())
        .render(area, frame);
}

fn atlas_body(
    suite: &UxAuditSuite,
    selected: &UxAuditScenario,
    selected_index: usize,
    viewport: ViewportClass,
) -> String {
    let mut lines = vec![
        format!("suite: {} | {}", suite.id, suite.title),
        format!("viewport ladder: studio > first-class > standard > compact"),
        format!("current: {}", viewport.name()),
        String::new(),
        "Personas".to_string(),
    ];
    for persona in &suite.personas {
        let marker = if persona.id == selected.persona_id {
            ">"
        } else {
            " "
        };
        lines.push(format!("{marker} {} | {}", persona.id, persona.title));
    }
    lines.push(String::new());
    lines.push("Scenarios".to_string());
    for (index, scenario) in suite.scenarios.iter().enumerate() {
        let marker = if index == selected_index { ">" } else { " " };
        lines.push(format!(
            "{marker} {}",
            compact_label(scenario.firehorse_scenario_id, 28)
        ));
        lines.push(format!(
            "  {} | {}",
            scenario.persona_id, scenario.default_viewport
        ));
    }
    lines.join("\n")
}

fn stage_body(model: &AuditLabModel, area: Rect) -> String {
    let state = model.state();
    let scenario = state.selected_scenario(model.suite());
    let result = match state.render_mode {
        AuditRenderMode::Mockup => model.lab_registry().render_mockup(
            model.suite().id,
            scenario.firehorse_scenario_id,
            Some(state.viewport),
        ),
        AuditRenderMode::Contract => model.lab_registry().render(
            model.suite().id,
            scenario.firehorse_scenario_id,
            Some(state.viewport),
        ),
    };

    let inner_width = area.width.saturating_sub(2) as usize;
    let inner_height = area.height.saturating_sub(2) as usize;
    match result {
        Ok(rendered) => crop_text(&rendered.text, inner_width, inner_height),
        Err(error) => format!("render failed: {error}"),
    }
}

fn dossier_body(
    model: &AuditLabModel,
    suite: &UxAuditSuite,
    scenario: &UxAuditScenario,
    persona: &super::model::UxPersona,
) -> String {
    match model.state().dossier_mode {
        DossierMode::Persona => persona_body(persona),
        DossierMode::Journey => journey_body(model, scenario),
        DossierMode::Mapping => mapping_body(model, scenario),
        DossierMode::Checklist => checklist_body(model, suite),
        DossierMode::Findings => findings_body(model, suite, scenario),
    }
}

fn persona_body(persona: &super::model::UxPersona) -> String {
    let mut lines = vec![
        format!("{} | {}", persona.title, persona.id),
        format!("role: {}", persona.role),
        format!("pressure: {}", persona.job_pressure),
        String::new(),
        "goals".to_string(),
    ];
    extend_bullets(&mut lines, &persona.goals);
    lines.push(String::new());
    lines.push("constraints".to_string());
    extend_bullets(&mut lines, &persona.constraints);
    lines.push(String::new());
    lines.push(format!("delight: {}", persona.delight_target));
    lines.push(String::new());
    lines.push("failure modes".to_string());
    extend_bullets(&mut lines, &persona.failure_modes);
    lines.join("\n")
}

fn journey_body(model: &AuditLabModel, scenario: &UxAuditScenario) -> String {
    let mut lines = vec![
        format!("{} | {}", scenario.title, scenario.id),
        format!("intent: {}", scenario.intent),
        String::new(),
    ];
    for (index, step) in scenario.steps.iter().enumerate() {
        let marker = if index == model.state().selected_step_index {
            ">"
        } else {
            " "
        };
        lines.push(format!("{marker} {} | {}", step.id, step.title));
        lines.push(format!("  user: {}", step.user_intent));
        lines.push("  surfaces".to_string());
        for surface in &step.expected_surfaces {
            lines.push(format!(
                "  - {} -> {} -> {}",
                surface.surface, surface.projection_path, surface.owner_workset
            ));
        }
        lines.push(format!("  actions: {}", step.expected_actions.join(", ")));
    }
    lines.join("\n")
}

fn mapping_body(model: &AuditLabModel, scenario: &UxAuditScenario) -> String {
    let mut lines = vec![
        "visible surface -> projection -> action/seam -> workset".to_string(),
        String::new(),
    ];
    for step in &scenario.steps {
        let action_text = if step.expected_actions.is_empty() {
            "no action id".to_string()
        } else {
            step.expected_actions.join(", ")
        };
        let seam_text = if step.seam_refs.is_empty() {
            "no OxVba seam required".to_string()
        } else {
            step.seam_refs
                .iter()
                .map(|seam| {
                    format!(
                        "{} [{}] -> {}",
                        seam.source,
                        seam_status_label(seam.status),
                        seam.downstream_workset
                    )
                })
                .collect::<Vec<_>>()
                .join(" | ")
        };
        for surface in &step.expected_surfaces {
            lines.push(format!("surface: {}", surface.surface));
            lines.push(format!("  projection: {}", surface.projection_path));
            lines.push(format!("  visible: {}", surface.visible_contract));
            lines.push(format!("  actions: {action_text}"));
            lines.push(format!("  seam: {seam_text}"));
            lines.push(format!("  owner: {}", surface.owner_workset));
            lines.push(String::new());
        }
    }
    if model
        .state()
        .selected_scenario(model.suite())
        .steps
        .iter()
        .all(|step| {
            step.expected_surfaces
                .iter()
                .all(|surface| !surface.projection_path.is_empty())
        })
    {
        lines.push("mapping preflight: projection paths present".to_string());
    }
    lines.join("\n")
}

fn checklist_body(model: &AuditLabModel, suite: &UxAuditSuite) -> String {
    let mut lines = vec![
        "p pass  c concern  f fail  d deferred".to_string(),
        "aesthetic pass/concern requires cited artifact in export".to_string(),
        String::new(),
    ];
    for (index, criterion) in suite.criteria.iter().enumerate() {
        let marker = if index == model.state().selected_criterion_index {
            ">"
        } else {
            " "
        };
        let finding = model
            .state()
            .findings
            .iter()
            .find(|finding| finding.criterion_id == criterion.id);
        let status = finding
            .map(|finding| status_label(finding.status))
            .unwrap_or("unmarked");
        lines.push(format!(
            "{marker} {} [{}] {}",
            criterion.id,
            criterion.category.name(),
            status
        ));
        lines.push(format!("  {}", criterion.question));
        lines.push(format!("  evidence: {}", criterion.evidence_required));
    }
    lines.join("\n")
}

fn findings_body(
    model: &AuditLabModel,
    suite: &UxAuditSuite,
    scenario: &UxAuditScenario,
) -> String {
    let scorecard = scorecard_for(
        suite,
        scenario.firehorse_scenario_id,
        model.state().viewport,
        model.lab_registry(),
    );
    let mut lines = vec![
        format!("scenario: {}", scenario.firehorse_scenario_id),
        match scorecard {
            Ok(scorecard) => format!(
                "scorecard gate: {:?} | dense lines {} | ansi {}",
                scorecard.gate,
                scorecard.objective_preflight.dense_lines,
                scorecard.objective_preflight.has_ansi_stream
            ),
            Err(error) => format!("scorecard unavailable: {error}"),
        },
        String::new(),
        "local findings".to_string(),
    ];
    let findings = model
        .state()
        .findings
        .iter()
        .filter(|finding| finding.scenario_id == scenario.id)
        .collect::<Vec<_>>();
    if findings.is_empty() {
        lines.push("No local findings marked yet.".to_string());
    } else {
        for finding in findings {
            lines.push(finding_line(finding));
        }
    }
    lines.push(String::new());
    lines.push("objective scorecards still require human review for emotional_fit.".to_string());
    lines.join("\n")
}

fn evidence_body(model: &AuditLabModel, scenario: &UxAuditScenario) -> String {
    let state = model.state();
    let mut lines = vec![
        format!(
            "render: {} | viewport: {} | selected artifact {}",
            state.render_mode.label(),
            state.viewport.name(),
            state.selected_artifact_index + 1
        ),
        format!(
            "mockup: target/release/oxide-uxlab.exe --suite firehorse --scenario {} --viewport {} --once --mockup",
            scenario.firehorse_scenario_id,
            state.viewport.name()
        ),
        format!(
            "score: target/release/oxide-uxlab.exe --audit --suite firehorse --scenario {} --viewport {} --evaluate functional,aesthetic --json",
            scenario.firehorse_scenario_id,
            state.viewport.name()
        ),
    ];
    for (index, artifact) in scenario.reference_artifacts.iter().enumerate() {
        let marker = if index == state.selected_artifact_index {
            ">"
        } else {
            " "
        };
        lines.push(format!(
            "{marker} {} | {} | {}",
            artifact.kind, artifact.title, artifact.path
        ));
    }
    lines.join("\n")
}

fn crop_text(text: &str, width: usize, height: usize) -> String {
    if width == 0 || height == 0 {
        return String::new();
    }
    text.lines()
        .take(height)
        .map(|line| compact_label(line, width))
        .collect::<Vec<_>>()
        .join("\n")
}

fn compact_label(text: &str, max_width: usize) -> String {
    if text.chars().count() <= max_width {
        return text.to_string();
    }
    if max_width <= 1 {
        return "~".to_string();
    }
    let mut value = text
        .chars()
        .take(max_width.saturating_sub(1))
        .collect::<String>();
    value.push('~');
    value
}

fn extend_bullets(lines: &mut Vec<String>, values: &[&str]) {
    for value in values {
        lines.push(format!("- {value}"));
    }
}

fn seam_status_label(status: SeamStatus) -> &'static str {
    match status {
        SeamStatus::Real => "real",
        SeamStatus::Future => "future",
        SeamStatus::Unavailable => "unavailable",
        SeamStatus::NotRequired => "not_required",
    }
}

fn status_label(status: AuditStatus) -> &'static str {
    match status {
        AuditStatus::Pass => "pass",
        AuditStatus::Concern => "concern",
        AuditStatus::Fail => "fail",
        AuditStatus::Deferred => "deferred",
    }
}

fn finding_line(finding: &UxAuditFinding) -> String {
    let owner = finding
        .downstream_owner
        .as_deref()
        .unwrap_or("no downstream owner");
    let confidence = match finding.confidence {
        AuditConfidence::Objective => "objective",
        AuditConfidence::StructuredJudgement => "structured",
        AuditConfidence::ManualRequired => "manual",
    };
    format!(
        "- {} {} | {} | {} | {}",
        finding.criterion_id,
        status_label(finding.status),
        confidence,
        owner,
        finding.rationale
    )
}

#[allow(dead_code)]
fn category_group(category: AuditCategory) -> &'static str {
    match category {
        AuditCategory::PersonaFit
        | AuditCategory::JourneyFit
        | AuditCategory::CommandClarity
        | AuditCategory::StateOwnership
        | AuditCategory::SeamHonesty
        | AuditCategory::Degradation => "functional",
        AuditCategory::Hierarchy
        | AuditCategory::Density
        | AuditCategory::Balance
        | AuditCategory::ToneAndColor
        | AuditCategory::TerminalCraft
        | AuditCategory::ReferenceFidelity
        | AuditCategory::TextFit
        | AuditCategory::EmotionalFit => "aesthetic",
    }
}

#[allow(dead_code)]
fn criterion_by_id<'a>(suite: &'a UxAuditSuite, id: &str) -> Option<&'a UxAuditCriterion> {
    suite.criteria.iter().find(|criterion| criterion.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shell::uxlab::LabCliSelection;
    use crate::shell::uxlab::audit::controller::DossierMode;

    fn model() -> AuditLabModel {
        let selection =
            LabCliSelection::parse(["--audit"].into_iter().map(String::from)).expect("selection");
        AuditLabModel::new(&selection).expect("model")
    }

    #[test]
    fn rendered_cockpit_contains_four_review_regions() {
        let text = model().render_text().expect("render text");

        assert!(text.contains("Scenario Atlas"));
        assert!(text.contains("Live Fire Horse Stage"));
        assert!(text.contains("Audit Dossier"));
        assert!(text.contains("Evidence Rail"));
        assert!(text.contains("firehorse-editing-lens-standard"));
    }

    #[test]
    fn persona_journey_mapping_checklist_and_findings_lenses_render_contract_terms() {
        let mut model = model();

        model.state_mut().set_dossier_mode(DossierMode::Persona);
        assert!(
            model
                .render_text()
                .expect("persona")
                .contains("Pricing Maintainer")
        );
        model.state_mut().set_dossier_mode(DossierMode::Journey);
        assert!(model.render_text().expect("journey").contains("actions:"));
        model.state_mut().set_dossier_mode(DossierMode::Mapping);
        assert!(
            model
                .render_text()
                .expect("mapping")
                .contains("projection:")
        );
        model.state_mut().set_dossier_mode(DossierMode::Checklist);
        assert!(
            model
                .render_text()
                .expect("checklist")
                .contains("functional.seam_honesty")
        );
        model.state_mut().set_dossier_mode(DossierMode::Findings);
        assert!(
            model
                .render_text()
                .expect("findings")
                .contains("scorecard gate:")
        );
    }

    #[test]
    fn compact_layout_still_keeps_core_regions_visible() {
        let selection = LabCliSelection::parse(
            ["--audit", "--viewport", "compact"]
                .into_iter()
                .map(String::from),
        )
        .expect("selection");
        let model = AuditLabModel::new(&selection).expect("model");

        let text = model.render_text().expect("compact render");

        assert!(text.contains("Scenario Atlas"));
        assert!(text.contains("Live Fire Horse Stage"));
        assert!(text.contains("Evidence Rail"));
    }
}
