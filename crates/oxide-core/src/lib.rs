//! GUI-neutral OxIde application orchestration.
//!
//! This crate owns GUI-native state transitions above OxIde domain
//! vocabulary. It must not import parked TUI session/editor code.

use oxide_domain::OxideDomainRole;
use serde::{Deserialize, Serialize};

/// Compile-time marker for the GUI core crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OxideCoreRole {
    /// State transitions and orchestration above domain vocabulary.
    GuiNeutralOrchestration,
}

impl OxideCoreRole {
    pub fn depends_on_domain(self) -> OxideDomainRole {
        match self {
            Self::GuiNeutralOrchestration => OxideDomainRole::HostIndependentIdeVocabulary,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentLifecycleState {
    persisted_source: String,
    working_source: String,
    capabilities: LifecycleCapabilities,
}

impl DocumentLifecycleState {
    pub fn open_clean(source: impl Into<String>, capabilities: LifecycleCapabilities) -> Self {
        let source = source.into();
        Self {
            persisted_source: source.clone(),
            working_source: source,
            capabilities,
        }
    }

    pub fn persisted_source(&self) -> &str {
        &self.persisted_source
    }

    pub fn working_source(&self) -> &str {
        &self.working_source
    }

    pub fn is_dirty(&self) -> bool {
        self.working_source != self.persisted_source
    }

    pub fn edit_working_source(&mut self, source: impl Into<String>) {
        self.working_source = source.into();
    }

    pub fn acknowledge_saved(&mut self) -> Result<(), LifecycleCommandDisabled> {
        self.ensure_enabled(LifecycleCommand::Save)?;
        self.persisted_source = self.working_source.clone();
        Ok(())
    }

    pub fn reload_from_persisted(
        &mut self,
        source: impl Into<String>,
    ) -> Result<(), LifecycleCommandDisabled> {
        self.ensure_enabled(LifecycleCommand::Reload)?;
        let source = source.into();
        self.persisted_source = source.clone();
        self.working_source = source;
        Ok(())
    }

    pub fn revert_to_persisted(&mut self) -> Result<(), LifecycleCommandDisabled> {
        self.ensure_enabled(LifecycleCommand::Revert)?;
        self.working_source = self.persisted_source.clone();
        Ok(())
    }

    pub fn command_status(&self, command: LifecycleCommand) -> LifecycleCommandStatus {
        match command {
            LifecycleCommand::Save if !self.capabilities.can_save => {
                LifecycleCommandStatus::disabled(
                    self.capabilities
                        .save_disabled_reason
                        .clone()
                        .unwrap_or_else(|| {
                            String::from("save is unavailable in this host profile")
                        }),
                )
            }
            LifecycleCommand::Reload if !self.capabilities.can_reload => {
                LifecycleCommandStatus::disabled(
                    self.capabilities
                        .reload_disabled_reason
                        .clone()
                        .unwrap_or_else(|| {
                            String::from("reload is unavailable in this host profile")
                        }),
                )
            }
            LifecycleCommand::Revert if !self.capabilities.can_revert => {
                LifecycleCommandStatus::disabled(
                    self.capabilities
                        .revert_disabled_reason
                        .clone()
                        .unwrap_or_else(|| {
                            String::from("revert is unavailable in this host profile")
                        }),
                )
            }
            LifecycleCommand::Save => LifecycleCommandStatus::enabled(self.is_dirty()),
            LifecycleCommand::Reload => LifecycleCommandStatus::enabled(true),
            LifecycleCommand::Revert => LifecycleCommandStatus::enabled(self.is_dirty()),
        }
    }

    fn ensure_enabled(&self, command: LifecycleCommand) -> Result<(), LifecycleCommandDisabled> {
        let status = self.command_status(command);
        if status.is_enabled {
            Ok(())
        } else {
            Err(LifecycleCommandDisabled {
                command,
                reason: status
                    .reason
                    .unwrap_or_else(|| String::from("command disabled")),
            })
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifecycleCapabilities {
    pub can_save: bool,
    pub can_reload: bool,
    pub can_revert: bool,
    pub save_disabled_reason: Option<String>,
    pub reload_disabled_reason: Option<String>,
    pub revert_disabled_reason: Option<String>,
}

impl LifecycleCapabilities {
    pub fn all_supported() -> Self {
        Self {
            can_save: true,
            can_reload: true,
            can_revert: true,
            save_disabled_reason: None,
            reload_disabled_reason: None,
            revert_disabled_reason: None,
        }
    }

    pub fn browser_limited() -> Self {
        let reason = String::from("browser-safe profile has no direct filesystem persistence");
        Self {
            can_save: false,
            can_reload: false,
            can_revert: true,
            save_disabled_reason: Some(reason.clone()),
            reload_disabled_reason: Some(reason),
            revert_disabled_reason: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleCommand {
    Save,
    Reload,
    Revert,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifecycleCommandStatus {
    pub is_enabled: bool,
    pub reason: Option<String>,
}

impl LifecycleCommandStatus {
    pub fn enabled(is_enabled: bool) -> Self {
        Self {
            is_enabled,
            reason: None,
        }
    }

    pub fn disabled(reason: impl Into<String>) -> Self {
        Self {
            is_enabled: false,
            reason: Some(reason.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifecycleCommandDisabled {
    pub command: LifecycleCommand,
    pub reason: String,
}

impl std::fmt::Display for LifecycleCommandDisabled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} disabled: {}", self.command, self.reason)
    }
}

impl std::error::Error for LifecycleCommandDisabled {}

pub trait DocumentPersistence {
    fn load(&self) -> Result<String, PersistenceError>;
    fn save(&mut self, source: &str) -> Result<(), PersistenceError>;
    fn capabilities(&self) -> LifecycleCapabilities;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InMemoryDocumentPersistence {
    persisted_source: String,
}

impl InMemoryDocumentPersistence {
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            persisted_source: source.into(),
        }
    }

    pub fn persisted_source(&self) -> &str {
        &self.persisted_source
    }
}

impl DocumentPersistence for InMemoryDocumentPersistence {
    fn load(&self) -> Result<String, PersistenceError> {
        Ok(self.persisted_source.clone())
    }

    fn save(&mut self, source: &str) -> Result<(), PersistenceError> {
        self.persisted_source = source.to_string();
        Ok(())
    }

    fn capabilities(&self) -> LifecycleCapabilities {
        LifecycleCapabilities::all_supported()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrowserLimitedPersistence {
    source_snapshot: String,
    disabled_reason: String,
}

impl BrowserLimitedPersistence {
    pub fn new(source_snapshot: impl Into<String>) -> Self {
        Self {
            source_snapshot: source_snapshot.into(),
            disabled_reason: String::from(
                "browser-safe profile has no direct filesystem persistence",
            ),
        }
    }

    pub fn source_snapshot(&self) -> &str {
        &self.source_snapshot
    }
}

impl DocumentPersistence for BrowserLimitedPersistence {
    fn load(&self) -> Result<String, PersistenceError> {
        Err(PersistenceError::Disabled {
            operation: PersistenceOperation::Load,
            reason: self.disabled_reason.clone(),
        })
    }

    fn save(&mut self, _source: &str) -> Result<(), PersistenceError> {
        Err(PersistenceError::Disabled {
            operation: PersistenceOperation::Save,
            reason: self.disabled_reason.clone(),
        })
    }

    fn capabilities(&self) -> LifecycleCapabilities {
        LifecycleCapabilities::browser_limited()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersistenceOperation {
    Load,
    Save,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PersistenceError {
    Disabled {
        operation: PersistenceOperation,
        reason: String,
    },
}

impl std::fmt::Display for PersistenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Disabled { operation, reason } => write!(f, "{:?} disabled: {reason}", operation),
        }
    }
}

impl std::error::Error for PersistenceError {}

pub fn open_lifecycle_from_persistence(
    persistence: &impl DocumentPersistence,
) -> Result<DocumentLifecycleState, PersistenceError> {
    Ok(DocumentLifecycleState::open_clean(
        persistence.load()?,
        persistence.capabilities(),
    ))
}

pub fn save_lifecycle_to_persistence(
    state: &mut DocumentLifecycleState,
    persistence: &mut impl DocumentPersistence,
) -> Result<(), PersistenceError> {
    persistence.save(state.working_source())?;
    state
        .acknowledge_saved()
        .map_err(|disabled| PersistenceError::Disabled {
            operation: PersistenceOperation::Save,
            reason: disabled.reason,
        })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionCapabilityProfile {
    AllSupported,
    BrowserLimited,
}

impl SessionCapabilityProfile {
    pub fn lifecycle_capabilities(self) -> LifecycleCapabilities {
        match self {
            Self::AllSupported => LifecycleCapabilities::all_supported(),
            Self::BrowserLimited => LifecycleCapabilities::browser_limited(),
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::AllSupported => "all-supported",
            Self::BrowserLimited => "browser-limited",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuiSessionSnapshot {
    pub workspace_path: String,
    pub active_module: String,
    pub persisted_source: String,
    pub working_source: String,
    pub capability_profile: SessionCapabilityProfile,
}

impl GuiSessionSnapshot {
    pub fn capture(
        workspace_path: impl Into<String>,
        active_module: impl Into<String>,
        document: &DocumentLifecycleState,
        capability_profile: SessionCapabilityProfile,
    ) -> Self {
        Self {
            workspace_path: workspace_path.into(),
            active_module: active_module.into(),
            persisted_source: document.persisted_source().to_string(),
            working_source: document.working_source().to_string(),
            capability_profile,
        }
    }

    pub fn is_dirty(&self) -> bool {
        self.persisted_source != self.working_source
    }

    pub fn restore(&self) -> RestoredGuiSession {
        let mut document = DocumentLifecycleState::open_clean(
            self.persisted_source.clone(),
            self.capability_profile.lifecycle_capabilities(),
        );
        document.edit_working_source(self.working_source.clone());
        RestoredGuiSession {
            workspace_path: self.workspace_path.clone(),
            active_module: self.active_module.clone(),
            document,
            capability_profile: self.capability_profile,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoredGuiSession {
    pub workspace_path: String,
    pub active_module: String,
    pub document: DocumentLifecycleState,
    pub capability_profile: SessionCapabilityProfile,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunCapabilityProfile {
    pub profile_name: String,
    pub provider_kind: RunProviderKind,
    pub can_run: bool,
    pub native_execution_available: bool,
    pub com_runtime_available: bool,
    pub disabled_reason: Option<String>,
}

impl RunCapabilityProfile {
    pub fn browser_safe_unsupported() -> Self {
        Self {
            profile_name: String::from("browser-safe"),
            provider_kind: RunProviderKind::BrowserUnsupported,
            can_run: false,
            native_execution_available: false,
            com_runtime_available: false,
            disabled_reason: Some(String::from(
                "Browser-safe profile cannot execute VBA; native execution provider unavailable.",
            )),
        }
    }

    pub fn simulated_supported() -> Self {
        Self {
            profile_name: String::from("simulated-supported"),
            provider_kind: RunProviderKind::Simulated,
            can_run: true,
            native_execution_available: false,
            com_runtime_available: false,
            disabled_reason: None,
        }
    }

    pub fn future_native_supported() -> Self {
        Self {
            profile_name: String::from("native-supported"),
            provider_kind: RunProviderKind::Native,
            can_run: true,
            native_execution_available: true,
            com_runtime_available: false,
            disabled_reason: None,
        }
    }

    pub fn command_status(&self) -> RunCommandStatus {
        if self.can_run {
            RunCommandStatus {
                is_enabled: true,
                reason: None,
            }
        } else {
            RunCommandStatus {
                is_enabled: false,
                reason: Some(
                    self.disabled_reason
                        .clone()
                        .unwrap_or_else(|| String::from("run is unavailable in this host profile")),
                ),
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RunProviderKind {
    BrowserUnsupported,
    Simulated,
    Native,
}

impl RunProviderKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::BrowserUnsupported => "browser-unsupported",
            Self::Simulated => "simulated",
            Self::Native => "native",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunCommandStatus {
    pub is_enabled: bool,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunRequest {
    pub project_name: String,
    pub module_name: String,
    pub entrypoint: String,
}

impl RunRequest {
    pub fn new(
        project_name: impl Into<String>,
        module_name: impl Into<String>,
        entrypoint: impl Into<String>,
    ) -> Self {
        Self {
            project_name: project_name.into(),
            module_name: module_name.into(),
            entrypoint: entrypoint.into(),
        }
    }

    pub fn display_target(&self) -> String {
        format!(
            "{}::{}.{}",
            self.project_name, self.module_name, self.entrypoint
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RunTranscriptStatus {
    Disabled,
    Completed,
}

impl RunTranscriptStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::Completed => "completed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunTranscript {
    pub request: RunRequest,
    pub provider_label: String,
    pub status: RunTranscriptStatus,
    pub events: Vec<RunOutputEvent>,
}

impl RunTranscript {
    pub fn browser_disabled(request: RunRequest, profile: RunCapabilityProfile) -> Self {
        let reason = profile
            .command_status()
            .reason
            .unwrap_or_else(|| String::from("run is unavailable in this host profile"));
        Self {
            request,
            provider_label: profile.provider_kind.label().to_string(),
            status: RunTranscriptStatus::Disabled,
            events: vec![
                RunOutputEvent::new(RunOutputEventKind::Lifecycle, "run requested"),
                RunOutputEvent::new(
                    RunOutputEventKind::Diagnostic,
                    format!("Run disabled: {reason}"),
                ),
            ],
        }
    }

    pub fn simulated_completed(request: RunRequest) -> Self {
        let target = request.display_target();
        Self {
            request,
            provider_label: RunProviderKind::Simulated.label().to_string(),
            status: RunTranscriptStatus::Completed,
            events: vec![
                RunOutputEvent::new(RunOutputEventKind::Lifecycle, "run started"),
                RunOutputEvent::new(
                    RunOutputEventKind::Activity,
                    format!("simulated provider invoked {target}"),
                ),
                RunOutputEvent::new(
                    RunOutputEventKind::Output,
                    "simulated output: Main completed with answer 42",
                ),
                RunOutputEvent::new(RunOutputEventKind::Lifecycle, "run completed"),
            ],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RunOutputEventKind {
    Lifecycle,
    Activity,
    Diagnostic,
    Output,
}

impl RunOutputEventKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Lifecycle => "lifecycle",
            Self::Activity => "activity",
            Self::Diagnostic => "diagnostic",
            Self::Output => "output",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunOutputEvent {
    pub kind: RunOutputEventKind,
    pub message: String,
}

impl RunOutputEvent {
    pub fn new(kind: RunOutputEventKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComReferenceFact {
    pub display_name: String,
    pub identifier: String,
    pub source_label: String,
}

impl ComReferenceFact {
    pub fn new(
        display_name: impl Into<String>,
        identifier: impl Into<String>,
        source_label: impl Into<String>,
    ) -> Self {
        Self {
            display_name: display_name.into(),
            identifier: identifier.into(),
            source_label: source_label.into(),
        }
    }

    pub fn scripting_dictionary_demo() -> Self {
        Self::new(
            "COM reference present: Scripting.Dictionary",
            "Scripting.Dictionary",
            "project reference fact projected for capability review",
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComHostProfileKind {
    BrowserSafe,
    NonWindowsNative,
    WindowsNativeServiceMissing,
    WindowsNativeServiceAvailable,
}

impl ComHostProfileKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::BrowserSafe => "browser-safe",
            Self::NonWindowsNative => "non-windows-native",
            Self::WindowsNativeServiceMissing => "windows-native-service-missing",
            Self::WindowsNativeServiceAvailable => "windows-native-service-available",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComCapabilityFeature {
    ReferenceDiscovery,
    RuntimeInvocation,
}

impl ComCapabilityFeature {
    pub fn label(self) -> &'static str {
        match self {
            Self::ReferenceDiscovery => "reference-discovery",
            Self::RuntimeInvocation => "runtime-invocation",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComCapabilityStatus {
    pub feature: ComCapabilityFeature,
    pub is_available: bool,
    pub reason: Option<String>,
}

impl ComCapabilityStatus {
    pub fn available(feature: ComCapabilityFeature) -> Self {
        Self {
            feature,
            is_available: true,
            reason: None,
        }
    }

    pub fn unavailable(feature: ComCapabilityFeature, reason: impl Into<String>) -> Self {
        Self {
            feature,
            is_available: false,
            reason: Some(reason.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComCapabilityProfile {
    pub profile_name: String,
    pub host_kind: ComHostProfileKind,
    pub reference: ComReferenceFact,
    pub native_execution_available: bool,
    pub native_com_service_configured: bool,
    pub windows_native_host_required: bool,
    pub reference_discovery: ComCapabilityStatus,
    pub runtime_invocation: ComCapabilityStatus,
}

impl ComCapabilityProfile {
    pub fn browser_unavailable(reference: ComReferenceFact) -> Self {
        Self {
            profile_name: String::from("browser-safe-com-unavailable"),
            host_kind: ComHostProfileKind::BrowserSafe,
            reference,
            native_execution_available: false,
            native_com_service_configured: false,
            windows_native_host_required: true,
            reference_discovery: ComCapabilityStatus::unavailable(
                ComCapabilityFeature::ReferenceDiscovery,
                "COM discovery unavailable in browser-safe profile; Windows native host required.",
            ),
            runtime_invocation: ComCapabilityStatus::unavailable(
                ComCapabilityFeature::RuntimeInvocation,
                "COM runtime unavailable in browser-safe profile; pure browser/WASM cannot directly call Windows COM.",
            ),
        }
    }

    pub fn non_windows_native_unavailable(reference: ComReferenceFact) -> Self {
        Self {
            profile_name: String::from("non-windows-native-com-unavailable"),
            host_kind: ComHostProfileKind::NonWindowsNative,
            reference,
            native_execution_available: true,
            native_com_service_configured: false,
            windows_native_host_required: true,
            reference_discovery: ComCapabilityStatus::unavailable(
                ComCapabilityFeature::ReferenceDiscovery,
                "COM discovery unavailable on non-Windows native host; Windows native host required.",
            ),
            runtime_invocation: ComCapabilityStatus::unavailable(
                ComCapabilityFeature::RuntimeInvocation,
                "COM runtime unavailable on non-Windows native host; Windows native host required.",
            ),
        }
    }

    pub fn windows_native_service_missing(reference: ComReferenceFact) -> Self {
        Self {
            profile_name: String::from("windows-native-com-service-missing"),
            host_kind: ComHostProfileKind::WindowsNativeServiceMissing,
            reference,
            native_execution_available: true,
            native_com_service_configured: false,
            windows_native_host_required: false,
            reference_discovery: ComCapabilityStatus::unavailable(
                ComCapabilityFeature::ReferenceDiscovery,
                "native COM service not configured; COM discovery blocked until service handoff is implemented.",
            ),
            runtime_invocation: ComCapabilityStatus::unavailable(
                ComCapabilityFeature::RuntimeInvocation,
                "native COM service not configured; COM runtime invocation disabled.",
            ),
        }
    }

    pub fn future_windows_native_service_available(reference: ComReferenceFact) -> Self {
        Self {
            profile_name: String::from("windows-native-com-service-available"),
            host_kind: ComHostProfileKind::WindowsNativeServiceAvailable,
            reference,
            native_execution_available: true,
            native_com_service_configured: true,
            windows_native_host_required: false,
            reference_discovery: ComCapabilityStatus::available(
                ComCapabilityFeature::ReferenceDiscovery,
            ),
            runtime_invocation: ComCapabilityStatus::available(
                ComCapabilityFeature::RuntimeInvocation,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_role_depends_on_domain_vocabulary() {
        assert_eq!(
            OxideCoreRole::GuiNeutralOrchestration.depends_on_domain(),
            OxideDomainRole::HostIndependentIdeVocabulary
        );
    }

    #[test]
    fn open_clean_document_starts_not_dirty() {
        let state = DocumentLifecycleState::open_clean(
            "Option Explicit",
            LifecycleCapabilities::all_supported(),
        );

        assert_eq!(state.persisted_source(), "Option Explicit");
        assert_eq!(state.working_source(), "Option Explicit");
        assert!(!state.is_dirty());
    }

    #[test]
    fn edit_makes_document_dirty() {
        let mut state =
            DocumentLifecycleState::open_clean("before", LifecycleCapabilities::all_supported());

        state.edit_working_source("after");

        assert_eq!(state.persisted_source(), "before");
        assert_eq!(state.working_source(), "after");
        assert!(state.is_dirty());
        assert!(state.command_status(LifecycleCommand::Save).is_enabled);
        assert!(state.command_status(LifecycleCommand::Revert).is_enabled);
    }

    #[test]
    fn save_acknowledgement_makes_working_source_clean() {
        let mut state =
            DocumentLifecycleState::open_clean("before", LifecycleCapabilities::all_supported());
        state.edit_working_source("after");

        state.acknowledge_saved().expect("save supported");

        assert_eq!(state.persisted_source(), "after");
        assert_eq!(state.working_source(), "after");
        assert!(!state.is_dirty());
        assert!(!state.command_status(LifecycleCommand::Save).is_enabled);
    }

    #[test]
    fn reload_replaces_persisted_and_working_source() {
        let mut state =
            DocumentLifecycleState::open_clean("before", LifecycleCapabilities::all_supported());
        state.edit_working_source("dirty");

        state
            .reload_from_persisted("from host")
            .expect("reload supported");

        assert_eq!(state.persisted_source(), "from host");
        assert_eq!(state.working_source(), "from host");
        assert!(!state.is_dirty());
    }

    #[test]
    fn revert_restores_persisted_source_without_saving() {
        let mut state =
            DocumentLifecycleState::open_clean("before", LifecycleCapabilities::all_supported());
        state.edit_working_source("dirty");

        state.revert_to_persisted().expect("revert supported");

        assert_eq!(state.persisted_source(), "before");
        assert_eq!(state.working_source(), "before");
        assert!(!state.is_dirty());
    }

    #[test]
    fn unsupported_commands_report_disabled_reasons() {
        let mut state =
            DocumentLifecycleState::open_clean("before", LifecycleCapabilities::browser_limited());
        state.edit_working_source("dirty");

        let save = state
            .acknowledge_saved()
            .expect_err("browser save disabled");
        assert_eq!(save.command, LifecycleCommand::Save);
        assert!(save.reason.contains("browser-safe"));
        assert!(
            state
                .command_status(LifecycleCommand::Reload)
                .reason
                .expect("reload disabled reason")
                .contains("filesystem")
        );
        state
            .revert_to_persisted()
            .expect("local revert remains pure");
        assert_eq!(state.working_source(), "before");
    }

    #[test]
    fn in_memory_persistence_loads_and_saves_without_disk() {
        let mut persistence = InMemoryDocumentPersistence::new("before");
        let mut state = open_lifecycle_from_persistence(&persistence).expect("memory load");

        state.edit_working_source("after");
        save_lifecycle_to_persistence(&mut state, &mut persistence).expect("memory save");

        assert_eq!(persistence.persisted_source(), "after");
        assert_eq!(persistence.load().as_deref(), Ok("after"));
        assert!(!state.is_dirty());
    }

    #[test]
    fn browser_limited_persistence_reports_disabled_reasons() {
        let mut persistence = BrowserLimitedPersistence::new("snapshot");

        assert_eq!(persistence.source_snapshot(), "snapshot");
        let load = persistence.load().expect_err("browser load disabled");
        assert_eq!(
            load,
            PersistenceError::Disabled {
                operation: PersistenceOperation::Load,
                reason: String::from("browser-safe profile has no direct filesystem persistence")
            }
        );
        let save = persistence
            .save("after")
            .expect_err("browser save disabled");
        assert!(save.to_string().contains("filesystem persistence"));
        assert!(!persistence.capabilities().can_save);
    }

    #[test]
    fn lifecycle_and_persistence_keep_working_source_distinct_until_save() {
        let mut persistence = InMemoryDocumentPersistence::new("persisted");
        let mut state = open_lifecycle_from_persistence(&persistence).expect("memory load");

        state.edit_working_source("working");

        assert_eq!(state.persisted_source(), "persisted");
        assert_eq!(state.working_source(), "working");
        assert_eq!(persistence.persisted_source(), "persisted");

        save_lifecycle_to_persistence(&mut state, &mut persistence).expect("memory save");

        assert_eq!(state.persisted_source(), "working");
        assert_eq!(persistence.persisted_source(), "working");
    }

    #[test]
    fn session_snapshot_round_trips_through_json() {
        let mut state = DocumentLifecycleState::open_clean(
            "persisted source",
            LifecycleCapabilities::browser_limited(),
        );
        state.edit_working_source("working source");
        let snapshot = GuiSessionSnapshot::capture(
            "examples/thin-slice/ThinSliceHello.basproj",
            "Module1.bas",
            &state,
            SessionCapabilityProfile::BrowserLimited,
        );

        let encoded = serde_json::to_string(&snapshot).expect("serialize snapshot");
        let decoded: GuiSessionSnapshot =
            serde_json::from_str(&encoded).expect("deserialize snapshot");

        assert_eq!(decoded, snapshot);
        assert!(decoded.is_dirty());
    }

    #[test]
    fn restored_dirty_session_preserves_active_module_and_working_source() {
        let mut state = DocumentLifecycleState::open_clean(
            "persisted",
            LifecycleCapabilities::browser_limited(),
        );
        state.edit_working_source("working");
        let snapshot = GuiSessionSnapshot::capture(
            "examples/thin-slice/ThinSliceHello.basproj",
            "Module1.bas",
            &state,
            SessionCapabilityProfile::BrowserLimited,
        );

        let restored = snapshot.restore();

        assert_eq!(
            restored.workspace_path,
            "examples/thin-slice/ThinSliceHello.basproj"
        );
        assert_eq!(restored.active_module, "Module1.bas");
        assert_eq!(restored.capability_profile.label(), "browser-limited");
        assert_eq!(restored.document.persisted_source(), "persisted");
        assert_eq!(restored.document.working_source(), "working");
        assert!(restored.document.is_dirty());
    }

    #[test]
    fn restored_clean_session_does_not_invent_edits() {
        let state =
            DocumentLifecycleState::open_clean("same", LifecycleCapabilities::all_supported());
        let snapshot = GuiSessionSnapshot::capture(
            "examples/thin-slice/ThinSliceHello.basproj",
            "Module1.bas",
            &state,
            SessionCapabilityProfile::AllSupported,
        );

        let restored = snapshot.restore();

        assert!(!snapshot.is_dirty());
        assert_eq!(restored.document.persisted_source(), "same");
        assert_eq!(restored.document.working_source(), "same");
        assert!(!restored.document.is_dirty());
    }

    #[test]
    fn browser_safe_run_profile_reports_disabled_reason_without_native_claims() {
        let profile = RunCapabilityProfile::browser_safe_unsupported();
        let status = profile.command_status();

        assert_eq!(profile.profile_name, "browser-safe");
        assert_eq!(profile.provider_kind, RunProviderKind::BrowserUnsupported);
        assert!(!profile.can_run);
        assert!(!profile.native_execution_available);
        assert!(!profile.com_runtime_available);
        assert!(!status.is_enabled);
        assert!(
            status
                .reason
                .expect("disabled reason")
                .contains("native execution provider unavailable")
        );
    }

    #[test]
    fn simulated_run_profile_is_available_but_not_native_or_com() {
        let profile = RunCapabilityProfile::simulated_supported();
        let status = profile.command_status();

        assert_eq!(profile.provider_kind.label(), "simulated");
        assert!(profile.can_run);
        assert!(status.is_enabled);
        assert!(status.reason.is_none());
        assert!(!profile.native_execution_available);
        assert!(!profile.com_runtime_available);
    }

    #[test]
    fn future_native_run_profile_is_labeled_separately_from_simulated() {
        let profile = RunCapabilityProfile::future_native_supported();

        assert_eq!(profile.provider_kind.label(), "native");
        assert_eq!(profile.profile_name, "native-supported");
        assert!(profile.can_run);
        assert!(profile.native_execution_available);
        assert!(!profile.com_runtime_available);
    }

    #[test]
    fn run_request_constructs_display_target() {
        let request = RunRequest::new("ThinSliceHello", "Module1", "Main");

        assert_eq!(request.project_name, "ThinSliceHello");
        assert_eq!(request.module_name, "Module1");
        assert_eq!(request.entrypoint, "Main");
        assert_eq!(request.display_target(), "ThinSliceHello::Module1.Main");
    }

    #[test]
    fn browser_disabled_run_transcript_is_structured() {
        let request = RunRequest::new("ThinSliceHello", "Module1", "Main");

        let transcript = RunTranscript::browser_disabled(
            request,
            RunCapabilityProfile::browser_safe_unsupported(),
        );

        assert_eq!(transcript.provider_label, "browser-unsupported");
        assert_eq!(transcript.status, RunTranscriptStatus::Disabled);
        assert_eq!(transcript.status.label(), "disabled");
        assert_eq!(transcript.events.len(), 2);
        assert_eq!(transcript.events[0].kind, RunOutputEventKind::Lifecycle);
        assert_eq!(transcript.events[0].message, "run requested");
        assert_eq!(transcript.events[1].kind, RunOutputEventKind::Diagnostic);
        assert!(transcript.events[1].message.contains("Run disabled"));
        assert!(
            transcript.events[1]
                .message
                .contains("native execution provider unavailable")
        );
    }

    #[test]
    fn simulated_run_transcript_has_deterministic_event_order() {
        let request = RunRequest::new("ThinSliceHello", "Module1", "Main");

        let transcript = RunTranscript::simulated_completed(request);

        assert_eq!(transcript.provider_label, "simulated");
        assert_eq!(transcript.status, RunTranscriptStatus::Completed);
        assert_eq!(transcript.status.label(), "completed");
        let messages = transcript
            .events
            .iter()
            .map(|event| event.message.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            messages,
            vec![
                "run started",
                "simulated provider invoked ThinSliceHello::Module1.Main",
                "simulated output: Main completed with answer 42",
                "run completed"
            ]
        );
    }

    #[test]
    fn run_output_event_kinds_have_stable_labels() {
        assert_eq!(RunOutputEventKind::Lifecycle.label(), "lifecycle");
        assert_eq!(RunOutputEventKind::Activity.label(), "activity");
        assert_eq!(RunOutputEventKind::Diagnostic.label(), "diagnostic");
        assert_eq!(RunOutputEventKind::Output.label(), "output");
    }

    #[test]
    fn browser_com_profile_reports_unavailable_discovery_and_runtime() {
        let profile = ComCapabilityProfile::browser_unavailable(
            ComReferenceFact::scripting_dictionary_demo(),
        );

        assert_eq!(profile.host_kind, ComHostProfileKind::BrowserSafe);
        assert_eq!(profile.host_kind.label(), "browser-safe");
        assert_eq!(profile.reference.identifier, "Scripting.Dictionary");
        assert!(!profile.native_execution_available);
        assert!(!profile.native_com_service_configured);
        assert!(profile.windows_native_host_required);
        assert!(!profile.reference_discovery.is_available);
        assert!(!profile.runtime_invocation.is_available);
        assert!(
            profile
                .reference_discovery
                .reason
                .as_deref()
                .expect("discovery reason")
                .contains("COM discovery unavailable in browser-safe profile")
        );
        assert!(
            profile
                .runtime_invocation
                .reason
                .as_deref()
                .expect("runtime reason")
                .contains("pure browser/WASM cannot directly call Windows COM")
        );
    }

    #[test]
    fn non_windows_native_com_profile_keeps_native_execution_distinct_from_com() {
        let profile = ComCapabilityProfile::non_windows_native_unavailable(
            ComReferenceFact::scripting_dictionary_demo(),
        );

        assert_eq!(profile.host_kind, ComHostProfileKind::NonWindowsNative);
        assert_eq!(profile.host_kind.label(), "non-windows-native");
        assert!(profile.native_execution_available);
        assert!(!profile.native_com_service_configured);
        assert!(profile.windows_native_host_required);
        assert!(!profile.reference_discovery.is_available);
        assert!(!profile.runtime_invocation.is_available);
        assert!(
            profile
                .runtime_invocation
                .reason
                .as_deref()
                .expect("runtime reason")
                .contains("non-Windows native host")
        );
    }

    #[test]
    fn windows_native_service_missing_blocks_com_without_browser_reason() {
        let profile = ComCapabilityProfile::windows_native_service_missing(
            ComReferenceFact::scripting_dictionary_demo(),
        );

        assert_eq!(
            profile.host_kind,
            ComHostProfileKind::WindowsNativeServiceMissing
        );
        assert_eq!(profile.host_kind.label(), "windows-native-service-missing");
        assert!(profile.native_execution_available);
        assert!(!profile.native_com_service_configured);
        assert!(!profile.windows_native_host_required);
        assert!(!profile.reference_discovery.is_available);
        assert!(!profile.runtime_invocation.is_available);
        assert!(
            profile
                .reference_discovery
                .reason
                .as_deref()
                .expect("discovery reason")
                .contains("native COM service not configured")
        );
        assert!(
            !profile
                .runtime_invocation
                .reason
                .as_deref()
                .expect("runtime reason")
                .contains("browser-safe")
        );
    }

    #[test]
    fn future_windows_native_service_available_is_labeled_separately() {
        let profile = ComCapabilityProfile::future_windows_native_service_available(
            ComReferenceFact::scripting_dictionary_demo(),
        );

        assert_eq!(
            profile.host_kind,
            ComHostProfileKind::WindowsNativeServiceAvailable
        );
        assert_eq!(
            profile.host_kind.label(),
            "windows-native-service-available"
        );
        assert!(profile.native_execution_available);
        assert!(profile.native_com_service_configured);
        assert!(!profile.windows_native_host_required);
        assert!(profile.reference_discovery.is_available);
        assert!(profile.runtime_invocation.is_available);
        assert!(profile.reference_discovery.reason.is_none());
        assert!(profile.runtime_invocation.reason.is_none());
    }

    #[test]
    fn com_capability_feature_labels_are_stable() {
        assert_eq!(
            ComCapabilityFeature::ReferenceDiscovery.label(),
            "reference-discovery"
        );
        assert_eq!(
            ComCapabilityFeature::RuntimeInvocation.label(),
            "runtime-invocation"
        );
    }
}
