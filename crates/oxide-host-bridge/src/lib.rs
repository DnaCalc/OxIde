//! Host-neutral service facade for shared OxIde UI consumers.
//!
//! This crate lets shared UI route commands to a host implementation without
//! depending on Tauri, DnaOxIde app code, DnaOneCalc product code, or OxVba
//! implementation details.

use oxide_bridge::DnaOneCalcWebShellHostPacket;
use oxide_core::{
    DebugServicePacket, GuiShellPacket, ImmediateServicePacket, RuntimeServicePacket,
};

/// Compile-time marker for the host bridge crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OxideHostBridgeRole {
    /// Host-neutral facade between shared UI and concrete host adapters.
    HostNeutralServiceFacade,
}

impl OxideHostBridgeRole {
    pub fn crate_name(self) -> &'static str {
        match self {
            Self::HostNeutralServiceFacade => "oxide-host-bridge",
        }
    }

    pub fn tauri_coupled(self) -> bool {
        false
    }

    pub fn app_folder_coupled(self) -> bool {
        false
    }
}

/// Current evidence state for one host service.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostBridgeCapabilityState {
    ProvenOxideOnly,
    OxVbaAvailableSubset,
    OxVbaFixtureEvidenced,
    PendingOxVbaHardening,
    UnavailableNoClaim,
}

impl HostBridgeCapabilityState {
    pub fn label(self) -> &'static str {
        match self {
            Self::ProvenOxideOnly => "proven-oxide-only",
            Self::OxVbaAvailableSubset => "oxvba-available-subset",
            Self::OxVbaFixtureEvidenced => "oxvba-fixture-evidenced",
            Self::PendingOxVbaHardening => "pending-oxvba-hardening",
            Self::UnavailableNoClaim => "unavailable-no-claim",
        }
    }

    pub fn full_claim_allowed(self) -> bool {
        false
    }
}

/// Stable service category names used by shared UI command routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostBridgeServiceCategory {
    Project,
    Document,
    Language,
    Compile,
    Reference,
    Runtime,
    Immediate,
    Debug,
    Settings,
    Capability,
}

impl HostBridgeServiceCategory {
    pub fn api_name(self) -> &'static str {
        match self {
            Self::Project => "HostProjectApi",
            Self::Document => "HostDocumentApi",
            Self::Language => "HostLanguageApi",
            Self::Compile => "HostCompileApi",
            Self::Reference => "HostReferenceApi",
            Self::Runtime => "HostRuntimeApi",
            Self::Immediate => "HostImmediateApi",
            Self::Debug => "HostDebugApi",
            Self::Settings => "HostSettingsApi",
            Self::Capability => "HostCapabilityApi",
        }
    }
}

/// One service availability row for UI command availability and disabled reasons.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostBridgeServiceStatus {
    pub category: HostBridgeServiceCategory,
    pub state: HostBridgeCapabilityState,
    pub disabled_reason: Option<String>,
    pub real_execution_claimed: bool,
    pub native_runtime_claimed: bool,
    pub com_runtime_claimed: bool,
    pub fake_immediate_responses: bool,
    pub fake_debug_data: bool,
}

impl HostBridgeServiceStatus {
    pub fn new(
        category: HostBridgeServiceCategory,
        state: HostBridgeCapabilityState,
        disabled_reason: Option<String>,
    ) -> Self {
        Self {
            category,
            state,
            disabled_reason,
            real_execution_claimed: false,
            native_runtime_claimed: false,
            com_runtime_claimed: false,
            fake_immediate_responses: false,
            fake_debug_data: false,
        }
    }

    pub fn proven_oxide_only(category: HostBridgeServiceCategory) -> Self {
        Self::new(category, HostBridgeCapabilityState::ProvenOxideOnly, None)
    }

    pub fn oxvba_available_subset(
        category: HostBridgeServiceCategory,
        detail: impl Into<String>,
    ) -> Self {
        Self::new(
            category,
            HostBridgeCapabilityState::OxVbaAvailableSubset,
            Some(detail.into()),
        )
    }

    pub fn oxvba_fixture_evidenced(
        category: HostBridgeServiceCategory,
        detail: impl Into<String>,
    ) -> Self {
        Self::new(
            category,
            HostBridgeCapabilityState::OxVbaFixtureEvidenced,
            Some(detail.into()),
        )
    }

    pub fn pending_oxvba_hardening(
        category: HostBridgeServiceCategory,
        reason: impl Into<String>,
    ) -> Self {
        Self::new(
            category,
            HostBridgeCapabilityState::PendingOxVbaHardening,
            Some(reason.into()),
        )
    }

    pub fn no_claim_flags_false(&self) -> bool {
        !self.real_execution_claimed
            && !self.native_runtime_claimed
            && !self.com_runtime_claimed
            && !self.fake_immediate_responses
            && !self.fake_debug_data
    }
}

/// Host identity that can implement the same bridge facade.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostBridgeConsumerKind {
    DnaOxIde,
    DnaOneCalc,
    BrowserReview,
    GuiLab,
}

impl HostBridgeConsumerKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::DnaOxIde => "dnaoxide",
            Self::DnaOneCalc => "dnaonecalc",
            Self::BrowserReview => "browser-review",
            Self::GuiLab => "oxide-guilab",
        }
    }
}

/// Thin command intent emitted by shared UI and routed by a host implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostCommandIntent {
    pub stable_id: String,
    pub category: HostBridgeServiceCategory,
}

impl HostCommandIntent {
    pub fn new(stable_id: impl Into<String>, category: HostBridgeServiceCategory) -> Self {
        Self {
            stable_id: stable_id.into(),
            category,
        }
    }
}

/// Stable command descriptor shared by UI renderers and host implementations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HostBridgeCommandSpec {
    pub stable_id: &'static str,
    pub label: &'static str,
    pub category: HostBridgeServiceCategory,
    pub state_override: Option<HostBridgeCapabilityState>,
    pub disabled_reason_override: Option<&'static str>,
}

impl HostBridgeCommandSpec {
    pub const fn new(
        stable_id: &'static str,
        label: &'static str,
        category: HostBridgeServiceCategory,
    ) -> Self {
        Self {
            stable_id,
            label,
            category,
            state_override: None,
            disabled_reason_override: None,
        }
    }

    pub const fn pending(
        stable_id: &'static str,
        label: &'static str,
        category: HostBridgeServiceCategory,
        reason: &'static str,
    ) -> Self {
        Self {
            stable_id,
            label,
            category,
            state_override: Some(HostBridgeCapabilityState::PendingOxVbaHardening),
            disabled_reason_override: Some(reason),
        }
    }

    pub fn intent(self) -> HostCommandIntent {
        HostCommandIntent::new(self.stable_id, self.category)
    }
}

/// Command availability projected from host service status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostBridgeCommandAvailability {
    pub stable_id: String,
    pub label: String,
    pub category: HostBridgeServiceCategory,
    pub state: HostBridgeCapabilityState,
    pub enabled: bool,
    pub disabled_reason: Option<String>,
    pub real_execution_claimed: bool,
    pub native_runtime_claimed: bool,
    pub com_runtime_claimed: bool,
    pub fake_immediate_responses: bool,
    pub fake_debug_data: bool,
}

impl HostBridgeCommandAvailability {
    pub fn from_status(spec: HostBridgeCommandSpec, status: &HostBridgeServiceStatus) -> Self {
        let state = spec.state_override.unwrap_or(status.state);
        let enabled = command_enabled_by_default(status, state, spec);
        Self {
            stable_id: spec.stable_id.to_string(),
            label: spec.label.to_string(),
            category: spec.category,
            state,
            enabled,
            disabled_reason: disabled_reason_for_command(spec, status, state, enabled),
            real_execution_claimed: status.real_execution_claimed,
            native_runtime_claimed: status.native_runtime_claimed,
            com_runtime_claimed: status.com_runtime_claimed,
            fake_immediate_responses: status.fake_immediate_responses,
            fake_debug_data: status.fake_debug_data,
        }
    }

    pub fn unavailable(spec: HostBridgeCommandSpec) -> Self {
        Self::from_status(
            spec,
            &HostBridgeServiceStatus::new(
                spec.category,
                HostBridgeCapabilityState::UnavailableNoClaim,
                Some(format!(
                    "{} has no host bridge service status in this host",
                    spec.category.api_name()
                )),
            ),
        )
    }

    pub fn no_claim_flags_false(&self) -> bool {
        !self.real_execution_claimed
            && !self.native_runtime_claimed
            && !self.com_runtime_claimed
            && !self.fake_immediate_responses
            && !self.fake_debug_data
    }
}

fn command_enabled_by_default(
    status: &HostBridgeServiceStatus,
    state: HostBridgeCapabilityState,
    spec: HostBridgeCommandSpec,
) -> bool {
    spec.state_override.is_none()
        && state == HostBridgeCapabilityState::ProvenOxideOnly
        && status.disabled_reason.is_none()
        && status.no_claim_flags_false()
}

fn disabled_reason_for_command(
    spec: HostBridgeCommandSpec,
    status: &HostBridgeServiceStatus,
    state: HostBridgeCapabilityState,
    enabled: bool,
) -> Option<String> {
    if enabled {
        return None;
    }
    Some(
        spec.disabled_reason_override
            .map(String::from)
            .or_else(|| status.disabled_reason.clone())
            .unwrap_or_else(|| {
                format!(
                    "{} command {} is {} for this host",
                    spec.category.api_name(),
                    spec.stable_id,
                    state.label()
                )
            }),
    )
}

/// Stable W343 command catalog in display order.
pub fn host_bridge_command_catalog() -> Vec<HostBridgeCommandSpec> {
    use HostBridgeServiceCategory as Category;
    vec![
        HostBridgeCommandSpec::new("project.open", "Open project", Category::Project),
        HostBridgeCommandSpec::new("project.inspect", "Inspect project", Category::Project),
        HostBridgeCommandSpec::new("document.save", "Save document", Category::Document),
        HostBridgeCommandSpec::new("document.reload", "Reload document", Category::Document),
        HostBridgeCommandSpec::new("document.revert", "Revert document", Category::Document),
        HostBridgeCommandSpec::new("language.diagnostics", "Diagnostics", Category::Language),
        HostBridgeCommandSpec::new("language.hover", "Hover", Category::Language),
        HostBridgeCommandSpec::new(
            "language.definition",
            "Go to definition",
            Category::Language,
        ),
        HostBridgeCommandSpec::new("language.references", "Find references", Category::Language),
        HostBridgeCommandSpec::pending(
            "compile.options",
            "Compile options",
            Category::Compile,
            "project properties / compile options DTOs pending OxIde adoption",
        ),
        HostBridgeCommandSpec::new("compile.check", "Build/check", Category::Compile),
        HostBridgeCommandSpec::new("references.show", "Show references", Category::Reference),
        HostBridgeCommandSpec::new(
            "references.com.search",
            "Search COM references",
            Category::Reference,
        ),
        HostBridgeCommandSpec::new("runtime.run", "Run", Category::Runtime),
        HostBridgeCommandSpec::pending(
            "runtime.stop",
            "Stop",
            Category::Runtime,
            "stop/cancel command availability pending OxIde adoption",
        ),
        HostBridgeCommandSpec::new("runtime.immediate", "Immediate", Category::Immediate),
        HostBridgeCommandSpec::new("runtime.debug", "Debug", Category::Debug),
        HostBridgeCommandSpec::new("debug.continue", "Continue", Category::Debug),
        HostBridgeCommandSpec::new("debug.step_into", "Step into", Category::Debug),
        HostBridgeCommandSpec::new("debug.step_over", "Step over", Category::Debug),
        HostBridgeCommandSpec::new("debug.step_out", "Step out", Category::Debug),
        HostBridgeCommandSpec::new("watch.upsert", "Upsert watch", Category::Debug),
        HostBridgeCommandSpec::new("breakpoint.set", "Set breakpoint", Category::Debug),
        HostBridgeCommandSpec::new("settings.open", "Open settings", Category::Settings),
        HostBridgeCommandSpec::new("capability.show", "Show capabilities", Category::Capability),
        HostBridgeCommandSpec::new(
            "shell.command_palette",
            "Command palette",
            Category::Capability,
        ),
    ]
}

/// Project command availability from current service statuses.
pub fn command_availability_for_statuses(
    specs: &[HostBridgeCommandSpec],
    statuses: &[HostBridgeServiceStatus],
) -> Vec<HostBridgeCommandAvailability> {
    specs
        .iter()
        .map(|spec| {
            statuses
                .iter()
                .find(|status| status.category == spec.category)
                .map(|status| HostBridgeCommandAvailability::from_status(*spec, status))
                .unwrap_or_else(|| HostBridgeCommandAvailability::unavailable(*spec))
        })
        .collect()
}

pub trait HostProjectApi {
    fn project_status(&self) -> HostBridgeServiceStatus;
    fn shell_packet(&self) -> HostBridgeResponse<GuiShellPacket>;
}

pub trait HostDocumentApi {
    fn document_status(&self) -> HostBridgeServiceStatus;
}

pub trait HostLanguageApi {
    fn language_status(&self) -> HostBridgeServiceStatus;
}

pub trait HostCompileApi {
    fn compile_status(&self) -> HostBridgeServiceStatus;
}

pub trait HostReferenceApi {
    fn reference_status(&self) -> HostBridgeServiceStatus;
}

pub trait HostRuntimeApi {
    fn runtime_status(&self) -> HostBridgeServiceStatus;
    fn runtime_packet(&self) -> RuntimeServicePacket;
}

pub trait HostImmediateApi {
    fn immediate_status(&self) -> HostBridgeServiceStatus;
    fn immediate_packet(&self) -> ImmediateServicePacket;
}

pub trait HostDebugApi {
    fn debug_status(&self) -> HostBridgeServiceStatus;
    fn debug_packet(&self) -> DebugServicePacket;
}

pub trait HostSettingsApi {
    fn settings_status(&self) -> HostBridgeServiceStatus;
}

pub trait HostCapabilityApi {
    fn capability_statuses(&self) -> Vec<HostBridgeServiceStatus>;
}

/// Optional facade for hosts that can produce the DnaOneCalc web-shell packet.
pub trait HostDnaOneCalcWebShellApi {
    fn dnaonecalc_web_shell_packet(&self) -> HostBridgeResponse<DnaOneCalcWebShellHostPacket>;
}

/// Generic host bridge response that carries evidence state without duplicating
/// final OxVba DTO ownership.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HostBridgeResponse<T> {
    Ready {
        value: T,
        state: HostBridgeCapabilityState,
    },
    Unavailable {
        status: HostBridgeServiceStatus,
    },
}

impl<T> HostBridgeResponse<T> {
    pub fn proven(value: T) -> Self {
        Self::Ready {
            value,
            state: HostBridgeCapabilityState::ProvenOxideOnly,
        }
    }

    pub fn unavailable(status: HostBridgeServiceStatus) -> Self {
        Self::Unavailable { status }
    }

    pub fn state_label(&self) -> &'static str {
        match self {
            Self::Ready { state, .. } => state.label(),
            Self::Unavailable { status } => status.state.label(),
        }
    }
}

/// Current OxVba available-subset adapter target names, kept separate from full claims.
pub const OXVBA_AVAILABLE_SUBSET_ADAPTERS: &[&str] = &[
    "HostWorkspaceSession",
    "inspect_workspace_target",
    "ComSelectionService",
    "EmbeddedBuildRunHost",
    "EmbeddedRunSession",
    "ImmediateSession",
    "DebugSession",
];

/// Sibling-repo evidence file that upgrades several adapter targets from blank gaps.
pub const OXVBA_THIN_SLICE_FIXTURE_EVIDENCE_DOC: &str =
    "../OxVba/docs/evidence/DNAOXIDE_THIN_SLICE_HELLO_FIXTURE_2026-05-07.md";

/// OxVba ThinSliceHello seams with fixture evidence, still requiring OxIde adapter tests.
pub const OXVBA_THIN_SLICE_FIXTURE_EVIDENCED_SEAMS: &[&str] = &[
    "HostWorkspaceSession::load_workspace_path",
    "HostWorkspaceSession::set_document_text",
    "workspace_roster",
    "EmbeddedBuildRunHost::build_workspace",
    "EmbeddedBuildRunHost::run_project",
    "EmbeddedRunSession::into_immediate_session",
    "ImmediateSession::evaluate",
    "EmbeddedRunSession::into_debug_session",
    "DebugSession::add_watch",
    "DebugSession::evaluate_watches",
    "DebugSession::set_source_breakpoint",
    "stable frame/watch/breakpoint/runtime IDs",
    "ComSelectionService::inspect_workspace_project_state",
    "ComSelectionService::capability_profile",
];

/// Browser-review fixture host for deterministic bridge tests.
#[derive(Debug, Clone)]
pub struct BrowserReviewFixtureHost {
    consumer: HostBridgeConsumerKind,
    shell: GuiShellPacket,
    runtime: RuntimeServicePacket,
    immediate: ImmediateServicePacket,
    debug: DebugServicePacket,
}

impl BrowserReviewFixtureHost {
    pub fn new(shell: GuiShellPacket) -> Self {
        let runtime = RuntimeServicePacket::native_service_missing(
            shell.workspace_path.clone(),
            shell.project_name.clone(),
            module_stem(&shell.active_module),
            "Main",
        );
        let immediate =
            ImmediateServicePacket::native_service_missing(Some(String::from("?answer")));
        let debug = DebugServicePacket::native_service_missing();
        Self {
            consumer: HostBridgeConsumerKind::BrowserReview,
            shell,
            runtime,
            immediate,
            debug,
        }
    }

    pub fn consumer(&self) -> HostBridgeConsumerKind {
        self.consumer
    }

    pub fn available_subset_adapters(&self) -> &'static [&'static str] {
        OXVBA_AVAILABLE_SUBSET_ADAPTERS
    }

    pub fn thin_slice_fixture_evidenced_seams(&self) -> &'static [&'static str] {
        OXVBA_THIN_SLICE_FIXTURE_EVIDENCED_SEAMS
    }
}

impl HostProjectApi for BrowserReviewFixtureHost {
    fn project_status(&self) -> HostBridgeServiceStatus {
        HostBridgeServiceStatus::proven_oxide_only(HostBridgeServiceCategory::Project)
    }

    fn shell_packet(&self) -> HostBridgeResponse<GuiShellPacket> {
        HostBridgeResponse::proven(self.shell.clone())
    }
}

impl HostDocumentApi for BrowserReviewFixtureHost {
    fn document_status(&self) -> HostBridgeServiceStatus {
        HostBridgeServiceStatus::proven_oxide_only(HostBridgeServiceCategory::Document)
    }
}

impl HostLanguageApi for BrowserReviewFixtureHost {
    fn language_status(&self) -> HostBridgeServiceStatus {
        HostBridgeServiceStatus::oxvba_available_subset(
            HostBridgeServiceCategory::Language,
            "HostWorkspaceSession language subset available; stable DTO hardening pending",
        )
    }
}

impl HostCompileApi for BrowserReviewFixtureHost {
    fn compile_status(&self) -> HostBridgeServiceStatus {
        HostBridgeServiceStatus::oxvba_fixture_evidenced(
            HostBridgeServiceCategory::Compile,
            "ThinSliceHello fixture covers EmbeddedBuildRunHost::build_workspace; compile options/run targets still pending OxIde adoption",
        )
    }
}

impl HostReferenceApi for BrowserReviewFixtureHost {
    fn reference_status(&self) -> HostBridgeServiceStatus {
        HostBridgeServiceStatus::oxvba_fixture_evidenced(
            HostBridgeServiceCategory::Reference,
            "ThinSliceHello fixture covers ComSelectionService reference state and capability_profile; native boundary/COM runtime invocation still unclaimed",
        )
    }
}

impl HostRuntimeApi for BrowserReviewFixtureHost {
    fn runtime_status(&self) -> HostBridgeServiceStatus {
        HostBridgeServiceStatus::oxvba_fixture_evidenced(
            HostBridgeServiceCategory::Runtime,
            "ThinSliceHello fixture covers EmbeddedBuildRunHost::run_project and stable runtime IDs; OxIde adapter tests/events/source spans pending",
        )
    }

    fn runtime_packet(&self) -> RuntimeServicePacket {
        self.runtime.clone()
    }
}

impl HostImmediateApi for BrowserReviewFixtureHost {
    fn immediate_status(&self) -> HostBridgeServiceStatus {
        HostBridgeServiceStatus::oxvba_fixture_evidenced(
            HostBridgeServiceCategory::Immediate,
            "ThinSliceHello fixture covers EmbeddedRunSession::into_immediate_session and ImmediateSession overlay evaluation; OxIde UX adapter pending",
        )
    }

    fn immediate_packet(&self) -> ImmediateServicePacket {
        self.immediate.clone()
    }
}

impl HostDebugApi for BrowserReviewFixtureHost {
    fn debug_status(&self) -> HostBridgeServiceStatus {
        HostBridgeServiceStatus::oxvba_fixture_evidenced(
            HostBridgeServiceCategory::Debug,
            "ThinSliceHello fixture covers debug attach, watch registry/evaluation, breakpoint binding DTOs, and stable IDs; OxIde source-span/UX adapter pending",
        )
    }

    fn debug_packet(&self) -> DebugServicePacket {
        self.debug.clone()
    }
}

impl HostSettingsApi for BrowserReviewFixtureHost {
    fn settings_status(&self) -> HostBridgeServiceStatus {
        HostBridgeServiceStatus::proven_oxide_only(HostBridgeServiceCategory::Settings)
    }
}

impl HostCapabilityApi for BrowserReviewFixtureHost {
    fn capability_statuses(&self) -> Vec<HostBridgeServiceStatus> {
        vec![
            self.project_status(),
            self.document_status(),
            self.language_status(),
            self.compile_status(),
            self.reference_status(),
            self.runtime_status(),
            self.immediate_status(),
            self.debug_status(),
            self.settings_status(),
            HostBridgeServiceStatus::proven_oxide_only(HostBridgeServiceCategory::Capability),
        ]
    }
}

fn module_stem(display_name: &str) -> String {
    display_name
        .strip_suffix(".bas")
        .or_else(|| display_name.strip_suffix(".cls"))
        .unwrap_or(display_name)
        .to_string()
}

/// All service categories in stable display order.
pub fn host_bridge_service_categories() -> Vec<HostBridgeServiceCategory> {
    vec![
        HostBridgeServiceCategory::Project,
        HostBridgeServiceCategory::Document,
        HostBridgeServiceCategory::Language,
        HostBridgeServiceCategory::Compile,
        HostBridgeServiceCategory::Reference,
        HostBridgeServiceCategory::Runtime,
        HostBridgeServiceCategory::Immediate,
        HostBridgeServiceCategory::Debug,
        HostBridgeServiceCategory::Settings,
        HostBridgeServiceCategory::Capability,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxide_core::{GuiShellDiagnosticSummary, GuiShellModuleSummary};

    fn fixture_shell_packet() -> GuiShellPacket {
        GuiShellPacket::browser_safe_baseline(
            "examples/thin-slice",
            "ThinSliceHello",
            vec![GuiShellModuleSummary::new("Module1.bas", true)],
            "Module1.bas",
            "Module1",
            "Public Sub Main()\nEnd Sub\n",
            vec![GuiShellDiagnosticSummary::new(
                "info",
                "host bridge fixture diagnostic",
                "oxide-host-bridge test",
            )],
        )
    }

    #[test]
    fn role_is_host_neutral_and_not_tauri_coupled() {
        let role = OxideHostBridgeRole::HostNeutralServiceFacade;
        assert_eq!(role.crate_name(), "oxide-host-bridge");
        assert!(!role.tauri_coupled());
        assert!(!role.app_folder_coupled());
    }

    #[test]
    fn all_host_api_categories_are_named() {
        let names = host_bridge_service_categories()
            .into_iter()
            .map(HostBridgeServiceCategory::api_name)
            .collect::<Vec<_>>();

        assert_eq!(names.len(), 10);
        assert!(names.contains(&"HostProjectApi"));
        assert!(names.contains(&"HostDocumentApi"));
        assert!(names.contains(&"HostLanguageApi"));
        assert!(names.contains(&"HostCompileApi"));
        assert!(names.contains(&"HostReferenceApi"));
        assert!(names.contains(&"HostRuntimeApi"));
        assert!(names.contains(&"HostImmediateApi"));
        assert!(names.contains(&"HostDebugApi"));
        assert!(names.contains(&"HostSettingsApi"));
        assert!(names.contains(&"HostCapabilityApi"));
    }

    #[test]
    fn service_statuses_keep_no_claim_flags_false() {
        let proven =
            HostBridgeServiceStatus::proven_oxide_only(HostBridgeServiceCategory::Document);
        let subset = HostBridgeServiceStatus::oxvba_available_subset(
            HostBridgeServiceCategory::Runtime,
            "EmbeddedBuildRunHost available subset",
        );
        let fixture = HostBridgeServiceStatus::oxvba_fixture_evidenced(
            HostBridgeServiceCategory::Debug,
            "ThinSliceHello fixture covers watch and breakpoint DTOs",
        );
        let pending = HostBridgeServiceStatus::pending_oxvba_hardening(
            HostBridgeServiceCategory::Debug,
            "source-span adoption pending",
        );

        assert_eq!(proven.state.label(), "proven-oxide-only");
        assert_eq!(subset.state.label(), "oxvba-available-subset");
        assert_eq!(fixture.state.label(), "oxvba-fixture-evidenced");
        assert_eq!(pending.state.label(), "pending-oxvba-hardening");
        assert!(proven.no_claim_flags_false());
        assert!(subset.no_claim_flags_false());
        assert!(fixture.no_claim_flags_false());
        assert!(pending.no_claim_flags_false());
        assert!(!proven.state.full_claim_allowed());
        assert!(!subset.state.full_claim_allowed());
        assert!(!fixture.state.full_claim_allowed());
        assert!(!pending.state.full_claim_allowed());
    }

    #[test]
    fn command_intent_maps_to_host_category() {
        let intent = HostCommandIntent::new("runtime.run", HostBridgeServiceCategory::Runtime);
        assert_eq!(intent.stable_id, "runtime.run");
        assert_eq!(intent.category.api_name(), "HostRuntimeApi");
    }

    #[test]
    fn command_catalog_maps_shared_ui_commands_to_host_categories() {
        let catalog = host_bridge_command_catalog();
        let ids = catalog
            .iter()
            .map(|command| command.stable_id)
            .collect::<Vec<_>>();

        assert_eq!(catalog.len(), 26);
        assert!(ids.contains(&"project.open"));
        assert!(ids.contains(&"document.save"));
        assert!(ids.contains(&"language.diagnostics"));
        assert!(ids.contains(&"compile.check"));
        assert!(ids.contains(&"references.com.search"));
        assert!(ids.contains(&"runtime.run"));
        assert!(ids.contains(&"runtime.immediate"));
        assert!(ids.contains(&"runtime.debug"));
        assert!(ids.contains(&"watch.upsert"));
        assert!(ids.contains(&"breakpoint.set"));
        assert!(ids.contains(&"shell.command_palette"));
        assert_eq!(
            catalog
                .iter()
                .find(|command| command.stable_id == "runtime.run")
                .expect("runtime command")
                .intent()
                .category,
            HostBridgeServiceCategory::Runtime
        );
    }

    #[test]
    fn command_availability_tracks_host_status_disabled_reasons_and_no_claims() {
        let host = BrowserReviewFixtureHost::new(fixture_shell_packet());
        let catalog = host_bridge_command_catalog();
        let availability = command_availability_for_statuses(&catalog, &host.capability_statuses());

        assert_eq!(availability.len(), catalog.len());
        assert!(availability
            .iter()
            .all(|command| command.no_claim_flags_false()));

        let project_open = availability
            .iter()
            .find(|command| command.stable_id == "project.open")
            .expect("project.open availability");
        assert!(project_open.enabled);
        assert_eq!(
            project_open.state,
            HostBridgeCapabilityState::ProvenOxideOnly
        );
        assert_eq!(project_open.disabled_reason, None);

        let run = availability
            .iter()
            .find(|command| command.stable_id == "runtime.run")
            .expect("runtime.run availability");
        assert!(!run.enabled);
        assert_eq!(run.state, HostBridgeCapabilityState::OxVbaFixtureEvidenced);
        assert!(run
            .disabled_reason
            .as_deref()
            .expect("runtime disabled reason")
            .contains("ThinSliceHello fixture covers EmbeddedBuildRunHost::run_project"));

        let palette = availability
            .iter()
            .find(|command| command.stable_id == "shell.command_palette")
            .expect("command palette availability");
        assert!(palette.enabled);
        assert_eq!(palette.state, HostBridgeCapabilityState::ProvenOxideOnly);
        assert_eq!(palette.disabled_reason, None);
    }

    #[test]
    fn consumers_include_dnaoxide_dnaonecalc_browser_and_guilab() {
        assert_eq!(HostBridgeConsumerKind::DnaOxIde.label(), "dnaoxide");
        assert_eq!(HostBridgeConsumerKind::DnaOneCalc.label(), "dnaonecalc");
        assert_eq!(
            HostBridgeConsumerKind::BrowserReview.label(),
            "browser-review"
        );
        assert_eq!(HostBridgeConsumerKind::GuiLab.label(), "oxide-guilab");
    }

    #[test]
    fn browser_review_fixture_exposes_proven_project_document_shell_packet() {
        let host = BrowserReviewFixtureHost::new(fixture_shell_packet());

        assert_eq!(host.consumer(), HostBridgeConsumerKind::BrowserReview);
        assert_eq!(
            host.project_status().state,
            HostBridgeCapabilityState::ProvenOxideOnly
        );
        assert_eq!(
            host.document_status().state,
            HostBridgeCapabilityState::ProvenOxideOnly
        );

        let shell = host.shell_packet();
        assert_eq!(shell.state_label(), "proven-oxide-only");
        match shell {
            HostBridgeResponse::Ready { value, .. } => {
                assert_eq!(value.project_name, "ThinSliceHello");
                assert_eq!(value.active_module, "Module1.bas");
            }
            HostBridgeResponse::Unavailable { .. } => panic!("shell packet should be available"),
        }
    }

    #[test]
    fn browser_review_fixture_separates_available_subset_and_fixture_evidence_from_claims() {
        let host = BrowserReviewFixtureHost::new(fixture_shell_packet());
        let adapters = host.available_subset_adapters();
        let seams = host.thin_slice_fixture_evidenced_seams();

        assert_eq!(
            OXVBA_THIN_SLICE_FIXTURE_EVIDENCE_DOC,
            "../OxVba/docs/evidence/DNAOXIDE_THIN_SLICE_HELLO_FIXTURE_2026-05-07.md"
        );
        assert!(adapters.contains(&"HostWorkspaceSession"));
        assert!(adapters.contains(&"inspect_workspace_target"));
        assert!(adapters.contains(&"ComSelectionService"));
        assert!(adapters.contains(&"EmbeddedBuildRunHost"));
        assert!(adapters.contains(&"EmbeddedRunSession"));
        assert!(adapters.contains(&"ImmediateSession"));
        assert!(adapters.contains(&"DebugSession"));
        assert!(seams.contains(&"HostWorkspaceSession::load_workspace_path"));
        assert!(seams.contains(&"HostWorkspaceSession::set_document_text"));
        assert!(seams.contains(&"workspace_roster"));
        assert!(seams.contains(&"EmbeddedBuildRunHost::build_workspace"));
        assert!(seams.contains(&"EmbeddedBuildRunHost::run_project"));
        assert!(seams.contains(&"EmbeddedRunSession::into_immediate_session"));
        assert!(seams.contains(&"ImmediateSession::evaluate"));
        assert!(seams.contains(&"EmbeddedRunSession::into_debug_session"));
        assert!(seams.contains(&"DebugSession::add_watch"));
        assert!(seams.contains(&"DebugSession::evaluate_watches"));
        assert!(seams.contains(&"DebugSession::set_source_breakpoint"));
        assert!(seams.contains(&"stable frame/watch/breakpoint/runtime IDs"));
        assert!(seams.contains(&"ComSelectionService::inspect_workspace_project_state"));
        assert!(seams.contains(&"ComSelectionService::capability_profile"));

        let language = host.language_status();
        let reference = host.reference_status();
        assert_eq!(
            language.state,
            HostBridgeCapabilityState::OxVbaAvailableSubset
        );
        assert_eq!(
            reference.state,
            HostBridgeCapabilityState::OxVbaFixtureEvidenced
        );
        assert!(language.no_claim_flags_false());
        assert!(reference.no_claim_flags_false());
    }

    #[test]
    fn browser_review_fixture_keeps_runtime_immediate_debug_unavailable_without_fake_data() {
        let host = BrowserReviewFixtureHost::new(fixture_shell_packet());

        let runtime = host.runtime_packet();
        let immediate = host.immediate_packet();
        let debug = host.debug_packet();

        assert_eq!(runtime.provider_label(), "native-service-missing");
        assert!(!runtime.real_execution_claimed);
        assert!(!runtime.native_runtime_claimed);
        assert!(!runtime.com_runtime_claimed);

        assert_eq!(immediate.provider_label(), "native-service-missing");
        assert_eq!(immediate.responses.len(), 0);
        assert!(!immediate.fake_responses);
        assert!(!immediate.native_runtime_claimed);
        assert!(!immediate.com_runtime_claimed);

        assert_eq!(debug.provider_label(), "native-service-missing");
        assert_eq!(debug.callstack.len(), 0);
        assert_eq!(debug.locals.len(), 0);
        assert_eq!(debug.watches.len(), 0);
        assert_eq!(debug.breakpoints.len(), 0);
        assert!(!debug.fake_debug_data);
        assert!(!debug.native_runtime_claimed);
        assert!(!debug.com_runtime_claimed);
    }

    #[test]
    fn browser_review_fixture_capability_statuses_cover_all_categories() {
        let host = BrowserReviewFixtureHost::new(fixture_shell_packet());
        let statuses = host.capability_statuses();

        assert_eq!(statuses.len(), 10);
        assert!(statuses
            .iter()
            .all(HostBridgeServiceStatus::no_claim_flags_false));
        assert!(statuses.iter().any(|status| status.category
            == HostBridgeServiceCategory::Compile
            && status.state == HostBridgeCapabilityState::OxVbaFixtureEvidenced));
        assert!(statuses.iter().any(|status| status.category
            == HostBridgeServiceCategory::Reference
            && status.state == HostBridgeCapabilityState::OxVbaFixtureEvidenced));
        assert!(statuses.iter().any(|status| status.category
            == HostBridgeServiceCategory::Runtime
            && status.state == HostBridgeCapabilityState::OxVbaFixtureEvidenced));
        assert!(statuses.iter().any(|status| status.category
            == HostBridgeServiceCategory::Immediate
            && status.state == HostBridgeCapabilityState::OxVbaFixtureEvidenced));
        assert!(statuses
            .iter()
            .any(|status| status.category == HostBridgeServiceCategory::Debug
                && status.state == HostBridgeCapabilityState::OxVbaFixtureEvidenced));
    }
}
