use ftui::layout::{Constraint, Flex, Rect};
use ftui::text::WrapMode;
use ftui::widgets::Widget;
use ftui::widgets::block::{Alignment, Block};
use ftui::widgets::borders::Borders;
use ftui::widgets::paragraph::Paragraph;
use ftui::{
    BufferDiff, Cell, Frame, GraphemePool, PackedRgba, Presenter, Style, TerminalCapabilities,
};

use super::projection::{
    ActivityRowProjection, BadgeTone, ContextCardProjection, FireHorseProjection, FitResult,
    GutterMarkerProjection, LayoutPosture, MockDiagnosticProjection, ProjectItemKind,
    ProjectSpineProjection, RunStepKind, RunStepStatus, TerminalFitProjection, ThemeProjection,
};
use crate::shell::uxlab::{LabRenderedFrame, LabScenarioDescriptor, ViewportClass, frame_to_text};

pub fn render_mockup_frame(
    scenario: LabScenarioDescriptor,
    viewport: ViewportClass,
    projection: &FireHorseProjection,
) -> LabRenderedFrame {
    let size = viewport.wtd_size();
    let text = with_mockup_frame(viewport, projection, |frame| frame_to_text(frame));

    LabRenderedFrame {
        scenario,
        viewport,
        size,
        text,
    }
}

pub fn render_mockup_terminal_stream(
    _scenario: LabScenarioDescriptor,
    viewport: ViewportClass,
    projection: &FireHorseProjection,
) -> Vec<u8> {
    with_mockup_frame(viewport, projection, frame_to_terminal_stream)
}

fn with_mockup_frame<R>(
    viewport: ViewportClass,
    projection: &FireHorseProjection,
    capture: impl FnOnce(&Frame<'_>) -> R,
) -> R {
    let size = viewport.wtd_size();
    let mut pool = GraphemePool::new();
    let mut frame = Frame::new(size.width, size.height, &mut pool);
    render_mockup_into_frame(&mut frame, viewport, projection);

    capture(&frame)
}

pub fn render_mockup_into_frame(
    mut frame: &mut Frame<'_>,
    viewport: ViewportClass,
    projection: &FireHorseProjection,
) {
    let theme = FireTheme::from_projection(projection.theme);
    frame.set_cursor(None);
    frame.set_cursor_visible(false);

    let root = Rect::new(0, 0, frame.width(), frame.height());
    clear(&mut frame, root, theme.root_style());

    match projection.layout {
        LayoutPosture::Launchpad => render_launchpad(&mut frame, root, viewport, projection, theme),
        LayoutPosture::ConsoleFit => {
            render_console_fit(&mut frame, root, viewport, projection, theme);
        }
        LayoutPosture::CompactFocus => {
            render_compact_focus(&mut frame, root, viewport, projection, theme);
        }
        LayoutPosture::CommandLens => {
            render_workbench(
                &mut frame,
                root,
                viewport,
                projection,
                theme,
                WorkbenchMode::Editing,
            );
            let sections = root_sections(root, viewport);
            render_identity(
                &mut frame,
                sections.identity,
                viewport,
                projection,
                theme,
                "Command Lens",
            );
            render_command_lens_overlay(&mut frame, root, viewport, projection, theme);
        }
        LayoutPosture::RunLane => {
            render_workbench(
                &mut frame,
                root,
                viewport,
                projection,
                theme,
                WorkbenchMode::Run,
            );
        }
        LayoutPosture::DebugCockpit => {
            render_workbench(
                &mut frame,
                root,
                viewport,
                projection,
                theme,
                WorkbenchMode::Debug,
            );
        }
        LayoutPosture::Editing => {
            render_workbench(
                &mut frame,
                root,
                viewport,
                projection,
                theme,
                WorkbenchMode::Editing,
            );
        }
    }
}

fn frame_to_terminal_stream(frame: &Frame<'_>) -> Vec<u8> {
    let diff = BufferDiff::full(frame.width(), frame.height());
    let mut presenter = Presenter::new(Vec::<u8>::new(), TerminalCapabilities::modern());
    presenter
        .present_with_pool(&frame.buffer, &diff, Some(frame.pool), None)
        .expect("mockup terminal stream should render");
    presenter
        .into_inner()
        .expect("mockup terminal stream should be recoverable")
}

#[derive(Clone, Copy)]
enum WorkbenchMode {
    Editing,
    Run,
    Debug,
}

#[derive(Clone, Copy)]
struct FireTheme {
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

impl FireTheme {
    fn from_projection(theme: ThemeProjection) -> Self {
        match theme {
            ThemeProjection::GraphiteEmber => Self {
                root: rgb(0x07, 0x0B, 0x10),
                panel: rgb(0x0D, 0x11, 0x17),
                panel_alt: rgb(0x12, 0x1A, 0x24),
                text: rgb(0xEC, 0xF2, 0xF4),
                muted: rgb(0x7A, 0x88, 0x96),
                ember: rgb(0xFF, 0x6B, 0x3D),
                gold: rgb(0xFF, 0xC4, 0x5C),
                cyan: rgb(0x58, 0xD9, 0xE6),
                green: rgb(0x74, 0xD9, 0x9F),
            },
            ThemeProjection::PaperEmber => Self {
                root: rgb(0xF4, 0xED, 0xDD),
                panel: rgb(0xFF, 0xFA, 0xF0),
                panel_alt: rgb(0xF0, 0xE1, 0xCA),
                text: rgb(0x20, 0x1A, 0x16),
                muted: rgb(0x74, 0x68, 0x5C),
                ember: rgb(0xB8, 0x37, 0x18),
                gold: rgb(0x8A, 0x5C, 0x16),
                cyan: rgb(0x1D, 0x6D, 0x75),
                green: rgb(0x2D, 0x74, 0x43),
            },
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

fn render_launchpad(
    frame: &mut Frame<'_>,
    root: Rect,
    viewport: ViewportClass,
    projection: &FireHorseProjection,
    theme: FireTheme,
) {
    let sections = root_sections(root, viewport);
    render_identity(
        frame,
        sections.identity,
        viewport,
        projection,
        theme,
        "Launchpad",
    );

    let columns = Flex::horizontal()
        .constraints([
            Constraint::Percentage(27.0),
            Constraint::Percentage(45.0),
            Constraint::Fill,
        ])
        .split(sections.body);

    render_panel(
        frame,
        columns[0],
        "Start Surface",
        &launchpad_start_body(),
        theme,
        theme.ember,
        true,
    );
    render_panel(
        frame,
        columns[1],
        "Recent Workspaces",
        &launchpad_recent_body(projection),
        theme,
        theme.gold,
        true,
    );
    render_panel(
        frame,
        columns[2],
        "Fit / Context",
        &launchpad_context_body(projection),
        theme,
        theme.cyan,
        false,
    );

    render_activity(
        frame,
        sections.activity,
        projection,
        theme,
        "Recent / Start Signals",
    );
    render_key_rail(frame, sections.key_rail, projection, theme);
}

fn render_workbench(
    frame: &mut Frame<'_>,
    root: Rect,
    viewport: ViewportClass,
    projection: &FireHorseProjection,
    theme: FireTheme,
    mode: WorkbenchMode,
) {
    let sections = root_sections(root, viewport);
    let title = match mode {
        WorkbenchMode::Editing => "Editing Lens",
        WorkbenchMode::Run => "Run Lane",
        WorkbenchMode::Debug => "Debug Cockpit",
    };
    render_identity(frame, sections.identity, viewport, projection, theme, title);

    let (left, right) = match (viewport, mode) {
        (ViewportClass::Studio, WorkbenchMode::Debug) => (31.0, 29.0),
        (ViewportClass::Studio, _) => (24.0, 28.0),
        (ViewportClass::FirstClass, WorkbenchMode::Debug) => (28.0, 29.0),
        (ViewportClass::FirstClass, _) => (23.0, 27.0),
        (_, WorkbenchMode::Debug) => (24.0, 29.0),
        _ => (22.0, 26.0),
    };
    let columns = Flex::horizontal()
        .constraints([
            Constraint::Percentage(left),
            Constraint::Percentage(100.0 - left - right),
            Constraint::Fill,
        ])
        .split(sections.body);

    let left_title = match mode {
        WorkbenchMode::Run => "Run Timeline",
        WorkbenchMode::Debug => "Project / Breakpoints",
        WorkbenchMode::Editing => "Project Spine",
    };
    let left_body = match mode {
        WorkbenchMode::Run => run_timeline_body(projection),
        _ => project_spine_body(projection.project_spine.as_ref()),
    };
    render_panel(
        frame,
        columns[0],
        left_title,
        &left_body,
        theme,
        theme.ember,
        true,
    );

    render_panel(
        frame,
        columns[1],
        &format!("Code Canvas  {}", projection.code_canvas.document_label),
        &code_canvas_body(projection, mode),
        theme,
        theme.cyan,
        true,
    );

    let (dock_title, dock_body, accent) = match mode {
        WorkbenchMode::Run => ("Run Context", run_context_body(projection), theme.gold),
        WorkbenchMode::Debug => ("Debug Dock", debug_dock_body(projection), theme.gold),
        WorkbenchMode::Editing => ("Context Dock", context_dock_body(projection), theme.gold),
    };
    render_panel(
        frame, columns[2], dock_title, &dock_body, theme, accent, true,
    );

    let activity_title = match mode {
        WorkbenchMode::Run => "Activity Deck / Run Timeline",
        WorkbenchMode::Debug => "Activity Deck / Watch Trace",
        WorkbenchMode::Editing => "Activity Deck / Problems",
    };
    render_activity(frame, sections.activity, projection, theme, activity_title);
    render_key_rail(frame, sections.key_rail, projection, theme);
}

fn render_compact_focus(
    frame: &mut Frame<'_>,
    root: Rect,
    viewport: ViewportClass,
    projection: &FireHorseProjection,
    theme: FireTheme,
) {
    let sections = root_sections(root, viewport);
    render_identity(
        frame,
        sections.identity,
        viewport,
        projection,
        theme,
        "Compact Focus",
    );

    render_panel(
        frame,
        sections.body,
        &format!(
            "Code Canvas  {}  project/context/activity peek docks",
            projection.code_canvas.document_label
        ),
        &code_canvas_body(projection, WorkbenchMode::Editing),
        theme,
        theme.cyan,
        true,
    );
    render_activity(frame, sections.activity, projection, theme, "Activity Rail");
    render_key_rail(frame, sections.key_rail, projection, theme);
}

fn render_console_fit(
    frame: &mut Frame<'_>,
    root: Rect,
    viewport: ViewportClass,
    projection: &FireHorseProjection,
    theme: FireTheme,
) {
    let sections = root_sections(root, viewport);
    render_identity(
        frame,
        sections.identity,
        viewport,
        projection,
        theme,
        "Console Fit",
    );

    let columns = Flex::horizontal()
        .constraints([
            Constraint::Percentage(42.0),
            Constraint::Percentage(34.0),
            Constraint::Fill,
        ])
        .split(sections.body);
    render_panel(
        frame,
        columns[0],
        "Capability Signals",
        &terminal_fit_body(projection.terminal_fit.as_ref()),
        theme,
        theme.ember,
        true,
    );
    render_panel(
        frame,
        columns[1],
        "Recommendations",
        &terminal_recommendation_body(projection.terminal_fit.as_ref()),
        theme,
        theme.gold,
        true,
    );
    render_panel(
        frame,
        columns[2],
        "Fallback Preview",
        "truecolor: preferred\nunicode rails: preferred\nascii fallback: explicit\nlabels: always visible\nW100 owns live probing",
        theme,
        theme.cyan,
        false,
    );
    render_activity(
        frame,
        sections.activity,
        projection,
        theme,
        "Capability Activity",
    );
    render_key_rail(frame, sections.key_rail, projection, theme);
}

fn render_command_lens_overlay(
    frame: &mut Frame<'_>,
    root: Rect,
    viewport: ViewportClass,
    projection: &FireHorseProjection,
    theme: FireTheme,
) {
    let Some(super::projection::OverlayProjection::CommandLens(command_lens)) =
        projection.overlay.as_ref()
    else {
        return;
    };

    let width_factor = if is_high_end(viewport) { 72 } else { 82 };
    let height_factor = if is_high_end(viewport) { 62 } else { 68 };
    let overlay_width = root.width.saturating_mul(width_factor).max(82) / 100;
    let overlay_height = root.height.saturating_mul(height_factor).max(18) / 100;
    let overlay = Rect::new(
        root.x + root.width.saturating_sub(overlay_width) / 2,
        root.y + root.height.saturating_sub(overlay_height) / 2,
        overlay_width.min(root.width),
        overlay_height.min(root.height),
    );
    frame.buffer.fill(overlay, Cell::default());

    let mut body = format!(
        "filter: {}    selected: {}\n\n",
        command_lens.filter, command_lens.selected_action_id
    );
    body.push_str("ACTIONS\n");
    for row in &command_lens.rows {
        let selected = if row.action_id == command_lens.selected_action_id {
            ">"
        } else {
            " "
        };
        let binding = row
            .binding
            .as_ref()
            .map(|binding| binding.label.as_str())
            .unwrap_or(" ");
        let state = if row.enabled {
            "enabled".to_string()
        } else {
            format!(
                "disabled: {}",
                row.disabled_reason.as_deref().unwrap_or("reason missing")
            )
        };
        body.push_str(&format!(
            "{selected} {:<24} {:<12} {:<22} {state}\n",
            row.label, binding, row.action_id
        ));
    }
    body.push_str("\nPREVIEW\n");
    body.push_str(&command_lens.preview.title);
    body.push('\n');
    for line in &command_lens.preview.body {
        body.push_str(line);
        body.push('\n');
    }
    body.push_str("\n");
    body.push_str(
        &command_lens
            .footer_hints
            .iter()
            .map(|hint| format!("{} {}", hint.binding.label, hint.label))
            .collect::<Vec<_>>()
            .join("   "),
    );

    render_panel(
        frame,
        overlay,
        "Command Lens",
        &body,
        theme,
        theme.ember,
        true,
    );
}

#[derive(Clone, Copy)]
struct RootSections {
    identity: Rect,
    body: Rect,
    activity: Rect,
    key_rail: Rect,
}

fn root_sections(root: Rect, viewport: ViewportClass) -> RootSections {
    let activity_height = match viewport {
        ViewportClass::Studio => 10,
        ViewportClass::FirstClass => 9,
        ViewportClass::Compact => 5,
        _ => 7,
    };
    let sections = Flex::vertical()
        .constraints([
            Constraint::Fixed(3),
            Constraint::Fill,
            Constraint::Fixed(activity_height),
            Constraint::Fixed(1),
        ])
        .split(root);

    RootSections {
        identity: sections[0],
        body: sections[1],
        activity: sections[2],
        key_rail: sections[3],
    }
}

fn render_identity(
    frame: &mut Frame<'_>,
    area: Rect,
    viewport: ViewportClass,
    projection: &FireHorseProjection,
    theme: FireTheme,
    scene_title: &str,
) {
    let size = viewport.wtd_size();
    let health = projection
        .identity
        .health
        .iter()
        .map(|badge| format!("{} {}", badge_tone_label(badge.tone), badge.label))
        .collect::<Vec<_>>()
        .join("  ");
    let cursor = projection
        .identity
        .cursor
        .map(|cursor| format!("Ln {} Col {}", cursor.line, cursor.column))
        .unwrap_or_else(|| "No cursor".to_string());
    let body = format!(
        "  OXIDE FIRE HORSE UX LAB   {}   target:{}   {}\n  {}   {}   viewport:{} {}x{}   scenario:{}",
        scene_title,
        projection.identity.target.as_deref().unwrap_or("none"),
        cursor,
        projection.identity.workspace_label,
        health,
        viewport.name(),
        size.width,
        size.height,
        projection.scenario_id
    );

    Paragraph::new(body)
        .style(theme.root_style().bold())
        .render(area, frame);
}

fn render_panel(
    frame: &mut Frame<'_>,
    area: Rect,
    title: &str,
    body: &str,
    theme: FireTheme,
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

fn render_activity(
    frame: &mut Frame<'_>,
    area: Rect,
    projection: &FireHorseProjection,
    theme: FireTheme,
    title: &str,
) {
    render_panel(
        frame,
        area,
        title,
        &activity_deck_body(projection),
        theme,
        theme.green,
        true,
    );
}

fn render_key_rail(
    frame: &mut Frame<'_>,
    area: Rect,
    projection: &FireHorseProjection,
    theme: FireTheme,
) {
    let body = projection
        .key_rail
        .hints
        .iter()
        .map(|hint| {
            let state = if hint.enabled { "" } else { " (disabled)" };
            format!("{} {}{}", hint.binding.label, hint.label, state)
        })
        .collect::<Vec<_>>()
        .join("   ");
    Paragraph::new(body)
        .style(Style::new().bg(theme.panel_alt).fg(theme.text).bold())
        .render(area, frame);
}

fn launchpad_start_body() -> String {
    [
        "Open Project        Ctrl+O",
        "Create Project      Ctrl+N",
        "Import Workbook     Ctrl+I",
        "Console Fit         F10",
        "",
        "Start rows are commands with visible disabled reasons once W040 owns project actions.",
    ]
    .join("\n")
}

fn launchpad_recent_body(projection: &FireHorseProjection) -> String {
    projection
        .activity_deck
        .rows
        .iter()
        .filter_map(|row| match row {
            ActivityRowProjection::Text { source, text }
                if *source == "SessionStore::recent_workspaces" =>
            {
                Some(format!("{}   {}", "RECENT", text))
            }
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn launchpad_context_body(projection: &FireHorseProjection) -> String {
    let mut rows = Vec::new();
    rows.push("Capability posture: modern desktop terminal preferred".to_string());
    rows.push("Fallbacks stay honest, but do not define the high-end UX.".to_string());
    if let Some(dock) = &projection.context_dock {
        for card in &dock.cards {
            if let ContextCardProjection::Unavailable(unavailable) = card {
                rows.push(format!(
                    "{} unavailable: {}",
                    unavailable.source, unavailable.reason
                ));
            }
        }
    }
    rows.join("\n")
}

fn project_spine_body(spine: Option<&ProjectSpineProjection>) -> String {
    let Some(spine) = spine else {
        return "Project Spine hidden\nAlt+1 opens peek dock".to_string();
    };

    spine
        .rows
        .iter()
        .map(|row| {
            let active = if row.active { ">" } else { " " };
            let dirty = if row.dirty { "*" } else { " " };
            let kind = project_kind_label(row.kind);
            let badges = row
                .badges
                .iter()
                .map(|badge| badge.label.as_str())
                .collect::<Vec<_>>()
                .join(",");
            format!(
                "{active}{dirty} {:<4} {}{} {}",
                kind,
                "  ".repeat(row.depth as usize),
                row.label,
                badges
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn code_canvas_body(projection: &FireHorseProjection, mode: WorkbenchMode) -> String {
    let mut rows = Vec::new();
    rows.push(format!(
        "{}    {}",
        projection.code_canvas.language, projection.code_canvas.document_label
    ));
    rows.push("".to_string());
    for line in &projection.code_canvas.lines {
        let marker = if projection.code_canvas.execution_line == Some(line.number) {
            "=>"
        } else if line
            .markers
            .iter()
            .any(|marker| matches!(marker, GutterMarkerProjection::Breakpoint))
        {
            "B "
        } else if line
            .markers
            .iter()
            .any(|marker| matches!(marker, GutterMarkerProjection::Diagnostic))
        {
            "! "
        } else {
            "  "
        };
        let selected = if projection
            .code_canvas
            .selection
            .is_some_and(|range| range.start.line == line.number)
        {
            ">"
        } else {
            " "
        };
        rows.push(format!(
            "{:>4} {}{} {}",
            line.number, selected, marker, line.text
        ));
    }

    if let Some(lens) = &projection.code_canvas.lens {
        rows.push("".to_string());
        rows.push(format!("SOURCE LENS  {}", lens.title));
        rows.push(format!(
            "source: {} / {}",
            lens.source.provider, lens.source.query
        ));
        for line in &lens.body {
            rows.push(format!("  {}", line));
        }
        rows.push(format!(
            "actions: {}",
            lens.actions
                .iter()
                .map(|action| action.action_id)
                .collect::<Vec<_>>()
                .join("  ")
        ));
    } else if matches!(mode, WorkbenchMode::Editing) {
        rows.push("".to_string());
        rows.push(
            "SOURCE LENS unavailable: current ShellState exposes no semantic hover".to_string(),
        );
    }

    rows.join("\n")
}

fn context_dock_body(projection: &FireHorseProjection) -> String {
    let Some(dock) = &projection.context_dock else {
        return "Context Dock hidden\nAlt+3 opens peek dock".to_string();
    };
    let mut rows = vec![dock.title.clone()];
    for card in &dock.cards {
        rows.extend(context_card_rows(card));
        rows.push(String::new());
    }
    rows.join("\n")
}

fn context_card_rows(card: &ContextCardProjection) -> Vec<String> {
    match card {
        ContextCardProjection::Diagnostic(diagnostic) => vec![
            format!(
                "Diagnostic  {} {}",
                severity_label(diagnostic),
                diagnostic.code
            ),
            format!(
                "line {}: {}",
                diagnostic.range.start.line, diagnostic.message
            ),
            format!("from {}", diagnostic.provenance.provider),
        ],
        ContextCardProjection::Symbol(symbol) => vec![
            format!("Symbol  {}", symbol.name),
            symbol.detail.clone(),
            format!("from {}", symbol.provenance.provider),
        ],
        ContextCardProjection::RunStatus(status) => vec![
            format!("Run Status  {}", status.target_id),
            format!(
                "{} {}",
                run_step_name(status.active_step),
                run_status_name(status.status)
            ),
            status.message.clone(),
        ],
        ContextCardProjection::CallStack(frames) => {
            let mut rows = vec!["Call Stack".to_string()];
            rows.extend(frames.iter().map(|frame| {
                format!(
                    "{}  {} line {}",
                    frame.frame_id, frame.procedure, frame.line
                )
            }));
            rows
        }
        ContextCardProjection::Locals(locals) => {
            let mut rows = vec!["Locals".to_string()];
            rows.extend(
                locals.iter().map(|local| {
                    format!("{} As {} = {}", local.name, local.type_label, local.value)
                }),
            );
            rows
        }
        ContextCardProjection::Watches(watches) => {
            let mut rows = vec!["Watches".to_string()];
            rows.extend(watches.iter().map(|watch| {
                format!(
                    "{} As {} = {}",
                    watch.expression, watch.type_label, watch.value
                )
            }));
            rows
        }
        ContextCardProjection::TerminalFit(fit) => {
            vec![format!("Terminal Fit  {}", fit.summary)]
        }
        ContextCardProjection::Unavailable(unavailable) => vec![format!(
            "{} unavailable: {}",
            unavailable.source, unavailable.reason
        )],
    }
}

fn run_timeline_body(projection: &FireHorseProjection) -> String {
    projection
        .seams
        .run_events
        .iter()
        .map(|event| {
            format!(
                "{:<8} {:<8} {:>4}ms\n  target: {}\n  {}",
                run_step_name(event.step),
                run_status_name(event.status),
                event.emitted_at_ms,
                event.target_id,
                event.message
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn run_context_body(projection: &FireHorseProjection) -> String {
    let mut rows = vec!["Run state is fixture-backed in W039.".to_string()];
    for event in &projection.seams.run_events {
        if event.status == RunStepStatus::Active {
            rows.push(format!(
                "Active: {} on {}",
                run_step_name(event.step),
                event.target_id
            ));
            rows.push(event.message.clone());
        }
    }
    rows.push("W070 owns real execution and Immediate behavior.".to_string());
    rows.join("\n")
}

fn debug_dock_body(projection: &FireHorseProjection) -> String {
    let mut rows = Vec::new();
    rows.push(format!(
        "Paused at {}:{}",
        projection.code_canvas.document_label,
        projection.code_canvas.execution_line.unwrap_or_default()
    ));
    rows.push("".to_string());
    rows.push("CALL STACK".to_string());
    for frame in &projection.seams.debug_frames {
        rows.push(format!(
            "{}  {}  line {}",
            frame.frame_id, frame.procedure, frame.line
        ));
    }
    rows.push("".to_string());
    rows.push("LOCALS".to_string());
    for local in &projection.seams.locals {
        rows.push(format!(
            "{} As {} = {}",
            local.name, local.type_label, local.value
        ));
    }
    rows.push("".to_string());
    rows.push("WATCHES".to_string());
    for watch in &projection.seams.watches {
        rows.push(format!(
            "{} As {} = {}",
            watch.expression, watch.type_label, watch.value
        ));
    }
    rows.push("".to_string());
    rows.push("W080 owns real debug contracts.".to_string());
    rows.join("\n")
}

fn terminal_fit_body(fit: Option<&TerminalFitProjection>) -> String {
    let Some(fit) = fit else {
        return "Terminal capability fixture unavailable".to_string();
    };
    let mut rows = vec![fit.summary.clone(), String::new()];
    rows.extend(fit.rows.iter().map(|row| {
        format!(
            "{:<14} {:<5} {}",
            row.signal,
            fit_result_name(row.result),
            row.detail
        )
    }));
    rows.join("\n")
}

fn terminal_recommendation_body(fit: Option<&TerminalFitProjection>) -> String {
    let Some(fit) = fit else {
        return "No recommendations".to_string();
    };
    fit.rows
        .iter()
        .map(|row| format!("{}: {}", row.signal, row.recommendation))
        .collect::<Vec<_>>()
        .join("\n")
}

fn activity_deck_body(projection: &FireHorseProjection) -> String {
    let tabs = projection
        .activity_deck
        .tabs
        .iter()
        .map(|tab| {
            tab.count
                .map(|count| format!("{} ({count})", tab.label))
                .unwrap_or_else(|| tab.label.clone())
        })
        .collect::<Vec<_>>()
        .join("  |  ");
    let mut rows = vec![
        format!("active: {}", projection.activity_deck.active.name()),
        tabs,
    ];
    rows.push(String::new());
    rows.extend(projection.activity_deck.rows.iter().map(activity_row_label));
    rows.join("\n")
}

fn activity_row_label(row: &ActivityRowProjection) -> String {
    match row {
        ActivityRowProjection::Diagnostic(diagnostic) => format!(
            "{} {}:{}  {}",
            diagnostic.code,
            diagnostic.document_id,
            diagnostic.range.start.line,
            diagnostic.message
        ),
        ActivityRowProjection::Symbol(symbol) => {
            format!("symbol {}  {}", symbol.name, symbol.detail)
        }
        ActivityRowProjection::RunEvent(event) => format!(
            "{} {}  {}",
            run_step_name(event.step),
            run_status_name(event.status),
            event.message
        ),
        ActivityRowProjection::StackFrame(frame) => {
            format!(
                "stack {}  {} line {}",
                frame.frame_id, frame.procedure, frame.line
            )
        }
        ActivityRowProjection::Local(local) => {
            format!(
                "local {} As {} = {}",
                local.name, local.type_label, local.value
            )
        }
        ActivityRowProjection::Watch(watch) => {
            format!(
                "watch {} As {} = {}",
                watch.expression, watch.type_label, watch.value
            )
        }
        ActivityRowProjection::Text { source, text } => format!("{source}: {text}"),
    }
}

fn project_kind_label(kind: ProjectItemKind) -> &'static str {
    match kind {
        ProjectItemKind::Project => "proj",
        ProjectItemKind::Module => "mod",
        ProjectItemKind::Class => "cls",
        ProjectItemKind::Form => "form",
        ProjectItemKind::Reference => "ref",
        ProjectItemKind::Target => "tgt",
    }
}

fn badge_tone_label(tone: BadgeTone) -> &'static str {
    match tone {
        BadgeTone::Info => "info",
        BadgeTone::Success => "ok",
        BadgeTone::Warning => "warn",
        BadgeTone::Error => "err",
    }
}

fn fit_result_name(result: FitResult) -> &'static str {
    match result {
        FitResult::Pass => "pass",
        FitResult::Warn => "warn",
        FitResult::Fail => "fail",
    }
}

fn run_step_name(step: RunStepKind) -> &'static str {
    match step {
        RunStepKind::Prepare => "Prepare",
        RunStepKind::Analyze => "Analyze",
        RunStepKind::Build => "Build",
        RunStepKind::Execute => "Execute",
        RunStepKind::Result => "Result",
    }
}

fn run_status_name(status: RunStepStatus) -> &'static str {
    match status {
        RunStepStatus::Pending => "pending",
        RunStepStatus::Active => "active",
        RunStepStatus::Complete => "complete",
        RunStepStatus::Failed => "failed",
    }
}

fn severity_label(diagnostic: &MockDiagnosticProjection) -> &'static str {
    match diagnostic.severity {
        super::projection::DiagnosticSeverity::Error => "error",
        super::projection::DiagnosticSeverity::Warning => "warning",
        super::projection::DiagnosticSeverity::Info => "info",
        super::projection::DiagnosticSeverity::Hint => "hint",
    }
}

fn is_high_end(viewport: ViewportClass) -> bool {
    matches!(viewport, ViewportClass::FirstClass | ViewportClass::Studio)
}

#[cfg(test)]
mod tests {
    use super::super::{FIRE_HORSE_SCENARIOS, fixtures::projection_for_scenario};
    use super::*;

    fn scenario(id: &str) -> LabScenarioDescriptor {
        FIRE_HORSE_SCENARIOS
            .iter()
            .find(|scenario| scenario.id == id)
            .copied()
            .expect("scenario descriptor")
    }

    #[test]
    fn mockup_renderer_uses_frankentui_surface_chrome_for_every_firehorse_scenario() {
        for descriptor in FIRE_HORSE_SCENARIOS {
            let projection = if descriptor.id == super::super::adapter::REAL_EDITING_SCENARIO_ID {
                super::super::adapter::thin_slice_editing_projection()
            } else {
                projection_for_scenario(descriptor.id).expect("projection")
            };
            let rendered =
                render_mockup_frame(descriptor, descriptor.default_viewport, &projection);

            assert!(rendered.text.contains("OXIDE FIRE HORSE UX LAB"));
            assert!(
                rendered.text.contains("┌"),
                "{} should use Block chrome",
                descriptor.id
            );
            assert!(
                rendered.text.contains("┘"),
                "{} should use Block chrome",
                descriptor.id
            );
            assert!(
                rendered.text.contains("Activity")
                    || rendered.text.contains("Capability")
                    || rendered.text.contains("Command Lens"),
                "{} should expose review surfaces",
                descriptor.id
            );
        }
    }

    #[test]
    fn mockup_ansi_stream_contains_true_terminal_style_sequences() {
        let descriptor = scenario("firehorse-editing-lens-standard");
        let projection = projection_for_scenario(descriptor.id).expect("editing projection");
        let stream = render_mockup_terminal_stream(descriptor, ViewportClass::Studio, &projection);
        let text = String::from_utf8(stream).expect("ansi stream should be utf8");

        assert!(text.contains("\u{1b}["));
        assert!(text.contains("OXIDE FIRE HORSE UX LAB"));
        assert!(text.contains("Code Canvas"));
    }
}
