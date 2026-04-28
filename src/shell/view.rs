use ftui::Cell;
use ftui::layout::{Constraint, Flex, Rect};
use ftui::prelude::Frame;
use ftui::text::WrapMode;
use ftui::widgets::Widget;
use ftui::widgets::block::{Alignment, Block};
use ftui::widgets::borders::Borders;
use ftui::widgets::paragraph::Paragraph;

use super::highlight;
use super::mock_data::ShellPanels;
use super::model::ShellModel;
use super::state::{FocusRegion, ShellScene, WidthClass};
use super::theme::{self, PanelTone};

/// Height of the always-present bottom status line (uxpass D3). One
/// terminal row — the status line never wraps and never grows.
const STATUS_LINE_HEIGHT: u16 = 1;

/// Top-level render pass.
///
/// Frame layout (uxpass D3 — the status line is permanent):
///   `[TopBar(Fixed 3), Body(Fill), LowerSurface(optional), StatusLine(Fixed 1)]`
///
/// The status line is the canonical "what keystrokes are available
/// right now" surface — it is never hidden and overlay scenes (Palette /
/// COM reference helper) reuse it rather than masking it.
pub fn render(model: &ShellModel, frame: &mut Frame) {
    let area = Rect::new(0, 0, frame.width(), frame.height());
    frame.set_cursor(None);
    frame.set_cursor_visible(false);
    let root_sections = split_root(area, model);

    let panels = model.panels();

    render_panel(
        root_sections[0],
        frame,
        &active_title("OxIde Shell", model, FocusRegion::TopBar),
        &panels.top_bar,
        PanelTone::TopBar,
        model.focus() == FocusRegion::TopBar,
        // TopBar is a single-row surface; wrapping would duplicate the
        // row and offset the body layout.
        None,
    );

    // Overlay scenes (Palette / ComReference) paint over the
    // backing scene's body shape — so Empty → F6 still renders the
    // single-panel Welcome underneath, not a 3-column fake project.
    // For non-overlay scenes the backing scene is the scene itself.
    let body_scene = if model.overlay_active() {
        model.previous_scene()
    } else {
        model.scene()
    };

    if model.inspector_is_collapsed() {
        render_narrow_body(root_sections[1], frame, model, &panels);
    } else if body_scene == ShellScene::Empty {
        render_empty_body(root_sections[1], frame, model, &panels);
    } else {
        render_wide_body(root_sections[1], frame, model, &panels);
    }

    let mut trailing_index = 2;
    if model.shows_lower_surface() {
        render_panel(
            root_sections[trailing_index],
            frame,
            &active_title(
                &model.lower_surface_title(),
                model,
                FocusRegion::LowerSurface,
            ),
            &panels.lower_surface,
            PanelTone::Utility,
            model.focus() == FocusRegion::LowerSurface,
            // Lower-surface diagnostics, output, references, etc. can
            // carry long lines (paths, stack traces, error messages);
            // wrap on word boundaries and fall back to char-splitting
            // for identifiers that exceed the column (D7).
            Some(WrapMode::WordChar),
        );
        trailing_index += 1;
    }

    render_status_line(root_sections[trailing_index], frame, model);

    if model.overlay_active() {
        render_overlay(frame, model, &panels);
    }

    // Hover popover renders on top of everything except the scene
    // overlay, since it can co-exist with Editing / Semantic /
    // BuildRun. When a scene overlay is active the popover should be
    // suppressed — F1 is gated by `editor_accepts_input()` in the
    // model layer so a popover cannot be opened during Palette /
    // ComReference, but a lingering popover from before the overlay
    // opened would persist without this guard.
    if !model.overlay_active() {
        render_hover_popover(frame, model, area);
    }

    if let Some(cursor_position) = editor_cursor_position(model, area) {
        frame.set_cursor(Some(cursor_position));
        frame.set_cursor_visible(true);
    }
}

/// Split the frame into its canonical horizontal sections.
///
/// Returns three sections when no lower surface is shown:
/// `[TopBar, Body, StatusLine]`. Four sections when the lower surface
/// is present: `[TopBar, Body, LowerSurface, StatusLine]`. The caller
/// indexes by role, not by raw position.
fn split_root(area: Rect, model: &ShellModel) -> Vec<Rect> {
    match model.lower_surface_height() {
        Some(lower_height) => Flex::vertical()
            .constraints([
                Constraint::Fixed(3),
                Constraint::Fill,
                Constraint::Fixed(lower_height),
                Constraint::Fixed(STATUS_LINE_HEIGHT),
            ])
            .split(area),
        None => Flex::vertical()
            .constraints([
                Constraint::Fixed(3),
                Constraint::Fill,
                Constraint::Fixed(STATUS_LINE_HEIGHT),
            ])
            .split(area),
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
        // Project paths, module includes, GUIDs can overflow the
        // Explorer column; wrap rather than truncate (D7).
        Some(WrapMode::WordChar),
    );
    render_editor_panel(columns[1], frame, model, panels);
    render_panel(
        columns[2],
        frame,
        &active_title(&model.inspector_title(), model, FocusRegion::Inspector),
        &panels.inspector,
        PanelTone::Context,
        model.focus() == FocusRegion::Inspector,
        // Diagnostics, symbol names, hover text all need to stay
        // readable in the narrowest Inspector column (D7).
        Some(WrapMode::WordChar),
    );
}

/// Render the Empty scene body.
///
/// Uxpass D1 (D1a + D1b): Empty has nothing to inspect (P5) and
/// nothing to navigate in a separate tree (D1b — the Welcome panel
/// owns the launcher role directly). The body collapses to a single
/// full-width Welcome surface carrying:
/// - the OxIde header + blurb,
/// - the Recent projects list with `>` selection marker (driven by
///   `launcher_selection`; `Up/Down` cycles it, `Ctrl+O` opens the
///   selected one — both are announced on the status line),
/// - the Start action list (informational; no second selection marker).
///
/// The status line below this region announces `Ctrl+O`, `Up/Down`,
/// `F6`, and `Ctrl+Q` (D3 / D8).
fn render_empty_body(area: Rect, frame: &mut Frame, model: &ShellModel, panels: &ShellPanels) {
    render_panel(
        area,
        frame,
        &active_title("Welcome", model, FocusRegion::Editor),
        &panels.editor,
        PanelTone::Editor,
        model.focus() == FocusRegion::Editor,
        // Welcome is user-facing prose: recent-project paths can be
        // long; the Start action list can grow. Wrap on word
        // boundaries with char-break fallback (D7).
        Some(WrapMode::WordChar),
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
        Some(WrapMode::WordChar),
    );
    render_editor_panel(columns[1], frame, model, panels);
}

/// Render the Editor panel for any non-Empty scene.
///
/// Unlike `render_panel`, this function produces a **styled** body via
/// `highlight::build_editor_text`: each source line carries a muted
/// line-number gutter and VBA tokens are colourised (keywords, type
/// names, strings, numbers, comments). Rendering bypasses wrap so
/// source-column positions survive one-to-one onto terminal columns —
/// the cursor-positioning path in `editor_cursor_position` depends on
/// that invariant.
///
/// The Empty scene keeps using `render_panel` with a plain string for
/// the Welcome surface; Welcome is prose, not code, and must stay
/// un-lexed.
fn render_editor_panel(area: Rect, frame: &mut Frame, model: &ShellModel, panels: &ShellPanels) {
    let active = model.focus() == FocusRegion::Editor;
    let title = active_title(&panels.editor_title, model, FocusRegion::Editor);
    let body = model
        .active_editor_lines()
        .map(|lines| highlight::build_editor_text(&lines, lines.len()))
        .unwrap_or_else(|| ftui::text::Text::raw("No buffer mounted."));

    Paragraph::new(body)
        .style(theme::content_style(PanelTone::Editor, active))
        .block(
            Block::new()
                .borders(Borders::ALL)
                .border_style(theme::border_style(PanelTone::Editor, active))
                .style(theme::panel_style(PanelTone::Editor, active))
                .title(title.as_str())
                .title_alignment(Alignment::Center),
        )
        .render(area, frame);
}

fn render_overlay(frame: &mut Frame, model: &ShellModel, panels: &ShellPanels) {
    let overlay = centered_rect(frame.width(), frame.height(), model.width_class());
    // J4-a: clear the overlay rect's cell contents before the panel
    // renders so editor text underneath cannot bleed through the
    // Block's `set_style_area` (which preserves cell content while
    // changing background color). Cell::default() resets the grapheme
    // to EMPTY; the Block below then paints its own background and
    // title/body on top of clean cells.
    frame.buffer.fill(overlay, Cell::default());
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
        // Palette filter / COM reference path fields wrap long
        // candidate labels so none of them silently truncate (D7).
        Some(WrapMode::WordChar),
    );
}

fn render_panel(
    area: Rect,
    frame: &mut Frame,
    title: &str,
    body: &str,
    tone: PanelTone,
    active: bool,
    wrap: Option<WrapMode>,
) {
    let mut paragraph = Paragraph::new(body)
        .style(theme::content_style(tone, active))
        .block(
            Block::new()
                .borders(Borders::ALL)
                .border_style(theme::border_style(tone, active))
                .style(theme::panel_style(tone, active))
                .title(title)
                .title_alignment(Alignment::Center),
        );
    if let Some(mode) = wrap {
        paragraph = paragraph.wrap(mode);
    }
    paragraph.render(area, frame);
}

/// Render the hover popover (F1) if one is active.
///
/// The popover is a small bordered block of lines positioned below
/// the editor cursor when possible, clamped inside the editor panel
/// so the popover does not spill into the Inspector / Explorer /
/// Lower Surface. Width is sized to the longest line plus a 2-cell
/// border budget; height to the number of lines plus 2.
///
/// Falls back to "no-op" when the editor panel is not being rendered
/// (Empty scene) or when the popover's anchor is off-screen — the
/// anchor becomes off-screen after a cursor move, which the model
/// layer already dismisses the popover for, so in practice this is
/// a belt-and-braces guard.
fn render_hover_popover(frame: &mut Frame, model: &ShellModel, frame_area: Rect) {
    let Some(popover) = model.hover_popover() else {
        return;
    };
    if popover.lines.is_empty() {
        return;
    }
    let Some(editor_area) = editor_panel_area(model, frame_area) else {
        return;
    };
    let editor_inner = panel_inner_area(editor_area);
    if editor_inner.is_empty() {
        return;
    }

    // Size the popover to its content with sensible caps.
    let content_width = popover
        .lines
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0) as u16;
    let border_budget = 2_u16;
    let max_width = editor_inner.width.saturating_sub(2).max(10);
    let popover_width = (content_width + border_budget).min(max_width).max(12);
    let popover_height = (popover.lines.len() as u16 + border_budget).min(editor_inner.height);

    // Anchor below the cursor's visible row; if that would overflow
    // the editor panel, anchor above instead. If neither fits, clamp
    // to the top of the editor panel.
    let scroll_top = model.active_editor_scroll_top().unwrap_or(0);
    let visible_row = popover
        .anchor
        .line
        .saturating_sub(1)
        .checked_sub(scroll_top)
        .and_then(|row| {
            if row < editor_inner.height {
                Some(row)
            } else {
                None
            }
        });

    let total_lines = model.active_editor_lines().map(|l| l.len()).unwrap_or(0);
    let gutter = highlight::gutter_total_width(total_lines) as u16;
    let visible_col = popover
        .anchor
        .column
        .saturating_sub(1)
        .saturating_add(gutter);

    let (anchor_x, anchor_y) = match visible_row {
        Some(row) => (
            editor_inner.x.saturating_add(visible_col),
            editor_inner.y.saturating_add(row).saturating_add(1),
        ),
        None => (editor_inner.x, editor_inner.y),
    };

    // Prefer a position below the cursor. If the popover would
    // overflow the bottom of the editor panel, flip to above.
    let mut popover_y = anchor_y;
    let bottom = editor_inner.y.saturating_add(editor_inner.height);
    if popover_y.saturating_add(popover_height) > bottom {
        popover_y = anchor_y
            .saturating_sub(1)
            .saturating_sub(popover_height)
            .max(editor_inner.y);
    }

    // Clamp horizontally so the popover fits entirely inside the
    // editor panel.
    let right_limit = editor_inner.x.saturating_add(editor_inner.width);
    let mut popover_x = anchor_x;
    if popover_x.saturating_add(popover_width) > right_limit {
        popover_x = right_limit
            .saturating_sub(popover_width)
            .max(editor_inner.x);
    }

    let rect = Rect::new(popover_x, popover_y, popover_width, popover_height);

    // Clear the rect before the popover Block paints — same logic as
    // J4-a for the palette overlay so editor glyphs don't bleed
    // through the popover's background.
    frame.buffer.fill(rect, Cell::default());

    let body = popover.lines.join("\n");
    Paragraph::new(body)
        .style(theme::content_style(PanelTone::Overlay, true))
        .block(
            Block::new()
                .borders(Borders::ALL)
                .border_style(theme::border_style(PanelTone::Overlay, true))
                .style(theme::panel_style(PanelTone::Overlay, true))
                .title("Hover"),
        )
        .wrap(WrapMode::WordChar)
        .render(rect, frame);
}

/// Render the always-present bottom status line (uxpass D3 / D8).
///
/// Single terminal row, no border, muted foreground on the panel
/// background. The text comes from `ShellModel::status_line_hint`,
/// which returns a per-scene hint string (see `ShellState::status_line_hint`).
fn render_status_line(area: Rect, frame: &mut Frame, model: &ShellModel) {
    let style = theme::content_style(PanelTone::TopBar, false);
    Paragraph::new(model.status_line_hint())
        .style(style)
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

    // Account for the gutter that `render_editor_panel` paints on
    // every line: `" <n> │ "`. Without this shift the terminal
    // cursor would render inside the gutter rather than on the
    // character the user is editing.
    let total_lines = model.active_editor_lines().map(|l| l.len()).unwrap_or(0);
    let gutter = highlight::gutter_total_width(total_lines) as u16;
    let visible_col = cursor.column.saturating_sub(1).saturating_add(gutter);
    if visible_col >= inner.width {
        return None;
    }

    Some((
        inner.x.saturating_add(visible_col),
        inner.y.saturating_add(visible_row),
    ))
}

fn editor_panel_area(model: &ShellModel, area: Rect) -> Option<Rect> {
    // Must mirror `split_root` + the body-section splits in `render_*`;
    // the status line at the trailing index is irrelevant because we
    // only need the body rect.
    let root_sections = split_root(area, model);
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

    /// Collect the printable characters from one row of a Frame's
    /// buffer within a horizontal `[x_start, x_end)` span. Used by the
    /// opaque-overlay regression test to assert that specific editor
    /// glyphs are absent from the overlay's interior rows.
    fn collect_row_chars(frame: &Frame, y: u16, x_start: u16, x_end: u16) -> String {
        let mut out = String::new();
        for x in x_start..x_end {
            if let Some(cell) = frame.buffer.get(x, y) {
                if let Some(ch) = cell.content.as_char() {
                    out.push(ch);
                } else if cell.content.is_grapheme() {
                    // Interned grapheme — treat as "non-empty, non-ASCII"
                    // for substring scans. Editor source code is ASCII,
                    // so the bleed-through characters we want to catch
                    // would appear as char-encoded cells, not graphemes.
                    out.push('?');
                }
            }
        }
        out
    }

    /// J4-a / P1 / P4 — the palette overlay must paint opaque cells
    /// over the editor surface. Before the fix, Block's
    /// `set_style_area` recolored cells in place, leaving editor
    /// glyphs (e.g. the `Integer` in `Dim answer As Integer`) visible
    /// inside the overlay frame. Regression: render Editing with the
    /// palette open and assert that the editor's characteristic
    /// source-code tokens are not found inside the overlay's inner
    /// rows.
    #[test]
    fn palette_overlay_is_opaque_over_editor_text() {
        let mut model = ShellModel::new(Some(std::path::PathBuf::from(
            "examples/thin-slice/ThinSliceHello.basproj",
        )));
        model.update(super::super::model::Msg::FocusRegion(FocusRegion::Editor));
        model.update(super::super::model::Msg::TogglePalette);
        assert!(model.overlay_active(), "palette must be open for this test");

        let mut pool = GraphemePool::new();
        let (width, height) = (120_u16, 40_u16);
        let mut frame = Frame::new(width, height, &mut pool);
        render(&model, &mut frame);

        let overlay = centered_rect(width, height, model.width_class());
        let inner = panel_inner_area(overlay);
        assert!(
            !inner.is_empty(),
            "overlay inner area must be non-empty for this test to be meaningful"
        );

        // `Integer` from `Dim answer As Integer` is the most obvious
        // bleed-through token at the 120x40 Editing layout: it lives
        // near the centre of the editor column where the overlay rect
        // falls. If J4-a regresses, that substring reappears in one
        // of the overlay's inner rows.
        for y in inner.y..inner.y.saturating_add(inner.height) {
            let row = collect_row_chars(&frame, y, inner.x, inner.x.saturating_add(inner.width));
            assert!(
                !row.contains("Integer"),
                "editor text bled through palette overlay at row {y}: {row:?}"
            );
            assert!(
                !row.contains("answer = 40"),
                "editor expression bled through palette overlay at row {y}: {row:?}"
            );
        }
    }
}
