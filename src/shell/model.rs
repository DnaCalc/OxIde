use ftui::{
    KeyEventKind,
    prelude::{Cmd, Event, Frame, KeyCode, KeyEvent, Model, Modifiers},
};

use super::mock_data::{ShellPanels, shell_panels};
use super::session::ProjectSession;
use super::state::{FocusRegion, LowerSurfaceMode, ShellScene, ShellState, WidthClass};
use super::view;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Msg {
    Quit,
    NextFocus,
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
            Event::Key(key) if is_toggle_palette_key(key) => Msg::TogglePalette,
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
            Event::Key(key) if matches!(key.code, KeyCode::F(5)) => {
                Msg::SetScene(ShellScene::BuildRun)
            }
            Event::Resize { width, height } => Msg::Resized(width, height),
            _ => Msg::Noop,
        }
    }
}

pub struct ShellModel {
    shell: ShellState,
}

impl ShellModel {
    pub fn new() -> Self {
        let mut model = Self {
            shell: ShellState::default(),
        };
        model.try_mount_example_workspace();
        model
    }

    pub fn panels(&self) -> ShellPanels {
        shell_panels(&self.shell)
    }

    pub fn palette_active(&self) -> bool {
        self.shell.palette_active()
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

    fn try_mount_example_workspace(&mut self) {
        let project_path = "examples/thin-slice/ThinSliceHello.basproj";
        if let Ok(session) = ProjectSession::load(project_path) {
            self.shell.mount_workspace(session.workspace_state());
        }
    }
}

impl Model for ShellModel {
    type Message = Msg;

    fn update(&mut self, msg: Self::Message) -> Cmd<Self::Message> {
        match msg {
            Msg::Quit => Cmd::quit(),
            Msg::NextFocus => {
                self.shell.cycle_focus();
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

fn is_toggle_palette_key(key: KeyEvent) -> bool {
    matches!(key.code, KeyCode::F(6))
}

fn is_focus_region_key(key: KeyEvent, digit: char) -> bool {
    matches!(key.code, KeyCode::Char(value) if value == digit)
        && key.modifiers.contains(Modifiers::ALT)
}

fn is_cycle_editor_view_key(key: KeyEvent) -> bool {
    matches!(key.code, KeyCode::Tab) && key.modifiers.contains(Modifiers::CTRL)
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
}
