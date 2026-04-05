use ftui::layout::{Constraint, Flex, Rect};
use ftui::prelude::Frame;
use ftui::widgets::Widget;
use ftui::widgets::block::{Alignment, Block};
use ftui::widgets::borders::Borders;
use ftui::widgets::paragraph::Paragraph;

use super::mock_data::ShellPanels;
use super::model::ShellModel;
use super::state::{FocusRegion, ShellScene, WidthClass};
use super::theme::{self, PanelTone};

pub fn render(model: &ShellModel, frame: &mut Frame) {
    let area = Rect::new(0, 0, frame.width(), frame.height());
    let root_sections = match model.lower_surface_height() {
        Some(lower_height) => Flex::vertical()
            .constraints([
                Constraint::Fixed(3),
                Constraint::Fill,
                Constraint::Fixed(lower_height),
            ])
            .split(area),
        None => Flex::vertical()
            .constraints([Constraint::Fixed(3), Constraint::Fill])
            .split(area),
    };

    let panels = model.panels();

    render_panel(
        root_sections[0],
        frame,
        &active_title("OxIde Shell", model, FocusRegion::TopBar),
        &panels.top_bar,
        PanelTone::TopBar,
        model.focus() == FocusRegion::TopBar,
    );

    if model.inspector_is_collapsed() {
        render_narrow_body(root_sections[1], frame, model, &panels);
    } else if model.scene() == ShellScene::Empty {
        render_empty_body(root_sections[1], frame, model, &panels);
    } else {
        render_wide_body(root_sections[1], frame, model, &panels);
    }

    if model.shows_lower_surface() {
        render_panel(
            root_sections[2],
            frame,
            &active_title(
                &model.lower_surface_title(),
                model,
                FocusRegion::LowerSurface,
            ),
            &panels.lower_surface,
            PanelTone::Utility,
            model.focus() == FocusRegion::LowerSurface,
        );
    }

    if model.palette_active() {
        render_palette_overlay(frame, model, &panels);
    }
}

fn render_wide_body(area: Rect, frame: &mut Frame, model: &ShellModel, panels: &ShellPanels) {
    let columns = Flex::horizontal()
        .constraints([
            Constraint::Percentage(model.explorer_width_percent()),
            Constraint::Percentage(model.editor_width_percent()),
            Constraint::Fill,
        ])
        .split(area);

    render_panel(
        columns[0],
        frame,
        &active_title(model.explorer_title(), model, FocusRegion::Explorer),
        &panels.explorer,
        PanelTone::Navigation,
        model.focus() == FocusRegion::Explorer,
    );
    render_panel(
        columns[1],
        frame,
        &active_title(&panels.editor_title, model, FocusRegion::Editor),
        &panels.editor,
        PanelTone::Editor,
        model.focus() == FocusRegion::Editor,
    );
    render_panel(
        columns[2],
        frame,
        &active_title(&model.inspector_title(), model, FocusRegion::Inspector),
        &panels.inspector,
        PanelTone::Context,
        model.focus() == FocusRegion::Inspector,
    );
}

fn render_empty_body(area: Rect, frame: &mut Frame, model: &ShellModel, panels: &ShellPanels) {
    let columns = Flex::horizontal()
        .constraints([
            Constraint::Percentage(model.explorer_width_percent()),
            Constraint::Percentage(model.editor_width_percent()),
            Constraint::Fill,
        ])
        .split(area);

    render_panel(
        columns[0],
        frame,
        &active_title(model.explorer_title(), model, FocusRegion::Explorer),
        &panels.explorer,
        PanelTone::Navigation,
        model.focus() == FocusRegion::Explorer,
    );
    render_panel(
        columns[1],
        frame,
        &active_title(&panels.editor_title, model, FocusRegion::Editor),
        &panels.editor,
        PanelTone::Editor,
        model.focus() == FocusRegion::Editor,
    );
    render_panel(
        columns[2],
        frame,
        &active_title(&model.inspector_title(), model, FocusRegion::Inspector),
        &panels.inspector,
        PanelTone::Context,
        model.focus() == FocusRegion::Inspector,
    );
}

fn render_narrow_body(area: Rect, frame: &mut Frame, model: &ShellModel, panels: &ShellPanels) {
    let columns = Flex::horizontal()
        .constraints([Constraint::Percentage(20.0), Constraint::Fill])
        .split(area);

    render_panel(
        columns[0],
        frame,
        &active_title(model.explorer_title(), model, FocusRegion::Explorer),
        &panels.explorer,
        PanelTone::Navigation,
        model.focus() == FocusRegion::Explorer,
    );
    render_panel(
        columns[1],
        frame,
        &active_title(&panels.editor_title, model, FocusRegion::Editor),
        &panels.editor,
        PanelTone::Editor,
        model.focus() == FocusRegion::Editor,
    );
}

fn render_palette_overlay(frame: &mut Frame, model: &ShellModel, panels: &ShellPanels) {
    let overlay = centered_rect(frame.width(), frame.height(), model.width_class());
    render_panel(
        overlay,
        frame,
        &active_title("Palette", model, FocusRegion::Palette),
        &panels.palette,
        PanelTone::Overlay,
        true,
    );
}

fn render_panel(
    area: Rect,
    frame: &mut Frame,
    title: &str,
    body: &str,
    tone: PanelTone,
    active: bool,
) {
    Paragraph::new(body)
        .style(theme::content_style(tone, active))
        .block(
            Block::new()
                .borders(Borders::ALL)
                .border_style(theme::border_style(tone, active))
                .style(theme::panel_style(tone, active))
                .title(title)
                .title_alignment(Alignment::Center),
        )
        .render(area, frame);
}

fn centered_rect(width: u16, height: u16, width_class: WidthClass) -> Rect {
    let (width_factor, height_factor, min_width, min_height) = match width_class {
        WidthClass::Wide => (56_u16, 56_u16, 60_u16, 14_u16),
        WidthClass::Standard => (62_u16, 60_u16, 54_u16, 14_u16),
        WidthClass::Narrow => (82_u16, 68_u16, 42_u16, 12_u16),
    };
    let overlay_width = width.saturating_mul(width_factor).max(min_width) / 100;
    let overlay_height = height.saturating_mul(height_factor).max(min_height) / 100;
    let x = width.saturating_sub(overlay_width) / 2;
    let y = height.saturating_sub(overlay_height) / 2;
    Rect::new(
        x,
        y,
        overlay_width.max(min_width),
        overlay_height.max(min_height),
    )
}

fn active_title(base: &str, model: &ShellModel, region: FocusRegion) -> String {
    if model.focus() == region {
        format!("> {base} <")
    } else {
        base.to_string()
    }
}
