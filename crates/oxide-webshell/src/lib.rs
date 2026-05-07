//! Thin web-shell adapter over `oxide-core::GuiShellPacket`.
//!
//! This crate deliberately renders packet-derived markup without choosing a
//! broad web framework. DOM/browser claims belong to later smoke tests.

use oxide_core::GuiShellPacket;
use scraper::{Html, Selector};

/// Compile-time marker for the web shell adapter crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OxideWebShellRole {
    /// Packet-to-web-shell rendering adapter.
    PacketRenderingAdapter,
}

/// Rendered web-shell boundary snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebShellSnapshot {
    markup: String,
    pub source_contract: &'static str,
    pub dom_smoke_tested: bool,
    pub dom_accessibility_audited: bool,
    pub filesystem_persistence_claimed: bool,
    pub native_runtime_claimed: bool,
    pub com_runtime_claimed: bool,
    pub parked_tui_imported: bool,
}

impl WebShellSnapshot {
    pub fn markup(&self) -> &str {
        &self.markup
    }
}

/// One deterministic parsed-HTML DOM smoke check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebShellDomSmokeCheck {
    pub name: String,
    pub passed: bool,
    pub detail: String,
}

impl WebShellDomSmokeCheck {
    fn passed(name: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            passed: true,
            detail: detail.into(),
        }
    }

    fn failed(name: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            passed: false,
            detail: detail.into(),
        }
    }
}

/// Parsed-HTML DOM smoke report for the static shell boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebShellDomSmokeReport {
    pub snapshot: WebShellSnapshot,
    pub smoke_kind: &'static str,
    pub dom_smoke_tested: bool,
    pub browser_runtime_claimed: bool,
    pub dom_accessibility_audited: bool,
    pub checks: Vec<WebShellDomSmokeCheck>,
}

impl WebShellDomSmokeReport {
    pub fn all_passed(&self) -> bool {
        self.checks.iter().all(|check| check.passed)
    }
}

/// Render a `GuiShellPacket` into deterministic web-shell markup.
pub fn render_web_shell_snapshot(packet: &GuiShellPacket) -> WebShellSnapshot {
    let mut markup = String::new();
    markup.push_str("<section role=\"web-shell-adapter\" data-source=\"GuiShellPacket\" data-web-framework=\"unselected\" data-dom-smoke-tested=\"false\" data-dom-audited=\"false\" data-filesystem-persistence=\"");
    markup.push_str(if packet.web_framework_bound {
        "true"
    } else {
        "false"
    });
    markup.push_str("\" data-native-runtime=\"");
    markup.push_str(if packet.native_execution_claimed {
        "true"
    } else {
        "false"
    });
    markup.push_str("\" data-com-runtime=\"");
    markup.push_str(if packet.com_runtime_claimed {
        "true"
    } else {
        "false"
    });
    markup.push_str("\" data-parked-tui-imported=\"");
    markup.push_str(if packet.parked_tui_imported {
        "true"
    } else {
        "false"
    });
    markup.push_str("\">\n");
    render_project_tree(&mut markup, packet);
    render_editor(&mut markup, packet);
    render_diagnostics(&mut markup, packet);
    render_lifecycle(&mut markup, packet);
    render_run_output(&mut markup, packet);
    render_com_capability(&mut markup, packet);
    render_command_summary(&mut markup, packet);
    render_focus_accessibility_summary(&mut markup, packet);
    markup.push_str("  <footer role=\"web-capability-footer\">");
    markup.push_str(&html_escape(&packet.capability_footer));
    markup.push_str("</footer>\n");
    markup.push_str("  <div role=\"web-shell-adapter-policy\">Web shell adapter consumes GuiShellPacket; no framework, DOM audit, filesystem persistence, native runtime, or COM runtime is claimed by this boundary.</div>\n");
    markup.push_str("</section>\n");

    WebShellSnapshot {
        markup,
        source_contract: "GuiShellPacket",
        dom_smoke_tested: false,
        dom_accessibility_audited: false,
        filesystem_persistence_claimed: packet.web_framework_bound,
        native_runtime_claimed: packet.native_execution_claimed,
        com_runtime_claimed: packet.com_runtime_claimed,
        parked_tui_imported: packet.parked_tui_imported,
    }
}

/// Parse the adapter markup as an HTML tree and verify the static shell
/// surface with DOM-selector checks. This is not a browser runtime claim and
/// not an accessibility audit.
pub fn run_static_shell_dom_smoke(packet: &GuiShellPacket) -> WebShellDomSmokeReport {
    let snapshot = render_web_shell_snapshot(packet);
    let document = Html::parse_fragment(snapshot.markup());
    let checks = vec![
        selector_attr_check(
            &document,
            "section[role='web-shell-adapter']",
            "data-source",
            "GuiShellPacket",
            "root consumes GuiShellPacket",
        ),
        selector_attr_check(
            &document,
            "section[role='web-shell-adapter']",
            "data-dom-smoke-tested",
            "false",
            "adapter boundary itself does not claim pre-smoked DOM",
        ),
        selector_attr_check(
            &document,
            "section[role='web-shell-adapter']",
            "data-dom-audited",
            "false",
            "DOM accessibility audit remains unclaimed",
        ),
        selector_attr_check(
            &document,
            "section[role='web-shell-adapter']",
            "data-filesystem-persistence",
            "false",
            "filesystem persistence remains unclaimed",
        ),
        selector_attr_check(
            &document,
            "section[role='web-shell-adapter']",
            "data-native-runtime",
            "false",
            "native runtime remains unclaimed",
        ),
        selector_attr_check(
            &document,
            "section[role='web-shell-adapter']",
            "data-com-runtime",
            "false",
            "COM runtime remains unclaimed",
        ),
        selector_attr_check(
            &document,
            "section[role='web-shell-adapter']",
            "data-parked-tui-imported",
            "false",
            "parked TUI state remains isolated",
        ),
        selector_attr_check(
            &document,
            "nav[role='web-project-tree']",
            "data-project",
            &packet.project_name,
            "project tree carries project name",
        ),
        selector_text_check(
            &document,
            "div[role='web-project-module']",
            &packet.active_module,
            "project tree shows active module",
        ),
        selector_text_check(
            &document,
            "pre[role='web-source-editor']",
            "Public Sub Main()",
            "source editor shows module source",
        ),
        selector_attr_check(
            &document,
            "section[role='web-diagnostics']",
            "data-count",
            &packet.diagnostics.len().to_string(),
            "diagnostics surface carries count",
        ),
        selector_attr_check(
            &document,
            "section[role='web-run-output']",
            "data-provider",
            packet.run_transcript.provider_label.as_str(),
            "run output carries provider",
        ),
        selector_attr_check(
            &document,
            "section[role='web-run-output']",
            "data-native-execution",
            "false",
            "run output keeps native execution unclaimed",
        ),
        selector_attr_check(
            &document,
            "section[role='web-com-capability']",
            "data-runtime-available",
            "false",
            "COM runtime remains unavailable",
        ),
        selector_attr_check(
            &document,
            "section[role='web-command-summary']",
            "data-command-count",
            &packet.command_palette.commands.len().to_string(),
            "command summary carries packet command count",
        ),
        selector_attr_check(
            &document,
            "section[role='web-focus-accessibility-summary']",
            "data-route-length",
            &packet.focus_graph.no_mouse_route.len().to_string(),
            "focus route length is packet-derived",
        ),
        selector_text_check(
            &document,
            "footer[role='web-capability-footer']",
            &packet.capability_footer,
            "capability footer remains visible",
        ),
    ];

    WebShellDomSmokeReport {
        snapshot,
        smoke_kind: "parsed-html-tree",
        dom_smoke_tested: true,
        browser_runtime_claimed: false,
        dom_accessibility_audited: false,
        checks,
    }
}

/// Parse the adapter markup as an HTML tree and verify command palette
/// command IDs, keyboard gestures, availability, and disabled reasons.
pub fn run_command_palette_dom_smoke(packet: &GuiShellPacket) -> WebShellDomSmokeReport {
    let snapshot = render_web_shell_snapshot(packet);
    let document = Html::parse_fragment(snapshot.markup());
    let checks = vec![
        selector_attr_check(
            &document,
            "section[role='web-command-summary']",
            "data-command-count",
            &packet.command_palette.commands.len().to_string(),
            "command summary carries packet command count",
        ),
        selector_attr_check(
            &document,
            "section[role='web-command-summary']",
            "data-keybinding-count",
            &packet.keyboard_map.bindings.len().to_string(),
            "command summary carries packet keybinding count",
        ),
        command_attr_check(
            &document,
            "project.open",
            "data-gesture",
            "Ctrl+O",
            "project.open gesture survives DOM mounting",
        ),
        command_text_check(
            &document,
            "project.open",
            "Open Project",
            "project.open label survives DOM mounting",
        ),
        command_attr_check(
            &document,
            "document.save",
            "data-gesture",
            "Ctrl+S",
            "document.save gesture survives DOM mounting",
        ),
        command_text_check(
            &document,
            "document.save",
            "browser-safe profile has no direct filesystem persistence",
            "document.save disabled reason remains visible",
        ),
        command_attr_check(
            &document,
            "runtime.run",
            "data-gesture",
            "F5",
            "runtime.run gesture survives DOM mounting",
        ),
        command_text_check(
            &document,
            "runtime.run",
            "native execution provider unavailable",
            "runtime.run disabled reason remains visible",
        ),
        command_attr_check(
            &document,
            "runtime.immediate",
            "data-gesture",
            "Enter",
            "runtime.immediate gesture survives DOM mounting",
        ),
        command_text_check(
            &document,
            "runtime.immediate",
            "no native OxVba runtime session",
            "runtime.immediate disabled reason remains visible",
        ),
        command_attr_check(
            &document,
            "runtime.debug",
            "data-gesture",
            "F10",
            "runtime.debug gesture survives DOM mounting",
        ),
        command_text_check(
            &document,
            "runtime.debug",
            "no OxVba debug session",
            "runtime.debug disabled reason remains visible",
        ),
        command_attr_check(
            &document,
            "shell.command_palette",
            "data-gesture",
            "Ctrl+Shift+P",
            "shell.command_palette gesture survives DOM mounting",
        ),
        command_text_check(
            &document,
            "shell.command_palette",
            "Command Palette",
            "shell.command_palette label survives DOM mounting",
        ),
        selector_attr_check(
            &document,
            "section[role='web-shell-adapter']",
            "data-parked-tui-imported",
            "false",
            "parked TUI command model remains isolated",
        ),
    ];

    WebShellDomSmokeReport {
        snapshot,
        smoke_kind: "parsed-html-command-palette",
        dom_smoke_tested: true,
        browser_runtime_claimed: false,
        dom_accessibility_audited: false,
        checks,
    }
}

fn command_attr_check(
    document: &Html,
    command_id: &str,
    attr: &str,
    expected: &str,
    name: &str,
) -> WebShellDomSmokeCheck {
    let selector = format!("div[role='web-command-summary-row'][data-command-id='{command_id}']");
    selector_attr_check(document, &selector, attr, expected, name)
}

fn command_text_check(
    document: &Html,
    command_id: &str,
    expected: &str,
    name: &str,
) -> WebShellDomSmokeCheck {
    let selector = format!("div[role='web-command-summary-row'][data-command-id='{command_id}']");
    selector_text_check(document, &selector, expected, name)
}

fn selector_attr_check(
    document: &Html,
    selector: &str,
    attr: &str,
    expected: &str,
    name: &str,
) -> WebShellDomSmokeCheck {
    let selector = Selector::parse(selector).expect("static selector parses");
    let Some(element) = document.select(&selector).next() else {
        return WebShellDomSmokeCheck::failed(name, "selector not found");
    };
    match element.value().attr(attr) {
        Some(actual) if actual == expected => {
            WebShellDomSmokeCheck::passed(name, format!("{attr}={actual}"))
        }
        Some(actual) => {
            WebShellDomSmokeCheck::failed(name, format!("expected {attr}={expected}, got {actual}"))
        }
        None => WebShellDomSmokeCheck::failed(name, format!("missing attribute {attr}")),
    }
}

fn selector_text_check(
    document: &Html,
    selector: &str,
    expected: &str,
    name: &str,
) -> WebShellDomSmokeCheck {
    let selector = Selector::parse(selector).expect("static selector parses");
    let Some(element) = document.select(&selector).next() else {
        return WebShellDomSmokeCheck::failed(name, "selector not found");
    };
    let text = element.text().collect::<String>();
    if text.contains(expected) {
        WebShellDomSmokeCheck::passed(name, expected)
    } else {
        WebShellDomSmokeCheck::failed(name, format!("expected text containing {expected}"))
    }
}

fn render_project_tree(markup: &mut String, packet: &GuiShellPacket) {
    markup.push_str("  <nav role=\"web-project-tree\" data-project=\"");
    markup.push_str(&html_escape(&packet.project_name));
    markup.push_str("\" data-module-count=\"");
    markup.push_str(&packet.modules.len().to_string());
    markup.push_str("\">\n");
    for module in &packet.modules {
        markup.push_str("    <div role=\"web-project-module\" data-active=\"");
        markup.push_str(if module.is_active { "true" } else { "false" });
        markup.push_str("\">");
        markup.push_str(&html_escape(&module.display_name));
        markup.push_str("</div>\n");
    }
    markup.push_str("  </nav>\n");
}

fn render_editor(markup: &mut String, packet: &GuiShellPacket) {
    markup.push_str("  <pre role=\"web-source-editor\" data-active-module=\"");
    markup.push_str(&html_escape(&packet.active_module));
    markup.push_str("\">");
    markup.push_str(&html_escape(&packet.source_text));
    markup.push_str("</pre>\n");
}

fn render_diagnostics(markup: &mut String, packet: &GuiShellPacket) {
    markup.push_str("  <section role=\"web-diagnostics\" data-count=\"");
    markup.push_str(&packet.diagnostics.len().to_string());
    markup.push_str("\">\n");
    for diagnostic in &packet.diagnostics {
        markup.push_str("    <div role=\"web-diagnostic-row\" data-severity=\"");
        markup.push_str(&html_escape(&diagnostic.severity_label));
        markup.push_str("\" data-provenance=\"");
        markup.push_str(&html_escape(&diagnostic.provenance_label));
        markup.push_str("\">");
        markup.push_str(&html_escape(&diagnostic.message));
        markup.push_str("</div>\n");
    }
    markup.push_str("  </section>\n");
}

fn render_lifecycle(markup: &mut String, packet: &GuiShellPacket) {
    markup.push_str("  <section role=\"web-lifecycle\" data-profile=\"");
    markup.push_str(&html_escape(&packet.lifecycle_profile_label));
    markup.push_str("\">\n");
    for command in &packet.lifecycle_commands {
        markup.push_str("    <button role=\"web-lifecycle-command\" data-command=\"");
        markup.push_str(&html_escape(&command.command_name));
        markup.push_str("\" data-enabled=\"");
        markup.push_str(if command.is_enabled { "true" } else { "false" });
        markup.push_str("\">");
        markup.push_str(&html_escape(&command.command_name));
        if let Some(reason) = &command.disabled_reason {
            markup.push_str(" disabled: ");
            markup.push_str(&html_escape(reason));
        }
        markup.push_str("</button>\n");
    }
    markup.push_str("  </section>\n");
}

fn render_run_output(markup: &mut String, packet: &GuiShellPacket) {
    markup.push_str("  <section role=\"web-run-output\" data-provider=\"");
    markup.push_str(packet.run_transcript.provider_label.as_str());
    markup.push_str("\" data-status=\"");
    markup.push_str(packet.run_transcript.status.label());
    markup.push_str("\" data-native-execution=\"");
    markup.push_str(if packet.run_capability.native_execution_available {
        "true"
    } else {
        "false"
    });
    markup.push_str("\" data-com-runtime=\"");
    markup.push_str(if packet.run_capability.com_runtime_available {
        "true"
    } else {
        "false"
    });
    markup.push_str("\">\n");
    for event in &packet.run_transcript.events {
        markup.push_str("    <div role=\"web-run-event\" data-event-kind=\"");
        markup.push_str(event.kind.label());
        markup.push_str("\">");
        markup.push_str(&html_escape(&event.message));
        markup.push_str("</div>\n");
    }
    markup.push_str("  </section>\n");
}

fn render_com_capability(markup: &mut String, packet: &GuiShellPacket) {
    markup.push_str("  <section role=\"web-com-capability\" data-profile=\"");
    markup.push_str(packet.com_capability.host_kind.label());
    markup.push_str("\" data-runtime-available=\"");
    markup.push_str(if packet.com_capability.runtime_invocation.is_available {
        "true"
    } else {
        "false"
    });
    markup.push_str("\">\n");
    markup.push_str("    <div role=\"web-com-reference\">");
    markup.push_str(&html_escape(&packet.com_capability.reference.display_name));
    markup.push_str("</div>\n");
    for feature in [
        &packet.com_capability.reference_discovery,
        &packet.com_capability.runtime_invocation,
    ] {
        markup.push_str("    <div role=\"web-com-feature\" data-feature=\"");
        markup.push_str(feature.feature.label());
        markup.push_str("\" data-available=\"");
        markup.push_str(if feature.is_available {
            "true"
        } else {
            "false"
        });
        markup.push_str("\">");
        if let Some(reason) = &feature.reason {
            markup.push_str(&html_escape(reason));
        }
        markup.push_str("</div>\n");
    }
    markup.push_str("  </section>\n");
}

fn render_command_summary(markup: &mut String, packet: &GuiShellPacket) {
    markup.push_str("  <section role=\"web-command-summary\" data-command-count=\"");
    markup.push_str(&packet.command_palette.commands.len().to_string());
    markup.push_str("\" data-keybinding-count=\"");
    markup.push_str(&packet.keyboard_map.bindings.len().to_string());
    markup.push_str("\">\n");
    for command in &packet.command_palette.commands {
        let gesture = packet
            .keyboard_map
            .bindings
            .iter()
            .find(|binding| binding.command_id == command.id)
            .map(|binding| binding.gesture.display.as_str())
            .unwrap_or("unbound");
        markup.push_str("    <div role=\"web-command-summary-row\" data-command-id=\"");
        markup.push_str(&html_escape(&command.stable_id));
        markup.push_str("\" data-gesture=\"");
        markup.push_str(&html_escape(gesture));
        markup.push_str("\" data-enabled=\"");
        markup.push_str(if command.availability.is_enabled {
            "true"
        } else {
            "false"
        });
        markup.push_str("\">\n");
        markup.push_str("      <span role=\"web-command-label\">");
        markup.push_str(&html_escape(&command.label));
        markup.push_str("</span>\n");
        if let Some(reason) = &command.availability.disabled_reason {
            markup.push_str("      <span role=\"web-command-disabled-reason\">");
            markup.push_str(&html_escape(reason));
            markup.push_str("</span>\n");
        }
        markup.push_str("    </div>\n");
    }
    markup.push_str("  </section>\n");
}

fn render_focus_accessibility_summary(markup: &mut String, packet: &GuiShellPacket) {
    markup.push_str("  <section role=\"web-focus-accessibility-summary\" data-focus-node-count=\"");
    markup.push_str(&packet.focus_graph.nodes.len().to_string());
    markup.push_str("\" data-route-length=\"");
    markup.push_str(&packet.focus_graph.no_mouse_route.len().to_string());
    markup.push_str("\" data-accessibility-surface-count=\"");
    markup.push_str(&packet.accessibility.nodes.len().to_string());
    markup.push_str("\">\n");
    for step in &packet.focus_graph.no_mouse_route {
        markup.push_str("    <div role=\"web-focus-step\" data-index=\"");
        markup.push_str(&step.index.to_string());
        markup.push_str("\" data-node-id=\"");
        markup.push_str(&html_escape(&step.node_id));
        markup.push_str("\"></div>\n");
    }
    for node in &packet.accessibility.nodes {
        markup.push_str("    <div role=\"web-accessible-surface\" data-surface-id=\"");
        markup.push_str(&html_escape(&node.surface_id));
        markup.push_str("\" data-role=\"");
        markup.push_str(node.role.label());
        markup.push_str("\">\n");
        markup.push_str("      <span role=\"web-accessible-label\">");
        markup.push_str(&html_escape(&node.accessible_label));
        markup.push_str("</span>\n");
        if let Some(reason) = &node.disabled_reason {
            markup.push_str("      <span role=\"web-accessible-disabled-reason\">");
            markup.push_str(&html_escape(reason));
            markup.push_str("</span>\n");
        }
        markup.push_str("    </div>\n");
    }
    markup.push_str("  </section>\n");
}

fn html_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxide_core::{GuiShellDiagnosticSummary, GuiShellModuleSummary, GuiShellPacket};

    fn baseline_packet() -> GuiShellPacket {
        GuiShellPacket::browser_safe_baseline(
            "examples/thin-slice/ThinSliceHello.basproj",
            "ThinSliceHello",
            vec![GuiShellModuleSummary::new("Module1.bas", true)],
            "Module1.bas",
            "Module1",
            "Attribute VB_Name = \"Module1\"\nOption Explicit\nPublic Sub Main()\nEnd Sub\n",
            vec![GuiShellDiagnosticSummary::new(
                "error",
                "use of undeclared variable: answer",
                "OxVba language service",
            )],
        )
    }

    #[test]
    fn web_shell_boundary_consumes_shell_packet_without_host_claims() {
        let packet = baseline_packet();

        let snapshot = render_web_shell_snapshot(&packet);
        let markup = snapshot.markup();

        assert_eq!(snapshot.source_contract, "GuiShellPacket");
        assert!(!snapshot.dom_smoke_tested);
        assert!(!snapshot.dom_accessibility_audited);
        assert!(!snapshot.filesystem_persistence_claimed);
        assert!(!snapshot.native_runtime_claimed);
        assert!(!snapshot.com_runtime_claimed);
        assert!(!snapshot.parked_tui_imported);
        assert!(markup.contains("role=\"web-shell-adapter\""));
        assert!(markup.contains("data-source=\"GuiShellPacket\""));
        assert!(markup.contains("data-web-framework=\"unselected\""));
        assert!(markup.contains("data-dom-smoke-tested=\"false\""));
        assert!(markup.contains("data-dom-audited=\"false\""));
        assert!(markup.contains("data-filesystem-persistence=\"false\""));
        assert!(markup.contains("data-native-runtime=\"false\""));
        assert!(markup.contains("data-com-runtime=\"false\""));
        assert!(markup.contains("data-parked-tui-imported=\"false\""));
    }

    #[test]
    fn static_shell_dom_smoke_parses_markup_and_verifies_shell_contract() {
        let packet = baseline_packet();

        let report = run_static_shell_dom_smoke(&packet);

        assert!(report.dom_smoke_tested);
        assert_eq!(report.smoke_kind, "parsed-html-tree");
        assert!(!report.browser_runtime_claimed);
        assert!(!report.dom_accessibility_audited);
        assert!(report.all_passed());
        assert_eq!(report.checks.len(), 17);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "root consumes GuiShellPacket")
        );
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "project tree carries project name")
        );
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "source editor shows module source")
        );
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "filesystem persistence remains unclaimed")
        );
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "native runtime remains unclaimed")
        );
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "COM runtime remains unclaimed")
        );
    }

    #[test]
    fn command_palette_dom_smoke_verifies_ids_gestures_and_disabled_reasons() {
        let packet = baseline_packet();

        let report = run_command_palette_dom_smoke(&packet);

        assert!(report.dom_smoke_tested);
        assert_eq!(report.smoke_kind, "parsed-html-command-palette");
        assert!(!report.browser_runtime_claimed);
        assert!(!report.dom_accessibility_audited);
        assert!(report.all_passed());
        assert_eq!(report.checks.len(), 15);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "document.save gesture survives DOM mounting")
        );
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "runtime.run disabled reason remains visible")
        );
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "runtime.immediate gesture survives DOM mounting")
        );
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "runtime.debug disabled reason remains visible")
        );
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "shell.command_palette gesture survives DOM mounting")
        );
    }

    #[test]
    fn web_shell_boundary_renders_major_shell_surfaces() {
        let packet = baseline_packet();

        let snapshot = render_web_shell_snapshot(&packet);
        let markup = snapshot.markup();

        assert!(markup.contains("role=\"web-project-tree\""));
        assert!(markup.contains("ThinSliceHello"));
        assert!(markup.contains("Module1.bas"));
        assert!(markup.contains("role=\"web-source-editor\""));
        assert!(markup.contains("Public Sub Main()"));
        assert!(markup.contains("role=\"web-diagnostics\" data-count=\"1\""));
        assert!(markup.contains("use of undeclared variable: answer"));
        assert!(markup.contains("role=\"web-lifecycle\" data-profile=\"browser-limited\""));
        assert!(markup.contains("browser-safe profile has no direct filesystem persistence"));
        assert!(markup.contains(
            "role=\"web-run-output\" data-provider=\"browser-unsupported\" data-status=\"disabled\""
        ));
        assert!(markup.contains("native execution provider unavailable"));
        assert!(markup.contains("role=\"web-com-capability\" data-profile=\"browser-safe\" data-runtime-available=\"false\""));
        assert!(markup.contains("COM discovery unavailable in browser-safe profile"));
        assert!(markup.contains(
            "role=\"web-command-summary\" data-command-count=\"10\" data-keybinding-count=\"11\""
        ));
        assert!(markup.contains("data-command-id=\"runtime.run\""));
        assert!(markup.contains("role=\"web-focus-accessibility-summary\" data-focus-node-count=\"9\" data-route-length=\"10\" data-accessibility-surface-count=\"10\""));
        assert!(markup.contains("role=\"web-accessible-label\">Source editor"));
        assert!(markup.contains("role=\"web-capability-footer\""));
        assert!(markup.contains("Web shell adapter consumes GuiShellPacket"));
    }
}
