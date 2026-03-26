use std::cell::RefCell;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;

use ftui::layout::{Constraint, Flex, Rect};
use ftui::prelude::{App, Cmd, Event, Frame, KeyCode, KeyEvent, Model, Modifiers, ScreenMode};
use ftui::widgets::block::{Alignment, Block};
use ftui::widgets::borders::Borders;
use ftui::widgets::paragraph::Paragraph;
use ftui::widgets::textarea::{TextArea, TextAreaState};
use ftui::widgets::{StatefulWidget, Widget};
use oxvba_compiler::ProjectManifest;

struct ShellModel {
    show_help: bool,
    command_input: CommandInput,
    project_session: ProjectSession,
    document_session: DocumentSession,
    oxvba_services: Box<dyn OxVbaServices>,
    last_execution: Option<OxVbaExecutionResult>,
    editor: TextArea,
    editor_state: RefCell<TextAreaState>,
    status: String,
}

#[derive(Default)]
struct CommandInput {
    active: bool,
    value: String,
}

struct DocumentSession {
    path: Option<PathBuf>,
    has_backing_file: bool,
    saved_text: String,
}

#[derive(Default)]
struct ProjectSession {
    project_path: Option<PathBuf>,
    manifest: Option<ProjectManifest>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum OxVbaExecutionAction {
    Build,
    Run,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum OxVbaExecutionTarget {
    LooseFile(PathBuf),
    Project(PathBuf),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OxVbaExecutionRequest {
    action: OxVbaExecutionAction,
    target: OxVbaExecutionTarget,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OxVbaExecutionResult {
    action: OxVbaExecutionAction,
    target: OxVbaExecutionTarget,
    success: bool,
    exit_code: Option<i32>,
    stdout: String,
    stderr: String,
}

trait OxVbaServices {
    fn execute(&self, request: &OxVbaExecutionRequest) -> io::Result<OxVbaExecutionResult>;
}

struct CargoOxVbaServices {
    workspace_root: PathBuf,
}

impl CargoOxVbaServices {
    fn discover() -> Self {
        let workspace_root = env::var_os("OXVBA_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("../OxVba"));
        Self { workspace_root }
    }
}

impl OxVbaServices for CargoOxVbaServices {
    fn execute(&self, request: &OxVbaExecutionRequest) -> io::Result<OxVbaExecutionResult> {
        let output = Command::new("cargo")
            .args(oxvba_cli_args_for_request(request))
            .current_dir(&self.workspace_root)
            .output()?;

        Ok(OxVbaExecutionResult {
            action: request.action.clone(),
            target: request.target.clone(),
            success: output.status.success(),
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).trim().to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_string(),
        })
    }
}

impl DocumentSession {
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

    fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    fn saved_text(&self) -> &str {
        &self.saved_text
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

    fn save_as(&mut self, path: PathBuf, current_text: &str) -> io::Result<String> {
        self.path = Some(path);
        self.save(current_text)
    }

    fn open(path: PathBuf) -> io::Result<(Self, String)> {
        Self::load(Some(path))
    }

    fn execution_target(&self, current_text: &str) -> Result<OxVbaExecutionTarget, String> {
        if self.path.is_none() {
            return Err(String::from("Build/run requires a file path."));
        }

        if self.is_dirty(current_text) {
            return Err(String::from("Save the current buffer before build/run."));
        }

        let path = match &self.path {
            Some(path) => path.clone(),
            None => return Err(String::from("Build/run requires a file path.")),
        };
        let target_path = if path.is_absolute() {
            path
        } else {
            env::current_dir()
                .map_err(|error| format!("Cannot resolve working directory: {error}"))?
                .join(path)
        };

        let extension = target_path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_ascii_lowercase());

        if extension.as_deref() == Some("basproj") {
            Ok(OxVbaExecutionTarget::Project(target_path))
        } else {
            Ok(OxVbaExecutionTarget::LooseFile(target_path))
        }
    }
}

impl ProjectSession {
    fn from_document_path(path: Option<&PathBuf>) -> Self {
        let mut session = Self::default();
        if let Some(path) = path
            && path
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case("basproj"))
        {
            session.bind_path(path.clone());
        }
        session
    }

    fn bind_path(&mut self, path: PathBuf) {
        self.project_path = Some(path);
        self.manifest = None;
    }

    fn bind_manifest(&mut self, project_path: PathBuf, manifest: ProjectManifest) {
        self.project_path = Some(project_path);
        self.manifest = Some(manifest);
    }

    fn clear(&mut self) {
        self.project_path = None;
        self.manifest = None;
    }

    fn display_name(&self) -> String {
        match &self.project_path {
            Some(path) => path.display().to_string(),
            None => String::from("(no project)"),
        }
    }

    fn has_manifest(&self) -> bool {
        self.manifest.is_some()
    }
}

impl ShellModel {
    fn new(path: Option<PathBuf>) -> io::Result<Self> {
        Self::with_services(path, Box::new(CargoOxVbaServices::discover()))
    }

    fn with_services(
        path: Option<PathBuf>,
        oxvba_services: Box<dyn OxVbaServices>,
    ) -> io::Result<Self> {
        let (document_session, status) = DocumentSession::load(path)?;
        let project_session = ProjectSession::from_document_path(document_session.path());
        let editor = new_editor(document_session.saved_text());

        Ok(Self {
            show_help: true,
            command_input: CommandInput::default(),
            project_session,
            document_session,
            oxvba_services,
            last_execution: None,
            editor,
            editor_state: RefCell::new(TextAreaState::default()),
            status,
        })
    }

    fn header_text(&self) -> String {
        format!(
            "OxIde  |  Buffer: {}{}  |  Project: {}",
            self.document_session.display_name(),
            self.dirty_marker(),
            self.project_session.display_name()
        )
    }

    fn help_text(&self) -> &'static str {
        "Editor keys\n\n\
         Ctrl-Q  quit\n\
         Ctrl-S  save current file\n\
         :       command mode\n\
         F1      toggle help\n\
         arrows  move cursor\n\
         Enter   newline\n\
         Backspace/Delete edit text\n\
         Ctrl-K  delete to end of line\n\
         Ctrl-Z  undo\n\
         Ctrl-Y  redo\n\n\
         Commands\n\n\
         :open <path>\n\
         :write [path]\n\
         :build\n\
         :run\n\
         :quit\n\n\
         Start with: cargo run -- path/to/file.bas"
    }

    fn footer_text(&self) -> String {
        let cursor = self.editor.cursor();
        let file_state = self.document_session.state_label(&self.editor.text());
        let command_line = if self.command_input.active {
            format!(":{}", self.command_input.value)
        } else {
            String::from(": command mode  |  :open <path>  :write [path]  :build  :run  :quit")
        };
        format!(
            "Ctrl-Q quit  Ctrl-S save  : command  F1 help  |  line {} col {}  lines {}  |  {}  |  {}\n{}",
            cursor.line + 1,
            cursor.visual_col + 1,
            self.editor.line_count(),
            file_state,
            self.status,
            command_line
        )
    }

    fn output_title(&self) -> &'static str {
        "OxVba Output"
    }

    fn output_text(&self) -> String {
        let Some(result) = &self.last_execution else {
            return String::from("No OxVba output yet.");
        };

        let action = match result.action {
            OxVbaExecutionAction::Build => "Build",
            OxVbaExecutionAction::Run => "Run",
        };

        let mut lines = vec![
            format!("Action: {action}"),
            format!("Target: {}", result.target.display_name()),
            format!("Success: {}", if result.success { "yes" } else { "no" }),
        ];

        if let Some(code) = result.exit_code {
            lines.push(format!("Exit code: {code}"));
        }

        lines.push(String::new());
        lines.push(String::from("Stdout:"));
        lines.push(if result.stdout.is_empty() {
            String::from("(empty)")
        } else {
            result.stdout.clone()
        });
        lines.push(String::new());
        lines.push(String::from("Stderr:"));
        lines.push(if result.stderr.is_empty() {
            String::from("(empty)")
        } else {
            result.stderr.clone()
        });

        lines.join("\n")
    }

    fn buffer_title(&self) -> String {
        format!(
            "Buffer  {}{}",
            self.document_session.display_name(),
            self.dirty_marker()
        )
    }

    fn dirty_marker(&self) -> &'static str {
        if self.is_dirty() { " *" } else { "" }
    }

    fn is_dirty(&self) -> bool {
        self.document_session.is_dirty(&self.editor.text())
    }

    fn save_current_file(&mut self) {
        let current_text = self.editor.text();
        self.status = match self.document_session.save(&current_text) {
            Ok(status) => status,
            Err(error) if error.kind() == io::ErrorKind::InvalidInput => {
                String::from("No file path yet. Start OxIde with a file path for save support.")
            }
            Err(error) => format!("Save failed: {error}"),
        };
    }

    fn save_current_file_as(&mut self, path: PathBuf) {
        let current_text = self.editor.text();
        self.status = match self.document_session.save_as(path, &current_text) {
            Ok(status) => status,
            Err(error) => format!("Save failed: {error}"),
        };
    }

    fn enter_command_mode(&mut self) {
        self.command_input.active = true;
        self.command_input.value.clear();
        self.status = String::from("Command mode.");
    }

    fn cancel_command_mode(&mut self) {
        self.command_input.active = false;
        self.command_input.value.clear();
        self.status = String::from("Command cancelled.");
    }

    fn handle_command_event(&mut self, event: Event) -> Cmd<Msg> {
        let Event::Key(key) = event else {
            return Cmd::none();
        };

        match key.code {
            KeyCode::Escape => {
                self.cancel_command_mode();
                Cmd::none()
            }
            KeyCode::Enter => self.dispatch_command_line(),
            KeyCode::Backspace => {
                self.command_input.value.pop();
                Cmd::none()
            }
            KeyCode::Char(ch)
                if !key.modifiers.contains(Modifiers::CTRL)
                    && !key.modifiers.contains(Modifiers::ALT) =>
            {
                self.command_input.value.push(ch);
                Cmd::none()
            }
            _ => Cmd::none(),
        }
    }

    fn dispatch_command_line(&mut self) -> Cmd<Msg> {
        let raw = self.command_input.value.trim().to_string();
        self.command_input.active = false;
        self.command_input.value.clear();

        if raw.is_empty() {
            self.status = String::from("Empty command.");
            return Cmd::none();
        }

        let (command, arg) = split_command(&raw);

        match command {
            "open" => match arg {
                Some(path_text) => self.open_document(PathBuf::from(path_text)),
                None => {
                    self.status = String::from("Usage: :open <path>");
                    Cmd::none()
                }
            },
            "write" => {
                if let Some(path_text) = arg {
                    self.save_current_file_as(PathBuf::from(path_text));
                } else {
                    self.save_current_file();
                }
                Cmd::none()
            }
            "quit" => Cmd::quit(),
            "build" => self.execute_oxvba(OxVbaExecutionAction::Build),
            "run" => self.execute_oxvba(OxVbaExecutionAction::Run),
            _ => {
                self.status = format!("Unknown command: :{raw}");
                Cmd::none()
            }
        }
    }

    fn open_document(&mut self, path: PathBuf) -> Cmd<Msg> {
        match DocumentSession::open(path) {
            Ok((document_session, status)) => {
                self.document_session = document_session;
                self.project_session = ProjectSession::from_document_path(self.document_session.path());
                self.editor = new_editor(self.document_session.saved_text());
                self.editor_state = RefCell::new(TextAreaState::default());
                self.status = status;
            }
            Err(error) => {
                self.status = format!("Open failed: {error}");
            }
        }

        Cmd::none()
    }

    fn execute_oxvba(&mut self, action: OxVbaExecutionAction) -> Cmd<Msg> {
        let current_text = self.editor.text();
        let target = match self.document_session.execution_target(&current_text) {
            Ok(target) => target,
            Err(message) => {
                self.status = message;
                return Cmd::none();
            }
        };

        let request = OxVbaExecutionRequest { action, target };

        match self.oxvba_services.execute(&request) {
            Ok(result) => {
                self.status = result.status_summary();
                self.last_execution = Some(result);
            }
            Err(error) => {
                self.status = format!("OxVbaServices invocation failed: {error}");
                self.last_execution = None;
            }
        }

        Cmd::none()
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
                if self.command_input.active {
                    return self.handle_command_event(event);
                }

                if is_command_key(&event) {
                    self.enter_command_mode();
                    return Cmd::none();
                }

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
            .constraints([Constraint::Fixed(3), Constraint::Fill, Constraint::Fixed(4)])
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

        let main_sections = Flex::vertical()
            .constraints([Constraint::Percentage(68.0), Constraint::Fill])
            .split(body_sections[0]);

        let buffer_title = self.buffer_title();
        let editor_block = Block::new()
            .borders(Borders::ALL)
            .title(&buffer_title)
            .title_alignment(Alignment::Center);
        editor_block.render(main_sections[0], frame);
        let editor_area = editor_block.inner(main_sections[0]);
        StatefulWidget::render(
            &self.editor,
            editor_area,
            frame,
            &mut self.editor_state.borrow_mut(),
        );

        Paragraph::new(self.output_text())
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .title(self.output_title())
                    .title_alignment(Alignment::Center),
            )
            .render(main_sections[1], frame);

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

fn is_command_key(event: &Event) -> bool {
    matches!(
        event,
        Event::Key(key)
            if matches!(key.code, KeyCode::Char(':'))
                && !key.modifiers.contains(Modifiers::CTRL)
                && !key.modifiers.contains(Modifiers::ALT)
    )
}

fn split_command(input: &str) -> (&str, Option<&str>) {
    let mut parts = input.trim().splitn(2, char::is_whitespace);
    let command = parts.next().unwrap_or("");
    let argument = parts.next().map(str::trim).filter(|part| !part.is_empty());
    (command, argument)
}

fn oxvba_cli_args_for_request(request: &OxVbaExecutionRequest) -> Vec<OsString> {
    let mut args = vec![
        OsString::from("run"),
        OsString::from("--quiet"),
        OsString::from("-p"),
        OsString::from("oxvba-cli"),
        OsString::from("--"),
    ];

    match (&request.action, &request.target) {
        (OxVbaExecutionAction::Build, OxVbaExecutionTarget::LooseFile(path)) => {
            args.push(OsString::from("compile"));
            args.push(path.as_os_str().to_os_string());
        }
        (OxVbaExecutionAction::Build, OxVbaExecutionTarget::Project(path)) => {
            args.push(OsString::from("build"));
            args.push(path.as_os_str().to_os_string());
        }
        (OxVbaExecutionAction::Run, OxVbaExecutionTarget::LooseFile(path)) => {
            args.push(OsString::from("run"));
            args.push(path.as_os_str().to_os_string());
        }
        (OxVbaExecutionAction::Run, OxVbaExecutionTarget::Project(path)) => {
            args.push(OsString::from("run-project"));
            args.push(path.as_os_str().to_os_string());
        }
    }

    args
}

impl OxVbaExecutionTarget {
    fn display_name(&self) -> String {
        match self {
            Self::LooseFile(path) | Self::Project(path) => path.display().to_string(),
        }
    }
}

impl OxVbaExecutionResult {
    fn status_summary(&self) -> String {
        let action = match self.action {
            OxVbaExecutionAction::Build => "Build",
            OxVbaExecutionAction::Run => "Run",
        };

        if self.success {
            format!("{action} succeeded for {}.", self.target.display_name())
        } else if let Some(code) = self.exit_code {
            format!(
                "{action} failed for {} (exit {code}).",
                self.target.display_name()
            )
        } else {
            format!("{action} failed for {}.", self.target.display_name())
        }
    }
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
        DocumentSession, Msg, OxVbaExecutionAction, OxVbaExecutionRequest, OxVbaExecutionResult,
        OxVbaExecutionTarget, OxVbaServices, ProjectSession, ShellModel, is_command_key,
        is_help_key, is_quit_key, is_save_key, oxvba_cli_args_for_request, split_command,
        startup_path_from_args,
    };
    use ftui::prelude::{Cmd, Event, KeyCode, KeyEvent, Model, Modifiers};
    use oxvba_compiler::{ProjectKind, ProjectManifest};
    use std::cell::RefCell;
    use std::collections::BTreeMap;
    use std::env;
    use std::ffi::OsString;
    use std::fs;
    use std::io;
    use std::collections::VecDeque;
    use std::path::PathBuf;
    use std::rc::Rc;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[derive(Default)]
    struct FakeOxVbaServicesState {
        requests: RefCell<Vec<OxVbaExecutionRequest>>,
        results: RefCell<VecDeque<io::Result<OxVbaExecutionResult>>>,
    }

    struct FakeOxVbaServices {
        state: Rc<FakeOxVbaServicesState>,
    }

    impl FakeOxVbaServices {
        fn succeed(result: OxVbaExecutionResult) -> Self {
            Self {
                state: Rc::new(FakeOxVbaServicesState {
                    requests: RefCell::new(Vec::new()),
                    results: RefCell::new(VecDeque::from([Ok(result)])),
                }),
            }
        }

        fn queued(results: Vec<io::Result<OxVbaExecutionResult>>) -> (Self, Rc<FakeOxVbaServicesState>) {
            let state = Rc::new(FakeOxVbaServicesState {
                requests: RefCell::new(Vec::new()),
                results: RefCell::new(results.into()),
            });
            (
                Self {
                    state: Rc::clone(&state),
                },
                state,
            )
        }
    }

    fn unique_test_path(label: &str, extension: &str) -> Result<PathBuf, String> {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| error.to_string())?
            .as_nanos();
        Ok(env::temp_dir().join(format!("oxide-{label}-{nanos}.{extension}")))
    }

    fn write_test_file(path: &PathBuf, contents: &str) -> Result<(), String> {
        fs::write(path, contents).map_err(|error| error.to_string())
    }

    fn enter_and_dispatch_command(model: &mut ShellModel, command: &str) -> Cmd<Msg> {
        model.enter_command_mode();
        model.command_input.value = String::from(command);
        model.dispatch_command_line()
    }

    fn expect_none_cmd(cmd: Cmd<Msg>, context: &str) -> Result<(), String> {
        if !matches!(cmd, Cmd::None) {
            return Err(format!("{context} should not request a side effect"));
        }
        Ok(())
    }

    fn expect_project_request(
        request: &OxVbaExecutionRequest,
        expected_action: OxVbaExecutionAction,
        expected_path: &PathBuf,
    ) -> Result<(), String> {
        if request.action != expected_action {
            return Err(String::from("service request action did not match the command"));
        }

        match &request.target {
            OxVbaExecutionTarget::Project(path) if path == expected_path => Ok(()),
            _ => Err(String::from(
                "service request should target the saved project path",
            )),
        }
    }

    fn sample_module_text() -> &'static str {
        "Attribute VB_Name = \"Module1\"\n\
\n\
Option Explicit\n\
\n\
Public Sub Main()\n\
    Dim answer As Integer\n\
    answer = 40 + 2\n\
End Sub\n"
    }

    fn sample_project_text() -> &'static str {
        "<Project Sdk=\"OxVba.Sdk/0.1.0\">\n\
  <PropertyGroup>\n\
    <OutputType>Exe</OutputType>\n\
    <ProjectName>ThinSliceSmoke</ProjectName>\n\
    <EntryPoint>Module1.Main</EntryPoint>\n\
  </PropertyGroup>\n\
  <ItemGroup>\n\
    <Module Include=\"Module1.bas\" />\n\
  </ItemGroup>\n\
</Project>\n"
    }

    fn sample_manifest(name: &str) -> ProjectManifest {
        ProjectManifest {
            project_name: String::from(name),
            project_kind: ProjectKind::Source,
            modules: Vec::new(),
            references: Vec::new(),
            reference_projects: Vec::new(),
            conditional_constants: BTreeMap::new(),
        }
    }

    impl OxVbaServices for FakeOxVbaServices {
        fn execute(&self, request: &OxVbaExecutionRequest) -> io::Result<OxVbaExecutionResult> {
            self.state.requests.borrow_mut().push(request.clone());
            match self.state.results.borrow_mut().pop_front() {
                Some(result) => result,
                None => Err(io::Error::other("missing fake result")),
            }
        }
    }

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
    fn colon_enters_command_mode() -> Result<(), String> {
        let key = Event::Key(KeyEvent::new(KeyCode::Char(':')));

        if !is_command_key(&key) {
            return Err(String::from("colon should enter command mode"));
        }

        let mut model = ShellModel::new(None).map_err(|error| error.to_string())?;
        let cmd = model.update(Msg::Editor(key));

        if !model.command_input.active {
            return Err(String::from("colon should activate command mode"));
        }

        if !matches!(cmd, Cmd::None) {
            return Err(String::from(
                "entering command mode should not request a side effect",
            ));
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
        let mut document = DocumentSession {
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
    fn command_parser_preserves_path_arguments() -> Result<(), String> {
        let (command, argument) = split_command("open path/with spaces/file.bas");

        if command != "open" {
            return Err(String::from("command should be parsed"));
        }

        if argument != Some("path/with spaces/file.bas") {
            return Err(String::from("path argument should preserve spaces"));
        }

        Ok(())
    }

    #[test]
    fn execution_target_requires_saved_buffer() -> Result<(), String> {
        let document = DocumentSession {
            path: Some(PathBuf::from("sample.bas")),
            has_backing_file: true,
            saved_text: String::from("old"),
        };

        let error = document
            .execution_target("new")
            .err()
            .ok_or_else(|| String::from("dirty buffer should be rejected"))?;

        if !error.contains("Save") {
            return Err(String::from("dirty-buffer message should mention save"));
        }

        Ok(())
    }

    #[test]
    fn execution_target_distinguishes_project_files() -> Result<(), String> {
        let project = DocumentSession {
            path: Some(PathBuf::from("demo.basproj")),
            has_backing_file: true,
            saved_text: String::from("same"),
        };
        let loose = DocumentSession {
            path: Some(PathBuf::from("module.bas")),
            has_backing_file: true,
            saved_text: String::from("same"),
        };

        if !matches!(
            project.execution_target("same"),
            Ok(OxVbaExecutionTarget::Project(_))
        ) {
            return Err(String::from("basproj should map to project execution"));
        }

        if !matches!(
            loose.execution_target("same"),
            Ok(OxVbaExecutionTarget::LooseFile(_))
        ) {
            return Err(String::from(
                "non-basproj should map to loose-file execution",
            ));
        }

        Ok(())
    }

    #[test]
    fn execution_target_resolves_relative_paths_to_absolute() -> Result<(), String> {
        let document = DocumentSession {
            path: Some(PathBuf::from("module.bas")),
            has_backing_file: true,
            saved_text: String::from("same"),
        };

        let target = document.execution_target("same")?;

        match target {
            OxVbaExecutionTarget::LooseFile(path) => {
                if !path.is_absolute() {
                    return Err(String::from("execution target should be absolute"));
                }
            }
            _ => return Err(String::from("expected loose-file target")),
        }

        Ok(())
    }

    #[test]
    fn output_text_shows_placeholder_before_execution() -> Result<(), String> {
        let model = ShellModel::new(None).map_err(|error| error.to_string())?;

        if model.output_text() != "No OxVba output yet." {
            return Err(String::from(
                "placeholder output should be shown before execution",
            ));
        }

        Ok(())
    }

    #[test]
    fn oxvba_cli_args_match_action_and_target() -> Result<(), String> {
        let build_request = OxVbaExecutionRequest {
            action: OxVbaExecutionAction::Build,
            target: OxVbaExecutionTarget::LooseFile(PathBuf::from("module.bas")),
        };
        let run_request = OxVbaExecutionRequest {
            action: OxVbaExecutionAction::Run,
            target: OxVbaExecutionTarget::Project(PathBuf::from("demo.basproj")),
        };

        let build_args = oxvba_cli_args_for_request(&build_request);
        let run_args = oxvba_cli_args_for_request(&run_request);

        if build_args[5] != OsString::from("compile") {
            return Err(String::from("loose-file build should use compile"));
        }

        if run_args[5] != OsString::from("run-project") {
            return Err(String::from("project run should use run-project"));
        }

        Ok(())
    }

    #[test]
    fn open_command_loads_a_new_document() -> Result<(), String> {
        let path = env::current_dir()
            .map_err(|error| error.to_string())?
            .join("Cargo.toml");
        let mut model = ShellModel::new(None).map_err(|error| error.to_string())?;
        model.enter_command_mode();
        model.command_input.value = format!("open {}", path.display());

        let cmd = model.dispatch_command_line();

        if !model.editor.text().contains("name = \"ox-ide\"") {
            return Err(String::from("open command should load the requested file"));
        }

        if model.document_session.display_name() != path.display().to_string() {
            return Err(String::from(
                "document session should track the opened path",
            ));
        }

        if !matches!(cmd, Cmd::None) {
            return Err(String::from("open should not request a side effect"));
        }

        Ok(())
    }

    #[test]
    fn write_command_can_bind_and_save_a_new_path() -> Result<(), String> {
        let path = PathBuf::from(env::temp_dir()).join("oxide-bd-237-5-write-command.bas");
        let mut model = ShellModel::new(None).map_err(|error| error.to_string())?;
        model.update(Msg::Editor(Event::Key(KeyEvent::new(KeyCode::Char('a')))));
        model.enter_command_mode();
        model.command_input.value = format!("write {}", path.display());

        let cmd = model.dispatch_command_line();

        if !model.status.contains("Saved") {
            return Err(String::from("write command should save the file"));
        }

        if model.is_dirty() {
            return Err(String::from("write command should clear dirty state"));
        }

        if model.document_session.display_name() != path.display().to_string() {
            return Err(String::from("write command should bind the document path"));
        }

        if !matches!(cmd, Cmd::None) {
            return Err(String::from("write should not request a side effect"));
        }

        Ok(())
    }

    #[test]
    fn build_and_run_commands_route_without_execution() -> Result<(), String> {
        let path = PathBuf::from(env::temp_dir()).join("oxide-bd-237-6-run.bas");
        let run_result = OxVbaExecutionResult {
            action: OxVbaExecutionAction::Run,
            target: OxVbaExecutionTarget::LooseFile(path.clone()),
            success: true,
            exit_code: Some(0),
            stdout: String::from("ok"),
            stderr: String::new(),
        };
        let mut model =
            ShellModel::with_services(Some(path), Box::new(FakeOxVbaServices::succeed(run_result)))
                .map_err(|error| error.to_string())?;
        model.enter_command_mode();
        model.command_input.value = String::from("run");
        let cmd = model.dispatch_command_line();

        if !model.status.contains("Run succeeded") {
            return Err(String::from("run should report service success"));
        }

        if model.last_execution.is_none() {
            return Err(String::from(
                "run should store the structured execution result",
            ));
        }

        if !matches!(cmd, Cmd::None) {
            return Err(String::from("run should not request a side effect"));
        }

        Ok(())
    }

    #[test]
    fn output_text_renders_structured_execution_result() -> Result<(), String> {
        let path = PathBuf::from(env::temp_dir()).join("oxide-bd-237-13-run.bas");
        let run_result = OxVbaExecutionResult {
            action: OxVbaExecutionAction::Run,
            target: OxVbaExecutionTarget::LooseFile(path),
            success: false,
            exit_code: Some(2),
            stdout: String::from("line one"),
            stderr: String::from("line two"),
        };
        let mut model = ShellModel::new(None).map_err(|error| error.to_string())?;
        model.last_execution = Some(run_result);

        let output = model.output_text();

        if !output.contains("Action: Run") {
            return Err(String::from("output should include the action"));
        }

        if !output.contains("Exit code: 2") {
            return Err(String::from("output should include the exit code"));
        }

        if !output.contains("Stdout:\nline one") {
            return Err(String::from("output should include stdout"));
        }

        if !output.contains("Stderr:\nline two") {
            return Err(String::from("output should include stderr"));
        }

        Ok(())
    }

    #[test]
    fn build_command_blocks_unsaved_buffers_before_service_call() -> Result<(), String> {
        let path = PathBuf::from(env::temp_dir()).join("oxide-bd-237-6-dirty.bas");
        let result = OxVbaExecutionResult {
            action: OxVbaExecutionAction::Build,
            target: OxVbaExecutionTarget::LooseFile(path.clone()),
            success: true,
            exit_code: Some(0),
            stdout: String::new(),
            stderr: String::new(),
        };
        let mut model =
            ShellModel::with_services(Some(path), Box::new(FakeOxVbaServices::succeed(result)))
                .map_err(|error| error.to_string())?;
        model.update(Msg::Editor(Event::Key(KeyEvent::new(KeyCode::Char('a')))));
        model.enter_command_mode();
        model.command_input.value = String::from("build");
        model.dispatch_command_line();

        if !model.status.contains("Save the current buffer") {
            return Err(String::from("dirty build should require save first"));
        }

        if model.last_execution.is_some() {
            return Err(String::from("service should not run for a dirty buffer"));
        }

        Ok(())
    }

    #[test]
    fn quit_command_requests_quit() -> Result<(), String> {
        let mut model = ShellModel::new(None).map_err(|error| error.to_string())?;
        model.enter_command_mode();
        model.command_input.value = String::from("quit");

        let cmd = model.dispatch_command_line();

        if !matches!(cmd, Cmd::Quit) {
            return Err(String::from("quit command should request application quit"));
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

    #[test]
    fn startup_with_basproj_binds_project_session() -> Result<(), String> {
        let path = PathBuf::from("sample.basproj");
        let model = ShellModel::new(Some(path)).map_err(|error| error.to_string())?;

        if model.project_session.display_name() != "sample.basproj" {
            return Err(String::from("startup basproj should bind the project session"));
        }

        if !model.header_text().contains("Project: sample.basproj") {
            return Err(String::from("header should surface the bound project session"));
        }

        Ok(())
    }

    #[test]
    fn opening_non_project_document_clears_project_session() -> Result<(), String> {
        let basproj = PathBuf::from("sample.basproj");
        let module = env::current_dir()
            .map_err(|error| error.to_string())?
            .join("Cargo.toml");
        let mut model = ShellModel::new(Some(basproj)).map_err(|error| error.to_string())?;
        model.enter_command_mode();
        model.command_input.value = format!("open {}", module.display());

        let cmd = model.dispatch_command_line();

        if model.project_session.display_name() != "(no project)" {
            return Err(String::from("opening a non-project document should clear the project session"));
        }

        if !matches!(cmd, Cmd::None) {
            return Err(String::from("open should not request a side effect"));
        }

        Ok(())
    }

    #[test]
    fn project_session_can_store_manifest_for_future_project_work() -> Result<(), String> {
        let mut session = ProjectSession::default();
        let path = PathBuf::from("demo.basproj");
        session.bind_manifest(path.clone(), sample_manifest("DemoProject"));

        if session.display_name() != path.display().to_string() {
            return Err(String::from("project session should retain the project path"));
        }

        if !session.has_manifest() {
            return Err(String::from("project session should retain the manifest seam"));
        }

        session.clear();

        if session.has_manifest() || session.display_name() != "(no project)" {
            return Err(String::from("clearing the project session should drop project state"));
        }

        Ok(())
    }

    #[test]
    fn smoke_flow_covers_launch_edit_save_open_build_and_run() -> Result<(), String> {
        let module_path = unique_test_path("bd-237-8-module", "bas")?;
        let project_path = unique_test_path("bd-237-8-project", "basproj")?;

        write_test_file(&project_path, sample_project_text())?;

        let build_result = OxVbaExecutionResult {
            action: OxVbaExecutionAction::Build,
            target: OxVbaExecutionTarget::Project(project_path.clone()),
            success: true,
            exit_code: Some(0),
            stdout: String::from("built sample"),
            stderr: String::new(),
        };
        let run_result = OxVbaExecutionResult {
            action: OxVbaExecutionAction::Run,
            target: OxVbaExecutionTarget::Project(project_path.clone()),
            success: true,
            exit_code: Some(0),
            stdout: String::from("ran sample"),
            stderr: String::new(),
        };
        let (fake_services, state) =
            FakeOxVbaServices::queued(vec![Ok(build_result), Ok(run_result)]);
        let mut model = ShellModel::with_services(Some(module_path.clone()), Box::new(fake_services))
            .map_err(|error| error.to_string())?;

        if model.document_session.display_name() != module_path.display().to_string() {
            return Err(String::from("startup path should bind the launched document"));
        }

        for ch in sample_module_text().chars() {
            let cmd = model.update(Msg::Editor(Event::Key(KeyEvent::new(KeyCode::Char(ch)))));
            expect_none_cmd(cmd, "typing into the editor")?;
        }

        if !model.is_dirty() {
            return Err(String::from("editing should make the launch buffer dirty"));
        }

        expect_none_cmd(model.update(Msg::Save), "saving the launch buffer")?;

        if model.is_dirty() {
            return Err(String::from("save should clear the dirty state"));
        }

        let saved_module = fs::read_to_string(&module_path).map_err(|error| error.to_string())?;
        if saved_module != sample_module_text() {
            return Err(String::from("save should persist the edited module text"));
        }

        let open_cmd = format!("open {}", project_path.display());
        expect_none_cmd(enter_and_dispatch_command(&mut model, &open_cmd), "opening the project")?;

        if model.document_session.display_name() != project_path.display().to_string() {
            return Err(String::from("open should switch the active document to the project"));
        }

        expect_none_cmd(enter_and_dispatch_command(&mut model, "build"), "build command")?;
        if !model.status.contains("Build succeeded") {
            return Err(String::from("build should report service success"));
        }

        expect_none_cmd(enter_and_dispatch_command(&mut model, "run"), "run command")?;
        if !model.status.contains("Run succeeded") {
            return Err(String::from("run should report service success"));
        }

        let requests = state.requests.borrow();
        if requests.len() != 2 {
            return Err(String::from("smoke flow should issue one build and one run request"));
        }
        expect_project_request(
            &requests[0],
            OxVbaExecutionAction::Build,
            &project_path,
        )?;
        expect_project_request(
            &requests[1],
            OxVbaExecutionAction::Run,
            &project_path,
        )?;

        let output = model.output_text();
        if !output.contains("Action: Run") || !output.contains("Stdout:\nran sample") {
            return Err(String::from(
                "output pane should render the final structured run result",
            ));
        }

        Ok(())
    }
}
