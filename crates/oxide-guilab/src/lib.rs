//! Browser-oriented GUI scenario lab boundary for OxIde.
//!
//! W210 starts with deterministic text rendering so the project-open
//! spine can be reviewed before a full browser mount exists.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use oxide_bridge::EmbeddedIdePacket;
use oxide_core::{
    ComCapabilityProfile, ComCapabilityStatus, ComReferenceFact, DebugCapabilityProfile,
    DocumentLifecycleState, GuiAccessibilityProjection, GuiCommandPalette, GuiFocusGraph,
    GuiKeyboardMap, GuiSessionSnapshot, ImmediateCapabilityProfile, InMemoryDocumentPersistence,
    LifecycleCapabilities, LifecycleCommand, LifecycleCommandStatus, RunCapabilityProfile,
    RunRequest, RunTimeline, RunTranscript, SessionCapabilityProfile,
    save_lifecycle_to_persistence,
};
use oxide_domain::{DiagnosticRow, OxideDomainRole};
use oxide_editor_core::{EditOperation, SourceSnapshot};
use oxide_oxvba::{
    EditedDocumentDiagnosticsError, ProjectOpenSpineError, load_edited_document_diagnostics,
    load_project_open_spine,
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
                fixture_path: thin_slice,
                kind: GuiScenarioKind::AccessibilityDisabledReasons,
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
        | GuiScenarioKind::AccessibilityDisabledReasons => view.active_source.source_text.clone(),
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
