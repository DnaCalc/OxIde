use ftui::layout::{Constraint, Flex, Rect};
use ftui::prelude::Frame;
use ftui::widgets::Widget;
use ftui::widgets::block::{Alignment, Block};
use ftui::widgets::borders::Borders;
use ftui::widgets::paragraph::Paragraph;

use super::mock_data::ShellPanels;
use super::model::ShellModel;
use super::state::FocusRegion;

pub fn render(model: &ShellModel, frame: &mut Frame) {
    let area = Rect::new(0, 0, frame.width(), frame.height());
    let root_sections = Flex::vertical()
        .constraints([Constraint::Fixed(3), Constraint::Fill, Constraint::Fixed(8)])
        .split(area);

    let panels = model.panels();

    render_panel(
        root_sections[0],
        frame,
        &active_title("OxIde Shell", model, FocusRegion::TopBar),
        &panels.top_bar,
    );

    if model.inspector_is_collapsed() {
        render_narrow_body(root_sections[1], frame, model, &panels);
    } else {
        render_wide_body(root_sections[1], frame, model, &panels);
    }

    render_panel(
        root_sections[2],
        frame,
        &active_title(
            &model.lower_surface_title(),
            model,
            FocusRegion::LowerSurface,
        ),
        &panels.lower_surface,
    );

    if model.palette_active() {
        render_palette_overlay(frame, model, &panels);
    }
}

fn render_wide_body(area: Rect, frame: &mut Frame, model: &ShellModel, panels: &ShellPanels) {
    let columns = Flex::horizontal()
        .constraints([
            Constraint::Percentage(20.0),
            Constraint::Percentage(58.0),
            Constraint::Fill,
        ])
        .split(area);

    render_panel(
        columns[0],
        frame,
        &active_title("Explorer", model, FocusRegion::Explorer),
        &panels.explorer,
    );
    render_panel(
        columns[1],
        frame,
        &active_title(&panels.editor_title, model, FocusRegion::Editor),
        &panels.editor,
    );
    render_panel(
        columns[2],
        frame,
        &active_title(&model.inspector_title(), model, FocusRegion::Inspector),
        &panels.inspector,
    );
}

fn render_narrow_body(area: Rect, frame: &mut Frame, model: &ShellModel, panels: &ShellPanels) {
    let columns = Flex::horizontal()
        .constraints([Constraint::Percentage(24.0), Constraint::Fill])
        .split(area);

    render_panel(
        columns[0],
        frame,
        &active_title("Explorer", model, FocusRegion::Explorer),
        &panels.explorer,
    );
    render_panel(
        columns[1],
        frame,
        &active_title(&panels.editor_title, model, FocusRegion::Editor),
        &panels.editor,
    );
}

fn render_palette_overlay(frame: &mut Frame, model: &ShellModel, panels: &ShellPanels) {
    let overlay = centered_rect(frame.width(), frame.height());
    render_panel(
        overlay,
        frame,
        &active_title("Palette", model, FocusRegion::Palette),
        &panels.palette,
    );
}

fn render_panel(area: Rect, frame: &mut Frame, title: &str, body: &str) {
    Paragraph::new(body)
        .block(
            Block::new()
                .borders(Borders::ALL)
                .title(title)
                .title_alignment(Alignment::Center),
        )
        .render(area, frame);
}

fn centered_rect(width: u16, height: u16) -> Rect {
    let overlay_width = width.saturating_mul(60).max(60) / 100;
    let overlay_height = height.saturating_mul(55).max(12) / 100;
    let x = width.saturating_sub(overlay_width) / 2;
    let y = height.saturating_sub(overlay_height) / 2;
    Rect::new(x, y, overlay_width.max(48), overlay_height.max(12))
}

fn active_title(base: &str, model: &ShellModel, region: FocusRegion) -> String {
    if model.focus() == region {
        format!("> {base} <")
    } else {
        base.to_string()
    }
}
