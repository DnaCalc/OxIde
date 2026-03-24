use std::cell::RefCell;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::PathBuf;

use ftui::layout::{Constraint, Flex, Rect};
use ftui::prelude::{App, Cmd, Event, Frame, KeyCode, KeyEvent, Model, Modifiers, ScreenMode};
use ftui::widgets::block::{Alignment, Block};
use ftui::widgets::borders::Borders;
use ftui::widgets::paragraph::Paragraph;
use ftui::widgets::textarea::{TextArea, TextAreaState};
use ftui::widgets::{StatefulWidget, Widget};

struct ShellModel {
    show_help: bool,
    document: Document,
    editor: TextArea,
    editor_state: RefCell<TextAreaState>,
    status: String,
}

struct Document {
    path: Option<PathBuf>,
    has_backing_file: bool,
    saved_text: String,
}

impl Document {
    fn load(path: Option<PathBuf>) -> io::Result<(Self, String)> {
        match path {
            Some(path) if path.exists() => {
                let text = fs::read_to_string(&path)?;
                let status = format!("Opened {}.", path.display());
                Ok((
                    Self {
                        path: Some(path),
                        has_backing_file: true,
                        saved_text: text.clone(),
                    },
                    status,
                ))
            }
            Some(path) => {
                let status = format!("New file {}.", path.display());
                Ok((
                    Self {
                        path: Some(path),
                        has_backing_file: false,
                        saved_text: String::new(),
                    },
                    status,
                ))
            }
            None => Ok((
                Self {
                    path: None,
                    has_backing_file: false,
                    saved_text: String::new(),
                },
                String::from("Untitled buffer."),
            )),
        }
    }

    fn display_name(&self) -> String {
        match &self.path {
            Some(path) => path.display().to_string(),
            None => String::from("untitled"),
        }
    }

    fn is_dirty(&self, current_text: &str) -> bool {
        current_text != self.saved_text
    }

    fn state_label(&self, current_text: &str) -> &'static str {
        if self.path.is_none() {
            "untitled"
        } else if !self.has_backing_file {
            if self.is_dirty(current_text) {
                "new*"
            } else {
                "new"
            }
        } else if self.is_dirty(current_text) {
            "modified"
        } else {
            "saved"
        }
    }

    fn save(&mut self, current_text: &str) -> io::Result<String> {
        let path = match &self.path {
            Some(path) => path,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "no file path is associated with this buffer",
                ));
            }
        };

        fs::write(path, current_text)?;
        self.has_backing_file = true;
        self.saved_text = String::from(current_text);
        Ok(format!("Saved {}.", path.display()))
    }
}

impl ShellModel {
    fn new(path: Option<PathBuf>) -> io::Result<Self> {
        let (document, status) = Document::load(path)?;
        let editor = new_editor(&document.saved_text);

        Ok(Self {
            show_help: true,
            document,
            editor,
            editor_state: RefCell::new(TextAreaState::default()),
            status,
        })
    }

    fn header_text(&self) -> String {
        format!(
            "OxIde  |  {}{}",
            self.document.display_name(),
            self.dirty_marker()
        )
    }

    fn help_text(&self) -> &'static str {
        "Editor keys\n\n\
         Ctrl-Q  quit\n\
         Ctrl-S  save current file\n\
         F1      toggle help\n\
         arrows  move cursor\n\
         Enter   newline\n\
         Backspace/Delete edit text\n\
         Ctrl-K  delete to end of line\n\
         Ctrl-Z  undo\n\
         Ctrl-Y  redo\n\n\
         Start with: cargo run -- path/to/file.bas"
    }

    fn footer_text(&self) -> String {
        let cursor = self.editor.cursor();
        let file_state = self.document.state_label(&self.editor.text());
        format!(
            "Ctrl-Q quit  Ctrl-S save  F1 help  |  line {} col {}  lines {}  |  {}  |  {}",
            cursor.line + 1,
            cursor.visual_col + 1,
            self.editor.line_count(),
            file_state,
            self.status
        )
    }

    fn buffer_title(&self) -> String {
        format!(
            "Buffer  {}{}",
            self.document.display_name(),
            self.dirty_marker()
        )
    }

    fn dirty_marker(&self) -> &'static str {
        if self.is_dirty() { " *" } else { "" }
    }

    fn is_dirty(&self) -> bool {
        self.document.is_dirty(&self.editor.text())
    }

    fn save_current_file(&mut self) {
        let current_text = self.editor.text();
        self.status = match self.document.save(&current_text) {
            Ok(status) => status,
            Err(error) if error.kind() == io::ErrorKind::InvalidInput => {
                String::from("No file path yet. Start OxIde with a file path for save support.")
            }
            Err(error) => format!("Save failed: {error}"),
        };
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Msg {
    Quit,
    Save,
    ToggleHelp,
    Editor(Event),
    Resized(u16, u16),
}

impl From<Event> for Msg {
    fn from(event: Event) -> Self {
        match event {
            Event::Key(key) if is_quit_key(key) => Msg::Quit,
            Event::Key(key) if is_save_key(key) => Msg::Save,
            Event::Key(key) if is_help_key(key) => Msg::ToggleHelp,
            Event::Resize { width, height } => Msg::Resized(width, height),
            other => Msg::Editor(other),
        }
    }
}

impl Model for ShellModel {
    type Message = Msg;

    fn update(&mut self, msg: Self::Message) -> Cmd<Self::Message> {
        match msg {
            Msg::Quit => Cmd::quit(),
            Msg::Save => {
                self.save_current_file();
                Cmd::none()
            }
            Msg::ToggleHelp => {
                self.show_help = !self.show_help;
                self.status = if self.show_help {
                    String::from("Help opened.")
                } else {
                    String::from("Help hidden.")
                };
                Cmd::none()
            }
            Msg::Editor(event) => {
                if self.editor.handle_event(&event) {
                    self.status = if self.is_dirty() {
                        String::from("Buffer modified.")
                    } else {
                        String::from("Buffer matches saved file.")
                    };
                }
                Cmd::none()
            }
            Msg::Resized(width, height) => {
                self.status = format!("Resized to {width}x{height}.");
                Cmd::none()
            }
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

        let body_sections = if self.show_help {
            Flex::horizontal()
                .constraints([Constraint::Percentage(72.0), Constraint::Fill])
                .split(sections[1])
        } else {
            vec![sections[1]]
        };

        let buffer_title = self.buffer_title();
        let editor_block = Block::new()
            .borders(Borders::ALL)
            .title(&buffer_title)
            .title_alignment(Alignment::Center);
        editor_block.render(body_sections[0], frame);
        let editor_area = editor_block.inner(body_sections[0]);
        StatefulWidget::render(
            &self.editor,
            editor_area,
            frame,
            &mut self.editor_state.borrow_mut(),
        );

        if self.show_help {
            Paragraph::new(self.help_text())
                .block(
                    Block::new()
                        .borders(Borders::ALL)
                        .title("Help")
                        .title_alignment(Alignment::Center),
                )
                .render(body_sections[1], frame);
        }

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

fn new_editor(text: &str) -> TextArea {
    let editor = TextArea::new()
        .with_placeholder("Type OxVba code here")
        .with_focus(true)
        .with_line_numbers(true);

    if text.is_empty() {
        editor
    } else {
        editor.with_text(text)
    }
}

fn is_quit_key(key: KeyEvent) -> bool {
    key.is_char('q') && key.modifiers.contains(Modifiers::CTRL)
}

fn is_save_key(key: KeyEvent) -> bool {
    key.is_char('s') && key.modifiers.contains(Modifiers::CTRL)
}

fn is_help_key(key: KeyEvent) -> bool {
    matches!(key.code, KeyCode::F(1))
}

fn startup_path_from_args<I>(args: I) -> io::Result<Option<PathBuf>>
where
    I: IntoIterator<Item = OsString>,
{
    let mut args = args.into_iter();
    let _program = args.next();

    let path = args.next().map(PathBuf::from);

    if args.next().is_some() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "expected at most one file path argument",
        ));
    }

    Ok(path)
}

fn main() -> io::Result<()> {
    let path = startup_path_from_args(env::args_os())?;

    App::new(ShellModel::new(path)?)
        .screen_mode(ScreenMode::AltScreen)
        .run()
}

#[cfg(test)]
mod tests {
    use super::{
        Document, Msg, ShellModel, is_help_key, is_quit_key, is_save_key, startup_path_from_args,
    };
    use ftui::prelude::{Cmd, Event, KeyCode, KeyEvent, Model, Modifiers};
    use std::env;
    use std::ffi::OsString;
    use std::path::PathBuf;

    #[test]
    fn quit_key_mapping_requires_ctrl_q() -> Result<(), String> {
        let quit = KeyEvent::new(KeyCode::Char('q')).with_modifiers(Modifiers::CTRL);

        if !is_quit_key(quit) {
            return Err(String::from("Ctrl-Q should quit"));
        }

        if is_quit_key(KeyEvent::new(KeyCode::Char('q'))) {
            return Err(String::from("plain q should remain editor input"));
        }

        Ok(())
    }

    #[test]
    fn ctrl_s_maps_to_save() -> Result<(), String> {
        let save = KeyEvent::new(KeyCode::Char('s')).with_modifiers(Modifiers::CTRL);

        if !is_save_key(save) {
            return Err(String::from("Ctrl-S should save"));
        }

        let msg = Msg::from(Event::Key(save));

        if !matches!(msg, Msg::Save) {
            return Err(String::from("Ctrl-S should map to Save"));
        }

        Ok(())
    }

    #[test]
    fn f1_toggles_help() -> Result<(), String> {
        let help_key = KeyEvent::new(KeyCode::F(1));

        if !is_help_key(help_key) {
            return Err(String::from("F1 should toggle help"));
        }

        let msg = Msg::from(Event::Key(help_key));

        if !matches!(msg, Msg::ToggleHelp) {
            return Err(String::from("F1 should map to ToggleHelp"));
        }

        Ok(())
    }

    #[test]
    fn toggle_help_updates_model_state() -> Result<(), String> {
        let mut model = ShellModel::new(None).map_err(|error| error.to_string())?;
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

    #[test]
    fn editor_events_mark_the_buffer_dirty() -> Result<(), String> {
        let mut model = ShellModel::new(None).map_err(|error| error.to_string())?;
        let cmd = model.update(Msg::Editor(Event::Key(KeyEvent::new(KeyCode::Char('a')))));

        if model.editor.text() != "a" {
            return Err(String::from("editor should insert typed characters"));
        }

        if !model.is_dirty() {
            return Err(String::from("editing should mark the buffer dirty"));
        }

        if !model.status.contains("modified") {
            return Err(String::from("status should report dirty edits"));
        }

        if !matches!(cmd, Cmd::None) {
            return Err(String::from("editing should not request a side effect"));
        }

        Ok(())
    }

    #[test]
    fn save_without_a_path_reports_the_constraint() -> Result<(), String> {
        let mut model = ShellModel::new(None).map_err(|error| error.to_string())?;
        model.update(Msg::Editor(Event::Key(KeyEvent::new(KeyCode::Char('a')))));
        model.update(Msg::Save);

        if !model.status.contains("No file path yet") {
            return Err(String::from("missing path should be reported"));
        }

        if !model.is_dirty() {
            return Err(String::from("failed save should keep the buffer dirty"));
        }

        Ok(())
    }

    #[test]
    fn existing_file_is_loaded_into_the_buffer() -> Result<(), String> {
        let path = env::current_dir()
            .map_err(|error| error.to_string())?
            .join("Cargo.toml");
        let model = ShellModel::new(Some(path)).map_err(|error| error.to_string())?;

        if !model.editor.text().contains("name = \"ox-ide\"") {
            return Err(String::from("existing file should be loaded"));
        }

        if model.is_dirty() {
            return Err(String::from("freshly loaded file should not be dirty"));
        }

        Ok(())
    }

    #[test]
    fn missing_startup_file_is_shown_as_new() -> Result<(), String> {
        let path = PathBuf::from(env::temp_dir()).join("oxide-bd-237-4-new-file.bas");
        let model = ShellModel::new(Some(path)).map_err(|error| error.to_string())?;

        if !model.footer_text().contains("new") {
            return Err(String::from("missing startup file should be shown as new"));
        }

        Ok(())
    }

    #[test]
    fn document_save_updates_saved_state() -> Result<(), String> {
        let path = PathBuf::from(env::temp_dir()).join("oxide-bd-237-4-save-test.bas");
        let mut document = Document {
            path: Some(path.clone()),
            has_backing_file: false,
            saved_text: String::new(),
        };

        let status = document
            .save("Print \"Hello\"")
            .map_err(|error| error.to_string())?;

        if !status.contains(path.to_string_lossy().as_ref()) {
            return Err(String::from("save status should include the file path"));
        }

        if document.is_dirty("Print \"Hello\"") {
            return Err(String::from("saved text should reset dirty state"));
        }

        if document.state_label("Print \"Hello\"") != "saved" {
            return Err(String::from("saved document should report saved state"));
        }

        Ok(())
    }

    #[test]
    fn startup_path_accepts_at_most_one_file_argument() -> Result<(), String> {
        let args = vec![
            OsString::from("oxide"),
            OsString::from("one.bas"),
            OsString::from("two.bas"),
        ];

        if startup_path_from_args(args).is_ok() {
            return Err(String::from("only one startup file should be accepted"));
        }

        Ok(())
    }
}
