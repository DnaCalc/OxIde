use std::path::PathBuf;

use ftui::{
    KeyEventKind,
    prelude::{Cmd, Event, Frame, KeyCode, KeyEvent, Model, Modifiers},
};
use oxvba_project::ComSelectionCandidate;

use super::mock_data::{ShellPanels, shell_panels};
use super::oxvba::run_project_state;
use super::project_actions::{
    ComReferenceDiscovery, add_com_reference_candidate, add_scaffolded_module, cycle_output_type,
    discover_com_reference_candidates, next_module_name,
};
use super::session::ProjectSession;
use super::state::{
    ComReferenceHelperState, ComReferenceSearchMode, CursorPosition, FocusRegion, LowerSurfaceMode,
    ShellScene, ShellState, WidthClass, WorkspaceProjectModuleKind, WorkspaceProjectState,
};
use super::view;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Msg {
    Quit,
    NextFocus,
    MoveEditorLeft,
    MoveEditorRight,
    MoveEditorUp,
    MoveEditorDown,
    InsertEditorChar(char),
    InsertEditorNewline,
    BackspaceEditorChar,
    /// Save the active buffer to its source file. Bound to `Ctrl+S`.
    /// No-op on buffers without a `source_path` (Welcome).
    SaveActiveBuffer,
    /// Save every dirty buffer in the active workspace. Bound to
    /// `Ctrl+Shift+S`.
    SaveAllDirtyBuffers,
    /// Undo the most recent edit primitive on the active buffer.
    /// Bound to `Ctrl+Z`.
    UndoActiveBuffer,
    /// Re-apply the most recently undone edit. Bound to `Ctrl+Y`.
    RedoActiveBuffer,
    OpenSelectedProject,
    RunProject,
    AddProjectModule,
    AddProjectClass,
    OpenComReferenceHelper,
    CloseOverlay,
    CycleProjectTarget,
    FocusRegion(FocusRegion),
    NextEditorView,
    TogglePalette,
    SetScene(ShellScene),
    Resized(u16, u16),
    Noop,
}

impl From<Event> for Msg {
    fn from(event: Event) -> Self {
        match event {
            Event::Key(key) if !is_actionable_key(key) => Msg::Noop,
            Event::Key(key) if is_quit_key(key) => Msg::Quit,
            Event::Key(key) if matches!(key.code, KeyCode::Escape) => Msg::CloseOverlay,
            Event::Key(key) if is_open_project_key(key) => Msg::OpenSelectedProject,
            Event::Key(key) if matches!(key.code, KeyCode::F(5)) => Msg::RunProject,
            Event::Key(key) if is_toggle_palette_key(key) => Msg::TogglePalette,
            Event::Key(key) if is_save_all_key(key) => Msg::SaveAllDirtyBuffers,
            Event::Key(key) if is_save_key(key) => Msg::SaveActiveBuffer,
            Event::Key(key) if is_undo_key(key) => Msg::UndoActiveBuffer,
            Event::Key(key) if is_redo_key(key) => Msg::RedoActiveBuffer,
            Event::Key(key) if is_add_project_module_key(key) => Msg::AddProjectModule,
            Event::Key(key) if is_add_project_class_key(key) => Msg::AddProjectClass,
            Event::Key(key) if is_open_com_reference_helper_key(key) => Msg::OpenComReferenceHelper,
            Event::Key(key) if is_cycle_project_target_key(key) => Msg::CycleProjectTarget,
            Event::Key(key) if matches!(key.code, KeyCode::Left) => Msg::MoveEditorLeft,
            Event::Key(key) if matches!(key.code, KeyCode::Right) => Msg::MoveEditorRight,
            Event::Key(key) if matches!(key.code, KeyCode::Up) => Msg::MoveEditorUp,
            Event::Key(key) if matches!(key.code, KeyCode::Down) => Msg::MoveEditorDown,
            Event::Key(key) if matches!(key.code, KeyCode::Enter) => Msg::InsertEditorNewline,
            Event::Key(key) if matches!(key.code, KeyCode::Backspace) => Msg::BackspaceEditorChar,
            Event::Key(key) if is_focus_region_key(key, '1') => {
                Msg::FocusRegion(FocusRegion::Explorer)
            }
            Event::Key(key) if is_focus_region_key(key, '2') => {
                Msg::FocusRegion(FocusRegion::Editor)
            }
            Event::Key(key) if is_focus_region_key(key, '3') => {
                Msg::FocusRegion(FocusRegion::Inspector)
            }
            Event::Key(key) if is_focus_region_key(key, '4') => {
                Msg::FocusRegion(FocusRegion::LowerSurface)
            }
            Event::Key(key) if is_cycle_editor_view_key(key) => Msg::NextEditorView,
            Event::Key(key) if matches!(key.code, KeyCode::Tab) => Msg::NextFocus,
            Event::Key(key) if matches!(key.code, KeyCode::F(2)) => {
                Msg::SetScene(ShellScene::Empty)
            }
            Event::Key(key) if matches!(key.code, KeyCode::F(3)) => {
                Msg::SetScene(ShellScene::Editing)
            }
            Event::Key(key) if matches!(key.code, KeyCode::F(4)) => {
                Msg::SetScene(ShellScene::Semantic)
            }
            Event::Key(key) if editor_input_char(key).is_some() => {
                Msg::InsertEditorChar(editor_input_char(key).expect("checked above"))
            }
            Event::Resize { width, height } => Msg::Resized(width, height),
            _ => Msg::Noop,
        }
    }
}

pub struct ShellModel {
    shell: ShellState,
    project_path: Option<PathBuf>,
    com_reference_candidates: Vec<ComSelectionCandidate>,
    dev_scenes: bool,
}

impl ShellModel {
    /// Construct the default user-facing shell. Equivalent to
    /// `with_dev_scenes(project_path, false)`.
    pub fn new(project_path: Option<PathBuf>) -> Self {
        Self::with_dev_scenes(project_path, false)
    }

    /// Construct the shell, optionally enabling the dev-only `F2/F3/F4`
    /// scene-flips and the palette's `Mockup States` group (uxpass D6).
    pub fn with_dev_scenes(project_path: Option<PathBuf>, dev_scenes: bool) -> Self {
        let mut shell = ShellState::default();
        shell.set_dev_scenes(dev_scenes);
        let mut model = Self {
            shell,
            project_path: None,
            com_reference_candidates: Vec::new(),
            dev_scenes,
        };
        model.shell.apply_scene(ShellScene::Empty);
        model.discover_recent_projects();
        if let Some(project_path) = project_path {
            model.try_mount_workspace(project_path);
        }
        model
    }

    pub fn panels(&self) -> ShellPanels {
        shell_panels(&self.shell)
    }

    pub fn palette_active(&self) -> bool {
        self.shell.palette_active()
    }

    pub fn com_reference_helper_active(&self) -> bool {
        self.shell.com_reference_helper_active()
    }

    pub fn scene(&self) -> ShellScene {
        self.shell.scene
    }

    pub fn inspector_is_collapsed(&self) -> bool {
        self.shell.inspector_is_collapsed()
    }

    pub fn shows_lower_surface(&self) -> bool {
        self.shell.shows_lower_surface()
    }

    pub fn lower_surface_height(&self) -> Option<u16> {
        self.shell.lower_surface_height()
    }

    pub fn explorer_width_percent(&self) -> f32 {
        self.shell.explorer_width_percent()
    }

    pub fn editor_width_percent(&self) -> f32 {
        self.shell.editor_width_percent()
    }

    pub fn focus(&self) -> FocusRegion {
        self.shell.runtime.focus
    }

    pub fn width_class(&self) -> WidthClass {
        self.shell.runtime.width_class
    }

    pub fn active_editor_cursor(&self) -> Option<CursorPosition> {
        self.shell
            .runtime
            .workspace
            .active_view()
            .map(|view| view.surface.cursor)
    }

    pub fn active_editor_scroll_top(&self) -> Option<u16> {
        self.shell
            .runtime
            .workspace
            .active_view()
            .map(|view| view.surface.scroll_top)
    }

    /// Explorer column title. Uxpass D1b removed the Launcher column
    /// from Empty, so this is only ever called on scenes that render
    /// a project tree on the left. The Empty arm used to return
    /// `"Launcher"`; it is gone.
    pub fn explorer_title(&self) -> &'static str {
        "Explorer"
    }

    pub fn inspector_title(&self) -> String {
        // Uxpass D1 (D1a): Empty has no Inspector column, so the
        // `Environment` title is unreachable on that scene. The
        // remaining scenes show `Inspector <Mode>`; `<Mode>` is a
        // user-visible cue for what the Inspector is currently
        // presenting (Diagnostics, Symbols, Hover, …).
        format!("Inspector {}", self.shell.runtime.inspector_mode.label())
    }

    /// Status-line text for the always-present bottom row (uxpass D3 / D8).
    pub fn status_line_hint(&self) -> &'static str {
        self.shell.status_line_hint()
    }

    pub fn lower_surface_title(&self) -> String {
        let mode = self.shell.runtime.lower_mode;
        match mode {
            LowerSurfaceMode::Launcher => String::from("Lower Surface Launcher"),
            _ => format!("Lower Surface {}", mode.label()),
        }
    }

    pub fn overlay_active(&self) -> bool {
        self.shell.overlay_active()
    }

    pub fn overlay_title(&self) -> &'static str {
        match self.shell.scene {
            ShellScene::Palette => "Palette",
            ShellScene::ComReference => "COM Reference Helper",
            _ => "",
        }
    }

    fn discover_recent_projects(&mut self) {
        let projects = ProjectSession::discover_projects(".").unwrap_or_default();
        self.shell.set_recent_projects(projects);
    }

    fn try_mount_workspace(&mut self, project_path: impl Into<PathBuf>) {
        let project_path = project_path.into();
        if let Ok(session) = ProjectSession::load(&project_path) {
            self.shell.set_execution(session.execution_state());
            self.shell.mount_workspace(session.workspace_state());
            self.shell.apply_scene(ShellScene::Editing);
            self.project_path = Some(project_path);
        }
    }

    fn run_project(&mut self) {
        self.shell.apply_scene(ShellScene::BuildRun);
        let Some(project_path) = self.project_path.clone() else {
            return;
        };

        let execution = run_project_state(
            &project_path,
            self.shell.runtime.execution.profile.clone(),
            self.shell.runtime.execution.entry_point.clone(),
        );
        self.shell.set_execution(execution);
    }

    fn open_com_reference_helper(&mut self) {
        self.shell.open_com_reference_helper();
        self.refresh_com_reference_helper();
    }

    fn apply_project_action(
        &mut self,
        action: impl FnOnce(&PathBuf, &WorkspaceProjectState) -> std::io::Result<()>,
    ) {
        if self
            .shell
            .runtime
            .workspace
            .buffers
            .iter()
            .any(|buffer| buffer.dirty)
        {
            return;
        }

        let Some(project_path) = self.project_path.clone() else {
            return;
        };
        let Some(project) = self.shell.runtime.workspace.project.as_ref() else {
            return;
        };

        if action(&project_path, project).is_ok() {
            self.try_mount_workspace(project_path);
        }
    }

    fn refresh_com_reference_helper(&mut self) {
        let helper = self.shell.runtime.com_reference_helper.clone();
        let Some(project_path) = self.project_path.clone() else {
            self.com_reference_candidates.clear();
            self.shell
                .set_com_reference_helper(ComReferenceHelperState {
                    status_lines: vec![String::from("Open a project before adding COM references")],
                    ..helper
                });
            return;
        };

        match discover_com_reference_candidates(
            &project_path,
            helper.mode,
            helper.query.as_str(),
            helper.selection,
        ) {
            Ok(ComReferenceDiscovery { candidates, helper }) => {
                self.com_reference_candidates = candidates;
                self.shell.set_com_reference_helper(helper);
            }
            Err(error) => {
                self.com_reference_candidates.clear();
                self.shell
                    .set_com_reference_helper(ComReferenceHelperState {
                        status_lines: vec![error.to_string()],
                        ..helper
                    });
            }
        }
    }

    fn cycle_com_reference_mode(&mut self) {
        let helper = &mut self.shell.runtime.com_reference_helper;
        helper.mode = match helper.mode {
            ComReferenceSearchMode::Search => ComReferenceSearchMode::File,
            ComReferenceSearchMode::File => ComReferenceSearchMode::Search,
        };
        helper.selection = 0;
        self.refresh_com_reference_helper();
    }

    fn move_com_reference_selection(&mut self, direction: i8) {
        let helper = &mut self.shell.runtime.com_reference_helper;
        if helper.candidates.is_empty() {
            return;
        }

        let len = helper.candidates.len();
        let index = helper.selection;
        helper.selection = if direction >= 0 {
            (index + 1) % len
        } else if index == 0 {
            len - 1
        } else {
            index - 1
        };
        self.refresh_com_reference_helper();
    }

    fn edit_com_reference_query(&mut self, ch: Option<char>, backspace: bool) {
        let helper = &mut self.shell.runtime.com_reference_helper;
        if backspace {
            helper.query.pop();
        } else if let Some(ch) = ch {
            helper.query.push(ch);
        }
        helper.selection = 0;
        self.refresh_com_reference_helper();
    }

    fn apply_selected_com_reference(&mut self) {
        if self
            .shell
            .runtime
            .workspace
            .buffers
            .iter()
            .any(|buffer| buffer.dirty)
        {
            let mut helper = self.shell.runtime.com_reference_helper.clone();
            helper.status_lines.push(String::from(
                "Save or reload dirty buffers before mutating project references",
            ));
            self.shell.set_com_reference_helper(helper);
            return;
        }

        let Some(project_path) = self.project_path.clone() else {
            return;
        };
        let Some(candidate) = self
            .com_reference_candidates
            .get(self.shell.runtime.com_reference_helper.selection)
            .cloned()
        else {
            return;
        };

        match add_com_reference_candidate(&project_path, &candidate) {
            Ok(()) => {
                self.shell.close_overlay();
                self.try_mount_workspace(project_path);
            }
            Err(error) => {
                let mut helper = self.shell.runtime.com_reference_helper.clone();
                helper.status_lines.push(error.to_string());
                self.shell.set_com_reference_helper(helper);
            }
        }
    }

    fn editor_accepts_input(&self) -> bool {
        !matches!(
            self.shell.scene,
            ShellScene::Empty | ShellScene::Palette | ShellScene::ComReference
        ) && self.shell.runtime.focus == FocusRegion::Editor
    }
}

impl Model for ShellModel {
    type Message = Msg;

    fn update(&mut self, msg: Self::Message) -> Cmd<Self::Message> {
        match msg {
            Msg::Quit => Cmd::quit(),
            Msg::NextFocus => {
                if self.com_reference_helper_active() {
                    self.cycle_com_reference_mode();
                } else {
                    self.shell.cycle_focus();
                }
                Cmd::none()
            }
            Msg::MoveEditorLeft => {
                if self.editor_accepts_input() {
                    self.shell.move_editor_cursor_left();
                }
                Cmd::none()
            }
            Msg::MoveEditorRight => {
                if self.editor_accepts_input() {
                    self.shell.move_editor_cursor_right();
                }
                Cmd::none()
            }
            Msg::MoveEditorUp => {
                if self.shell.scene == ShellScene::Empty {
                    self.shell.cycle_launcher_selection(-1);
                } else if self.com_reference_helper_active() {
                    self.move_com_reference_selection(-1);
                } else if self.editor_accepts_input() {
                    self.shell.move_editor_cursor_up();
                }
                Cmd::none()
            }
            Msg::MoveEditorDown => {
                if self.shell.scene == ShellScene::Empty {
                    self.shell.cycle_launcher_selection(1);
                } else if self.com_reference_helper_active() {
                    self.move_com_reference_selection(1);
                } else if self.editor_accepts_input() {
                    self.shell.move_editor_cursor_down();
                }
                Cmd::none()
            }
            Msg::InsertEditorChar(ch) => {
                if self.com_reference_helper_active() {
                    self.edit_com_reference_query(Some(ch), false);
                } else if self.editor_accepts_input() {
                    self.shell.insert_editor_char(ch);
                }
                Cmd::none()
            }
            Msg::InsertEditorNewline => {
                if self.com_reference_helper_active() {
                    self.apply_selected_com_reference();
                } else if self.editor_accepts_input() {
                    self.shell.insert_editor_newline();
                }
                Cmd::none()
            }
            Msg::BackspaceEditorChar => {
                if self.com_reference_helper_active() {
                    self.edit_com_reference_query(None, true);
                } else if self.editor_accepts_input() {
                    self.shell.backspace_editor_char();
                }
                Cmd::none()
            }
            Msg::SaveActiveBuffer => {
                // Overlays block the save to preserve modal invariants;
                // once the overlay closes, the user's `*` still marks
                // the dirty buffer and they can save then.
                if !self.shell.overlay_active() {
                    let _ = self.shell.save_active_buffer();
                }
                Cmd::none()
            }
            Msg::SaveAllDirtyBuffers => {
                if !self.shell.overlay_active() {
                    let _ = self.shell.save_all_dirty_buffers();
                }
                Cmd::none()
            }
            Msg::UndoActiveBuffer => {
                if self.editor_accepts_input() {
                    self.shell.undo_active_buffer();
                }
                Cmd::none()
            }
            Msg::RedoActiveBuffer => {
                if self.editor_accepts_input() {
                    self.shell.redo_active_buffer();
                }
                Cmd::none()
            }
            Msg::OpenSelectedProject => {
                if let Some(path) = self.shell.selected_project_path().cloned() {
                    self.try_mount_workspace(path);
                }
                Cmd::none()
            }
            Msg::RunProject => {
                self.run_project();
                Cmd::none()
            }
            Msg::OpenComReferenceHelper => {
                self.open_com_reference_helper();
                Cmd::none()
            }
            Msg::CloseOverlay => {
                self.shell.close_overlay();
                Cmd::none()
            }
            Msg::AddProjectModule => {
                self.apply_project_action(|project_path, project| {
                    let logical_name =
                        next_module_name(project, WorkspaceProjectModuleKind::Module);
                    add_scaffolded_module(
                        project_path,
                        WorkspaceProjectModuleKind::Module,
                        logical_name.as_str(),
                    )
                });
                Cmd::none()
            }
            Msg::AddProjectClass => {
                self.apply_project_action(|project_path, project| {
                    let logical_name = next_module_name(project, WorkspaceProjectModuleKind::Class);
                    add_scaffolded_module(
                        project_path,
                        WorkspaceProjectModuleKind::Class,
                        logical_name.as_str(),
                    )
                });
                Cmd::none()
            }
            Msg::CycleProjectTarget => {
                self.apply_project_action(|project_path, _| {
                    cycle_output_type(project_path).map(|_| ())
                });
                Cmd::none()
            }
            Msg::FocusRegion(region) => {
                self.shell.focus_region(region);
                Cmd::none()
            }
            Msg::NextEditorView => {
                self.shell.cycle_active_editor_view();
                self.shell.focus_region(FocusRegion::Editor);
                Cmd::none()
            }
            Msg::TogglePalette => {
                self.shell.toggle_palette();
                Cmd::none()
            }
            Msg::SetScene(scene) => {
                // Scene-flip is a dev-only affordance; in the default build
                // (uxpass D6) the F2/F3/F4 keys no longer mutate scene. The
                // Msg is still produced by `From<Event>` so tests can assert
                // the mapping, but we drop it here.
                if self.dev_scenes {
                    self.shell.apply_scene(scene);
                }
                Cmd::none()
            }
            Msg::Resized(width, height) => {
                self.shell.update_size(width, height);
                Cmd::none()
            }
            Msg::Noop => Cmd::none(),
        }
    }

    fn view(&self, frame: &mut Frame) {
        view::render(self, frame);
    }
}

fn is_quit_key(key: KeyEvent) -> bool {
    key.is_char('q') && key.modifiers.contains(Modifiers::CTRL)
}

fn is_open_project_key(key: KeyEvent) -> bool {
    key.is_char('o') && key.modifiers.contains(Modifiers::CTRL)
}

fn is_toggle_palette_key(key: KeyEvent) -> bool {
    matches!(key.code, KeyCode::F(6))
}

fn is_add_project_module_key(key: KeyEvent) -> bool {
    key.is_char('m') && key.modifiers.contains(Modifiers::CTRL | Modifiers::SHIFT)
}

fn is_add_project_class_key(key: KeyEvent) -> bool {
    key.is_char('c') && key.modifiers.contains(Modifiers::CTRL | Modifiers::SHIFT)
}

fn is_open_com_reference_helper_key(key: KeyEvent) -> bool {
    key.is_char('r') && key.modifiers.contains(Modifiers::CTRL | Modifiers::SHIFT)
}

fn is_cycle_project_target_key(key: KeyEvent) -> bool {
    key.is_char('t') && key.modifiers.contains(Modifiers::CTRL | Modifiers::SHIFT)
}

/// `Ctrl+S` — save the active buffer. Explicitly excludes the
/// `Shift` modifier so `Ctrl+Shift+S` routes to `Msg::SaveAllDirtyBuffers`
/// without ambiguity.
fn is_save_key(key: KeyEvent) -> bool {
    key.is_char('s')
        && key.modifiers.contains(Modifiers::CTRL)
        && !key.modifiers.contains(Modifiers::SHIFT)
}

/// `Ctrl+Shift+S` — save every dirty buffer in the workspace.
fn is_save_all_key(key: KeyEvent) -> bool {
    key.is_char('s') && key.modifiers.contains(Modifiers::CTRL | Modifiers::SHIFT)
}

/// `Ctrl+Z` — undo the most recent edit on the active buffer.
fn is_undo_key(key: KeyEvent) -> bool {
    key.is_char('z')
        && key.modifiers.contains(Modifiers::CTRL)
        && !key.modifiers.contains(Modifiers::SHIFT)
}

/// `Ctrl+Y` — redo the most recently undone edit.
///
/// We deliberately pick `Ctrl+Y` (Windows convention) rather than
/// `Ctrl+Shift+Z` (Unix convention) for redo — this matches the VBA
/// IDE, Office, and the broader Windows ecosystem. A future VBA-IDE
/// keymap profile can layer additional bindings; the default keymap
/// stays Windows-native.
fn is_redo_key(key: KeyEvent) -> bool {
    key.is_char('y') && key.modifiers.contains(Modifiers::CTRL)
}

fn is_focus_region_key(key: KeyEvent, digit: char) -> bool {
    matches!(key.code, KeyCode::Char(value) if value == digit)
        && key.modifiers.contains(Modifiers::ALT)
}

fn is_cycle_editor_view_key(key: KeyEvent) -> bool {
    matches!(key.code, KeyCode::Tab) && key.modifiers.contains(Modifiers::CTRL)
}

fn editor_input_char(key: KeyEvent) -> Option<char> {
    if key.modifiers.contains(Modifiers::CTRL) || key.modifiers.contains(Modifiers::ALT) {
        return None;
    }

    match key.code {
        KeyCode::Char(ch) => Some(ch),
        _ => None,
    }
}

fn is_actionable_key(key: KeyEvent) -> bool {
    matches!(key.kind, KeyEventKind::Press)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignores_key_release_events() {
        let msg = Msg::from(Event::Key(
            KeyEvent::new(KeyCode::Tab).with_kind(KeyEventKind::Release),
        ));
        assert_eq!(msg, Msg::Noop);
    }

    #[test]
    fn reacts_to_key_press_events() {
        let msg = Msg::from(Event::Key(
            KeyEvent::new(KeyCode::Tab).with_kind(KeyEventKind::Press),
        ));
        assert_eq!(msg, Msg::NextFocus);
    }

    #[test]
    fn maps_alt_number_to_region_focus() {
        let msg = Msg::from(Event::Key(
            KeyEvent::new(KeyCode::Char('3')).with_modifiers(Modifiers::ALT),
        ));
        assert_eq!(msg, Msg::FocusRegion(FocusRegion::Inspector));
    }

    #[test]
    fn maps_f6_to_palette_toggle() {
        let msg = Msg::from(Event::Key(KeyEvent::new(KeyCode::F(6))));
        assert_eq!(msg, Msg::TogglePalette);
    }

    #[test]
    fn maps_ctrl_o_to_open_selected_project() {
        let msg = Msg::from(Event::Key(
            KeyEvent::new(KeyCode::Char('o')).with_modifiers(Modifiers::CTRL),
        ));
        assert_eq!(msg, Msg::OpenSelectedProject);
    }

    #[test]
    fn maps_f5_to_run_project() {
        let msg = Msg::from(Event::Key(KeyEvent::new(KeyCode::F(5))));
        assert_eq!(msg, Msg::RunProject);
    }

    #[test]
    fn maps_ctrl_shift_m_to_add_project_module() {
        let msg = Msg::from(Event::Key(
            KeyEvent::new(KeyCode::Char('m')).with_modifiers(Modifiers::CTRL | Modifiers::SHIFT),
        ));
        assert_eq!(msg, Msg::AddProjectModule);
    }

    #[test]
    fn maps_ctrl_s_to_save_active_buffer() {
        let msg = Msg::from(Event::Key(
            KeyEvent::new(KeyCode::Char('s')).with_modifiers(Modifiers::CTRL),
        ));
        assert_eq!(msg, Msg::SaveActiveBuffer);
    }

    #[test]
    fn maps_ctrl_shift_s_to_save_all_dirty_buffers() {
        // Ctrl+Shift+S must take precedence over Ctrl+S because the
        // `Shift` modifier is a superset check. The ordering in the
        // `Msg::from` match must put save-all before save.
        let msg = Msg::from(Event::Key(
            KeyEvent::new(KeyCode::Char('s')).with_modifiers(Modifiers::CTRL | Modifiers::SHIFT),
        ));
        assert_eq!(msg, Msg::SaveAllDirtyBuffers);
    }

    #[test]
    fn maps_ctrl_z_to_undo_active_buffer() {
        let msg = Msg::from(Event::Key(
            KeyEvent::new(KeyCode::Char('z')).with_modifiers(Modifiers::CTRL),
        ));
        assert_eq!(msg, Msg::UndoActiveBuffer);
    }

    #[test]
    fn maps_ctrl_y_to_redo_active_buffer() {
        let msg = Msg::from(Event::Key(
            KeyEvent::new(KeyCode::Char('y')).with_modifiers(Modifiers::CTRL),
        ));
        assert_eq!(msg, Msg::RedoActiveBuffer);
    }

    #[test]
    fn save_dispatches_through_model_and_clears_dirty_marker() {
        // End-to-end: type a character, send Ctrl+S, confirm the
        // buffer's dirty flag clears. Uses the thin-slice fixture so
        // the buffer has a real on-disk `source_path`.
        //
        // To keep the test hermetic we copy the thin-slice module
        // into a scratch directory, then restore the original text
        // after the test so the checked-in fixture does not change.
        let scratch = seed_project_fixture("save_dispatches_through_model_and_clears_dirty_marker");
        let mut model = ShellModel::new(Some(scratch.clone()));
        assert_eq!(
            model.scene(),
            ShellScene::Editing,
            "precondition: project mounted"
        );
        model.update(Msg::FocusRegion(FocusRegion::Editor));
        model.update(Msg::InsertEditorChar('Z'));
        assert!(
            model
                .shell
                .runtime
                .workspace
                .buffers
                .iter()
                .any(|buffer| buffer.dirty),
            "precondition: typed edit marks the active buffer dirty"
        );

        model.update(Msg::SaveActiveBuffer);

        assert!(
            model
                .shell
                .runtime
                .workspace
                .buffers
                .iter()
                .all(|buffer| !buffer.dirty),
            "Ctrl+S must clear dirty on every buffer that carries a source_path"
        );
    }

    #[test]
    fn save_is_suppressed_while_palette_overlay_is_open() {
        // Modal invariant: Ctrl+S inside the palette should not save
        // (the palette's Enter-key handler owns acceptance). After
        // Esc closes the overlay, the user's dirty marker is still
        // there and a second Ctrl+S persists.
        let scratch = seed_project_fixture("save_is_suppressed_while_palette_overlay_is_open");
        let mut model = ShellModel::new(Some(scratch));
        model.update(Msg::FocusRegion(FocusRegion::Editor));
        model.update(Msg::InsertEditorChar('Z'));

        model.update(Msg::TogglePalette);
        assert!(model.palette_active(), "precondition: palette is open");

        model.update(Msg::SaveActiveBuffer);
        assert!(
            model
                .shell
                .runtime
                .workspace
                .buffers
                .iter()
                .any(|buffer| buffer.dirty),
            "save must be suppressed while an overlay is up"
        );

        model.update(Msg::CloseOverlay);
        model.update(Msg::SaveActiveBuffer);
        assert!(
            model
                .shell
                .runtime
                .workspace
                .buffers
                .iter()
                .all(|buffer| !buffer.dirty),
            "save must succeed once the overlay closes"
        );
    }

    #[test]
    fn undo_and_redo_round_trip_through_the_model() {
        let scratch = seed_project_fixture("undo_and_redo_round_trip_through_the_model");
        let mut model = ShellModel::new(Some(scratch));
        model.update(Msg::FocusRegion(FocusRegion::Editor));

        let before = model.shell.runtime.workspace.buffers[0].lines.clone();
        model.update(Msg::InsertEditorChar('A'));
        let after_edit = model.shell.runtime.workspace.buffers[0].lines.clone();
        assert_ne!(before, after_edit, "edit must mutate lines");

        model.update(Msg::UndoActiveBuffer);
        assert_eq!(
            model.shell.runtime.workspace.buffers[0].lines, before,
            "Ctrl+Z must restore pre-edit lines"
        );

        model.update(Msg::RedoActiveBuffer);
        assert_eq!(
            model.shell.runtime.workspace.buffers[0].lines, after_edit,
            "Ctrl+Y must re-apply the undone edit"
        );
    }

    #[test]
    fn maps_ctrl_shift_r_to_open_com_reference_helper() {
        let msg = Msg::from(Event::Key(
            KeyEvent::new(KeyCode::Char('r')).with_modifiers(Modifiers::CTRL | Modifiers::SHIFT),
        ));
        assert_eq!(msg, Msg::OpenComReferenceHelper);
    }

    #[test]
    fn maps_plain_char_to_editor_input() {
        let msg = Msg::from(Event::Key(KeyEvent::new(KeyCode::Char('x'))));
        assert_eq!(msg, Msg::InsertEditorChar('x'));
    }

    #[test]
    fn starts_in_empty_scene_without_startup_project() {
        let model = ShellModel::new(None);

        assert_eq!(model.shell.scene, ShellScene::Empty);
        assert!(!model.shell.runtime.recent_projects.is_empty());
        assert!(model.shell.runtime.session_workspace.is_none());
    }

    /// Uxpass D6: the `F2/F3/F4` scene-flip keys are dev affordances and the
    /// default build ignores their scene-change effect. The `Msg::SetScene`
    /// mapping from the key event is still produced (the From impl is
    /// self-contained), but `update()` must not apply the scene change.
    #[test]
    fn f2_does_not_change_scene_in_default_build() {
        let mut model = ShellModel::new(None);
        assert_eq!(model.shell.scene, ShellScene::Empty);

        // F2 would normally request Empty; apply another scene first so we
        // can observe that a subsequent F2 does NOT reset it in default mode.
        model.update(Msg::RunProject);
        let scene_before = model.shell.scene;
        model.update(Msg::SetScene(ShellScene::Empty));

        assert_eq!(
            model.shell.scene, scene_before,
            "F2 must be a no-op without --dev-scenes"
        );
    }

    /// Uxpass D6: with `--dev-scenes` the F2/F3/F4 scene-flips remain
    /// available so W035 prototyping can still flip between mockup shapes.
    #[test]
    fn f3_changes_scene_when_dev_scenes_enabled() {
        let mut model = ShellModel::with_dev_scenes(None, true);
        assert_eq!(model.shell.scene, ShellScene::Empty);

        model.update(Msg::SetScene(ShellScene::Editing));

        assert_eq!(model.shell.scene, ShellScene::Editing);
    }

    /// Uxpass D6 + D8: the Empty scene's status line announces `Ctrl+O`
    /// and never the dev-only F2/F3/F4 scene-flip bindings in the default
    /// build. (The previous "Welcome hint" paragraph inside the Welcome
    /// body is gone — D1b merged it into the always-present status line.)
    #[test]
    fn empty_status_line_announces_ctrl_o_and_omits_dev_scene_flips() {
        let model = ShellModel::new(None);
        let hint = model.status_line_hint();

        assert!(
            hint.contains("Ctrl+O"),
            "Empty status line must announce Ctrl+O (D8), got: {hint:?}"
        );
        for banned in ["F2", "F3", "F4"] {
            assert!(
                !hint.contains(banned),
                "default-build status line must not mention {banned} (D6), got: {hint:?}"
            );
        }
    }

    /// Uxpass D6: the palette's `Mockup States` group is dev-only.
    #[test]
    fn palette_state_commands_are_empty_in_default_build() {
        let model = ShellModel::new(None);
        assert!(
            model.shell.runtime.content.palette.state_commands.is_empty(),
            "default-build palette must not surface the Mockup States group"
        );
    }

    #[test]
    fn palette_state_commands_are_populated_when_dev_scenes_enabled() {
        let model = ShellModel::with_dev_scenes(None, true);
        let state_commands = &model.shell.runtime.content.palette.state_commands;
        assert!(
            !state_commands.is_empty(),
            "--dev-scenes must repopulate the Mockup States group"
        );
        assert!(
            state_commands
                .iter()
                .any(|cmd| cmd.shortcut == "F2" && cmd.label == "Empty")
        );
    }

    #[test]
    fn starts_in_editing_scene_when_given_startup_project() {
        let model = ShellModel::new(Some(PathBuf::from(
            "examples/thin-slice/ThinSliceHello.basproj",
        )));

        assert_eq!(model.shell.scene, ShellScene::Editing);
        assert_eq!(
            model.shell.runtime.workspace.project_name.as_deref(),
            Some("ThinSliceHello")
        );
    }

    #[test]
    fn run_project_transitions_to_build_run_scene_and_updates_output() {
        let mut model = ShellModel::new(Some(PathBuf::from(
            "examples/thin-slice/ThinSliceHello.basproj",
        )));

        model.update(Msg::RunProject);

        assert_eq!(model.shell.scene, ShellScene::BuildRun);
        assert_eq!(model.shell.runtime.execution.runtime_status, "completed");
        assert!(
            model
                .shell
                .runtime
                .execution
                .output_lines
                .iter()
                .any(|line| line.contains("project run completed"))
        );
    }

    #[test]
    fn editor_input_mutates_the_active_buffer_when_editor_is_focused() {
        let mut model = ShellModel::new(Some(PathBuf::from(
            "examples/thin-slice/ThinSliceHello.basproj",
        )));
        model.shell.focus_region(FocusRegion::Editor);

        model.update(Msg::InsertEditorChar('X'));

        let buffer = model
            .shell
            .runtime
            .workspace
            .active_buffer()
            .expect("buffer");
        assert!(buffer.dirty);
        assert!(buffer.lines[0].contains('X'));
    }

    #[test]
    fn add_project_module_reloads_workspace_with_new_module() {
        let fixture = seed_project_fixture("model-add-module");
        let mut model = ShellModel::new(Some(fixture));

        model.update(Msg::AddProjectModule);

        assert!(
            model
                .shell
                .runtime
                .workspace
                .project
                .as_ref()
                .is_some_and(|project| project
                    .modules
                    .iter()
                    .any(|module| module.include == "Module2.bas"))
        );
    }

    #[test]
    fn cycle_project_target_reloads_workspace_with_new_output_type() {
        let fixture = seed_project_fixture("model-cycle-target");
        let mut model = ShellModel::new(Some(fixture));

        model.update(Msg::CycleProjectTarget);

        assert_eq!(model.shell.runtime.workspace.target_name, "Library");
    }

    #[test]
    fn com_reference_helper_opens_and_discovers_prog_id_candidate() {
        let fixture = seed_project_fixture("model-com-helper");
        let mut model = ShellModel::new(Some(fixture));

        model.update(Msg::OpenComReferenceHelper);
        for ch in "OxVba.TestDispatch".chars() {
            model.update(Msg::InsertEditorChar(ch));
        }

        assert!(model.com_reference_helper_active());
        assert!(
            model
                .shell
                .runtime
                .com_reference_helper
                .candidates
                .iter()
                .any(|candidate| candidate.title == "OxVba.TestDispatch")
        );
    }

    fn seed_project_fixture(name: &str) -> PathBuf {
        let root = PathBuf::from("target")
            .join("test-workspaces")
            .join("model")
            .join(name);
        std::fs::create_dir_all(&root).unwrap();
        let basproj = root.join("FixtureApp.basproj");
        std::fs::write(
            &basproj,
            r#"<Project Sdk="OxVba.Sdk/0.1.0">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <ProjectName>FixtureApp</ProjectName>
    <EntryPoint>Module1.Main</EntryPoint>
  </PropertyGroup>
  <ItemGroup>
    <Module Include="Module1.bas" />
  </ItemGroup>
</Project>
"#,
        )
        .unwrap();
        std::fs::write(
            root.join("Module1.bas"),
            "Option Explicit\n\nPublic Sub Main()\nEnd Sub\n",
        )
        .unwrap();
        basproj
    }
}
