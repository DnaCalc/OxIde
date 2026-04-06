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
    frame.set_cursor(None);
    frame.set_cursor_visible(false);
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

    if model.overlay_active() {
        render_overlay(frame, model, &panels);
    }

    if let Some(cursor_position) = editor_cursor_position(model, area) {
        frame.set_cursor(Some(cursor_position));
        frame.set_cursor_visible(true);
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

fn render_overlay(frame: &mut Frame, model: &ShellModel, panels: &ShellPanels) {
    let overlay = centered_rect(frame.width(), frame.height(), model.width_class());
    let body = if model.com_reference_helper_active() {
        &panels.com_reference_helper
    } else {
        &panels.palette
    };
    render_panel(
        overlay,
        frame,
        &active_title(model.overlay_title(), model, FocusRegion::Palette),
        body,
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

fn editor_cursor_position(model: &ShellModel, area: Rect) -> Option<(u16, u16)> {
    if model.overlay_active()
        || model.scene() == ShellScene::Empty
        || model.focus() != FocusRegion::Editor
    {
        return None;
    }

    let editor_area = editor_panel_area(model, area)?;
    let inner = panel_inner_area(editor_area);
    if inner.is_empty() {
        return None;
    }

    let cursor = model.active_editor_cursor()?;
    let scroll_top = model.active_editor_scroll_top().unwrap_or(0);
    let line = cursor.line.saturating_sub(1);
    let visible_row = line.checked_sub(scroll_top)?;
    if visible_row >= inner.height {
        return None;
    }

    let visible_col = cursor.column.saturating_sub(1);
    if visible_col >= inner.width {
        return None;
    }

    Some((
        inner.x.saturating_add(visible_col),
        inner.y.saturating_add(visible_row),
    ))
}

fn editor_panel_area(model: &ShellModel, area: Rect) -> Option<Rect> {
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
    let body = root_sections.get(1).copied()?;

    if model.inspector_is_collapsed() {
        let columns = Flex::horizontal()
            .constraints([Constraint::Percentage(20.0), Constraint::Fill])
            .split(body);
        columns.get(1).copied()
    } else {
        let columns = Flex::horizontal()
            .constraints([
                Constraint::Percentage(model.explorer_width_percent()),
                Constraint::Percentage(model.editor_width_percent()),
                Constraint::Fill,
            ])
            .split(body);
        columns.get(1).copied()
    }
}

fn panel_inner_area(area: Rect) -> Rect {
    Rect::new(
        area.x.saturating_add(1),
        area.y.saturating_add(1),
        area.width.saturating_sub(2),
        area.height.saturating_sub(2),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use ftui::{GraphemePool, prelude::Model};

    #[test]
    fn editor_focus_places_terminal_cursor_in_editor_panel() {
        let mut model = ShellModel::new(Some(std::path::PathBuf::from(
            "examples/thin-slice/ThinSliceHello.basproj",
        )));
        model.update(super::super::model::Msg::FocusRegion(FocusRegion::Editor));

        let mut pool = GraphemePool::new();
        let mut frame = Frame::new(120, 40, &mut pool);
        render(&model, &mut frame);

        assert!(frame.cursor_position.is_some());
        assert!(frame.cursor_visible);
    }
}
