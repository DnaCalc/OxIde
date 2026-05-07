//! OxIde-side adapter boundary for OxVba-owned truth.
//!
//! This crate consumes authoritative OxVba APIs and projects their results
//! into OxIde-owned GUI view models.

use std::fs;
use std::path::{Path, PathBuf};

use oxide_domain::{
    ActiveSourceSummary, DiagnosticRow, EditedDocumentDiagnosticsView, HostCapabilitySummary,
    OxideDomainRole, ProjectModuleSummary, ProjectOpenSpineView,
};
use oxvba_languageservice::{
    DiagnosticSeverity, DocumentId, HostSessionError, HostWorkspaceSession,
};
use oxvba_project::inspect_workspace_target;

/// Compile-time marker for the OxVba adapter crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OxideOxVbaRole {
    /// Adapter over authoritative OxVba APIs and types.
    AuthoritativeOxVbaAdapter,
}

impl OxideOxVbaRole {
    pub fn consumes_domain_vocabulary(self) -> OxideDomainRole {
        match self {
            Self::AuthoritativeOxVbaAdapter => OxideDomainRole::HostIndependentIdeVocabulary,
        }
    }
}

/// Load a `.basproj` workspace through OxVba and project the smallest
/// W210 GUI-neutral project-open spine view model.
pub fn load_project_open_spine(
    workspace_path: impl AsRef<Path>,
) -> Result<ProjectOpenSpineView, ProjectOpenSpineError> {
    let workspace_path = workspace_path.as_ref();
    let surface = inspect_workspace_target(workspace_path).map_err(|source| {
        ProjectOpenSpineError::InspectWorkspace {
            path: workspace_path.to_path_buf(),
            message: source.to_string(),
        }
    })?;

    let mut modules = surface
        .modules
        .iter()
        .map(|module| ProjectModuleSummary {
            display_name: module
                .source_path
                .file_name()
                .and_then(|name| name.to_str())
                .map(String::from)
                .unwrap_or_else(|| module.identity.effective_name.clone()),
            include_path: module.include.clone(),
            is_active: false,
        })
        .collect::<Vec<_>>();

    modules.sort_by(|left, right| left.display_name.cmp(&right.display_name));

    let active_module = modules
        .first_mut()
        .ok_or_else(|| ProjectOpenSpineError::NoModules {
            path: workspace_path.to_path_buf(),
        })?;
    active_module.is_active = true;
    let active_display_name = active_module.display_name.clone();

    let active_source_path = surface
        .modules
        .iter()
        .find(|module| {
            module
                .source_path
                .file_name()
                .and_then(|name| name.to_str())
                == Some(active_display_name.as_str())
        })
        .map(|module| module.source_path.clone())
        .ok_or_else(|| ProjectOpenSpineError::ActiveModuleMissing {
            module: active_display_name.clone(),
        })?;

    let source_text = fs::read_to_string(&active_source_path).map_err(|source| {
        ProjectOpenSpineError::ReadSource {
            path: active_source_path.clone(),
            message: source.to_string(),
        }
    })?;

    Ok(ProjectOpenSpineView {
        project_name: surface.project_name,
        modules,
        active_source: ActiveSourceSummary {
            module_display_name: active_display_name,
            source_text,
        },
        capability: HostCapabilitySummary::browser_safe_default(),
    })
}

pub fn load_edited_document_diagnostics(
    workspace_path: impl AsRef<Path>,
    module_display_name: &str,
    edited_source_text: &str,
) -> Result<EditedDocumentDiagnosticsView, EditedDocumentDiagnosticsError> {
    let workspace_path = workspace_path.as_ref();
    let mut session =
        HostWorkspaceSession::load_workspace_path(workspace_path).map_err(|source| {
            EditedDocumentDiagnosticsError::SessionLoad {
                path: workspace_path.to_path_buf(),
                message: source.to_string(),
            }
        })?;
    let document_id = find_document_id(&session, module_display_name).ok_or_else(|| {
        EditedDocumentDiagnosticsError::ActiveDocumentMissing {
            module: module_display_name.to_string(),
        }
    })?;

    session
        .set_document_text(&document_id, edited_source_text)
        .map_err(|source| EditedDocumentDiagnosticsError::SetDocumentText {
            document: document_id.0.clone(),
            message: source.to_string(),
        })?;
    let diagnostics = session.diagnostics(&document_id).map_err(|source| {
        EditedDocumentDiagnosticsError::Diagnostics {
            document: document_id.0.clone(),
            message: source.to_string(),
        }
    })?;
    let project_name = session
        .documents()
        .into_iter()
        .find(|document| document.id == document_id)
        .and_then(|document| document.project_name)
        .unwrap_or_else(|| String::from("<unknown>"));

    Ok(EditedDocumentDiagnosticsView {
        project_name,
        module_display_name: module_display_name.to_string(),
        edited_source_text: edited_source_text.to_string(),
        diagnostics: diagnostics
            .into_iter()
            .map(|diagnostic| DiagnosticRow {
                module_display_name: module_display_name.to_string(),
                severity_label: severity_label(diagnostic.severity).to_string(),
                message: diagnostic.message,
                span_start: diagnostic.span.start,
                span_end: diagnostic.span.end,
                provenance_label: String::from("OxVba language service"),
            })
            .collect(),
        capability: HostCapabilitySummary::browser_safe_default(),
    })
}

fn find_document_id(
    session: &HostWorkspaceSession,
    module_display_name: &str,
) -> Option<DocumentId> {
    let stem = module_display_name
        .strip_suffix(".bas")
        .unwrap_or(module_display_name);
    session
        .documents()
        .into_iter()
        .find(|document| document.id.0 == module_display_name || document.id.0 == stem)
        .map(|document| document.id)
}

fn severity_label(severity: DiagnosticSeverity) -> &'static str {
    match severity {
        DiagnosticSeverity::Error => "error",
        DiagnosticSeverity::Warning => "warning",
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectOpenSpineError {
    InspectWorkspace { path: PathBuf, message: String },
    NoModules { path: PathBuf },
    ActiveModuleMissing { module: String },
    ReadSource { path: PathBuf, message: String },
}

impl std::fmt::Display for ProjectOpenSpineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InspectWorkspace { path, message } => {
                write!(f, "inspect workspace {}: {message}", path.display())
            }
            Self::NoModules { path } => write!(f, "workspace {} has no modules", path.display()),
            Self::ActiveModuleMissing { module } => {
                write!(f, "active module {module} was not found in OxVba surface")
            }
            Self::ReadSource { path, message } => {
                write!(f, "read source {}: {message}", path.display())
            }
        }
    }
}

impl std::error::Error for ProjectOpenSpineError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditedDocumentDiagnosticsError {
    SessionLoad { path: PathBuf, message: String },
    ActiveDocumentMissing { module: String },
    SetDocumentText { document: String, message: String },
    Diagnostics { document: String, message: String },
}

impl std::fmt::Display for EditedDocumentDiagnosticsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SessionLoad { path, message } => {
                write!(f, "load OxVba host session {}: {message}", path.display())
            }
            Self::ActiveDocumentMissing { module } => {
                write!(
                    f,
                    "active module {module} was not found in OxVba host session"
                )
            }
            Self::SetDocumentText { document, message } => {
                write!(f, "set OxVba document {document} text: {message}")
            }
            Self::Diagnostics { document, message } => {
                write!(f, "read OxVba diagnostics for {document}: {message}")
            }
        }
    }
}

impl std::error::Error for EditedDocumentDiagnosticsError {}

impl From<HostSessionError> for EditedDocumentDiagnosticsError {
    fn from(source: HostSessionError) -> Self {
        Self::Diagnostics {
            document: String::from("<unknown>"),
            message: source.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn thin_slice_fixture() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("examples")
            .join("thin-slice")
            .join("ThinSliceHello.basproj")
    }

    #[test]
    fn adapter_role_declares_authoritative_oxvba_boundary() {
        assert_eq!(
            OxideOxVbaRole::AuthoritativeOxVbaAdapter.consumes_domain_vocabulary(),
            OxideDomainRole::HostIndependentIdeVocabulary
        );
    }

    #[test]
    fn thin_slice_project_open_spine_reports_project_name() {
        let view = load_project_open_spine(thin_slice_fixture()).expect("thin-slice loads");

        assert_eq!(view.project_name, "ThinSliceHello");
    }

    #[test]
    fn thin_slice_project_open_spine_reports_module_list() {
        let view = load_project_open_spine(thin_slice_fixture()).expect("thin-slice loads");

        assert!(view.modules.iter().any(|module| {
            module.display_name == "Module1.bas" && module.include_path == "Module1.bas"
        }));
        assert_eq!(
            view.modules
                .iter()
                .filter(|module| module.is_active)
                .count(),
            1
        );
    }

    #[test]
    fn thin_slice_project_open_spine_reports_active_source_text() {
        let view = load_project_open_spine(thin_slice_fixture()).expect("thin-slice loads");

        assert_eq!(view.active_source.module_display_name, "Module1.bas");
        assert!(view.active_source.source_text.contains("Public Sub Main()"));
    }

    #[test]
    fn thin_slice_project_open_spine_reports_browser_safe_capability() {
        let view = load_project_open_spine(thin_slice_fixture()).expect("thin-slice loads");

        assert_eq!(view.capability.profile_name, "browser-safe");
        assert!(view.capability.oxvba_semantics_available);
        assert!(!view.capability.oxvba_execution_available);
        assert!(!view.capability.com_runtime_available);
    }

    #[test]
    fn edited_document_diagnostics_reports_oxvba_undeclared_variable() {
        let edited_source = "Attribute VB_Name = \"Module1\"\n\nOption Explicit\n\nPublic Sub Main()\n    answer = 40 + 2\nEnd Sub\n";

        let view =
            load_edited_document_diagnostics(thin_slice_fixture(), "Module1.bas", edited_source)
                .expect("diagnostics load");

        assert_eq!(view.project_name, "ThinSliceHello");
        assert_eq!(view.module_display_name, "Module1.bas");
        assert_eq!(view.edited_source_text, edited_source);
        assert!(view.capability.status_text.contains("COM unavailable"));
        assert!(
            view.diagnostics.iter().any(|diagnostic| {
                diagnostic.severity_label == "error"
                    && diagnostic.message.contains("undeclared variable")
                    && diagnostic.message.contains("answer")
                    && diagnostic.provenance_label == "OxVba language service"
            }),
            "expected OxVba undeclared-variable diagnostic, got {:?}",
            view.diagnostics
        );
    }
}
