//! GUI-neutral OxIde application orchestration.
//!
//! This crate owns GUI-native state transitions above OxIde domain
//! vocabulary. It must not import parked TUI session/editor code.

use oxide_domain::OxideDomainRole;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeFilesystemDocumentPersistence {
    module_path: PathBuf,
}

impl NativeFilesystemDocumentPersistence {
    pub fn new(module_path: impl Into<PathBuf>) -> Self {
        Self {
            module_path: module_path.into(),
        }
    }

    pub fn module_path(&self) -> &Path {
        &self.module_path
    }

    pub fn provider_label(&self) -> &'static str {
        "native-filesystem"
    }

    fn io_error(&self, operation: PersistenceOperation, error: std::io::Error) -> PersistenceError {
        PersistenceError::Io {
            operation,
            path: self.module_path.display().to_string(),
            message: error.to_string(),
        }
    }
}

impl DocumentPersistence for NativeFilesystemDocumentPersistence {
    fn load(&self) -> Result<String, PersistenceError> {
        std::fs::read_to_string(&self.module_path)
            .map_err(|error| self.io_error(PersistenceOperation::Load, error))
    }

    fn save(&mut self, source: &str) -> Result<(), PersistenceError> {
        std::fs::write(&self.module_path, source)
            .map_err(|error| self.io_error(PersistenceOperation::Save, error))
    }

    fn capabilities(&self) -> LifecycleCapabilities {
        LifecycleCapabilities::all_supported()
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
    Io {
        operation: PersistenceOperation,
        path: String,
        message: String,
    },
}

impl std::fmt::Display for PersistenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Disabled { operation, reason } => write!(f, "{:?} disabled: {reason}", operation),
            Self::Io {
                operation,
                path,
                message,
            } => write!(f, "{:?} failed for {path}: {message}", operation),
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
pub struct RunTimeline {
    pub request: RunRequest,
    pub provider_label: String,
    pub status: RunTranscriptStatus,
    pub native_execution_available: bool,
    pub com_runtime_available: bool,
    pub entries: Vec<RunTimelineEntry>,
}

impl RunTimeline {
    pub fn from_transcript(transcript: &RunTranscript, profile: &RunCapabilityProfile) -> Self {
        Self {
            request: transcript.request.clone(),
            provider_label: transcript.provider_label.clone(),
            status: transcript.status,
            native_execution_available: profile.native_execution_available,
            com_runtime_available: profile.com_runtime_available,
            entries: transcript
                .events
                .iter()
                .enumerate()
                .map(|(index, event)| RunTimelineEntry {
                    index: index + 1,
                    kind: event.kind,
                    message: event.message.clone(),
                    provenance_label: transcript.provider_label.clone(),
                })
                .collect(),
        }
    }

    pub fn status_label(&self) -> &'static str {
        self.status.label()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunTimelineEntry {
    pub index: usize,
    pub kind: RunOutputEventKind,
    pub message: String,
    pub provenance_label: String,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimeSurfaceProfileKind {
    BrowserDisabled,
    NativeRuntimeRequired,
    FutureSupported,
}

impl RuntimeSurfaceProfileKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::BrowserDisabled => "browser-disabled",
            Self::NativeRuntimeRequired => "native-runtime-required",
            Self::FutureSupported => "future-supported",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeSurfaceCommandStatus {
    pub is_enabled: bool,
    pub reason: Option<String>,
}

impl RuntimeSurfaceCommandStatus {
    pub fn enabled() -> Self {
        Self {
            is_enabled: true,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImmediateCapabilityProfile {
    pub profile_name: String,
    pub kind: RuntimeSurfaceProfileKind,
    pub command_status: RuntimeSurfaceCommandStatus,
    pub native_runtime_required: bool,
    pub com_runtime_required: bool,
    pub fake_responses_available: bool,
}

impl ImmediateCapabilityProfile {
    pub fn browser_disabled() -> Self {
        Self {
            profile_name: String::from("immediate-browser-disabled"),
            kind: RuntimeSurfaceProfileKind::BrowserDisabled,
            command_status: RuntimeSurfaceCommandStatus::disabled(
                "Immediate disabled: browser-safe profile has no native OxVba runtime session.",
            ),
            native_runtime_required: true,
            com_runtime_required: false,
            fake_responses_available: false,
        }
    }

    pub fn native_runtime_required() -> Self {
        Self {
            profile_name: String::from("immediate-native-runtime-required"),
            kind: RuntimeSurfaceProfileKind::NativeRuntimeRequired,
            command_status: RuntimeSurfaceCommandStatus::disabled(
                "Immediate requires a native OxVba runtime session.",
            ),
            native_runtime_required: true,
            com_runtime_required: false,
            fake_responses_available: false,
        }
    }

    pub fn future_supported() -> Self {
        Self {
            profile_name: String::from("immediate-future-supported"),
            kind: RuntimeSurfaceProfileKind::FutureSupported,
            command_status: RuntimeSurfaceCommandStatus::enabled(),
            native_runtime_required: false,
            com_runtime_required: false,
            fake_responses_available: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugCapabilityProfile {
    pub profile_name: String,
    pub kind: RuntimeSurfaceProfileKind,
    pub command_status: RuntimeSurfaceCommandStatus,
    pub native_runtime_required: bool,
    pub com_runtime_required: bool,
    pub fake_debug_data_available: bool,
}

impl DebugCapabilityProfile {
    pub fn browser_disabled() -> Self {
        Self {
            profile_name: String::from("debug-browser-disabled"),
            kind: RuntimeSurfaceProfileKind::BrowserDisabled,
            command_status: RuntimeSurfaceCommandStatus::disabled(
                "Debug disabled: browser-safe profile has no OxVba debug session.",
            ),
            native_runtime_required: true,
            com_runtime_required: false,
            fake_debug_data_available: false,
        }
    }

    pub fn native_runtime_required() -> Self {
        Self {
            profile_name: String::from("debug-native-runtime-required"),
            kind: RuntimeSurfaceProfileKind::NativeRuntimeRequired,
            command_status: RuntimeSurfaceCommandStatus::disabled(
                "Debug requires a native OxVba runtime/debug session.",
            ),
            native_runtime_required: true,
            com_runtime_required: false,
            fake_debug_data_available: false,
        }
    }

    pub fn future_supported() -> Self {
        Self {
            profile_name: String::from("debug-future-supported"),
            kind: RuntimeSurfaceProfileKind::FutureSupported,
            command_status: RuntimeSurfaceCommandStatus::enabled(),
            native_runtime_required: false,
            com_runtime_required: false,
            fake_debug_data_available: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GuiCommandCategory {
    Project,
    Document,
    Runtime,
    Capability,
    Shell,
}

impl GuiCommandCategory {
    pub fn label(self) -> &'static str {
        match self {
            Self::Project => "project",
            Self::Document => "document",
            Self::Runtime => "runtime",
            Self::Capability => "capability",
            Self::Shell => "shell",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GuiCommandId {
    ProjectOpen,
    DocumentSave,
    DocumentReload,
    DocumentRevert,
    RuntimeRun,
    RuntimeStop,
    RuntimeImmediate,
    RuntimeDebug,
    CapabilityShowCom,
    ShellCommandPalette,
}

impl GuiCommandId {
    pub fn stable_id(self) -> &'static str {
        match self {
            Self::ProjectOpen => "project.open",
            Self::DocumentSave => "document.save",
            Self::DocumentReload => "document.reload",
            Self::DocumentRevert => "document.revert",
            Self::RuntimeRun => "runtime.run",
            Self::RuntimeStop => "runtime.stop",
            Self::RuntimeImmediate => "runtime.immediate",
            Self::RuntimeDebug => "runtime.debug",
            Self::CapabilityShowCom => "capability.show_com",
            Self::ShellCommandPalette => "shell.command_palette",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::ProjectOpen => "Open Project",
            Self::DocumentSave => "Save",
            Self::DocumentReload => "Reload",
            Self::DocumentRevert => "Revert",
            Self::RuntimeRun => "Run",
            Self::RuntimeStop => "Stop Run",
            Self::RuntimeImmediate => "Immediate",
            Self::RuntimeDebug => "Debug",
            Self::CapabilityShowCom => "Show COM Capability",
            Self::ShellCommandPalette => "Command Palette",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::ProjectOpen => "Open or switch an OxVba project in the IDE surface.",
            Self::DocumentSave => "Save the active module through the current persistence profile.",
            Self::DocumentReload => {
                "Reload the active module from the current persistence profile."
            }
            Self::DocumentRevert => "Revert the working source to the last persisted source.",
            Self::RuntimeRun => {
                "Run the selected entrypoint through the configured execution provider."
            }
            Self::RuntimeStop => "Stop the active run session when one exists.",
            Self::RuntimeImmediate => {
                "Use the Immediate surface when an authoritative runtime session exists."
            }
            Self::RuntimeDebug => {
                "Start or use the debug surface when an authoritative debug session exists."
            }
            Self::CapabilityShowCom => "Show COM capability and disabled-reason evidence.",
            Self::ShellCommandPalette => "Open the GUI command palette.",
        }
    }

    pub fn category(self) -> GuiCommandCategory {
        match self {
            Self::ProjectOpen => GuiCommandCategory::Project,
            Self::DocumentSave | Self::DocumentReload | Self::DocumentRevert => {
                GuiCommandCategory::Document
            }
            Self::RuntimeRun | Self::RuntimeStop | Self::RuntimeImmediate | Self::RuntimeDebug => {
                GuiCommandCategory::Runtime
            }
            Self::CapabilityShowCom => GuiCommandCategory::Capability,
            Self::ShellCommandPalette => GuiCommandCategory::Shell,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuiCommandAvailability {
    pub is_enabled: bool,
    pub disabled_reason: Option<String>,
    pub capability_label: String,
}

impl GuiCommandAvailability {
    pub fn enabled(capability_label: impl Into<String>) -> Self {
        Self {
            is_enabled: true,
            disabled_reason: None,
            capability_label: capability_label.into(),
        }
    }

    pub fn disabled(reason: impl Into<String>, capability_label: impl Into<String>) -> Self {
        Self {
            is_enabled: false,
            disabled_reason: Some(reason.into()),
            capability_label: capability_label.into(),
        }
    }

    fn from_lifecycle_status(
        status: LifecycleCommandStatus,
        capability_label: impl Into<String>,
    ) -> Self {
        let capability_label = capability_label.into();
        if status.is_enabled {
            Self::enabled(capability_label)
        } else {
            Self::disabled(
                status
                    .reason
                    .unwrap_or_else(|| String::from("command has no current effect")),
                capability_label,
            )
        }
    }

    fn from_run_profile(profile: &RunCapabilityProfile) -> Self {
        let status = profile.command_status();
        if status.is_enabled {
            Self::enabled(profile.provider_kind.label())
        } else {
            Self::disabled(
                status
                    .reason
                    .unwrap_or_else(|| String::from("run is unavailable in this host profile")),
                profile.provider_kind.label(),
            )
        }
    }

    fn from_runtime_surface(
        status: &RuntimeSurfaceCommandStatus,
        capability_label: impl Into<String>,
    ) -> Self {
        let capability_label = capability_label.into();
        if status.is_enabled {
            Self::enabled(capability_label)
        } else {
            Self::disabled(
                status
                    .reason
                    .clone()
                    .unwrap_or_else(|| String::from("runtime surface is unavailable")),
                capability_label,
            )
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuiCommand {
    pub id: GuiCommandId,
    pub stable_id: String,
    pub label: String,
    pub description: String,
    pub category: GuiCommandCategory,
    pub availability: GuiCommandAvailability,
}

impl GuiCommand {
    pub fn new(id: GuiCommandId, availability: GuiCommandAvailability) -> Self {
        Self {
            id,
            stable_id: id.stable_id().to_string(),
            label: id.label().to_string(),
            description: id.description().to_string(),
            category: id.category(),
            availability,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuiCommandPalette {
    pub commands: Vec<GuiCommand>,
    pub source_label: String,
    pub parked_tui_imported: bool,
}

impl GuiCommandPalette {
    pub fn browser_safe_baseline(document: &DocumentLifecycleState) -> Self {
        Self::with_profiles(
            document,
            "browser-limited",
            &RunCapabilityProfile::browser_safe_unsupported(),
            &ImmediateCapabilityProfile::browser_disabled(),
            &DebugCapabilityProfile::browser_disabled(),
            &ComCapabilityProfile::browser_unavailable(
                ComReferenceFact::scripting_dictionary_demo(),
            ),
        )
    }

    pub fn with_profiles(
        document: &DocumentLifecycleState,
        lifecycle_capability_label: impl Into<String>,
        run_profile: &RunCapabilityProfile,
        immediate_profile: &ImmediateCapabilityProfile,
        debug_profile: &DebugCapabilityProfile,
        com_profile: &ComCapabilityProfile,
    ) -> Self {
        let lifecycle_capability_label = lifecycle_capability_label.into();
        let commands = vec![
            GuiCommand::new(
                GuiCommandId::ProjectOpen,
                GuiCommandAvailability::enabled("project-open"),
            ),
            GuiCommand::new(
                GuiCommandId::DocumentSave,
                GuiCommandAvailability::from_lifecycle_status(
                    document.command_status(LifecycleCommand::Save),
                    lifecycle_capability_label.clone(),
                ),
            ),
            GuiCommand::new(
                GuiCommandId::DocumentReload,
                GuiCommandAvailability::from_lifecycle_status(
                    document.command_status(LifecycleCommand::Reload),
                    lifecycle_capability_label.clone(),
                ),
            ),
            GuiCommand::new(
                GuiCommandId::DocumentRevert,
                GuiCommandAvailability::from_lifecycle_status(
                    document.command_status(LifecycleCommand::Revert),
                    lifecycle_capability_label,
                ),
            ),
            GuiCommand::new(
                GuiCommandId::RuntimeRun,
                GuiCommandAvailability::from_run_profile(run_profile),
            ),
            GuiCommand::new(
                GuiCommandId::RuntimeStop,
                GuiCommandAvailability::disabled(
                    "no active runtime session to stop",
                    run_profile.provider_kind.label(),
                ),
            ),
            GuiCommand::new(
                GuiCommandId::RuntimeImmediate,
                GuiCommandAvailability::from_runtime_surface(
                    &immediate_profile.command_status,
                    immediate_profile.kind.label(),
                ),
            ),
            GuiCommand::new(
                GuiCommandId::RuntimeDebug,
                GuiCommandAvailability::from_runtime_surface(
                    &debug_profile.command_status,
                    debug_profile.kind.label(),
                ),
            ),
            GuiCommand::new(
                GuiCommandId::CapabilityShowCom,
                GuiCommandAvailability::enabled(com_profile.host_kind.label()),
            ),
            GuiCommand::new(
                GuiCommandId::ShellCommandPalette,
                GuiCommandAvailability::enabled("gui-shell"),
            ),
        ];

        Self {
            commands,
            source_label: String::from("gui-core command registry"),
            parked_tui_imported: false,
        }
    }

    pub fn command(&self, id: GuiCommandId) -> Option<&GuiCommand> {
        self.commands.iter().find(|command| command.id == id)
    }

    pub fn stable_ids(&self) -> Vec<&str> {
        self.commands
            .iter()
            .map(|command| command.stable_id.as_str())
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum GuiKeyboardContext {
    GlobalShell,
    ProjectTree,
    Editor,
    Diagnostics,
    RunOutput,
    Immediate,
    Debug,
    CommandPalette,
}

impl GuiKeyboardContext {
    pub fn label(self) -> &'static str {
        match self {
            Self::GlobalShell => "global-shell",
            Self::ProjectTree => "project-tree",
            Self::Editor => "editor",
            Self::Diagnostics => "diagnostics",
            Self::RunOutput => "run-output",
            Self::Immediate => "immediate",
            Self::Debug => "debug",
            Self::CommandPalette => "command-palette",
        }
    }

    pub fn display_label(self) -> &'static str {
        match self {
            Self::GlobalShell => "Global shell",
            Self::ProjectTree => "Project tree",
            Self::Editor => "Editor",
            Self::Diagnostics => "Diagnostics",
            Self::RunOutput => "Run output",
            Self::Immediate => "Immediate",
            Self::Debug => "Debug",
            Self::CommandPalette => "Command palette",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuiKeyboardContextDescriptor {
    pub context: GuiKeyboardContext,
    pub label: String,
}

impl GuiKeyboardContextDescriptor {
    pub fn new(context: GuiKeyboardContext) -> Self {
        Self {
            context,
            label: context.display_label().to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuiKeyGesture {
    pub display: String,
    pub normalized: String,
}

impl GuiKeyGesture {
    pub fn new(display: impl Into<String>) -> Self {
        let display = display.into();
        let normalized = display
            .chars()
            .filter(|character| !character.is_whitespace())
            .collect::<String>()
            .to_ascii_lowercase();
        Self {
            display,
            normalized,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuiKeyBinding {
    pub context: GuiKeyboardContext,
    pub command_id: GuiCommandId,
    pub command_stable_id: String,
    pub gesture: GuiKeyGesture,
    pub availability: GuiCommandAvailability,
    pub allow_same_gesture_in_distinct_contexts: bool,
}

impl GuiKeyBinding {
    pub fn new(
        context: GuiKeyboardContext,
        command: &GuiCommand,
        gesture: impl Into<String>,
        allow_same_gesture_in_distinct_contexts: bool,
    ) -> Self {
        Self {
            context,
            command_id: command.id,
            command_stable_id: command.stable_id.clone(),
            gesture: GuiKeyGesture::new(gesture),
            availability: command.availability.clone(),
            allow_same_gesture_in_distinct_contexts,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuiKeyBindingCollision {
    pub gesture: String,
    pub contexts: Vec<GuiKeyboardContext>,
    pub command_stable_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuiKeyboardMap {
    pub contexts: Vec<GuiKeyboardContextDescriptor>,
    pub bindings: Vec<GuiKeyBinding>,
    pub source_label: String,
    pub host_specific_overrides_required: bool,
}

impl GuiKeyboardMap {
    pub fn baseline(palette: &GuiCommandPalette) -> Self {
        let command = |id| palette.command(id).expect("baseline command exists");
        let contexts = vec![
            GuiKeyboardContextDescriptor::new(GuiKeyboardContext::GlobalShell),
            GuiKeyboardContextDescriptor::new(GuiKeyboardContext::ProjectTree),
            GuiKeyboardContextDescriptor::new(GuiKeyboardContext::Editor),
            GuiKeyboardContextDescriptor::new(GuiKeyboardContext::Diagnostics),
            GuiKeyboardContextDescriptor::new(GuiKeyboardContext::RunOutput),
            GuiKeyboardContextDescriptor::new(GuiKeyboardContext::Immediate),
            GuiKeyboardContextDescriptor::new(GuiKeyboardContext::Debug),
            GuiKeyboardContextDescriptor::new(GuiKeyboardContext::CommandPalette),
        ];
        let bindings = vec![
            GuiKeyBinding::new(
                GuiKeyboardContext::GlobalShell,
                command(GuiCommandId::ShellCommandPalette),
                "Ctrl+Shift+P",
                false,
            ),
            GuiKeyBinding::new(
                GuiKeyboardContext::GlobalShell,
                command(GuiCommandId::ProjectOpen),
                "Ctrl+O",
                false,
            ),
            GuiKeyBinding::new(
                GuiKeyboardContext::Editor,
                command(GuiCommandId::DocumentSave),
                "Ctrl+S",
                false,
            ),
            GuiKeyBinding::new(
                GuiKeyboardContext::Editor,
                command(GuiCommandId::DocumentReload),
                "Ctrl+Alt+R",
                false,
            ),
            GuiKeyBinding::new(
                GuiKeyboardContext::Editor,
                command(GuiCommandId::DocumentRevert),
                "Ctrl+Alt+Backspace",
                false,
            ),
            GuiKeyBinding::new(
                GuiKeyboardContext::GlobalShell,
                command(GuiCommandId::RuntimeRun),
                "F5",
                false,
            ),
            GuiKeyBinding::new(
                GuiKeyboardContext::GlobalShell,
                command(GuiCommandId::RuntimeStop),
                "Shift+F5",
                false,
            ),
            GuiKeyBinding::new(
                GuiKeyboardContext::ProjectTree,
                command(GuiCommandId::ProjectOpen),
                "Enter",
                true,
            ),
            GuiKeyBinding::new(
                GuiKeyboardContext::Immediate,
                command(GuiCommandId::RuntimeImmediate),
                "Enter",
                true,
            ),
            GuiKeyBinding::new(
                GuiKeyboardContext::Debug,
                command(GuiCommandId::RuntimeDebug),
                "F10",
                false,
            ),
            GuiKeyBinding::new(
                GuiKeyboardContext::CommandPalette,
                command(GuiCommandId::ShellCommandPalette),
                "Escape",
                false,
            ),
        ];

        Self {
            contexts,
            bindings,
            source_label: String::from("gui-core keyboard map"),
            host_specific_overrides_required: false,
        }
    }

    pub fn context_collisions(&self) -> Vec<GuiKeyBindingCollision> {
        let mut groups: std::collections::BTreeMap<
            (GuiKeyboardContext, String),
            Vec<&GuiKeyBinding>,
        > = std::collections::BTreeMap::new();
        for binding in &self.bindings {
            groups
                .entry((binding.context, binding.gesture.normalized.clone()))
                .or_default()
                .push(binding);
        }
        groups
            .into_iter()
            .filter_map(|((context, gesture), bindings)| {
                if bindings.len() > 1 {
                    Some(GuiKeyBindingCollision {
                        gesture,
                        contexts: vec![context],
                        command_stable_ids: bindings
                            .iter()
                            .map(|binding| binding.command_stable_id.clone())
                            .collect(),
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn disallowed_cross_context_collisions(&self) -> Vec<GuiKeyBindingCollision> {
        let mut groups: std::collections::BTreeMap<String, Vec<&GuiKeyBinding>> =
            std::collections::BTreeMap::new();
        for binding in &self.bindings {
            groups
                .entry(binding.gesture.normalized.clone())
                .or_default()
                .push(binding);
        }
        groups
            .into_iter()
            .filter_map(|(gesture, bindings)| {
                let contexts = bindings
                    .iter()
                    .map(|binding| binding.context)
                    .collect::<std::collections::BTreeSet<_>>()
                    .into_iter()
                    .collect::<Vec<_>>();
                let crosses_contexts = contexts.len() > 1;
                let allowed = bindings
                    .iter()
                    .all(|binding| binding.allow_same_gesture_in_distinct_contexts);
                if crosses_contexts && !allowed {
                    Some(GuiKeyBindingCollision {
                        gesture,
                        contexts,
                        command_stable_ids: bindings
                            .iter()
                            .map(|binding| binding.command_stable_id.clone())
                            .collect(),
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn binding_for(&self, command_id: GuiCommandId) -> Option<&GuiKeyBinding> {
        self.bindings
            .iter()
            .find(|binding| binding.command_id == command_id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GuiFocusNodeKind {
    ProjectTree,
    Editor,
    Diagnostics,
    LifecycleControls,
    RunOutput,
    Immediate,
    Debug,
    ComCapability,
    CommandPalette,
}

impl GuiFocusNodeKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::ProjectTree => "project-tree",
            Self::Editor => "editor",
            Self::Diagnostics => "diagnostics",
            Self::LifecycleControls => "lifecycle-controls",
            Self::RunOutput => "run-output",
            Self::Immediate => "immediate",
            Self::Debug => "debug",
            Self::ComCapability => "com-capability",
            Self::CommandPalette => "command-palette",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuiFocusNode {
    pub node_id: String,
    pub kind: GuiFocusNodeKind,
    pub label: String,
    pub focusable: bool,
    pub disabled_reason_visible: bool,
    pub restore_target: Option<String>,
}

impl GuiFocusNode {
    pub fn new(
        node_id: impl Into<String>,
        kind: GuiFocusNodeKind,
        label: impl Into<String>,
        disabled_reason_visible: bool,
        restore_target: Option<String>,
    ) -> Self {
        Self {
            node_id: node_id.into(),
            kind,
            label: label.into(),
            focusable: true,
            disabled_reason_visible,
            restore_target,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuiFocusRouteStep {
    pub index: usize,
    pub node_id: String,
    pub restoration_hint: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuiFocusGraph {
    pub nodes: Vec<GuiFocusNode>,
    pub no_mouse_route: Vec<GuiFocusRouteStep>,
    pub source_label: String,
}

impl GuiFocusGraph {
    pub fn baseline(palette: &GuiCommandPalette) -> Self {
        let command_disabled = |id| {
            palette
                .command(id)
                .map(|command| command.availability.disabled_reason.is_some())
                .unwrap_or(false)
        };
        let lifecycle_disabled = [
            GuiCommandId::DocumentSave,
            GuiCommandId::DocumentReload,
            GuiCommandId::DocumentRevert,
        ]
        .into_iter()
        .any(command_disabled);
        let nodes = vec![
            GuiFocusNode::new(
                "project-tree",
                GuiFocusNodeKind::ProjectTree,
                "Project tree",
                false,
                None,
            ),
            GuiFocusNode::new(
                "source-editor",
                GuiFocusNodeKind::Editor,
                "Source editor",
                false,
                None,
            ),
            GuiFocusNode::new(
                "diagnostics-panel",
                GuiFocusNodeKind::Diagnostics,
                "Diagnostics",
                false,
                None,
            ),
            GuiFocusNode::new(
                "lifecycle-controls",
                GuiFocusNodeKind::LifecycleControls,
                "Lifecycle controls",
                lifecycle_disabled,
                None,
            ),
            GuiFocusNode::new(
                "run-output",
                GuiFocusNodeKind::RunOutput,
                "Run output",
                command_disabled(GuiCommandId::RuntimeRun),
                None,
            ),
            GuiFocusNode::new(
                "immediate-panel",
                GuiFocusNodeKind::Immediate,
                "Immediate",
                command_disabled(GuiCommandId::RuntimeImmediate),
                None,
            ),
            GuiFocusNode::new(
                "debug-panel",
                GuiFocusNodeKind::Debug,
                "Debug",
                command_disabled(GuiCommandId::RuntimeDebug),
                None,
            ),
            GuiFocusNode::new(
                "com-capability",
                GuiFocusNodeKind::ComCapability,
                "COM capability",
                true,
                None,
            ),
            GuiFocusNode::new(
                "command-palette",
                GuiFocusNodeKind::CommandPalette,
                "Command palette",
                false,
                Some(String::from("source-editor")),
            ),
        ];
        let route_ids = [
            "project-tree",
            "source-editor",
            "diagnostics-panel",
            "lifecycle-controls",
            "run-output",
            "immediate-panel",
            "debug-panel",
            "com-capability",
            "command-palette",
            "source-editor",
        ];
        let no_mouse_route = route_ids
            .iter()
            .enumerate()
            .map(|(index, node_id)| GuiFocusRouteStep {
                index: index + 1,
                node_id: (*node_id).to_string(),
                restoration_hint: if *node_id == "command-palette" {
                    Some(String::from("returns to source-editor"))
                } else {
                    None
                },
            })
            .collect();

        Self {
            nodes,
            no_mouse_route,
            source_label: String::from("gui-core focus graph"),
        }
    }

    pub fn node(&self, node_id: &str) -> Option<&GuiFocusNode> {
        self.nodes.iter().find(|node| node.node_id == node_id)
    }

    pub fn route_node_ids(&self) -> Vec<&str> {
        self.no_mouse_route
            .iter()
            .map(|step| step.node_id.as_str())
            .collect()
    }

    pub fn disabled_focusable_node_ids(&self) -> Vec<&str> {
        self.nodes
            .iter()
            .filter(|node| node.focusable && node.disabled_reason_visible)
            .map(|node| node.node_id.as_str())
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GuiAccessibleSurfaceRole {
    ProjectTree,
    Editor,
    Diagnostics,
    LifecycleControls,
    RunOutput,
    Immediate,
    Debug,
    ComCapability,
    CommandPalette,
    CapabilityFooter,
}

impl GuiAccessibleSurfaceRole {
    pub fn label(self) -> &'static str {
        match self {
            Self::ProjectTree => "project-tree",
            Self::Editor => "editor",
            Self::Diagnostics => "diagnostics",
            Self::LifecycleControls => "lifecycle-controls",
            Self::RunOutput => "run-output",
            Self::Immediate => "immediate",
            Self::Debug => "debug",
            Self::ComCapability => "com-capability",
            Self::CommandPalette => "command-palette",
            Self::CapabilityFooter => "capability-footer",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuiAccessibilityNode {
    pub surface_id: String,
    pub role: GuiAccessibleSurfaceRole,
    pub accessible_label: String,
    pub accessible_description: String,
    pub disabled_reason: Option<String>,
}

impl GuiAccessibilityNode {
    pub fn new(
        surface_id: impl Into<String>,
        role: GuiAccessibleSurfaceRole,
        accessible_label: impl Into<String>,
        accessible_description: impl Into<String>,
        disabled_reason: Option<String>,
    ) -> Self {
        Self {
            surface_id: surface_id.into(),
            role,
            accessible_label: accessible_label.into(),
            accessible_description: accessible_description.into(),
            disabled_reason,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuiAccessibilityProjection {
    pub nodes: Vec<GuiAccessibilityNode>,
    pub source_label: String,
    pub web_framework_bound: bool,
}

impl GuiAccessibilityProjection {
    pub fn baseline(palette: &GuiCommandPalette) -> Self {
        let command_reason = |id| {
            palette
                .command(id)
                .and_then(|command| command.availability.disabled_reason.clone())
        };
        let lifecycle_reason = [
            GuiCommandId::DocumentSave,
            GuiCommandId::DocumentReload,
            GuiCommandId::DocumentRevert,
        ]
        .into_iter()
        .filter_map(command_reason)
        .collect::<Vec<_>>()
        .join("; ");
        let com_profile = ComCapabilityProfile::browser_unavailable(
            ComReferenceFact::scripting_dictionary_demo(),
        );
        let com_reason = format!(
            "{} {}",
            com_profile
                .reference_discovery
                .reason
                .clone()
                .unwrap_or_else(|| String::from("COM discovery unavailable.")),
            com_profile
                .runtime_invocation
                .reason
                .clone()
                .unwrap_or_else(|| String::from("COM runtime unavailable."))
        );

        let nodes = vec![
            GuiAccessibilityNode::new(
                "project-tree",
                GuiAccessibleSurfaceRole::ProjectTree,
                "Project tree",
                "Navigate modules in the active OxVba project.",
                None,
            ),
            GuiAccessibilityNode::new(
                "source-editor",
                GuiAccessibleSurfaceRole::Editor,
                "Source editor",
                "Edit the active VBA module source.",
                None,
            ),
            GuiAccessibilityNode::new(
                "diagnostics-panel",
                GuiAccessibleSurfaceRole::Diagnostics,
                "Diagnostics",
                "Review OxVba language-service diagnostics for the active source.",
                None,
            ),
            GuiAccessibilityNode::new(
                "lifecycle-controls",
                GuiAccessibleSurfaceRole::LifecycleControls,
                "Document lifecycle controls",
                "Save, reload, or revert the active document through the current persistence profile.",
                if lifecycle_reason.is_empty() {
                    None
                } else {
                    Some(lifecycle_reason)
                },
            ),
            GuiAccessibilityNode::new(
                "run-output",
                GuiAccessibleSurfaceRole::RunOutput,
                "Run output",
                "Run transcript and output events for the selected entrypoint.",
                command_reason(GuiCommandId::RuntimeRun),
            ),
            GuiAccessibilityNode::new(
                "immediate-panel",
                GuiAccessibleSurfaceRole::Immediate,
                "Immediate window",
                "Immediate execution surface for an authoritative OxVba runtime session.",
                command_reason(GuiCommandId::RuntimeImmediate),
            ),
            GuiAccessibilityNode::new(
                "debug-panel",
                GuiAccessibleSurfaceRole::Debug,
                "Debug panel",
                "Debug session surface for an authoritative OxVba debug adapter.",
                command_reason(GuiCommandId::RuntimeDebug),
            ),
            GuiAccessibilityNode::new(
                "com-capability",
                GuiAccessibleSurfaceRole::ComCapability,
                "COM capability",
                "COM reference discovery and runtime invocation capability status.",
                Some(com_reason),
            ),
            GuiAccessibilityNode::new(
                "command-palette",
                GuiAccessibleSurfaceRole::CommandPalette,
                "Command palette",
                "Search GUI commands and hear disabled reasons before activation.",
                None,
            ),
            GuiAccessibilityNode::new(
                "capability-footer",
                GuiAccessibleSurfaceRole::CapabilityFooter,
                "Host capability summary",
                "Browser-safe profile: editing and semantic projection available; native execution and COM unavailable.",
                Some(String::from(
                    "native execution and COM are unavailable in the browser-safe profile",
                )),
            ),
        ];

        Self {
            nodes,
            source_label: String::from("gui-core accessibility projection"),
            web_framework_bound: false,
        }
    }

    pub fn node(&self, surface_id: &str) -> Option<&GuiAccessibilityNode> {
        self.nodes.iter().find(|node| node.surface_id == surface_id)
    }

    pub fn disabled_reason_surface_ids(&self) -> Vec<&str> {
        self.nodes
            .iter()
            .filter(|node| node.disabled_reason.is_some())
            .map(|node| node.surface_id.as_str())
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuiShellModuleSummary {
    pub display_name: String,
    pub is_active: bool,
}

impl GuiShellModuleSummary {
    pub fn new(display_name: impl Into<String>, is_active: bool) -> Self {
        Self {
            display_name: display_name.into(),
            is_active,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuiShellDiagnosticSummary {
    pub severity_label: String,
    pub message: String,
    pub provenance_label: String,
}

impl GuiShellDiagnosticSummary {
    pub fn new(
        severity_label: impl Into<String>,
        message: impl Into<String>,
        provenance_label: impl Into<String>,
    ) -> Self {
        Self {
            severity_label: severity_label.into(),
            message: message.into(),
            provenance_label: provenance_label.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuiShellLifecycleCommandSummary {
    pub command_name: String,
    pub is_enabled: bool,
    pub disabled_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuiShellPacket {
    pub workspace_path: String,
    pub project_name: String,
    pub modules: Vec<GuiShellModuleSummary>,
    pub active_module: String,
    pub source_text: String,
    pub diagnostics: Vec<GuiShellDiagnosticSummary>,
    pub lifecycle_profile_label: String,
    pub lifecycle_commands: Vec<GuiShellLifecycleCommandSummary>,
    pub run_capability: RunCapabilityProfile,
    pub run_transcript: RunTranscript,
    pub run_timeline: RunTimeline,
    pub com_capability: ComCapabilityProfile,
    pub command_palette: GuiCommandPalette,
    pub keyboard_map: GuiKeyboardMap,
    pub focus_graph: GuiFocusGraph,
    pub accessibility: GuiAccessibilityProjection,
    pub capability_footer: String,
    pub parked_tui_imported: bool,
    pub native_execution_claimed: bool,
    pub com_runtime_claimed: bool,
    pub web_framework_bound: bool,
}

impl GuiShellPacket {
    pub fn browser_safe_baseline(
        workspace_path: impl Into<String>,
        project_name: impl Into<String>,
        modules: Vec<GuiShellModuleSummary>,
        active_module: impl Into<String>,
        active_module_stem: impl Into<String>,
        source_text: impl Into<String>,
        diagnostics: Vec<GuiShellDiagnosticSummary>,
    ) -> Self {
        let project_name = project_name.into();
        let source_text = source_text.into();
        let document = DocumentLifecycleState::open_clean(
            source_text.clone(),
            LifecycleCapabilities::browser_limited(),
        );
        let run_capability = RunCapabilityProfile::browser_safe_unsupported();
        let request = RunRequest::new(project_name.clone(), active_module_stem, "Main");
        let run_transcript = RunTranscript::browser_disabled(request, run_capability.clone());
        let run_timeline = RunTimeline::from_transcript(&run_transcript, &run_capability);
        let com_capability = ComCapabilityProfile::browser_unavailable(
            ComReferenceFact::scripting_dictionary_demo(),
        );
        let command_palette = GuiCommandPalette::browser_safe_baseline(&document);
        let keyboard_map = GuiKeyboardMap::baseline(&command_palette);
        let focus_graph = GuiFocusGraph::baseline(&command_palette);
        let accessibility = GuiAccessibilityProjection::baseline(&command_palette);
        let lifecycle_commands = [
            LifecycleCommand::Save,
            LifecycleCommand::Reload,
            LifecycleCommand::Revert,
        ]
        .into_iter()
        .map(|command| {
            let status = document.command_status(command);
            GuiShellLifecycleCommandSummary {
                command_name: lifecycle_command_stable_name(command).to_string(),
                is_enabled: status.is_enabled,
                disabled_reason: status.reason,
            }
        })
        .collect();
        let com_runtime_claimed = com_capability.runtime_invocation.is_available
            || run_capability.com_runtime_available
            || run_timeline.com_runtime_available;
        let native_execution_claimed =
            run_capability.native_execution_available || run_timeline.native_execution_available;

        Self {
            workspace_path: workspace_path.into(),
            project_name,
            modules,
            active_module: active_module.into(),
            source_text,
            diagnostics,
            lifecycle_profile_label: String::from("browser-limited"),
            lifecycle_commands,
            run_capability,
            run_transcript,
            run_timeline,
            com_capability,
            parked_tui_imported: command_palette.parked_tui_imported,
            web_framework_bound: accessibility.web_framework_bound,
            command_palette,
            keyboard_map,
            focus_graph,
            accessibility,
            capability_footer: String::from(
                "Browser-safe profile: editing and semantic projection available; native execution and COM unavailable.",
            ),
            native_execution_claimed,
            com_runtime_claimed,
        }
    }
}

fn lifecycle_command_stable_name(command: LifecycleCommand) -> &'static str {
    match command {
        LifecycleCommand::Save => "save",
        LifecycleCommand::Reload => "reload",
        LifecycleCommand::Revert => "revert",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn thin_slice_fixture_module_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("examples")
            .join("thin-slice")
            .join("Module1.bas")
    }

    fn unique_temp_project_dir(test_name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time after epoch")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!(
            "oxide-w320-{test_name}-{}-{unique}",
            std::process::id()
        ));
        fs::create_dir_all(&dir).expect("create test-owned temp project dir");
        dir
    }

    fn copy_thin_slice_module_to_temp(test_name: &str) -> (PathBuf, String, PathBuf) {
        let fixture_path = thin_slice_fixture_module_path();
        let fixture_source = fs::read_to_string(&fixture_path).expect("read checked-in fixture");
        let temp_dir = unique_temp_project_dir(test_name);
        let module_path = temp_dir.join("Module1.bas");
        fs::write(&module_path, &fixture_source).expect("copy fixture to test-owned temp project");
        (module_path, fixture_source, fixture_path)
    }

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
        assert!(!persistence.capabilities().can_reload);
    }

    #[test]
    fn native_filesystem_persistence_writes_and_reloads_test_owned_project_copy() {
        let (module_path, fixture_source, fixture_path) =
            copy_thin_slice_module_to_temp("native-write-reload");
        let fixture_before = fs::read_to_string(&fixture_path).expect("read fixture before save");
        let mut persistence = NativeFilesystemDocumentPersistence::new(&module_path);

        assert_eq!(persistence.provider_label(), "native-filesystem");
        assert_eq!(persistence.module_path(), module_path.as_path());
        assert_eq!(persistence.load().as_deref(), Ok(fixture_source.as_str()));
        assert!(persistence.capabilities().can_save);
        assert!(persistence.capabilities().can_reload);

        let edited = fixture_source.replace("answer = 40 + 2", "answer = 41 + 1");
        persistence.save(&edited).expect("native save to temp copy");

        assert_eq!(persistence.load().as_deref(), Ok(edited.as_str()));
        assert_eq!(
            fs::read_to_string(&module_path).expect("read temp module after save"),
            edited
        );
        assert_eq!(
            fs::read_to_string(&fixture_path).expect("read checked-in fixture after save"),
            fixture_before,
            "native persistence test must not mutate the checked-in thin-slice fixture"
        );
    }

    #[test]
    fn native_filesystem_save_lifecycle_acknowledges_disk_write_without_mutating_fixture() {
        let (module_path, fixture_source, fixture_path) =
            copy_thin_slice_module_to_temp("native-lifecycle-save");
        let fixture_before = fs::read_to_string(&fixture_path).expect("read fixture before save");
        let mut persistence = NativeFilesystemDocumentPersistence::new(&module_path);
        let mut state = open_lifecycle_from_persistence(&persistence).expect("native load");

        let edited = fixture_source.replace("Dim answer As Integer", "Dim answer As Long");
        state.edit_working_source(edited.clone());
        assert!(state.is_dirty());

        save_lifecycle_to_persistence(&mut state, &mut persistence).expect("native lifecycle save");

        assert!(!state.is_dirty());
        assert_eq!(state.persisted_source(), edited);
        assert_eq!(state.working_source(), edited);
        assert_eq!(persistence.load().as_deref(), Ok(edited.as_str()));
        assert_eq!(
            fs::read_to_string(&fixture_path)
                .expect("read checked-in fixture after lifecycle save"),
            fixture_before,
            "native lifecycle save must only write the test-owned temp project copy"
        );
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
    fn simulated_run_timeline_preserves_event_order_and_provider() {
        let profile = RunCapabilityProfile::simulated_supported();
        let request = RunRequest::new("ThinSliceHello", "Module1", "Main");
        let transcript = RunTranscript::simulated_completed(request);

        let timeline = RunTimeline::from_transcript(&transcript, &profile);

        assert_eq!(timeline.provider_label, "simulated");
        assert_eq!(timeline.status_label(), "completed");
        assert!(!timeline.native_execution_available);
        assert!(!timeline.com_runtime_available);
        assert_eq!(
            timeline.request.display_target(),
            "ThinSliceHello::Module1.Main"
        );
        assert_eq!(timeline.entries.len(), 4);
        assert_eq!(timeline.entries[0].index, 1);
        assert_eq!(timeline.entries[0].kind, RunOutputEventKind::Lifecycle);
        assert_eq!(timeline.entries[0].message, "run started");
        assert_eq!(timeline.entries[1].index, 2);
        assert!(
            timeline.entries[1]
                .message
                .contains("simulated provider invoked ThinSliceHello::Module1.Main")
        );
        assert_eq!(timeline.entries[2].kind, RunOutputEventKind::Output);
        assert!(timeline.entries[2].message.contains("answer 42"));
        assert!(
            timeline
                .entries
                .iter()
                .all(|entry| entry.provenance_label == "simulated")
        );
    }

    #[test]
    fn browser_disabled_run_timeline_preserves_disabled_reason() {
        let profile = RunCapabilityProfile::browser_safe_unsupported();
        let request = RunRequest::new("ThinSliceHello", "Module1", "Main");
        let transcript = RunTranscript::browser_disabled(request, profile.clone());

        let timeline = RunTimeline::from_transcript(&transcript, &profile);

        assert_eq!(timeline.provider_label, "browser-unsupported");
        assert_eq!(timeline.status_label(), "disabled");
        assert!(!timeline.native_execution_available);
        assert!(!timeline.com_runtime_available);
        assert_eq!(timeline.entries.len(), 2);
        assert_eq!(timeline.entries[0].message, "run requested");
        assert_eq!(timeline.entries[1].kind, RunOutputEventKind::Diagnostic);
        assert!(
            timeline.entries[1]
                .message
                .contains("native execution provider unavailable")
        );
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

    #[test]
    fn immediate_browser_disabled_reports_native_runtime_session_unavailable() {
        let profile = ImmediateCapabilityProfile::browser_disabled();

        assert_eq!(profile.kind, RuntimeSurfaceProfileKind::BrowserDisabled);
        assert_eq!(profile.kind.label(), "browser-disabled");
        assert!(!profile.command_status.is_enabled);
        assert!(profile.native_runtime_required);
        assert!(!profile.com_runtime_required);
        assert!(!profile.fake_responses_available);
        assert!(
            profile
                .command_status
                .reason
                .as_deref()
                .expect("Immediate disabled reason")
                .contains("no native OxVba runtime session")
        );
    }

    #[test]
    fn debug_browser_disabled_reports_debug_session_unavailable_without_fake_data() {
        let profile = DebugCapabilityProfile::browser_disabled();

        assert_eq!(profile.kind, RuntimeSurfaceProfileKind::BrowserDisabled);
        assert!(!profile.command_status.is_enabled);
        assert!(profile.native_runtime_required);
        assert!(!profile.com_runtime_required);
        assert!(!profile.fake_debug_data_available);
        assert!(
            profile
                .command_status
                .reason
                .as_deref()
                .expect("debug disabled reason")
                .contains("no OxVba debug session")
        );
    }

    #[test]
    fn native_runtime_required_profiles_are_distinct_from_future_supported() {
        let immediate = ImmediateCapabilityProfile::native_runtime_required();
        let debug = DebugCapabilityProfile::native_runtime_required();

        assert_eq!(immediate.kind.label(), "native-runtime-required");
        assert_eq!(debug.kind.label(), "native-runtime-required");
        assert!(!immediate.command_status.is_enabled);
        assert!(!debug.command_status.is_enabled);
        assert!(immediate.native_runtime_required);
        assert!(debug.native_runtime_required);
        assert!(
            immediate
                .command_status
                .reason
                .as_deref()
                .expect("Immediate native reason")
                .contains("native OxVba runtime session")
        );
        assert!(
            debug
                .command_status
                .reason
                .as_deref()
                .expect("debug native reason")
                .contains("native OxVba runtime/debug session")
        );
    }

    #[test]
    fn future_supported_runtime_surface_profiles_do_not_imply_com_or_fake_data() {
        let immediate = ImmediateCapabilityProfile::future_supported();
        let debug = DebugCapabilityProfile::future_supported();

        assert_eq!(immediate.kind.label(), "future-supported");
        assert_eq!(debug.kind.label(), "future-supported");
        assert!(immediate.command_status.is_enabled);
        assert!(debug.command_status.is_enabled);
        assert!(!immediate.native_runtime_required);
        assert!(!debug.native_runtime_required);
        assert!(!immediate.com_runtime_required);
        assert!(!debug.com_runtime_required);
        assert!(!immediate.fake_responses_available);
        assert!(!debug.fake_debug_data_available);
    }

    #[test]
    fn command_palette_baseline_has_stable_command_ids_without_tui_import() {
        let document = DocumentLifecycleState::open_clean(
            "Option Explicit",
            LifecycleCapabilities::browser_limited(),
        );

        let palette = GuiCommandPalette::browser_safe_baseline(&document);

        assert_eq!(
            palette.stable_ids(),
            vec![
                "project.open",
                "document.save",
                "document.reload",
                "document.revert",
                "runtime.run",
                "runtime.stop",
                "runtime.immediate",
                "runtime.debug",
                "capability.show_com",
                "shell.command_palette",
            ]
        );
        assert_eq!(palette.source_label, "gui-core command registry");
        assert!(!palette.parked_tui_imported);
        assert_eq!(
            palette
                .command(GuiCommandId::RuntimeRun)
                .expect("run command")
                .category
                .label(),
            "runtime"
        );
    }

    #[test]
    fn browser_safe_command_palette_surfaces_disabled_reasons() {
        let document = DocumentLifecycleState::open_clean(
            "Option Explicit",
            LifecycleCapabilities::browser_limited(),
        );

        let palette = GuiCommandPalette::browser_safe_baseline(&document);

        let save = palette
            .command(GuiCommandId::DocumentSave)
            .expect("save command");
        assert!(!save.availability.is_enabled);
        assert!(
            save.availability
                .disabled_reason
                .as_deref()
                .expect("save disabled reason")
                .contains("filesystem persistence")
        );

        let run = palette
            .command(GuiCommandId::RuntimeRun)
            .expect("run command");
        assert!(!run.availability.is_enabled);
        assert_eq!(run.availability.capability_label, "browser-unsupported");
        assert!(
            run.availability
                .disabled_reason
                .as_deref()
                .expect("run disabled reason")
                .contains("native execution provider unavailable")
        );

        let immediate = palette
            .command(GuiCommandId::RuntimeImmediate)
            .expect("Immediate command");
        assert!(!immediate.availability.is_enabled);
        assert_eq!(immediate.availability.capability_label, "browser-disabled");
        assert!(
            immediate
                .availability
                .disabled_reason
                .as_deref()
                .expect("Immediate disabled reason")
                .contains("no native OxVba runtime session")
        );

        let debug = palette
            .command(GuiCommandId::RuntimeDebug)
            .expect("debug command");
        assert!(!debug.availability.is_enabled);
        assert!(
            debug
                .availability
                .disabled_reason
                .as_deref()
                .expect("debug disabled reason")
                .contains("no OxVba debug session")
        );

        let stop = palette
            .command(GuiCommandId::RuntimeStop)
            .expect("stop command");
        assert!(!stop.availability.is_enabled);
        assert_eq!(
            stop.availability.disabled_reason.as_deref(),
            Some("no active runtime session to stop")
        );

        assert!(
            palette
                .command(GuiCommandId::CapabilityShowCom)
                .expect("COM command")
                .availability
                .is_enabled
        );
        assert!(
            palette
                .command(GuiCommandId::ShellCommandPalette)
                .expect("palette command")
                .availability
                .is_enabled
        );
    }

    #[test]
    fn simulated_run_command_remains_labeled_simulated() {
        let document = DocumentLifecycleState::open_clean(
            "Option Explicit",
            LifecycleCapabilities::all_supported(),
        );

        let palette = GuiCommandPalette::with_profiles(
            &document,
            "all-supported",
            &RunCapabilityProfile::simulated_supported(),
            &ImmediateCapabilityProfile::browser_disabled(),
            &DebugCapabilityProfile::browser_disabled(),
            &ComCapabilityProfile::browser_unavailable(
                ComReferenceFact::scripting_dictionary_demo(),
            ),
        );

        let run = palette
            .command(GuiCommandId::RuntimeRun)
            .expect("run command");
        assert!(run.availability.is_enabled);
        assert_eq!(run.availability.capability_label, "simulated");
        assert!(run.availability.disabled_reason.is_none());
    }

    #[test]
    fn keyboard_contexts_are_explicit_and_default_map_has_no_collisions() {
        let document = DocumentLifecycleState::open_clean(
            "Option Explicit",
            LifecycleCapabilities::browser_limited(),
        );
        let palette = GuiCommandPalette::browser_safe_baseline(&document);
        let keyboard = GuiKeyboardMap::baseline(&palette);

        assert_eq!(keyboard.source_label, "gui-core keyboard map");
        assert!(!keyboard.host_specific_overrides_required);
        assert_eq!(
            keyboard
                .contexts
                .iter()
                .map(|context| context.context.label())
                .collect::<Vec<_>>(),
            vec![
                "global-shell",
                "project-tree",
                "editor",
                "diagnostics",
                "run-output",
                "immediate",
                "debug",
                "command-palette",
            ]
        );
        assert!(keyboard.context_collisions().is_empty());
        assert!(keyboard.disallowed_cross_context_collisions().is_empty());
    }

    #[test]
    fn same_gesture_across_contexts_requires_explicit_allowance() {
        let document = DocumentLifecycleState::open_clean(
            "Option Explicit",
            LifecycleCapabilities::browser_limited(),
        );
        let palette = GuiCommandPalette::browser_safe_baseline(&document);
        let keyboard = GuiKeyboardMap::baseline(&palette);

        let enter_bindings = keyboard
            .bindings
            .iter()
            .filter(|binding| binding.gesture.display == "Enter")
            .collect::<Vec<_>>();
        assert_eq!(enter_bindings.len(), 2);
        assert!(
            enter_bindings
                .iter()
                .all(|binding| binding.allow_same_gesture_in_distinct_contexts)
        );

        let mut invalid = keyboard.clone();
        invalid.bindings[0].gesture = GuiKeyGesture::new("Enter");
        invalid.bindings[0].allow_same_gesture_in_distinct_contexts = false;

        let collisions = invalid.disallowed_cross_context_collisions();
        assert_eq!(collisions.len(), 1);
        assert_eq!(collisions[0].gesture, "enter");
        assert!(
            collisions[0]
                .command_stable_ids
                .contains(&String::from("shell.command_palette"))
        );
    }

    #[test]
    fn keyboard_bindings_preserve_disabled_reasons_from_command_palette() {
        let document = DocumentLifecycleState::open_clean(
            "Option Explicit",
            LifecycleCapabilities::browser_limited(),
        );
        let palette = GuiCommandPalette::browser_safe_baseline(&document);
        let keyboard = GuiKeyboardMap::baseline(&palette);

        let run = keyboard
            .binding_for(GuiCommandId::RuntimeRun)
            .expect("run keybinding");
        assert_eq!(run.context, GuiKeyboardContext::GlobalShell);
        assert_eq!(run.gesture.display, "F5");
        assert!(!run.availability.is_enabled);
        assert!(
            run.availability
                .disabled_reason
                .as_deref()
                .expect("run disabled reason")
                .contains("native execution provider unavailable")
        );

        let save = keyboard
            .binding_for(GuiCommandId::DocumentSave)
            .expect("save keybinding");
        assert_eq!(save.context, GuiKeyboardContext::Editor);
        assert_eq!(save.gesture.display, "Ctrl+S");
        assert!(
            save.availability
                .disabled_reason
                .as_deref()
                .expect("save disabled reason")
                .contains("filesystem persistence")
        );
    }

    #[test]
    fn focus_graph_contains_required_gui_surface_nodes() {
        let document = DocumentLifecycleState::open_clean(
            "Option Explicit",
            LifecycleCapabilities::browser_limited(),
        );
        let palette = GuiCommandPalette::browser_safe_baseline(&document);
        let focus = GuiFocusGraph::baseline(&palette);

        assert_eq!(focus.source_label, "gui-core focus graph");
        assert_eq!(
            focus
                .nodes
                .iter()
                .map(|node| node.kind.label())
                .collect::<Vec<_>>(),
            vec![
                "project-tree",
                "editor",
                "diagnostics",
                "lifecycle-controls",
                "run-output",
                "immediate",
                "debug",
                "com-capability",
                "command-palette",
            ]
        );
        assert!(focus.nodes.iter().all(|node| node.focusable));
        assert_eq!(
            focus
                .node("command-palette")
                .expect("command palette focus node")
                .restore_target
                .as_deref(),
            Some("source-editor")
        );
    }

    #[test]
    fn focus_graph_no_mouse_route_reaches_core_surfaces_in_order() {
        let document = DocumentLifecycleState::open_clean(
            "Option Explicit",
            LifecycleCapabilities::browser_limited(),
        );
        let palette = GuiCommandPalette::browser_safe_baseline(&document);
        let focus = GuiFocusGraph::baseline(&palette);

        assert_eq!(
            focus.route_node_ids(),
            vec![
                "project-tree",
                "source-editor",
                "diagnostics-panel",
                "lifecycle-controls",
                "run-output",
                "immediate-panel",
                "debug-panel",
                "com-capability",
                "command-palette",
                "source-editor",
            ]
        );
        assert_eq!(focus.no_mouse_route[0].index, 1);
        assert_eq!(focus.no_mouse_route[0].node_id, "project-tree");
        assert_eq!(
            focus.no_mouse_route[8].restoration_hint.as_deref(),
            Some("returns to source-editor")
        );
    }

    #[test]
    fn disabled_reason_panels_remain_focusable() {
        let document = DocumentLifecycleState::open_clean(
            "Option Explicit",
            LifecycleCapabilities::browser_limited(),
        );
        let palette = GuiCommandPalette::browser_safe_baseline(&document);
        let focus = GuiFocusGraph::baseline(&palette);

        let disabled_focusable = focus.disabled_focusable_node_ids();
        assert!(disabled_focusable.contains(&"lifecycle-controls"));
        assert!(disabled_focusable.contains(&"run-output"));
        assert!(disabled_focusable.contains(&"immediate-panel"));
        assert!(disabled_focusable.contains(&"debug-panel"));
        assert!(disabled_focusable.contains(&"com-capability"));
    }

    #[test]
    fn accessibility_projection_labels_every_major_surface() {
        let document = DocumentLifecycleState::open_clean(
            "Option Explicit",
            LifecycleCapabilities::browser_limited(),
        );
        let palette = GuiCommandPalette::browser_safe_baseline(&document);
        let accessibility = GuiAccessibilityProjection::baseline(&palette);

        assert_eq!(
            accessibility
                .nodes
                .iter()
                .map(|node| node.role.label())
                .collect::<Vec<_>>(),
            vec![
                "project-tree",
                "editor",
                "diagnostics",
                "lifecycle-controls",
                "run-output",
                "immediate",
                "debug",
                "com-capability",
                "command-palette",
                "capability-footer",
            ]
        );
        assert!(
            accessibility
                .nodes
                .iter()
                .all(|node| !node.accessible_label.is_empty())
        );
        assert!(!accessibility.web_framework_bound);
    }

    #[test]
    fn accessibility_projection_exposes_disabled_reason_descriptions() {
        let document = DocumentLifecycleState::open_clean(
            "Option Explicit",
            LifecycleCapabilities::browser_limited(),
        );
        let palette = GuiCommandPalette::browser_safe_baseline(&document);
        let accessibility = GuiAccessibilityProjection::baseline(&palette);

        assert!(
            accessibility
                .disabled_reason_surface_ids()
                .contains(&"run-output")
        );
        assert!(
            accessibility
                .node("run-output")
                .and_then(|node| node.disabled_reason.as_deref())
                .expect("run disabled reason")
                .contains("native execution provider unavailable")
        );
        assert!(
            accessibility
                .node("immediate-panel")
                .and_then(|node| node.disabled_reason.as_deref())
                .expect("Immediate disabled reason")
                .contains("no native OxVba runtime session")
        );
        assert!(
            accessibility
                .node("debug-panel")
                .and_then(|node| node.disabled_reason.as_deref())
                .expect("debug disabled reason")
                .contains("no OxVba debug session")
        );
        assert!(
            accessibility
                .node("com-capability")
                .and_then(|node| node.disabled_reason.as_deref())
                .expect("COM disabled reason")
                .contains("COM discovery unavailable in browser-safe profile")
        );
    }

    #[test]
    fn shell_packet_round_trips_and_reuses_existing_projections() {
        let packet = GuiShellPacket::browser_safe_baseline(
            "examples/thin-slice/ThinSliceHello.basproj",
            "ThinSliceHello",
            vec![GuiShellModuleSummary::new("Module1.bas", true)],
            "Module1.bas",
            "Module1",
            "Option Explicit",
            vec![],
        );

        let encoded = serde_json::to_string(&packet).expect("serialize shell packet");
        let decoded: GuiShellPacket = serde_json::from_str(&encoded).expect("decode shell packet");

        assert_eq!(decoded, packet);
        assert_eq!(packet.command_palette.commands.len(), 10);
        assert_eq!(packet.keyboard_map.contexts.len(), 8);
        assert_eq!(packet.focus_graph.nodes.len(), 9);
        assert_eq!(packet.accessibility.nodes.len(), 10);
        assert_eq!(packet.run_timeline.provider_label, "browser-unsupported");
    }

    #[test]
    fn shell_packet_preserves_browser_safe_limitations_without_tui_import() {
        let packet = GuiShellPacket::browser_safe_baseline(
            "examples/thin-slice/ThinSliceHello.basproj",
            "ThinSliceHello",
            vec![GuiShellModuleSummary::new("Module1.bas", true)],
            "Module1.bas",
            "Module1",
            "Option Explicit",
            vec![GuiShellDiagnosticSummary::new(
                "error",
                "use of undeclared variable: answer",
                "OxVba language service",
            )],
        );

        assert_eq!(packet.project_name, "ThinSliceHello");
        assert_eq!(packet.active_module, "Module1.bas");
        assert_eq!(packet.diagnostics.len(), 1);
        assert_eq!(packet.lifecycle_profile_label, "browser-limited");
        assert!(!packet.parked_tui_imported);
        assert!(!packet.native_execution_claimed);
        assert!(!packet.com_runtime_claimed);
        assert!(!packet.web_framework_bound);
        assert!(!packet.run_capability.can_run);
        assert!(
            packet
                .run_capability
                .disabled_reason
                .as_deref()
                .expect("run disabled reason")
                .contains("native execution provider unavailable")
        );
        assert!(!packet.com_capability.runtime_invocation.is_available);
        assert!(
            packet
                .capability_footer
                .contains("native execution and COM unavailable")
        );
    }
}
