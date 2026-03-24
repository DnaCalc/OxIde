use std::io;

use ftui::layout::{Constraint, Flex, Rect};
use ftui::prelude::{App, Cmd, Event, Frame, KeyCode, KeyEvent, Model, Modifiers, ScreenMode};
use ftui::widgets::Widget;
use ftui::widgets::block::{Alignment, Block};
use ftui::widgets::borders::Borders;
use ftui::widgets::paragraph::Paragraph;

struct ShellModel {
    show_help: bool,
    status: String,
}

impl ShellModel {
    fn new() -> Self {
        Self {
            show_help: true,
            status: String::from("Shell ready. Press q to quit."),
        }
    }

    fn header_text(&self) -> &'static str {
        "OxIde  |  console shell frame"
    }

    fn body_text(&self) -> String {
        let help_state = if self.show_help {
            "help visible"
        } else {
            "help hidden"
        };

        if self.show_help {
            format!(
                "The first OxIde shell frame is running.\n\n\
                 Scope in this bead:\n\
                 - FrankenTui event loop\n\
                 - base shell layout\n\
                 - fullscreen terminal lifecycle\n\n\
                 Next beads will add:\n\
                 - editor surface\n\
                 - command input\n\
                 - OxVba build and run integration\n\n\
                 Keys:\n\
                 - q / Esc / Ctrl-C: quit\n\
                 - ?: toggle this help\n\n\
                 State: {help_state}"
            )
        } else {
            format!(
                "OxIde shell frame is active.\n\
                 Press ? for help.\n\n\
                 State: {help_state}"
            )
        }
    }

    fn footer_text(&self) -> String {
        format!("q quit  ? help  |  {}", self.status)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Msg {
    Quit,
    ToggleHelp,
    Resized(u16, u16),
    Ignored,
}

impl From<Event> for Msg {
    fn from(event: Event) -> Self {
        match event {
            Event::Key(key) if is_quit_key(key) => Msg::Quit,
            Event::Key(key) if key.is_char('?') => Msg::ToggleHelp,
            Event::Resize { width, height } => Msg::Resized(width, height),
            _ => Msg::Ignored,
        }
    }
}

impl Model for ShellModel {
    type Message = Msg;

    fn update(&mut self, msg: Self::Message) -> Cmd<Self::Message> {
        match msg {
            Msg::Quit => Cmd::quit(),
            Msg::ToggleHelp => {
                self.show_help = !self.show_help;
                self.status = if self.show_help {
                    String::from("Help opened.")
                } else {
                    String::from("Help hidden.")
                };
                Cmd::none()
            }
            Msg::Resized(width, height) => {
                self.status = format!("Resized to {width}x{height}.");
                Cmd::none()
            }
            Msg::Ignored => Cmd::none(),
        }
    }

    fn view(&self, frame: &mut Frame) {
        let area = Rect::new(0, 0, frame.width(), frame.height());
        let sections = Flex::vertical()
            .constraints([Constraint::Fixed(3), Constraint::Fill, Constraint::Fixed(3)])
            .split(area);

        Paragraph::new(self.header_text())
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .title("OxIde")
                    .title_alignment(Alignment::Center),
            )
            .render(sections[0], frame);

        Paragraph::new(self.body_text())
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .title("Shell")
                    .title_alignment(Alignment::Center),
            )
            .render(sections[1], frame);

        Paragraph::new(self.footer_text())
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .title("Status")
                    .title_alignment(Alignment::Center),
            )
            .render(sections[2], frame);
    }
}

fn is_quit_key(key: KeyEvent) -> bool {
    matches!(key.code, KeyCode::Escape)
        || key.is_char('q')
        || (key.is_char('c') && key.modifiers.contains(Modifiers::CTRL))
}

fn main() -> io::Result<()> {
    App::new(ShellModel::new())
        .screen_mode(ScreenMode::AltScreen)
        .run()
}

#[cfg(test)]
mod tests {
    use super::{Msg, ShellModel, is_quit_key};
    use ftui::prelude::{Cmd, Event, KeyCode, KeyEvent, Model, Modifiers};

    #[test]
    fn quit_key_mapping_covers_expected_shortcuts() -> Result<(), String> {
        let cases = [
            KeyEvent::new(KeyCode::Char('q')),
            KeyEvent::new(KeyCode::Escape),
            KeyEvent::new(KeyCode::Char('c')).with_modifiers(Modifiers::CTRL),
        ];

        for case in cases {
            if !is_quit_key(case) {
                return Err(format!("expected key to quit: {case:?}"));
            }
        }

        Ok(())
    }

    #[test]
    fn question_mark_toggles_help() -> Result<(), String> {
        let msg = Msg::from(Event::Key(KeyEvent::new(KeyCode::Char('?'))));

        if !matches!(msg, Msg::ToggleHelp) {
            return Err(String::from("question mark should toggle help"));
        }

        Ok(())
    }

    #[test]
    fn toggle_help_updates_model_state() -> Result<(), String> {
        let mut model = ShellModel::new();
        let cmd = model.update(Msg::ToggleHelp);

        if model.show_help {
            return Err(String::from("toggle should hide help on first press"));
        }

        if !model.status.contains("hidden") {
            return Err(String::from("status should mention hidden help"));
        }

        if !matches!(cmd, Cmd::None) {
            return Err(String::from("toggle should not request a side effect"));
        }

        Ok(())
    }
}
