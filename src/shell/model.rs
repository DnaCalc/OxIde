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
}

impl ShellModel {
    pub fn new(project_path: Option<PathBuf>) -> Self {
        let mut model = Self {
            shell: ShellState::default(),
            project_path: None,
            com_reference_candidates: Vec::new(),
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

    pub fn explorer_title(&self) -> &'static str {
        match self.shell.scene {
            ShellScene::Empty => "Launcher",
            _ => "Explorer",
        }
    }

    pub fn inspector_title(&self) -> String {
        match self.shell.scene {
            ShellScene::Empty => String::from("Environment"),
            _ => format!("Inspector {}", self.shell.runtime.inspector_mode.label()),
        }
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
                self.shell.apply_scene(scene);
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
