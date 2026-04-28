use super::projection::{
    ActivityRowProjection, CommandLensProjection, ContextCardProjection, FireHorseProjection,
    FitResult, MockDiagnosticProjection, MockRunEventProjection, OverlayProjection,
    ProjectSpineProjection, RunStepKind, RunStepStatus, TerminalFitProjection,
};
use crate::shell::uxlab::{LabRenderedFrame, LabScenarioDescriptor, ViewportClass};

const LEFT_WIDTH: usize = 20;
const MID_WIDTH: usize = 62;
const RIGHT_WIDTH: usize = 32;

pub fn render_launchpad_frame(
    scenario: LabScenarioDescriptor,
    viewport: ViewportClass,
    projection: &FireHorseProjection,
) -> LabRenderedFrame {
    let size = viewport.wtd_size();
    let mut lines = Vec::new();

    lines.push(full_row("Fire Horse Launchpad".to_string()));
    lines.push(full_row(format!(
        "Identity Rail | {} | {} | Launchpad | Target none | Ready",
        projection.identity.product, projection.identity.workspace_label,
    )));
    lines.push(rule(size.width as usize));
    lines.push(full_row(
        "Start Surface | Open Project | Create Project | Import | Console Fit".to_string(),
    ));
    lines.push(rule(size.width as usize));
    lines.push(columns(
        "Recent",
        "project | target | health | recency",
        "Capability",
    ));

    let recent_rows = projection
        .activity_deck
        .rows
        .iter()
        .filter_map(|row| match row {
            ActivityRowProjection::Text { source, text }
                if *source == "SessionStore::recent_workspaces" =>
            {
                Some(text.as_str())
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    let context = unavailable_context(projection);
    for (idx, recent) in recent_rows.iter().enumerate() {
        lines.push(columns(
            &format!("recent {}", idx + 1),
            recent,
            if idx == 0 {
                "F10 Console Fit"
            } else {
                "fixture only"
            },
        ));
    }

    lines.push(rule(size.width as usize));
    lines.push(full_row("Context Dock: Start Context".to_string()));
    lines.push(full_row(context));
    lines.push(rule(size.width as usize));
    lines.push(full_row("Activity: Recent".to_string()));
    for recent in recent_rows {
        lines.push(full_row(format!(
            "SessionStore fixture | {}",
            recent.replace(" | ", " / ")
        )));
    }
    lines.push(rule(size.width as usize));
    lines.push(full_row(key_rail_text(projection)));

    LabRenderedFrame {
        scenario,
        viewport,
        size,
        text: fixed_width_text(&lines, size.width, size.height),
    }
}

pub fn render_editing_lens_frame(
    scenario: LabScenarioDescriptor,
    viewport: ViewportClass,
    projection: &FireHorseProjection,
) -> LabRenderedFrame {
    if is_high_end(viewport) {
        return render_editing_lens_high_end_frame(scenario, viewport, projection);
    }

    let size = viewport.wtd_size();
    let mut lines = Vec::new();

    lines.push(full_row("Fire Horse Editing Lens".to_string()));
    lines.push(full_row(format!(
        "Identity Rail | {} | {} | {} | Target {} | Ready | Ln {} Col {}",
        projection.identity.product,
        projection.identity.workspace_label,
        projection.identity.scene.name(),
        projection.identity.target.as_deref().unwrap_or("none"),
        projection
            .identity
            .cursor
            .map(|cursor| cursor.line)
            .unwrap_or_default(),
        projection
            .identity
            .cursor
            .map(|cursor| cursor.column)
            .unwrap_or_default(),
    )));
    lines.push(rule(size.width as usize));
    lines.push(columns(
        "Project Spine",
        &format!("Code Canvas | {}", projection.code_canvas.document_label),
        "Context Dock",
    ));
    lines.push(rule(size.width as usize));

    let spine = projection.project_spine.as_ref();
    let diagnostic = first_diagnostic(projection);
    let source_lines = projection
        .code_canvas
        .lines
        .iter()
        .map(|line| {
            let marker = if line.markers.is_empty() { " " } else { "!" };
            format!("{:>3} {} {}", line.number, marker, line.text)
        })
        .collect::<Vec<_>>();

    let project_rows = project_rows(spine);
    let context_rows = context_rows(projection, diagnostic);

    for row in 0..9 {
        lines.push(columns(
            project_rows.get(row).map(String::as_str).unwrap_or(""),
            source_lines.get(row).map(String::as_str).unwrap_or(""),
            context_rows.get(row).map(String::as_str).unwrap_or(""),
        ));
    }

    if let Some(lens) = &projection.code_canvas.lens {
        lines.push(columns(
            "",
            &format!("Source Lens | {}", lens.title),
            "Symbol | PriceFor",
        ));
        lines.push(columns(
            "",
            &format!(
                "{} | {}",
                lens.source.provider,
                lens.body
                    .first()
                    .map(String::as_str)
                    .unwrap_or("Source lens body unavailable")
            ),
            "F12 definition | Shift+F12 references",
        ));
        lines.push(columns(
            "",
            &format!(
                "Actions: {}",
                lens.actions
                    .iter()
                    .map(|action| action.action_id)
                    .collect::<Vec<_>>()
                    .join(" | ")
            ),
            "Pinned: semantic.lens.pin",
        ));
    } else {
        lines.push(columns(
            "",
            "Source Lens unavailable | HostWorkspaceSession::hover",
            "Unavailable seam",
        ));
        lines.push(columns(
            "",
            "No source lens projection available from current ShellState",
            "W060 handoff",
        ));
        lines.push(columns(
            "",
            "Actions: semantic.hover display only",
            "Pinned: unavailable",
        ));
    }
    lines.push(rule(size.width as usize));
    lines.push(full_row(
        "Activity: Problems | Output | References | Watch/Trace".to_string(),
    ));
    if let Some(diagnostic) = diagnostic {
        lines.push(full_row(format!(
            "Problems 1 | {} {}:{} | {} | {}",
            diagnostic.code,
            projection.code_canvas.document_label,
            diagnostic.range.start.line,
            severity_label(diagnostic),
            diagnostic.message
        )));
    }
    lines.push(full_row(
        "Output ready | Last semantic pass: HostWorkspaceSession::diagnostics".to_string(),
    ));
    lines.push(rule(size.width as usize));
    lines.push(full_row(key_rail_text(projection)));

    LabRenderedFrame {
        scenario,
        viewport,
        size,
        text: fixed_width_text(&lines, size.width, size.height),
    }
}

pub fn render_command_lens_frame(
    scenario: LabScenarioDescriptor,
    viewport: ViewportClass,
    projection: &FireHorseProjection,
) -> LabRenderedFrame {
    if is_high_end(viewport) {
        return render_command_lens_high_end_frame(scenario, viewport, projection);
    }

    let size = viewport.wtd_size();
    let mut lines = Vec::new();
    let command_lens = match projection.overlay.as_ref() {
        Some(OverlayProjection::CommandLens(command_lens)) => command_lens,
        _ => {
            return render_editing_lens_frame(scenario, viewport, projection);
        }
    };

    lines.push(full_row("Fire Horse Command Lens".to_string()));
    lines.push(full_row(format!(
        "Identity Rail | {} | {} | Command Lens | Target {} | Backing scene inactive",
        projection.identity.product,
        projection.identity.workspace_label,
        projection.identity.target.as_deref().unwrap_or("none"),
    )));
    lines.push(rule(size.width as usize));
    lines.push(full_row(
        "Backing: Project Spine | Code Canvas | Context Dock".to_string(),
    ));
    lines.push(full_row(
        "Overlay: Command Lens | filter: run | focus: command rows".to_string(),
    ));
    lines.push(rule(size.width as usize));
    lines.push(columns(
        "Binding",
        "Command row | action id | state",
        "Preview",
    ));
    lines.push(rule(size.width as usize));

    for row in &command_lens.rows {
        let binding = row
            .binding
            .as_ref()
            .map(|binding| binding.label.as_str())
            .unwrap_or("");
        let state = if row.enabled {
            "enabled".to_string()
        } else {
            format!(
                "disabled: {}",
                row.disabled_reason.as_deref().unwrap_or("reason missing")
            )
        };
        let selected = if row.action_id == command_lens.selected_action_id {
            ">"
        } else {
            " "
        };
        lines.push(columns(
            binding,
            &format!("{selected} {} | {} | {state}", row.label, row.action_id),
            &row.preview.title,
        ));
    }

    lines.push(rule(size.width as usize));
    lines.push(full_row(format!(
        "Preview: {} | selected action id: {}",
        command_lens.preview.title, command_lens.selected_action_id
    )));
    for body in &command_lens.preview.body {
        lines.push(full_row(body.clone()));
    }
    lines.push(rule(size.width as usize));
    lines.push(full_row(command_lens_key_rail_text(command_lens)));

    LabRenderedFrame {
        scenario,
        viewport,
        size,
        text: fixed_width_text(&lines, size.width, size.height),
    }
}

pub fn render_run_lane_frame(
    scenario: LabScenarioDescriptor,
    viewport: ViewportClass,
    projection: &FireHorseProjection,
) -> LabRenderedFrame {
    if is_high_end(viewport) {
        return render_run_lane_high_end_frame(scenario, viewport, projection);
    }

    let size = viewport.wtd_size();
    let mut lines = Vec::new();

    lines.push(full_row("Fire Horse Run Lane".to_string()));
    lines.push(full_row(format!(
        "Identity Rail | {} | {} | Run Lane | Target {} | run.start active",
        projection.identity.product,
        projection.identity.workspace_label,
        projection.identity.target.as_deref().unwrap_or("none"),
    )));
    lines.push(rule(size.width as usize));
    lines.push(full_row(
        "Run Lane | staged execution timeline from MockRunEventProjection".to_string(),
    ));
    lines.push(columns("Step", "Status | target | message", "Context Dock"));
    lines.push(rule(size.width as usize));

    for event in &projection.seams.run_events {
        lines.push(columns(
            run_step_name(event.step),
            &format!(
                "{} | {} | {}",
                run_status_name(event.status),
                event.target_id,
                event.message
            ),
            run_context_label(event),
        ));
    }

    lines.push(rule(size.width as usize));
    lines.push(full_row(
        "Activity: Run Timeline | Output | Immediate".to_string(),
    ));
    for event in &projection.seams.run_events {
        lines.push(full_row(format!(
            "{} {} at {}ms | {}",
            run_step_name(event.step),
            run_status_name(event.status),
            event.emitted_at_ms,
            event.message
        )));
    }
    lines.push(rule(size.width as usize));
    lines.push(full_row(key_rail_text(projection)));

    LabRenderedFrame {
        scenario,
        viewport,
        size,
        text: fixed_width_text(&lines, size.width, size.height),
    }
}

pub fn render_debug_cockpit_frame(
    scenario: LabScenarioDescriptor,
    viewport: ViewportClass,
    projection: &FireHorseProjection,
) -> LabRenderedFrame {
    if is_high_end(viewport) {
        return render_debug_cockpit_high_end_frame(scenario, viewport, projection);
    }

    let size = viewport.wtd_size();
    let mut lines = Vec::new();
    let current_frame = projection.seams.debug_frames.first();

    lines.push(full_row("Fire Horse Debug Cockpit".to_string()));
    lines.push(full_row(format!(
        "Identity Rail | {} | {} | Debug Cockpit | Target {} | paused {}:{}",
        projection.identity.product,
        projection.identity.workspace_label,
        projection.identity.target.as_deref().unwrap_or("none"),
        projection.code_canvas.document_label,
        projection.code_canvas.execution_line.unwrap_or_default(),
    )));
    lines.push(rule(size.width as usize));
    lines.push(columns(
        "Code Canvas",
        "Paused Line | Call Stack",
        "Context Dock",
    ));
    lines.push(rule(size.width as usize));

    for line in &projection.code_canvas.lines {
        let marker = if projection.code_canvas.execution_line == Some(line.number) {
            "=>"
        } else {
            "  "
        };
        let call_stack = current_frame
            .map(|frame| format!("{} @ line {}", frame.procedure, frame.line))
            .unwrap_or_else(|| "unavailable".to_string());
        let context = if line.number == projection.code_canvas.execution_line.unwrap_or_default() {
            "Debug Cockpit | paused"
        } else {
            ""
        };
        lines.push(columns(
            &format!("{:>3} {} {}", line.number, marker, line.text),
            &call_stack,
            context,
        ));
    }

    lines.push(rule(size.width as usize));
    lines.push(full_row("Call Stack".to_string()));
    for frame in &projection.seams.debug_frames {
        lines.push(full_row(format!(
            "{} | {} | {}:{} | {}",
            frame.frame_id, frame.procedure, frame.document_id, frame.line, frame.source.provider
        )));
    }
    lines.push(full_row("Locals".to_string()));
    for local in &projection.seams.locals {
        lines.push(full_row(format!(
            "{} As {} = {} | {}",
            local.name, local.type_label, local.value, local.source.provider
        )));
    }
    lines.push(full_row("Watches".to_string()));
    for watch in &projection.seams.watches {
        lines.push(full_row(format!(
            "{} As {} = {} | {}",
            watch.expression, watch.type_label, watch.value, watch.source.provider
        )));
    }
    lines.push(rule(size.width as usize));
    lines.push(full_row(
        "Activity: Watch/Trace | Immediate | Output".to_string(),
    ));
    lines.push(full_row(key_rail_text(projection)));

    LabRenderedFrame {
        scenario,
        viewport,
        size,
        text: fixed_width_text(&lines, size.width, size.height),
    }
}

pub fn render_console_fit_frame(
    scenario: LabScenarioDescriptor,
    viewport: ViewportClass,
    projection: &FireHorseProjection,
) -> LabRenderedFrame {
    let size = viewport.wtd_size();
    let mut lines = Vec::new();
    let fit = projection.terminal_fit.as_ref();

    lines.push(full_row("Fire Horse Console Fit".to_string()));
    lines.push(full_row(format!(
        "Identity Rail | {} | {} | Console Fit | Theme Paper Ember | Labels visible without color",
        projection.identity.product, projection.identity.workspace_label,
    )));
    lines.push(rule(size.width as usize));
    lines.push(full_row(
        fit.map(|fit| format!("Capability Summary | {}", fit.summary))
            .unwrap_or_else(|| "Capability Summary | unavailable".to_string()),
    ));
    lines.push(rule(size.width as usize));
    lines.push(columns("Signal", "result | detail", "Recommendation"));
    lines.push(rule(size.width as usize));

    if let Some(fit) = fit {
        for row in &fit.rows {
            lines.push(columns(
                row.signal,
                &format!("{} | {}", fit_result_name(row.result), row.detail),
                &row.recommendation,
            ));
        }
    }

    lines.push(rule(size.width as usize));
    lines.push(full_row(
        "Activity: Signals | terminal capability rows are fixtures, not live probes".to_string(),
    ));
    for row in terminal_fit_activity_rows(fit) {
        lines.push(full_row(row));
    }
    lines.push(rule(size.width as usize));
    lines.push(full_row(key_rail_text(projection)));

    LabRenderedFrame {
        scenario,
        viewport,
        size,
        text: fixed_width_text(&lines, size.width, size.height),
    }
}

pub fn render_compact_focus_frame(
    scenario: LabScenarioDescriptor,
    viewport: ViewportClass,
    projection: &FireHorseProjection,
) -> LabRenderedFrame {
    let size = viewport.wtd_size();
    let mut lines = Vec::new();
    let diagnostic = first_diagnostic(projection);

    lines.push(full_row("Fire Horse Compact Focus".to_string()));
    lines.push(full_row(format!(
        "Identity Rail | {} | {} | Compact Focus | Target {} | Ln {} Col {}",
        projection.identity.product,
        projection.identity.workspace_label,
        projection.identity.target.as_deref().unwrap_or("none"),
        projection
            .identity
            .cursor
            .map(|cursor| cursor.line)
            .unwrap_or_default(),
        projection
            .identity
            .cursor
            .map(|cursor| cursor.column)
            .unwrap_or_default(),
    )));
    lines.push(rule(size.width as usize));
    lines.push(full_row(
        "Project Spine: hidden | Context Dock: hidden | temporary docks Alt+1/Alt+3/Alt+4"
            .to_string(),
    ));
    lines.push(rule(size.width as usize));
    lines.push(full_row(format!(
        "Code Canvas | {} | {}",
        projection.code_canvas.document_label, projection.code_canvas.language
    )));
    lines.push(rule(size.width as usize));

    for line in &projection.code_canvas.lines {
        let marker = if line.markers.is_empty() { " " } else { "!" };
        lines.push(full_row(format!(
            "{:>3} {} {}",
            line.number, marker, line.text
        )));
    }

    if let Some(lens) = &projection.code_canvas.lens {
        lines.push(rule(size.width as usize));
        lines.push(full_row(format!("Source Lens | {}", lens.title)));
        for body in &lens.body {
            lines.push(full_row(body.clone()));
        }
    }

    lines.push(rule(size.width as usize));
    lines.push(full_row("Activity Rail: Problems".to_string()));
    if let Some(diagnostic) = diagnostic {
        lines.push(full_row(format!(
            "{} {}:{} | {}",
            diagnostic.code,
            projection.code_canvas.document_label,
            diagnostic.range.start.line,
            diagnostic.message
        )));
    }
    lines.push(rule(size.width as usize));
    lines.push(full_row(key_rail_text(projection)));

    LabRenderedFrame {
        scenario,
        viewport,
        size,
        text: fixed_width_text(&lines, size.width, size.height),
    }
}

pub fn key_rail_text(projection: &FireHorseProjection) -> String {
    projection
        .key_rail
        .hints
        .iter()
        .map(|hint| format!("{} {}", hint.binding.label, hint.label))
        .collect::<Vec<_>>()
        .join("  ")
}

#[derive(Clone, Copy, Debug)]
struct HighEndLayout {
    width: usize,
    spine: usize,
    canvas: usize,
    dock: usize,
}

impl HighEndLayout {
    fn new(viewport: ViewportClass) -> Self {
        let size = viewport.wtd_size();
        let width = size.width as usize;
        let (spine, dock) = match viewport {
            ViewportClass::Studio => (34, 48),
            _ => (30, 42),
        };
        let canvas = width.saturating_sub(spine + dock + 6);
        Self {
            width,
            spine,
            canvas,
            dock,
        }
    }
}

fn is_high_end(viewport: ViewportClass) -> bool {
    matches!(viewport, ViewportClass::FirstClass | ViewportClass::Studio)
}

fn render_editing_lens_high_end_frame(
    scenario: LabScenarioDescriptor,
    viewport: ViewportClass,
    projection: &FireHorseProjection,
) -> LabRenderedFrame {
    let size = viewport.wtd_size();
    let layout = HighEndLayout::new(viewport);
    let mut lines = Vec::new();
    let diagnostic = first_diagnostic(projection);
    let project_rows = project_rows(projection.project_spine.as_ref());
    let source_rows = projection
        .code_canvas
        .lines
        .iter()
        .map(|line| {
            let marker = if line.markers.is_empty() { " " } else { "!" };
            let active = if projection
                .code_canvas
                .selection
                .is_some_and(|range| range.start.line == line.number)
            {
                ">"
            } else {
                " "
            };
            format!("{:>3} {}{}  {}", line.number, active, marker, line.text)
        })
        .collect::<Vec<_>>();
    let context_rows = vec![
        format!(
            "Diagnostic  {}",
            diagnostic
                .map(|diagnostic| format!("{} {}", severity_label(diagnostic), diagnostic.code))
                .unwrap_or_else(|| "clean".to_string())
        ),
        diagnostic
            .map(|diagnostic| {
                format!(
                    "line {}  {}",
                    diagnostic.range.start.line, diagnostic.message
                )
            })
            .unwrap_or_else(|| "No diagnostics from HostWorkspaceSession".to_string()),
        "Symbol      PriceFor".to_string(),
        "Kind        Function".to_string(),
        "Actions     F12 definition  Shift+F12 refs".to_string(),
        "Run Status  idle; last run clean".to_string(),
        "Deck        Problems 1 | References 4".to_string(),
    ];

    lines.push(high_title(
        projection,
        viewport,
        "Fire Horse Editing Lens",
        "source-centered desktop target",
    ));
    lines.push(high_rule(layout.width));
    lines.push(high_columns(
        layout,
        "PROJECT SPINE",
        &format!(
            "CODE CANVAS  {}  {}",
            projection.code_canvas.document_label, projection.code_canvas.language
        ),
        "CONTEXT DOCK",
    ));
    lines.push(high_rule(layout.width));

    let row_count = source_rows.len().max(context_rows.len()).max(9);
    for row in 0..row_count {
        lines.push(high_columns(
            layout,
            project_rows.get(row).map(String::as_str).unwrap_or(""),
            source_rows.get(row).map(String::as_str).unwrap_or(""),
            context_rows.get(row).map(String::as_str).unwrap_or(""),
        ));
    }

    lines.push(high_rule(layout.width));
    if let Some(lens) = &projection.code_canvas.lens {
        lines.push(high_columns(
            layout,
            "SEMANTIC LENS",
            &format!("{}  ::  {}", lens.title, lens.source.provider),
            "PINNED ACTIONS",
        ));
        lines.push(high_columns(
            layout,
            "",
            lens.body
                .first()
                .map(String::as_str)
                .unwrap_or("No lens body"),
            "F1 pin  F12 go  Shift+F12 refs",
        ));
        lines.push(high_columns(
            layout,
            "",
            &format!(
                "Actions: {}",
                lens.actions
                    .iter()
                    .map(|action| action.action_id)
                    .collect::<Vec<_>>()
                    .join("  ")
            ),
            "W060 replaces fixture hover",
        ));
    }
    lines.push(high_rule(layout.width));
    lines.push(high_full(
        layout.width,
        "ACTIVITY DECK  Problems (1)  |  Output ready  |  References (4)  |  Watch/Trace",
    ));
    if let Some(diagnostic) = diagnostic {
        lines.push(high_full(
            layout.width,
            &format!(
                "warning {}  {}:{}  {}",
                diagnostic.code,
                projection.code_canvas.document_label,
                diagnostic.range.start.line,
                diagnostic.message
            ),
        ));
    }
    lines.push(high_rule(layout.width));
    lines.push(high_full(layout.width, &key_rail_text(projection)));

    LabRenderedFrame {
        scenario,
        viewport,
        size,
        text: fixed_width_text(&lines, size.width, size.height),
    }
}

fn render_command_lens_high_end_frame(
    scenario: LabScenarioDescriptor,
    viewport: ViewportClass,
    projection: &FireHorseProjection,
) -> LabRenderedFrame {
    let size = viewport.wtd_size();
    let layout = HighEndLayout::new(viewport);
    let command_lens = match projection.overlay.as_ref() {
        Some(OverlayProjection::CommandLens(command_lens)) => command_lens,
        _ => return render_editing_lens_high_end_frame(scenario, viewport, projection),
    };
    let action_width = if matches!(viewport, ViewportClass::Studio) {
        88
    } else {
        74
    };
    let preview_width = layout.width.saturating_sub(action_width + 5);
    let mut lines = Vec::new();

    lines.push(high_title(
        projection,
        viewport,
        "Fire Horse Command Lens",
        "modal command surface",
    ));
    lines.push(high_rule(layout.width));
    lines.push(high_full(
        layout.width,
        &format!(
            "COMMAND LENS  filter: '{}'  selected: {}  backing scene: inactive",
            command_lens.filter, command_lens.selected_action_id
        ),
    ));
    lines.push(high_rule(layout.width));
    lines.push(format!(
        "{} │ {}",
        cell("ACTIONS", action_width),
        cell("PREVIEW", preview_width)
    ));
    lines.push(high_rule(layout.width));

    for row in &command_lens.rows {
        let binding = row
            .binding
            .as_ref()
            .map(|binding| binding.label.as_str())
            .unwrap_or("");
        let state = if row.enabled {
            "enabled".to_string()
        } else {
            format!(
                "disabled: {}",
                row.disabled_reason.as_deref().unwrap_or("reason missing")
            )
        };
        let selected = if row.action_id == command_lens.selected_action_id {
            ">"
        } else {
            " "
        };
        lines.push(format!(
            "{} │ {}",
            cell(
                &format!(
                    "{} {:<20} {:<14} {:<26} {}",
                    selected, row.label, binding, row.action_id, state
                ),
                action_width
            ),
            cell(&row.preview.title, preview_width)
        ));
    }

    lines.push(high_rule(layout.width));
    lines.push(format!(
        "{} │ {}",
        cell("CONSEQUENCE PREVIEW", action_width),
        cell(&command_lens.preview.title, preview_width)
    ));
    for body in &command_lens.preview.body {
        lines.push(format!(
            "{} │ {}",
            cell("", action_width),
            cell(body, preview_width)
        ));
    }
    lines.push(high_rule(layout.width));
    lines.push(high_full(
        layout.width,
        &command_lens_key_rail_text(command_lens),
    ));

    LabRenderedFrame {
        scenario,
        viewport,
        size,
        text: fixed_width_text(&lines, size.width, size.height),
    }
}

fn render_run_lane_high_end_frame(
    scenario: LabScenarioDescriptor,
    viewport: ViewportClass,
    projection: &FireHorseProjection,
) -> LabRenderedFrame {
    let size = viewport.wtd_size();
    let layout = HighEndLayout::new(viewport);
    let mut lines = Vec::new();
    let source_rows = projection
        .code_canvas
        .lines
        .iter()
        .map(|line| format!("{:>3}    {}", line.number, line.text))
        .collect::<Vec<_>>();
    let event_rows = projection
        .seams
        .run_events
        .iter()
        .map(|event| {
            format!(
                "{:<8} {:<8} {:>4}ms  {}",
                run_step_name(event.step),
                run_status_name(event.status),
                event.emitted_at_ms,
                event.message
            )
        })
        .collect::<Vec<_>>();
    let context_rows = projection
        .seams
        .run_events
        .iter()
        .map(|event| {
            format!(
                "{}  {}",
                run_step_name(event.step),
                run_context_label(event)
            )
        })
        .collect::<Vec<_>>();

    lines.push(high_title(
        projection,
        viewport,
        "Fire Horse Run Lane",
        "workflow rail plus source continuity",
    ));
    lines.push(high_rule(layout.width));
    lines.push(high_full(
        layout.width,
        "RUN  prepare ok  >  analyze ok  >  build active  >  execute pending  >  result pending",
    ));
    lines.push(high_rule(layout.width));
    lines.push(high_columns(
        layout,
        "RUN TIMELINE",
        &format!("CODE CANVAS  {}", projection.code_canvas.document_label),
        "RUN CONTEXT",
    ));
    lines.push(high_rule(layout.width));

    let row_count = event_rows
        .len()
        .max(source_rows.len())
        .max(context_rows.len());
    for row in 0..row_count {
        lines.push(high_columns(
            layout,
            event_rows.get(row).map(String::as_str).unwrap_or(""),
            source_rows.get(row).map(String::as_str).unwrap_or(""),
            context_rows.get(row).map(String::as_str).unwrap_or(""),
        ));
    }

    lines.push(high_rule(layout.width));
    lines.push(high_full(
        layout.width,
        "ACTIVITY DECK  Run Timeline active  |  Output structured  |  Immediate ready",
    ));
    for event in &projection.seams.run_events {
        lines.push(high_full(
            layout.width,
            &format!(
                "{} {}  target {}  {}",
                run_step_name(event.step),
                run_status_name(event.status),
                event.target_id,
                event.message
            ),
        ));
    }
    lines.push(high_rule(layout.width));
    lines.push(high_full(layout.width, &key_rail_text(projection)));

    LabRenderedFrame {
        scenario,
        viewport,
        size,
        text: fixed_width_text(&lines, size.width, size.height),
    }
}

fn render_debug_cockpit_high_end_frame(
    scenario: LabScenarioDescriptor,
    viewport: ViewportClass,
    projection: &FireHorseProjection,
) -> LabRenderedFrame {
    let size = viewport.wtd_size();
    let layout = HighEndLayout::new(viewport);
    let mut lines = Vec::new();
    let source_rows = projection
        .code_canvas
        .lines
        .iter()
        .map(|line| {
            let marker = if projection.code_canvas.execution_line == Some(line.number) {
                "=>"
            } else {
                "  "
            };
            format!("{:>3} {} {}", line.number, marker, line.text)
        })
        .collect::<Vec<_>>();
    let mut debug_rows = Vec::new();
    debug_rows.push("CALL STACK".to_string());
    for frame in &projection.seams.debug_frames {
        debug_rows.push(format!(
            "{}  {}  line {}",
            frame.frame_id, frame.procedure, frame.line
        ));
    }
    debug_rows.push("LOCALS".to_string());
    for local in &projection.seams.locals {
        debug_rows.push(format!(
            "{} As {} = {}",
            local.name, local.type_label, local.value
        ));
    }
    debug_rows.push("WATCHES".to_string());
    for watch in &projection.seams.watches {
        debug_rows.push(format!(
            "{} As {} = {}",
            watch.expression, watch.type_label, watch.value
        ));
    }
    let project_rows = project_rows(projection.project_spine.as_ref());

    lines.push(high_title(
        projection,
        viewport,
        "Fire Horse Debug Cockpit",
        "paused source plus debug dock",
    ));
    lines.push(high_rule(layout.width));
    lines.push(high_full(
        layout.width,
        &format!(
            "DEBUG  paused at {}:{}  breakpoint hit  W080 owns real debug contract",
            projection.code_canvas.document_label,
            projection.code_canvas.execution_line.unwrap_or_default()
        ),
    ));
    lines.push(high_rule(layout.width));
    lines.push(high_columns(
        layout,
        "PROJECT / BREAKPOINTS",
        &format!("CODE CANVAS  {}", projection.code_canvas.document_label),
        "DEBUG DOCK",
    ));
    lines.push(high_rule(layout.width));

    let row_count = source_rows
        .len()
        .max(debug_rows.len())
        .max(project_rows.len());
    for row in 0..row_count {
        lines.push(high_columns(
            layout,
            project_rows.get(row).map(String::as_str).unwrap_or(""),
            source_rows.get(row).map(String::as_str).unwrap_or(""),
            debug_rows.get(row).map(String::as_str).unwrap_or(""),
        ));
    }

    lines.push(high_rule(layout.width));
    lines.push(high_full(
        layout.width,
        "ACTIVITY DECK  Watch/Trace active  |  Immediate ? answer = 42  |  Output paused",
    ));
    lines.push(high_full(
        layout.width,
        "Immediate context: frame-0 PriceFor  |  Watches can be pinned from source or Immediate",
    ));
    lines.push(high_rule(layout.width));
    lines.push(high_full(layout.width, &key_rail_text(projection)));

    LabRenderedFrame {
        scenario,
        viewport,
        size,
        text: fixed_width_text(&lines, size.width, size.height),
    }
}

fn high_title(
    projection: &FireHorseProjection,
    viewport: ViewportClass,
    scene: &str,
    posture: &str,
) -> String {
    let size = viewport.wtd_size();
    format!(
        "OxIde  {}  target:{}  {}  {}  viewport:{} {}x{}",
        projection.identity.workspace_label,
        projection.identity.target.as_deref().unwrap_or("none"),
        scene,
        posture,
        viewport.name(),
        size.width,
        size.height
    )
}

fn high_rule(width: usize) -> String {
    "─".repeat(width)
}

fn high_columns(layout: HighEndLayout, left: &str, middle: &str, right: &str) -> String {
    format!(
        "{} │ {} │ {}",
        cell(left, layout.spine),
        cell(middle, layout.canvas),
        cell(right, layout.dock)
    )
}

fn high_full(width: usize, text: &str) -> String {
    cell(text, width)
}

fn unavailable_context(projection: &FireHorseProjection) -> String {
    projection
        .context_dock
        .as_ref()
        .and_then(|dock| {
            dock.cards.iter().find_map(|card| match card {
                ContextCardProjection::Unavailable(unavailable) => Some(format!(
                    "{} unavailable | {}",
                    unavailable.source, unavailable.reason
                )),
                _ => None,
            })
        })
        .unwrap_or_else(|| "ProjectSession unavailable | no launchpad context".to_string())
}

fn terminal_fit_activity_rows(fit: Option<&TerminalFitProjection>) -> Vec<String> {
    fit.map(|fit| {
        fit.rows
            .iter()
            .map(|row| {
                format!(
                    "{}: {} | {} | {}",
                    row.signal,
                    fit_result_name(row.result),
                    row.detail,
                    row.recommendation
                )
            })
            .collect()
    })
    .unwrap_or_else(|| vec!["terminal-fit: unavailable".to_string()])
}

fn fit_result_name(result: FitResult) -> &'static str {
    match result {
        FitResult::Pass => "pass",
        FitResult::Warn => "warn",
        FitResult::Fail => "fail",
    }
}

fn run_context_label(event: &MockRunEventProjection) -> &'static str {
    match event.status {
        RunStepStatus::Active => "active step",
        RunStepStatus::Failed => "failed step",
        RunStepStatus::Complete => "complete",
        RunStepStatus::Pending => "pending",
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

pub fn command_lens_key_rail_text(command_lens: &CommandLensProjection) -> String {
    command_lens
        .footer_hints
        .iter()
        .map(|hint| {
            let label = if hint.action_id == "command.execute_selected" {
                "run"
            } else {
                hint.label.as_str()
            };
            format!("{} {}", hint.binding.label, label)
        })
        .collect::<Vec<_>>()
        .join("  ")
}

fn project_rows(spine: Option<&ProjectSpineProjection>) -> Vec<String> {
    spine
        .map(|spine| {
            spine
                .rows
                .iter()
                .map(|row| {
                    let active = if row.active { ">" } else { " " };
                    let dirty = if row.dirty { "*" } else { " " };
                    format!(
                        "{}{}{}{}",
                        active,
                        dirty,
                        " ".repeat(row.depth as usize * 2),
                        row.label
                    )
                })
                .collect()
        })
        .unwrap_or_else(|| vec!["hidden".to_string()])
}

fn context_rows(
    projection: &FireHorseProjection,
    diagnostic: Option<&MockDiagnosticProjection>,
) -> Vec<String> {
    let mut rows = vec![
        "Diagnostic".to_string(),
        "Symbol".to_string(),
        "Run Status".to_string(),
        "References".to_string(),
    ];
    if let Some(diagnostic) = diagnostic {
        rows.insert(
            1,
            format!("{} {}", severity_label(diagnostic), diagnostic.code),
        );
        rows.insert(
            2,
            format!(
                "line {}: {}",
                diagnostic.range.start.line, diagnostic.message
            ),
        );
    } else if let Some(dock) = &projection.context_dock {
        let unavailable = dock.cards.iter().filter_map(|card| match card {
            ContextCardProjection::Unavailable(unavailable) => {
                Some(format!("{} | {}", unavailable.source, unavailable.reason))
            }
            _ => None,
        });
        rows.extend(unavailable);
    }
    rows
}

fn first_diagnostic(projection: &FireHorseProjection) -> Option<&MockDiagnosticProjection> {
    projection.context_dock.as_ref().and_then(|dock| {
        dock.cards.iter().find_map(|card| match card {
            ContextCardProjection::Diagnostic(diagnostic) => Some(diagnostic),
            _ => None,
        })
    })
}

fn severity_label(diagnostic: &MockDiagnosticProjection) -> &'static str {
    match diagnostic.severity {
        super::projection::DiagnosticSeverity::Error => "error",
        super::projection::DiagnosticSeverity::Warning => "warning",
        super::projection::DiagnosticSeverity::Info => "info",
        super::projection::DiagnosticSeverity::Hint => "hint",
    }
}

fn columns(left: &str, middle: &str, right: &str) -> String {
    format!(
        "{} | {} | {}",
        cell(left, LEFT_WIDTH),
        cell(middle, MID_WIDTH),
        cell(right, RIGHT_WIDTH)
    )
}

fn full_row(text: String) -> String {
    text
}

fn rule(width: usize) -> String {
    "-".repeat(width)
}

fn cell(text: &str, width: usize) -> String {
    let mut out: String = text.chars().take(width).collect();
    let len = out.chars().count();
    for _ in len..width {
        out.push(' ');
    }
    out
}

fn fixed_width_text(lines: &[String], width: u16, height: u16) -> String {
    let width = width as usize;
    let height = height as usize;
    let mut output = String::new();

    for row in 0..height {
        if row > 0 {
            output.push('\n');
        }
        let mut line = lines.get(row).cloned().unwrap_or_default();
        let mut line_len = line.chars().count();
        if line_len > width {
            line = line.chars().take(width).collect();
            line_len = width;
        }
        output.push_str(&line);
        for _ in line_len..width {
            output.push(' ');
        }
    }
    output.push('\n');
    output
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
    fn launchpad_rows_are_mru_shaped_and_render_start_posture() {
        let projection = projection_for_scenario("firehorse-launchpad-standard")
            .expect("launchpad fixture should load");
        let rows = projection
            .activity_deck
            .rows
            .iter()
            .filter_map(|row| match row {
                ActivityRowProjection::Text { source, text }
                    if *source == "SessionStore::recent_workspaces" =>
                {
                    Some(text)
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        assert!(!rows.is_empty());
        for row in &rows {
            let parts = row.split(" | ").collect::<Vec<_>>();
            assert_eq!(
                parts.len(),
                4,
                "MRU row should project project/target/health/recency"
            );
            assert!(!parts[0].is_empty());
            assert!(!parts[1].is_empty());
            assert!(!parts[2].is_empty());
            assert!(!parts[3].is_empty());
        }

        let rendered = render_launchpad_frame(
            scenario("firehorse-launchpad-standard"),
            ViewportClass::Standard,
            &projection,
        );
        assert!(rendered.text.contains("Fire Horse Launchpad"));
        assert!(rendered.text.contains("Open Project"));
        assert!(
            rendered
                .text
                .contains("NorthwindPricing | ExcelDesktop | healthy | 2h ago")
        );
        assert!(rendered.text.contains("ProjectSession unavailable"));
        assert!(rendered.text.contains("F10 Console Fit"));
    }

    #[test]
    fn editing_lens_renderer_exposes_required_regions() {
        let projection = projection_for_scenario("firehorse-editing-lens-standard")
            .expect("editing fixture should load");
        let rendered = render_editing_lens_frame(
            scenario("firehorse-editing-lens-standard"),
            ViewportClass::Standard,
            &projection,
        );

        assert!(rendered.text.contains("Identity Rail"));
        assert!(rendered.text.contains("Project Spine"));
        assert!(rendered.text.contains("Code Canvas"));
        assert!(rendered.text.contains("Source Lens"));
        assert!(rendered.text.contains("Context Dock"));
        assert!(rendered.text.contains("Activity: Problems"));
        assert!(rendered.text.contains("F6 Command Lens"));
        assert!(rendered.text.contains("PriceFor"));
    }

    #[test]
    fn editing_lens_key_rail_fits_standard_width() {
        let projection = projection_for_scenario("firehorse-editing-lens-standard")
            .expect("editing fixture should load");
        let key_rail = key_rail_text(&projection);

        assert!(key_rail.len() <= ViewportClass::Standard.wtd_size().width as usize);
        assert!(key_rail.contains("Ctrl+S Save"));
        assert!(key_rail.contains("F6 Command Lens"));
    }

    #[test]
    fn editing_lens_has_first_class_desktop_layout() {
        let projection = projection_for_scenario("firehorse-editing-lens-standard")
            .expect("editing fixture should load");
        let rendered = render_editing_lens_frame(
            scenario("firehorse-editing-lens-standard"),
            ViewportClass::FirstClass,
            &projection,
        );

        assert_eq!(rendered.size.width, 160);
        assert_eq!(rendered.size.height, 42);
        assert!(rendered.text.contains("viewport:first-class 160x42"));
        assert!(rendered.text.contains("SEMANTIC LENS"));
        assert!(rendered.text.contains("ACTIVITY DECK"));
        assert!(rendered.text.contains("CODE CANVAS"));
    }

    #[test]
    fn command_lens_renderer_exposes_overlay_contracts() {
        let projection = projection_for_scenario("firehorse-command-lens-standard")
            .expect("command lens fixture should load");
        let rendered = render_command_lens_frame(
            scenario("firehorse-command-lens-standard"),
            ViewportClass::Standard,
            &projection,
        );

        assert!(rendered.text.contains("Overlay: Command Lens"));
        assert!(rendered.text.contains("filter: run"));
        assert!(rendered.text.contains("Run Project"));
        assert!(rendered.text.contains("run.start"));
        assert!(rendered.text.contains("Stop Run"));
        assert!(rendered.text.contains("disabled: No active run"));
        assert!(rendered.text.contains("Enter run"));
    }

    #[test]
    fn command_lens_rows_have_action_ids_and_disabled_reasons() {
        let projection = projection_for_scenario("firehorse-command-lens-standard")
            .expect("command lens fixture should load");
        let command_lens = match projection.overlay {
            Some(OverlayProjection::CommandLens(command_lens)) => command_lens,
            _ => panic!("command lens overlay expected"),
        };

        assert!(
            command_lens
                .rows
                .iter()
                .all(|row| !row.action_id.is_empty())
        );
        let stop_run = command_lens
            .rows
            .iter()
            .find(|row| row.action_id == "run.stop")
            .expect("stop run row");
        assert!(!stop_run.enabled);
        assert_eq!(stop_run.disabled_reason.as_deref(), Some("No active run"));
    }

    #[test]
    fn run_lane_has_exactly_one_active_step_and_renders_timeline() {
        let projection = projection_for_scenario("firehorse-run-lane-standard")
            .expect("run lane fixture should load");
        let active_steps = projection
            .seams
            .run_events
            .iter()
            .filter(|event| event.status == RunStepStatus::Active)
            .count();
        assert_eq!(active_steps, 1);
        assert!(
            projection
                .seams
                .run_events
                .iter()
                .any(|event| event.step == RunStepKind::Build)
        );

        let rendered = render_run_lane_frame(
            scenario("firehorse-run-lane-standard"),
            ViewportClass::Standard,
            &projection,
        );
        assert!(rendered.text.contains("Run Lane"));
        assert!(rendered.text.contains("Activity: Run Timeline"));
        assert!(rendered.text.contains("Build"));
        assert!(rendered.text.contains("F8 Stop Run"));
    }

    #[test]
    fn debug_cockpit_has_paused_state_and_debug_key_rail() {
        let projection = projection_for_scenario("firehorse-debug-cockpit-standard")
            .expect("debug cockpit fixture should load");
        assert_eq!(projection.code_canvas.document_label, "PriceFor.bas");
        assert_eq!(projection.code_canvas.execution_line, Some(8));
        assert!(!projection.seams.debug_frames.is_empty());

        let rendered = render_debug_cockpit_frame(
            scenario("firehorse-debug-cockpit-standard"),
            ViewportClass::Standard,
            &projection,
        );
        assert!(rendered.text.contains("Debug Cockpit"));
        assert!(rendered.text.contains("paused PriceFor.bas:8"));
        assert!(rendered.text.contains("Call Stack"));
        assert!(rendered.text.contains("Locals"));
        assert!(rendered.text.contains("Watches"));
        assert!(rendered.text.contains("F5 Continue"));
        assert!(rendered.text.contains("F8 Step"));
        assert!(rendered.text.contains("Esc Return"));
    }

    #[test]
    fn debug_cockpit_has_studio_debug_dock_layout() {
        let projection = projection_for_scenario("firehorse-debug-cockpit-standard")
            .expect("debug cockpit fixture should load");
        let rendered = render_debug_cockpit_frame(
            scenario("firehorse-debug-cockpit-standard"),
            ViewportClass::Studio,
            &projection,
        );

        assert_eq!(rendered.size.width, 190);
        assert_eq!(rendered.size.height, 48);
        assert!(rendered.text.contains("viewport:studio 190x48"));
        assert!(rendered.text.contains("DEBUG DOCK"));
        assert!(rendered.text.contains("Immediate ? answer = 42"));
        assert!(rendered.text.contains("W080 owns real debug contract"));
    }

    #[test]
    fn console_fit_rows_have_text_results_and_recommendations() {
        let projection = projection_for_scenario("firehorse-console-fit-light")
            .expect("console fit fixture should load");
        let fit = projection.terminal_fit.as_ref().expect("terminal fit rows");
        assert!(!fit.rows.is_empty());
        for row in &fit.rows {
            assert!(!row.signal.is_empty());
            assert!(!fit_result_name(row.result).is_empty());
            assert!(!row.detail.is_empty());
            assert!(!row.recommendation.is_empty());
        }

        let rendered = render_console_fit_frame(
            scenario("firehorse-console-fit-light"),
            ViewportClass::Standard,
            &projection,
        );
        assert!(rendered.text.contains("Console Fit"));
        assert!(rendered.text.contains("Labels visible without color"));
        assert!(rendered.text.contains("truecolor"));
        assert!(rendered.text.contains("pass"));
        assert!(rendered.text.contains("warn"));
        assert!(rendered.text.contains("Prefer ASCII rail fallback"));
    }

    #[test]
    fn compact_focus_keeps_source_activity_and_key_rail_without_side_docks() {
        let projection = projection_for_scenario("firehorse-focus-compact")
            .expect("compact focus fixture should load");
        assert!(projection.project_spine.is_none());
        assert!(projection.context_dock.is_none());
        assert!(!projection.code_canvas.lines.is_empty());
        assert!(!projection.activity_deck.rows.is_empty());
        assert!(!projection.key_rail.hints.is_empty());

        let rendered = render_compact_focus_frame(
            scenario("firehorse-focus-compact"),
            ViewportClass::Compact,
            &projection,
        );
        assert_eq!(rendered.size.width, 92);
        assert!(rendered.text.contains("Compact Focus"));
        assert!(rendered.text.contains("Project Spine: hidden"));
        assert!(rendered.text.contains("Code Canvas | PriceFor.bas"));
        assert!(rendered.text.contains("Activity Rail: Problems"));
        assert!(rendered.text.contains("Alt+1 Project"));
        assert!(rendered.text.contains("F6 Command"));
    }
}
