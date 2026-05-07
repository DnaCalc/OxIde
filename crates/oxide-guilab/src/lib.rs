//! Browser-oriented GUI scenario lab boundary for OxIde.
//!
//! W210 starts with deterministic text rendering so the project-open
//! spine can be reviewed before a full browser mount exists.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use oxide_bridge::{DnaOneCalcWebShellHostPacket, EmbeddedIdePacket, WebShellDomReadinessSummary};
use oxide_core::{
    ComCapabilityProfile, ComCapabilityStatus, ComReferenceFact, DebugCapabilityProfile,
    DebugServicePacket, DocumentLifecycleState, DocumentPersistence, GuiAccessibilityProjection,
    GuiBrowserFilesystemDisabledProjection, GuiCommandPalette, GuiFocusGraph, GuiKeyboardMap,
    GuiNativeSaveReloadProjection, GuiSessionSnapshot, GuiShellDiagnosticSummary,
    GuiShellModuleSummary, GuiShellPacket, ImmediateCapabilityProfile, ImmediateServicePacket,
    InMemoryDocumentPersistence, LifecycleCapabilities, LifecycleCommand, LifecycleCommandStatus,
    NativeFilesystemDocumentPersistence, NativeFilesystemSessionPersistence, RunCapabilityProfile,
    RunRequest, RunTimeline, RunTranscript, RuntimeServicePacket, SessionCapabilityProfile,
    open_lifecycle_from_persistence, save_lifecycle_to_persistence,
};
use oxide_domain::{DiagnosticRow, OxideDomainRole};
use oxide_editor_core::{EditOperation, SourceSnapshot};
use oxide_host_bridge::{
    BrowserReviewFixtureHost, HostCapabilityApi, command_availability_for_statuses,
    host_bridge_command_catalog,
};
use oxide_oxvba::{
    EditedDocumentDiagnosticsError, ProjectOpenSpineError, load_edited_document_diagnostics,
    load_project_open_spine,
};
use oxide_ui_leptos::{
    SharedIdeSurfaceModel, UiDataProvenance, render_host_bridge_command_panel,
    render_shared_ide_surface,
};
use oxide_webshell::{
    render_web_shell_snapshot, run_command_palette_dom_smoke, run_no_mouse_accessibility_dom_smoke,
    run_static_shell_dom_smoke,
};

pub const GUI_THIN_SLICE_LOADED: &str = "gui-thin-slice-loaded";
pub const GUI_THIN_SLICE_EDITED_DIAGNOSTICS: &str = "gui-thin-slice-edited-diagnostics";
pub const GUI_THIN_SLICE_LIFECYCLE: &str = "gui-thin-slice-lifecycle";
pub const GUI_RUN_OUTPUT_BROWSER_DISABLED: &str = "gui-run-output-browser-disabled";
pub const GUI_RUN_OUTPUT_SIMULATED_SUPPORTED: &str = "gui-run-output-simulated-supported";
pub const GUI_DNAONECALC_EMBEDDING_CONTRACT: &str = "gui-dnaonecalc-embedding-contract";
pub const GUI_COM_REFERENCE_BROWSER_UNAVAILABLE: &str = "gui-com-reference-browser-unavailable";
pub const GUI_COM_REFERENCE_NONWINDOWS_UNAVAILABLE: &str =
    "gui-com-reference-nonwindows-unavailable";
pub const GUI_COM_REFERENCE_NATIVE_SERVICE_MISSING: &str =
    "gui-com-reference-native-service-missing";
pub const GUI_RUN_TIMELINE_SIMULATED: &str = "gui-run-timeline-simulated";
pub const GUI_IMMEDIATE_BROWSER_DISABLED: &str = "gui-immediate-browser-disabled";
pub const GUI_DEBUG_BROWSER_DISABLED: &str = "gui-debug-browser-disabled";
pub const GUI_COMMAND_PALETTE_BASELINE: &str = "gui-command-palette-baseline";
pub const GUI_KEYBOARD_CONTEXTS_BASELINE: &str = "gui-keyboard-contexts-baseline";
pub const GUI_FOCUS_GRAPH_NO_MOUSE: &str = "gui-focus-graph-no-mouse";
pub const GUI_ACCESSIBILITY_DISABLED_REASONS: &str = "gui-accessibility-disabled-reasons";
pub const GUI_SHELL_PACKET_BASELINE: &str = "gui-shell-packet-baseline";
pub const GUI_MOUNTED_SHELL_STATIC: &str = "gui-mounted-shell-static";
pub const GUI_MOUNTED_COMMAND_PALETTE: &str = "gui-mounted-command-palette";
pub const GUI_MOUNTED_NO_MOUSE_ACCESSIBILITY: &str = "gui-mounted-no-mouse-accessibility";
pub const GUI_WEB_SHELL_ADAPTER_BOUNDARY: &str = "gui-web-shell-adapter-boundary";
pub const GUI_WEB_SHELL_DOM_SMOKE: &str = "gui-web-shell-dom-smoke";
pub const GUI_WEB_COMMAND_PALETTE_DOM_SMOKE: &str = "gui-web-command-palette-dom-smoke";
pub const GUI_WEB_NO_MOUSE_ACCESSIBILITY_DOM_SMOKE: &str =
    "gui-web-no-mouse-accessibility-dom-smoke";
pub const GUI_DNAONECALC_WEB_SHELL_HOST_CONTRACT: &str = "gui-dnaonecalc-web-shell-host-contract";
pub const GUI_DNAONECALC_WEB_SHELL_DOM_READINESS: &str = "gui-dnaonecalc-web-shell-dom-readiness";
pub const GUI_NATIVE_SAVE_RELOAD_DISK: &str = "gui-native-save-reload-disk";
pub const GUI_NATIVE_SESSION_RESTORE_DISK: &str = "gui-native-session-restore-disk";
pub const GUI_BROWSER_FILESYSTEM_STILL_DISABLED: &str = "gui-browser-filesystem-still-disabled";
pub const GUI_RUNTIME_SERVICE_CONTRACT_BROWSER_DISABLED: &str =
    "gui-runtime-service-contract-browser-disabled";
pub const GUI_RUNTIME_SERVICE_CONTRACT_NATIVE_MISSING: &str =
    "gui-runtime-service-contract-native-missing";
pub const GUI_IMMEDIATE_SERVICE_CONTRACT_NATIVE_MISSING: &str =
    "gui-immediate-service-contract-native-missing";
pub const GUI_DEBUG_SERVICE_CONTRACT_NATIVE_MISSING: &str =
    "gui-debug-service-contract-native-missing";
pub const GUI_SHARED_UI_SHELL_COMPONENT: &str = "gui-shared-ui-shell-component";
pub const GUI_HOST_BRIDGE_COMMAND_DISPATCH: &str = "gui-host-bridge-command-dispatch";

/// Compile-time marker for the GUI lab crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OxideGuiLabRole {
    /// Deterministic scenario review surface for the GUI pivot.
    ScenarioReviewSurface,
}

impl OxideGuiLabRole {
    pub fn consumes_domain_vocabulary(self) -> OxideDomainRole {
        match self {
            Self::ScenarioReviewSurface => OxideDomainRole::HostIndependentIdeVocabulary,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuiScenarioDescriptor {
    pub id: &'static str,
    pub title: &'static str,
    pub fixture_path: PathBuf,
    pub kind: GuiScenarioKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuiScenarioKind {
    ReadOnlyProject,
    EditedDiagnostics,
    Lifecycle,
    BrowserRunDisabled,
    SimulatedRunOutput,
    DnaOneCalcEmbeddingContract,
    BrowserComUnavailable,
    NonWindowsComUnavailable,
    NativeComServiceMissing,
    RunTimelineSimulated,
    ImmediateBrowserDisabled,
    DebugBrowserDisabled,
    CommandPaletteBaseline,
    KeyboardContextsBaseline,
    FocusGraphNoMouse,
    AccessibilityDisabledReasons,
    ShellPacketBaseline,
    MountedShellStatic,
    MountedCommandPalette,
    MountedNoMouseAccessibility,
    WebShellAdapterBoundary,
    WebShellDomSmoke,
    WebCommandPaletteDomSmoke,
    WebNoMouseAccessibilityDomSmoke,
    DnaOneCalcWebShellHostContract,
    DnaOneCalcWebShellDomReadiness,
    NativeSaveReloadDisk,
    NativeSessionRestoreDisk,
    BrowserFilesystemStillDisabled,
    RuntimeServiceContractBrowserDisabled,
    RuntimeServiceContractNativeMissing,
    ImmediateServiceContractNativeMissing,
    DebugServiceContractNativeMissing,
    SharedUiShellComponent,
    HostBridgeCommandDispatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuiScenarioRegistry {
    scenarios: Vec<GuiScenarioDescriptor>,
}

impl GuiScenarioRegistry {
    pub fn new(scenarios: Vec<GuiScenarioDescriptor>) -> Result<Self, GuiLabError> {
        let mut seen = HashSet::new();
        for scenario in &scenarios {
            if !seen.insert(scenario.id) {
                return Err(GuiLabError::DuplicateScenarioId {
                    id: scenario.id.to_string(),
                });
            }
        }
        Ok(Self { scenarios })
    }

    pub fn built_in(repo_root: impl AsRef<Path>) -> Self {
        let repo_root = repo_root.as_ref();
        let thin_slice = repo_root
            .join("examples")
            .join("thin-slice")
            .join("ThinSliceHello.basproj");
        Self::new(vec![
            GuiScenarioDescriptor {
                id: GUI_THIN_SLICE_LOADED,
                title: "Thin-slice project loaded",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::ReadOnlyProject,
            },
            GuiScenarioDescriptor {
                id: GUI_THIN_SLICE_EDITED_DIAGNOSTICS,
                title: "Thin-slice edited diagnostics",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::EditedDiagnostics,
            },
            GuiScenarioDescriptor {
                id: GUI_THIN_SLICE_LIFECYCLE,
                title: "Thin-slice lifecycle state",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::Lifecycle,
            },
            GuiScenarioDescriptor {
                id: GUI_RUN_OUTPUT_BROWSER_DISABLED,
                title: "Run output browser disabled",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::BrowserRunDisabled,
            },
            GuiScenarioDescriptor {
                id: GUI_RUN_OUTPUT_SIMULATED_SUPPORTED,
                title: "Run output simulated supported",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::SimulatedRunOutput,
            },
            GuiScenarioDescriptor {
                id: GUI_DNAONECALC_EMBEDDING_CONTRACT,
                title: "DnaOneCalc embedding contract",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::DnaOneCalcEmbeddingContract,
            },
            GuiScenarioDescriptor {
                id: GUI_COM_REFERENCE_BROWSER_UNAVAILABLE,
                title: "COM reference browser unavailable",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::BrowserComUnavailable,
            },
            GuiScenarioDescriptor {
                id: GUI_COM_REFERENCE_NONWINDOWS_UNAVAILABLE,
                title: "COM reference non-Windows unavailable",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::NonWindowsComUnavailable,
            },
            GuiScenarioDescriptor {
                id: GUI_COM_REFERENCE_NATIVE_SERVICE_MISSING,
                title: "COM reference native service missing",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::NativeComServiceMissing,
            },
            GuiScenarioDescriptor {
                id: GUI_RUN_TIMELINE_SIMULATED,
                title: "Run timeline simulated",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::RunTimelineSimulated,
            },
            GuiScenarioDescriptor {
                id: GUI_IMMEDIATE_BROWSER_DISABLED,
                title: "Immediate browser disabled",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::ImmediateBrowserDisabled,
            },
            GuiScenarioDescriptor {
                id: GUI_DEBUG_BROWSER_DISABLED,
                title: "Debug browser disabled",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::DebugBrowserDisabled,
            },
            GuiScenarioDescriptor {
                id: GUI_COMMAND_PALETTE_BASELINE,
                title: "Command palette baseline",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::CommandPaletteBaseline,
            },
            GuiScenarioDescriptor {
                id: GUI_KEYBOARD_CONTEXTS_BASELINE,
                title: "Keyboard contexts baseline",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::KeyboardContextsBaseline,
            },
            GuiScenarioDescriptor {
                id: GUI_FOCUS_GRAPH_NO_MOUSE,
                title: "Focus graph no-mouse",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::FocusGraphNoMouse,
            },
            GuiScenarioDescriptor {
                id: GUI_ACCESSIBILITY_DISABLED_REASONS,
                title: "Accessibility disabled reasons",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::AccessibilityDisabledReasons,
            },
            GuiScenarioDescriptor {
                id: GUI_SHELL_PACKET_BASELINE,
                title: "Shell packet baseline",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::ShellPacketBaseline,
            },
            GuiScenarioDescriptor {
                id: GUI_MOUNTED_SHELL_STATIC,
                title: "Mounted shell static",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::MountedShellStatic,
            },
            GuiScenarioDescriptor {
                id: GUI_MOUNTED_COMMAND_PALETTE,
                title: "Mounted command palette",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::MountedCommandPalette,
            },
            GuiScenarioDescriptor {
                id: GUI_MOUNTED_NO_MOUSE_ACCESSIBILITY,
                title: "Mounted no-mouse accessibility",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::MountedNoMouseAccessibility,
            },
            GuiScenarioDescriptor {
                id: GUI_WEB_SHELL_ADAPTER_BOUNDARY,
                title: "Web shell adapter boundary",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::WebShellAdapterBoundary,
            },
            GuiScenarioDescriptor {
                id: GUI_WEB_SHELL_DOM_SMOKE,
                title: "Web shell DOM smoke",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::WebShellDomSmoke,
            },
            GuiScenarioDescriptor {
                id: GUI_WEB_COMMAND_PALETTE_DOM_SMOKE,
                title: "Web command palette DOM smoke",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::WebCommandPaletteDomSmoke,
            },
            GuiScenarioDescriptor {
                id: GUI_WEB_NO_MOUSE_ACCESSIBILITY_DOM_SMOKE,
                title: "Web no-mouse accessibility DOM smoke",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::WebNoMouseAccessibilityDomSmoke,
            },
            GuiScenarioDescriptor {
                id: GUI_DNAONECALC_WEB_SHELL_HOST_CONTRACT,
                title: "DnaOneCalc web shell host contract",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::DnaOneCalcWebShellHostContract,
            },
            GuiScenarioDescriptor {
                id: GUI_DNAONECALC_WEB_SHELL_DOM_READINESS,
                title: "DnaOneCalc web shell DOM readiness",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::DnaOneCalcWebShellDomReadiness,
            },
            GuiScenarioDescriptor {
                id: GUI_NATIVE_SAVE_RELOAD_DISK,
                title: "Native save reload disk",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::NativeSaveReloadDisk,
            },
            GuiScenarioDescriptor {
                id: GUI_NATIVE_SESSION_RESTORE_DISK,
                title: "Native session restore disk",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::NativeSessionRestoreDisk,
            },
            GuiScenarioDescriptor {
                id: GUI_BROWSER_FILESYSTEM_STILL_DISABLED,
                title: "Browser filesystem still disabled",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::BrowserFilesystemStillDisabled,
            },
            GuiScenarioDescriptor {
                id: GUI_RUNTIME_SERVICE_CONTRACT_BROWSER_DISABLED,
                title: "Runtime service contract browser disabled",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::RuntimeServiceContractBrowserDisabled,
            },
            GuiScenarioDescriptor {
                id: GUI_RUNTIME_SERVICE_CONTRACT_NATIVE_MISSING,
                title: "Runtime service contract native missing",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::RuntimeServiceContractNativeMissing,
            },
            GuiScenarioDescriptor {
                id: GUI_IMMEDIATE_SERVICE_CONTRACT_NATIVE_MISSING,
                title: "Immediate service contract native missing",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::ImmediateServiceContractNativeMissing,
            },
            GuiScenarioDescriptor {
                id: GUI_DEBUG_SERVICE_CONTRACT_NATIVE_MISSING,
                title: "Debug service contract native missing",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::DebugServiceContractNativeMissing,
            },
            GuiScenarioDescriptor {
                id: GUI_SHARED_UI_SHELL_COMPONENT,
                title: "Shared UI shell component",
                fixture_path: thin_slice.clone(),
                kind: GuiScenarioKind::SharedUiShellComponent,
            },
            GuiScenarioDescriptor {
                id: GUI_HOST_BRIDGE_COMMAND_DISPATCH,
                title: "Host bridge command dispatch",
                fixture_path: thin_slice,
                kind: GuiScenarioKind::HostBridgeCommandDispatch,
            },
        ])
        .expect("built-in GUI scenarios have unique IDs")
    }

    pub fn scenarios(&self) -> &[GuiScenarioDescriptor] {
        &self.scenarios
    }

    pub fn find(&self, id: &str) -> Option<&GuiScenarioDescriptor> {
        self.scenarios.iter().find(|scenario| scenario.id == id)
    }

    pub fn render_text(&self, id: &str) -> Result<String, GuiLabError> {
        let scenario = self
            .find(id)
            .ok_or_else(|| GuiLabError::UnknownScenario { id: id.to_string() })?;
        render_project_open_spine(scenario)
    }

    pub fn list_text(&self) -> String {
        let mut output = String::new();
        for scenario in &self.scenarios {
            output.push_str(scenario.id);
            output.push_str("\t");
            output.push_str(scenario.title);
            output.push('\n');
        }
        output
    }
}

pub fn run_cli(args: Vec<String>, repo_root: impl AsRef<Path>) -> Result<String, GuiLabError> {
    let registry = GuiScenarioRegistry::built_in(repo_root);
    match args.as_slice() {
        [] => Err(GuiLabError::Usage {
            message: String::from("usage: oxide-guilab list | render <scenario-id>"),
        }),
        [command] if command == "list" => Ok(registry.list_text()),
        [command, scenario_id] if command == "render" => registry.render_text(scenario_id),
        _ => Err(GuiLabError::Usage {
            message: String::from("usage: oxide-guilab list | render <scenario-id>"),
        }),
    }
}

fn render_project_open_spine(scenario: &GuiScenarioDescriptor) -> Result<String, GuiLabError> {
    let view = load_project_open_spine(&scenario.fixture_path).map_err(GuiLabError::ProjectOpen)?;
    let mut output = String::new();
    output.push_str("<section data-scenario=\"");
    output.push_str(scenario.id);
    output.push_str("\">\n");
    output.push_str("  <h1>");
    output.push_str(scenario.title);
    output.push_str("</h1>\n");
    output.push_str("  <div role=\"project-name\">");
    output.push_str(&view.project_name);
    output.push_str("</div>\n");
    output.push_str("  <nav role=\"project-spine\">\n");
    for module in &view.modules {
        output.push_str("    <div role=\"module-row\" data-active=\"");
        output.push_str(if module.is_active { "true" } else { "false" });
        output.push_str("\">");
        output.push_str(&module.display_name);
        output.push_str("</div>\n");
    }
    output.push_str("  </nav>\n");
    let source_text = match scenario.kind {
        GuiScenarioKind::ReadOnlyProject
        | GuiScenarioKind::BrowserRunDisabled
        | GuiScenarioKind::SimulatedRunOutput
        | GuiScenarioKind::DnaOneCalcEmbeddingContract
        | GuiScenarioKind::BrowserComUnavailable
        | GuiScenarioKind::NonWindowsComUnavailable
        | GuiScenarioKind::NativeComServiceMissing
        | GuiScenarioKind::RunTimelineSimulated
        | GuiScenarioKind::ImmediateBrowserDisabled
        | GuiScenarioKind::DebugBrowserDisabled
        | GuiScenarioKind::CommandPaletteBaseline
        | GuiScenarioKind::KeyboardContextsBaseline
        | GuiScenarioKind::FocusGraphNoMouse
        | GuiScenarioKind::AccessibilityDisabledReasons
        | GuiScenarioKind::ShellPacketBaseline
        | GuiScenarioKind::MountedShellStatic
        | GuiScenarioKind::MountedCommandPalette
        | GuiScenarioKind::MountedNoMouseAccessibility
        | GuiScenarioKind::WebShellAdapterBoundary
        | GuiScenarioKind::WebShellDomSmoke
        | GuiScenarioKind::WebCommandPaletteDomSmoke
        | GuiScenarioKind::WebNoMouseAccessibilityDomSmoke
        | GuiScenarioKind::DnaOneCalcWebShellHostContract
        | GuiScenarioKind::DnaOneCalcWebShellDomReadiness
        | GuiScenarioKind::NativeSaveReloadDisk
        | GuiScenarioKind::NativeSessionRestoreDisk
        | GuiScenarioKind::BrowserFilesystemStillDisabled
        | GuiScenarioKind::RuntimeServiceContractBrowserDisabled
        | GuiScenarioKind::RuntimeServiceContractNativeMissing
        | GuiScenarioKind::ImmediateServiceContractNativeMissing
        | GuiScenarioKind::DebugServiceContractNativeMissing
        | GuiScenarioKind::SharedUiShellComponent
        | GuiScenarioKind::HostBridgeCommandDispatch => view.active_source.source_text.clone(),
        GuiScenarioKind::EditedDiagnostics | GuiScenarioKind::Lifecycle => {
            edited_diagnostics_source(&view.active_source.source_text)
        }
    };

    output.push_str("  <pre role=\"source\" data-module=\"");
    output.push_str(&view.active_source.module_display_name);
    output.push_str("\">");
    output.push_str(&html_escape(&source_text));
    output.push_str("</pre>\n");
    match scenario.kind {
        GuiScenarioKind::ReadOnlyProject => {}
        GuiScenarioKind::EditedDiagnostics => {
            let diagnostics = load_edited_document_diagnostics(
                &scenario.fixture_path,
                &view.active_source.module_display_name,
                &source_text,
            )
            .map_err(GuiLabError::Diagnostics)?;
            output.push_str("  <section role=\"diagnostics\">\n");
            for diagnostic in &diagnostics.diagnostics {
                render_diagnostic_row(&mut output, diagnostic);
            }
            output.push_str("  </section>\n");
        }
        GuiScenarioKind::Lifecycle => render_lifecycle_section(
            &mut output,
            &scenario.fixture_path,
            &view.active_source.module_display_name,
            &view.active_source.source_text,
            &source_text,
        ),
        GuiScenarioKind::BrowserRunDisabled => render_browser_run_disabled_section(
            &mut output,
            &view.project_name,
            &view.active_source.module_display_name,
        ),
        GuiScenarioKind::SimulatedRunOutput => render_simulated_run_output_section(
            &mut output,
            &view.project_name,
            &view.active_source.module_display_name,
        ),
        GuiScenarioKind::DnaOneCalcEmbeddingContract => {
            render_dnaonecalc_embedding_contract_section(
                &mut output,
                &scenario.fixture_path,
                &view.project_name,
                &view.active_source.module_display_name,
                &view.active_source.source_text,
            )
        }
        GuiScenarioKind::BrowserComUnavailable => render_com_capability_section(
            &mut output,
            ComCapabilityProfile::browser_unavailable(ComReferenceFact::scripting_dictionary_demo()),
        ),
        GuiScenarioKind::NonWindowsComUnavailable => render_com_capability_section(
            &mut output,
            ComCapabilityProfile::non_windows_native_unavailable(
                ComReferenceFact::scripting_dictionary_demo(),
            ),
        ),
        GuiScenarioKind::NativeComServiceMissing => render_com_capability_section(
            &mut output,
            ComCapabilityProfile::windows_native_service_missing(
                ComReferenceFact::scripting_dictionary_demo(),
            ),
        ),
        GuiScenarioKind::RunTimelineSimulated => render_run_timeline_section(
            &mut output,
            &view.project_name,
            &view.active_source.module_display_name,
        ),
        GuiScenarioKind::ImmediateBrowserDisabled => {
            render_immediate_browser_disabled_section(&mut output)
        }
        GuiScenarioKind::DebugBrowserDisabled => render_debug_browser_disabled_section(&mut output),
        GuiScenarioKind::CommandPaletteBaseline => {
            render_command_palette_baseline_section(&mut output, &view.active_source.source_text)
        }
        GuiScenarioKind::KeyboardContextsBaseline => {
            render_keyboard_contexts_baseline_section(&mut output, &view.active_source.source_text)
        }
        GuiScenarioKind::FocusGraphNoMouse => {
            render_focus_graph_no_mouse_section(&mut output, &view.active_source.source_text)
        }
        GuiScenarioKind::AccessibilityDisabledReasons => {
            render_accessibility_disabled_reasons_section(
                &mut output,
                &view.active_source.source_text,
            )
        }
        GuiScenarioKind::ShellPacketBaseline => render_shell_packet_baseline_section(
            &mut output,
            &scenario.fixture_path,
            &view.project_name,
            &view
                .modules
                .iter()
                .map(|module| GuiShellModuleSummary::new(&module.display_name, module.is_active))
                .collect::<Vec<_>>(),
            &view.active_source.module_display_name,
            &view.active_source.source_text,
        ),
        GuiScenarioKind::MountedShellStatic => render_mounted_shell_static_section(
            &mut output,
            &scenario.fixture_path,
            &view.project_name,
            &view
                .modules
                .iter()
                .map(|module| GuiShellModuleSummary::new(&module.display_name, module.is_active))
                .collect::<Vec<_>>(),
            &view.active_source.module_display_name,
            &view.active_source.source_text,
        ),
        GuiScenarioKind::MountedCommandPalette => render_mounted_command_palette_section(
            &mut output,
            &scenario.fixture_path,
            &view.project_name,
            &view
                .modules
                .iter()
                .map(|module| GuiShellModuleSummary::new(&module.display_name, module.is_active))
                .collect::<Vec<_>>(),
            &view.active_source.module_display_name,
            &view.active_source.source_text,
        ),
        GuiScenarioKind::MountedNoMouseAccessibility => {
            render_mounted_no_mouse_accessibility_section(
                &mut output,
                &scenario.fixture_path,
                &view.project_name,
                &view
                    .modules
                    .iter()
                    .map(|module| {
                        GuiShellModuleSummary::new(&module.display_name, module.is_active)
                    })
                    .collect::<Vec<_>>(),
                &view.active_source.module_display_name,
                &view.active_source.source_text,
            )
        }
        GuiScenarioKind::WebShellAdapterBoundary => render_web_shell_adapter_boundary_section(
            &mut output,
            &scenario.fixture_path,
            &view.project_name,
            &view
                .modules
                .iter()
                .map(|module| GuiShellModuleSummary::new(&module.display_name, module.is_active))
                .collect::<Vec<_>>(),
            &view.active_source.module_display_name,
            &view.active_source.source_text,
        ),
        GuiScenarioKind::WebShellDomSmoke => render_web_shell_dom_smoke_section(
            &mut output,
            &scenario.fixture_path,
            &view.project_name,
            &view
                .modules
                .iter()
                .map(|module| GuiShellModuleSummary::new(&module.display_name, module.is_active))
                .collect::<Vec<_>>(),
            &view.active_source.module_display_name,
            &view.active_source.source_text,
        ),
        GuiScenarioKind::WebCommandPaletteDomSmoke => render_web_command_palette_dom_smoke_section(
            &mut output,
            &scenario.fixture_path,
            &view.project_name,
            &view
                .modules
                .iter()
                .map(|module| GuiShellModuleSummary::new(&module.display_name, module.is_active))
                .collect::<Vec<_>>(),
            &view.active_source.module_display_name,
            &view.active_source.source_text,
        ),
        GuiScenarioKind::WebNoMouseAccessibilityDomSmoke => {
            render_web_no_mouse_accessibility_dom_smoke_section(
                &mut output,
                &scenario.fixture_path,
                &view.project_name,
                &view
                    .modules
                    .iter()
                    .map(|module| {
                        GuiShellModuleSummary::new(&module.display_name, module.is_active)
                    })
                    .collect::<Vec<_>>(),
                &view.active_source.module_display_name,
                &view.active_source.source_text,
            )
        }
        GuiScenarioKind::DnaOneCalcWebShellHostContract => {
            render_dnaonecalc_web_shell_host_contract_section(
                &mut output,
                &scenario.fixture_path,
                &view.project_name,
                &view
                    .modules
                    .iter()
                    .map(|module| {
                        GuiShellModuleSummary::new(&module.display_name, module.is_active)
                    })
                    .collect::<Vec<_>>(),
                &view.active_source.module_display_name,
                &view.active_source.source_text,
            )
        }
        GuiScenarioKind::DnaOneCalcWebShellDomReadiness => {
            render_dnaonecalc_web_shell_dom_readiness_section(
                &mut output,
                &scenario.fixture_path,
                &view.project_name,
                &view
                    .modules
                    .iter()
                    .map(|module| {
                        GuiShellModuleSummary::new(&module.display_name, module.is_active)
                    })
                    .collect::<Vec<_>>(),
                &view.active_source.module_display_name,
                &view.active_source.source_text,
            )
        }
        GuiScenarioKind::NativeSaveReloadDisk => render_native_save_reload_disk_section(
            &mut output,
            &scenario.fixture_path,
            &view.active_source.module_display_name,
            &view.active_source.source_text,
        ),
        GuiScenarioKind::NativeSessionRestoreDisk => render_native_session_restore_disk_section(
            &mut output,
            &scenario.fixture_path,
            &view.active_source.module_display_name,
            &view.active_source.source_text,
        ),
        GuiScenarioKind::BrowserFilesystemStillDisabled => {
            render_browser_filesystem_still_disabled_section(
                &mut output,
                &view.active_source.source_text,
            )
        }
        GuiScenarioKind::RuntimeServiceContractBrowserDisabled => {
            render_runtime_service_contract_section(
                &mut output,
                RuntimeServicePacket::browser_disabled(
                    scenario.fixture_path.display().to_string(),
                    &view.project_name,
                    module_stem(&view.active_source.module_display_name),
                    "Main",
                ),
            )
        }
        GuiScenarioKind::RuntimeServiceContractNativeMissing => {
            render_runtime_service_contract_section(
                &mut output,
                RuntimeServicePacket::native_service_missing(
                    scenario.fixture_path.display().to_string(),
                    &view.project_name,
                    module_stem(&view.active_source.module_display_name),
                    "Main",
                ),
            )
        }
        GuiScenarioKind::ImmediateServiceContractNativeMissing => {
            render_immediate_service_contract_section(
                &mut output,
                ImmediateServicePacket::native_service_missing(Some(String::from("?answer"))),
            )
        }
        GuiScenarioKind::DebugServiceContractNativeMissing => {
            render_debug_service_contract_section(
                &mut output,
                DebugServicePacket::native_service_missing(),
            )
        }
        GuiScenarioKind::SharedUiShellComponent => render_shared_ui_shell_component_section(
            &mut output,
            &scenario.fixture_path,
            &view.project_name,
            &view
                .modules
                .iter()
                .map(|module| GuiShellModuleSummary::new(&module.display_name, module.is_active))
                .collect::<Vec<_>>(),
            &view.active_source.module_display_name,
            &view.active_source.source_text,
        ),
        GuiScenarioKind::HostBridgeCommandDispatch => render_host_bridge_command_dispatch_section(
            &mut output,
            &scenario.fixture_path,
            &view.project_name,
            &view
                .modules
                .iter()
                .map(|module| GuiShellModuleSummary::new(&module.display_name, module.is_active))
                .collect::<Vec<_>>(),
            &view.active_source.module_display_name,
            &view.active_source.source_text,
        ),
    }
    output.push_str("  <footer role=\"host-capability\">");
    output.push_str(scenario_host_capability_text(
        scenario.kind,
        &view.capability.status_text,
    ));
    output.push_str("</footer>\n");
    output.push_str("</section>\n");
    Ok(output)
}

fn scenario_host_capability_text<'a>(kind: GuiScenarioKind, default_text: &'a str) -> &'a str {
    match kind {
        GuiScenarioKind::NonWindowsComUnavailable => {
            "Non-Windows native profile: native execution capability is separate from Windows COM; COM unavailable."
        }
        GuiScenarioKind::NativeComServiceMissing => {
            "Windows native profile: native host admitted, but native COM service is not configured; COM runtime disabled."
        }
        GuiScenarioKind::NativeSaveReloadDisk | GuiScenarioKind::NativeSessionRestoreDisk => {
            "Native filesystem profile: disk-backed persistence proven in a test-owned temp project; runtime and COM remain unavailable."
        }
        GuiScenarioKind::BrowserFilesystemStillDisabled => {
            "Browser-safe profile: direct filesystem persistence remains unavailable; native runtime and COM unavailable."
        }
        GuiScenarioKind::RuntimeServiceContractBrowserDisabled
        | GuiScenarioKind::RuntimeServiceContractNativeMissing
        | GuiScenarioKind::ImmediateServiceContractNativeMissing
        | GuiScenarioKind::DebugServiceContractNativeMissing
        | GuiScenarioKind::SharedUiShellComponent
        | GuiScenarioKind::HostBridgeCommandDispatch => {
            "Shared UI component profile: no real execution, fake data, native runtime, or COM runtime is claimed."
        }
        _ => default_text,
    }
}

fn render_browser_run_disabled_section(
    output: &mut String,
    project_name: &str,
    module_display_name: &str,
) {
    let request = RunRequest::new(project_name, module_stem(module_display_name), "Main");
    let profile = RunCapabilityProfile::browser_safe_unsupported();
    let command_status = profile.command_status();
    let transcript = RunTranscript::browser_disabled(request, profile.clone());

    output.push_str("  <section role=\"run-output\" data-provider=\"");
    output.push_str(transcript.provider_label.as_str());
    output.push_str("\" data-status=\"");
    output.push_str(transcript.status.label());
    output.push_str("\">\n");
    output.push_str("    <div role=\"run-request\" data-target=\"");
    output.push_str(&html_escape(&transcript.request.display_target()));
    output.push_str("\">");
    output.push_str(&html_escape(&transcript.request.entrypoint));
    output.push_str("</div>\n");
    output.push_str("    <div role=\"run-command\" data-enabled=\"");
    output.push_str(if command_status.is_enabled {
        "true"
    } else {
        "false"
    });
    output.push_str("\">Run");
    if let Some(reason) = command_status.reason {
        output.push_str(" disabled: ");
        output.push_str(&html_escape(&reason));
    }
    output.push_str("</div>\n");
    output.push_str("    <section role=\"output-activity\">\n");
    for event in &transcript.events {
        output.push_str("      <div role=\"output-event\" data-event-kind=\"");
        output.push_str(event.kind.label());
        output.push_str("\">");
        output.push_str(&html_escape(&event.message));
        output.push_str("</div>\n");
    }
    output.push_str("    </section>\n");
    output.push_str("  </section>\n");
}

fn render_simulated_run_output_section(
    output: &mut String,
    project_name: &str,
    module_display_name: &str,
) {
    let request = RunRequest::new(project_name, module_stem(module_display_name), "Main");
    let profile = RunCapabilityProfile::simulated_supported();
    let command_status = profile.command_status();
    let transcript = RunTranscript::simulated_completed(request);

    output.push_str("  <section role=\"run-output\" data-provider=\"");
    output.push_str(transcript.provider_label.as_str());
    output.push_str("\" data-status=\"");
    output.push_str(transcript.status.label());
    output.push_str("\" data-native-execution=\"");
    output.push_str(if profile.native_execution_available {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-com-runtime=\"");
    output.push_str(if profile.com_runtime_available {
        "true"
    } else {
        "false"
    });
    output.push_str("\">\n");
    output.push_str("    <div role=\"run-request\" data-target=\"");
    output.push_str(&html_escape(&transcript.request.display_target()));
    output.push_str("\">");
    output.push_str(&html_escape(&transcript.request.entrypoint));
    output.push_str("</div>\n");
    output.push_str("    <div role=\"run-command\" data-enabled=\"");
    output.push_str(if command_status.is_enabled {
        "true"
    } else {
        "false"
    });
    output.push_str("\">Run enabled by simulated provider</div>\n");
    output.push_str("    <section role=\"output-activity\">\n");
    for event in &transcript.events {
        output.push_str("      <div role=\"output-event\" data-event-kind=\"");
        output.push_str(event.kind.label());
        output.push_str("\">");
        output.push_str(&html_escape(&event.message));
        output.push_str("</div>\n");
    }
    output.push_str("    </section>\n");
    output.push_str("  </section>\n");
}

fn render_run_timeline_section(output: &mut String, project_name: &str, module_display_name: &str) {
    let request = RunRequest::new(project_name, module_stem(module_display_name), "Main");
    let profile = RunCapabilityProfile::simulated_supported();
    let transcript = RunTranscript::simulated_completed(request);
    let timeline = RunTimeline::from_transcript(&transcript, &profile);

    output.push_str("  <section role=\"run-timeline\" data-provider=\"");
    output.push_str(&html_escape(&timeline.provider_label));
    output.push_str("\" data-status=\"");
    output.push_str(timeline.status_label());
    output.push_str("\" data-native-execution=\"");
    output.push_str(if timeline.native_execution_available {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-com-runtime=\"");
    output.push_str(if timeline.com_runtime_available {
        "true"
    } else {
        "false"
    });
    output.push_str("\">\n");
    output.push_str("    <div role=\"run-timeline-target\">");
    output.push_str(&html_escape(&timeline.request.display_target()));
    output.push_str("</div>\n");
    for entry in &timeline.entries {
        output.push_str("    <div role=\"run-timeline-entry\" data-index=\"");
        output.push_str(&entry.index.to_string());
        output.push_str("\" data-event-kind=\"");
        output.push_str(entry.kind.label());
        output.push_str("\" data-provenance=\"");
        output.push_str(&html_escape(&entry.provenance_label));
        output.push_str("\">");
        output.push_str(&html_escape(&entry.message));
        output.push_str("</div>\n");
    }
    output.push_str("  </section>\n");
}

fn render_immediate_browser_disabled_section(output: &mut String) {
    let profile = ImmediateCapabilityProfile::browser_disabled();
    output.push_str("  <section role=\"immediate-panel\" data-profile=\"");
    output.push_str(profile.kind.label());
    output.push_str("\" data-enabled=\"");
    output.push_str(if profile.command_status.is_enabled {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-native-runtime-required=\"");
    output.push_str(if profile.native_runtime_required {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-com-runtime-required=\"");
    output.push_str(if profile.com_runtime_required {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-fake-responses=\"");
    output.push_str(if profile.fake_responses_available {
        "true"
    } else {
        "false"
    });
    output.push_str("\">\n");
    if let Some(reason) = &profile.command_status.reason {
        output.push_str("    <div role=\"immediate-command\" data-enabled=\"false\">");
        output.push_str(&html_escape(reason));
        output.push_str("</div>\n");
    }
    output.push_str("    <div role=\"immediate-empty-state\">No Immediate responses rendered without runtime session.</div>\n");
    output.push_str("  </section>\n");
}

fn render_debug_browser_disabled_section(output: &mut String) {
    let profile = DebugCapabilityProfile::browser_disabled();
    output.push_str("  <section role=\"debug-panel\" data-profile=\"");
    output.push_str(profile.kind.label());
    output.push_str("\" data-enabled=\"");
    output.push_str(if profile.command_status.is_enabled {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-native-runtime-required=\"");
    output.push_str(if profile.native_runtime_required {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-com-runtime-required=\"");
    output.push_str(if profile.com_runtime_required {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-fake-debug-data=\"");
    output.push_str(if profile.fake_debug_data_available {
        "true"
    } else {
        "false"
    });
    output.push_str("\">\n");
    if let Some(reason) = &profile.command_status.reason {
        output.push_str("    <div role=\"debug-command\" data-enabled=\"false\">");
        output.push_str(&html_escape(reason));
        output.push_str("</div>\n");
    }
    output.push_str("    <div role=\"debug-callstack\">unavailable; no fake debug data</div>\n");
    output.push_str("    <div role=\"debug-locals\">unavailable; no fake debug data</div>\n");
    output.push_str("    <div role=\"debug-watches\">unavailable; no fake debug data</div>\n");
    output.push_str("    <div role=\"debug-breakpoints\">unavailable; no fake debug data</div>\n");
    output.push_str("  </section>\n");
}

fn render_command_palette_baseline_section(output: &mut String, source_text: &str) {
    let document =
        DocumentLifecycleState::open_clean(source_text, LifecycleCapabilities::browser_limited());
    let palette = GuiCommandPalette::browser_safe_baseline(&document);

    output.push_str("  <section role=\"command-palette\" data-source=\"");
    output.push_str(&html_escape(&palette.source_label));
    output.push_str("\" data-parked-tui-imported=\"");
    output.push_str(if palette.parked_tui_imported {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-command-count=\"");
    output.push_str(&palette.commands.len().to_string());
    output.push_str("\">\n");
    for command in &palette.commands {
        output.push_str("    <div role=\"command-row\" data-command-id=\"");
        output.push_str(&html_escape(&command.stable_id));
        output.push_str("\" data-category=\"");
        output.push_str(command.category.label());
        output.push_str("\" data-enabled=\"");
        output.push_str(if command.availability.is_enabled {
            "true"
        } else {
            "false"
        });
        output.push_str("\" data-capability=\"");
        output.push_str(&html_escape(&command.availability.capability_label));
        output.push_str("\">\n");
        output.push_str("      <span role=\"command-label\">");
        output.push_str(&html_escape(&command.label));
        output.push_str("</span>\n");
        output.push_str("      <span role=\"command-description\">");
        output.push_str(&html_escape(&command.description));
        output.push_str("</span>\n");
        if let Some(reason) = &command.availability.disabled_reason {
            output.push_str("      <span role=\"command-disabled-reason\">");
            output.push_str(&html_escape(reason));
            output.push_str("</span>\n");
        }
        output.push_str("    </div>\n");
    }
    output.push_str("    <div role=\"command-registry-note\">GUI-native command registry; parked TUI command model not imported.</div>\n");
    output.push_str("  </section>\n");
}

fn render_keyboard_contexts_baseline_section(output: &mut String, source_text: &str) {
    let document =
        DocumentLifecycleState::open_clean(source_text, LifecycleCapabilities::browser_limited());
    let palette = GuiCommandPalette::browser_safe_baseline(&document);
    let keyboard = GuiKeyboardMap::baseline(&palette);

    output.push_str("  <section role=\"keyboard-contexts\" data-source=\"");
    output.push_str(&html_escape(&keyboard.source_label));
    output.push_str("\" data-host-specific-overrides-required=\"");
    output.push_str(if keyboard.host_specific_overrides_required {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-context-collisions=\"");
    output.push_str(&keyboard.context_collisions().len().to_string());
    output.push_str("\" data-cross-context-collisions=\"");
    output.push_str(
        &keyboard
            .disallowed_cross_context_collisions()
            .len()
            .to_string(),
    );
    output.push_str("\">\n");
    output.push_str("    <section role=\"keyboard-context-list\">\n");
    for context in &keyboard.contexts {
        output.push_str("      <div role=\"keyboard-context\" data-context=\"");
        output.push_str(context.context.label());
        output.push_str("\">");
        output.push_str(&html_escape(&context.label));
        output.push_str("</div>\n");
    }
    output.push_str("    </section>\n");
    output.push_str("    <section role=\"keybinding-list\">\n");
    for binding in &keyboard.bindings {
        output.push_str("      <div role=\"keybinding\" data-context=\"");
        output.push_str(binding.context.label());
        output.push_str("\" data-command-id=\"");
        output.push_str(&html_escape(&binding.command_stable_id));
        output.push_str("\" data-gesture=\"");
        output.push_str(&html_escape(&binding.gesture.display));
        output.push_str("\" data-enabled=\"");
        output.push_str(if binding.availability.is_enabled {
            "true"
        } else {
            "false"
        });
        output.push_str("\" data-allow-cross-context=\"");
        output.push_str(if binding.allow_same_gesture_in_distinct_contexts {
            "true"
        } else {
            "false"
        });
        output.push_str("\">\n");
        if let Some(reason) = &binding.availability.disabled_reason {
            output.push_str("        <span role=\"keybinding-disabled-reason\">");
            output.push_str(&html_escape(reason));
            output.push_str("</span>\n");
        }
        output.push_str("      </div>\n");
    }
    output.push_str("    </section>\n");
    output.push_str("    <div role=\"keyboard-policy-note\">Host-independent GUI keyboard map; no browser-specific key trap is product truth.</div>\n");
    output.push_str("  </section>\n");
}

fn render_focus_graph_no_mouse_section(output: &mut String, source_text: &str) {
    let document =
        DocumentLifecycleState::open_clean(source_text, LifecycleCapabilities::browser_limited());
    let palette = GuiCommandPalette::browser_safe_baseline(&document);
    let focus = GuiFocusGraph::baseline(&palette);

    output.push_str("  <section role=\"focus-graph\" data-source=\"");
    output.push_str(&html_escape(&focus.source_label));
    output.push_str("\" data-node-count=\"");
    output.push_str(&focus.nodes.len().to_string());
    output.push_str("\" data-route-length=\"");
    output.push_str(&focus.no_mouse_route.len().to_string());
    output.push_str("\">\n");
    output.push_str("    <section role=\"focus-node-list\">\n");
    for node in &focus.nodes {
        output.push_str("      <div role=\"focus-node\" data-node-id=\"");
        output.push_str(&html_escape(&node.node_id));
        output.push_str("\" data-kind=\"");
        output.push_str(node.kind.label());
        output.push_str("\" data-focusable=\"");
        output.push_str(if node.focusable { "true" } else { "false" });
        output.push_str("\" data-disabled-reason-visible=\"");
        output.push_str(if node.disabled_reason_visible {
            "true"
        } else {
            "false"
        });
        output.push_str("\">\n");
        output.push_str("        <span role=\"focus-label\">");
        output.push_str(&html_escape(&node.label));
        output.push_str("</span>\n");
        if let Some(restore_target) = &node.restore_target {
            output.push_str("        <span role=\"focus-restore-target\">");
            output.push_str(&html_escape(restore_target));
            output.push_str("</span>\n");
        }
        output.push_str("      </div>\n");
    }
    output.push_str("    </section>\n");
    output.push_str("    <section role=\"no-mouse-route\">\n");
    for step in &focus.no_mouse_route {
        output.push_str("      <div role=\"focus-route-step\" data-index=\"");
        output.push_str(&step.index.to_string());
        output.push_str("\" data-node-id=\"");
        output.push_str(&html_escape(&step.node_id));
        output.push_str("\">\n");
        if let Some(restoration_hint) = &step.restoration_hint {
            output.push_str("        <span role=\"focus-restoration-hint\">");
            output.push_str(&html_escape(restoration_hint));
            output.push_str("</span>\n");
        }
        output.push_str("      </div>\n");
    }
    output.push_str("    </section>\n");
    output.push_str("    <div role=\"focus-policy-note\">Disabled reason panels remain reachable in the no-mouse route.</div>\n");
    output.push_str("  </section>\n");
}

fn render_accessibility_disabled_reasons_section(output: &mut String, source_text: &str) {
    let document =
        DocumentLifecycleState::open_clean(source_text, LifecycleCapabilities::browser_limited());
    let palette = GuiCommandPalette::browser_safe_baseline(&document);
    let accessibility = GuiAccessibilityProjection::baseline(&palette);

    output.push_str("  <section role=\"accessibility-projection\" data-source=\"");
    output.push_str(&html_escape(&accessibility.source_label));
    output.push_str("\" data-web-framework-bound=\"");
    output.push_str(if accessibility.web_framework_bound {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-surface-count=\"");
    output.push_str(&accessibility.nodes.len().to_string());
    output.push_str("\">\n");
    for node in &accessibility.nodes {
        output.push_str("    <div role=\"accessible-surface\" data-surface-id=\"");
        output.push_str(&html_escape(&node.surface_id));
        output.push_str("\" data-role=\"");
        output.push_str(node.role.label());
        output.push_str("\" data-has-disabled-reason=\"");
        output.push_str(if node.disabled_reason.is_some() {
            "true"
        } else {
            "false"
        });
        output.push_str("\">\n");
        output.push_str("      <span role=\"accessible-label\">");
        output.push_str(&html_escape(&node.accessible_label));
        output.push_str("</span>\n");
        output.push_str("      <span role=\"accessible-description\">");
        output.push_str(&html_escape(&node.accessible_description));
        output.push_str("</span>\n");
        if let Some(reason) = &node.disabled_reason {
            output.push_str("      <span role=\"accessible-disabled-reason\">");
            output.push_str(&html_escape(reason));
            output.push_str("</span>\n");
        }
        output.push_str("    </div>\n");
    }
    output.push_str("    <div role=\"accessibility-policy-note\">Projection data only; no web framework accessibility API is chosen in core.</div>\n");
    output.push_str("  </section>\n");
}

fn build_shell_packet_baseline(
    workspace_path: &Path,
    project_name: &str,
    modules: &[GuiShellModuleSummary],
    active_module: &str,
    source_text: &str,
) -> GuiShellPacket {
    GuiShellPacket::browser_safe_baseline(
        workspace_path.display().to_string(),
        project_name,
        modules.to_vec(),
        active_module,
        module_stem(active_module),
        source_text,
        vec![GuiShellDiagnosticSummary::new(
            "info",
            "shell packet baseline keeps diagnostics surface available",
            "OxIde GUI shell packet",
        )],
    )
}

fn build_dnaonecalc_web_shell_host_packet(
    workspace_path: &Path,
    project_name: &str,
    modules: &[GuiShellModuleSummary],
    active_module: &str,
    source_text: &str,
) -> DnaOneCalcWebShellHostPacket {
    let embedded = EmbeddedIdePacket::dnaonecalc_thin_slice_browser_disabled(
        workspace_path.display().to_string(),
        project_name,
        active_module,
        source_text,
    );
    let shell_packet = build_shell_packet_baseline(
        workspace_path,
        project_name,
        modules,
        active_module,
        source_text,
    );
    DnaOneCalcWebShellHostPacket::from_packets(
        embedded,
        &shell_packet,
        WebShellDomReadinessSummary::parsed_html_all_passed(),
    )
}

fn render_shared_ui_shell_component_section(
    output: &mut String,
    workspace_path: &Path,
    project_name: &str,
    modules: &[GuiShellModuleSummary],
    active_module: &str,
    source_text: &str,
) {
    let shell = build_shell_packet_baseline(
        workspace_path,
        project_name,
        modules,
        active_module,
        source_text,
    );
    let runtime = RuntimeServicePacket::native_service_missing(
        workspace_path.display().to_string(),
        project_name,
        module_stem(active_module),
        "Main",
    );
    let immediate = ImmediateServicePacket::native_service_missing(Some(String::from("?answer")));
    let debug = DebugServicePacket::native_service_missing();
    let model = SharedIdeSurfaceModel {
        shell,
        runtime,
        immediate,
        debug,
        provenance: UiDataProvenance::PendingOxVbaHardening {
            gap: "W342 shared UI consumes packets; full OxVba stable IDs/events/watch/breakpoint/COM runtime evidence pending",
        },
    };
    let render = render_shared_ide_surface(&model);

    output.push_str("  <section role=\"shared-ui-component-route\" data-source=\"oxide-ui-leptos\" data-component-crate=\"");
    output.push_str(render.component_crate);
    output.push_str("\" data-native-runtime=\"");
    output.push_str(if render.native_runtime_claimed {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-com-runtime=\"");
    output.push_str(if render.com_runtime_claimed {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-fake-responses=\"");
    output.push_str(if render.fake_immediate_responses {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-fake-debug-data=\"");
    output.push_str(if render.fake_debug_data {
        "true"
    } else {
        "false"
    });
    output.push_str("\">\n");
    output.push_str(render.markup());
    output.push_str("    <div role=\"shared-ui-component-policy\">Shared UI route renders oxide-ui-leptos component output from packets; no live Tauri/WebView, real runtime, fake Immediate response, fake debug data, or COM runtime is claimed.</div>\n");
    output.push_str("  </section>\n");
}

fn render_host_bridge_command_dispatch_section(
    output: &mut String,
    workspace_path: &Path,
    project_name: &str,
    modules: &[GuiShellModuleSummary],
    active_module: &str,
    source_text: &str,
) {
    let shell = build_shell_packet_baseline(
        workspace_path,
        project_name,
        modules,
        active_module,
        source_text,
    );
    let host = BrowserReviewFixtureHost::new(shell);
    let availability = command_availability_for_statuses(
        &host_bridge_command_catalog(),
        &host.capability_statuses(),
    );
    let render = render_host_bridge_command_panel(&availability);

    output.push_str("  <section role=\"host-bridge-command-dispatch-route\" data-source=\"oxide-host-bridge+oxide-ui-leptos\" data-component-crate=\"");
    output.push_str(render.component_crate);
    output.push_str("\" data-command-count=\"");
    output.push_str(&render.command_count.to_string());
    output.push_str("\" data-enabled-count=\"");
    output.push_str(&render.enabled_count.to_string());
    output.push_str("\" data-native-runtime=\"");
    output.push_str(if render.native_runtime_claimed {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-com-runtime=\"");
    output.push_str(if render.com_runtime_claimed {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-fake-responses=\"");
    output.push_str(if render.fake_immediate_responses {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-fake-debug-data=\"");
    output.push_str(if render.fake_debug_data {
        "true"
    } else {
        "false"
    });
    output.push_str("\">\n");
    output.push_str(render.markup());
    output.push_str("    <div role=\"host-bridge-command-policy\">Shared UI command intents are projected from HostBridgeServiceStatus through oxide-host-bridge; oxvba-fixture-evidenced commands remain adapter-target labels, not real DnaOxIde runtime/debug/Immediate/COM claims.</div>\n");
    output.push_str("  </section>\n");
}

fn render_shell_packet_baseline_section(
    output: &mut String,
    workspace_path: &Path,
    project_name: &str,
    modules: &[GuiShellModuleSummary],
    active_module: &str,
    source_text: &str,
) {
    let packet = build_shell_packet_baseline(
        workspace_path,
        project_name,
        modules,
        active_module,
        source_text,
    );

    output.push_str("  <section role=\"shell-packet\" data-source=\"oxide-core GuiShellPacket\" data-project=\"");
    output.push_str(&html_escape(&packet.project_name));
    output.push_str("\" data-active-module=\"");
    output.push_str(&html_escape(&packet.active_module));
    output.push_str("\" data-module-count=\"");
    output.push_str(&packet.modules.len().to_string());
    output.push_str("\" data-diagnostics-count=\"");
    output.push_str(&packet.diagnostics.len().to_string());
    output.push_str("\" data-native-execution-claimed=\"");
    output.push_str(if packet.native_execution_claimed {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-com-runtime-claimed=\"");
    output.push_str(if packet.com_runtime_claimed {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-web-framework-bound=\"");
    output.push_str(if packet.web_framework_bound {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-parked-tui-imported=\"");
    output.push_str(if packet.parked_tui_imported {
        "true"
    } else {
        "false"
    });
    output.push_str("\">\n");
    output.push_str("    <div role=\"shell-packet-workspace\">");
    output.push_str(&html_escape(&packet.workspace_path));
    output.push_str("</div>\n");
    output.push_str("    <section role=\"shell-packet-surfaces\">\n");
    for surface in [
        "project-spine",
        "source-editor",
        "diagnostics",
        "document-lifecycle",
        "run-output",
        "run-timeline",
        "com-capability",
        "command-palette",
        "keyboard-map",
        "focus-graph",
        "accessibility-projection",
        "capability-footer",
    ] {
        output.push_str("      <div role=\"shell-packet-surface\" data-surface=\"");
        output.push_str(surface);
        output.push_str("\"></div>\n");
    }
    output.push_str("    </section>\n");
    output.push_str("    <div role=\"shell-packet-command-count\">");
    output.push_str(&packet.command_palette.commands.len().to_string());
    output.push_str("</div>\n");
    output.push_str("    <div role=\"shell-packet-keybinding-count\">");
    output.push_str(&packet.keyboard_map.bindings.len().to_string());
    output.push_str("</div>\n");
    output.push_str("    <div role=\"shell-packet-focus-node-count\">");
    output.push_str(&packet.focus_graph.nodes.len().to_string());
    output.push_str("</div>\n");
    output.push_str("    <div role=\"shell-packet-accessibility-count\">");
    output.push_str(&packet.accessibility.nodes.len().to_string());
    output.push_str("</div>\n");
    output.push_str("    <div role=\"shell-packet-run-provider\" data-provider=\"");
    output.push_str(packet.run_capability.provider_kind.label());
    output.push_str("\">");
    if let Some(reason) = &packet.run_capability.disabled_reason {
        output.push_str(&html_escape(reason));
    }
    output.push_str("</div>\n");
    output.push_str("    <div role=\"shell-packet-com-profile\" data-profile=\"");
    output.push_str(packet.com_capability.host_kind.label());
    output.push_str("\">");
    output.push_str(&html_escape(
        packet
            .com_capability
            .runtime_invocation
            .reason
            .as_deref()
            .unwrap_or("COM runtime available"),
    ));
    output.push_str("</div>\n");
    output.push_str("    <div role=\"shell-packet-capability-footer\">");
    output.push_str(&html_escape(&packet.capability_footer));
    output.push_str("</div>\n");
    output.push_str("  </section>\n");
}

fn render_mounted_shell_static_section(
    output: &mut String,
    workspace_path: &Path,
    project_name: &str,
    modules: &[GuiShellModuleSummary],
    active_module: &str,
    source_text: &str,
) {
    let packet = build_shell_packet_baseline(
        workspace_path,
        project_name,
        modules,
        active_module,
        source_text,
    );

    output.push_str("  <section role=\"mounted-shell-static\" data-source=\"GuiShellPacket\" data-static-render=\"true\" data-dom-audited=\"false\" data-filesystem-persistence=\"false\" data-native-runtime=\"");
    output.push_str(if packet.native_execution_claimed {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-com-runtime=\"");
    output.push_str(if packet.com_runtime_claimed {
        "true"
    } else {
        "false"
    });
    output.push_str("\">\n");
    output.push_str("    <header role=\"mounted-shell-title\">");
    output.push_str(&html_escape(&packet.project_name));
    output.push_str("</header>\n");
    output.push_str("    <aside role=\"mounted-project-tree\" data-module-count=\"");
    output.push_str(&packet.modules.len().to_string());
    output.push_str("\">\n");
    for module in &packet.modules {
        output.push_str("      <div role=\"mounted-module-row\" data-active=\"");
        output.push_str(if module.is_active { "true" } else { "false" });
        output.push_str("\">");
        output.push_str(&html_escape(&module.display_name));
        output.push_str("</div>\n");
    }
    output.push_str("    </aside>\n");
    output.push_str("    <main role=\"mounted-editor\" data-active-module=\"");
    output.push_str(&html_escape(&packet.active_module));
    output.push_str("\"></main>\n");
    output.push_str("    <section role=\"mounted-diagnostics\" data-count=\"");
    output.push_str(&packet.diagnostics.len().to_string());
    output.push_str("\"></section>\n");
    output.push_str("    <section role=\"mounted-lifecycle\" data-profile=\"");
    output.push_str(&html_escape(&packet.lifecycle_profile_label));
    output.push_str("\" data-command-count=\"");
    output.push_str(&packet.lifecycle_commands.len().to_string());
    output.push_str("\"></section>\n");
    output.push_str("    <section role=\"mounted-run-output\" data-provider=\"");
    output.push_str(packet.run_capability.provider_kind.label());
    output.push_str("\" data-status=\"");
    output.push_str(packet.run_transcript.status.label());
    output.push_str("\">");
    if let Some(reason) = &packet.run_capability.disabled_reason {
        output.push_str(&html_escape(reason));
    }
    output.push_str("</section>\n");
    output.push_str("    <section role=\"mounted-com-capability\" data-profile=\"");
    output.push_str(packet.com_capability.host_kind.label());
    output.push_str("\" data-runtime-available=\"");
    output.push_str(if packet.com_capability.runtime_invocation.is_available {
        "true"
    } else {
        "false"
    });
    output.push_str("\"></section>\n");
    output.push_str("    <section role=\"mounted-command-palette\" data-command-count=\"");
    output.push_str(&packet.command_palette.commands.len().to_string());
    output.push_str("\"></section>\n");
    output.push_str(
        "    <section role=\"mounted-keyboard-focus-accessibility\" data-keybinding-count=\"",
    );
    output.push_str(&packet.keyboard_map.bindings.len().to_string());
    output.push_str("\" data-focus-node-count=\"");
    output.push_str(&packet.focus_graph.nodes.len().to_string());
    output.push_str("\" data-accessibility-surface-count=\"");
    output.push_str(&packet.accessibility.nodes.len().to_string());
    output.push_str("\"></section>\n");
    output.push_str("    <footer role=\"mounted-capability-footer\">");
    output.push_str(&html_escape(&packet.capability_footer));
    output.push_str("</footer>\n");
    output.push_str("    <div role=\"mounted-shell-policy\">Static shell render consumes GuiShellPacket; no DOM accessibility audit, filesystem persistence, native runtime, or COM runtime is claimed.</div>\n");
    output.push_str("  </section>\n");
}

fn render_mounted_command_palette_section(
    output: &mut String,
    workspace_path: &Path,
    project_name: &str,
    modules: &[GuiShellModuleSummary],
    active_module: &str,
    source_text: &str,
) {
    let packet = build_shell_packet_baseline(
        workspace_path,
        project_name,
        modules,
        active_module,
        source_text,
    );

    output.push_str("  <section role=\"mounted-command-palette-detail\" data-source=\"GuiShellPacket.command_palette\" data-parked-tui-imported=\"");
    output.push_str(if packet.parked_tui_imported {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-command-count=\"");
    output.push_str(&packet.command_palette.commands.len().to_string());
    output.push_str("\">\n");
    for command in &packet.command_palette.commands {
        let gesture = packet
            .keyboard_map
            .bindings
            .iter()
            .find(|binding| binding.command_id == command.id)
            .map(|binding| binding.gesture.display.as_str())
            .unwrap_or("unbound");
        output.push_str("    <div role=\"mounted-command-row\" data-command-id=\"");
        output.push_str(&html_escape(&command.stable_id));
        output.push_str("\" data-category=\"");
        output.push_str(command.category.label());
        output.push_str("\" data-gesture=\"");
        output.push_str(&html_escape(gesture));
        output.push_str("\" data-enabled=\"");
        output.push_str(if command.availability.is_enabled {
            "true"
        } else {
            "false"
        });
        output.push_str("\" data-capability=\"");
        output.push_str(&html_escape(&command.availability.capability_label));
        output.push_str("\">\n");
        output.push_str("      <span role=\"mounted-command-label\">");
        output.push_str(&html_escape(&command.label));
        output.push_str("</span>\n");
        if let Some(reason) = &command.availability.disabled_reason {
            output.push_str("      <span role=\"mounted-command-disabled-reason\">");
            output.push_str(&html_escape(reason));
            output.push_str("</span>\n");
        }
        output.push_str("    </div>\n");
    }
    output.push_str("    <div role=\"mounted-command-palette-policy\">Mounted command palette consumes packet state; parked TUI command model not imported.</div>\n");
    output.push_str("  </section>\n");
}

fn render_mounted_no_mouse_accessibility_section(
    output: &mut String,
    workspace_path: &Path,
    project_name: &str,
    modules: &[GuiShellModuleSummary],
    active_module: &str,
    source_text: &str,
) {
    let packet = build_shell_packet_baseline(
        workspace_path,
        project_name,
        modules,
        active_module,
        source_text,
    );

    output.push_str("  <section role=\"mounted-no-mouse-accessibility\" data-source=\"GuiShellPacket.focus_graph+accessibility\" data-web-framework-bound=\"");
    output.push_str(if packet.web_framework_bound {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-dom-audited=\"false\" data-route-length=\"");
    output.push_str(&packet.focus_graph.no_mouse_route.len().to_string());
    output.push_str("\" data-accessibility-surface-count=\"");
    output.push_str(&packet.accessibility.nodes.len().to_string());
    output.push_str("\">\n");
    output.push_str("    <section role=\"mounted-no-mouse-route\">\n");
    for step in &packet.focus_graph.no_mouse_route {
        output.push_str("      <div role=\"mounted-focus-step\" data-index=\"");
        output.push_str(&step.index.to_string());
        output.push_str("\" data-node-id=\"");
        output.push_str(&html_escape(&step.node_id));
        output.push_str("\">\n");
        if let Some(restoration_hint) = &step.restoration_hint {
            output.push_str("        <span role=\"mounted-focus-restoration\">");
            output.push_str(&html_escape(restoration_hint));
            output.push_str("</span>\n");
        }
        output.push_str("      </div>\n");
    }
    output.push_str("    </section>\n");
    output.push_str("    <section role=\"mounted-accessibility-surfaces\">\n");
    for node in &packet.accessibility.nodes {
        output.push_str("      <div role=\"mounted-accessible-surface\" data-surface-id=\"");
        output.push_str(&html_escape(&node.surface_id));
        output.push_str("\" data-role=\"");
        output.push_str(node.role.label());
        output.push_str("\" data-has-disabled-reason=\"");
        output.push_str(if node.disabled_reason.is_some() {
            "true"
        } else {
            "false"
        });
        output.push_str("\">\n");
        output.push_str("        <span role=\"mounted-accessible-label\">");
        output.push_str(&html_escape(&node.accessible_label));
        output.push_str("</span>\n");
        output.push_str("        <span role=\"mounted-accessible-description\">");
        output.push_str(&html_escape(&node.accessible_description));
        output.push_str("</span>\n");
        if let Some(reason) = &node.disabled_reason {
            output.push_str("        <span role=\"mounted-accessible-disabled-reason\">");
            output.push_str(&html_escape(reason));
            output.push_str("</span>\n");
        }
        output.push_str("      </div>\n");
    }
    output.push_str("    </section>\n");
    output.push_str("    <div role=\"mounted-accessibility-policy\">Mounted accessibility projection is packet-derived; DOM accessibility audit is not claimed.</div>\n");
    output.push_str("  </section>\n");
}

fn render_web_shell_adapter_boundary_section(
    output: &mut String,
    workspace_path: &Path,
    project_name: &str,
    modules: &[GuiShellModuleSummary],
    active_module: &str,
    source_text: &str,
) {
    let packet = build_shell_packet_baseline(
        workspace_path,
        project_name,
        modules,
        active_module,
        source_text,
    );
    let snapshot = render_web_shell_snapshot(&packet);

    output.push_str("  <section role=\"web-shell-boundary-snapshot\" data-source=\"");
    output.push_str(snapshot.source_contract);
    output.push_str("\" data-dom-smoke-tested=\"");
    output.push_str(if snapshot.dom_smoke_tested {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-dom-audited=\"");
    output.push_str(if snapshot.dom_accessibility_audited {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-filesystem-persistence=\"");
    output.push_str(if snapshot.filesystem_persistence_claimed {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-native-runtime=\"");
    output.push_str(if snapshot.native_runtime_claimed {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-com-runtime=\"");
    output.push_str(if snapshot.com_runtime_claimed {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-parked-tui-imported=\"");
    output.push_str(if snapshot.parked_tui_imported {
        "true"
    } else {
        "false"
    });
    output.push_str("\">\n");
    output.push_str(snapshot.markup());
    output.push_str("  </section>\n");
}

fn render_web_shell_dom_smoke_section(
    output: &mut String,
    workspace_path: &Path,
    project_name: &str,
    modules: &[GuiShellModuleSummary],
    active_module: &str,
    source_text: &str,
) {
    let packet = build_shell_packet_baseline(
        workspace_path,
        project_name,
        modules,
        active_module,
        source_text,
    );
    let report = run_static_shell_dom_smoke(&packet);

    output.push_str("  <section role=\"web-shell-dom-smoke\" data-source=\"");
    output.push_str(report.snapshot.source_contract);
    output.push_str("\" data-smoke-kind=\"");
    output.push_str(report.smoke_kind);
    output.push_str("\" data-dom-smoke-tested=\"");
    output.push_str(if report.dom_smoke_tested {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-browser-runtime=\"");
    output.push_str(if report.browser_runtime_claimed {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-dom-audited=\"");
    output.push_str(if report.dom_accessibility_audited {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-all-passed=\"");
    output.push_str(if report.all_passed() { "true" } else { "false" });
    output.push_str("\" data-check-count=\"");
    output.push_str(&report.checks.len().to_string());
    output.push_str("\">\n");
    for check in &report.checks {
        output.push_str("    <div role=\"web-dom-smoke-check\" data-check=\"");
        output.push_str(&html_escape(&check.name));
        output.push_str("\" data-passed=\"");
        output.push_str(if check.passed { "true" } else { "false" });
        output.push_str("\">");
        output.push_str(&html_escape(&check.detail));
        output.push_str("</div>\n");
    }
    output.push_str("    <div role=\"web-dom-smoke-policy\">Parsed HTML DOM smoke only; no browser runtime or DOM accessibility audit is claimed.</div>\n");
    output.push_str("  </section>\n");
}

fn render_web_command_palette_dom_smoke_section(
    output: &mut String,
    workspace_path: &Path,
    project_name: &str,
    modules: &[GuiShellModuleSummary],
    active_module: &str,
    source_text: &str,
) {
    let packet = build_shell_packet_baseline(
        workspace_path,
        project_name,
        modules,
        active_module,
        source_text,
    );
    let report = run_command_palette_dom_smoke(&packet);

    output.push_str("  <section role=\"web-command-palette-dom-smoke\" data-source=\"");
    output.push_str(report.snapshot.source_contract);
    output.push_str("\" data-smoke-kind=\"");
    output.push_str(report.smoke_kind);
    output.push_str("\" data-dom-smoke-tested=\"");
    output.push_str(if report.dom_smoke_tested {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-browser-runtime=\"");
    output.push_str(if report.browser_runtime_claimed {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-dom-audited=\"");
    output.push_str(if report.dom_accessibility_audited {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-all-passed=\"");
    output.push_str(if report.all_passed() { "true" } else { "false" });
    output.push_str("\" data-check-count=\"");
    output.push_str(&report.checks.len().to_string());
    output.push_str("\">\n");
    for check in &report.checks {
        output.push_str("    <div role=\"web-command-palette-dom-check\" data-check=\"");
        output.push_str(&html_escape(&check.name));
        output.push_str("\" data-passed=\"");
        output.push_str(if check.passed { "true" } else { "false" });
        output.push_str("\">");
        output.push_str(&html_escape(&check.detail));
        output.push_str("</div>\n");
    }
    output.push_str("    <div role=\"web-command-palette-dom-policy\">Command palette DOM smoke consumes GuiShellPacket command_palette and keyboard_map; parked TUI command model remains isolated.</div>\n");
    output.push_str("  </section>\n");
}

fn render_web_no_mouse_accessibility_dom_smoke_section(
    output: &mut String,
    workspace_path: &Path,
    project_name: &str,
    modules: &[GuiShellModuleSummary],
    active_module: &str,
    source_text: &str,
) {
    let packet = build_shell_packet_baseline(
        workspace_path,
        project_name,
        modules,
        active_module,
        source_text,
    );
    let report = run_no_mouse_accessibility_dom_smoke(&packet);

    output.push_str("  <section role=\"web-no-mouse-accessibility-dom-smoke\" data-source=\"");
    output.push_str(report.snapshot.source_contract);
    output.push_str("\" data-smoke-kind=\"");
    output.push_str(report.smoke_kind);
    output.push_str("\" data-dom-smoke-tested=\"");
    output.push_str(if report.dom_smoke_tested {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-browser-runtime=\"");
    output.push_str(if report.browser_runtime_claimed {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-dom-audited=\"");
    output.push_str(if report.dom_accessibility_audited {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-all-passed=\"");
    output.push_str(if report.all_passed() { "true" } else { "false" });
    output.push_str("\" data-check-count=\"");
    output.push_str(&report.checks.len().to_string());
    output.push_str("\">\n");
    for check in &report.checks {
        output.push_str("    <div role=\"web-no-mouse-accessibility-dom-check\" data-check=\"");
        output.push_str(&html_escape(&check.name));
        output.push_str("\" data-passed=\"");
        output.push_str(if check.passed { "true" } else { "false" });
        output.push_str("\">");
        output.push_str(&html_escape(&check.detail));
        output.push_str("</div>\n");
    }
    output.push_str("    <div role=\"web-no-mouse-accessibility-dom-policy\">No-mouse/accessibility DOM smoke consumes GuiShellPacket focus_graph and accessibility; this is not a full accessibility audit.</div>\n");
    output.push_str("  </section>\n");
}

fn render_dnaonecalc_web_shell_host_contract_section(
    output: &mut String,
    workspace_path: &Path,
    project_name: &str,
    modules: &[GuiShellModuleSummary],
    active_module: &str,
    source_text: &str,
) {
    let packet = build_dnaonecalc_web_shell_host_packet(
        workspace_path,
        project_name,
        modules,
        active_module,
        source_text,
    );

    output.push_str("  <section role=\"dnaonecalc-web-shell-host-contract\" data-host=\"");
    output.push_str(&html_escape(&packet.embedded_ide.consumer.host_name));
    output.push_str("\" data-state-contract=\"");
    output.push_str(&html_escape(&packet.web_shell.state_contract));
    output.push_str("\" data-embedding-contract=\"EmbeddedIdePacket\" data-web-adapter=\"");
    output.push_str(&html_escape(&packet.web_shell.adapter_crate));
    output.push_str("\" data-sibling-repo-writes=\"");
    output.push_str(if packet.sibling_repo_writes() {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-host-mount-claimed=\"");
    output.push_str(if packet.web_shell.host_mount_claimed {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-filesystem-persistence=\"");
    output.push_str(if packet.web_shell.filesystem_persistence_claimed {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-native-runtime=\"");
    output.push_str(if packet.web_shell.native_runtime_claimed {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-com-runtime=\"");
    output.push_str(if packet.web_shell.com_runtime_claimed {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-dom-audited=\"");
    output.push_str(if packet.web_shell.dom_accessibility_audit_claimed {
        "true"
    } else {
        "false"
    });
    output.push_str("\">\n");
    output.push_str("    <div role=\"host-product-role\">");
    output.push_str(&html_escape(&packet.embedded_ide.consumer.product_role));
    output.push_str("</div>\n");
    output.push_str("    <section role=\"host-mount-inputs\">\n");
    for input in &packet.required_mount_inputs {
        output.push_str("      <div role=\"host-mount-input\">");
        output.push_str(&html_escape(input));
        output.push_str("</div>\n");
    }
    output.push_str("    </section>\n");
    output.push_str("    <section role=\"host-surface-slots\">\n");
    for surface in &packet.embedded_ide.surfaces {
        output.push_str("      <div role=\"host-surface-slot\" data-slot=\"");
        output.push_str(&html_escape(&surface.slot_id));
        output.push_str("\" data-owner=\"");
        output.push_str(&html_escape(&surface.owner));
        output.push_str("\">");
        output.push_str(&html_escape(&surface.label));
        output.push_str("</div>\n");
    }
    output.push_str("    </section>\n");
    output.push_str("    <section role=\"host-ownership-boundaries\">\n");
    for boundary in &packet.embedded_ide.ownership_boundaries {
        output.push_str("      <div role=\"host-ownership-boundary\" data-owner=\"");
        output.push_str(&html_escape(&boundary.owner));
        output.push_str("\">");
        output.push_str(&html_escape(&boundary.responsibility));
        output.push_str("</div>\n");
    }
    output.push_str("    </section>\n");
    output.push_str("    <section role=\"host-web-shell-summary\" data-project=\"");
    output.push_str(&html_escape(&packet.web_shell.project_name));
    output.push_str("\" data-active-module=\"");
    output.push_str(&html_escape(&packet.web_shell.active_module));
    output.push_str("\" data-module-count=\"");
    output.push_str(&packet.web_shell.module_count.to_string());
    output.push_str("\" data-command-count=\"");
    output.push_str(&packet.web_shell.command_count.to_string());
    output.push_str("\" data-keybinding-count=\"");
    output.push_str(&packet.web_shell.keybinding_count.to_string());
    output.push_str("\" data-focus-route-length=\"");
    output.push_str(&packet.web_shell.focus_route_length.to_string());
    output.push_str("\" data-accessibility-surface-count=\"");
    output.push_str(&packet.web_shell.accessibility_surface_count.to_string());
    output.push_str("\"></section>\n");
    output.push_str("    <section role=\"host-dom-readiness\" data-smoke-kind=\"");
    output.push_str(&html_escape(&packet.web_shell.dom_readiness.smoke_kind));
    output.push_str("\" data-all-passed=\"");
    output.push_str(if packet.web_shell.dom_readiness.all_passed() {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-static-shell=\"");
    output.push_str(
        if packet.web_shell.dom_readiness.static_shell_dom_smoke_passed {
            "true"
        } else {
            "false"
        },
    );
    output.push_str("\" data-command-palette=\"");
    output.push_str(
        if packet
            .web_shell
            .dom_readiness
            .command_palette_dom_smoke_passed
        {
            "true"
        } else {
            "false"
        },
    );
    output.push_str("\" data-no-mouse-accessibility=\"");
    output.push_str(
        if packet
            .web_shell
            .dom_readiness
            .no_mouse_accessibility_dom_smoke_passed
        {
            "true"
        } else {
            "false"
        },
    );
    output.push_str("\" data-browser-runtime=\"");
    output.push_str(if packet.web_shell.dom_readiness.browser_runtime_claimed {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-dom-audited=\"");
    output.push_str(
        if packet
            .web_shell
            .dom_readiness
            .dom_accessibility_audit_claimed
        {
            "true"
        } else {
            "false"
        },
    );
    output.push_str("\"></section>\n");
    output.push_str("    <section role=\"host-limitations\">\n");
    for limitation in &packet.limitations {
        output.push_str("      <div role=\"host-limitation\">");
        output.push_str(&html_escape(limitation));
        output.push_str("</div>\n");
    }
    output.push_str("    </section>\n");
    output.push_str("  </section>\n");
}

fn render_dnaonecalc_web_shell_dom_readiness_section(
    output: &mut String,
    workspace_path: &Path,
    project_name: &str,
    modules: &[GuiShellModuleSummary],
    active_module: &str,
    source_text: &str,
) {
    let shell_packet = build_shell_packet_baseline(
        workspace_path,
        project_name,
        modules,
        active_module,
        source_text,
    );
    let static_report = run_static_shell_dom_smoke(&shell_packet);
    let command_report = run_command_palette_dom_smoke(&shell_packet);
    let accessibility_report = run_no_mouse_accessibility_dom_smoke(&shell_packet);

    output.push_str("  <section role=\"dnaonecalc-web-shell-dom-readiness\" data-host=\"DnaOneCalc\" data-source=\"W300 DOM smoke reports\" data-static-shell=\"");
    output.push_str(if static_report.all_passed() {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-command-palette=\"");
    output.push_str(if command_report.all_passed() {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-no-mouse-accessibility=\"");
    output.push_str(if accessibility_report.all_passed() {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-browser-runtime=\"false\" data-dnaonecalc-host-smoke=\"false\" data-dom-audited=\"false\" data-filesystem-persistence=\"false\" data-native-runtime=\"false\" data-com-runtime=\"false\">\n");
    for (label, report) in [
        ("static-shell", &static_report),
        ("command-palette", &command_report),
        ("no-mouse-accessibility", &accessibility_report),
    ] {
        output.push_str("    <section role=\"dnaonecalc-dom-readiness-report\" data-report=\"");
        output.push_str(label);
        output.push_str("\" data-smoke-kind=\"");
        output.push_str(report.smoke_kind);
        output.push_str("\" data-all-passed=\"");
        output.push_str(if report.all_passed() { "true" } else { "false" });
        output.push_str("\" data-browser-runtime=\"");
        output.push_str(if report.browser_runtime_claimed {
            "true"
        } else {
            "false"
        });
        output.push_str("\" data-dom-audited=\"");
        output.push_str(if report.dom_accessibility_audited {
            "true"
        } else {
            "false"
        });
        output.push_str("\" data-check-count=\"");
        output.push_str(&report.checks.len().to_string());
        output.push_str("\"></section>\n");
    }
    output.push_str("    <div role=\"dnaonecalc-dom-readiness-policy\">OxIde parsed HTML DOM readiness only; DnaOneCalc browser host smoke, filesystem persistence, native runtime, COM runtime, and full accessibility audit are not claimed.</div>\n");
    output.push_str("  </section>\n");
}

struct GuiLabTempProjectCopy {
    temp_dir: PathBuf,
    module_path: PathBuf,
    fixture_module_path: PathBuf,
    fixture_source_before: String,
}

impl GuiLabTempProjectCopy {
    fn checked_in_fixture_mutated(&self) -> bool {
        fs::read_to_string(&self.fixture_module_path)
            .map(|current| current != self.fixture_source_before)
            .unwrap_or(true)
    }
}

fn create_test_owned_temp_project_copy(
    workspace_path: &Path,
    active_module: &str,
    source_text: &str,
    scenario_slug: &str,
) -> GuiLabTempProjectCopy {
    let fixture_module_path = workspace_path
        .parent()
        .expect("thin-slice fixture has parent")
        .join(active_module);
    let fixture_source_before =
        fs::read_to_string(&fixture_module_path).expect("read checked-in fixture module");
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time after epoch")
        .as_nanos();
    let temp_dir = std::env::temp_dir().join(format!(
        "oxide-guilab-w320-{scenario_slug}-{}-{unique}",
        std::process::id()
    ));
    fs::create_dir_all(&temp_dir).expect("create GUI-lab test-owned temp project dir");
    let module_path = temp_dir.join(active_module);
    fs::write(&module_path, source_text).expect("write temp module copy");
    let temp_workspace_path = temp_dir.join(
        workspace_path
            .file_name()
            .expect("thin-slice workspace file name"),
    );
    fs::write(
        &temp_workspace_path,
        fs::read_to_string(workspace_path).expect("read checked-in basproj fixture"),
    )
    .expect("write temp basproj copy");
    GuiLabTempProjectCopy {
        temp_dir,
        module_path,
        fixture_module_path,
        fixture_source_before,
    }
}

fn render_native_save_reload_disk_section(
    output: &mut String,
    workspace_path: &Path,
    active_module: &str,
    source_text: &str,
) {
    let temp_project = create_test_owned_temp_project_copy(
        workspace_path,
        active_module,
        source_text,
        "save-reload",
    );
    let mut persistence = NativeFilesystemDocumentPersistence::new(&temp_project.module_path);
    let mut document = open_lifecycle_from_persistence(&persistence).expect("native load");
    let edited_source = source_text.replace("answer = 40 + 2", "answer = 21 * 2");
    document.edit_working_source(edited_source);
    let dirty_before_save = document.is_dirty();
    save_lifecycle_to_persistence(&mut document, &mut persistence).expect("native save");
    let disk_source = persistence.load().expect("native reload disk source");
    document
        .reload_from_persisted(disk_source.clone())
        .expect("native reload");
    let checked_in_fixture_mutated = temp_project.checked_in_fixture_mutated();
    let projection = GuiNativeSaveReloadProjection::from_disk_round_trip(
        dirty_before_save,
        &document,
        &disk_source,
        checked_in_fixture_mutated,
    );

    output.push_str("  <section role=\"native-save-reload-disk\" data-provider=\"");
    output.push_str(&html_escape(&projection.provider_label));
    output.push_str("\" data-filesystem-persistence=\"");
    output.push_str(if projection.filesystem_persistence {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-test-owned-temp-project=\"");
    output.push_str(if projection.test_owned_temp_project {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-checked-in-fixture-mutated=\"");
    output.push_str(if projection.checked_in_fixture_mutated {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-dirty-before-save=\"");
    output.push_str(if projection.dirty_before_save {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-dirty-after-save=\"");
    output.push_str(if projection.dirty_after_save {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-save-acknowledged=\"");
    output.push_str(if projection.save_acknowledged {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-reload-source-matches-disk=\"");
    output.push_str(if projection.reload_source_matches_disk {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-native-runtime=\"");
    output.push_str(if projection.native_runtime {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-com-runtime=\"");
    output.push_str(if projection.com_runtime {
        "true"
    } else {
        "false"
    });
    output.push_str("\">\n");
    output.push_str("    <div role=\"native-persistence-source\">");
    output.push_str(&html_escape(&projection.saved_source_excerpt));
    output.push_str("</div>\n");
    output.push_str("    <div role=\"native-reload-source\">");
    output.push_str(&html_escape(&projection.reloaded_source_excerpt));
    output.push_str("</div>\n");
    output.push_str("    <div role=\"native-persistence-policy\">Disk-backed save/reload is proven only against a GUI-lab test-owned temp project copy; native runtime and COM runtime are not claimed.</div>\n");
    output.push_str("  </section>\n");
}

fn render_native_session_restore_disk_section(
    output: &mut String,
    workspace_path: &Path,
    active_module: &str,
    source_text: &str,
) {
    let temp_project = create_test_owned_temp_project_copy(
        workspace_path,
        active_module,
        source_text,
        "session-restore",
    );
    let session_path = temp_project.temp_dir.join("oxide.session.json");
    let mut document =
        DocumentLifecycleState::open_clean(source_text, LifecycleCapabilities::all_supported());
    document.edit_working_source(source_text.replace("answer = 40 + 2", "answer = 84 / 2"));
    document
        .acknowledge_saved()
        .expect("native session scenario save acknowledgement");
    let snapshot = GuiSessionSnapshot::capture(
        temp_project
            .temp_dir
            .join(workspace_path.file_name().expect("workspace file name"))
            .display()
            .to_string(),
        active_module,
        &document,
        SessionCapabilityProfile::AllSupported,
    );
    let session_persistence = NativeFilesystemSessionPersistence::new(&session_path);
    session_persistence
        .save_snapshot(&snapshot)
        .expect("save native session snapshot");
    let loaded = session_persistence
        .load_snapshot()
        .expect("load native session snapshot");
    let restored = loaded.restore();
    let checked_in_fixture_mutated = temp_project.checked_in_fixture_mutated();

    output.push_str("  <section role=\"native-session-restore-disk\" data-provider=\"native-filesystem\" data-session-provider=\"");
    output.push_str(session_persistence.provider_label());
    output.push_str("\" data-filesystem-persistence=\"true\" data-test-owned-temp-project=\"true\" data-session-file-written=\"");
    output.push_str(if session_persistence.session_path().exists() {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-checked-in-fixture-mutated=\"");
    output.push_str(if checked_in_fixture_mutated {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-restored-dirty=\"");
    output.push_str(if restored.document.is_dirty() {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-native-runtime=\"false\" data-com-runtime=\"false\">\n");
    output.push_str("    <div role=\"native-session-workspace\">");
    output.push_str(&html_escape(&restored.workspace_path));
    output.push_str("</div>\n");
    output.push_str("    <div role=\"native-session-module\">");
    output.push_str(&html_escape(&restored.active_module));
    output.push_str("</div>\n");
    output.push_str("    <div role=\"native-session-source\">");
    output.push_str(&html_escape(
        restored
            .document
            .working_source()
            .lines()
            .find(|line| line.contains("answer ="))
            .unwrap_or(""),
    ));
    output.push_str("</div>\n");
    output.push_str("    <div role=\"native-session-policy\">OxIde-owned session JSON persisted through a native filesystem provider; .basproj semantics remain OxVba-owned, and runtime/COM are not claimed.</div>\n");
    output.push_str("  </section>\n");
}

fn render_browser_filesystem_still_disabled_section(output: &mut String, source_text: &str) {
    let mut document =
        DocumentLifecycleState::open_clean(source_text, LifecycleCapabilities::browser_limited());
    document.edit_working_source(source_text.replace("answer = 40 + 2", "answer = 21 * 2"));
    let projection = GuiBrowserFilesystemDisabledProjection::from_document(&document);

    output.push_str("  <section role=\"browser-filesystem-still-disabled\" data-provider=\"");
    output.push_str(&html_escape(&projection.provider_label));
    output.push_str("\" data-filesystem-persistence=\"");
    output.push_str(if projection.filesystem_persistence {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-save-enabled=\"");
    output.push_str(if projection.save_command.is_enabled {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-reload-enabled=\"");
    output.push_str(if projection.reload_command.is_enabled {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-native-runtime=\"");
    output.push_str(if projection.native_runtime {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-com-runtime=\"");
    output.push_str(if projection.com_runtime {
        "true"
    } else {
        "false"
    });
    output.push_str("\">\n");
    output.push_str("    <div role=\"browser-filesystem-disabled-reason\">");
    output.push_str(&html_escape(&projection.disabled_reason));
    output.push_str("</div>\n");
    output.push_str("    <div role=\"browser-filesystem-policy\">Browser/WASM direct filesystem persistence remains disabled; native disk persistence is a separate capability profile.</div>\n");
    output.push_str("  </section>\n");
}

fn render_runtime_service_contract_section(output: &mut String, packet: RuntimeServicePacket) {
    output.push_str("  <section role=\"runtime-service-contract\" data-provider=\"");
    output.push_str(packet.provider_label());
    output.push_str("\" data-command-enabled=\"");
    output.push_str(if packet.command_status.is_enabled {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-real-execution=\"");
    output.push_str(if packet.real_execution_claimed {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-native-runtime=\"");
    output.push_str(if packet.native_runtime_claimed {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-com-runtime=\"");
    output.push_str(if packet.com_runtime_claimed {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-event-count=\"");
    output.push_str(&packet.events.len().to_string());
    output.push_str("\">\n");
    output.push_str("    <div role=\"runtime-service-target\">");
    output.push_str(&html_escape(&packet.request.display_target()));
    output.push_str("</div>\n");
    if let Some(reason) = &packet.command_status.disabled_reason {
        output.push_str("    <div role=\"runtime-service-disabled-reason\">");
        output.push_str(&html_escape(reason));
        output.push_str("</div>\n");
    }
    for event in &packet.events {
        output.push_str("    <div role=\"runtime-service-event\" data-event-kind=\"");
        output.push_str(event.kind.label());
        output.push_str("\">");
        output.push_str(&html_escape(&event.message));
        output.push_str("</div>\n");
    }
    output.push_str("    <div role=\"runtime-service-policy\">Runtime service contract only; real OxVba execution, native runtime, and COM runtime are not claimed.</div>\n");
    output.push_str("  </section>\n");
}

fn render_immediate_service_contract_section(output: &mut String, packet: ImmediateServicePacket) {
    output.push_str("  <section role=\"immediate-service-contract\" data-provider=\"");
    output.push_str(packet.provider_label());
    output.push_str("\" data-command-enabled=\"");
    output.push_str(if packet.command_status.is_enabled {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-response-count=\"");
    output.push_str(&packet.responses.len().to_string());
    output.push_str("\" data-fake-responses=\"");
    output.push_str(if packet.fake_responses {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-native-runtime=\"");
    output.push_str(if packet.native_runtime_claimed {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-com-runtime=\"");
    output.push_str(if packet.com_runtime_claimed {
        "true"
    } else {
        "false"
    });
    output.push_str("\">\n");
    if let Some(request_text) = &packet.request_text {
        output.push_str("    <div role=\"immediate-service-request\">");
        output.push_str(&html_escape(request_text));
        output.push_str("</div>\n");
    }
    if let Some(reason) = &packet.command_status.reason {
        output.push_str("    <div role=\"immediate-service-disabled-reason\">");
        output.push_str(&html_escape(reason));
        output.push_str("</div>\n");
    }
    output.push_str("    <div role=\"immediate-service-policy\">No Immediate responses are rendered without a native OxVba runtime service; fake responses are not allowed.</div>\n");
    output.push_str("  </section>\n");
}

fn render_debug_service_contract_section(output: &mut String, packet: DebugServicePacket) {
    output.push_str("  <section role=\"debug-service-contract\" data-provider=\"");
    output.push_str(packet.provider_label());
    output.push_str("\" data-state=\"");
    output.push_str(packet.state_label());
    output.push_str("\" data-command-enabled=\"");
    output.push_str(if packet.command_status.is_enabled {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-command-count=\"");
    output.push_str(&packet.debug_commands.len().to_string());
    output.push_str("\" data-callstack-count=\"");
    output.push_str(&packet.callstack.len().to_string());
    output.push_str("\" data-locals-count=\"");
    output.push_str(&packet.locals.len().to_string());
    output.push_str("\" data-watches-count=\"");
    output.push_str(&packet.watches.len().to_string());
    output.push_str("\" data-breakpoints-count=\"");
    output.push_str(&packet.breakpoints.len().to_string());
    output.push_str("\" data-fake-debug-data=\"");
    output.push_str(if packet.fake_debug_data {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-native-runtime=\"");
    output.push_str(if packet.native_runtime_claimed {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-com-runtime=\"");
    output.push_str(if packet.com_runtime_claimed {
        "true"
    } else {
        "false"
    });
    output.push_str("\">\n");
    if let Some(reason) = &packet.command_status.reason {
        output.push_str("    <div role=\"debug-service-disabled-reason\">");
        output.push_str(&html_escape(reason));
        output.push_str("</div>\n");
    }
    for command in &packet.debug_commands {
        output.push_str("    <div role=\"debug-service-command\" data-command=\"");
        output.push_str(command.command_id.label());
        output.push_str("\" data-enabled=\"");
        output.push_str(if command.is_enabled { "true" } else { "false" });
        output.push_str("\"></div>\n");
    }
    output.push_str("    <div role=\"debug-service-policy\">No callstack, locals, watches, or breakpoints are rendered without a native OxVba debug service; fake debug data is not allowed.</div>\n");
    output.push_str("  </section>\n");
}

fn render_com_capability_section(output: &mut String, profile: ComCapabilityProfile) {
    output.push_str("  <section role=\"com-capability\" data-profile=\"");
    output.push_str(profile.host_kind.label());
    output.push_str("\" data-native-execution=\"");
    output.push_str(if profile.native_execution_available {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-com-service-configured=\"");
    output.push_str(if profile.native_com_service_configured {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-windows-native-host-required=\"");
    output.push_str(if profile.windows_native_host_required {
        "true"
    } else {
        "false"
    });
    output.push_str("\">\n");
    output.push_str("    <div role=\"com-reference-fact\">");
    output.push_str(&html_escape(&profile.reference.display_name));
    output.push_str("</div>\n");
    output.push_str("    <div role=\"com-reference-identifier\">");
    output.push_str(&html_escape(&profile.reference.identifier));
    output.push_str("</div>\n");
    output.push_str("    <div role=\"com-reference-source\">");
    output.push_str(&html_escape(&profile.reference.source_label));
    output.push_str("</div>\n");
    render_com_status(output, &profile.reference_discovery);
    render_com_status(output, &profile.runtime_invocation);
    if profile.windows_native_host_required {
        output.push_str("    <div role=\"com-required-host\">Windows native host required</div>\n");
    }
    output.push_str("    <div role=\"com-honesty-note\">No COM runtime support is claimed in this profile.</div>\n");
    output.push_str("  </section>\n");
}

fn render_com_status(output: &mut String, status: &ComCapabilityStatus) {
    output.push_str("    <div role=\"com-status\" data-feature=\"");
    output.push_str(status.feature.label());
    output.push_str("\" data-available=\"");
    output.push_str(if status.is_available { "true" } else { "false" });
    output.push_str("\">");
    if status.is_available {
        output.push_str("available");
    } else if let Some(reason) = &status.reason {
        output.push_str(&html_escape(reason));
    } else {
        output.push_str("unavailable");
    }
    output.push_str("</div>\n");
}

fn render_dnaonecalc_embedding_contract_section(
    output: &mut String,
    workspace_path: &Path,
    project_name: &str,
    module_display_name: &str,
    source_text: &str,
) {
    let packet = EmbeddedIdePacket::dnaonecalc_thin_slice_browser_disabled(
        workspace_path.display().to_string(),
        project_name,
        module_display_name,
        source_text,
    );

    output.push_str("  <section role=\"embedded-host-contract\" data-host=\"");
    output.push_str(&html_escape(&packet.consumer.host_name));
    output.push_str("\" data-sibling-repo-writes=\"");
    output.push_str(if packet.sibling_repo_writes {
        "true"
    } else {
        "false"
    });
    output.push_str("\">\n");
    output.push_str("    <div role=\"embedded-consumer\">");
    output.push_str(&html_escape(&packet.consumer.host_name));
    output.push_str(" — ");
    output.push_str(&html_escape(&packet.consumer.product_role));
    output.push_str("</div>\n");
    output.push_str("    <div role=\"host-shell-owner\">");
    output.push_str(&html_escape(&packet.consumer.shell_owner));
    output.push_str("</div>\n");
    output.push_str("    <section role=\"embedded-surface-slots\">\n");
    for surface in &packet.surfaces {
        output.push_str("      <div role=\"embedded-surface\" data-slot=\"");
        output.push_str(&html_escape(&surface.slot_id));
        output.push_str("\" data-owner=\"");
        output.push_str(&html_escape(&surface.owner));
        output.push_str("\">");
        output.push_str(&html_escape(&surface.label));
        output.push_str("</div>\n");
    }
    output.push_str("    </section>\n");
    output.push_str("    <section role=\"ownership-boundaries\">\n");
    for boundary in &packet.ownership_boundaries {
        output.push_str("      <div role=\"ownership-boundary\" data-owner=\"");
        output.push_str(&html_escape(&boundary.owner));
        output.push_str("\">");
        output.push_str(&html_escape(&boundary.responsibility));
        output.push_str("</div>\n");
    }
    output.push_str("    </section>\n");
    output.push_str("    <section role=\"embedded-run-capability\" data-provider=\"");
    output.push_str(packet.run_capability.provider_kind.label());
    output.push_str("\" data-status=\"");
    output.push_str(packet.run_transcript.status.label());
    output.push_str("\" data-native-execution=\"");
    output.push_str(if packet.run_capability.native_execution_available {
        "true"
    } else {
        "false"
    });
    output.push_str("\" data-com-runtime=\"");
    output.push_str(if packet.run_capability.com_runtime_available {
        "true"
    } else {
        "false"
    });
    output.push_str("\">\n");
    output.push_str("      <div role=\"embedded-run-target\">");
    output.push_str(&html_escape(
        &packet.run_transcript.request.display_target(),
    ));
    output.push_str("</div>\n");
    for event in &packet.run_transcript.events {
        output.push_str("      <div role=\"embedded-run-event\" data-event-kind=\"");
        output.push_str(event.kind.label());
        output.push_str("\">");
        output.push_str(&html_escape(&event.message));
        output.push_str("</div>\n");
    }
    output.push_str("    </section>\n");
    output.push_str("    <section role=\"embedding-limitations\">\n");
    for limitation in &packet.limitations {
        output.push_str("      <div role=\"embedding-limitation\">");
        output.push_str(&html_escape(limitation));
        output.push_str("</div>\n");
    }
    output.push_str("    </section>\n");
    output.push_str("  </section>\n");
}

fn module_stem(module_display_name: &str) -> String {
    module_display_name
        .strip_suffix(".bas")
        .unwrap_or(module_display_name)
        .to_string()
}

fn edited_diagnostics_source(source_text: &str) -> String {
    SourceSnapshot::new(source_text)
        .apply(&EditOperation::remove_first_answer_declaration())
        .snapshot
        .text()
        .to_string()
}

fn render_lifecycle_section(
    output: &mut String,
    workspace_path: &Path,
    active_module: &str,
    persisted_source: &str,
    working_source: &str,
) {
    let mut browser_state = DocumentLifecycleState::open_clean(
        persisted_source,
        LifecycleCapabilities::browser_limited(),
    );
    browser_state.edit_working_source(working_source);

    output.push_str(
        "  <section role=\"document-lifecycle\" data-provider=\"browser-limited\" data-dirty=\"",
    );
    output.push_str(if browser_state.is_dirty() {
        "true"
    } else {
        "false"
    });
    output.push_str("\">\n");
    output.push_str("    <div role=\"document-state\">");
    output.push_str(if browser_state.is_dirty() {
        "dirty"
    } else {
        "clean"
    });
    output.push_str("</div>\n");
    render_lifecycle_command(
        output,
        LifecycleCommand::Save,
        browser_state.command_status(LifecycleCommand::Save),
    );
    render_lifecycle_command(
        output,
        LifecycleCommand::Reload,
        browser_state.command_status(LifecycleCommand::Reload),
    );
    render_lifecycle_command(
        output,
        LifecycleCommand::Revert,
        browser_state.command_status(LifecycleCommand::Revert),
    );
    output.push_str("  </section>\n");

    let mut memory_provider = InMemoryDocumentPersistence::new(persisted_source);
    let mut memory_state = DocumentLifecycleState::open_clean(
        persisted_source,
        LifecycleCapabilities::all_supported(),
    );
    memory_state.edit_working_source(working_source);
    save_lifecycle_to_persistence(&mut memory_state, &mut memory_provider)
        .expect("in-memory persistence provider supports save");

    output.push_str("  <section role=\"persistence-proof\" data-provider=\"in-memory\" data-filesystem=\"false\" data-dirty-after-save=\"");
    output.push_str(if memory_state.is_dirty() {
        "true"
    } else {
        "false"
    });
    output.push_str("\">\n");
    output.push_str("    <div role=\"persistence-note\">In-memory provider only; no filesystem persistence claimed.</div>\n");
    output.push_str("    <div role=\"persistence-state\">saved clean</div>\n");
    output.push_str("  </section>\n");

    let snapshot = GuiSessionSnapshot::capture(
        workspace_path.display().to_string(),
        active_module,
        &browser_state,
        SessionCapabilityProfile::BrowserLimited,
    );
    let restored = snapshot.restore();
    output.push_str("  <section role=\"session-restore\" data-profile=\"");
    output.push_str(restored.capability_profile.label());
    output.push_str("\" data-dirty=\"");
    output.push_str(if restored.document.is_dirty() {
        "true"
    } else {
        "false"
    });
    output.push_str("\">\n");
    output.push_str("    <div role=\"restored-workspace\">");
    output.push_str(&html_escape(&restored.workspace_path));
    output.push_str("</div>\n");
    output.push_str("    <div role=\"restored-module\">");
    output.push_str(&html_escape(&restored.active_module));
    output.push_str("</div>\n");
    output.push_str("    <pre role=\"restored-working-source\">");
    output.push_str(&html_escape(restored.document.working_source()));
    output.push_str("</pre>\n");
    output.push_str("  </section>\n");
}

fn render_lifecycle_command(
    output: &mut String,
    command: LifecycleCommand,
    status: LifecycleCommandStatus,
) {
    output.push_str("    <div role=\"lifecycle-command\" data-command=\"");
    output.push_str(lifecycle_command_name(command));
    output.push_str("\" data-enabled=\"");
    output.push_str(if status.is_enabled { "true" } else { "false" });
    output.push_str("\">");
    output.push_str(lifecycle_command_label(command));
    output.push_str(if status.is_enabled {
        " enabled"
    } else {
        " disabled"
    });
    if let Some(reason) = status.reason {
        output.push_str(": ");
        output.push_str(&html_escape(&reason));
    }
    output.push_str("</div>\n");
}

fn lifecycle_command_name(command: LifecycleCommand) -> &'static str {
    match command {
        LifecycleCommand::Save => "save",
        LifecycleCommand::Reload => "reload",
        LifecycleCommand::Revert => "revert",
    }
}

fn lifecycle_command_label(command: LifecycleCommand) -> &'static str {
    match command {
        LifecycleCommand::Save => "Save",
        LifecycleCommand::Reload => "Reload",
        LifecycleCommand::Revert => "Revert",
    }
}

fn render_diagnostic_row(output: &mut String, diagnostic: &DiagnosticRow) {
    output.push_str("    <div role=\"diagnostic-row\" data-severity=\"");
    output.push_str(&html_escape(&diagnostic.severity_label));
    output.push_str("\" data-module=\"");
    output.push_str(&html_escape(&diagnostic.module_display_name));
    output.push_str("\" data-start=\"");
    output.push_str(&diagnostic.span_start.to_string());
    output.push_str("\" data-end=\"");
    output.push_str(&diagnostic.span_end.to_string());
    output.push_str("\">");
    output.push_str(&html_escape(&diagnostic.message));
    output.push_str(" <span role=\"diagnostic-provenance\">");
    output.push_str(&html_escape(&diagnostic.provenance_label));
    output.push_str("</span></div>\n");
}

fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuiLabError {
    DuplicateScenarioId { id: String },
    UnknownScenario { id: String },
    Usage { message: String },
    ProjectOpen(ProjectOpenSpineError),
    Diagnostics(EditedDocumentDiagnosticsError),
}

impl std::fmt::Display for GuiLabError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateScenarioId { id } => write!(f, "duplicate GUI scenario id {id}"),
            Self::UnknownScenario { id } => write!(f, "unknown GUI scenario id {id}"),
            Self::Usage { message } => write!(f, "{message}"),
            Self::ProjectOpen(source) => write!(f, "project-open scenario failed: {source}"),
            Self::Diagnostics(source) => write!(f, "diagnostics scenario failed: {source}"),
        }
    }
}

impl std::error::Error for GuiLabError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
    }

    #[test]
    fn guilab_role_consumes_domain_vocabulary() {
        assert_eq!(
            OxideGuiLabRole::ScenarioReviewSurface.consumes_domain_vocabulary(),
            OxideDomainRole::HostIndependentIdeVocabulary
        );
    }

    #[test]
    fn built_in_registry_finds_thin_slice_scenario_by_id() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let scenario = registry
            .find(GUI_THIN_SLICE_LOADED)
            .expect("thin-slice scenario");
        assert_eq!(scenario.title, "Thin-slice project loaded");
    }

    #[test]
    fn duplicate_scenario_ids_are_rejected_by_name() {
        let duplicate = GuiScenarioDescriptor {
            id: GUI_THIN_SLICE_LOADED,
            title: "duplicate",
            fixture_path: PathBuf::from("unused.basproj"),
            kind: GuiScenarioKind::ReadOnlyProject,
        };

        let error = GuiScenarioRegistry::new(vec![duplicate.clone(), duplicate])
            .expect_err("duplicate IDs must fail");

        assert_eq!(
            error,
            GuiLabError::DuplicateScenarioId {
                id: GUI_THIN_SLICE_LOADED.to_string()
            }
        );
    }

    #[test]
    fn cli_list_names_thin_slice_scenario() {
        let output = run_cli(vec![String::from("list")], repo_root()).expect("list scenarios");

        assert!(output.contains("gui-thin-slice-loaded"));
        assert!(output.contains("Thin-slice project loaded"));
        assert!(output.contains("gui-thin-slice-edited-diagnostics"));
        assert!(output.contains("Thin-slice edited diagnostics"));
        assert!(output.contains("gui-thin-slice-lifecycle"));
        assert!(output.contains("Thin-slice lifecycle state"));
        assert!(output.contains("gui-run-output-browser-disabled"));
        assert!(output.contains("Run output browser disabled"));
        assert!(output.contains("gui-run-output-simulated-supported"));
        assert!(output.contains("Run output simulated supported"));
        assert!(output.contains("gui-dnaonecalc-embedding-contract"));
        assert!(output.contains("DnaOneCalc embedding contract"));
        assert!(output.contains("gui-com-reference-browser-unavailable"));
        assert!(output.contains("COM reference browser unavailable"));
        assert!(output.contains("gui-com-reference-nonwindows-unavailable"));
        assert!(output.contains("COM reference non-Windows unavailable"));
        assert!(output.contains("gui-com-reference-native-service-missing"));
        assert!(output.contains("COM reference native service missing"));
        assert!(output.contains("gui-run-timeline-simulated"));
        assert!(output.contains("Run timeline simulated"));
        assert!(output.contains("gui-immediate-browser-disabled"));
        assert!(output.contains("Immediate browser disabled"));
        assert!(output.contains("gui-debug-browser-disabled"));
        assert!(output.contains("Debug browser disabled"));
        assert!(output.contains("gui-command-palette-baseline"));
        assert!(output.contains("Command palette baseline"));
        assert!(output.contains("gui-keyboard-contexts-baseline"));
        assert!(output.contains("Keyboard contexts baseline"));
        assert!(output.contains("gui-focus-graph-no-mouse"));
        assert!(output.contains("Focus graph no-mouse"));
        assert!(output.contains("gui-accessibility-disabled-reasons"));
        assert!(output.contains("Accessibility disabled reasons"));
        assert!(output.contains("gui-shell-packet-baseline"));
        assert!(output.contains("Shell packet baseline"));
        assert!(output.contains("gui-mounted-shell-static"));
        assert!(output.contains("Mounted shell static"));
        assert!(output.contains("gui-mounted-command-palette"));
        assert!(output.contains("Mounted command palette"));
        assert!(output.contains("gui-mounted-no-mouse-accessibility"));
        assert!(output.contains("Mounted no-mouse accessibility"));
        assert!(output.contains("gui-web-shell-adapter-boundary"));
        assert!(output.contains("Web shell adapter boundary"));
        assert!(output.contains("gui-web-shell-dom-smoke"));
        assert!(output.contains("Web shell DOM smoke"));
        assert!(output.contains("gui-web-command-palette-dom-smoke"));
        assert!(output.contains("Web command palette DOM smoke"));
        assert!(output.contains("gui-web-no-mouse-accessibility-dom-smoke"));
        assert!(output.contains("Web no-mouse accessibility DOM smoke"));
        assert!(output.contains("gui-dnaonecalc-web-shell-host-contract"));
        assert!(output.contains("DnaOneCalc web shell host contract"));
        assert!(output.contains("gui-dnaonecalc-web-shell-dom-readiness"));
        assert!(output.contains("DnaOneCalc web shell DOM readiness"));
        assert!(output.contains("gui-native-save-reload-disk"));
        assert!(output.contains("Native save reload disk"));
        assert!(output.contains("gui-native-session-restore-disk"));
        assert!(output.contains("Native session restore disk"));
        assert!(output.contains("gui-browser-filesystem-still-disabled"));
        assert!(output.contains("Browser filesystem still disabled"));
        assert!(output.contains("gui-runtime-service-contract-browser-disabled"));
        assert!(output.contains("Runtime service contract browser disabled"));
        assert!(output.contains("gui-runtime-service-contract-native-missing"));
        assert!(output.contains("Runtime service contract native missing"));
        assert!(output.contains("gui-immediate-service-contract-native-missing"));
        assert!(output.contains("Immediate service contract native missing"));
        assert!(output.contains("gui-debug-service-contract-native-missing"));
        assert!(output.contains("Debug service contract native missing"));
    }

    #[test]
    fn cli_unknown_scenario_reports_named_error() {
        let error = run_cli(
            vec![String::from("render"), String::from("missing-scenario")],
            repo_root(),
        )
        .expect_err("unknown scenario should fail");

        assert_eq!(
            error,
            GuiLabError::UnknownScenario {
                id: String::from("missing-scenario")
            }
        );
    }

    #[test]
    fn thin_slice_scenario_renders_project_module_source_and_capability() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_THIN_SLICE_LOADED)
            .expect("render thin-slice scenario");

        assert!(rendered.contains("data-scenario=\"gui-thin-slice-loaded\""));
        assert!(rendered.contains("ThinSliceHello"));
        assert!(rendered.contains("Module1.bas"));
        assert!(rendered.contains("Public Sub Main()"));
        assert!(rendered.contains("COM unavailable"));
        assert!(!rendered.contains("'Option Explicit"));
        assert!(!rendered.contains("role=\"document-lifecycle\""));
    }

    #[test]
    fn built_in_registry_finds_edited_diagnostics_scenario_by_id() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let scenario = registry
            .find(GUI_THIN_SLICE_EDITED_DIAGNOSTICS)
            .expect("edited diagnostics scenario");

        assert_eq!(scenario.title, "Thin-slice edited diagnostics");
        assert_eq!(scenario.kind, GuiScenarioKind::EditedDiagnostics);
    }

    #[test]
    fn edited_diagnostics_scenario_renders_deterministic_edit_token() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_THIN_SLICE_EDITED_DIAGNOSTICS)
            .expect("render edited diagnostics scenario");

        assert!(rendered.contains("data-scenario=\"gui-thin-slice-edited-diagnostics\""));
        assert!(rendered.contains("ThinSliceHello"));
        assert!(rendered.contains("Module1.bas"));
        assert!(rendered.contains("Option Explicit"));
        assert!(!rendered.contains("Dim answer"));
        assert!(rendered.contains("answer = 40 + 2"));
        assert!(rendered.contains("Public Sub Main()"));
        assert!(rendered.contains("COM unavailable"));
        assert!(rendered.contains("role=\"diagnostics\""));
        assert!(rendered.contains("role=\"diagnostic-row\""));
        assert!(rendered.contains("data-severity=\"error\""));
        assert!(rendered.contains("undeclared variable"));
        assert!(rendered.contains("answer"));
        assert!(rendered.contains("OxVba language service"));
    }

    #[test]
    fn built_in_registry_finds_lifecycle_scenario_by_id() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let scenario = registry
            .find(GUI_THIN_SLICE_LIFECYCLE)
            .expect("lifecycle scenario");

        assert_eq!(scenario.title, "Thin-slice lifecycle state");
        assert_eq!(scenario.kind, GuiScenarioKind::Lifecycle);
    }

    #[test]
    fn built_in_registry_finds_browser_run_disabled_scenario_by_id() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let scenario = registry
            .find(GUI_RUN_OUTPUT_BROWSER_DISABLED)
            .expect("browser run disabled scenario");

        assert_eq!(scenario.title, "Run output browser disabled");
        assert_eq!(scenario.kind, GuiScenarioKind::BrowserRunDisabled);
    }

    #[test]
    fn built_in_registry_finds_simulated_run_output_scenario_by_id() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let scenario = registry
            .find(GUI_RUN_OUTPUT_SIMULATED_SUPPORTED)
            .expect("simulated run output scenario");

        assert_eq!(scenario.title, "Run output simulated supported");
        assert_eq!(scenario.kind, GuiScenarioKind::SimulatedRunOutput);
    }

    #[test]
    fn built_in_registry_finds_dnaonecalc_embedding_contract_scenario_by_id() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let scenario = registry
            .find(GUI_DNAONECALC_EMBEDDING_CONTRACT)
            .expect("DnaOneCalc embedding contract scenario");

        assert_eq!(scenario.title, "DnaOneCalc embedding contract");
        assert_eq!(scenario.kind, GuiScenarioKind::DnaOneCalcEmbeddingContract);
    }

    #[test]
    fn built_in_registry_finds_com_unavailable_scenarios_by_id() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let browser = registry
            .find(GUI_COM_REFERENCE_BROWSER_UNAVAILABLE)
            .expect("browser COM unavailable scenario");
        let nonwindows = registry
            .find(GUI_COM_REFERENCE_NONWINDOWS_UNAVAILABLE)
            .expect("non-Windows COM unavailable scenario");
        let native_missing = registry
            .find(GUI_COM_REFERENCE_NATIVE_SERVICE_MISSING)
            .expect("native COM service missing scenario");

        assert_eq!(browser.title, "COM reference browser unavailable");
        assert_eq!(browser.kind, GuiScenarioKind::BrowserComUnavailable);
        assert_eq!(nonwindows.title, "COM reference non-Windows unavailable");
        assert_eq!(nonwindows.kind, GuiScenarioKind::NonWindowsComUnavailable);
        assert_eq!(native_missing.title, "COM reference native service missing");
        assert_eq!(
            native_missing.kind,
            GuiScenarioKind::NativeComServiceMissing
        );
    }

    #[test]
    fn built_in_registry_finds_w270_runtime_surface_scenarios_by_id() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let run_timeline = registry
            .find(GUI_RUN_TIMELINE_SIMULATED)
            .expect("run timeline scenario");
        let immediate = registry
            .find(GUI_IMMEDIATE_BROWSER_DISABLED)
            .expect("Immediate browser disabled scenario");
        let debug = registry
            .find(GUI_DEBUG_BROWSER_DISABLED)
            .expect("debug browser disabled scenario");

        assert_eq!(run_timeline.title, "Run timeline simulated");
        assert_eq!(run_timeline.kind, GuiScenarioKind::RunTimelineSimulated);
        assert_eq!(immediate.title, "Immediate browser disabled");
        assert_eq!(immediate.kind, GuiScenarioKind::ImmediateBrowserDisabled);
        assert_eq!(debug.title, "Debug browser disabled");
        assert_eq!(debug.kind, GuiScenarioKind::DebugBrowserDisabled);
    }

    #[test]
    fn built_in_registry_finds_w280_command_palette_scenarios_by_id() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let command_palette = registry
            .find(GUI_COMMAND_PALETTE_BASELINE)
            .expect("command palette baseline scenario");
        let keyboard_contexts = registry
            .find(GUI_KEYBOARD_CONTEXTS_BASELINE)
            .expect("keyboard contexts baseline scenario");
        let focus_graph = registry
            .find(GUI_FOCUS_GRAPH_NO_MOUSE)
            .expect("focus graph no-mouse scenario");
        let accessibility = registry
            .find(GUI_ACCESSIBILITY_DISABLED_REASONS)
            .expect("accessibility disabled reasons scenario");
        let shell_packet = registry
            .find(GUI_SHELL_PACKET_BASELINE)
            .expect("shell packet baseline scenario");
        let mounted_shell = registry
            .find(GUI_MOUNTED_SHELL_STATIC)
            .expect("mounted shell static scenario");
        let mounted_command_palette = registry
            .find(GUI_MOUNTED_COMMAND_PALETTE)
            .expect("mounted command palette scenario");
        let mounted_no_mouse_accessibility = registry
            .find(GUI_MOUNTED_NO_MOUSE_ACCESSIBILITY)
            .expect("mounted no-mouse accessibility scenario");
        let web_shell_adapter_boundary = registry
            .find(GUI_WEB_SHELL_ADAPTER_BOUNDARY)
            .expect("web shell adapter boundary scenario");
        let web_shell_dom_smoke = registry
            .find(GUI_WEB_SHELL_DOM_SMOKE)
            .expect("web shell DOM smoke scenario");
        let web_command_palette_dom_smoke = registry
            .find(GUI_WEB_COMMAND_PALETTE_DOM_SMOKE)
            .expect("web command palette DOM smoke scenario");
        let web_no_mouse_accessibility_dom_smoke = registry
            .find(GUI_WEB_NO_MOUSE_ACCESSIBILITY_DOM_SMOKE)
            .expect("web no-mouse accessibility DOM smoke scenario");
        let dnaonecalc_web_shell_host_contract = registry
            .find(GUI_DNAONECALC_WEB_SHELL_HOST_CONTRACT)
            .expect("DnaOneCalc web shell host contract scenario");
        let dnaonecalc_web_shell_dom_readiness = registry
            .find(GUI_DNAONECALC_WEB_SHELL_DOM_READINESS)
            .expect("DnaOneCalc web shell DOM readiness scenario");

        assert_eq!(command_palette.title, "Command palette baseline");
        assert_eq!(
            command_palette.kind,
            GuiScenarioKind::CommandPaletteBaseline
        );
        assert_eq!(keyboard_contexts.title, "Keyboard contexts baseline");
        assert_eq!(
            keyboard_contexts.kind,
            GuiScenarioKind::KeyboardContextsBaseline
        );
        assert_eq!(focus_graph.title, "Focus graph no-mouse");
        assert_eq!(focus_graph.kind, GuiScenarioKind::FocusGraphNoMouse);
        assert_eq!(accessibility.title, "Accessibility disabled reasons");
        assert_eq!(
            accessibility.kind,
            GuiScenarioKind::AccessibilityDisabledReasons
        );
        assert_eq!(shell_packet.title, "Shell packet baseline");
        assert_eq!(shell_packet.kind, GuiScenarioKind::ShellPacketBaseline);
        assert_eq!(mounted_shell.title, "Mounted shell static");
        assert_eq!(mounted_shell.kind, GuiScenarioKind::MountedShellStatic);
        assert_eq!(mounted_command_palette.title, "Mounted command palette");
        assert_eq!(
            mounted_command_palette.kind,
            GuiScenarioKind::MountedCommandPalette
        );
        assert_eq!(
            mounted_no_mouse_accessibility.title,
            "Mounted no-mouse accessibility"
        );
        assert_eq!(
            mounted_no_mouse_accessibility.kind,
            GuiScenarioKind::MountedNoMouseAccessibility
        );
        assert_eq!(
            web_shell_adapter_boundary.title,
            "Web shell adapter boundary"
        );
        assert_eq!(
            web_shell_adapter_boundary.kind,
            GuiScenarioKind::WebShellAdapterBoundary
        );
        assert_eq!(web_shell_dom_smoke.title, "Web shell DOM smoke");
        assert_eq!(web_shell_dom_smoke.kind, GuiScenarioKind::WebShellDomSmoke);
        assert_eq!(
            web_command_palette_dom_smoke.title,
            "Web command palette DOM smoke"
        );
        assert_eq!(
            web_command_palette_dom_smoke.kind,
            GuiScenarioKind::WebCommandPaletteDomSmoke
        );
        assert_eq!(
            web_no_mouse_accessibility_dom_smoke.title,
            "Web no-mouse accessibility DOM smoke"
        );
        assert_eq!(
            web_no_mouse_accessibility_dom_smoke.kind,
            GuiScenarioKind::WebNoMouseAccessibilityDomSmoke
        );
        assert_eq!(
            dnaonecalc_web_shell_host_contract.title,
            "DnaOneCalc web shell host contract"
        );
        assert_eq!(
            dnaonecalc_web_shell_host_contract.kind,
            GuiScenarioKind::DnaOneCalcWebShellHostContract
        );
        assert_eq!(
            dnaonecalc_web_shell_dom_readiness.title,
            "DnaOneCalc web shell DOM readiness"
        );
        assert_eq!(
            dnaonecalc_web_shell_dom_readiness.kind,
            GuiScenarioKind::DnaOneCalcWebShellDomReadiness
        );
    }

    #[test]
    fn built_in_registry_finds_w320_persistence_scenarios_by_id() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let native_save_reload = registry
            .find(GUI_NATIVE_SAVE_RELOAD_DISK)
            .expect("native save/reload disk scenario");
        let native_session_restore = registry
            .find(GUI_NATIVE_SESSION_RESTORE_DISK)
            .expect("native session restore disk scenario");
        let browser_disabled = registry
            .find(GUI_BROWSER_FILESYSTEM_STILL_DISABLED)
            .expect("browser filesystem disabled scenario");

        assert_eq!(native_save_reload.title, "Native save reload disk");
        assert_eq!(
            native_save_reload.kind,
            GuiScenarioKind::NativeSaveReloadDisk
        );
        assert_eq!(native_session_restore.title, "Native session restore disk");
        assert_eq!(
            native_session_restore.kind,
            GuiScenarioKind::NativeSessionRestoreDisk
        );
        assert_eq!(browser_disabled.title, "Browser filesystem still disabled");
        assert_eq!(
            browser_disabled.kind,
            GuiScenarioKind::BrowserFilesystemStillDisabled
        );
    }

    #[test]
    fn built_in_registry_finds_w330_runtime_service_scenarios_by_id() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let runtime_browser = registry
            .find(GUI_RUNTIME_SERVICE_CONTRACT_BROWSER_DISABLED)
            .expect("runtime browser disabled service scenario");
        let runtime_native_missing = registry
            .find(GUI_RUNTIME_SERVICE_CONTRACT_NATIVE_MISSING)
            .expect("runtime native missing service scenario");
        let immediate_native_missing = registry
            .find(GUI_IMMEDIATE_SERVICE_CONTRACT_NATIVE_MISSING)
            .expect("Immediate native missing service scenario");
        let debug_native_missing = registry
            .find(GUI_DEBUG_SERVICE_CONTRACT_NATIVE_MISSING)
            .expect("debug native missing service scenario");

        assert_eq!(
            runtime_browser.kind,
            GuiScenarioKind::RuntimeServiceContractBrowserDisabled
        );
        assert_eq!(
            runtime_native_missing.kind,
            GuiScenarioKind::RuntimeServiceContractNativeMissing
        );
        assert_eq!(
            immediate_native_missing.kind,
            GuiScenarioKind::ImmediateServiceContractNativeMissing
        );
        assert_eq!(
            debug_native_missing.kind,
            GuiScenarioKind::DebugServiceContractNativeMissing
        );
    }

    #[test]
    fn built_in_registry_finds_w342_shared_ui_component_scenario_by_id() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let shared_ui = registry
            .find(GUI_SHARED_UI_SHELL_COMPONENT)
            .expect("shared UI shell component scenario");

        assert_eq!(shared_ui.kind, GuiScenarioKind::SharedUiShellComponent);
    }

    #[test]
    fn built_in_registry_finds_w343_host_bridge_command_dispatch_scenario_by_id() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let command_dispatch = registry
            .find(GUI_HOST_BRIDGE_COMMAND_DISPATCH)
            .expect("host bridge command dispatch scenario");

        assert_eq!(
            command_dispatch.kind,
            GuiScenarioKind::HostBridgeCommandDispatch
        );
    }

    #[test]
    fn shared_ui_shell_component_scenario_renders_shared_component_markup() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_SHARED_UI_SHELL_COMPONENT)
            .expect("render shared UI shell component scenario");

        assert!(rendered.contains("data-scenario=\"gui-shared-ui-shell-component\""));
        assert!(rendered.contains("role=\"shared-ui-component-route\""));
        assert!(rendered.contains("data-source=\"oxide-ui-leptos\""));
        assert!(rendered.contains("role=\"shared-ide-surface\""));
        assert!(rendered.contains("data-component-crate=\"oxide-ui-leptos\""));
        assert!(rendered.contains("data-provenance=\"pending-oxvba-hardening\""));
        assert!(rendered.contains("role=\"shared-project-spine\""));
        assert!(rendered.contains("role=\"shared-editor-boundary\""));
        assert!(rendered.contains("role=\"shared-diagnostics-summary\""));
        assert!(rendered.contains("role=\"shared-lifecycle-summary\""));
        assert!(rendered.contains("role=\"shared-run-output\""));
        assert!(rendered.contains("role=\"shared-command-palette\""));
        assert!(rendered.contains("role=\"shared-focus-accessibility\""));
        assert!(rendered.contains("role=\"shared-runtime-service\""));
        assert!(rendered.contains("role=\"shared-immediate-service\""));
        assert!(rendered.contains("role=\"shared-debug-service\""));
        assert!(rendered.contains("role=\"shared-com-capability\""));
        assert!(rendered.contains("ThinSliceHello"));
        assert!(rendered.contains("Module1.bas"));
        assert!(rendered.contains("Public Sub Main()"));
        assert!(rendered.contains("data-native-runtime=\"false\""));
        assert!(rendered.contains("data-com-runtime=\"false\""));
        assert!(rendered.contains("data-fake-responses=\"false\""));
        assert!(rendered.contains("data-fake-debug-data=\"false\""));
        assert!(rendered.contains("no live Tauri/WebView, real runtime, fake Immediate response, fake debug data, or COM runtime is claimed"));
    }

    #[test]
    fn host_bridge_command_dispatch_scenario_renders_command_availability() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_HOST_BRIDGE_COMMAND_DISPATCH)
            .expect("render host bridge command dispatch scenario");

        assert!(rendered.contains("data-scenario=\"gui-host-bridge-command-dispatch\""));
        assert!(rendered.contains("role=\"host-bridge-command-dispatch-route\""));
        assert!(rendered.contains("data-source=\"oxide-host-bridge+oxide-ui-leptos\""));
        assert!(rendered.contains("role=\"shared-command-dispatch\""));
        assert!(rendered.contains("data-source=\"HostBridgeCommandAvailability\""));
        assert!(rendered.contains("data-command-count=\"26\""));
        assert!(rendered.contains("data-dnaonecalc-reusable=\"true\""));
        assert!(rendered.contains("data-command-id=\"project.open\""));
        assert!(rendered.contains("data-category=\"HostProjectApi\""));
        assert!(rendered.contains("data-enabled=\"true\""));
        assert!(rendered.contains("data-command-id=\"runtime.run\""));
        assert!(rendered.contains("data-state=\"oxvba-fixture-evidenced\""));
        assert!(
            rendered.contains("ThinSliceHello fixture covers EmbeddedBuildRunHost::run_project")
        );
        assert!(rendered.contains("data-command-id=\"runtime.immediate\""));
        assert!(rendered.contains("EmbeddedRunSession::into_immediate_session"));
        assert!(rendered.contains("data-command-id=\"watch.upsert\""));
        assert!(rendered.contains("watch registry/evaluation"));
        assert!(rendered.contains("data-command-id=\"references.com.search\""));
        assert!(rendered.contains("ComSelectionService reference state and capability_profile"));
        assert!(rendered.contains("data-native-runtime=\"false\""));
        assert!(rendered.contains("data-com-runtime=\"false\""));
        assert!(rendered.contains("data-fake-responses=\"false\""));
        assert!(rendered.contains("data-fake-debug-data=\"false\""));
        assert!(rendered.contains("not real DnaOxIde runtime/debug/Immediate/COM claims"));
    }

    #[test]
    fn browser_run_disabled_scenario_renders_structured_output_reason() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_RUN_OUTPUT_BROWSER_DISABLED)
            .expect("render browser run disabled scenario");

        assert!(rendered.contains("data-scenario=\"gui-run-output-browser-disabled\""));
        assert!(rendered.contains("ThinSliceHello"));
        assert!(rendered.contains("Module1.bas"));
        assert!(rendered.contains("role=\"run-output\""));
        assert!(rendered.contains("data-provider=\"browser-unsupported\""));
        assert!(rendered.contains("data-status=\"disabled\""));
        assert!(rendered.contains("role=\"run-request\""));
        assert!(rendered.contains("ThinSliceHello::Module1.Main"));
        assert!(rendered.contains("role=\"run-command\" data-enabled=\"false\""));
        assert!(rendered.contains("native execution provider unavailable"));
        assert!(rendered.contains("role=\"output-activity\""));
        assert!(rendered.contains("data-event-kind=\"lifecycle\""));
        assert!(rendered.contains("run requested"));
        assert!(rendered.contains("data-event-kind=\"diagnostic\""));
        assert!(rendered.contains("Run disabled"));
        assert!(rendered.contains("COM unavailable"));
        assert!(!rendered.contains("role=\"diagnostics\""));
    }

    #[test]
    fn simulated_run_output_scenario_renders_completed_structured_events() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_RUN_OUTPUT_SIMULATED_SUPPORTED)
            .expect("render simulated run output scenario");

        assert!(rendered.contains("data-scenario=\"gui-run-output-simulated-supported\""));
        assert!(rendered.contains("ThinSliceHello"));
        assert!(rendered.contains("Module1.bas"));
        assert!(rendered.contains("role=\"run-output\""));
        assert!(rendered.contains("data-provider=\"simulated\""));
        assert!(rendered.contains("data-status=\"completed\""));
        assert!(rendered.contains("data-native-execution=\"false\""));
        assert!(rendered.contains("data-com-runtime=\"false\""));
        assert!(rendered.contains("role=\"run-command\" data-enabled=\"true\""));
        assert!(rendered.contains("Run enabled by simulated provider"));
        assert!(rendered.contains("run started"));
        assert!(rendered.contains("simulated provider invoked ThinSliceHello::Module1.Main"));
        assert!(rendered.contains("simulated output: Main completed with answer 42"));
        assert!(rendered.contains("run completed"));
        assert!(rendered.contains("COM unavailable"));
    }

    #[test]
    fn dnaonecalc_embedding_contract_scenario_renders_host_boundary() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_DNAONECALC_EMBEDDING_CONTRACT)
            .expect("render DnaOneCalc embedding contract scenario");

        assert!(rendered.contains("data-scenario=\"gui-dnaonecalc-embedding-contract\""));
        assert!(rendered.contains("ThinSliceHello"));
        assert!(rendered.contains("Module1.bas"));
        assert!(rendered.contains("role=\"embedded-host-contract\" data-host=\"DnaOneCalc\""));
        assert!(rendered.contains("data-sibling-repo-writes=\"false\""));
        assert!(rendered.contains("DnaOneCalc owns product shell and host policy"));
        assert!(rendered.contains("data-slot=\"project-spine\""));
        assert!(rendered.contains("data-slot=\"source-editor\""));
        assert!(rendered.contains("data-slot=\"diagnostics\""));
        assert!(rendered.contains("data-slot=\"document-lifecycle\""));
        assert!(rendered.contains("data-slot=\"run-output\""));
        assert!(rendered.contains("data-slot=\"capability-footer\""));
        assert!(rendered.contains("role=\"ownership-boundary\" data-owner=\"DnaOneCalc\""));
        assert!(rendered.contains("role=\"ownership-boundary\" data-owner=\"OxIde\""));
        assert!(rendered.contains("role=\"ownership-boundary\" data-owner=\"OxVba\""));
        assert!(rendered.contains("role=\"embedded-run-capability\""));
        assert!(rendered.contains("data-provider=\"browser-unsupported\""));
        assert!(rendered.contains("data-status=\"disabled\""));
        assert!(rendered.contains("data-native-execution=\"false\""));
        assert!(rendered.contains("data-com-runtime=\"false\""));
        assert!(rendered.contains("ThinSliceHello::Module1.Main"));
        assert!(rendered.contains("native execution provider unavailable"));
        assert!(rendered.contains("did not modify DnaOneCalc files"));
        assert!(rendered.contains("COM unavailable"));
    }

    #[test]
    fn browser_com_unavailable_scenario_renders_reference_and_disabled_reasons() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_COM_REFERENCE_BROWSER_UNAVAILABLE)
            .expect("render browser COM unavailable scenario");

        assert!(rendered.contains("data-scenario=\"gui-com-reference-browser-unavailable\""));
        assert!(rendered.contains("ThinSliceHello"));
        assert!(rendered.contains("Module1.bas"));
        assert!(rendered.contains("role=\"com-capability\" data-profile=\"browser-safe\""));
        assert!(rendered.contains("data-native-execution=\"false\""));
        assert!(rendered.contains("data-com-service-configured=\"false\""));
        assert!(rendered.contains("data-windows-native-host-required=\"true\""));
        assert!(rendered.contains("COM reference present: Scripting.Dictionary"));
        assert!(rendered.contains(
            "role=\"com-status\" data-feature=\"reference-discovery\" data-available=\"false\""
        ));
        assert!(rendered.contains("COM discovery unavailable in browser-safe profile"));
        assert!(rendered.contains(
            "role=\"com-status\" data-feature=\"runtime-invocation\" data-available=\"false\""
        ));
        assert!(rendered.contains("pure browser/WASM cannot directly call Windows COM"));
        assert!(rendered.contains("Windows native host required"));
        assert!(rendered.contains("No COM runtime support is claimed"));
        assert!(rendered.contains("COM unavailable"));
    }

    #[test]
    fn nonwindows_com_unavailable_scenario_keeps_native_execution_distinct() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_COM_REFERENCE_NONWINDOWS_UNAVAILABLE)
            .expect("render non-Windows COM unavailable scenario");

        assert!(rendered.contains("data-scenario=\"gui-com-reference-nonwindows-unavailable\""));
        assert!(rendered.contains("role=\"com-capability\" data-profile=\"non-windows-native\""));
        assert!(rendered.contains("data-native-execution=\"true\""));
        assert!(rendered.contains("data-com-service-configured=\"false\""));
        assert!(rendered.contains("data-windows-native-host-required=\"true\""));
        assert!(rendered.contains("COM reference present: Scripting.Dictionary"));
        assert!(rendered.contains("COM discovery unavailable on non-Windows native host"));
        assert!(rendered.contains("COM runtime unavailable on non-Windows native host"));
        assert!(rendered.contains("Windows native host required"));
        assert!(rendered.contains("No COM runtime support is claimed"));
        assert!(rendered.contains("Non-Windows native profile"));
        assert!(rendered.contains("COM unavailable"));
    }

    #[test]
    fn native_service_missing_scenario_renders_blocked_windows_service_seam() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_COM_REFERENCE_NATIVE_SERVICE_MISSING)
            .expect("render native COM service missing scenario");

        assert!(rendered.contains("data-scenario=\"gui-com-reference-native-service-missing\""));
        assert!(
            rendered.contains(
                "role=\"com-capability\" data-profile=\"windows-native-service-missing\""
            )
        );
        assert!(rendered.contains("data-native-execution=\"true\""));
        assert!(rendered.contains("data-com-service-configured=\"false\""));
        assert!(rendered.contains("data-windows-native-host-required=\"false\""));
        assert!(rendered.contains("COM reference present: Scripting.Dictionary"));
        assert!(rendered.contains("native COM service not configured"));
        assert!(rendered.contains("COM discovery blocked until service handoff is implemented"));
        assert!(rendered.contains("COM runtime invocation disabled"));
        assert!(rendered.contains("No COM runtime support is claimed"));
        assert!(rendered.contains("Windows native profile"));
        assert!(rendered.contains("COM runtime disabled"));
        assert!(!rendered.contains("pure browser/WASM cannot directly call Windows COM"));
    }

    #[test]
    fn run_timeline_simulated_scenario_renders_ordered_timeline() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_RUN_TIMELINE_SIMULATED)
            .expect("render run timeline scenario");

        assert!(rendered.contains("data-scenario=\"gui-run-timeline-simulated\""));
        assert!(rendered.contains("role=\"run-timeline\" data-provider=\"simulated\""));
        assert!(rendered.contains("data-status=\"completed\""));
        assert!(rendered.contains("data-native-execution=\"false\""));
        assert!(rendered.contains("data-com-runtime=\"false\""));
        assert!(rendered.contains("role=\"run-timeline-target\">ThinSliceHello::Module1.Main"));
        assert!(rendered.contains(
            "data-index=\"1\" data-event-kind=\"lifecycle\" data-provenance=\"simulated\""
        ));
        assert!(rendered.contains("run started"));
        assert!(rendered.contains("data-index=\"2\" data-event-kind=\"activity\""));
        assert!(rendered.contains("simulated provider invoked ThinSliceHello::Module1.Main"));
        assert!(rendered.contains("data-index=\"3\" data-event-kind=\"output\""));
        assert!(rendered.contains("simulated output: Main completed with answer 42"));
        assert!(rendered.contains("data-index=\"4\" data-event-kind=\"lifecycle\""));
        assert!(rendered.contains("run completed"));
    }

    #[test]
    fn immediate_browser_disabled_scenario_renders_disabled_panel_without_responses() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_IMMEDIATE_BROWSER_DISABLED)
            .expect("render Immediate browser disabled scenario");

        assert!(rendered.contains("data-scenario=\"gui-immediate-browser-disabled\""));
        assert!(rendered.contains("role=\"immediate-panel\" data-profile=\"browser-disabled\""));
        assert!(rendered.contains("data-enabled=\"false\""));
        assert!(rendered.contains("data-native-runtime-required=\"true\""));
        assert!(rendered.contains("data-com-runtime-required=\"false\""));
        assert!(rendered.contains("data-fake-responses=\"false\""));
        assert!(rendered.contains(
            "Immediate disabled: browser-safe profile has no native OxVba runtime session"
        ));
        assert!(rendered.contains("No Immediate responses rendered without runtime session"));
        assert!(rendered.contains("COM unavailable"));
    }

    #[test]
    fn debug_browser_disabled_scenario_renders_disabled_panel_without_fake_debug_data() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_DEBUG_BROWSER_DISABLED)
            .expect("render debug browser disabled scenario");

        assert!(rendered.contains("data-scenario=\"gui-debug-browser-disabled\""));
        assert!(rendered.contains("role=\"debug-panel\" data-profile=\"browser-disabled\""));
        assert!(rendered.contains("data-enabled=\"false\""));
        assert!(rendered.contains("data-native-runtime-required=\"true\""));
        assert!(rendered.contains("data-com-runtime-required=\"false\""));
        assert!(rendered.contains("data-fake-debug-data=\"false\""));
        assert!(
            rendered.contains("Debug disabled: browser-safe profile has no OxVba debug session")
        );
        assert!(rendered.contains("role=\"debug-callstack\""));
        assert!(rendered.contains("role=\"debug-locals\""));
        assert!(rendered.contains("role=\"debug-watches\""));
        assert!(rendered.contains("role=\"debug-breakpoints\""));
        assert!(rendered.contains("unavailable; no fake debug data"));
    }

    #[test]
    fn command_palette_baseline_scenario_renders_command_ids_and_disabled_reasons() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_COMMAND_PALETTE_BASELINE)
            .expect("render command palette baseline scenario");

        assert!(rendered.contains("data-scenario=\"gui-command-palette-baseline\""));
        assert!(rendered.contains("role=\"command-palette\""));
        assert!(rendered.contains("data-source=\"gui-core command registry\""));
        assert!(rendered.contains("data-parked-tui-imported=\"false\""));
        assert!(rendered.contains("data-command-count=\"10\""));
        assert!(rendered.contains("data-command-id=\"project.open\""));
        assert!(rendered.contains("data-command-id=\"document.save\""));
        assert!(rendered.contains("browser-safe profile has no direct filesystem persistence"));
        assert!(rendered.contains("data-command-id=\"runtime.run\""));
        assert!(rendered.contains("data-capability=\"browser-unsupported\""));
        assert!(rendered.contains("native execution provider unavailable"));
        assert!(rendered.contains("data-command-id=\"runtime.stop\""));
        assert!(rendered.contains("no active runtime session to stop"));
        assert!(rendered.contains("data-command-id=\"runtime.immediate\""));
        assert!(rendered.contains("no native OxVba runtime session"));
        assert!(rendered.contains("data-command-id=\"runtime.debug\""));
        assert!(rendered.contains("no OxVba debug session"));
        assert!(rendered.contains("data-command-id=\"capability.show_com\""));
        assert!(rendered.contains("data-command-id=\"shell.command_palette\""));
        assert!(
            rendered.contains("GUI-native command registry; parked TUI command model not imported")
        );
        assert!(rendered.contains("COM unavailable"));
    }

    #[test]
    fn keyboard_contexts_baseline_scenario_renders_contexts_bindings_and_disabled_reasons() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_KEYBOARD_CONTEXTS_BASELINE)
            .expect("render keyboard contexts baseline scenario");

        assert!(rendered.contains("data-scenario=\"gui-keyboard-contexts-baseline\""));
        assert!(rendered.contains("role=\"keyboard-contexts\""));
        assert!(rendered.contains("data-source=\"gui-core keyboard map\""));
        assert!(rendered.contains("data-host-specific-overrides-required=\"false\""));
        assert!(rendered.contains("data-context-collisions=\"0\""));
        assert!(rendered.contains("data-cross-context-collisions=\"0\""));
        assert!(rendered.contains("data-context=\"global-shell\""));
        assert!(rendered.contains("data-context=\"project-tree\""));
        assert!(rendered.contains("data-context=\"editor\""));
        assert!(rendered.contains("data-context=\"diagnostics\""));
        assert!(rendered.contains("data-context=\"run-output\""));
        assert!(rendered.contains("data-context=\"immediate\""));
        assert!(rendered.contains("data-context=\"debug\""));
        assert!(rendered.contains("data-context=\"command-palette\""));
        assert!(
            rendered.contains(
                "data-command-id=\"shell.command_palette\" data-gesture=\"Ctrl+Shift+P\""
            )
        );
        assert!(rendered.contains("data-command-id=\"document.save\" data-gesture=\"Ctrl+S\""));
        assert!(rendered.contains("data-command-id=\"runtime.run\" data-gesture=\"F5\""));
        assert!(rendered.contains("native execution provider unavailable"));
        assert!(rendered.contains("data-command-id=\"runtime.immediate\" data-gesture=\"Enter\""));
        assert!(rendered.contains("data-allow-cross-context=\"true\""));
        assert!(rendered.contains("no native OxVba runtime session"));
        assert!(rendered.contains("no browser-specific key trap is product truth"));
    }

    #[test]
    fn focus_graph_no_mouse_scenario_renders_ordered_focus_route() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_FOCUS_GRAPH_NO_MOUSE)
            .expect("render focus graph no-mouse scenario");

        assert!(rendered.contains("data-scenario=\"gui-focus-graph-no-mouse\""));
        assert!(rendered.contains("role=\"focus-graph\""));
        assert!(rendered.contains("data-source=\"gui-core focus graph\""));
        assert!(rendered.contains("data-node-count=\"9\""));
        assert!(rendered.contains("data-route-length=\"10\""));
        assert!(rendered.contains("data-node-id=\"project-tree\" data-kind=\"project-tree\""));
        assert!(rendered.contains("data-node-id=\"source-editor\" data-kind=\"editor\""));
        assert!(rendered.contains("data-node-id=\"diagnostics-panel\" data-kind=\"diagnostics\""));
        assert!(rendered.contains("data-node-id=\"run-output\" data-kind=\"run-output\" data-focusable=\"true\" data-disabled-reason-visible=\"true\""));
        assert!(rendered.contains("data-node-id=\"immediate-panel\" data-kind=\"immediate\""));
        assert!(rendered.contains("data-node-id=\"debug-panel\" data-kind=\"debug\""));
        assert!(
            rendered.contains("data-node-id=\"command-palette\" data-kind=\"command-palette\"")
        );
        assert!(rendered.contains("role=\"focus-restore-target\">source-editor"));
        assert!(rendered.contains("data-index=\"1\" data-node-id=\"project-tree\""));
        assert!(rendered.contains("data-index=\"5\" data-node-id=\"run-output\""));
        assert!(rendered.contains("data-index=\"6\" data-node-id=\"immediate-panel\""));
        assert!(rendered.contains("data-index=\"7\" data-node-id=\"debug-panel\""));
        assert!(rendered.contains("data-index=\"9\" data-node-id=\"command-palette\""));
        assert!(rendered.contains("returns to source-editor"));
        assert!(rendered.contains("Disabled reason panels remain reachable"));
    }

    #[test]
    fn accessibility_disabled_reasons_scenario_renders_labels_and_descriptions() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_ACCESSIBILITY_DISABLED_REASONS)
            .expect("render accessibility disabled reasons scenario");

        assert!(rendered.contains("data-scenario=\"gui-accessibility-disabled-reasons\""));
        assert!(rendered.contains("role=\"accessibility-projection\""));
        assert!(rendered.contains("data-source=\"gui-core accessibility projection\""));
        assert!(rendered.contains("data-web-framework-bound=\"false\""));
        assert!(rendered.contains("data-surface-count=\"10\""));
        assert!(rendered.contains("data-surface-id=\"source-editor\" data-role=\"editor\""));
        assert!(rendered.contains("role=\"accessible-label\">Source editor"));
        assert!(
            rendered.contains("data-surface-id=\"diagnostics-panel\" data-role=\"diagnostics\"")
        );
        assert!(rendered.contains("OxVba language-service diagnostics"));
        assert!(rendered.contains("data-surface-id=\"run-output\" data-role=\"run-output\" data-has-disabled-reason=\"true\""));
        assert!(rendered.contains("native execution provider unavailable"));
        assert!(rendered.contains("data-surface-id=\"immediate-panel\" data-role=\"immediate\" data-has-disabled-reason=\"true\""));
        assert!(rendered.contains("no native OxVba runtime session"));
        assert!(rendered.contains(
            "data-surface-id=\"debug-panel\" data-role=\"debug\" data-has-disabled-reason=\"true\""
        ));
        assert!(rendered.contains("no OxVba debug session"));
        assert!(rendered.contains("data-surface-id=\"com-capability\" data-role=\"com-capability\" data-has-disabled-reason=\"true\""));
        assert!(rendered.contains("COM discovery unavailable in browser-safe profile"));
        assert!(
            rendered
                .contains("data-surface-id=\"capability-footer\" data-role=\"capability-footer\"")
        );
        assert!(rendered.contains("no web framework accessibility API is chosen in core"));
    }

    #[test]
    fn shell_packet_baseline_scenario_renders_projection_contract_tokens() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_SHELL_PACKET_BASELINE)
            .expect("render shell packet baseline scenario");

        assert!(rendered.contains("data-scenario=\"gui-shell-packet-baseline\""));
        assert!(rendered.contains("role=\"shell-packet\""));
        assert!(rendered.contains("data-source=\"oxide-core GuiShellPacket\""));
        assert!(rendered.contains("data-project=\"ThinSliceHello\""));
        assert!(rendered.contains("data-active-module=\"Module1.bas\""));
        assert!(rendered.contains("data-module-count=\"1\""));
        assert!(rendered.contains("data-diagnostics-count=\"1\""));
        assert!(rendered.contains("data-native-execution-claimed=\"false\""));
        assert!(rendered.contains("data-com-runtime-claimed=\"false\""));
        assert!(rendered.contains("data-web-framework-bound=\"false\""));
        assert!(rendered.contains("data-parked-tui-imported=\"false\""));
        assert!(rendered.contains("data-surface=\"project-spine\""));
        assert!(rendered.contains("data-surface=\"source-editor\""));
        assert!(rendered.contains("data-surface=\"run-timeline\""));
        assert!(rendered.contains("data-surface=\"command-palette\""));
        assert!(rendered.contains("data-surface=\"keyboard-map\""));
        assert!(rendered.contains("data-surface=\"focus-graph\""));
        assert!(rendered.contains("data-surface=\"accessibility-projection\""));
        assert!(rendered.contains("role=\"shell-packet-command-count\">10"));
        assert!(rendered.contains("role=\"shell-packet-keybinding-count\">11"));
        assert!(rendered.contains("role=\"shell-packet-focus-node-count\">9"));
        assert!(rendered.contains("role=\"shell-packet-accessibility-count\">10"));
        assert!(rendered.contains("data-provider=\"browser-unsupported\""));
        assert!(rendered.contains("native execution provider unavailable"));
        assert!(rendered.contains("data-profile=\"browser-safe\""));
        assert!(rendered.contains("COM runtime unavailable in browser-safe profile"));
    }

    #[test]
    fn mounted_shell_static_scenario_renders_major_surfaces_from_packet() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_MOUNTED_SHELL_STATIC)
            .expect("render mounted shell static scenario");

        assert!(rendered.contains("data-scenario=\"gui-mounted-shell-static\""));
        assert!(rendered.contains("role=\"mounted-shell-static\""));
        assert!(rendered.contains("data-source=\"GuiShellPacket\""));
        assert!(rendered.contains("data-static-render=\"true\""));
        assert!(rendered.contains("data-dom-audited=\"false\""));
        assert!(rendered.contains("data-filesystem-persistence=\"false\""));
        assert!(rendered.contains("data-native-runtime=\"false\""));
        assert!(rendered.contains("data-com-runtime=\"false\""));
        assert!(rendered.contains("role=\"mounted-shell-title\">ThinSliceHello"));
        assert!(rendered.contains("role=\"mounted-project-tree\" data-module-count=\"1\""));
        assert!(rendered.contains("role=\"mounted-editor\" data-active-module=\"Module1.bas\""));
        assert!(rendered.contains("role=\"mounted-diagnostics\" data-count=\"1\""));
        assert!(rendered.contains("role=\"mounted-lifecycle\" data-profile=\"browser-limited\""));
        assert!(rendered.contains("role=\"mounted-run-output\" data-provider=\"browser-unsupported\" data-status=\"disabled\""));
        assert!(rendered.contains("native execution provider unavailable"));
        assert!(rendered.contains("role=\"mounted-com-capability\" data-profile=\"browser-safe\" data-runtime-available=\"false\""));
        assert!(rendered.contains("role=\"mounted-command-palette\" data-command-count=\"10\""));
        assert!(rendered.contains("data-keybinding-count=\"11\""));
        assert!(rendered.contains("data-focus-node-count=\"9\""));
        assert!(rendered.contains("data-accessibility-surface-count=\"10\""));
        assert!(rendered.contains("Static shell render consumes GuiShellPacket"));
        assert!(rendered.contains("no DOM accessibility audit, filesystem persistence, native runtime, or COM runtime is claimed"));
    }

    #[test]
    fn mounted_command_palette_scenario_preserves_commands_gestures_and_disabled_reasons() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_MOUNTED_COMMAND_PALETTE)
            .expect("render mounted command palette scenario");

        assert!(rendered.contains("data-scenario=\"gui-mounted-command-palette\""));
        assert!(rendered.contains("role=\"mounted-command-palette-detail\""));
        assert!(rendered.contains("data-source=\"GuiShellPacket.command_palette\""));
        assert!(rendered.contains("data-parked-tui-imported=\"false\""));
        assert!(rendered.contains("data-command-count=\"10\""));
        assert!(rendered.contains("data-command-id=\"project.open\""));
        assert!(rendered.contains(
            "data-command-id=\"document.save\" data-category=\"document\" data-gesture=\"Ctrl+S\""
        ));
        assert!(rendered.contains("browser-safe profile has no direct filesystem persistence"));
        assert!(rendered.contains(
            "data-command-id=\"runtime.run\" data-category=\"runtime\" data-gesture=\"F5\""
        ));
        assert!(rendered.contains("native execution provider unavailable"));
        assert!(rendered.contains(
            "data-command-id=\"runtime.immediate\" data-category=\"runtime\" data-gesture=\"Enter\""
        ));
        assert!(rendered.contains("no native OxVba runtime session"));
        assert!(rendered.contains(
            "data-command-id=\"runtime.debug\" data-category=\"runtime\" data-gesture=\"F10\""
        ));
        assert!(rendered.contains("no OxVba debug session"));
        assert!(rendered.contains("Mounted command palette consumes packet state"));
        assert!(rendered.contains("parked TUI command model not imported"));
    }

    #[test]
    fn mounted_no_mouse_accessibility_scenario_preserves_focus_route_and_labels() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_MOUNTED_NO_MOUSE_ACCESSIBILITY)
            .expect("render mounted no-mouse accessibility scenario");

        assert!(rendered.contains("data-scenario=\"gui-mounted-no-mouse-accessibility\""));
        assert!(rendered.contains("role=\"mounted-no-mouse-accessibility\""));
        assert!(rendered.contains("data-source=\"GuiShellPacket.focus_graph+accessibility\""));
        assert!(rendered.contains("data-web-framework-bound=\"false\""));
        assert!(rendered.contains("data-dom-audited=\"false\""));
        assert!(rendered.contains("data-route-length=\"10\""));
        assert!(rendered.contains("data-accessibility-surface-count=\"10\""));
        assert!(rendered.contains("data-index=\"1\" data-node-id=\"project-tree\""));
        assert!(rendered.contains("data-index=\"2\" data-node-id=\"source-editor\""));
        assert!(rendered.contains("data-index=\"3\" data-node-id=\"diagnostics-panel\""));
        assert!(rendered.contains("data-index=\"5\" data-node-id=\"run-output\""));
        assert!(rendered.contains("data-index=\"6\" data-node-id=\"immediate-panel\""));
        assert!(rendered.contains("data-index=\"7\" data-node-id=\"debug-panel\""));
        assert!(rendered.contains("data-index=\"8\" data-node-id=\"com-capability\""));
        assert!(rendered.contains("data-index=\"9\" data-node-id=\"command-palette\""));
        assert!(rendered.contains("returns to source-editor"));
        assert!(rendered.contains("data-surface-id=\"source-editor\" data-role=\"editor\""));
        assert!(rendered.contains("role=\"mounted-accessible-label\">Source editor"));
        assert!(rendered.contains("data-surface-id=\"run-output\" data-role=\"run-output\" data-has-disabled-reason=\"true\""));
        assert!(rendered.contains("native execution provider unavailable"));
        assert!(rendered.contains("data-surface-id=\"com-capability\" data-role=\"com-capability\" data-has-disabled-reason=\"true\""));
        assert!(rendered.contains("COM discovery unavailable in browser-safe profile"));
        assert!(rendered.contains("Mounted accessibility projection is packet-derived"));
        assert!(rendered.contains("DOM accessibility audit is not claimed"));
    }

    #[test]
    fn web_shell_adapter_boundary_scenario_renders_packet_markup_without_host_claims() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_WEB_SHELL_ADAPTER_BOUNDARY)
            .expect("render web shell adapter boundary scenario");

        assert!(rendered.contains("data-scenario=\"gui-web-shell-adapter-boundary\""));
        assert!(rendered.contains("role=\"web-shell-boundary-snapshot\""));
        assert!(rendered.contains("data-source=\"GuiShellPacket\""));
        assert!(rendered.contains("data-dom-smoke-tested=\"false\""));
        assert!(rendered.contains("data-dom-audited=\"false\""));
        assert!(rendered.contains("data-filesystem-persistence=\"false\""));
        assert!(rendered.contains("data-native-runtime=\"false\""));
        assert!(rendered.contains("data-com-runtime=\"false\""));
        assert!(rendered.contains("data-parked-tui-imported=\"false\""));
        assert!(rendered.contains("role=\"web-shell-adapter\""));
        assert!(rendered.contains("data-web-framework=\"unselected\""));
        assert!(rendered.contains("role=\"web-project-tree\""));
        assert!(rendered.contains("role=\"web-source-editor\""));
        assert!(rendered.contains("role=\"web-diagnostics\" data-count=\"1\""));
        assert!(rendered.contains("role=\"web-lifecycle\" data-profile=\"browser-limited\""));
        assert!(rendered.contains(
            "role=\"web-run-output\" data-provider=\"browser-unsupported\" data-status=\"disabled\""
        ));
        assert!(rendered.contains("native execution provider unavailable"));
        assert!(rendered.contains("role=\"web-com-capability\" data-profile=\"browser-safe\" data-runtime-available=\"false\""));
        assert!(rendered.contains(
            "role=\"web-command-summary\" data-command-count=\"10\" data-keybinding-count=\"11\""
        ));
        assert!(rendered.contains("data-command-id=\"runtime.run\""));
        assert!(rendered.contains("role=\"web-focus-accessibility-summary\" data-focus-node-count=\"9\" data-route-length=\"10\" data-accessibility-surface-count=\"10\""));
        assert!(rendered.contains("role=\"web-accessible-label\">Source editor"));
        assert!(rendered.contains("Web shell adapter consumes GuiShellPacket"));
        assert!(rendered.contains("no framework, DOM audit, filesystem persistence, native runtime, or COM runtime is claimed"));
    }

    #[test]
    fn web_shell_dom_smoke_scenario_reports_parsed_html_checks_without_audit_claim() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_WEB_SHELL_DOM_SMOKE)
            .expect("render web shell DOM smoke scenario");

        assert!(rendered.contains("data-scenario=\"gui-web-shell-dom-smoke\""));
        assert!(rendered.contains("role=\"web-shell-dom-smoke\""));
        assert!(rendered.contains("data-source=\"GuiShellPacket\""));
        assert!(rendered.contains("data-smoke-kind=\"parsed-html-tree\""));
        assert!(rendered.contains("data-dom-smoke-tested=\"true\""));
        assert!(rendered.contains("data-browser-runtime=\"false\""));
        assert!(rendered.contains("data-dom-audited=\"false\""));
        assert!(rendered.contains("data-all-passed=\"true\""));
        assert!(rendered.contains("data-check-count=\"17\""));
        assert!(
            rendered.contains("data-check=\"root consumes GuiShellPacket\" data-passed=\"true\"")
        );
        assert!(
            rendered
                .contains("data-check=\"project tree carries project name\" data-passed=\"true\"")
        );
        assert!(rendered.contains("ThinSliceHello"));
        assert!(
            rendered
                .contains("data-check=\"project tree shows active module\" data-passed=\"true\"")
        );
        assert!(rendered.contains("Module1.bas"));
        assert!(
            rendered
                .contains("data-check=\"source editor shows module source\" data-passed=\"true\"")
        );
        assert!(rendered.contains("Public Sub Main()"));
        assert!(rendered.contains(
            "data-check=\"filesystem persistence remains unclaimed\" data-passed=\"true\""
        ));
        assert!(
            rendered
                .contains("data-check=\"native runtime remains unclaimed\" data-passed=\"true\"")
        );
        assert!(
            rendered.contains("data-check=\"COM runtime remains unclaimed\" data-passed=\"true\"")
        );
        assert!(rendered.contains("Parsed HTML DOM smoke only"));
        assert!(rendered.contains("no browser runtime or DOM accessibility audit is claimed"));
    }

    #[test]
    fn web_command_palette_dom_smoke_scenario_reports_command_ids_gestures_and_disabled_reasons() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_WEB_COMMAND_PALETTE_DOM_SMOKE)
            .expect("render web command palette DOM smoke scenario");

        assert!(rendered.contains("data-scenario=\"gui-web-command-palette-dom-smoke\""));
        assert!(rendered.contains("role=\"web-command-palette-dom-smoke\""));
        assert!(rendered.contains("data-source=\"GuiShellPacket\""));
        assert!(rendered.contains("data-smoke-kind=\"parsed-html-command-palette\""));
        assert!(rendered.contains("data-dom-smoke-tested=\"true\""));
        assert!(rendered.contains("data-browser-runtime=\"false\""));
        assert!(rendered.contains("data-dom-audited=\"false\""));
        assert!(rendered.contains("data-all-passed=\"true\""));
        assert!(rendered.contains("data-check-count=\"15\""));
        assert!(rendered.contains(
            "data-check=\"project.open gesture survives DOM mounting\" data-passed=\"true\""
        ));
        assert!(rendered.contains("Ctrl+O"));
        assert!(rendered.contains(
            "data-check=\"document.save gesture survives DOM mounting\" data-passed=\"true\""
        ));
        assert!(rendered.contains("Ctrl+S"));
        assert!(rendered.contains(
            "data-check=\"runtime.run gesture survives DOM mounting\" data-passed=\"true\""
        ));
        assert!(rendered.contains("F5"));
        assert!(rendered.contains(
            "data-check=\"runtime.run disabled reason remains visible\" data-passed=\"true\""
        ));
        assert!(rendered.contains("native execution provider unavailable"));
        assert!(rendered.contains(
            "data-check=\"runtime.immediate gesture survives DOM mounting\" data-passed=\"true\""
        ));
        assert!(rendered.contains("Enter"));
        assert!(rendered.contains("no native OxVba runtime session"));
        assert!(rendered.contains(
            "data-check=\"runtime.debug gesture survives DOM mounting\" data-passed=\"true\""
        ));
        assert!(rendered.contains("F10"));
        assert!(rendered.contains("no OxVba debug session"));
        assert!(rendered.contains("data-check=\"shell.command_palette gesture survives DOM mounting\" data-passed=\"true\""));
        assert!(rendered.contains("Ctrl+Shift+P"));
        assert!(rendered.contains("parked TUI command model remains isolated"));
    }

    #[test]
    fn web_no_mouse_accessibility_dom_smoke_scenario_reports_route_labels_and_disabled_reasons() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_WEB_NO_MOUSE_ACCESSIBILITY_DOM_SMOKE)
            .expect("render web no-mouse accessibility DOM smoke scenario");

        assert!(rendered.contains("data-scenario=\"gui-web-no-mouse-accessibility-dom-smoke\""));
        assert!(rendered.contains("role=\"web-no-mouse-accessibility-dom-smoke\""));
        assert!(rendered.contains("data-source=\"GuiShellPacket\""));
        assert!(rendered.contains("data-smoke-kind=\"parsed-html-no-mouse-accessibility\""));
        assert!(rendered.contains("data-dom-smoke-tested=\"true\""));
        assert!(rendered.contains("data-browser-runtime=\"false\""));
        assert!(rendered.contains("data-dom-audited=\"false\""));
        assert!(rendered.contains("data-all-passed=\"true\""));
        assert!(rendered.contains("data-check-count=\"18\""));
        assert!(
            rendered
                .contains("data-check=\"focus route starts at project tree\" data-passed=\"true\"")
        );
        assert!(rendered.contains("project-tree"));
        assert!(
            rendered
                .contains("data-check=\"focus route reaches source editor\" data-passed=\"true\"")
        );
        assert!(rendered.contains("source-editor"));
        assert!(
            rendered
                .contains("data-check=\"focus route reaches diagnostics\" data-passed=\"true\"")
        );
        assert!(rendered.contains("diagnostics-panel"));
        assert!(
            rendered.contains("data-check=\"focus route reaches run output\" data-passed=\"true\"")
        );
        assert!(rendered.contains("run-output"));
        assert!(
            rendered.contains("data-check=\"focus route reaches Immediate\" data-passed=\"true\"")
        );
        assert!(rendered.contains("immediate-panel"));
        assert!(rendered.contains("data-check=\"focus route reaches debug\" data-passed=\"true\""));
        assert!(rendered.contains("debug-panel"));
        assert!(
            rendered
                .contains("data-check=\"focus route reaches COM capability\" data-passed=\"true\"")
        );
        assert!(rendered.contains("com-capability"));
        assert!(
            rendered.contains(
                "data-check=\"focus route reaches command palette\" data-passed=\"true\""
            )
        );
        assert!(rendered.contains("command-palette"));
        assert!(
            rendered.contains(
                "data-check=\"command palette restores editor focus\" data-passed=\"true\""
            )
        );
        assert!(rendered.contains("returns to source-editor"));
        assert!(rendered.contains("Source editor"));
        assert!(rendered.contains("Edit the active VBA module source."));
        assert!(rendered.contains("native execution provider unavailable"));
        assert!(rendered.contains("no native OxVba runtime session"));
        assert!(rendered.contains("no OxVba debug session"));
        assert!(rendered.contains("COM discovery unavailable in browser-safe profile"));
        assert!(rendered.contains("not a full accessibility audit"));
    }

    #[test]
    fn dnaonecalc_web_shell_host_contract_scenario_renders_boundaries_and_no_claims() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_DNAONECALC_WEB_SHELL_HOST_CONTRACT)
            .expect("render DnaOneCalc web shell host contract scenario");

        assert!(rendered.contains("data-scenario=\"gui-dnaonecalc-web-shell-host-contract\""));
        assert!(rendered.contains("role=\"dnaonecalc-web-shell-host-contract\""));
        assert!(rendered.contains("data-host=\"DnaOneCalc\""));
        assert!(rendered.contains("data-state-contract=\"GuiShellPacket\""));
        assert!(rendered.contains("data-embedding-contract=\"EmbeddedIdePacket\""));
        assert!(rendered.contains("data-web-adapter=\"oxide-webshell\""));
        assert!(rendered.contains("data-sibling-repo-writes=\"false\""));
        assert!(rendered.contains("data-host-mount-claimed=\"false\""));
        assert!(rendered.contains("data-filesystem-persistence=\"false\""));
        assert!(rendered.contains("data-native-runtime=\"false\""));
        assert!(rendered.contains("data-com-runtime=\"false\""));
        assert!(rendered.contains("data-dom-audited=\"false\""));
        assert!(rendered.contains("role=\"host-mount-input\">EmbeddedIdePacket"));
        assert!(rendered.contains("role=\"host-mount-input\">GuiShellPacket"));
        assert!(
            rendered
                .contains("role=\"host-mount-input\">oxide-webshell snapshot or mounted component")
        );
        assert!(rendered.contains(
            "role=\"host-surface-slot\" data-slot=\"project-spine\" data-owner=\"OxIde\""
        ));
        assert!(rendered.contains(
            "role=\"host-surface-slot\" data-slot=\"source-editor\" data-owner=\"OxIde\""
        ));
        assert!(rendered.contains("role=\"host-ownership-boundary\" data-owner=\"DnaOneCalc\""));
        assert!(rendered.contains("role=\"host-ownership-boundary\" data-owner=\"OxIde\""));
        assert!(rendered.contains("role=\"host-ownership-boundary\" data-owner=\"OxVba\""));
        assert!(rendered.contains("role=\"host-web-shell-summary\" data-project=\"ThinSliceHello\" data-active-module=\"Module1.bas\""));
        assert!(rendered.contains("data-command-count=\"10\""));
        assert!(rendered.contains("data-keybinding-count=\"11\""));
        assert!(rendered.contains("data-focus-route-length=\"10\""));
        assert!(rendered.contains("data-accessibility-surface-count=\"10\""));
        assert!(rendered.contains(
            "role=\"host-dom-readiness\" data-smoke-kind=\"parsed-html\" data-all-passed=\"true\""
        ));
        assert!(rendered.contains("data-static-shell=\"true\""));
        assert!(rendered.contains("data-command-palette=\"true\""));
        assert!(rendered.contains("data-no-mouse-accessibility=\"true\""));
        assert!(rendered.contains("DnaOneCalc browser host smoke is not claimed"));
        assert!(rendered.contains("did not modify DnaOneCalc files"));
    }

    #[test]
    fn dnaonecalc_web_shell_dom_readiness_scenario_reuses_w300_reports_without_host_claims() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_DNAONECALC_WEB_SHELL_DOM_READINESS)
            .expect("render DnaOneCalc web shell DOM readiness scenario");

        assert!(rendered.contains("data-scenario=\"gui-dnaonecalc-web-shell-dom-readiness\""));
        assert!(rendered.contains("role=\"dnaonecalc-web-shell-dom-readiness\""));
        assert!(rendered.contains("data-host=\"DnaOneCalc\""));
        assert!(rendered.contains("data-source=\"W300 DOM smoke reports\""));
        assert!(rendered.contains("data-static-shell=\"true\""));
        assert!(rendered.contains("data-command-palette=\"true\""));
        assert!(rendered.contains("data-no-mouse-accessibility=\"true\""));
        assert!(rendered.contains("data-browser-runtime=\"false\""));
        assert!(rendered.contains("data-dnaonecalc-host-smoke=\"false\""));
        assert!(rendered.contains("data-dom-audited=\"false\""));
        assert!(rendered.contains("data-filesystem-persistence=\"false\""));
        assert!(rendered.contains("data-native-runtime=\"false\""));
        assert!(rendered.contains("data-com-runtime=\"false\""));
        assert!(rendered.contains("data-report=\"static-shell\" data-smoke-kind=\"parsed-html-tree\" data-all-passed=\"true\""));
        assert!(rendered.contains("data-report=\"command-palette\" data-smoke-kind=\"parsed-html-command-palette\" data-all-passed=\"true\""));
        assert!(rendered.contains("data-report=\"no-mouse-accessibility\" data-smoke-kind=\"parsed-html-no-mouse-accessibility\" data-all-passed=\"true\""));
        assert!(rendered.contains("OxIde parsed HTML DOM readiness only"));
        assert!(rendered.contains("DnaOneCalc browser host smoke"));
        assert!(rendered.contains("full accessibility audit are not claimed"));
    }

    #[test]
    fn native_save_reload_disk_scenario_renders_disk_backed_persistence_tokens() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_NATIVE_SAVE_RELOAD_DISK)
            .expect("render native save/reload disk scenario");

        assert!(rendered.contains("data-scenario=\"gui-native-save-reload-disk\""));
        assert!(rendered.contains("role=\"native-save-reload-disk\""));
        assert!(rendered.contains("data-provider=\"native-filesystem\""));
        assert!(rendered.contains("data-filesystem-persistence=\"true\""));
        assert!(rendered.contains("data-test-owned-temp-project=\"true\""));
        assert!(rendered.contains("data-checked-in-fixture-mutated=\"false\""));
        assert!(rendered.contains("data-dirty-before-save=\"true\""));
        assert!(rendered.contains("data-dirty-after-save=\"false\""));
        assert!(rendered.contains("data-save-acknowledged=\"true\""));
        assert!(rendered.contains("data-reload-source-matches-disk=\"true\""));
        assert!(rendered.contains("data-native-runtime=\"false\""));
        assert!(rendered.contains("data-com-runtime=\"false\""));
        assert!(rendered.contains("answer = 21 * 2"));
        assert!(rendered.contains("test-owned temp project copy"));
        assert!(rendered.contains("runtime and COM runtime are not claimed"));
    }

    #[test]
    fn native_session_restore_disk_scenario_renders_disk_backed_session_tokens() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_NATIVE_SESSION_RESTORE_DISK)
            .expect("render native session restore disk scenario");

        assert!(rendered.contains("data-scenario=\"gui-native-session-restore-disk\""));
        assert!(rendered.contains("role=\"native-session-restore-disk\""));
        assert!(rendered.contains("data-provider=\"native-filesystem\""));
        assert!(rendered.contains("data-session-provider=\"native-filesystem-session\""));
        assert!(rendered.contains("data-filesystem-persistence=\"true\""));
        assert!(rendered.contains("data-test-owned-temp-project=\"true\""));
        assert!(rendered.contains("data-session-file-written=\"true\""));
        assert!(rendered.contains("data-checked-in-fixture-mutated=\"false\""));
        assert!(rendered.contains("data-restored-dirty=\"false\""));
        assert!(rendered.contains("data-native-runtime=\"false\""));
        assert!(rendered.contains("data-com-runtime=\"false\""));
        assert!(rendered.contains("role=\"native-session-module\">Module1.bas"));
        assert!(rendered.contains("answer = 84 / 2"));
        assert!(rendered.contains("OxIde-owned session JSON"));
        assert!(rendered.contains("runtime/COM are not claimed"));
    }

    #[test]
    fn browser_filesystem_still_disabled_scenario_preserves_browser_limitations() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_BROWSER_FILESYSTEM_STILL_DISABLED)
            .expect("render browser filesystem disabled scenario");

        assert!(rendered.contains("data-scenario=\"gui-browser-filesystem-still-disabled\""));
        assert!(rendered.contains("role=\"browser-filesystem-still-disabled\""));
        assert!(rendered.contains("data-provider=\"browser-limited\""));
        assert!(rendered.contains("data-filesystem-persistence=\"false\""));
        assert!(rendered.contains("data-save-enabled=\"false\""));
        assert!(rendered.contains("data-reload-enabled=\"false\""));
        assert!(rendered.contains("data-native-runtime=\"false\""));
        assert!(rendered.contains("data-com-runtime=\"false\""));
        assert!(rendered.contains("browser-safe profile has no direct filesystem persistence"));
        assert!(rendered.contains("Browser/WASM direct filesystem persistence remains disabled"));
    }

    #[test]
    fn runtime_service_contract_browser_disabled_scenario_keeps_real_execution_false() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_RUNTIME_SERVICE_CONTRACT_BROWSER_DISABLED)
            .expect("render runtime browser disabled contract scenario");

        assert!(
            rendered.contains("data-scenario=\"gui-runtime-service-contract-browser-disabled\"")
        );
        assert!(
            rendered.contains(
                "role=\"runtime-service-contract\" data-provider=\"browser-unsupported\""
            )
        );
        assert!(rendered.contains("data-command-enabled=\"false\""));
        assert!(rendered.contains("data-real-execution=\"false\""));
        assert!(rendered.contains("data-native-runtime=\"false\""));
        assert!(rendered.contains("data-com-runtime=\"false\""));
        assert!(rendered.contains("ThinSliceHello::Module1.Main"));
        assert!(rendered.contains("native execution provider unavailable"));
        assert!(
            rendered
                .contains("real OxVba execution, native runtime, and COM runtime are not claimed")
        );
    }

    #[test]
    fn runtime_service_contract_native_missing_scenario_keeps_real_execution_false() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_RUNTIME_SERVICE_CONTRACT_NATIVE_MISSING)
            .expect("render runtime native missing contract scenario");

        assert!(rendered.contains("data-scenario=\"gui-runtime-service-contract-native-missing\""));
        assert!(rendered.contains(
            "role=\"runtime-service-contract\" data-provider=\"native-service-missing\""
        ));
        assert!(rendered.contains("data-command-enabled=\"false\""));
        assert!(rendered.contains("data-real-execution=\"false\""));
        assert!(rendered.contains("data-native-runtime=\"false\""));
        assert!(rendered.contains("data-com-runtime=\"false\""));
        assert!(rendered.contains("native OxVba runtime service not configured"));
        assert!(rendered.contains("real execution unavailable"));
    }

    #[test]
    fn immediate_service_contract_native_missing_scenario_keeps_fake_responses_false() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_IMMEDIATE_SERVICE_CONTRACT_NATIVE_MISSING)
            .expect("render Immediate native missing contract scenario");

        assert!(
            rendered.contains("data-scenario=\"gui-immediate-service-contract-native-missing\"")
        );
        assert!(rendered.contains(
            "role=\"immediate-service-contract\" data-provider=\"native-service-missing\""
        ));
        assert!(rendered.contains("data-command-enabled=\"false\""));
        assert!(rendered.contains("data-response-count=\"0\""));
        assert!(rendered.contains("data-fake-responses=\"false\""));
        assert!(rendered.contains("data-native-runtime=\"false\""));
        assert!(rendered.contains("data-com-runtime=\"false\""));
        assert!(rendered.contains("role=\"immediate-service-request\">?answer"));
        assert!(rendered.contains("native OxVba runtime service not configured"));
        assert!(rendered.contains("fake responses are not allowed"));
    }

    #[test]
    fn debug_service_contract_native_missing_scenario_keeps_fake_debug_data_false() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_DEBUG_SERVICE_CONTRACT_NATIVE_MISSING)
            .expect("render debug native missing contract scenario");

        assert!(rendered.contains("data-scenario=\"gui-debug-service-contract-native-missing\""));
        assert!(
            rendered.contains(
                "role=\"debug-service-contract\" data-provider=\"native-service-missing\""
            )
        );
        assert!(rendered.contains("data-state=\"unavailable\""));
        assert!(rendered.contains("data-command-enabled=\"false\""));
        assert!(rendered.contains("data-command-count=\"6\""));
        assert!(rendered.contains("data-callstack-count=\"0\""));
        assert!(rendered.contains("data-locals-count=\"0\""));
        assert!(rendered.contains("data-watches-count=\"0\""));
        assert!(rendered.contains("data-breakpoints-count=\"0\""));
        assert!(rendered.contains("data-fake-debug-data=\"false\""));
        assert!(rendered.contains("data-native-runtime=\"false\""));
        assert!(rendered.contains("data-com-runtime=\"false\""));
        assert!(rendered.contains("native OxVba runtime/debug service not configured"));
        assert!(rendered.contains("fake debug data is not allowed"));
    }

    #[test]
    fn lifecycle_scenario_renders_dirty_state_and_persistence_honesty() {
        let registry = GuiScenarioRegistry::built_in(repo_root());

        let rendered = registry
            .render_text(GUI_THIN_SLICE_LIFECYCLE)
            .expect("render lifecycle scenario");

        assert!(rendered.contains("data-scenario=\"gui-thin-slice-lifecycle\""));
        assert!(rendered.contains("ThinSliceHello"));
        assert!(rendered.contains("Module1.bas"));
        assert!(rendered.contains("answer = 40 + 2"));
        assert!(!rendered.contains("Dim answer"));
        assert!(rendered.contains("role=\"document-lifecycle\""));
        assert!(rendered.contains("data-provider=\"browser-limited\""));
        assert!(rendered.contains("data-dirty=\"true\""));
        assert!(rendered.contains("role=\"document-state\">dirty"));
        assert!(rendered.contains("data-command=\"save\" data-enabled=\"false\""));
        assert!(rendered.contains("data-command=\"reload\" data-enabled=\"false\""));
        assert!(rendered.contains("data-command=\"revert\" data-enabled=\"true\""));
        assert!(rendered.contains("browser-safe profile has no direct filesystem persistence"));
        assert!(rendered.contains("role=\"persistence-proof\""));
        assert!(rendered.contains("data-provider=\"in-memory\""));
        assert!(rendered.contains("data-filesystem=\"false\""));
        assert!(rendered.contains("no filesystem persistence claimed"));
        assert!(rendered.contains("saved clean"));
        assert!(rendered.contains("role=\"session-restore\""));
        assert!(rendered.contains("data-profile=\"browser-limited\""));
        assert!(rendered.contains("role=\"restored-workspace\""));
        assert!(rendered.contains("ThinSliceHello.basproj"));
        assert!(rendered.contains("role=\"restored-module\">Module1.bas"));
        assert!(rendered.contains("role=\"restored-working-source\""));
        assert!(rendered.contains("COM unavailable"));
    }
}
