//! Fire Horse UX-lab provider.
//!
//! W039 starts with fixture-backed projections. The full terminal-cell renderer
//! lands in later W039 beads; this provider first proves that the scenario ids
//! and seam-shaped fixture data are loadable through the W038 lab registry.

pub mod adapter;
pub mod fixtures;
pub mod mockup;
pub mod projection;
pub mod renderer;

use super::{
    LabRenderError, LabRenderedFrame, LabScenarioDescriptor, LabScenarioProvider, ViewportClass,
};
use fixtures::projection_for_scenario;
use projection::FireHorseProjection;

pub struct FireHorseScenarioProvider;

impl LabScenarioProvider for FireHorseScenarioProvider {
    fn suite(&self) -> &'static str {
        "firehorse"
    }

    fn scenarios(&self) -> &'static [LabScenarioDescriptor] {
        &FIRE_HORSE_SCENARIOS
    }

    fn render(
        &self,
        scenario_id: &str,
        viewport: ViewportClass,
    ) -> Result<LabRenderedFrame, LabRenderError> {
        let (scenario, projection) = scenario_projection(scenario_id)?;

        if scenario.id == "firehorse-launchpad-standard" {
            Ok(renderer::render_launchpad_frame(
                scenario,
                viewport,
                &projection,
            ))
        } else if scenario.id == "firehorse-editing-lens-standard" {
            Ok(renderer::render_editing_lens_frame(
                scenario,
                viewport,
                &projection,
            ))
        } else if scenario.id == "firehorse-command-lens-standard" {
            Ok(renderer::render_command_lens_frame(
                scenario,
                viewport,
                &projection,
            ))
        } else if scenario.id == "firehorse-run-lane-standard" {
            Ok(renderer::render_run_lane_frame(
                scenario,
                viewport,
                &projection,
            ))
        } else if scenario.id == "firehorse-debug-cockpit-standard" {
            Ok(renderer::render_debug_cockpit_frame(
                scenario,
                viewport,
                &projection,
            ))
        } else if scenario.id == "firehorse-console-fit-light" {
            Ok(renderer::render_console_fit_frame(
                scenario,
                viewport,
                &projection,
            ))
        } else if scenario.id == "firehorse-focus-compact" {
            Ok(renderer::render_compact_focus_frame(
                scenario,
                viewport,
                &projection,
            ))
        } else if scenario.id == adapter::REAL_EDITING_SCENARIO_ID {
            Ok(renderer::render_editing_lens_frame(
                scenario,
                viewport,
                &projection,
            ))
        } else {
            Ok(render_fixture_summary(scenario, viewport, &projection))
        }
    }

    fn render_mockup(
        &self,
        scenario_id: &str,
        viewport: ViewportClass,
    ) -> Result<LabRenderedFrame, LabRenderError> {
        let (scenario, projection) = scenario_projection(scenario_id)?;
        Ok(mockup::render_mockup_frame(scenario, viewport, &projection))
    }

    fn render_mockup_terminal_stream(
        &self,
        scenario_id: &str,
        viewport: ViewportClass,
    ) -> Result<Vec<u8>, LabRenderError> {
        let (scenario, projection) = scenario_projection(scenario_id)?;
        Ok(mockup::render_mockup_terminal_stream(
            scenario,
            viewport,
            &projection,
        ))
    }
}

pub static FIRE_HORSE_PROVIDER: FireHorseScenarioProvider = FireHorseScenarioProvider;

pub static FIRE_HORSE_SCENARIOS: [LabScenarioDescriptor; 8] = [
    LabScenarioDescriptor {
        id: "firehorse-launchpad-standard",
        suite: "firehorse",
        title: "Fire Horse Launchpad",
        purpose: "Empty/recent/start/capability posture from the Fire Horse direction.",
        default_viewport: ViewportClass::Standard,
        tags: &["firehorse", "launchpad", "fixture", "w039"],
    },
    LabScenarioDescriptor {
        id: "firehorse-editing-lens-standard",
        suite: "firehorse",
        title: "Fire Horse Editing Lens",
        purpose: "North-star editing scene with full surface grammar.",
        default_viewport: ViewportClass::Standard,
        tags: &["firehorse", "editing-lens", "fixture", "w039"],
    },
    LabScenarioDescriptor {
        id: "firehorse-command-lens-standard",
        suite: "firehorse",
        title: "Fire Horse Command Lens",
        purpose: "Command overlay, preview, action ids, and disabled reasons.",
        default_viewport: ViewportClass::Standard,
        tags: &["firehorse", "command-lens", "fixture", "w039"],
    },
    LabScenarioDescriptor {
        id: "firehorse-run-lane-standard",
        suite: "firehorse",
        title: "Fire Horse Run Lane",
        purpose: "Staged run timeline and activity deck fixture.",
        default_viewport: ViewportClass::Standard,
        tags: &["firehorse", "run-lane", "fixture", "w039"],
    },
    LabScenarioDescriptor {
        id: "firehorse-debug-cockpit-standard",
        suite: "firehorse",
        title: "Fire Horse Debug Cockpit",
        purpose: "Paused debug posture with call stack, locals, watches, and Immediate deck.",
        default_viewport: ViewportClass::Standard,
        tags: &["firehorse", "debug-cockpit", "fixture", "w039"],
    },
    LabScenarioDescriptor {
        id: "firehorse-console-fit-light",
        suite: "firehorse",
        title: "Fire Horse Console Fit",
        purpose: "Light theme terminal capability posture with text labels and recommendations.",
        default_viewport: ViewportClass::Standard,
        tags: &["firehorse", "console-fit", "fixture", "w039"],
    },
    LabScenarioDescriptor {
        id: "firehorse-focus-compact",
        suite: "firehorse",
        title: "Fire Horse Compact Focus",
        purpose: "Compact source-first posture at the W038 compact viewport.",
        default_viewport: ViewportClass::Compact,
        tags: &["firehorse", "compact-focus", "fixture", "w039"],
    },
    LabScenarioDescriptor {
        id: adapter::REAL_EDITING_SCENARIO_ID,
        suite: "firehorse",
        title: "Fire Horse Real Editing Adapter",
        purpose: "Read-only ShellState to FireHorseProjection adapter over the thin-slice project.",
        default_viewport: ViewportClass::Standard,
        tags: &["firehorse", "real-editing", "adapter", "w039"],
    },
];

fn scenario_projection(
    scenario_id: &str,
) -> Result<(LabScenarioDescriptor, FireHorseProjection), LabRenderError> {
    let scenario = FIRE_HORSE_SCENARIOS
        .iter()
        .find(|scenario| scenario.id == scenario_id)
        .copied()
        .ok_or_else(|| LabRenderError::UnknownScenario {
            suite: "firehorse",
            id: scenario_id.to_string(),
        })?;
    let projection = if scenario.id == adapter::REAL_EDITING_SCENARIO_ID {
        adapter::thin_slice_editing_projection()
    } else {
        projection_for_scenario(scenario_id).ok_or_else(|| LabRenderError::UnknownScenario {
            suite: "firehorse",
            id: scenario_id.to_string(),
        })?
    };

    Ok((scenario, projection))
}

fn render_fixture_summary(
    scenario: LabScenarioDescriptor,
    viewport: ViewportClass,
    projection: &FireHorseProjection,
) -> LabRenderedFrame {
    let size = viewport.wtd_size();
    let mut lines = vec![
        "OxIde Fire Horse UX Lab".to_string(),
        format!("suite: {}  scenario: {}", scenario.suite, scenario.id),
        format!("title: {}", scenario.title),
        format!(
            "viewport: {} {}x{}",
            viewport.name(),
            size.width,
            size.height
        ),
        format!("layout: {}", projection.layout.name()),
        format!("identity: {}", projection.identity.workspace_label),
        format!("activity: {}", projection.activity_deck.active.name()),
        format!(
            "key rail: {}",
            projection
                .key_rail
                .hints
                .iter()
                .map(|hint| hint.action_id)
                .collect::<Vec<_>>()
                .join(", ")
        ),
    ];

    if let Some(spine) = &projection.project_spine {
        lines.push(format!("project spine rows: {}", spine.rows.len()));
    } else {
        lines.push("project spine rows: hidden".to_string());
    }
    lines.push(format!(
        "code canvas: {} ({} lines)",
        projection.code_canvas.document_label,
        projection.code_canvas.lines.len()
    ));
    if let Some(dock) = &projection.context_dock {
        lines.push(format!(
            "context dock: {} ({} cards)",
            dock.title,
            dock.cards.len()
        ));
    } else {
        lines.push("context dock: hidden".to_string());
    }
    if let Some(overlay) = &projection.overlay {
        lines.push(format!("overlay: {}", overlay.name()));
    }
    if let Some(fit) = &projection.terminal_fit {
        lines.push(format!("terminal fit: {}", fit.summary));
    }
    lines.push(format!(
        "seam fixtures: diagnostics {} | symbols {} | run events {} | debug frames {}",
        projection.seams.diagnostics.len(),
        projection.seams.symbols.len(),
        projection.seams.run_events.len(),
        projection.seams.debug_frames.len()
    ));

    LabRenderedFrame {
        scenario,
        viewport,
        size,
        text: fixed_width_text(&lines, size.width, size.height),
    }
}

fn fixed_width_text(lines: &[String], width: u16, height: u16) -> String {
    let width = width as usize;
    let height = height as usize;
    let mut output = String::new();

    for row in 0..height {
        if row > 0 {
            output.push('\n');
        }
        let mut line = lines.get(row).cloned().unwrap_or_default();
        if line.len() > width {
            line.truncate(width);
        }
        output.push_str(&line);
        for _ in line.len()..width {
            output.push(' ');
        }
    }
    output.push('\n');
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shell::uxlab::LabScenarioRegistry;
    use projection::{LayoutPosture, RunStepKind};

    #[test]
    fn every_firehorse_scenario_resolves_to_a_projection() {
        let registry = LabScenarioRegistry::built_in();

        for descriptor in FIRE_HORSE_SCENARIOS {
            let projection = projection_for_descriptor(descriptor);
            assert_eq!(projection.scenario_id, descriptor.id);
            assert_eq!(projection.layout, projection.expected_layout);
            assert!(
                registry.find("firehorse", descriptor.id).is_some(),
                "registry should expose {}",
                descriptor.id
            );
        }
    }

    #[test]
    fn every_firehorse_projection_has_core_rails() {
        for descriptor in FIRE_HORSE_SCENARIOS {
            let projection = projection_for_descriptor(descriptor);

            assert_eq!(projection.identity.product, "OxIde");
            assert!(!projection.identity.workspace_label.is_empty());
            assert!(
                !projection.activity_deck.tabs.is_empty(),
                "{} should have activity tabs",
                descriptor.id
            );
            assert!(
                !projection.activity_deck.rows.is_empty(),
                "{} should have activity rows",
                descriptor.id
            );
            assert!(
                !projection.key_rail.hints.is_empty(),
                "{} should have key rail hints",
                descriptor.id
            );
        }
    }

    #[test]
    fn fixture_data_uses_oxvba_shaped_seam_structs() {
        let editing = projection_for_scenario("firehorse-editing-lens-standard")
            .expect("editing fixture should load");
        assert!(!editing.seams.diagnostics.is_empty());
        assert!(!editing.seams.symbols.is_empty());
        let diagnostic = &editing.seams.diagnostics[0];
        assert_eq!(
            diagnostic.document_id,
            "doc://NorthwindPricing/PriceFor.bas"
        );
        assert_eq!(diagnostic.range.start.line, 8);
        assert_eq!(
            diagnostic.provenance.provider,
            "HostWorkspaceSession::diagnostics"
        );
        let symbol = &editing.seams.symbols[0];
        assert_eq!(symbol.provenance.provider, "HostWorkspaceSession::hover");
        assert_eq!(symbol.range.start.line, 6);

        let run_lane = projection_for_scenario("firehorse-run-lane-standard")
            .expect("run fixture should load");
        assert_eq!(run_lane.layout, LayoutPosture::RunLane);
        assert!(!run_lane.seams.run_events.is_empty());
        assert_eq!(run_lane.seams.run_events[0].target_id, "ExcelDesktop");
        assert_eq!(run_lane.seams.run_events[0].step, RunStepKind::Prepare);

        let debug = projection_for_scenario("firehorse-debug-cockpit-standard")
            .expect("debug fixture should load");
        assert_eq!(debug.layout, LayoutPosture::DebugCockpit);
        assert!(!debug.seams.debug_frames.is_empty());
        assert_eq!(
            debug.seams.debug_frames[0].document_id,
            "doc://NorthwindPricing/PriceFor.bas"
        );
        assert!(!debug.seams.locals.is_empty());
        assert!(!debug.seams.watches.is_empty());
    }

    #[test]
    fn firehorse_provider_lists_all_scenarios_with_viewports() {
        let registry = LabScenarioRegistry::built_in();
        let rows = registry.available_rows_for_suite("firehorse").join("\n");

        assert!(rows.contains("firehorse/firehorse-launchpad-standard"));
        assert!(rows.contains("firehorse/firehorse-editing-lens-standard"));
        assert!(rows.contains("firehorse/firehorse-focus-compact"));
        assert!(rows.contains("firehorse/firehorse-real-editing"));
        assert!(rows.contains("compact 92x30"));
    }

    #[test]
    fn list_command_can_filter_to_firehorse_suite() {
        let registry = LabScenarioRegistry::built_in();
        let mut output = Vec::new();

        crate::shell::uxlab::run_cli(
            ["--suite", "firehorse", "--list"]
                .into_iter()
                .map(String::from),
            &registry,
            &mut output,
        )
        .expect("firehorse list should render");

        let text = String::from_utf8(output).expect("utf8 list output");
        assert!(text.contains("firehorse/firehorse-editing-lens-standard"));
        assert!(text.contains("firehorse/firehorse-focus-compact"));
        assert!(text.contains("firehorse/firehorse-real-editing"));
        assert!(!text.contains("lab-smoke/lab-smoke-editing"));
    }

    fn projection_for_descriptor(descriptor: LabScenarioDescriptor) -> FireHorseProjection {
        if descriptor.id == adapter::REAL_EDITING_SCENARIO_ID {
            adapter::thin_slice_editing_projection()
        } else {
            projection_for_scenario(descriptor.id)
                .unwrap_or_else(|| panic!("missing projection for {}", descriptor.id))
        }
    }
}
