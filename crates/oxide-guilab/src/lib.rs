//! Browser-oriented GUI scenario lab boundary for OxIde.
//!
//! W210 starts with deterministic text rendering so the project-open
//! spine can be reviewed before a full browser mount exists.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use oxide_domain::OxideDomainRole;
use oxide_oxvba::{ProjectOpenSpineError, load_project_open_spine};

pub const GUI_THIN_SLICE_LOADED: &str = "gui-thin-slice-loaded";

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
        Self::new(vec![GuiScenarioDescriptor {
            id: GUI_THIN_SLICE_LOADED,
            title: "Thin-slice project loaded",
            fixture_path: repo_root
                .join("examples")
                .join("thin-slice")
                .join("ThinSliceHello.basproj"),
        }])
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
    output.push_str("  <pre role=\"source\" data-module=\"");
    output.push_str(&view.active_source.module_display_name);
    output.push_str("\">");
    output.push_str(&html_escape(&view.active_source.source_text));
    output.push_str("</pre>\n");
    output.push_str("  <footer role=\"host-capability\">");
    output.push_str(&view.capability.status_text);
    output.push_str("</footer>\n");
    output.push_str("</section>\n");
    Ok(output)
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
}

impl std::fmt::Display for GuiLabError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateScenarioId { id } => write!(f, "duplicate GUI scenario id {id}"),
            Self::UnknownScenario { id } => write!(f, "unknown GUI scenario id {id}"),
            Self::Usage { message } => write!(f, "{message}"),
            Self::ProjectOpen(source) => write!(f, "project-open scenario failed: {source}"),
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
    }
}
