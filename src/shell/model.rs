use ftui::prelude::{Cmd, Event, Frame, KeyCode, KeyEvent, Model, Modifiers};

use super::mock_data::{ShellPanels, shell_panels};
use super::state::{FocusRegion, LowerSurfaceMode, MockState, ShellState};
use super::view;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Msg {
    Quit,
    NextFocus,
    SetMockState(MockState),
    Resized(u16, u16),
    Noop,
}

impl From<Event> for Msg {
    fn from(event: Event) -> Self {
        match event {
            Event::Key(key) if is_quit_key(key) => Msg::Quit,
            Event::Key(key) if matches!(key.code, KeyCode::Tab) => Msg::NextFocus,
            Event::Key(key) if matches!(key.code, KeyCode::F(2)) => {
                Msg::SetMockState(MockState::Empty)
            }
            Event::Key(key) if matches!(key.code, KeyCode::F(3)) => {
                Msg::SetMockState(MockState::Editing)
            }
            Event::Key(key) if matches!(key.code, KeyCode::F(4)) => {
                Msg::SetMockState(MockState::Semantic)
            }
            Event::Key(key) if matches!(key.code, KeyCode::F(5)) => {
                Msg::SetMockState(MockState::BuildRun)
            }
            Event::Key(key) if matches!(key.code, KeyCode::F(6)) => {
                Msg::SetMockState(MockState::Palette)
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
        Self {
            shell: ShellState::default(),
        }
    }

    pub fn panels(&self) -> ShellPanels {
        shell_panels(&self.shell)
    }

    pub fn palette_active(&self) -> bool {
        self.shell.palette_active()
    }

    pub fn inspector_is_collapsed(&self) -> bool {
        self.shell.inspector_is_collapsed()
    }

    pub fn focus(&self) -> FocusRegion {
        self.shell.focus
    }

    pub fn inspector_title(&self) -> String {
        format!("Inspector {}", self.shell.inspector_mode.label())
    }

    pub fn lower_surface_title(&self) -> String {
        let mode = self.shell.lower_mode;
        match mode {
            LowerSurfaceMode::Launcher => String::from("Lower Surface Launcher"),
            _ => format!("Lower Surface {}", mode.label()),
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
            Msg::SetMockState(mock_state) => {
                self.shell.apply_mock_state(mock_state);
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
