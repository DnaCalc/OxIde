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
    PendingOxVbaHardening,
    UnavailableNoClaim,
}

impl HostBridgeCapabilityState {
    pub fn label(self) -> &'static str {
        match self {
            Self::ProvenOxideOnly => "proven-oxide-only",
            Self::OxVbaAvailableSubset => "oxvba-available-subset",
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
        HostBridgeServiceStatus::pending_oxvba_hardening(
            HostBridgeServiceCategory::Compile,
            "compile options and run target DTOs pending",
        )
    }
}

impl HostReferenceApi for BrowserReviewFixtureHost {
    fn reference_status(&self) -> HostBridgeServiceStatus {
        HostBridgeServiceStatus::oxvba_available_subset(
            HostBridgeServiceCategory::Reference,
            "ComSelectionService subset available; COM capability/native boundary hardening pending",
        )
    }
}

impl HostRuntimeApi for BrowserReviewFixtureHost {
    fn runtime_status(&self) -> HostBridgeServiceStatus {
        HostBridgeServiceStatus::pending_oxvba_hardening(
            HostBridgeServiceCategory::Runtime,
            "runtime session IDs, events, command availability, and source spans pending",
        )
    }

    fn runtime_packet(&self) -> RuntimeServicePacket {
        self.runtime.clone()
    }
}

impl HostImmediateApi for BrowserReviewFixtureHost {
    fn immediate_status(&self) -> HostBridgeServiceStatus {
        HostBridgeServiceStatus::pending_oxvba_hardening(
            HostBridgeServiceCategory::Immediate,
            "Immediate attach/session ID hardening pending",
        )
    }

    fn immediate_packet(&self) -> ImmediateServicePacket {
        self.immediate.clone()
    }
}

impl HostDebugApi for BrowserReviewFixtureHost {
    fn debug_status(&self) -> HostBridgeServiceStatus {
        HostBridgeServiceStatus::pending_oxvba_hardening(
            HostBridgeServiceCategory::Debug,
            "debug watch/breakpoint/source-span DTOs pending",
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
            HostBridgeServiceStatus::pending_oxvba_hardening(
                HostBridgeServiceCategory::Capability,
                "shared capability/error taxonomy pending",
            ),
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
        let pending = HostBridgeServiceStatus::pending_oxvba_hardening(
            HostBridgeServiceCategory::Debug,
            "watch and breakpoint DTOs pending",
        );

        assert_eq!(proven.state.label(), "proven-oxide-only");
        assert_eq!(subset.state.label(), "oxvba-available-subset");
        assert_eq!(pending.state.label(), "pending-oxvba-hardening");
        assert!(proven.no_claim_flags_false());
        assert!(subset.no_claim_flags_false());
        assert!(pending.no_claim_flags_false());
        assert!(!proven.state.full_claim_allowed());
        assert!(!subset.state.full_claim_allowed());
        assert!(!pending.state.full_claim_allowed());
    }

    #[test]
    fn command_intent_maps_to_host_category() {
        let intent = HostCommandIntent::new("runtime.run", HostBridgeServiceCategory::Runtime);
        assert_eq!(intent.stable_id, "runtime.run");
        assert_eq!(intent.category.api_name(), "HostRuntimeApi");
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
    fn browser_review_fixture_separates_available_subset_adapters_from_claims() {
        let host = BrowserReviewFixtureHost::new(fixture_shell_packet());
        let adapters = host.available_subset_adapters();

        assert!(adapters.contains(&"HostWorkspaceSession"));
        assert!(adapters.contains(&"inspect_workspace_target"));
        assert!(adapters.contains(&"ComSelectionService"));
        assert!(adapters.contains(&"EmbeddedBuildRunHost"));
        assert!(adapters.contains(&"EmbeddedRunSession"));
        assert!(adapters.contains(&"ImmediateSession"));
        assert!(adapters.contains(&"DebugSession"));

        let language = host.language_status();
        let reference = host.reference_status();
        assert_eq!(
            language.state,
            HostBridgeCapabilityState::OxVbaAvailableSubset
        );
        assert_eq!(
            reference.state,
            HostBridgeCapabilityState::OxVbaAvailableSubset
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
            && status.state == HostBridgeCapabilityState::PendingOxVbaHardening));
        assert!(statuses.iter().any(|status| status.category
            == HostBridgeServiceCategory::Reference
            && status.state == HostBridgeCapabilityState::OxVbaAvailableSubset));
    }
}
