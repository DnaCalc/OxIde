use std::cell::RefCell;
use std::collections::BTreeMap;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use ftui::layout::{Constraint, Flex, Rect};
use ftui::prelude::{App, Cmd, Event, Frame, KeyCode, KeyEvent, Model, Modifiers, ScreenMode};
use ftui::text::{CursorNavigator, Rope};
use ftui::widgets::block::{Alignment, Block};
use ftui::widgets::borders::Borders;
use ftui::widgets::paragraph::Paragraph;
use ftui::widgets::textarea::{TextArea, TextAreaState};
use ftui::widgets::{StatefulWidget, Widget};
use oxvba_compiler::{ModuleKind, ProjectKind, ProjectManifest};
use oxvba_languageservice::{
    CompletionItem, DiagnosticSeverity, DocumentId, DocumentSymbol, HostSessionError,
    HostWorkspaceSession, HoverInfo, SpannedDiagnostic,
};
use oxvba_languageservice::service::Position;
use oxvba_project::{
    ClassModuleMetadata, LoadedProject, NativeExportDescriptor, OutputType, RuntimeFlavor,
    generate_basproj_xml, load_basproj, load_basproj_from_str,
};

struct ShellModel {
    show_help: bool,
    command_input: CommandInput,
    project_session: ProjectSession,
    document_session: DocumentSession,
    output_mode: OutputMode,
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
    loaded_project: Option<LoadedProject>,
    host_session: Option<HostWorkspaceSession>,
    selected_module_index: Option<usize>,
    runtime_profile: Option<String>,
    policy_preset: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ProjectModuleEntry {
    module_name: String,
    path: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProjectRunDisposition {
    Executable,
    HostActivated,
    BuildOnly,
}

#[derive(Debug, Clone)]
struct LanguageStatus {
    document_id: Option<DocumentId>,
    diagnostics: Vec<SpannedDiagnostic>,
    symbols: Vec<DocumentSymbol>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputMode {
    Execution,
    Diagnostics,
    Symbols,
    Hover,
    Completions,
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
    runtime_profile: Option<String>,
    policy_preset: Option<String>,
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
    fn from_document_path(path: Option<&PathBuf>) -> io::Result<(Self, Option<String>)> {
        let Some(path) = path else {
            return Ok((Self::default(), None));
        };
        if !path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("basproj"))
        {
            return Ok((Self::default(), None));
        }

        Self::open_or_create(path)
    }

    fn open_or_create(path: &Path) -> io::Result<(Self, Option<String>)> {
        if path.exists() {
            let loaded_project = load_basproj(path).map_err(project_error_to_io)?;
            let text = canonical_project_xml(&loaded_project);
            Ok((
                Self {
                    project_path: Some(path.to_path_buf()),
                    loaded_project: Some(loaded_project),
                    host_session: Some(load_host_workspace_session(path)?),
                    selected_module_index: Some(0),
                    runtime_profile: None,
                    policy_preset: None,
                },
                Some(text),
            ))
        } else {
            let loaded_project = default_loaded_project_for_path(path);
            let text = canonical_project_xml(&loaded_project);
            Ok((
                Self {
                    project_path: Some(path.to_path_buf()),
                    loaded_project: Some(loaded_project),
                    host_session: None,
                    selected_module_index: Some(0),
                    runtime_profile: None,
                    policy_preset: None,
                },
                Some(text),
            ))
        }
    }

    fn sync_from_text(&mut self, project_text: &str, project_path: &Path) -> io::Result<()> {
        let project_dir = project_path.parent().unwrap_or_else(|| Path::new("."));
        let loaded_project =
            load_basproj_from_str(project_text, project_dir).map_err(project_error_to_io)?;
        let default_runtime_profile = loaded_project.default_runtime_profile.clone();
        let default_policy_preset = loaded_project.default_policy_preset.clone();
        self.project_path = Some(project_path.to_path_buf());
        self.loaded_project = Some(loaded_project);
        self.host_session = if project_path.exists() {
            Some(load_host_workspace_session(project_path)?)
        } else {
            None
        };
        self.selected_module_index = Some(0);
        self.runtime_profile = default_runtime_profile;
        self.policy_preset = default_policy_preset;
        Ok(())
    }

    fn save(&mut self, project_path: &Path, project_text: &str) -> io::Result<String> {
        self.sync_from_text(project_text, project_path)?;
        let canonical_xml = self
            .canonical_xml()
            .ok_or_else(|| io::Error::other("project session is missing a loaded project"))?;
        fs::write(project_path, &canonical_xml)?;
        self.host_session = Some(load_host_workspace_session(project_path)?);
        Ok(format!("Saved {}.", project_path.display()))
    }

    fn reload_from_disk(&mut self) -> io::Result<()> {
        let Some(project_path) = self.project_path.clone() else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "no project is active",
            ));
        };
        let selected_module_name = self.selected_module().map(|module| module.module_name);
        let loaded_project = load_basproj(&project_path).map_err(project_error_to_io)?;
        self.loaded_project = Some(loaded_project);
        self.host_session = Some(load_host_workspace_session(&project_path)?);
        self.selected_module_index = Some(0);
        if let Some(selected_module_name) = selected_module_name {
            let _ = self.select_module_by_name(&selected_module_name);
        }
        Ok(())
    }

    fn clear(&mut self) {
        self.project_path = None;
        self.loaded_project = None;
        self.host_session = None;
        self.selected_module_index = None;
        self.runtime_profile = None;
        self.policy_preset = None;
    }

    fn display_name(&self) -> String {
        match &self.project_path {
            Some(path) => path.display().to_string(),
            None => String::from("(no project)"),
        }
    }

    fn has_manifest(&self) -> bool {
        self.loaded_project.is_some()
    }

    fn manifest(&self) -> Option<&ProjectManifest> {
        self.loaded_project.as_ref().map(|loaded| &loaded.manifest)
    }

    fn canonical_xml(&self) -> Option<String> {
        self.loaded_project.as_ref().map(canonical_project_xml)
    }

    fn output_type(&self) -> Option<OutputType> {
        self.loaded_project.as_ref().map(|loaded| loaded.output_type)
    }

    fn run_disposition(&self) -> Option<ProjectRunDisposition> {
        self.output_type().map(|output_type| match output_type {
            OutputType::Exe | OutputType::ComExe => ProjectRunDisposition::Executable,
            OutputType::HostModule | OutputType::Addin => ProjectRunDisposition::HostActivated,
            OutputType::Library | OutputType::ComServer => ProjectRunDisposition::BuildOnly,
        })
    }

    fn build_artifact_label(&self) -> &'static str {
        match self.output_type() {
            Some(OutputType::HostModule) => "host-target bundle",
            Some(OutputType::Library) => "library bundle",
            Some(OutputType::Exe) => "executable bundle",
            Some(OutputType::Addin) => "addin bundle",
            Some(OutputType::ComServer) => "COM server bundle",
            Some(OutputType::ComExe) => "COM executable bundle",
            None => "bundle",
        }
    }

    fn run_action_label(&self) -> &'static str {
        match self.run_disposition() {
            Some(ProjectRunDisposition::Executable) => "run-project",
            Some(ProjectRunDisposition::HostActivated) => "host activation",
            Some(ProjectRunDisposition::BuildOnly) => "build only",
            None => "not available",
        }
    }

    fn output_type_label(&self) -> &'static str {
        match self.output_type() {
            Some(OutputType::HostModule) => "HostModule",
            Some(OutputType::Library) => "Library",
            Some(OutputType::Exe) => "Exe",
            Some(OutputType::Addin) => "Addin",
            Some(OutputType::ComServer) => "ComServer",
            Some(OutputType::ComExe) => "ComExe",
            None => "(none)",
        }
    }

    fn run_block_reason(&self) -> Option<String> {
        match self.run_disposition()? {
            ProjectRunDisposition::Executable => None,
            ProjectRunDisposition::HostActivated => Some(format!(
                "{} targets require host activation; OxIde does not launch host activation targets yet.",
                self.output_type_label()
            )),
            ProjectRunDisposition::BuildOnly => Some(format!(
                "{} targets are build-only in OxIde; use :build to produce the {}.",
                self.output_type_label(),
                self.build_artifact_label()
            )),
        }
    }

    fn selected_runtime_profile(&self) -> Option<&str> {
        self.runtime_profile.as_deref().or_else(|| {
            self.loaded_project
                .as_ref()
                .and_then(|loaded| loaded.default_runtime_profile.as_deref())
        })
    }

    fn selected_policy_preset(&self) -> Option<&str> {
        self.policy_preset.as_deref().or_else(|| {
            self.loaded_project
                .as_ref()
                .and_then(|loaded| loaded.default_policy_preset.as_deref())
        })
    }

    fn set_runtime_profile(&mut self, raw: &str) -> Result<(), String> {
        let value = raw.trim();
        if value.eq_ignore_ascii_case("default") {
            self.runtime_profile = self
                .loaded_project
                .as_ref()
                .and_then(|loaded| loaded.default_runtime_profile.clone());
            return Ok(());
        }
        if !is_known_runtime_profile(value) {
            return Err(format!("Unknown runtime profile: {value}"));
        }
        self.runtime_profile = Some(value.to_string());
        Ok(())
    }

    fn set_policy_preset(&mut self, raw: &str) -> Result<(), String> {
        let value = raw.trim();
        if value.eq_ignore_ascii_case("default") {
            self.policy_preset = self
                .loaded_project
                .as_ref()
                .and_then(|loaded| loaded.default_policy_preset.clone());
            return Ok(());
        }
        if !is_known_policy_preset(value) {
            return Err(format!("Unknown host policy preset: {value}"));
        }
        self.policy_preset = Some(value.to_string());
        Ok(())
    }

    fn module_entries(&self) -> Vec<ProjectModuleEntry> {
        let Some(project_path) = &self.project_path else {
            return Vec::new();
        };
        let Some(loaded_project) = &self.loaded_project else {
            return Vec::new();
        };
        let project_dir = project_path.parent().unwrap_or_else(|| Path::new("."));

        loaded_project
            .manifest
            .modules
            .iter()
            .map(|module| ProjectModuleEntry {
                module_name: module.module_name.clone(),
                path: project_dir.join(module_filename_for_kind(
                    &module.module_name,
                    module.module_kind,
                )),
            })
            .collect()
    }

    fn selected_module(&self) -> Option<ProjectModuleEntry> {
        let modules = self.module_entries();
        let index = self.selected_module_index?;
        modules.get(index).cloned()
    }

    fn select_module_by_name(&mut self, module_name: &str) -> Result<ProjectModuleEntry, String> {
        let modules = self.module_entries();
        let Some((index, module)) = modules.iter().enumerate().find(|(_, module)| {
            module.module_name.eq_ignore_ascii_case(module_name)
                || module.path.display().to_string().eq_ignore_ascii_case(module_name)
        }) else {
            return Err(format!("Unknown project module: {module_name}"));
        };
        self.selected_module_index = Some(index);
        Ok(module.clone())
    }

    fn select_adjacent_module(&mut self, step: isize) -> Result<ProjectModuleEntry, String> {
        let modules = self.module_entries();
        if modules.is_empty() {
            return Err(String::from("Project has no modules to navigate."));
        }

        let current = self.selected_module_index.unwrap_or(0) as isize;
        let len = modules.len() as isize;
        let next = (current + step).rem_euclid(len) as usize;
        self.selected_module_index = Some(next);
        Ok(modules[next].clone())
    }

    fn sync_selection_from_document_path(&mut self, path: &Path) {
        let modules = self.module_entries();
        if let Some((index, _)) = modules.iter().enumerate().find(|(_, module)| module.path == path)
        {
            self.selected_module_index = Some(index);
        }
    }

    fn sync_module_source_from_path(&mut self, path: &Path, source: &str) -> bool {
        let module_entries = self.module_entries();
        let Some((index, _)) = module_entries
            .iter()
            .enumerate()
            .find(|(_, module)| module.path == path)
        else {
            return false;
        };

        let Some(loaded_project) = self.loaded_project.as_mut() else {
            return false;
        };
        let Some(module) = loaded_project.manifest.modules.get_mut(index) else {
            return false;
        };

        module.source = source.to_string();
        true
    }

    fn workspace_document_count(&self) -> usize {
        self.host_session
            .as_ref()
            .map(|session| session.documents().len())
            .unwrap_or(0)
    }

    fn current_document_id(&self, current_document: &DocumentSession) -> Option<DocumentId> {
        let current_path = current_document.path()?;
        let project_path = self.project_path.as_ref()?;
        let loaded_project = self.loaded_project.as_ref()?;
        let project_dir = project_path.parent().unwrap_or_else(|| Path::new("."));

        loaded_project
            .manifest
            .modules
            .iter()
            .find(|module| {
                project_dir.join(module_filename_for_kind(&module.module_name, module.module_kind))
                    == *current_path
            })
            .map(|module| DocumentId::new(module.module_name.clone()))
    }

    fn restore_current_document_baseline(
        &mut self,
        current_document: &DocumentSession,
    ) -> Result<(), HostSessionError> {
        let Some(document_id) = self.current_document_id(current_document) else {
            return Ok(());
        };
        let Some(host_session) = self.host_session.as_mut() else {
            return Ok(());
        };
        host_session.close_document(&document_id)
    }

    fn sync_current_document_text(
        &mut self,
        current_document: &DocumentSession,
        current_text: &str,
    ) -> Result<Option<LanguageStatus>, HostSessionError> {
        let Some(document_id) = self.current_document_id(current_document) else {
            return Ok(None);
        };
        let Some(host_session) = self.host_session.as_mut() else {
            return Ok(None);
        };

        host_session.set_document_text(&document_id, current_text)?;
        let diagnostics = host_session.diagnostics(&document_id)?;
        let symbols = host_session.document_symbols(&document_id)?;
        Ok(Some(LanguageStatus {
            document_id: Some(document_id),
            diagnostics,
            symbols,
        }))
    }

    fn current_language_status(
        &self,
        current_document: &DocumentSession,
    ) -> Result<Option<LanguageStatus>, HostSessionError> {
        let Some(document_id) = self.current_document_id(current_document) else {
            return Ok(None);
        };
        let Some(host_session) = self.host_session.as_ref() else {
            return Ok(None);
        };
        let diagnostics = host_session.diagnostics(&document_id)?;
        let symbols = host_session.document_symbols(&document_id)?;
        Ok(Some(LanguageStatus {
            document_id: Some(document_id),
            diagnostics,
            symbols,
        }))
    }

    fn current_hover(
        &self,
        current_document: &DocumentSession,
        position: Position,
    ) -> Result<Option<(DocumentId, Option<HoverInfo>)>, HostSessionError> {
        let Some(document_id) = self.current_document_id(current_document) else {
            return Ok(None);
        };
        let Some(host_session) = self.host_session.as_ref() else {
            return Ok(None);
        };
        let hover = host_session.hover(&document_id, position)?;
        Ok(Some((document_id, hover)))
    }

    fn current_completions(
        &self,
        current_document: &DocumentSession,
        position: Position,
    ) -> Result<Option<(DocumentId, Vec<CompletionItem>)>, HostSessionError> {
        let Some(document_id) = self.current_document_id(current_document) else {
            return Ok(None);
        };
        let Some(host_session) = self.host_session.as_ref() else {
            return Ok(None);
        };
        let completions = host_session.completions(&document_id, position)?;
        Ok(Some((document_id, completions)))
    }

    fn surface_text(&self, current_document: &DocumentSession) -> String {
        let Some(loaded_project) = &self.loaded_project else {
            return String::from("No project session.\n\nOpen a .basproj to show project/workspace state.");
        };

        let mut lines = vec![
            format!("Path: {}", self.display_name()),
            format!("Kind: {:?}", loaded_project.manifest.project_kind),
            format!("Target: {}", self.output_type_label()),
            format!(
                "Runtime Profile: {}",
                self.selected_runtime_profile().unwrap_or("(none)")
            ),
            format!(
                "Host Policy: {}",
                self.selected_policy_preset().unwrap_or("(none)")
            ),
            format!("Build Output: {}", self.build_artifact_label()),
            format!("Run Surface: {}", self.run_action_label()),
            format!("Modules: {}", loaded_project.manifest.modules.len()),
            format!("References: {}", loaded_project.manifest.references.len()),
        ];

        if let Some(selected) = self.selected_module() {
            lines.push(format!("Selected: {}", selected.module_name));
        }

        lines.push(String::new());
        lines.push(String::from("Modules"));

        for (index, module) in self.module_entries().iter().enumerate() {
            let mut marker = if Some(index) == self.selected_module_index {
                ">"
            } else {
                " "
            };
            if current_document.path().is_some_and(|path| path == &module.path) {
                marker = "*";
            }
            lines.push(format!("{marker} {}", module.module_name));
        }

        if self.module_entries().is_empty() {
            lines.push(String::from("(none)"));
        }

        lines.push(String::new());
        lines.push(String::from(
            "Commands: :module <name>  :module-next  :module-prev",
        ));
        lines.join("\n")
    }

    fn language_surface_text(&self, current_document: &DocumentSession) -> String {
        let mut lines = vec![format!("Workspace Docs: {}", self.workspace_document_count())];

        if let Some(host_session) = &self.host_session {
            lines.push(format!(
                "Workspace Target: {}",
                host_session.workspace_target().display()
            ));
        }

        match self.current_language_status(current_document) {
            Ok(Some(status)) => {
                let error_count = status
                    .diagnostics
                    .iter()
                    .filter(|diag| diag.severity == DiagnosticSeverity::Error)
                    .count();
                let warning_count = status
                    .diagnostics
                    .iter()
                    .filter(|diag| diag.severity == DiagnosticSeverity::Warning)
                    .count();

                lines.push(format!(
                    "Current Doc: {}",
                    status
                        .document_id
                        .as_ref()
                        .map(|id| id.0.as_str())
                        .unwrap_or("none")
                ));
                lines.push(format!("Document Symbols: {}", status.symbols.len()));
                lines.push(format!(
                    "Diagnostics: {} error(s), {} warning(s)",
                    error_count, warning_count
                ));

                if let Some(first_symbol) = status.symbols.first() {
                    lines.push(format!("First Symbol: {}", first_symbol.name));
                }

                if let Some(first) = status.diagnostics.first() {
                    lines.push(String::new());
                    lines.push(format!(
                        "First Diagnostic: {} [{}..{}]",
                        first.message, first.span.start, first.span.end
                    ));
                }
            }
            Ok(None) => {
                lines.push(String::from("Current Doc: none"));
            }
            Err(error) => {
                lines.push(format!("Language Session Error: {error}"));
            }
        }

        lines.join("\n")
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
        let (mut document_session, mut status) = DocumentSession::load(path)?;
        let (project_session, project_text) =
            ProjectSession::from_document_path(document_session.path())?;
        if let Some(project_text) = project_text {
            document_session.saved_text = project_text.clone();
            status = match document_session.path() {
                Some(path) if path.exists() => format!("Opened project {}.", path.display()),
                Some(path) => format!("New project {}.", path.display()),
                None => status,
            };
        }
        let editor = new_editor(document_session.saved_text());

        Ok(Self {
            show_help: true,
            command_input: CommandInput::default(),
            project_session,
            document_session,
            output_mode: OutputMode::Diagnostics,
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
         :project-new <path>\n\
         :module <name>\n\
         :module-next\n\
         :module-prev\n\
         :revert\n\
         :workspace-reload\n\
         :diagnostics\n\
         :symbols\n\
         :hover\n\
         :complete\n\
         :profile <id|default>\n\
         :policy <preset|default>\n\
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
            String::from(
                ": command mode  |  :open <path>  :write [path]  :project-new <path>  :module <name>  :module-next  :module-prev  :revert  :workspace-reload  :diagnostics  :symbols  :hover  :complete  :profile <id>  :policy <preset>  :build  :run  :quit",
            )
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
        match self.output_mode {
            OutputMode::Execution => "OxVba Output",
            OutputMode::Diagnostics => "Diagnostics",
            OutputMode::Symbols => "Symbols",
            OutputMode::Hover => "Hover",
            OutputMode::Completions => "Completions",
        }
    }

    fn output_text(&self) -> String {
        match self.output_mode {
            OutputMode::Execution => self.execution_output_text(),
            OutputMode::Diagnostics => self.diagnostics_output_text(),
            OutputMode::Symbols => self.symbols_output_text(),
            OutputMode::Hover => self.hover_output_text(),
            OutputMode::Completions => self.completions_output_text(),
        }
    }

    fn buffer_title(&self) -> String {
        format!(
            "Buffer  {}{}  |  LS docs {}",
            self.document_session.display_name(),
            self.dirty_marker(),
            self.project_session.workspace_document_count()
        )
    }

    fn dirty_marker(&self) -> &'static str {
        if self.is_dirty() { " *" } else { "" }
    }

    fn is_dirty(&self) -> bool {
        self.document_session.is_dirty(&self.editor.text())
    }

    fn sync_language_session(&mut self, current_text: &str) {
        if let Err(error) = self
            .project_session
            .sync_current_document_text(&self.document_session, current_text)
        {
            self.status = format!("Language session update failed: {error}");
        }
    }

    fn restore_current_language_document(&mut self) -> bool {
        match self
            .project_session
            .restore_current_document_baseline(&self.document_session)
        {
            Ok(()) => true,
            Err(error) => {
                self.status = format!("Language session restore failed: {error}");
                false
            }
        }
    }

    fn current_language_status(&self) -> Result<Option<LanguageStatus>, HostSessionError> {
        self.project_session
            .current_language_status(&self.document_session)
    }

    fn current_language_summary(&self) -> Option<String> {
        match self.current_language_status() {
            Ok(Some(status)) => Some(format_language_summary(&status)),
            Ok(None) => None,
            Err(error) => Some(format!("Language session error: {error}")),
        }
    }

    fn current_language_position(&self) -> Option<Position> {
        let text = self.editor.text();
        let rope = Rope::from_text(&text);
        let navigator = CursorNavigator::new(&rope);
        let byte_index = navigator.to_byte_index(self.editor.cursor());
        u32::try_from(byte_index).ok()
    }

    fn execution_output_text(&self) -> String {
        let Some(result) = &self.last_execution else {
            return String::from(
                "No OxVba execution output yet.\n\nUse :diagnostics or :symbols for live language-service views.",
            );
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

    fn diagnostics_output_text(&self) -> String {
        match self.current_language_status() {
            Ok(Some(status)) => format_diagnostics_output(&status),
            Ok(None) => String::from(
                "No active project-backed language document.\n\nOpen a project module to show diagnostics.",
            ),
            Err(error) => format!("Language session error: {error}"),
        }
    }

    fn symbols_output_text(&self) -> String {
        match self.current_language_status() {
            Ok(Some(status)) => format_symbols_output(&status),
            Ok(None) => {
                String::from("No active project-backed language document.\n\nOpen a project module to show symbols.")
            }
            Err(error) => format!("Language session error: {error}"),
        }
    }

    fn hover_output_text(&self) -> String {
        let Some(position) = self.current_language_position() else {
            return String::from("Current cursor position is unavailable.");
        };

        match self
            .project_session
            .current_hover(&self.document_session, position)
        {
            Ok(Some((document_id, hover))) => format_hover_output(&document_id, position, hover),
            Ok(None) => String::from(
                "No active project-backed language document.\n\nOpen a project module to show hover details.",
            ),
            Err(error) => format!("Language session error: {error}"),
        }
    }

    fn completions_output_text(&self) -> String {
        let Some(position) = self.current_language_position() else {
            return String::from("Current cursor position is unavailable.");
        };

        match self
            .project_session
            .current_completions(&self.document_session, position)
        {
            Ok(Some((document_id, completions))) => {
                format_completions_output(&document_id, position, &completions)
            }
            Ok(None) => String::from(
                "No active project-backed language document.\n\nOpen a project module to show completions.",
            ),
            Err(error) => format!("Language session error: {error}"),
        }
    }

    fn show_diagnostics(&mut self) -> Cmd<Msg> {
        self.output_mode = OutputMode::Diagnostics;
        self.status = String::from("Diagnostics view opened.");
        Cmd::none()
    }

    fn show_symbols(&mut self) -> Cmd<Msg> {
        self.output_mode = OutputMode::Symbols;
        self.status = String::from("Symbols view opened.");
        Cmd::none()
    }

    fn show_hover(&mut self) -> Cmd<Msg> {
        self.output_mode = OutputMode::Hover;
        self.status = String::from("Hover view opened.");
        Cmd::none()
    }

    fn show_completions(&mut self) -> Cmd<Msg> {
        self.output_mode = OutputMode::Completions;
        self.status = String::from("Completions view opened.");
        Cmd::none()
    }

    fn revert_current_document(&mut self) -> Cmd<Msg> {
        let Some(path) = self.document_session.path().cloned() else {
            self.status = String::from("Revert requires a bound file path.");
            return Cmd::none();
        };
        self.open_document(path)
    }

    fn reload_workspace(&mut self) -> Cmd<Msg> {
        if self.is_dirty() {
            self.status = String::from("Save or revert the current buffer before reloading the workspace.");
            return Cmd::none();
        }

        if is_basproj_path(self.document_session.path().map_or_else(|| Path::new(""), |p| p)) {
            if let Some(path) = self.document_session.path().cloned() {
                return self.open_document(path);
            }
        }

        let Some(current_path) = self.document_session.path().cloned() else {
            self.status = String::from("Workspace reload requires a bound project or file path.");
            return Cmd::none();
        };

        if !self.project_session.has_manifest() {
            return self.open_document(current_path);
        }

        if let Err(error) = self.project_session.reload_from_disk() {
            self.status = format!("Workspace reload failed: {error}");
            return Cmd::none();
        }

        match DocumentSession::open(current_path.clone()) {
            Ok((document_session, _)) => {
                self.project_session.sync_selection_from_document_path(&current_path);
                self.document_session = document_session;
                self.editor = new_editor(self.document_session.saved_text());
                self.editor_state = RefCell::new(TextAreaState::default());
                let current_text = self.document_session.saved_text().to_string();
                self.sync_language_session(&current_text);
                self.status = format!("Reloaded workspace for {}.", self.project_session.display_name());
            }
            Err(error) => {
                self.status = format!("Workspace reload failed: {error}");
            }
        }

        Cmd::none()
    }

    fn save_current_file(&mut self) {
        let current_text = self.editor.text();
        self.status = match self.save_document_to_current_path(&current_text) {
            Ok(status) => status,
            Err(error) if error.kind() == io::ErrorKind::InvalidInput => {
                String::from("No file path yet. Start OxIde with a file path for save support.")
            }
            Err(error) => format!("Save failed: {error}"),
        };
    }

    fn save_current_file_as(&mut self, path: PathBuf) {
        let current_text = self.editor.text();
        self.status = match self.save_document_to_path(path, &current_text) {
            Ok(status) => status,
            Err(error) => format!("Save failed: {error}"),
        };
    }

    fn save_document_to_current_path(&mut self, current_text: &str) -> io::Result<String> {
        match self.document_session.path() {
            Some(path) => self.save_document_to_path(path.clone(), current_text),
            None => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "no file path is associated with this buffer",
            )),
        }
    }

    fn save_document_to_path(&mut self, path: PathBuf, current_text: &str) -> io::Result<String> {
        if is_basproj_path(&path) {
            let status = self.project_session.save(&path, current_text)?;
            let canonical_xml = self
                .project_session
                .canonical_xml()
                .ok_or_else(|| io::Error::other("project save did not produce project text"))?;
            self.document_session.path = Some(path);
            self.document_session.has_backing_file = true;
            self.document_session.saved_text = canonical_xml.clone();
            self.editor = new_editor(&canonical_xml);
            self.editor_state = RefCell::new(TextAreaState::default());
            self.sync_language_session(&canonical_xml);
            Ok(status)
        } else {
            let status = self.document_session.save_as(path, current_text)?;
            if let Some(saved_path) = self.document_session.path() {
                self.project_session
                    .sync_module_source_from_path(saved_path, current_text);
            }
            self.sync_language_session(current_text);
            Ok(status)
        }
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
            "project-new" => match arg {
                Some(path_text) => self.open_document(PathBuf::from(path_text)),
                None => {
                    self.status = String::from("Usage: :project-new <path>");
                    Cmd::none()
                }
            },
            "module" => match arg {
                Some(module_name) => self.open_named_project_module(module_name),
                None => {
                    self.status = String::from("Usage: :module <name>");
                    Cmd::none()
                }
            },
            "module-next" => self.cycle_project_module(1),
            "module-prev" => self.cycle_project_module(-1),
            "revert" => self.revert_current_document(),
            "workspace-reload" => self.reload_workspace(),
            "diagnostics" => self.show_diagnostics(),
            "symbols" => self.show_symbols(),
            "hover" => self.show_hover(),
            "complete" => self.show_completions(),
            "profile" => match arg {
                Some(profile) => {
                    self.status = match self.project_session.set_runtime_profile(profile) {
                        Ok(()) => format!(
                            "Runtime profile set to {}.",
                            self.project_session
                                .selected_runtime_profile()
                                .unwrap_or("(none)")
                        ),
                        Err(message) => message,
                    };
                    Cmd::none()
                }
                None => {
                    self.status = String::from("Usage: :profile <id|default>");
                    Cmd::none()
                }
            },
            "policy" => match arg {
                Some(policy) => {
                    self.status = match self.project_session.set_policy_preset(policy) {
                        Ok(()) => format!(
                            "Host policy preset set to {}.",
                            self.project_session.selected_policy_preset().unwrap_or("(none)")
                        ),
                        Err(message) => message,
                    };
                    Cmd::none()
                }
                None => {
                    self.status = String::from("Usage: :policy <preset|default>");
                    Cmd::none()
                }
            },
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
        if !self.restore_current_language_document() {
            return Cmd::none();
        }

        match DocumentSession::open(path) {
            Ok((mut document_session, mut status)) => {
                let (project_session, project_text) =
                    match ProjectSession::from_document_path(document_session.path()) {
                        Ok(result) => result,
                        Err(error) => {
                            self.status = format!("Open failed: {error}");
                            return Cmd::none();
                        }
                    };
                if let Some(project_text) = project_text {
                    document_session.saved_text = project_text.clone();
                    status = match document_session.path() {
                        Some(path) if path.exists() => format!("Opened project {}.", path.display()),
                        Some(path) => format!("New project {}.", path.display()),
                        None => status,
                    };
                }
                self.document_session = document_session;
                self.project_session = project_session;
                self.editor = new_editor(self.document_session.saved_text());
                self.editor_state = RefCell::new(TextAreaState::default());
                let current_text = self.document_session.saved_text().to_string();
                self.sync_language_session(&current_text);
                self.status = status;
            }
            Err(error) => {
                self.status = format!("Open failed: {error}");
            }
        }

        Cmd::none()
    }

    fn open_selected_project_module(&mut self, module: ProjectModuleEntry) -> Cmd<Msg> {
        if !self.restore_current_language_document() {
            return Cmd::none();
        }

        match DocumentSession::open(module.path.clone()) {
            Ok((document_session, _status)) => {
                self.project_session.sync_selection_from_document_path(&module.path);
                self.document_session = document_session;
                self.editor = new_editor(self.document_session.saved_text());
                self.editor_state = RefCell::new(TextAreaState::default());
                let current_text = self.document_session.saved_text().to_string();
                self.sync_language_session(&current_text);
                self.status = format!("Opened project module {}.", module.module_name);
            }
            Err(error) => {
                self.status = format!("Open failed: {error}");
            }
        }

        Cmd::none()
    }

    fn open_named_project_module(&mut self, module_name: &str) -> Cmd<Msg> {
        match self.project_session.select_module_by_name(module_name) {
            Ok(module) => self.open_selected_project_module(module),
            Err(message) => {
                self.status = message;
                Cmd::none()
            }
        }
    }

    fn cycle_project_module(&mut self, step: isize) -> Cmd<Msg> {
        match self.project_session.select_adjacent_module(step) {
            Ok(module) => self.open_selected_project_module(module),
            Err(message) => {
                self.status = message;
                Cmd::none()
            }
        }
    }

    fn execute_oxvba(&mut self, action: OxVbaExecutionAction) -> Cmd<Msg> {
        if action == OxVbaExecutionAction::Run
            && let Some(message) = self.project_session.run_block_reason()
        {
            self.status = message;
            self.last_execution = None;
            return Cmd::none();
        }

        let current_text = self.editor.text();
        let target = match self.document_session.execution_target(&current_text) {
            Ok(target) => target,
            Err(message) => {
                self.status = message;
                return Cmd::none();
            }
        };

        let request = OxVbaExecutionRequest {
            action,
            target,
            runtime_profile: self.project_session.selected_runtime_profile().map(str::to_string),
            policy_preset: self.project_session.selected_policy_preset().map(str::to_string),
        };

        match self.oxvba_services.execute(&request) {
            Ok(result) => {
                self.status = result.status_summary();
                self.output_mode = OutputMode::Execution;
                self.last_execution = Some(result);
            }
            Err(error) => {
                self.status = format!("OxVbaServices invocation failed: {error}");
                self.output_mode = OutputMode::Execution;
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
                    let current_text = self.editor.text();
                    self.sync_language_session(&current_text);
                    self.status = if self.is_dirty() {
                        self.current_language_summary()
                            .map(|summary| format!("Buffer modified. {summary}"))
                            .unwrap_or_else(|| String::from("Buffer modified."))
                    } else {
                        self.current_language_summary()
                            .map(|summary| format!("Buffer matches saved file. {summary}"))
                            .unwrap_or_else(|| String::from("Buffer matches saved file."))
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

        let has_project_surface = self.project_session.has_manifest();
        let show_side_panel = has_project_surface || self.show_help;
        let main_and_side_sections = if show_side_panel {
            Flex::horizontal()
                .constraints([Constraint::Percentage(72.0), Constraint::Fill])
                .split(sections[1])
        } else {
            vec![sections[1]]
        };
        let main_area = main_and_side_sections[0];

        let main_sections = Flex::vertical()
            .constraints([Constraint::Percentage(68.0), Constraint::Fill])
            .split(main_area);

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

        if show_side_panel {
            let side_area = main_and_side_sections[1];
            let side_sections = if has_project_surface && self.show_help {
                Flex::vertical()
                    .constraints([Constraint::Percentage(58.0), Constraint::Fill])
                    .split(side_area)
            } else {
                vec![side_area]
            };

            if has_project_surface {
                Paragraph::new(format!(
                    "{}\n\nLanguage\n{}",
                    self.project_session.surface_text(&self.document_session),
                    self.project_session
                        .language_surface_text(&self.document_session)
                ))
                    .block(
                        Block::new()
                            .borders(Borders::ALL)
                            .title("Project")
                            .title_alignment(Alignment::Center),
                    )
                    .render(side_sections[0], frame);
            }

            if self.show_help {
                let help_area = if has_project_surface {
                    side_sections[1]
                } else {
                    side_sections[0]
                };
                Paragraph::new(self.help_text())
                    .block(
                        Block::new()
                            .borders(Borders::ALL)
                            .title("Help")
                            .title_alignment(Alignment::Center),
                    )
                    .render(help_area, frame);
            }
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

fn is_basproj_path(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("basproj"))
}

fn load_host_workspace_session(path: &Path) -> io::Result<HostWorkspaceSession> {
    HostWorkspaceSession::load_workspace_path(path)
        .map_err(|error| io::Error::other(error.to_string()))
}

fn format_language_summary(status: &LanguageStatus) -> String {
    let error_count = status
        .diagnostics
        .iter()
        .filter(|diag| diag.severity == DiagnosticSeverity::Error)
        .count();
    let warning_count = status
        .diagnostics
        .iter()
        .filter(|diag| diag.severity == DiagnosticSeverity::Warning)
        .count();
    format!(
        "Diagnostics: {} error(s), {} warning(s); symbols: {}.",
        error_count,
        warning_count,
        status.symbols.len()
    )
}

fn format_diagnostics_output(status: &LanguageStatus) -> String {
    let mut lines = vec![format!(
        "Document: {}",
        status
            .document_id
            .as_ref()
            .map(|id| id.0.as_str())
            .unwrap_or("none")
    )];

    if status.diagnostics.is_empty() {
        lines.push(String::from("No diagnostics."));
        return lines.join("\n");
    }

    let error_count = status
        .diagnostics
        .iter()
        .filter(|diag| diag.severity == DiagnosticSeverity::Error)
        .count();
    let warning_count = status
        .diagnostics
        .iter()
        .filter(|diag| diag.severity == DiagnosticSeverity::Warning)
        .count();
    lines.push(format!(
        "Summary: {} error(s), {} warning(s)",
        error_count, warning_count
    ));
    lines.push(String::new());

    for diagnostic in &status.diagnostics {
        lines.push(format!(
            "- {} [{}..{}] {}",
            diagnostic_severity_label(diagnostic.severity),
            diagnostic.span.start,
            diagnostic.span.end,
            diagnostic.message
        ));
    }

    lines.join("\n")
}

fn format_symbols_output(status: &LanguageStatus) -> String {
    let mut lines = vec![format!(
        "Document: {}",
        status
            .document_id
            .as_ref()
            .map(|id| id.0.as_str())
            .unwrap_or("none")
    )];

    if status.symbols.is_empty() {
        lines.push(String::from("No document symbols."));
        return lines.join("\n");
    }

    lines.push(format!("Symbol Count: {}", status.symbols.len()));
    lines.push(String::new());
    for symbol in &status.symbols {
        lines.push(format!(
            "- {} {:?} [{}..{}]",
            symbol.name, symbol.kind, symbol.span.start, symbol.span.end
        ));
    }
    lines.join("\n")
}

fn format_hover_output(document_id: &DocumentId, position: Position, hover: Option<HoverInfo>) -> String {
    let mut lines = vec![
        format!("Document: {}", document_id.0),
        format!("Cursor Byte Offset: {position}"),
    ];

    let Some(hover) = hover else {
        lines.push(String::from("No hover information at the current cursor."));
        return lines.join("\n");
    };

    lines.push(format!("Label: {}", hover.label));
    if let Some(detail) = hover.detail {
        lines.push(format!("Detail: {detail}"));
    }
    if let Some(provenance) = hover.provenance {
        lines.push(format!(
            "Provenance: {} ({:?})",
            provenance.project_name.as_deref().unwrap_or("workspace"),
            provenance.kind
        ));
    }
    if let Some(symbol_identity) = hover.symbol_identity {
        lines.push(format!(
            "Symbol: {}::{}/{}",
            symbol_identity.document_id, symbol_identity.normalized_name, symbol_identity.scope
        ));
    }

    lines.join("\n")
}

fn format_completions_output(
    document_id: &DocumentId,
    position: Position,
    completions: &[CompletionItem],
) -> String {
    let mut lines = vec![
        format!("Document: {}", document_id.0),
        format!("Cursor Byte Offset: {position}"),
    ];

    if completions.is_empty() {
        lines.push(String::from("No completions at the current cursor."));
        return lines.join("\n");
    }

    lines.push(format!("Completion Count: {}", completions.len()));
    lines.push(String::new());
    for item in completions.iter().take(24) {
        let mut line = format!("- {} {:?}", item.label, item.kind);
        if let Some(detail) = &item.detail {
            line.push_str(&format!(" :: {detail}"));
        }
        if let Some(source_document) = &item.source_document {
            line.push_str(&format!(" @ {}", source_document.0));
        }
        lines.push(line);
    }
    if completions.len() > 24 {
        lines.push(format!("... {} more", completions.len() - 24));
    }

    lines.join("\n")
}

fn diagnostic_severity_label(severity: DiagnosticSeverity) -> &'static str {
    match severity {
        DiagnosticSeverity::Error => "error",
        DiagnosticSeverity::Warning => "warning",
    }
}

fn is_known_runtime_profile(raw: &str) -> bool {
    matches!(
        raw.trim().to_ascii_lowercase().as_str(),
        "windows-gui"
            | "windows-headless"
            | "linux-stdio"
            | "wasm-wasi-local"
            | "wasm-browser-sandbox"
            | "null-floor"
            | "macos-headless"
    )
}

fn is_known_policy_preset(raw: &str) -> bool {
    matches!(
        raw.trim().to_ascii_lowercase().as_str(),
        "strict-ci"
            | "deterministic-runtime"
            | "deterministic-compile-time"
            | "interactive-dev"
    )
}

fn project_error_to_io(error: oxvba_project::BasProjError) -> io::Error {
    io::Error::other(error.to_string())
}

fn canonical_project_xml(loaded_project: &LoadedProject) -> String {
    generate_basproj_xml(
        &loaded_project.manifest,
        loaded_project.output_type,
        loaded_project.entry_point.as_deref(),
        Some(loaded_project.runtime_flavor),
        loaded_project.default_runtime_profile.as_deref(),
        loaded_project.default_policy_preset.as_deref(),
        Some(&loaded_project.default_root_object),
        &loaded_project.type_library_catalog,
        &loaded_project.native_exports,
        &loaded_project.class_module_metadata,
    )
}

fn module_filename_for_kind(module_name: &str, module_kind: ModuleKind) -> String {
    let extension = match module_kind {
        ModuleKind::Procedural => "bas",
        ModuleKind::Class | ModuleKind::Document | ModuleKind::Form | ModuleKind::Extension => {
            "cls"
        }
    };
    format!("{module_name}.{extension}")
}

fn default_loaded_project_for_path(path: &Path) -> LoadedProject {
    let project_name = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .filter(|stem| !stem.is_empty())
        .unwrap_or("NewProject")
        .to_string();

    LoadedProject {
        manifest: ProjectManifest {
            project_name: project_name.clone(),
            project_kind: ProjectKind::Source,
            modules: Vec::new(),
            references: Vec::new(),
            reference_projects: Vec::new(),
            conditional_constants: BTreeMap::new(),
        },
        native_exports: Vec::<NativeExportDescriptor>::new(),
        output_type: OutputType::Exe,
        runtime_flavor: RuntimeFlavor::Lite,
        default_runtime_profile: Some(String::from("windows-headless")),
        default_policy_preset: Some(String::from("deterministic-runtime")),
        default_root_object: String::from("Application"),
        entry_point: None,
        type_library_catalog: Vec::new(),
        class_module_metadata: BTreeMap::<String, ClassModuleMetadata>::new(),
    }
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
            if let Some(profile) = &request.runtime_profile {
                args.push(OsString::from("--profile"));
                args.push(OsString::from(profile));
            }
            if let Some(policy) = &request.policy_preset {
                args.push(OsString::from("--policy"));
                args.push(OsString::from(policy));
            }
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
    use ftui::text::{CursorNavigator, Rope};
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::env;
    use std::ffi::OsString;
    use std::fs;
    use std::io;
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

    fn unique_test_dir(label: &str) -> Result<PathBuf, String> {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| error.to_string())?
            .as_nanos();
        let path = env::temp_dir().join(format!("oxide-{label}-{nanos}"));
        fs::create_dir_all(&path).map_err(|error| error.to_string())?;
        Ok(path)
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

    fn move_cursor_to_first(model: &mut ShellModel, needle: &str) -> Result<(), String> {
        let text = model.editor.text();
        let byte_index = text
            .find(needle)
            .ok_or_else(|| format!("expected to find `{needle}` in editor text"))?;
        let rope = Rope::from_text(&text);
        let navigator = CursorNavigator::new(&rope);
        model
            .editor
            .set_cursor_position(navigator.from_byte_index(byte_index));
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

    fn sample_workspace_project_text() -> &'static str {
        "<Project Sdk=\"OxVba.Sdk/0.1.0\">\n\
  <PropertyGroup>\n\
    <OutputType>Exe</OutputType>\n\
    <ProjectName>WorkspaceSurface</ProjectName>\n\
    <EntryPoint>Module1.Main</EntryPoint>\n\
  </PropertyGroup>\n\
  <ItemGroup>\n\
    <Module Include=\"Module1.bas\" />\n\
    <Module Include=\"Module2.bas\" />\n\
  </ItemGroup>\n\
</Project>\n"
    }

    fn sample_project_text_for_output_type(output_type: &str) -> String {
        format!(
            "<Project Sdk=\"OxVba.Sdk/0.1.0\">\n\
  <PropertyGroup>\n\
    <OutputType>{output_type}</OutputType>\n\
    <ProjectName>TargetSurface</ProjectName>\n\
    <EntryPoint>Module1.Main</EntryPoint>\n\
  </PropertyGroup>\n\
  <ItemGroup>\n\
    <Module Include=\"Module1.bas\" />\n\
  </ItemGroup>\n\
</Project>\n"
        )
    }

    fn sample_second_module_text() -> &'static str {
        "Attribute VB_Name = \"Module2\"\n\
\n\
Option Explicit\n\
\n\
Public Sub Helper()\n\
End Sub\n"
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

        if !model
            .output_text()
            .contains("No active project-backed language document")
        {
            return Err(String::from(
                "diagnostics view should be the default placeholder before execution",
            ));
        }

        Ok(())
    }

    #[test]
    fn diagnostics_command_switches_output_to_language_diagnostics() -> Result<(), String> {
        let workspace_dir = unique_test_dir("bd-oxide-diagnostics-view")?;
        let project_path = workspace_dir.join("WorkspaceSurface.basproj");
        write_test_file(&workspace_dir.join("Module1.bas"), sample_module_text())?;
        write_test_file(&workspace_dir.join("Module2.bas"), sample_second_module_text())?;
        write_test_file(&project_path, sample_workspace_project_text())?;

        let mut model = ShellModel::new(Some(project_path)).map_err(|error| error.to_string())?;
        expect_none_cmd(
            enter_and_dispatch_command(&mut model, "module Module1"),
            "opening project module for diagnostics",
        )?;

        for ch in "\nSub Broken(\n".chars() {
            expect_none_cmd(
                model.update(Msg::Editor(Event::Key(KeyEvent::new(KeyCode::Char(ch))))),
                "typing invalid text for diagnostics output",
            )?;
        }

        expect_none_cmd(
            enter_and_dispatch_command(&mut model, "diagnostics"),
            "opening diagnostics output",
        )?;

        let output = model.output_text();
        if !output.contains("Summary:") || !output.contains("error") {
            return Err(String::from(
                "diagnostics view should render live language-service diagnostics",
            ));
        }

        Ok(())
    }

    #[test]
    fn symbols_command_switches_output_to_document_symbols() -> Result<(), String> {
        let workspace_dir = unique_test_dir("bd-oxide-symbols-view")?;
        let project_path = workspace_dir.join("WorkspaceSurface.basproj");
        write_test_file(&workspace_dir.join("Module1.bas"), sample_module_text())?;
        write_test_file(&workspace_dir.join("Module2.bas"), sample_second_module_text())?;
        write_test_file(&project_path, sample_workspace_project_text())?;

        let mut model = ShellModel::new(Some(project_path)).map_err(|error| error.to_string())?;
        expect_none_cmd(
            enter_and_dispatch_command(&mut model, "module Module1"),
            "opening project module for symbols",
        )?;
        expect_none_cmd(
            enter_and_dispatch_command(&mut model, "symbols"),
            "opening symbols output",
        )?;

        let output = model.output_text();
        if !output.contains("Symbol Count:") || !output.contains("Main") {
            return Err(String::from(
                "symbols view should render document symbols from the direct host session",
            ));
        }

        Ok(())
    }

    #[test]
    fn hover_command_switches_output_to_cursor_hover() -> Result<(), String> {
        let workspace_dir = unique_test_dir("bd-oxide-hover-view")?;
        let project_path = workspace_dir.join("WorkspaceSurface.basproj");
        write_test_file(&workspace_dir.join("Module1.bas"), sample_module_text())?;
        write_test_file(&workspace_dir.join("Module2.bas"), sample_second_module_text())?;
        write_test_file(&project_path, sample_workspace_project_text())?;

        let mut model = ShellModel::new(Some(project_path)).map_err(|error| error.to_string())?;
        expect_none_cmd(
            enter_and_dispatch_command(&mut model, "module Module1"),
            "opening project module for hover",
        )?;
        move_cursor_to_first(&mut model, "Main")?;
        expect_none_cmd(
            enter_and_dispatch_command(&mut model, "hover"),
            "opening hover output",
        )?;

        let output = model.output_text();
        if !output.contains("Label:") || !output.contains("Main") {
            return Err(String::from(
                "hover view should render hover information from the direct host session at the current cursor",
            ));
        }

        Ok(())
    }

    #[test]
    fn completion_command_switches_output_to_cursor_completions() -> Result<(), String> {
        let workspace_dir = unique_test_dir("bd-oxide-completion-view")?;
        let project_path = workspace_dir.join("WorkspaceSurface.basproj");
        write_test_file(&workspace_dir.join("Module1.bas"), sample_module_text())?;
        write_test_file(&workspace_dir.join("Module2.bas"), sample_second_module_text())?;
        write_test_file(&project_path, sample_workspace_project_text())?;

        let mut model = ShellModel::new(Some(project_path)).map_err(|error| error.to_string())?;
        expect_none_cmd(
            enter_and_dispatch_command(&mut model, "module Module1"),
            "opening project module for completions",
        )?;
        move_cursor_to_first(&mut model, "answer =")?;
        expect_none_cmd(
            enter_and_dispatch_command(&mut model, "complete"),
            "opening completions output",
        )?;

        let output = model.output_text();
        if !output.contains("Completion Count:") {
            return Err(String::from(
                "completions view should render completion results from the direct host session at the current cursor",
            ));
        }
        if !output.contains("answer") && !output.contains("Sub") {
            return Err(String::from(
                "completions view should include at least one semantic or keyword completion",
            ));
        }

        Ok(())
    }

    #[test]
    fn oxvba_cli_args_match_action_and_target() -> Result<(), String> {
        let build_request = OxVbaExecutionRequest {
            action: OxVbaExecutionAction::Build,
            target: OxVbaExecutionTarget::LooseFile(PathBuf::from("module.bas")),
            runtime_profile: None,
            policy_preset: None,
        };
        let run_request = OxVbaExecutionRequest {
            action: OxVbaExecutionAction::Run,
            target: OxVbaExecutionTarget::Project(PathBuf::from("demo.basproj")),
            runtime_profile: Some(String::from("linux-stdio")),
            policy_preset: Some(String::from("strict-ci")),
        };

        let build_args = oxvba_cli_args_for_request(&build_request);
        let run_args = oxvba_cli_args_for_request(&run_request);

        if build_args[5] != OsString::from("compile") {
            return Err(String::from("loose-file build should use compile"));
        }

        if run_args[5] != OsString::from("run-project") {
            return Err(String::from("project run should use run-project"));
        }

        if run_args[7] != OsString::from("--profile")
            || run_args[8] != OsString::from("linux-stdio")
            || run_args[9] != OsString::from("--policy")
            || run_args[10] != OsString::from("strict-ci")
        {
            return Err(String::from(
                "project run should forward runtime profile and policy selections",
            ));
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
    fn project_session_can_open_or_create_basproj_and_clear_state() -> Result<(), String> {
        let path = unique_test_dir("bd-q2k-2-session")?.join("Session.basproj");
        let (mut session, project_text) =
            ProjectSession::open_or_create(&path).map_err(|error| error.to_string())?;

        if session.display_name() != path.display().to_string() {
            return Err(String::from("project session should retain the project path"));
        }

        if !session.has_manifest() {
            return Err(String::from("project session should retain the manifest seam"));
        }

        let manifest = session
            .manifest()
            .ok_or_else(|| String::from("project session should expose the loaded manifest"))?;
        if manifest.project_name != "Session" {
            return Err(String::from(
                "new project sessions should derive the manifest name from the basproj path",
            ));
        }

        let project_text = project_text
            .ok_or_else(|| String::from("opening a basproj path should surface project text"))?;
        if !project_text.contains("<Project Sdk=\"OxVba.Sdk/0.1.0\">")
            || !project_text.contains("<ProjectName>")
        {
            return Err(String::from(
                "project session should produce canonical project XML for new projects",
            ));
        }

        session.clear();

        if session.has_manifest() || session.display_name() != "(no project)" {
            return Err(String::from("clearing the project session should drop project state"));
        }

        Ok(())
    }

    #[test]
    fn existing_basproj_loads_into_project_session_and_workspace_model() -> Result<(), String> {
        let workspace_dir = unique_test_dir("bd-32y-load-project")?;
        let project_path = workspace_dir.join("WorkspaceSurface.basproj");
        let module1_path = workspace_dir.join("Module1.bas");
        let module2_path = workspace_dir.join("Module2.bas");
        write_test_file(&module1_path, sample_module_text())?;
        write_test_file(&module2_path, sample_second_module_text())?;
        write_test_file(&project_path, sample_workspace_project_text())?;

        let model = ShellModel::new(Some(project_path.clone())).map_err(|error| error.to_string())?;

        if model.project_session.display_name() != project_path.display().to_string() {
            return Err(String::from(
                "existing basproj should bind an active project session",
            ));
        }

        let manifest = model
            .project_session
            .manifest()
            .ok_or_else(|| String::from("loaded basproj should expose a project manifest"))?;
        if manifest.project_name != "WorkspaceSurface" {
            return Err(String::from(
                "project session should load the real project identity from basproj",
            ));
        }

        let module_entries = model.project_session.module_entries();
        if module_entries.len() != 2 {
            return Err(String::from(
                "project session should load the module roster from the project",
            ));
        }

        if module_entries[0].path != module1_path || module_entries[1].path != module2_path {
            return Err(String::from(
                "project session should resolve module paths relative to the basproj location",
            ));
        }

        if model.project_session.workspace_document_count() != 2 {
            return Err(String::from(
                "loading a basproj should seed the direct host session with project modules",
            ));
        }

        Ok(())
    }

    #[test]
    fn project_new_command_can_create_and_save_basproj() -> Result<(), String> {
        let project_path = unique_test_dir("bd-q2k-2-new-project")?.join("NewProject.basproj");
        let mut model = ShellModel::new(None).map_err(|error| error.to_string())?;
        let open_cmd = format!("project-new {}", project_path.display());

        expect_none_cmd(
            enter_and_dispatch_command(&mut model, &open_cmd),
            "project-new command",
        )?;

        if model.document_session.display_name() != project_path.display().to_string() {
            return Err(String::from("project-new should bind the requested basproj path"));
        }

        if !model.project_session.has_manifest() {
            return Err(String::from("project-new should initialize a project session"));
        }

        let initial_text = model.editor.text();
        if !initial_text.contains("<Project Sdk=\"OxVba.Sdk/0.1.0\">") {
            return Err(String::from("project-new should seed canonical basproj XML"));
        }

        expect_none_cmd(model.update(Msg::Save), "saving a new basproj")?;

        let saved_text = fs::read_to_string(&project_path).map_err(|error| error.to_string())?;
        if saved_text != model.document_session.saved_text() {
            return Err(String::from(
                "saving a new basproj should persist the canonical project XML",
            ));
        }

        Ok(())
    }

    #[test]
    fn project_surface_lists_modules_and_navigation_commands() -> Result<(), String> {
        let workspace_dir = unique_test_dir("bd-q2k-3-surface")?;
        let project_path = workspace_dir.join("WorkspaceSurface.basproj");
        write_test_file(&workspace_dir.join("Module1.bas"), sample_module_text())?;
        write_test_file(&workspace_dir.join("Module2.bas"), sample_second_module_text())?;
        write_test_file(&project_path, sample_workspace_project_text())?;

        let model = ShellModel::new(Some(project_path)).map_err(|error| error.to_string())?;
        let surface = model.project_session.surface_text(&model.document_session);

        if !surface.contains("Modules: 2") {
            return Err(String::from("project surface should report the module count"));
        }

        if !surface.contains("> Module1") || !surface.contains("  Module2") {
            return Err(String::from("project surface should list the project modules"));
        }

        if !surface.contains(":module <name>") || !surface.contains(":module-next") {
            return Err(String::from(
                "project surface should advertise the module navigation commands",
            ));
        }

        Ok(())
    }

    #[test]
    fn module_command_opens_project_module_and_keeps_project_session() -> Result<(), String> {
        let workspace_dir = unique_test_dir("bd-q2k-3-open-module")?;
        let project_path = workspace_dir.join("WorkspaceSurface.basproj");
        let module1_path = workspace_dir.join("Module1.bas");
        write_test_file(&module1_path, sample_module_text())?;
        write_test_file(&workspace_dir.join("Module2.bas"), sample_second_module_text())?;
        write_test_file(&project_path, sample_workspace_project_text())?;

        let mut model = ShellModel::new(Some(project_path.clone())).map_err(|error| error.to_string())?;
        expect_none_cmd(
            enter_and_dispatch_command(&mut model, "module Module1"),
            "opening a project module",
        )?;

        if model.document_session.display_name() != module1_path.display().to_string() {
            return Err(String::from("module command should open the selected module document"));
        }

        if model.project_session.display_name() != project_path.display().to_string() {
            return Err(String::from("module command should retain the active project session"));
        }

        let surface = model.project_session.surface_text(&model.document_session);
        if !surface.contains("* Module1") {
            return Err(String::from("project surface should mark the active module document"));
        }

        Ok(())
    }

    #[test]
    fn module_next_and_prev_cycle_project_modules() -> Result<(), String> {
        let workspace_dir = unique_test_dir("bd-q2k-3-cycle-modules")?;
        let project_path = workspace_dir.join("WorkspaceSurface.basproj");
        let module1_path = workspace_dir.join("Module1.bas");
        let module2_path = workspace_dir.join("Module2.bas");
        write_test_file(&module1_path, sample_module_text())?;
        write_test_file(&module2_path, sample_second_module_text())?;
        write_test_file(&project_path, sample_workspace_project_text())?;

        let mut model = ShellModel::new(Some(project_path)).map_err(|error| error.to_string())?;

        expect_none_cmd(
            enter_and_dispatch_command(&mut model, "module-next"),
            "cycling to the next module",
        )?;
        if model.document_session.display_name() != module2_path.display().to_string() {
            return Err(String::from("module-next should open the next project module"));
        }

        expect_none_cmd(
            enter_and_dispatch_command(&mut model, "module-prev"),
            "cycling to the previous module",
        )?;
        if model.document_session.display_name() != module1_path.display().to_string() {
            return Err(String::from("module-prev should wrap back to the previous module"));
        }

        Ok(())
    }

    #[test]
    fn saving_project_module_updates_loaded_project_source() -> Result<(), String> {
        let workspace_dir = unique_test_dir("bd-1oa-save-module")?;
        let project_path = workspace_dir.join("WorkspaceSurface.basproj");
        let module1_path = workspace_dir.join("Module1.bas");
        write_test_file(&module1_path, sample_module_text())?;
        write_test_file(&workspace_dir.join("Module2.bas"), sample_second_module_text())?;
        write_test_file(&project_path, sample_workspace_project_text())?;

        let mut model = ShellModel::new(Some(project_path.clone())).map_err(|error| error.to_string())?;
        expect_none_cmd(
            enter_and_dispatch_command(&mut model, "module Module1"),
            "opening a project module",
        )?;

        let updated_source = format!("{}\n' saved through project model", sample_module_text());
        model.editor = crate::new_editor(&updated_source);
        model.editor_state = RefCell::new(crate::TextAreaState::default());

        expect_none_cmd(model.update(Msg::Save), "saving a project module")?;

        let saved_module = fs::read_to_string(&module1_path).map_err(|error| error.to_string())?;
        if saved_module != updated_source {
            return Err(String::from("saving a project module should update the backing module file"));
        }

        let manifest = model
            .project_session
            .manifest()
            .ok_or_else(|| String::from("project session should retain a loaded manifest"))?;
        let module = manifest
            .modules
            .iter()
            .find(|module| module.module_name == "Module1")
            .ok_or_else(|| String::from("project manifest should still contain Module1"))?;
        if module.source != updated_source {
            return Err(String::from(
                "saving a project module should round-trip edits back into the project model",
            ));
        }

        Ok(())
    }

    #[test]
    fn direct_host_session_tracks_project_modules_from_manifest() -> Result<(), String> {
        let workspace_dir = unique_test_dir("bd-q2k-4-workspace")?;
        let project_path = workspace_dir.join("WorkspaceSurface.basproj");
        write_test_file(&workspace_dir.join("Module1.bas"), sample_module_text())?;
        write_test_file(&workspace_dir.join("Module2.bas"), sample_second_module_text())?;
        write_test_file(&project_path, sample_workspace_project_text())?;

        let model = ShellModel::new(Some(project_path)).map_err(|error| error.to_string())?;

        if model.project_session.workspace_document_count() == 0 {
            return Err(String::from(
                "direct host session should expose a non-empty loaded workspace",
            ));
        }

        let language_surface = model
            .project_session
            .language_surface_text(&model.document_session);
        if !language_surface.contains("Workspace Target:") {
            return Err(String::from(
                "language surface should show the loaded host workspace session",
            ));
        }

        Ok(())
    }

    #[test]
    fn revert_command_restores_current_file_from_disk() -> Result<(), String> {
        let workspace_dir = unique_test_dir("bd-oxide-revert")?;
        let module_path = workspace_dir.join("Scratch.bas");
        write_test_file(&module_path, sample_module_text())?;

        let mut model = ShellModel::new(Some(module_path.clone())).map_err(|error| error.to_string())?;
        expect_none_cmd(
            model.update(Msg::Editor(Event::Key(KeyEvent::new(KeyCode::Char('\''))))),
            "editing current file before revert",
        )?;

        if !model.is_dirty() {
            return Err(String::from("edit before revert should make the buffer dirty"));
        }

        expect_none_cmd(
            enter_and_dispatch_command(&mut model, "revert"),
            "reverting current file",
        )?;

        if model.editor.text() != sample_module_text() || model.is_dirty() {
            return Err(String::from(
                "revert should restore the current file contents from disk and clear dirty state",
            ));
        }

        Ok(())
    }

    #[test]
    fn workspace_reload_refreshes_project_module_from_disk() -> Result<(), String> {
        let workspace_dir = unique_test_dir("bd-oxide-workspace-reload")?;
        let project_path = workspace_dir.join("WorkspaceSurface.basproj");
        let module1_path = workspace_dir.join("Module1.bas");
        write_test_file(&module1_path, sample_module_text())?;
        write_test_file(&workspace_dir.join("Module2.bas"), sample_second_module_text())?;
        write_test_file(&project_path, sample_workspace_project_text())?;

        let mut model = ShellModel::new(Some(project_path)).map_err(|error| error.to_string())?;
        expect_none_cmd(
            enter_and_dispatch_command(&mut model, "module Module1"),
            "opening module before workspace reload",
        )?;

        let reloaded_source = format!("{}\n' reloaded from disk", sample_module_text());
        write_test_file(&module1_path, &reloaded_source)?;

        expect_none_cmd(
            enter_and_dispatch_command(&mut model, "workspace-reload"),
            "reloading workspace",
        )?;

        if model.editor.text() != reloaded_source {
            return Err(String::from(
                "workspace-reload should refresh the active project-backed module from disk",
            ));
        }

        if !model.status.contains("Reloaded workspace") {
            return Err(String::from(
                "workspace reload should report that the active project workspace was refreshed",
            ));
        }

        Ok(())
    }

    #[test]
    fn direct_host_session_uses_unsaved_host_text_for_current_module() -> Result<(), String> {
        let workspace_dir = unique_test_dir("bd-q2k-4-host-text")?;
        let project_path = workspace_dir.join("WorkspaceSurface.basproj");
        write_test_file(&workspace_dir.join("Module1.bas"), sample_module_text())?;
        write_test_file(&workspace_dir.join("Module2.bas"), sample_second_module_text())?;
        write_test_file(&project_path, sample_workspace_project_text())?;

        let mut model = ShellModel::new(Some(project_path)).map_err(|error| error.to_string())?;
        expect_none_cmd(
            enter_and_dispatch_command(&mut model, "module Module1"),
            "opening a project module",
        )?;

        let initial_diagnostics = model
            .current_language_status()
            .map_err(|error| error.to_string())?
            .ok_or_else(|| String::from("expected host-backed document diagnostics"))?
            .diagnostics;

        for ch in "\nSub Broken(\n".chars() {
            expect_none_cmd(
                model.update(Msg::Editor(Event::Key(KeyEvent::new(KeyCode::Char(ch))))),
                "typing invalid module text",
            )?;
        }

        let updated_status = model
            .current_language_status()
            .map_err(|error| error.to_string())?
            .ok_or_else(|| String::from("expected updated host-backed diagnostics"))?;
        let updated_diagnostics = updated_status.diagnostics;
        if updated_diagnostics.len() <= initial_diagnostics.len() {
            return Err(String::from(
                "direct host session should analyze the unsaved editor text, not just disk contents",
            ));
        }

        if updated_status.symbols.is_empty() {
            return Err(String::from(
                "direct host session should also surface document symbols for the active module",
            ));
        }

        Ok(())
    }

    #[test]
    fn project_surface_reports_target_aware_build_and_run_modes() -> Result<(), String> {
        let workspace_dir = unique_test_dir("bd-q2k-5-surface")?;
        let project_path = workspace_dir.join("TargetSurface.basproj");
        write_test_file(&workspace_dir.join("Module1.bas"), sample_module_text())?;
        write_test_file(&project_path, &sample_project_text_for_output_type("ComServer"))?;

        let model = ShellModel::new(Some(project_path)).map_err(|error| error.to_string())?;
        let surface = model.project_session.surface_text(&model.document_session);

        if !surface.contains("Target: ComServer") {
            return Err(String::from("project surface should show the selected output target"));
        }

        if !surface.contains("Build Output: COM server bundle")
            || !surface.contains("Run Surface: build only")
        {
            return Err(String::from(
                "project surface should expose target-aware build and run semantics",
            ));
        }

        Ok(())
    }

    #[test]
    fn run_command_blocks_build_only_library_targets() -> Result<(), String> {
        let workspace_dir = unique_test_dir("bd-q2k-5-library")?;
        let project_path = workspace_dir.join("TargetSurface.basproj");
        write_test_file(&workspace_dir.join("Module1.bas"), sample_module_text())?;
        write_test_file(&project_path, &sample_project_text_for_output_type("Library"))?;
        let run_result = OxVbaExecutionResult {
            action: OxVbaExecutionAction::Run,
            target: OxVbaExecutionTarget::Project(project_path.clone()),
            success: true,
            exit_code: Some(0),
            stdout: String::new(),
            stderr: String::new(),
        };
        let (fake_services, state) = FakeOxVbaServices::queued(vec![Ok(run_result)]);
        let mut model =
            ShellModel::with_services(Some(project_path), Box::new(fake_services))
                .map_err(|error| error.to_string())?;

        expect_none_cmd(enter_and_dispatch_command(&mut model, "run"), "run command")?;

        if !model.status.contains("Library targets are build-only") {
            return Err(String::from(
                "run should explain that library targets are build-only",
            ));
        }

        if !state.requests.borrow().is_empty() {
            return Err(String::from(
                "run should not call OxVbaServices for build-only targets",
            ));
        }

        Ok(())
    }

    #[test]
    fn run_command_blocks_host_activation_targets() -> Result<(), String> {
        let workspace_dir = unique_test_dir("bd-q2k-5-addin")?;
        let project_path = workspace_dir.join("TargetSurface.basproj");
        write_test_file(&workspace_dir.join("Module1.bas"), sample_module_text())?;
        write_test_file(&project_path, &sample_project_text_for_output_type("Addin"))?;
        let run_result = OxVbaExecutionResult {
            action: OxVbaExecutionAction::Run,
            target: OxVbaExecutionTarget::Project(project_path.clone()),
            success: true,
            exit_code: Some(0),
            stdout: String::new(),
            stderr: String::new(),
        };
        let (fake_services, state) = FakeOxVbaServices::queued(vec![Ok(run_result)]);
        let mut model =
            ShellModel::with_services(Some(project_path), Box::new(fake_services))
                .map_err(|error| error.to_string())?;

        expect_none_cmd(enter_and_dispatch_command(&mut model, "run"), "run command")?;

        if !model.status.contains("Addin targets require host activation") {
            return Err(String::from(
                "run should explain that addin targets require host activation",
            ));
        }

        if !state.requests.borrow().is_empty() {
            return Err(String::from(
                "run should not call OxVbaServices for host-activation targets",
            ));
        }

        Ok(())
    }

    #[test]
    fn run_command_allows_executable_targets() -> Result<(), String> {
        let workspace_dir = unique_test_dir("bd-q2k-5-comexe")?;
        let project_path = workspace_dir.join("TargetSurface.basproj");
        write_test_file(&workspace_dir.join("Module1.bas"), sample_module_text())?;
        write_test_file(&project_path, &sample_project_text_for_output_type("ComExe"))?;
        let run_result = OxVbaExecutionResult {
            action: OxVbaExecutionAction::Run,
            target: OxVbaExecutionTarget::Project(project_path.clone()),
            success: true,
            exit_code: Some(0),
            stdout: String::from("ran target"),
            stderr: String::new(),
        };
        let (fake_services, state) = FakeOxVbaServices::queued(vec![Ok(run_result)]);
        let mut model =
            ShellModel::with_services(Some(project_path.clone()), Box::new(fake_services))
                .map_err(|error| error.to_string())?;

        expect_none_cmd(enter_and_dispatch_command(&mut model, "run"), "run command")?;

        if !model.status.contains("Run succeeded") {
            return Err(String::from(
                "run should still execute through OxVbaServices for executable targets",
            ));
        }

        let requests = state.requests.borrow();
        if requests.len() != 1 {
            return Err(String::from(
                "run should make exactly one OxVbaServices request for executable targets",
            ));
        }
        expect_project_request(&requests[0], OxVbaExecutionAction::Run, &project_path)?;

        Ok(())
    }

    #[test]
    fn profile_and_policy_commands_update_project_selection_surface() -> Result<(), String> {
        let workspace_dir = unique_test_dir("bd-q2k-6-surface")?;
        let project_path = workspace_dir.join("TargetSurface.basproj");
        write_test_file(&workspace_dir.join("Module1.bas"), sample_module_text())?;
        write_test_file(&project_path, sample_project_text())?;

        let mut model = ShellModel::new(Some(project_path)).map_err(|error| error.to_string())?;

        expect_none_cmd(
            enter_and_dispatch_command(&mut model, "profile linux-stdio"),
            "setting runtime profile",
        )?;
        expect_none_cmd(
            enter_and_dispatch_command(&mut model, "policy strict-ci"),
            "setting policy preset",
        )?;

        let surface = model.project_session.surface_text(&model.document_session);
        if !surface.contains("Runtime Profile: linux-stdio")
            || !surface.contains("Host Policy: strict-ci")
        {
            return Err(String::from(
                "project surface should show the selected runtime profile and policy preset",
            ));
        }

        Ok(())
    }

    #[test]
    fn run_command_forwards_selected_profile_and_policy() -> Result<(), String> {
        let workspace_dir = unique_test_dir("bd-q2k-6-run")?;
        let project_path = workspace_dir.join("TargetSurface.basproj");
        write_test_file(&workspace_dir.join("Module1.bas"), sample_module_text())?;
        write_test_file(&project_path, &sample_project_text_for_output_type("Exe"))?;
        let run_result = OxVbaExecutionResult {
            action: OxVbaExecutionAction::Run,
            target: OxVbaExecutionTarget::Project(project_path.clone()),
            success: true,
            exit_code: Some(0),
            stdout: String::new(),
            stderr: String::new(),
        };
        let (fake_services, state) = FakeOxVbaServices::queued(vec![Ok(run_result)]);
        let mut model =
            ShellModel::with_services(Some(project_path), Box::new(fake_services))
                .map_err(|error| error.to_string())?;

        expect_none_cmd(
            enter_and_dispatch_command(&mut model, "profile linux-stdio"),
            "setting runtime profile",
        )?;
        expect_none_cmd(
            enter_and_dispatch_command(&mut model, "policy strict-ci"),
            "setting host policy",
        )?;
        expect_none_cmd(enter_and_dispatch_command(&mut model, "run"), "run command")?;

        let requests = state.requests.borrow();
        let request = requests
            .first()
            .ok_or_else(|| String::from("expected a run request"))?;

        if request.runtime_profile.as_deref() != Some("linux-stdio")
            || request.policy_preset.as_deref() != Some("strict-ci")
        {
            return Err(String::from(
                "run should forward the selected runtime profile and policy preset",
            ));
        }

        Ok(())
    }

    #[test]
    fn smoke_flow_covers_launch_edit_save_open_build_and_run() -> Result<(), String> {
        let workspace_dir = unique_test_dir("bd-237-8")?;
        let module_path = workspace_dir.join("Scratch.bas");
        let project_path = workspace_dir.join("ThinSliceSmoke.basproj");
        let project_module_path = workspace_dir.join("Module1.bas");

        write_test_file(&project_module_path, sample_module_text())?;
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
