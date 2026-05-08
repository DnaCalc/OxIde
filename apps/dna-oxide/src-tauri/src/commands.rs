use std::path::{Path, PathBuf};

use serde::Serialize;

use oxide_core::{
    open_lifecycle_from_persistence, save_lifecycle_to_persistence, DebugServicePacket,
    GuiSessionSnapshot, GuiShellDiagnosticSummary, GuiShellModuleSummary, GuiShellPacket,
    ImmediateServicePacket, NativeFilesystemDocumentPersistence,
    NativeFilesystemSessionPersistence, PersistenceError, RuntimeServicePacket,
    SessionCapabilityProfile,
};
use oxide_host_bridge::{
    command_availability_for_statuses, host_bridge_command_catalog, BrowserReviewFixtureHost,
    HostBridgeCommandAvailability, HostCapabilityApi,
};
use oxide_oxvba::{load_project_open_spine, ProjectOpenSpineError};

pub const COMMAND_REGISTRATION_KIND: &str = "w352-tauri-linked-native-command-spine";
pub const LEGACY_W344_COMMAND_REGISTRATION_KIND: &str = "w344-rust-callable-tauri-ready";

pub const PROVEN_OXIDE_COMMAND_PLACEHOLDERS: &[&str] = &[
    "dna_oxide_get_host_capabilities",
    "dna_oxide_open_project_path",
    "dna_oxide_load_active_module",
    "dna_oxide_save_active_module",
    "dna_oxide_reload_active_module",
    "dna_oxide_revert_active_module",
    "dna_oxide_save_session_snapshot",
    "dna_oxide_load_session_snapshot",
    "dna_oxide_open_settings",
    "dna_oxide_open_command_palette",
];

pub const OXVBA_AVAILABLE_SUBSET_COMMAND_PLACEHOLDERS: &[&str] = &[
    "dna_oxide_inspect_project",
    "dna_oxide_language_diagnostics",
    "dna_oxide_language_hover",
    "dna_oxide_language_definition",
    "dna_oxide_language_references",
    "dna_oxide_debug_continue",
    "dna_oxide_debug_step_into",
    "dna_oxide_debug_step_over",
    "dna_oxide_debug_step_out",
];

pub const OXVBA_FIXTURE_EVIDENCED_COMMAND_PLACEHOLDERS: &[&str] = &[
    "dna_oxide_build_check",
    "dna_oxide_get_references",
    "dna_oxide_find_com_candidates",
    "dna_oxide_run_project",
    "dna_oxide_evaluate_immediate",
    "dna_oxide_debug_attach",
    "dna_oxide_watch_upsert",
    "dna_oxide_breakpoint_set",
];

pub const PENDING_OXVBA_COMMAND_PLACEHOLDERS: &[&str] = &[
    "dna_oxide_get_compile_options",
    "dna_oxide_apply_compile_options",
    "dna_oxide_apply_reference_plan",
    "dna_oxide_stop_runtime",
    "dna_oxide_reset_runtime",
    "dna_oxide_debug_stop",
    "dna_oxide_watch_remove",
    "dna_oxide_breakpoint_clear",
];

pub fn all_command_placeholders() -> Vec<&'static str> {
    PROVEN_OXIDE_COMMAND_PLACEHOLDERS
        .iter()
        .chain(OXVBA_AVAILABLE_SUBSET_COMMAND_PLACEHOLDERS.iter())
        .chain(OXVBA_FIXTURE_EVIDENCED_COMMAND_PLACEHOLDERS.iter())
        .chain(PENDING_OXVBA_COMMAND_PLACEHOLDERS.iter())
        .copied()
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DnaOxideCommandBucket {
    ProvenOxideOnly,
    OxVbaAvailableSubset,
    OxVbaFixtureEvidenced,
    PendingOxVbaHardening,
}

impl DnaOxideCommandBucket {
    pub fn label(self) -> &'static str {
        match self {
            Self::ProvenOxideOnly => "proven-oxide-only",
            Self::OxVbaAvailableSubset => "oxvba-available-subset",
            Self::OxVbaFixtureEvidenced => "oxvba-fixture-evidenced",
            Self::PendingOxVbaHardening => "pending-oxvba-hardening",
        }
    }

    pub fn enabled_by_default(self) -> bool {
        matches!(self, Self::ProvenOxideOnly)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DnaOxideNoClaimFlags {
    pub real_execution_claimed: bool,
    pub native_runtime_claimed: bool,
    pub com_runtime_claimed: bool,
    pub fake_responses: bool,
    pub fake_debug_data: bool,
}

impl DnaOxideNoClaimFlags {
    pub const fn all_false() -> Self {
        Self {
            real_execution_claimed: false,
            native_runtime_claimed: false,
            com_runtime_claimed: false,
            fake_responses: false,
            fake_debug_data: false,
        }
    }

    pub fn all_runtime_claims_false(self) -> bool {
        !self.real_execution_claimed
            && !self.native_runtime_claimed
            && !self.com_runtime_claimed
            && !self.fake_responses
            && !self.fake_debug_data
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnaOxideModuleCommandPacket {
    pub command_name: &'static str,
    pub host_bridge_command: &'static str,
    pub bucket: DnaOxideCommandBucket,
    pub enabled: bool,
    pub project_name: String,
    pub workspace_path: String,
    pub active_module: String,
    pub module_path: String,
    pub source_text: String,
    pub dirty: bool,
    pub provider_label: &'static str,
    pub no_claims: DnaOxideNoClaimFlags,
}

impl DnaOxideModuleCommandPacket {
    pub fn bucket_label(&self) -> &'static str {
        self.bucket.label()
    }

    pub fn no_claims_all_false(&self) -> bool {
        self.no_claims.all_runtime_claims_false()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnaOxideSessionCommandPacket {
    pub command_name: &'static str,
    pub host_bridge_command: &'static str,
    pub bucket: DnaOxideCommandBucket,
    pub enabled: bool,
    pub session_path: String,
    pub snapshot: GuiSessionSnapshot,
    pub provider_label: &'static str,
    pub no_claims: DnaOxideNoClaimFlags,
}

impl DnaOxideSessionCommandPacket {
    pub fn bucket_label(&self) -> &'static str {
        self.bucket.label()
    }

    pub fn no_claims_all_false(&self) -> bool {
        self.no_claims.all_runtime_claims_false()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnaOxideUnavailableCommandPacket {
    pub command_name: &'static str,
    pub host_bridge_command: &'static str,
    pub bucket: DnaOxideCommandBucket,
    pub enabled: bool,
    pub disabled_reason: String,
    pub evidence: Option<&'static str>,
    pub no_claims: DnaOxideNoClaimFlags,
}

impl DnaOxideUnavailableCommandPacket {
    pub fn bucket_label(&self) -> &'static str {
        self.bucket.label()
    }

    pub fn no_claims_all_false(&self) -> bool {
        self.no_claims.all_runtime_claims_false()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnaOxideRuntimeCommandPacket {
    pub command_name: &'static str,
    pub host_bridge_command: &'static str,
    pub bucket: DnaOxideCommandBucket,
    pub enabled: bool,
    pub disabled_reason: String,
    pub packet: RuntimeServicePacket,
    pub no_claims: DnaOxideNoClaimFlags,
}

impl DnaOxideRuntimeCommandPacket {
    pub fn bucket_label(&self) -> &'static str {
        self.bucket.label()
    }

    pub fn no_claims_all_false(&self) -> bool {
        self.no_claims.all_runtime_claims_false()
            && !self.packet.real_execution_claimed
            && !self.packet.native_runtime_claimed
            && !self.packet.com_runtime_claimed
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnaOxideImmediateCommandPacket {
    pub command_name: &'static str,
    pub host_bridge_command: &'static str,
    pub bucket: DnaOxideCommandBucket,
    pub enabled: bool,
    pub disabled_reason: String,
    pub packet: ImmediateServicePacket,
    pub no_claims: DnaOxideNoClaimFlags,
}

impl DnaOxideImmediateCommandPacket {
    pub fn bucket_label(&self) -> &'static str {
        self.bucket.label()
    }

    pub fn no_claims_all_false(&self) -> bool {
        self.no_claims.all_runtime_claims_false()
            && !self.packet.fake_responses
            && !self.packet.native_runtime_claimed
            && !self.packet.com_runtime_claimed
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnaOxideDebugCommandPacket {
    pub command_name: &'static str,
    pub host_bridge_command: &'static str,
    pub bucket: DnaOxideCommandBucket,
    pub enabled: bool,
    pub disabled_reason: String,
    pub packet: DebugServicePacket,
    pub no_claims: DnaOxideNoClaimFlags,
}

impl DnaOxideDebugCommandPacket {
    pub fn bucket_label(&self) -> &'static str {
        self.bucket.label()
    }

    pub fn no_claims_all_false(&self) -> bool {
        self.no_claims.all_runtime_claims_false()
            && !self.packet.fake_debug_data
            && !self.packet.native_runtime_claimed
            && !self.packet.com_runtime_claimed
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DnaOxideCommandError {
    ProjectOpen { path: String, message: String },
    MissingActiveModule { path: String },
    Persistence { message: String },
}

impl std::fmt::Display for DnaOxideCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProjectOpen { path, message } => write!(f, "open project {path}: {message}"),
            Self::MissingActiveModule { path } => write!(f, "project {path} has no active module"),
            Self::Persistence { message } => write!(f, "persistence command failed: {message}"),
        }
    }
}

impl std::error::Error for DnaOxideCommandError {}

pub fn dna_oxide_open_project_path(
    project_path: impl AsRef<Path>,
) -> Result<DnaOxideModuleCommandPacket, DnaOxideCommandError> {
    module_packet_from_project(
        "dna_oxide_open_project_path",
        "project.open",
        DnaOxideCommandBucket::ProvenOxideOnly,
        project_path.as_ref(),
        None,
    )
}

pub fn dna_oxide_load_active_module(
    project_path: impl AsRef<Path>,
) -> Result<DnaOxideModuleCommandPacket, DnaOxideCommandError> {
    module_packet_from_project(
        "dna_oxide_load_active_module",
        "document.reload",
        DnaOxideCommandBucket::ProvenOxideOnly,
        project_path.as_ref(),
        None,
    )
}

pub fn dna_oxide_save_active_module(
    project_path: impl AsRef<Path>,
    new_source: impl AsRef<str>,
) -> Result<DnaOxideModuleCommandPacket, DnaOxideCommandError> {
    let project_path = project_path.as_ref();
    let opened = load_project(project_path)?;
    let mut persistence = NativeFilesystemDocumentPersistence::new(opened.module_path.clone());
    let mut lifecycle = open_lifecycle_from_persistence(&persistence).map_err(command_error)?;
    lifecycle.edit_working_source(new_source.as_ref().to_string());
    save_lifecycle_to_persistence(&mut lifecycle, &mut persistence).map_err(command_error)?;

    Ok(packet_from_parts(
        "dna_oxide_save_active_module",
        "document.save",
        DnaOxideCommandBucket::ProvenOxideOnly,
        opened,
        lifecycle.working_source().to_string(),
        lifecycle.is_dirty(),
    ))
}

pub fn dna_oxide_reload_active_module(
    project_path: impl AsRef<Path>,
) -> Result<DnaOxideModuleCommandPacket, DnaOxideCommandError> {
    module_packet_from_project(
        "dna_oxide_reload_active_module",
        "document.reload",
        DnaOxideCommandBucket::ProvenOxideOnly,
        project_path.as_ref(),
        None,
    )
}

pub fn dna_oxide_revert_active_module(
    project_path: impl AsRef<Path>,
    working_source: impl AsRef<str>,
) -> Result<DnaOxideModuleCommandPacket, DnaOxideCommandError> {
    let project_path = project_path.as_ref();
    let opened = load_project(project_path)?;
    let persistence = NativeFilesystemDocumentPersistence::new(opened.module_path.clone());
    let mut lifecycle = open_lifecycle_from_persistence(&persistence).map_err(command_error)?;
    lifecycle.edit_working_source(working_source.as_ref().to_string());
    lifecycle.revert_to_persisted().map_err(|disabled| {
        command_error(PersistenceError::Disabled {
            operation: oxide_core::PersistenceOperation::Save,
            reason: disabled.reason,
        })
    })?;

    Ok(packet_from_parts(
        "dna_oxide_revert_active_module",
        "document.revert",
        DnaOxideCommandBucket::ProvenOxideOnly,
        opened,
        lifecycle.working_source().to_string(),
        lifecycle.is_dirty(),
    ))
}

pub fn dna_oxide_save_session_snapshot(
    project_path: impl AsRef<Path>,
    session_path: impl AsRef<Path>,
    working_source: impl AsRef<str>,
) -> Result<DnaOxideSessionCommandPacket, DnaOxideCommandError> {
    let project_path = project_path.as_ref();
    let opened = load_project(project_path)?;
    let persistence = NativeFilesystemDocumentPersistence::new(opened.module_path.clone());
    let mut lifecycle = open_lifecycle_from_persistence(&persistence).map_err(command_error)?;
    lifecycle.edit_working_source(working_source.as_ref().to_string());
    let snapshot = GuiSessionSnapshot::capture(
        project_path.display().to_string(),
        opened.active_module,
        &lifecycle,
        SessionCapabilityProfile::AllSupported,
    );
    let session_path = session_path.as_ref();
    let session_persistence = NativeFilesystemSessionPersistence::new(session_path);
    session_persistence
        .save_snapshot(&snapshot)
        .map_err(command_error)?;

    Ok(DnaOxideSessionCommandPacket {
        command_name: "dna_oxide_save_session_snapshot",
        host_bridge_command: "document.save",
        bucket: DnaOxideCommandBucket::ProvenOxideOnly,
        enabled: true,
        session_path: session_path.display().to_string(),
        snapshot,
        provider_label: session_persistence.provider_label(),
        no_claims: DnaOxideNoClaimFlags::all_false(),
    })
}

pub fn dna_oxide_load_session_snapshot(
    session_path: impl AsRef<Path>,
) -> Result<DnaOxideSessionCommandPacket, DnaOxideCommandError> {
    let session_path = session_path.as_ref();
    let session_persistence = NativeFilesystemSessionPersistence::new(session_path);
    let snapshot = session_persistence.load_snapshot().map_err(command_error)?;

    Ok(DnaOxideSessionCommandPacket {
        command_name: "dna_oxide_load_session_snapshot",
        host_bridge_command: "document.reload",
        bucket: DnaOxideCommandBucket::ProvenOxideOnly,
        enabled: true,
        session_path: session_path.display().to_string(),
        snapshot,
        provider_label: session_persistence.provider_label(),
        no_claims: DnaOxideNoClaimFlags::all_false(),
    })
}

pub fn dna_oxide_get_host_capabilities(
    project_path: impl AsRef<Path>,
) -> Result<Vec<HostBridgeCommandAvailability>, DnaOxideCommandError> {
    let shell = shell_packet_for_project(project_path.as_ref())?;
    let host = BrowserReviewFixtureHost::new(shell);
    Ok(command_availability_for_statuses(
        &host_bridge_command_catalog(),
        &host.capability_statuses(),
    ))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DesktopHostCommandSpinePacket {
    pub command_name: &'static str,
    pub command_spine: &'static str,
    pub app_name: &'static str,
    pub product_name: &'static str,
    pub linked_native_rust: bool,
    pub project_path: String,
    pub availability_count: usize,
    pub enabled_count: usize,
    pub disabled_count: usize,
    pub sample_enabled_command: Option<String>,
    pub sample_disabled_command: Option<String>,
    pub real_execution_claimed: bool,
    pub native_runtime_claimed: bool,
    pub com_runtime_claimed: bool,
    pub fake_responses: bool,
    pub fake_debug_data: bool,
}

impl DesktopHostCommandSpinePacket {
    pub fn no_claims_all_false(&self) -> bool {
        !self.real_execution_claimed
            && !self.native_runtime_claimed
            && !self.com_runtime_claimed
            && !self.fake_responses
            && !self.fake_debug_data
    }
}

pub fn dna_oxide_desktop_host_capabilities_probe(
    project_path: Option<impl AsRef<Path>>,
) -> Result<DesktopHostCommandSpinePacket, DnaOxideCommandError> {
    let project_path = match project_path {
        Some(path) => path.as_ref().to_path_buf(),
        None => default_thin_slice_project_path(),
    };
    let availability = dna_oxide_get_host_capabilities(&project_path)?;
    let enabled_count = availability.iter().filter(|command| command.enabled).count();
    let sample_enabled_command = availability
        .iter()
        .find(|command| command.enabled)
        .map(|command| command.stable_id.clone());
    let sample_disabled_command = availability
        .iter()
        .find(|command| !command.enabled)
        .map(|command| command.stable_id.clone());

    Ok(DesktopHostCommandSpinePacket {
        command_name: "dna_oxide_desktop_host_capabilities_probe",
        command_spine: "WebView UI -> Tauri invoke -> #[tauri::command] -> linked Rust command module -> typed packet",
        app_name: "DnaOxIde",
        product_name: "DNA OxIde",
        linked_native_rust: true,
        project_path: project_path.display().to_string(),
        availability_count: availability.len(),
        enabled_count,
        disabled_count: availability.len().saturating_sub(enabled_count),
        sample_enabled_command,
        sample_disabled_command,
        real_execution_claimed: false,
        native_runtime_claimed: false,
        com_runtime_claimed: false,
        fake_responses: false,
        fake_debug_data: false,
    })
}

pub fn dna_oxide_get_compile_options() -> DnaOxideUnavailableCommandPacket {
    pending_packet(
        "dna_oxide_get_compile_options",
        "compile.options",
        "project properties / compile options DTOs pending OxIde adoption",
    )
}

pub fn dna_oxide_apply_compile_options() -> DnaOxideUnavailableCommandPacket {
    pending_packet(
        "dna_oxide_apply_compile_options",
        "compile.options",
        "compile option mutation DTOs pending OxIde adoption",
    )
}

pub fn dna_oxide_build_check() -> DnaOxideUnavailableCommandPacket {
    fixture_packet(
        "dna_oxide_build_check",
        "compile.check",
        "ThinSliceHello fixture covers EmbeddedBuildRunHost::build_workspace; DnaOxIde adapter test pending",
        Some("EmbeddedBuildRunHost::build_workspace"),
    )
}

pub fn dna_oxide_get_references() -> DnaOxideUnavailableCommandPacket {
    fixture_packet(
        "dna_oxide_get_references",
        "references.show",
        "ThinSliceHello fixture covers ComSelectionService reference state; DnaOxIde adapter test pending",
        Some("ComSelectionService::inspect_workspace_project_state"),
    )
}

pub fn dna_oxide_find_com_candidates() -> DnaOxideUnavailableCommandPacket {
    fixture_packet(
        "dna_oxide_find_com_candidates",
        "references.com.search",
        "ComSelectionService subset and capability_profile are fixture-evidenced; COM runtime invocation remains unclaimed",
        Some("ComSelectionService::capability_profile"),
    )
}

pub fn dna_oxide_apply_reference_plan() -> DnaOxideUnavailableCommandPacket {
    pending_packet(
        "dna_oxide_apply_reference_plan",
        "references.com.search",
        "reference plan application DTO/adoption pending",
    )
}

pub fn dna_oxide_run_project(
    project_path: impl AsRef<Path>,
) -> Result<DnaOxideRuntimeCommandPacket, DnaOxideCommandError> {
    runtime_command_packet(
        "dna_oxide_run_project",
        "runtime.run",
        DnaOxideCommandBucket::OxVbaFixtureEvidenced,
        "ThinSliceHello fixture covers EmbeddedBuildRunHost::run_project and stable runtime IDs; DnaOxIde adapter test pending",
        project_path.as_ref(),
    )
}

pub fn dna_oxide_stop_runtime() -> DnaOxideUnavailableCommandPacket {
    pending_packet(
        "dna_oxide_stop_runtime",
        "runtime.stop",
        "stop/cancel command availability pending OxIde adoption",
    )
}

pub fn dna_oxide_reset_runtime() -> DnaOxideUnavailableCommandPacket {
    pending_packet(
        "dna_oxide_reset_runtime",
        "runtime.stop",
        "runtime reset command availability pending OxIde adoption",
    )
}

pub fn dna_oxide_evaluate_immediate(
    expression: impl Into<String>,
) -> DnaOxideImmediateCommandPacket {
    DnaOxideImmediateCommandPacket {
        command_name: "dna_oxide_evaluate_immediate",
        host_bridge_command: "runtime.immediate",
        bucket: DnaOxideCommandBucket::OxVbaFixtureEvidenced,
        enabled: false,
        disabled_reason: String::from(
            "ThinSliceHello fixture covers EmbeddedRunSession::into_immediate_session and ImmediateSession evaluation; DnaOxIde adapter test pending",
        ),
        packet: ImmediateServicePacket::native_service_missing(Some(expression.into())),
        no_claims: DnaOxideNoClaimFlags::all_false(),
    }
}

pub fn dna_oxide_debug_attach() -> DnaOxideDebugCommandPacket {
    debug_command_packet(
        "dna_oxide_debug_attach",
        "runtime.debug",
        DnaOxideCommandBucket::OxVbaFixtureEvidenced,
        "ThinSliceHello fixture covers EmbeddedRunSession::into_debug_session; DnaOxIde adapter test pending",
    )
}

pub fn dna_oxide_debug_continue() -> DnaOxideDebugCommandPacket {
    debug_command_packet(
        "dna_oxide_debug_continue",
        "debug.continue",
        DnaOxideCommandBucket::OxVbaAvailableSubset,
        "DebugSession continue subset exists; DnaOxIde adapter test pending",
    )
}

pub fn dna_oxide_debug_step_into() -> DnaOxideDebugCommandPacket {
    debug_command_packet(
        "dna_oxide_debug_step_into",
        "debug.step_into",
        DnaOxideCommandBucket::OxVbaAvailableSubset,
        "DebugSession step subset exists; DnaOxIde adapter test pending",
    )
}

pub fn dna_oxide_debug_step_over() -> DnaOxideDebugCommandPacket {
    debug_command_packet(
        "dna_oxide_debug_step_over",
        "debug.step_over",
        DnaOxideCommandBucket::OxVbaAvailableSubset,
        "DebugSession step subset exists; DnaOxIde adapter test pending",
    )
}

pub fn dna_oxide_debug_step_out() -> DnaOxideDebugCommandPacket {
    debug_command_packet(
        "dna_oxide_debug_step_out",
        "debug.step_out",
        DnaOxideCommandBucket::OxVbaAvailableSubset,
        "DebugSession step subset exists; DnaOxIde adapter test pending",
    )
}

pub fn dna_oxide_debug_stop() -> DnaOxideUnavailableCommandPacket {
    pending_packet(
        "dna_oxide_debug_stop",
        "runtime.stop",
        "debug stop/cleanup command availability pending OxIde adoption",
    )
}

pub fn dna_oxide_watch_upsert() -> DnaOxideDebugCommandPacket {
    debug_command_packet(
        "dna_oxide_watch_upsert",
        "watch.upsert",
        DnaOxideCommandBucket::OxVbaFixtureEvidenced,
        "ThinSliceHello fixture covers DebugSession::add_watch and evaluate_watches; DnaOxIde adapter test pending",
    )
}

pub fn dna_oxide_watch_remove() -> DnaOxideUnavailableCommandPacket {
    pending_packet(
        "dna_oxide_watch_remove",
        "watch.upsert",
        "watch remove DTO/adoption pending",
    )
}

pub fn dna_oxide_breakpoint_set() -> DnaOxideDebugCommandPacket {
    debug_command_packet(
        "dna_oxide_breakpoint_set",
        "breakpoint.set",
        DnaOxideCommandBucket::OxVbaFixtureEvidenced,
        "ThinSliceHello fixture covers DebugSession::set_source_breakpoint; DnaOxIde adapter test pending",
    )
}

pub fn dna_oxide_breakpoint_clear() -> DnaOxideUnavailableCommandPacket {
    pending_packet(
        "dna_oxide_breakpoint_clear",
        "breakpoint.set",
        "breakpoint clear/unbind DTO/adoption pending",
    )
}

struct OpenedProjectModule {
    project_name: String,
    workspace_path: String,
    active_module: String,
    module_path: PathBuf,
}

fn module_packet_from_project(
    command_name: &'static str,
    host_bridge_command: &'static str,
    bucket: DnaOxideCommandBucket,
    project_path: &Path,
    source_override: Option<String>,
) -> Result<DnaOxideModuleCommandPacket, DnaOxideCommandError> {
    let opened = load_project(project_path)?;
    let source_text = match source_override {
        Some(source) => source,
        None => std::fs::read_to_string(&opened.module_path).map_err(|error| {
            command_error(PersistenceError::Io {
                operation: oxide_core::PersistenceOperation::Load,
                path: opened.module_path.display().to_string(),
                message: error.to_string(),
            })
        })?,
    };
    Ok(packet_from_parts(
        command_name,
        host_bridge_command,
        bucket,
        opened,
        source_text,
        false,
    ))
}

fn packet_from_parts(
    command_name: &'static str,
    host_bridge_command: &'static str,
    bucket: DnaOxideCommandBucket,
    opened: OpenedProjectModule,
    source_text: String,
    dirty: bool,
) -> DnaOxideModuleCommandPacket {
    DnaOxideModuleCommandPacket {
        command_name,
        host_bridge_command,
        bucket,
        enabled: bucket.enabled_by_default(),
        project_name: opened.project_name,
        workspace_path: opened.workspace_path,
        active_module: opened.active_module,
        module_path: opened.module_path.display().to_string(),
        source_text,
        dirty,
        provider_label: "native-filesystem",
        no_claims: DnaOxideNoClaimFlags::all_false(),
    }
}

fn pending_packet(
    command_name: &'static str,
    host_bridge_command: &'static str,
    disabled_reason: impl Into<String>,
) -> DnaOxideUnavailableCommandPacket {
    DnaOxideUnavailableCommandPacket {
        command_name,
        host_bridge_command,
        bucket: DnaOxideCommandBucket::PendingOxVbaHardening,
        enabled: false,
        disabled_reason: disabled_reason.into(),
        evidence: None,
        no_claims: DnaOxideNoClaimFlags::all_false(),
    }
}

fn fixture_packet(
    command_name: &'static str,
    host_bridge_command: &'static str,
    disabled_reason: impl Into<String>,
    evidence: Option<&'static str>,
) -> DnaOxideUnavailableCommandPacket {
    DnaOxideUnavailableCommandPacket {
        command_name,
        host_bridge_command,
        bucket: DnaOxideCommandBucket::OxVbaFixtureEvidenced,
        enabled: false,
        disabled_reason: disabled_reason.into(),
        evidence,
        no_claims: DnaOxideNoClaimFlags::all_false(),
    }
}

fn runtime_command_packet(
    command_name: &'static str,
    host_bridge_command: &'static str,
    bucket: DnaOxideCommandBucket,
    disabled_reason: impl Into<String>,
    project_path: &Path,
) -> Result<DnaOxideRuntimeCommandPacket, DnaOxideCommandError> {
    let opened = load_project(project_path)?;
    Ok(DnaOxideRuntimeCommandPacket {
        command_name,
        host_bridge_command,
        bucket,
        enabled: false,
        disabled_reason: disabled_reason.into(),
        packet: RuntimeServicePacket::native_service_missing(
            opened.workspace_path,
            opened.project_name,
            module_stem(&opened.active_module),
            "Main",
        ),
        no_claims: DnaOxideNoClaimFlags::all_false(),
    })
}

fn debug_command_packet(
    command_name: &'static str,
    host_bridge_command: &'static str,
    bucket: DnaOxideCommandBucket,
    disabled_reason: impl Into<String>,
) -> DnaOxideDebugCommandPacket {
    DnaOxideDebugCommandPacket {
        command_name,
        host_bridge_command,
        bucket,
        enabled: false,
        disabled_reason: disabled_reason.into(),
        packet: DebugServicePacket::native_service_missing(),
        no_claims: DnaOxideNoClaimFlags::all_false(),
    }
}

fn shell_packet_for_project(project_path: &Path) -> Result<GuiShellPacket, DnaOxideCommandError> {
    let view = load_project_open_spine(project_path)
        .map_err(|error| project_error(project_path, error))?;
    let modules = view
        .modules
        .iter()
        .map(|module| GuiShellModuleSummary::new(&module.display_name, module.is_active))
        .collect::<Vec<_>>();
    Ok(GuiShellPacket::browser_safe_baseline(
        project_path.display().to_string(),
        view.project_name,
        modules,
        view.active_source.module_display_name.clone(),
        module_stem(&view.active_source.module_display_name),
        view.active_source.source_text,
        vec![GuiShellDiagnosticSummary::new(
            "info",
            "DnaOxIde host capability command uses W343 host bridge fixture state",
            "DnaOxIde W344",
        )],
    ))
}

fn module_stem(display_name: &str) -> String {
    display_name
        .strip_suffix(".bas")
        .or_else(|| display_name.strip_suffix(".cls"))
        .unwrap_or(display_name)
        .to_string()
}

fn default_thin_slice_project_path() -> PathBuf {
    let mut cursor = std::env::current_dir().unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")));
    loop {
        let candidate = cursor
            .join("examples")
            .join("thin-slice")
            .join("ThinSliceHello.basproj");
        if candidate.exists() {
            return candidate;
        }
        if !cursor.pop() {
            break;
        }
    }

    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("..")
        .join("examples")
        .join("thin-slice")
        .join("ThinSliceHello.basproj")
}

fn load_project(project_path: &Path) -> Result<OpenedProjectModule, DnaOxideCommandError> {
    let view = load_project_open_spine(project_path)
        .map_err(|error| project_error(project_path, error))?;
    let active = view
        .modules
        .iter()
        .find(|module| module.is_active)
        .ok_or_else(|| DnaOxideCommandError::MissingActiveModule {
            path: project_path.display().to_string(),
        })?;
    let module_path = project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(&active.include_path);

    Ok(OpenedProjectModule {
        project_name: view.project_name,
        workspace_path: project_path.display().to_string(),
        active_module: view.active_source.module_display_name,
        module_path,
    })
}

fn project_error(project_path: &Path, error: ProjectOpenSpineError) -> DnaOxideCommandError {
    DnaOxideCommandError::ProjectOpen {
        path: project_path.display().to_string(),
        message: error.to_string(),
    }
}

fn command_error(error: PersistenceError) -> DnaOxideCommandError {
    DnaOxideCommandError::Persistence {
        message: error.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn command_placeholders_cover_proven_lifecycle_path() {
        let commands = all_command_placeholders();
        assert!(commands.contains(&"dna_oxide_get_host_capabilities"));
        assert!(commands.contains(&"dna_oxide_open_project_path"));
        assert!(commands.contains(&"dna_oxide_save_active_module"));
        assert!(commands.contains(&"dna_oxide_load_session_snapshot"));
        assert!(commands.contains(&"dna_oxide_open_command_palette"));
    }

    #[test]
    fn command_placeholders_name_subset_fixture_and_pending_full_scope_services() {
        let commands = all_command_placeholders();
        assert!(commands.contains(&"dna_oxide_language_diagnostics"));
        assert!(commands.contains(&"dna_oxide_build_check"));
        assert!(commands.contains(&"dna_oxide_evaluate_immediate"));
        assert!(commands.contains(&"dna_oxide_debug_attach"));
        assert!(commands.contains(&"dna_oxide_watch_upsert"));
        assert!(commands.contains(&"dna_oxide_breakpoint_set"));
        assert!(commands.contains(&"dna_oxide_find_com_candidates"));
        assert!(commands.contains(&"dna_oxide_get_compile_options"));
        assert!(commands.contains(&"dna_oxide_stop_runtime"));
    }

    #[test]
    fn command_buckets_have_stable_labels_and_safe_defaults() {
        assert_eq!(
            DnaOxideCommandBucket::ProvenOxideOnly.label(),
            "proven-oxide-only"
        );
        assert_eq!(
            DnaOxideCommandBucket::OxVbaAvailableSubset.label(),
            "oxvba-available-subset"
        );
        assert_eq!(
            DnaOxideCommandBucket::OxVbaFixtureEvidenced.label(),
            "oxvba-fixture-evidenced"
        );
        assert_eq!(
            DnaOxideCommandBucket::PendingOxVbaHardening.label(),
            "pending-oxvba-hardening"
        );
        assert!(DnaOxideNoClaimFlags::all_false().all_runtime_claims_false());
    }

    #[test]
    fn open_and_load_project_commands_read_temp_fixture_copy_without_claims() {
        let fixture = copy_thin_slice_fixture("open-load");
        let project = fixture.join("ThinSliceHello.basproj");

        let opened = dna_oxide_open_project_path(&project).expect("open project command");
        assert_eq!(opened.command_name, "dna_oxide_open_project_path");
        assert_eq!(opened.host_bridge_command, "project.open");
        assert_eq!(opened.bucket_label(), "proven-oxide-only");
        assert!(opened.enabled);
        assert_eq!(opened.project_name, "ThinSliceHello");
        assert_eq!(opened.active_module, "Module1.bas");
        assert!(opened.source_text.contains("Public Sub Main()"));
        assert!(!opened.dirty);
        assert!(opened.no_claims_all_false());

        let loaded = dna_oxide_load_active_module(&project).expect("load module command");
        assert_eq!(loaded.command_name, "dna_oxide_load_active_module");
        assert_eq!(loaded.host_bridge_command, "document.reload");
        assert_eq!(loaded.source_text, opened.source_text);
        assert!(loaded.no_claims_all_false());
    }

    #[test]
    fn save_reload_and_revert_commands_operate_on_temp_fixture_copy() {
        let fixture = copy_thin_slice_fixture("save-reload-revert");
        let project = fixture.join("ThinSliceHello.basproj");
        let module = fixture.join("Module1.bas");
        let original = fs::read_to_string(&module).expect("read original temp module");
        let edited = original.replace(
            "answer = 40 + 2",
            "answer = 41 + 1 ' Edited by DnaOxIde test",
        );

        let saved = dna_oxide_save_active_module(&project, &edited).expect("save active module");
        assert_eq!(saved.command_name, "dna_oxide_save_active_module");
        assert_eq!(saved.host_bridge_command, "document.save");
        assert!(saved.source_text.contains("Edited by DnaOxIde test"));
        assert!(!saved.dirty);
        assert!(saved.no_claims_all_false());
        assert_eq!(
            fs::read_to_string(&module).expect("read saved temp module"),
            edited
        );

        let reloaded = dna_oxide_reload_active_module(&project).expect("reload active module");
        assert_eq!(reloaded.command_name, "dna_oxide_reload_active_module");
        assert!(reloaded.source_text.contains("Edited by DnaOxIde test"));
        assert!(!reloaded.dirty);

        let reverted = dna_oxide_revert_active_module(&project, "unsaved working text")
            .expect("revert active module");
        assert_eq!(reverted.command_name, "dna_oxide_revert_active_module");
        assert!(reverted.source_text.contains("Edited by DnaOxIde test"));
        assert!(!reverted.source_text.contains("unsaved working text"));
        assert_eq!(
            fs::read_to_string(&module).expect("read after revert"),
            edited
        );
    }

    #[test]
    fn session_snapshot_commands_round_trip_temp_project_state() {
        let fixture = copy_thin_slice_fixture("session-round-trip");
        let project = fixture.join("ThinSliceHello.basproj");
        let session = fixture.join("dna-oxide-session.json");
        let working_source = "Public Sub Main()\n    Debug.Print \"dirty session\"\nEnd Sub\n";

        let saved = dna_oxide_save_session_snapshot(&project, &session, working_source)
            .expect("save session snapshot");
        assert_eq!(saved.command_name, "dna_oxide_save_session_snapshot");
        assert_eq!(saved.bucket_label(), "proven-oxide-only");
        assert!(saved.snapshot.is_dirty());
        assert!(saved.snapshot.working_source.contains("dirty session"));
        assert!(saved.no_claims_all_false());

        let loaded = dna_oxide_load_session_snapshot(&session).expect("load session snapshot");
        assert_eq!(loaded.command_name, "dna_oxide_load_session_snapshot");
        assert_eq!(loaded.snapshot, saved.snapshot);
        assert!(loaded.no_claims_all_false());
    }

    #[test]
    fn host_capability_command_projects_w343_availability_without_claims() {
        let fixture = copy_thin_slice_fixture("host-capabilities");
        let project = fixture.join("ThinSliceHello.basproj");

        let availability = dna_oxide_get_host_capabilities(&project).expect("host capabilities");
        assert_eq!(availability.len(), 26);
        assert!(availability
            .iter()
            .all(|command| command.no_claim_flags_false()));
        assert!(availability
            .iter()
            .any(|command| command.stable_id == "project.open" && command.enabled));
        assert!(availability
            .iter()
            .any(|command| command.stable_id == "runtime.run"
                && command.state.label() == "oxvba-fixture-evidenced"
                && !command.enabled));
        assert!(availability
            .iter()
            .any(|command| command.stable_id == "compile.options"
                && command.state.label() == "pending-oxvba-hardening"
                && !command.enabled));
    }

    #[test]
    fn desktop_host_capabilities_probe_returns_typed_linked_rust_packet_without_claims() {
        let fixture = copy_thin_slice_fixture("desktop-host-probe");
        let project = fixture.join("ThinSliceHello.basproj");

        let packet = dna_oxide_desktop_host_capabilities_probe(Some(&project))
            .expect("desktop host capabilities probe");
        assert_eq!(
            packet.command_name,
            "dna_oxide_desktop_host_capabilities_probe"
        );
        assert!(packet.linked_native_rust);
        assert!(packet.command_spine.contains("Tauri invoke"));
        assert_eq!(packet.product_name, "DNA OxIde");
        assert_eq!(packet.availability_count, 26);
        assert!(packet.enabled_count > 0);
        assert!(packet.disabled_count > 0);
        assert_eq!(packet.sample_enabled_command.as_deref(), Some("project.open"));
        assert!(packet.no_claims_all_false());
    }

    #[test]
    fn unavailable_compile_reference_and_com_commands_are_labeled_without_claims() {
        let compile = dna_oxide_get_compile_options();
        assert_eq!(compile.command_name, "dna_oxide_get_compile_options");
        assert_eq!(compile.bucket_label(), "pending-oxvba-hardening");
        assert!(!compile.enabled);
        assert!(compile.no_claims_all_false());
        let apply_compile = dna_oxide_apply_compile_options();
        assert_eq!(
            apply_compile.command_name,
            "dna_oxide_apply_compile_options"
        );
        assert_eq!(apply_compile.bucket_label(), "pending-oxvba-hardening");
        assert!(apply_compile.no_claims_all_false());

        let build = dna_oxide_build_check();
        assert_eq!(build.command_name, "dna_oxide_build_check");
        assert_eq!(build.bucket_label(), "oxvba-fixture-evidenced");
        assert_eq!(
            build.evidence,
            Some("EmbeddedBuildRunHost::build_workspace")
        );
        assert!(build
            .disabled_reason
            .contains("DnaOxIde adapter test pending"));
        assert!(build.no_claims_all_false());

        let references = dna_oxide_get_references();
        assert_eq!(references.bucket_label(), "oxvba-fixture-evidenced");
        assert_eq!(
            references.evidence,
            Some("ComSelectionService::inspect_workspace_project_state")
        );
        assert!(references.no_claims_all_false());

        let com = dna_oxide_find_com_candidates();
        assert_eq!(com.command_name, "dna_oxide_find_com_candidates");
        assert_eq!(com.bucket_label(), "oxvba-fixture-evidenced");
        assert_eq!(
            com.evidence,
            Some("ComSelectionService::capability_profile")
        );
        assert!(com
            .disabled_reason
            .contains("COM runtime invocation remains unclaimed"));
        assert!(com.no_claims_all_false());

        let apply = dna_oxide_apply_reference_plan();
        assert_eq!(apply.bucket_label(), "pending-oxvba-hardening");
        assert!(apply.no_claims_all_false());
    }

    #[test]
    fn runtime_immediate_and_debug_commands_return_empty_native_missing_packets() {
        let fixture = copy_thin_slice_fixture("runtime-debug-unavailable");
        let project = fixture.join("ThinSliceHello.basproj");

        let run = dna_oxide_run_project(&project).expect("run command packet");
        assert_eq!(run.command_name, "dna_oxide_run_project");
        assert_eq!(run.bucket_label(), "oxvba-fixture-evidenced");
        assert!(!run.enabled);
        assert_eq!(run.packet.provider_label(), "native-service-missing");
        assert!(run.no_claims_all_false());

        let stop = dna_oxide_stop_runtime();
        assert_eq!(stop.bucket_label(), "pending-oxvba-hardening");
        assert!(stop.no_claims_all_false());
        let reset = dna_oxide_reset_runtime();
        assert_eq!(reset.bucket_label(), "pending-oxvba-hardening");
        assert!(reset.no_claims_all_false());

        let immediate = dna_oxide_evaluate_immediate("?answer");
        assert_eq!(immediate.command_name, "dna_oxide_evaluate_immediate");
        assert_eq!(immediate.bucket_label(), "oxvba-fixture-evidenced");
        assert_eq!(immediate.packet.provider_label(), "native-service-missing");
        assert_eq!(immediate.packet.responses.len(), 0);
        assert!(!immediate.packet.fake_responses);
        assert!(immediate.no_claims_all_false());

        for debug in [
            dna_oxide_debug_attach(),
            dna_oxide_debug_continue(),
            dna_oxide_debug_step_into(),
            dna_oxide_debug_step_over(),
            dna_oxide_debug_step_out(),
            dna_oxide_watch_upsert(),
            dna_oxide_breakpoint_set(),
        ] {
            assert!(matches!(
                debug.bucket_label(),
                "oxvba-fixture-evidenced" | "oxvba-available-subset"
            ));
            assert!(!debug.enabled);
            assert_eq!(debug.packet.provider_label(), "native-service-missing");
            assert_eq!(debug.packet.callstack.len(), 0);
            assert_eq!(debug.packet.locals.len(), 0);
            assert_eq!(debug.packet.watches.len(), 0);
            assert_eq!(debug.packet.breakpoints.len(), 0);
            assert!(!debug.packet.fake_debug_data);
            assert!(debug.no_claims_all_false());
        }

        assert_eq!(
            dna_oxide_debug_stop().bucket_label(),
            "pending-oxvba-hardening"
        );
        assert_eq!(
            dna_oxide_watch_remove().bucket_label(),
            "pending-oxvba-hardening"
        );
        assert_eq!(
            dna_oxide_breakpoint_clear().bucket_label(),
            "pending-oxvba-hardening"
        );
    }

    #[test]
    fn checked_in_thin_slice_fixture_is_not_mutated_by_command_tests() {
        let root = repo_root();
        let fixture_module = root.join("examples").join("thin-slice").join("Module1.bas");
        let source = fs::read_to_string(fixture_module).expect("read checked-in module");

        assert!(source.contains("answer = 40 + 2"));
        assert!(!source.contains("Edited by DnaOxIde test"));
        assert!(!source.contains("dirty session"));
    }

    fn copy_thin_slice_fixture(label: &str) -> PathBuf {
        let root = repo_root();
        let source_dir = root.join("examples").join("thin-slice");
        let dest_dir = root
            .join("target")
            .join("dnaoxide-command-tests")
            .join(format!("{label}-{}", unique_suffix()));
        fs::create_dir_all(&dest_dir).expect("create command test directory");
        fs::copy(
            source_dir.join("ThinSliceHello.basproj"),
            dest_dir.join("ThinSliceHello.basproj"),
        )
        .expect("copy basproj fixture");
        fs::copy(source_dir.join("Module1.bas"), dest_dir.join("Module1.bas"))
            .expect("copy module fixture");
        dest_dir
    }

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("..")
    }

    fn unique_suffix() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time")
            .as_nanos()
    }
}
