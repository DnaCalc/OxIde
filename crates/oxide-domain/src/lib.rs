//! Host-independent OxIde GUI domain vocabulary.
//!
//! These types are owned by OxIde because they describe IDE presentation
//! state rather than VBA/project truth. OxVba-owned project enums and
//! semantic types should be consumed at adapter boundaries rather than
//! copied into this crate.

use serde::{Deserialize, Serialize};

/// Compile-time marker for the GUI pivot domain crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OxideDomainRole {
    /// Pure, host-independent vocabulary owned by OxIde.
    HostIndependentIdeVocabulary,
}

impl OxideDomainRole {
    pub const fn label(self) -> &'static str {
        match self {
            Self::HostIndependentIdeVocabulary => "host-independent IDE vocabulary",
        }
    }
}

/// Minimal host capability profile for the W210 project-open spine.
///
/// This is intentionally a product-facing status shape, not a duplicate
/// of any OxVba runtime enum. Later worksets may replace or widen it with
/// a shared DNA Calc capability type if one lands upstream.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostCapabilitySummary {
    pub profile_name: String,
    pub oxvba_semantics_available: bool,
    pub oxvba_execution_available: bool,
    pub com_runtime_available: bool,
    pub status_text: String,
}

impl HostCapabilitySummary {
    pub fn browser_safe_default() -> Self {
        Self {
            profile_name: String::from("browser-safe"),
            oxvba_semantics_available: true,
            oxvba_execution_available: false,
            com_runtime_available: false,
            status_text: String::from(
                "Browser-safe profile: editing and semantic projection available; native execution and COM unavailable.",
            ),
        }
    }
}

/// One module row as presented in the GUI project spine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectModuleSummary {
    pub display_name: String,
    pub include_path: String,
    pub is_active: bool,
}

/// Source text for the active project-backed module.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActiveSourceSummary {
    pub module_display_name: String,
    pub source_text: String,
}

/// GUI-neutral view model for the first W210 project-open spine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectOpenSpineView {
    pub project_name: String,
    pub modules: Vec<ProjectModuleSummary>,
    pub active_source: ActiveSourceSummary,
    pub capability: HostCapabilitySummary,
}

/// OxIde-owned diagnostic presentation row projected from authoritative OxVba
/// diagnostics. Severity is presentation text to avoid copying OxVba enums.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticRow {
    pub module_display_name: String,
    pub severity_label: String,
    pub message: String,
    pub span_start: u32,
    pub span_end: u32,
    pub provenance_label: String,
}

/// GUI-neutral edited document diagnostic view for W220.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditedDocumentDiagnosticsView {
    pub project_name: String,
    pub module_display_name: String,
    pub edited_source_text: String,
    pub diagnostics: Vec<DiagnosticRow>,
    pub capability: HostCapabilitySummary,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_role_names_host_independent_vocabulary() {
        assert_eq!(
            OxideDomainRole::HostIndependentIdeVocabulary.label(),
            "host-independent IDE vocabulary"
        );
    }

    #[test]
    fn browser_safe_default_reports_native_execution_and_com_unavailable() {
        let capability = HostCapabilitySummary::browser_safe_default();

        assert_eq!(capability.profile_name, "browser-safe");
        assert!(capability.oxvba_semantics_available);
        assert!(!capability.oxvba_execution_available);
        assert!(!capability.com_runtime_available);
        assert!(capability.status_text.contains("COM unavailable"));
    }
}
