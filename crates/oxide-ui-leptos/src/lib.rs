//! Shared OxIde UI component boundary.
//!
//! W342 starts with deterministic HTML-string rendering over `GuiShellPacket`
//! so DnaOxIde, DnaOneCalc, and oxide-guilab can review the same UI contract
//! without coupling this crate to Tauri or app-specific host code.

use oxide_core::{
    DebugServicePacket, GuiShellModuleSummary, GuiShellPacket, ImmediateServicePacket,
    RuntimeServicePacket,
};

/// Compile-time marker for the shared UI crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OxideUiLeptosRole {
    /// Shared IDE UI components over OxIde packets/view models.
    SharedIdeComponentBoundary,
}

impl OxideUiLeptosRole {
    pub fn crate_name(self) -> &'static str {
        match self {
            Self::SharedIdeComponentBoundary => "oxide-ui-leptos",
        }
    }

    pub fn tauri_coupled(self) -> bool {
        false
    }

    pub fn dnaonecalc_reusable(self) -> bool {
        true
    }
}

/// Provenance label for data rendered by shared UI components.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UiDataProvenance {
    ProvenOxideState,
    OxVbaAvailableSubset {
        surface: &'static str,
        evidence: &'static str,
    },
    PendingOxVbaHardening {
        gap: &'static str,
    },
    UnavailableNoClaim {
        reason: &'static str,
    },
}

impl UiDataProvenance {
    pub fn label(&self) -> &'static str {
        match self {
            Self::ProvenOxideState => "proven-oxide-state",
            Self::OxVbaAvailableSubset { .. } => "oxvba-available-subset",
            Self::PendingOxVbaHardening { .. } => "pending-oxvba-hardening",
            Self::UnavailableNoClaim { .. } => "unavailable-no-claim",
        }
    }

    pub fn detail(&self) -> &'static str {
        match self {
            Self::ProvenOxideState => "state proven inside OxIde tests",
            Self::OxVbaAvailableSubset { evidence, .. } => evidence,
            Self::PendingOxVbaHardening { gap } => gap,
            Self::UnavailableNoClaim { reason } => reason,
        }
    }
}

/// Deterministic render output for a shared shell component.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedShellRender {
    markup: String,
    pub component_crate: &'static str,
    pub source_contract: &'static str,
    pub provenance_label: &'static str,
    pub tauri_coupled: bool,
    pub dnaonecalc_reusable: bool,
    pub native_runtime_claimed: bool,
    pub com_runtime_claimed: bool,
    pub fake_immediate_responses: bool,
    pub fake_debug_data: bool,
}

impl SharedShellRender {
    pub fn markup(&self) -> &str {
        &self.markup
    }
}

/// Full accepted-pane model for the shared IDE surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedIdeSurfaceModel {
    pub shell: GuiShellPacket,
    pub runtime: RuntimeServicePacket,
    pub immediate: ImmediateServicePacket,
    pub debug: DebugServicePacket,
    pub provenance: UiDataProvenance,
}

/// Deterministic render output for accepted shared IDE panes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedIdeSurfaceRender {
    markup: String,
    pub component_crate: &'static str,
    pub source_contracts: Vec<&'static str>,
    pub provenance_label: &'static str,
    pub native_runtime_claimed: bool,
    pub com_runtime_claimed: bool,
    pub fake_immediate_responses: bool,
    pub fake_debug_data: bool,
}

impl SharedIdeSurfaceRender {
    pub fn markup(&self) -> &str {
        &self.markup
    }
}

/// Render the first shared shell surface around `GuiShellPacket`.
pub fn render_shared_shell(
    packet: &GuiShellPacket,
    provenance: UiDataProvenance,
) -> SharedShellRender {
    let role = OxideUiLeptosRole::SharedIdeComponentBoundary;
    let mut markup = String::new();
    markup.push_str("<section role=\"shared-ide-shell\" data-component-crate=\"");
    markup.push_str(role.crate_name());
    markup.push_str("\" data-source=\"GuiShellPacket\" data-provenance=\"");
    markup.push_str(provenance.label());
    markup.push_str("\" data-provenance-detail=\"");
    markup.push_str(&html_escape(provenance.detail()));
    markup.push_str(
        "\" data-tauri-coupled=\"false\" data-dnaonecalc-reusable=\"true\" data-native-runtime=\"",
    );
    markup.push_str(bool_attr(packet.native_execution_claimed));
    markup.push_str("\" data-com-runtime=\"");
    markup.push_str(bool_attr(packet.com_runtime_claimed));
    markup.push_str("\" data-fake-responses=\"false\" data-fake-debug-data=\"false\">\n");

    render_shell_header(&mut markup, packet);
    render_project_spine(&mut markup, packet);
    render_editor_boundary(&mut markup, packet);
    render_diagnostics_summary(&mut markup, packet);
    render_lifecycle_summary(&mut markup, packet);
    render_capability_footer(&mut markup, packet);

    markup.push_str("</section>\n");

    SharedShellRender {
        markup,
        component_crate: role.crate_name(),
        source_contract: "GuiShellPacket",
        provenance_label: provenance.label(),
        tauri_coupled: role.tauri_coupled(),
        dnaonecalc_reusable: role.dnaonecalc_reusable(),
        native_runtime_claimed: packet.native_execution_claimed,
        com_runtime_claimed: packet.com_runtime_claimed,
        fake_immediate_responses: false,
        fake_debug_data: false,
    }
}

/// Render all accepted W210-W330 pane categories that have pure packet inputs.
pub fn render_shared_ide_surface(model: &SharedIdeSurfaceModel) -> SharedIdeSurfaceRender {
    let role = OxideUiLeptosRole::SharedIdeComponentBoundary;
    let mut markup = String::new();
    markup.push_str("<section role=\"shared-ide-surface\" data-component-crate=\"");
    markup.push_str(role.crate_name());
    markup.push_str("\" data-source=\"GuiShellPacket+RuntimeServicePacket+ImmediateServicePacket+DebugServicePacket\" data-provenance=\"");
    markup.push_str(model.provenance.label());
    markup.push_str("\" data-native-runtime=\"");
    markup.push_str(bool_attr(
        model.runtime.native_runtime_claimed
            || model.immediate.native_runtime_claimed
            || model.debug.native_runtime_claimed,
    ));
    markup.push_str("\" data-com-runtime=\"");
    markup.push_str(bool_attr(
        model.runtime.com_runtime_claimed
            || model.immediate.com_runtime_claimed
            || model.debug.com_runtime_claimed
            || model.shell.com_runtime_claimed,
    ));
    markup.push_str("\" data-fake-responses=\"");
    markup.push_str(bool_attr(model.immediate.fake_responses));
    markup.push_str("\" data-fake-debug-data=\"");
    markup.push_str(bool_attr(model.debug.fake_debug_data));
    markup.push_str("\">\n");

    render_shell_header(&mut markup, &model.shell);
    render_project_spine(&mut markup, &model.shell);
    render_editor_boundary(&mut markup, &model.shell);
    render_diagnostics_summary(&mut markup, &model.shell);
    render_lifecycle_summary(&mut markup, &model.shell);
    render_run_panel(&mut markup, &model.shell);
    render_command_palette_summary(&mut markup, &model.shell);
    render_focus_accessibility_summary(&mut markup, &model.shell);
    render_runtime_service_panel(&mut markup, &model.runtime);
    render_immediate_service_panel(&mut markup, &model.immediate);
    render_debug_service_panel(&mut markup, &model.debug);
    render_com_capability_panel(&mut markup, &model.shell);
    render_capability_footer(&mut markup, &model.shell);

    markup.push_str("</section>\n");

    SharedIdeSurfaceRender {
        markup,
        component_crate: role.crate_name(),
        source_contracts: vec![
            "GuiShellPacket",
            "RuntimeServicePacket",
            "ImmediateServicePacket",
            "DebugServicePacket",
        ],
        provenance_label: model.provenance.label(),
        native_runtime_claimed: model.runtime.native_runtime_claimed
            || model.immediate.native_runtime_claimed
            || model.debug.native_runtime_claimed,
        com_runtime_claimed: model.runtime.com_runtime_claimed
            || model.immediate.com_runtime_claimed
            || model.debug.com_runtime_claimed
            || model.shell.com_runtime_claimed,
        fake_immediate_responses: model.immediate.fake_responses,
        fake_debug_data: model.debug.fake_debug_data,
    }
}

fn render_shell_header(markup: &mut String, packet: &GuiShellPacket) {
    markup.push_str("  <header role=\"shared-shell-header\">\n");
    markup.push_str("    <h1>OxIde Shared IDE</h1>\n");
    markup.push_str("    <p role=\"shared-project-title\">");
    markup.push_str(&html_escape(&packet.project_name));
    markup.push_str("</p>\n");
    markup.push_str("  </header>\n");
}

fn render_project_spine(markup: &mut String, packet: &GuiShellPacket) {
    markup.push_str("  <nav role=\"shared-project-spine\" data-project=\"");
    markup.push_str(&html_escape(&packet.project_name));
    markup.push_str("\" data-active-module=\"");
    markup.push_str(&html_escape(&packet.active_module));
    markup.push_str("\">\n");
    for module in &packet.modules {
        render_module(markup, module);
    }
    markup.push_str("  </nav>\n");
}

fn render_module(markup: &mut String, module: &GuiShellModuleSummary) {
    markup.push_str("    <div role=\"shared-project-module\" data-active=\"");
    markup.push_str(bool_attr(module.is_active));
    markup.push_str("\">");
    markup.push_str(&html_escape(&module.display_name));
    markup.push_str("</div>\n");
}

fn render_editor_boundary(markup: &mut String, packet: &GuiShellPacket) {
    markup.push_str("  <section role=\"shared-editor-boundary\" data-active-module=\"");
    markup.push_str(&html_escape(&packet.active_module));
    markup.push_str("\" data-source-bytes=\"");
    markup.push_str(&packet.source_text.len().to_string());
    markup.push_str("\">\n");
    markup.push_str("    <pre role=\"shared-source-preview\">");
    markup.push_str(&html_escape(source_preview(&packet.source_text)));
    markup.push_str("</pre>\n");
    markup.push_str("  </section>\n");
}

fn render_diagnostics_summary(markup: &mut String, packet: &GuiShellPacket) {
    markup.push_str("  <section role=\"shared-diagnostics-summary\" data-count=\"");
    markup.push_str(&packet.diagnostics.len().to_string());
    markup.push_str("\">\n");
    for diagnostic in &packet.diagnostics {
        markup.push_str("    <div role=\"shared-diagnostic-row\" data-severity=\"");
        markup.push_str(&html_escape(&diagnostic.severity_label));
        markup.push_str("\" data-provenance=\"");
        markup.push_str(&html_escape(&diagnostic.provenance_label));
        markup.push_str("\">");
        markup.push_str(&html_escape(&diagnostic.message));
        markup.push_str("</div>\n");
    }
    markup.push_str("  </section>\n");
}

fn render_lifecycle_summary(markup: &mut String, packet: &GuiShellPacket) {
    markup.push_str("  <section role=\"shared-lifecycle-summary\" data-profile=\"");
    markup.push_str(&html_escape(&packet.lifecycle_profile_label));
    markup.push_str("\">\n");
    for command in &packet.lifecycle_commands {
        markup.push_str("    <div role=\"shared-lifecycle-command\" data-command=\"");
        markup.push_str(&html_escape(&command.command_name));
        markup.push_str("\" data-enabled=\"");
        markup.push_str(bool_attr(command.is_enabled));
        markup.push_str("\">");
        if let Some(reason) = &command.disabled_reason {
            markup.push_str(&html_escape(reason));
        }
        markup.push_str("</div>\n");
    }
    markup.push_str("  </section>\n");
}

fn render_run_panel(markup: &mut String, packet: &GuiShellPacket) {
    markup.push_str("  <section role=\"shared-run-output\" data-provider=\"");
    markup.push_str(&html_escape(&packet.run_transcript.provider_label));
    markup.push_str("\" data-status=\"");
    markup.push_str(packet.run_transcript.status.label());
    markup.push_str("\" data-event-count=\"");
    markup.push_str(&packet.run_transcript.events.len().to_string());
    markup.push_str("\" data-native-execution=\"");
    markup.push_str(bool_attr(packet.run_capability.native_execution_available));
    markup.push_str("\">\n");
    markup.push_str("    <div role=\"shared-run-target\">");
    markup.push_str(&html_escape(
        &packet.run_transcript.request.display_target(),
    ));
    markup.push_str("</div>\n");
    markup.push_str("  </section>\n");
}

fn render_command_palette_summary(markup: &mut String, packet: &GuiShellPacket) {
    markup.push_str("  <section role=\"shared-command-palette\" data-command-count=\"");
    markup.push_str(&packet.command_palette.commands.len().to_string());
    markup.push_str("\" data-parked-tui-imported=\"");
    markup.push_str(bool_attr(packet.command_palette.parked_tui_imported));
    markup.push_str("\">\n");
    for command in &packet.command_palette.commands {
        markup.push_str("    <div role=\"shared-command\" data-command=\"");
        markup.push_str(&html_escape(&command.stable_id));
        markup.push_str("\" data-enabled=\"");
        markup.push_str(bool_attr(command.availability.is_enabled));
        markup.push_str("\">");
        markup.push_str(&html_escape(&command.label));
        markup.push_str("</div>\n");
    }
    markup.push_str("  </section>\n");
}

fn render_focus_accessibility_summary(markup: &mut String, packet: &GuiShellPacket) {
    markup.push_str("  <section role=\"shared-focus-accessibility\" data-focus-node-count=\"");
    markup.push_str(&packet.focus_graph.nodes.len().to_string());
    markup.push_str("\" data-route-length=\"");
    markup.push_str(&packet.focus_graph.no_mouse_route.len().to_string());
    markup.push_str("\" data-accessibility-node-count=\"");
    markup.push_str(&packet.accessibility.nodes.len().to_string());
    markup.push_str("\" data-dom-audited=\"false\">\n");
    markup.push_str("  </section>\n");
}

fn render_runtime_service_panel(markup: &mut String, runtime: &RuntimeServicePacket) {
    markup.push_str("  <section role=\"shared-runtime-service\" data-provider=\"");
    markup.push_str(runtime.provider_label());
    markup.push_str("\" data-command-enabled=\"");
    markup.push_str(bool_attr(runtime.command_status.is_enabled));
    markup.push_str("\" data-real-execution=\"");
    markup.push_str(bool_attr(runtime.real_execution_claimed));
    markup.push_str("\" data-native-runtime=\"");
    markup.push_str(bool_attr(runtime.native_runtime_claimed));
    markup.push_str("\" data-com-runtime=\"");
    markup.push_str(bool_attr(runtime.com_runtime_claimed));
    markup.push_str("\" data-event-count=\"");
    markup.push_str(&runtime.events.len().to_string());
    markup.push_str("\">\n");
    if let Some(reason) = &runtime.command_status.disabled_reason {
        markup.push_str("    <div role=\"shared-runtime-disabled-reason\">");
        markup.push_str(&html_escape(reason));
        markup.push_str("</div>\n");
    }
    markup.push_str("  </section>\n");
}

fn render_immediate_service_panel(markup: &mut String, immediate: &ImmediateServicePacket) {
    markup.push_str("  <section role=\"shared-immediate-service\" data-provider=\"");
    markup.push_str(immediate.provider_label());
    markup.push_str("\" data-command-enabled=\"");
    markup.push_str(bool_attr(immediate.command_status.is_enabled));
    markup.push_str("\" data-response-count=\"");
    markup.push_str(&immediate.responses.len().to_string());
    markup.push_str("\" data-fake-responses=\"");
    markup.push_str(bool_attr(immediate.fake_responses));
    markup.push_str("\" data-native-runtime=\"");
    markup.push_str(bool_attr(immediate.native_runtime_claimed));
    markup.push_str("\" data-com-runtime=\"");
    markup.push_str(bool_attr(immediate.com_runtime_claimed));
    markup.push_str("\">\n");
    if let Some(request_text) = &immediate.request_text {
        markup.push_str("    <div role=\"shared-immediate-request\">");
        markup.push_str(&html_escape(request_text));
        markup.push_str("</div>\n");
    }
    markup.push_str("  </section>\n");
}

fn render_debug_service_panel(markup: &mut String, debug: &DebugServicePacket) {
    markup.push_str("  <section role=\"shared-debug-service\" data-provider=\"");
    markup.push_str(debug.provider_label());
    markup.push_str("\" data-state=\"");
    markup.push_str(debug.state_label());
    markup.push_str("\" data-command-enabled=\"");
    markup.push_str(bool_attr(debug.command_status.is_enabled));
    markup.push_str("\" data-callstack-count=\"");
    markup.push_str(&debug.callstack.len().to_string());
    markup.push_str("\" data-locals-count=\"");
    markup.push_str(&debug.locals.len().to_string());
    markup.push_str("\" data-watches-count=\"");
    markup.push_str(&debug.watches.len().to_string());
    markup.push_str("\" data-breakpoints-count=\"");
    markup.push_str(&debug.breakpoints.len().to_string());
    markup.push_str("\" data-fake-debug-data=\"");
    markup.push_str(bool_attr(debug.fake_debug_data));
    markup.push_str("\" data-native-runtime=\"");
    markup.push_str(bool_attr(debug.native_runtime_claimed));
    markup.push_str("\" data-com-runtime=\"");
    markup.push_str(bool_attr(debug.com_runtime_claimed));
    markup.push_str("\">\n");
    markup.push_str("  </section>\n");
}

fn render_com_capability_panel(markup: &mut String, packet: &GuiShellPacket) {
    markup.push_str("  <section role=\"shared-com-capability\" data-host-kind=\"");
    markup.push_str(packet.com_capability.host_kind.label());
    markup.push_str("\" data-reference-discovery=\"");
    markup.push_str(bool_attr(
        packet.com_capability.reference_discovery.is_available,
    ));
    markup.push_str("\" data-runtime-available=\"");
    markup.push_str(bool_attr(
        packet.com_capability.runtime_invocation.is_available,
    ));
    markup.push_str("\">\n");
    markup.push_str("    <div role=\"shared-com-reference\">");
    markup.push_str(&html_escape(&packet.com_capability.reference.display_name));
    markup.push_str("</div>\n");
    markup.push_str("  </section>\n");
}

fn render_capability_footer(markup: &mut String, packet: &GuiShellPacket) {
    markup.push_str("  <footer role=\"shared-capability-footer\">");
    markup.push_str(&html_escape(&packet.capability_footer));
    markup.push_str("</footer>\n");
}

fn source_preview(source: &str) -> &str {
    source
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or("")
}

fn bool_attr(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

fn html_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxide_core::{
        DebugServicePacket, GuiShellDiagnosticSummary, GuiShellModuleSummary,
        ImmediateServicePacket, RuntimeServicePacket,
    };

    fn shell_packet() -> GuiShellPacket {
        GuiShellPacket::browser_safe_baseline(
            "examples/thin-slice",
            "ThinSliceHello",
            vec![GuiShellModuleSummary::new("Module1.bas", true)],
            "Module1.bas",
            "Module1",
            "Public Sub Main()\n    Debug.Print 42\nEnd Sub\n",
            vec![GuiShellDiagnosticSummary::new(
                "info",
                "diagnostic projection ready",
                "OxIde test",
            )],
        )
    }

    #[test]
    fn role_is_shared_and_not_tauri_coupled() {
        let role = OxideUiLeptosRole::SharedIdeComponentBoundary;
        assert_eq!(role.crate_name(), "oxide-ui-leptos");
        assert!(!role.tauri_coupled());
        assert!(role.dnaonecalc_reusable());
    }

    #[test]
    fn shared_shell_renders_packet_identity_and_footer() {
        let packet = shell_packet();
        let render = render_shared_shell(&packet, UiDataProvenance::ProvenOxideState);
        let markup = render.markup();

        assert_eq!(render.component_crate, "oxide-ui-leptos");
        assert_eq!(render.source_contract, "GuiShellPacket");
        assert_eq!(render.provenance_label, "proven-oxide-state");
        assert!(!render.tauri_coupled);
        assert!(render.dnaonecalc_reusable);
        assert!(!render.native_runtime_claimed);
        assert!(!render.com_runtime_claimed);
        assert!(!render.fake_immediate_responses);
        assert!(!render.fake_debug_data);

        assert!(markup.contains("role=\"shared-ide-shell\""));
        assert!(markup.contains("data-component-crate=\"oxide-ui-leptos\""));
        assert!(markup.contains("data-source=\"GuiShellPacket\""));
        assert!(markup.contains("data-tauri-coupled=\"false\""));
        assert!(markup.contains("data-dnaonecalc-reusable=\"true\""));
        assert!(markup.contains("ThinSliceHello"));
        assert!(markup.contains("Module1.bas"));
        assert!(markup.contains("Public Sub Main()"));
        assert!(markup.contains("data-count=\"1\""));
        assert!(markup.contains("diagnostic projection ready"));
        assert!(markup.contains("Browser-safe profile"));
    }

    fn surface_model() -> SharedIdeSurfaceModel {
        SharedIdeSurfaceModel {
            shell: shell_packet(),
            runtime: RuntimeServicePacket::native_service_missing(
                "examples/thin-slice",
                "ThinSliceHello",
                "Module1",
                "Main",
            ),
            immediate: ImmediateServicePacket::native_service_missing(Some("?answer".to_string())),
            debug: DebugServicePacket::native_service_missing(),
            provenance: UiDataProvenance::PendingOxVbaHardening {
                gap: "runtime/debug/Immediate/COM stable DTOs pending",
            },
        }
    }

    #[test]
    fn shared_shell_can_label_oxvba_available_subset_without_full_claims() {
        let packet = shell_packet();
        let render = render_shared_shell(
            &packet,
            UiDataProvenance::OxVbaAvailableSubset {
                surface: "HostWorkspaceSession",
                evidence: "direct host session subset evidence",
            },
        );
        let markup = render.markup();

        assert_eq!(render.provenance_label, "oxvba-available-subset");
        assert!(markup.contains("data-provenance=\"oxvba-available-subset\""));
        assert!(markup.contains("direct host session subset evidence"));
        assert!(markup.contains("data-native-runtime=\"false\""));
        assert!(markup.contains("data-com-runtime=\"false\""));
        assert!(markup.contains("data-fake-responses=\"false\""));
        assert!(markup.contains("data-fake-debug-data=\"false\""));
    }

    #[test]
    fn shared_ide_surface_renders_accepted_panes_without_fake_data() {
        let model = surface_model();
        let render = render_shared_ide_surface(&model);
        let markup = render.markup();

        assert_eq!(render.component_crate, "oxide-ui-leptos");
        assert_eq!(render.provenance_label, "pending-oxvba-hardening");
        assert!(render.source_contracts.contains(&"GuiShellPacket"));
        assert!(render.source_contracts.contains(&"RuntimeServicePacket"));
        assert!(render.source_contracts.contains(&"ImmediateServicePacket"));
        assert!(render.source_contracts.contains(&"DebugServicePacket"));
        assert!(!render.native_runtime_claimed);
        assert!(!render.com_runtime_claimed);
        assert!(!render.fake_immediate_responses);
        assert!(!render.fake_debug_data);

        for role in [
            "shared-ide-surface",
            "shared-project-spine",
            "shared-editor-boundary",
            "shared-diagnostics-summary",
            "shared-lifecycle-summary",
            "shared-run-output",
            "shared-command-palette",
            "shared-focus-accessibility",
            "shared-runtime-service",
            "shared-immediate-service",
            "shared-debug-service",
            "shared-com-capability",
            "shared-capability-footer",
        ] {
            assert!(
                markup.contains(&format!("role=\"{role}\"")),
                "missing role {role}"
            );
        }

        assert!(markup.contains("ThinSliceHello"));
        assert!(markup.contains("Module1.bas"));
        assert!(markup.contains("Public Sub Main()"));
        assert!(markup.contains("data-provider=\"native-service-missing\""));
        assert!(markup.contains("data-response-count=\"0\""));
        assert!(markup.contains("data-callstack-count=\"0\""));
        assert!(markup.contains("data-locals-count=\"0\""));
        assert!(markup.contains("data-watches-count=\"0\""));
        assert!(markup.contains("data-breakpoints-count=\"0\""));
        assert!(markup.contains("data-fake-responses=\"false\""));
        assert!(markup.contains("data-fake-debug-data=\"false\""));
        assert!(markup.contains("data-runtime-available=\"false\""));
        assert!(markup.contains("data-dom-audited=\"false\""));
    }
}
