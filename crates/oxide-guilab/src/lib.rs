//! Browser-oriented GUI scenario lab boundary for OxIde.
//!
//! W210 starts with deterministic text rendering so the project-open
//! spine can be reviewed before a full browser mount exists.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use oxide_bridge::EmbeddedIdePacket;
use oxide_core::{
    ComCapabilityProfile, ComCapabilityStatus, ComReferenceFact, DocumentLifecycleState,
    GuiSessionSnapshot, InMemoryDocumentPersistence, LifecycleCapabilities, LifecycleCommand,
    LifecycleCommandStatus, RunCapabilityProfile, RunRequest, RunTranscript,
    SessionCapabilityProfile, save_lifecycle_to_persistence,
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
                fixture_path: thin_slice,
                kind: GuiScenarioKind::NonWindowsComUnavailable,
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
        | GuiScenarioKind::NonWindowsComUnavailable => view.active_source.source_text.clone(),
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

        assert_eq!(browser.title, "COM reference browser unavailable");
        assert_eq!(browser.kind, GuiScenarioKind::BrowserComUnavailable);
        assert_eq!(nonwindows.title, "COM reference non-Windows unavailable");
        assert_eq!(nonwindows.kind, GuiScenarioKind::NonWindowsComUnavailable);
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
