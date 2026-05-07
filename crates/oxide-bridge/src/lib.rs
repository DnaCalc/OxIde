//! Serializable host-boundary contracts for embedded OxIde surfaces.
//!
//! This crate owns OxIde boundary packets only. It consumes `oxide-core`
//! lifecycle/session/run state instead of creating a second copy of those
//! concepts for DNA Calc hosts.

use oxide_core::{
    DocumentLifecycleState, GuiSessionSnapshot, GuiShellPacket, LifecycleCapabilities,
    RunCapabilityProfile, RunRequest, RunTranscript, SessionCapabilityProfile,
};
use serde::{Deserialize, Serialize};

/// Compile-time marker for the OxIde bridge crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OxideBridgeRole {
    /// Serializable boundary packets for host/UI embedding seams.
    EmbeddedHostBoundary,
}

impl OxideBridgeRole {
    pub fn consumes_core_state(self) -> bool {
        match self {
            Self::EmbeddedHostBoundary => true,
        }
    }
}

/// Host-facing identity for a DNA Calc consumer of embedded OxIde surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedHostConsumer {
    pub host_name: String,
    pub product_role: String,
    pub shell_owner: String,
    pub persistence_owner: String,
}

impl EmbeddedHostConsumer {
    pub fn dnaonecalc() -> Self {
        Self {
            host_name: String::from("DnaOneCalc"),
            product_role: String::from("single-formula proving host"),
            shell_owner: String::from("DnaOneCalc owns product shell and host policy"),
            persistence_owner: String::from("DnaOneCalc owns host persistence policy"),
        }
    }
}

/// OxIde-owned surface slots that an embedding host may choose to mount.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmbeddedIdeSurfaceKind {
    ProjectSpine,
    SourceEditor,
    Diagnostics,
    DocumentLifecycle,
    RunOutput,
    CapabilityFooter,
}

impl EmbeddedIdeSurfaceKind {
    pub fn slot_id(self) -> &'static str {
        match self {
            Self::ProjectSpine => "project-spine",
            Self::SourceEditor => "source-editor",
            Self::Diagnostics => "diagnostics",
            Self::DocumentLifecycle => "document-lifecycle",
            Self::RunOutput => "run-output",
            Self::CapabilityFooter => "capability-footer",
        }
    }
}

/// One mountable IDE surface plus the ownership statement visible to a host.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedIdeSurface {
    pub kind: EmbeddedIdeSurfaceKind,
    pub slot_id: String,
    pub label: String,
    pub owner: String,
    pub required_for_first_proof: bool,
}

impl EmbeddedIdeSurface {
    pub fn new(kind: EmbeddedIdeSurfaceKind, label: impl Into<String>) -> Self {
        Self {
            kind,
            slot_id: kind.slot_id().to_string(),
            label: label.into(),
            owner: String::from("OxIde"),
            required_for_first_proof: true,
        }
    }
}

/// Source/project identity carried across the embedding boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedProjectDocument {
    pub workspace_path: String,
    pub project_name: String,
    pub active_module: String,
    pub document_display_name: String,
}

impl EmbeddedProjectDocument {
    pub fn new(
        workspace_path: impl Into<String>,
        project_name: impl Into<String>,
        active_module: impl Into<String>,
        document_display_name: impl Into<String>,
    ) -> Self {
        Self {
            workspace_path: workspace_path.into(),
            project_name: project_name.into(),
            active_module: active_module.into(),
            document_display_name: document_display_name.into(),
        }
    }
}

/// Explicit ownership boundary shown to host consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnershipBoundary {
    pub owner: String,
    pub responsibility: String,
}

impl OwnershipBoundary {
    pub fn new(owner: impl Into<String>, responsibility: impl Into<String>) -> Self {
        Self {
            owner: owner.into(),
            responsibility: responsibility.into(),
        }
    }
}

/// DnaOneCalc/OxIde embedding proof packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedIdePacket {
    pub consumer: EmbeddedHostConsumer,
    pub project_document: EmbeddedProjectDocument,
    pub surfaces: Vec<EmbeddedIdeSurface>,
    pub session_snapshot: GuiSessionSnapshot,
    pub run_capability: RunCapabilityProfile,
    pub run_transcript: RunTranscript,
    pub ownership_boundaries: Vec<OwnershipBoundary>,
    pub limitations: Vec<String>,
    pub sibling_repo_writes: bool,
}

impl EmbeddedIdePacket {
    pub fn dnaonecalc_thin_slice_browser_disabled(
        workspace_path: impl Into<String>,
        project_name: impl Into<String>,
        module_display_name: impl Into<String>,
        persisted_source: impl Into<String>,
    ) -> Self {
        let workspace_path = workspace_path.into();
        let project_name = project_name.into();
        let module_display_name = module_display_name.into();
        let active_module = module_stem(&module_display_name);
        let persisted_source = persisted_source.into();
        let document = DocumentLifecycleState::open_clean(
            persisted_source,
            LifecycleCapabilities::browser_limited(),
        );
        let session_snapshot = GuiSessionSnapshot::capture(
            workspace_path.clone(),
            module_display_name.clone(),
            &document,
            SessionCapabilityProfile::BrowserLimited,
        );
        let run_request = RunRequest::new(project_name.clone(), active_module.clone(), "Main");
        let run_capability = RunCapabilityProfile::browser_safe_unsupported();
        let run_transcript = RunTranscript::browser_disabled(run_request, run_capability.clone());

        Self {
            consumer: EmbeddedHostConsumer::dnaonecalc(),
            project_document: EmbeddedProjectDocument::new(
                workspace_path,
                project_name,
                active_module,
                module_display_name,
            ),
            surfaces: default_embedding_surfaces(),
            session_snapshot,
            run_capability,
            run_transcript,
            ownership_boundaries: vec![
                OwnershipBoundary::new(
                    "DnaOneCalc",
                    "owns product shell, host policy, persistence policy, and where embedded OxIde appears",
                ),
                OwnershipBoundary::new(
                    "OxIde",
                    "owns IDE experience, editor/project surface, lifecycle UX, run/output presentation, and embedding contract",
                ),
                OwnershipBoundary::new(
                    "OxVba",
                    "owns VBA project, language-service, semantic, build/run, Immediate, debug, and runtime truth",
                ),
            ],
            limitations: vec![
                String::from("OxIde repo-scoped W250 did not modify DnaOneCalc files"),
                String::from(
                    "browser-safe profile cannot execute VBA; native execution provider unavailable",
                ),
                String::from("pure browser/WASM cannot directly call Windows COM"),
                String::from(
                    "first W250 proof is a contract and lab scenario, not a paired DnaOneCalc smoke",
                ),
            ],
            sibling_repo_writes: false,
        }
    }

    pub fn surface_slot_ids(&self) -> Vec<&str> {
        self.surfaces
            .iter()
            .map(|surface| surface.slot_id.as_str())
            .collect()
    }

    pub fn ownership_owner_names(&self) -> Vec<&str> {
        self.ownership_boundaries
            .iter()
            .map(|boundary| boundary.owner.as_str())
            .collect()
    }
}

/// W300 DOM-smoke readiness projected for a consuming host.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebShellDomReadinessSummary {
    pub static_shell_dom_smoke_passed: bool,
    pub command_palette_dom_smoke_passed: bool,
    pub no_mouse_accessibility_dom_smoke_passed: bool,
    pub smoke_kind: String,
    pub browser_runtime_claimed: bool,
    pub dom_accessibility_audit_claimed: bool,
}

impl WebShellDomReadinessSummary {
    pub fn parsed_html_all_passed() -> Self {
        Self {
            static_shell_dom_smoke_passed: true,
            command_palette_dom_smoke_passed: true,
            no_mouse_accessibility_dom_smoke_passed: true,
            smoke_kind: String::from("parsed-html"),
            browser_runtime_claimed: false,
            dom_accessibility_audit_claimed: false,
        }
    }

    pub fn all_passed(&self) -> bool {
        self.static_shell_dom_smoke_passed
            && self.command_palette_dom_smoke_passed
            && self.no_mouse_accessibility_dom_smoke_passed
    }
}

/// Summary of the W300 web shell expected by DnaOneCalc.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DnaOneCalcWebShellMountContract {
    pub state_contract: String,
    pub adapter_crate: String,
    pub project_name: String,
    pub active_module: String,
    pub module_count: usize,
    pub command_count: usize,
    pub keybinding_count: usize,
    pub focus_route_length: usize,
    pub accessibility_surface_count: usize,
    pub dom_readiness: WebShellDomReadinessSummary,
    pub host_mount_claimed: bool,
    pub sibling_repo_writes: bool,
    pub filesystem_persistence_claimed: bool,
    pub native_runtime_claimed: bool,
    pub com_runtime_claimed: bool,
    pub dom_accessibility_audit_claimed: bool,
}

impl DnaOneCalcWebShellMountContract {
    pub fn from_shell_packet(
        shell_packet: &GuiShellPacket,
        dom_readiness: WebShellDomReadinessSummary,
    ) -> Self {
        Self {
            state_contract: String::from("GuiShellPacket"),
            adapter_crate: String::from("oxide-webshell"),
            project_name: shell_packet.project_name.clone(),
            active_module: shell_packet.active_module.clone(),
            module_count: shell_packet.modules.len(),
            command_count: shell_packet.command_palette.commands.len(),
            keybinding_count: shell_packet.keyboard_map.bindings.len(),
            focus_route_length: shell_packet.focus_graph.no_mouse_route.len(),
            accessibility_surface_count: shell_packet.accessibility.nodes.len(),
            dom_accessibility_audit_claimed: dom_readiness.dom_accessibility_audit_claimed,
            dom_readiness,
            host_mount_claimed: false,
            sibling_repo_writes: false,
            filesystem_persistence_claimed: false,
            native_runtime_claimed: shell_packet.native_execution_claimed,
            com_runtime_claimed: shell_packet.com_runtime_claimed,
        }
    }
}

/// OxIde-side DnaOneCalc web-shell hosting contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DnaOneCalcWebShellHostPacket {
    pub embedded_ide: EmbeddedIdePacket,
    pub web_shell: DnaOneCalcWebShellMountContract,
    pub required_mount_inputs: Vec<String>,
    pub limitations: Vec<String>,
}

impl DnaOneCalcWebShellHostPacket {
    pub fn from_packets(
        embedded_ide: EmbeddedIdePacket,
        shell_packet: &GuiShellPacket,
        dom_readiness: WebShellDomReadinessSummary,
    ) -> Self {
        let web_shell =
            DnaOneCalcWebShellMountContract::from_shell_packet(shell_packet, dom_readiness);
        Self {
            embedded_ide,
            web_shell,
            required_mount_inputs: vec![
                String::from("EmbeddedIdePacket"),
                String::from("GuiShellPacket"),
                String::from("oxide-webshell snapshot or mounted component"),
            ],
            limitations: vec![
                String::from("OxIde-side W310 contract did not modify DnaOneCalc files"),
                String::from("DnaOneCalc browser host smoke is not claimed"),
                String::from("parsed HTML DOM smoke is not a full accessibility audit"),
                String::from("browser-safe profile has no direct filesystem persistence"),
                String::from("native OxVba runtime/debug/Immediate provider unavailable"),
                String::from("COM runtime unavailable in browser-safe profile"),
            ],
        }
    }

    pub fn ownership_owner_names(&self) -> Vec<&str> {
        self.embedded_ide.ownership_owner_names()
    }

    pub fn sibling_repo_writes(&self) -> bool {
        self.embedded_ide.sibling_repo_writes || self.web_shell.sibling_repo_writes
    }
}

pub fn default_embedding_surfaces() -> Vec<EmbeddedIdeSurface> {
    vec![
        EmbeddedIdeSurface::new(EmbeddedIdeSurfaceKind::ProjectSpine, "Project spine"),
        EmbeddedIdeSurface::new(EmbeddedIdeSurfaceKind::SourceEditor, "Source editor"),
        EmbeddedIdeSurface::new(EmbeddedIdeSurfaceKind::Diagnostics, "Diagnostics"),
        EmbeddedIdeSurface::new(
            EmbeddedIdeSurfaceKind::DocumentLifecycle,
            "Document lifecycle",
        ),
        EmbeddedIdeSurface::new(EmbeddedIdeSurfaceKind::RunOutput, "Run output"),
        EmbeddedIdeSurface::new(
            EmbeddedIdeSurfaceKind::CapabilityFooter,
            "Capability footer",
        ),
    ]
}

fn module_stem(module_display_name: &str) -> String {
    module_display_name
        .strip_suffix(".bas")
        .unwrap_or(module_display_name)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxide_core::{
        GuiShellDiagnosticSummary, GuiShellModuleSummary, RunProviderKind, RunTranscriptStatus,
    };

    fn packet() -> EmbeddedIdePacket {
        EmbeddedIdePacket::dnaonecalc_thin_slice_browser_disabled(
            "examples/thin-slice/ThinSliceHello.basproj",
            "ThinSliceHello",
            "Module1.bas",
            "Option Explicit\nPublic Sub Main()\nEnd Sub\n",
        )
    }

    fn shell_packet() -> GuiShellPacket {
        GuiShellPacket::browser_safe_baseline(
            "examples/thin-slice/ThinSliceHello.basproj",
            "ThinSliceHello",
            vec![GuiShellModuleSummary::new("Module1.bas", true)],
            "Module1.bas",
            "Module1",
            "Option Explicit\nPublic Sub Main()\nEnd Sub\n",
            vec![GuiShellDiagnosticSummary::new(
                "info",
                "shell packet baseline keeps diagnostics surface available",
                "OxIde GUI shell packet",
            )],
        )
    }

    fn web_host_packet() -> DnaOneCalcWebShellHostPacket {
        DnaOneCalcWebShellHostPacket::from_packets(
            packet(),
            &shell_packet(),
            WebShellDomReadinessSummary::parsed_html_all_passed(),
        )
    }

    #[test]
    fn bridge_role_consumes_core_state() {
        assert!(OxideBridgeRole::EmbeddedHostBoundary.consumes_core_state());
    }

    #[test]
    fn dnaonecalc_packet_round_trips_through_json() {
        let packet = packet();

        let encoded = serde_json::to_string(&packet).expect("serialize embedded packet");
        let decoded: EmbeddedIdePacket =
            serde_json::from_str(&encoded).expect("deserialize embedded packet");

        assert_eq!(decoded, packet);
    }

    #[test]
    fn dnaonecalc_packet_names_required_surface_slots() {
        let packet = packet();
        let slots = packet.surface_slot_ids();

        assert_eq!(
            slots,
            vec![
                "project-spine",
                "source-editor",
                "diagnostics",
                "document-lifecycle",
                "run-output",
                "capability-footer"
            ]
        );
        assert!(
            packet
                .surfaces
                .iter()
                .all(|surface| surface.owner == "OxIde")
        );
    }

    #[test]
    fn dnaonecalc_packet_preserves_project_document_identity() {
        let packet = packet();

        assert_eq!(packet.consumer.host_name, "DnaOneCalc");
        assert_eq!(packet.project_document.project_name, "ThinSliceHello");
        assert_eq!(packet.project_document.active_module, "Module1");
        assert_eq!(packet.project_document.document_display_name, "Module1.bas");
        assert!(
            packet
                .project_document
                .workspace_path
                .ends_with("ThinSliceHello.basproj")
        );
    }

    #[test]
    fn dnaonecalc_packet_reuses_core_session_and_run_state() {
        let packet = packet();

        assert_eq!(
            packet.session_snapshot.capability_profile,
            SessionCapabilityProfile::BrowserLimited
        );
        assert!(!packet.session_snapshot.is_dirty());
        assert_eq!(
            packet.run_capability.provider_kind,
            RunProviderKind::BrowserUnsupported
        );
        assert!(!packet.run_capability.can_run);
        assert!(!packet.run_capability.native_execution_available);
        assert!(!packet.run_capability.com_runtime_available);
        assert_eq!(packet.run_transcript.status, RunTranscriptStatus::Disabled);
        assert_eq!(packet.run_transcript.provider_label, "browser-unsupported");
        assert!(packet.run_transcript.events.iter().any(|event| {
            event
                .message
                .contains("native execution provider unavailable")
        }));
    }

    #[test]
    fn dnaonecalc_web_shell_host_packet_round_trips_through_json() {
        let packet = web_host_packet();

        let encoded = serde_json::to_string(&packet).expect("serialize web host packet");
        let decoded: DnaOneCalcWebShellHostPacket =
            serde_json::from_str(&encoded).expect("deserialize web host packet");

        assert_eq!(decoded, packet);
    }

    #[test]
    fn dnaonecalc_web_shell_host_packet_composes_embedding_and_shell_contracts() {
        let packet = web_host_packet();

        assert_eq!(packet.embedded_ide.consumer.host_name, "DnaOneCalc");
        assert_eq!(packet.web_shell.state_contract, "GuiShellPacket");
        assert_eq!(packet.web_shell.adapter_crate, "oxide-webshell");
        assert_eq!(packet.web_shell.project_name, "ThinSliceHello");
        assert_eq!(packet.web_shell.active_module, "Module1.bas");
        assert_eq!(packet.web_shell.module_count, 1);
        assert_eq!(packet.web_shell.command_count, 10);
        assert_eq!(packet.web_shell.keybinding_count, 11);
        assert_eq!(packet.web_shell.focus_route_length, 10);
        assert_eq!(packet.web_shell.accessibility_surface_count, 10);
        assert!(
            packet
                .required_mount_inputs
                .iter()
                .any(|input| input == "EmbeddedIdePacket")
        );
        assert!(
            packet
                .required_mount_inputs
                .iter()
                .any(|input| input == "GuiShellPacket")
        );
    }

    #[test]
    fn dnaonecalc_web_shell_host_packet_preserves_boundaries_and_no_claim_flags() {
        let packet = web_host_packet();

        assert_eq!(
            packet.ownership_owner_names(),
            vec!["DnaOneCalc", "OxIde", "OxVba"]
        );
        assert!(!packet.sibling_repo_writes());
        assert!(!packet.web_shell.host_mount_claimed);
        assert!(!packet.web_shell.filesystem_persistence_claimed);
        assert!(!packet.web_shell.native_runtime_claimed);
        assert!(!packet.web_shell.com_runtime_claimed);
        assert!(!packet.web_shell.dom_accessibility_audit_claimed);
        assert!(packet.web_shell.dom_readiness.all_passed());
        assert!(packet.web_shell.dom_readiness.static_shell_dom_smoke_passed);
        assert!(
            packet
                .web_shell
                .dom_readiness
                .command_palette_dom_smoke_passed
        );
        assert!(
            packet
                .web_shell
                .dom_readiness
                .no_mouse_accessibility_dom_smoke_passed
        );
        assert!(!packet.web_shell.dom_readiness.browser_runtime_claimed);
        assert!(
            !packet
                .web_shell
                .dom_readiness
                .dom_accessibility_audit_claimed
        );
        assert!(
            packet
                .limitations
                .iter()
                .any(|limitation| limitation.contains("did not modify DnaOneCalc files"))
        );
        assert!(
            packet
                .limitations
                .iter()
                .any(|limitation| limitation.contains("browser host smoke is not claimed"))
        );
    }

    #[test]
    fn dnaonecalc_packet_names_ownership_boundaries_without_sibling_writes() {
        let packet = packet();
        let owners = packet.ownership_owner_names();

        assert_eq!(owners, vec!["DnaOneCalc", "OxIde", "OxVba"]);
        assert!(!packet.sibling_repo_writes);
        assert!(
            packet
                .limitations
                .iter()
                .any(|limitation| limitation.contains("did not modify DnaOneCalc files"))
        );
        assert!(
            packet
                .ownership_boundaries
                .iter()
                .any(|boundary| boundary.owner == "OxVba"
                    && boundary.responsibility.contains("runtime truth"))
        );
    }
}
