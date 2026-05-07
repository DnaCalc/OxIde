//! Shared OxIde UI component boundary.
//!
//! W342 starts with deterministic HTML-string rendering over `GuiShellPacket`
//! so DnaOxIde, DnaOneCalc, and oxide-guilab can review the same UI contract
//! without coupling this crate to Tauri or app-specific host code.

use oxide_core::{GuiShellPacket, GuiShellModuleSummary};

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
    OxVbaAvailableSubset { surface: &'static str, evidence: &'static str },
    PendingOxVbaHardening { gap: &'static str },
    UnavailableNoClaim { reason: &'static str },
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

/// Render the first shared shell surface around `GuiShellPacket`.
pub fn render_shared_shell(packet: &GuiShellPacket, provenance: UiDataProvenance) -> SharedShellRender {
    let role = OxideUiLeptosRole::SharedIdeComponentBoundary;
    let mut markup = String::new();
    markup.push_str("<section role=\"shared-ide-shell\" data-component-crate=\"");
    markup.push_str(role.crate_name());
    markup.push_str("\" data-source=\"GuiShellPacket\" data-provenance=\"");
    markup.push_str(provenance.label());
    markup.push_str("\" data-provenance-detail=\"");
    markup.push_str(&html_escape(provenance.detail()));
    markup.push_str("\" data-tauri-coupled=\"false\" data-dnaonecalc-reusable=\"true\" data-native-runtime=\"");
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
    if value { "true" } else { "false" }
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
    use oxide_core::{GuiShellDiagnosticSummary, GuiShellModuleSummary};

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
}
