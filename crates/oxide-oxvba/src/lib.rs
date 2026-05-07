//! OxIde-side adapter boundary for OxVba-owned truth.
//!
//! This crate consumes authoritative OxVba APIs and projects their results
//! into OxIde-owned GUI view models.

use std::fs;
use std::path::{Path, PathBuf};

use oxide_domain::{
    ActiveSourceSummary, HostCapabilitySummary, OxideDomainRole, ProjectModuleSummary,
    ProjectOpenSpineView,
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
}
