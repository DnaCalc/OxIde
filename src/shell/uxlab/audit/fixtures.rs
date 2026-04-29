use std::collections::BTreeSet;

use super::model::*;
use crate::shell::uxlab::firehorse::adapter::REAL_EDITING_SCENARIO_ID;

pub fn firehorse_audit_suite() -> UxAuditSuite {
    UxAuditSuite {
        schema_version: AUDIT_SCHEMA_VERSION,
        id: "firehorse",
        title: "Fire Horse UX Audit",
        personas: personas(),
        scenarios: scenarios(),
        criteria: criteria(),
        rubrics: rubrics(),
    }
}

pub fn design_brief_for(
    suite: &UxAuditSuite,
    scenario_id: &str,
    viewport: &str,
) -> Option<UxDesignBrief> {
    let scenario = suite.find_scenario(scenario_id)?;
    let persona = suite.find_persona(scenario.persona_id)?;
    Some(UxDesignBrief {
        scenario_id: scenario.id,
        firehorse_scenario_id: scenario.firehorse_scenario_id,
        viewport: viewport.to_string(),
        persona_id: persona.id,
        design_intent: scenario.intent,
        aesthetic_target: persona.delight_target,
        must_preserve: must_preserve_for(scenario.firehorse_scenario_id),
        likely_files: vec![
            "src/shell/uxlab/firehorse/mockup.rs",
            "src/shell/uxlab/firehorse/fixtures.rs",
            "src/shell/uxlab/audit/",
            "docs/firehorse_mockups/ux_audit_lab/",
        ],
        render_commands: render_commands(scenario.firehorse_scenario_id, viewport),
        evaluation_commands: evaluation_commands(scenario.firehorse_scenario_id, viewport),
        reference_artifacts: scenario.reference_artifacts.clone(),
    })
}

pub fn matrix_rows(suite: &UxAuditSuite) -> Vec<UxAuditMatrixRow> {
    suite
        .scenarios
        .iter()
        .map(|scenario| {
            let criteria = suite
                .criteria
                .iter()
                .map(|criterion| criterion.id)
                .collect::<Vec<_>>();
            let downstream_worksets = scenario
                .steps
                .iter()
                .flat_map(|step| step.expected_surfaces.iter())
                .map(|surface| surface.owner_workset)
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();

            UxAuditMatrixRow {
                audit_scenario_id: scenario.id,
                firehorse_scenario_id: scenario.firehorse_scenario_id,
                persona_id: scenario.persona_id,
                default_viewport: scenario.default_viewport,
                criteria,
                downstream_worksets,
            }
        })
        .collect()
}

fn personas() -> Vec<UxPersona> {
    vec![
        UxPersona {
            id: "pricing_maintainer",
            title: "Pricing Maintainer",
            role: "Excel/VBA maintainer responsible for pricing logic.",
            job_pressure: "Needs fast confidence while editing and running business-critical code.",
            goals: vec![
                "Keep source context central while inspecting diagnostics.",
                "Run the pricing project without losing the active source trail.",
                "Understand semantic warnings without guessing their provenance.",
            ],
            constraints: vec![
                "Works inside a terminal on a desktop workstation.",
                "Needs host truth from OxVba, not decorative UI inference.",
            ],
            delight_target: "The IDE feels dense, fast, and calm enough to trust during pricing changes.",
            failure_modes: vec![
                "Diagnostic context hides source code.",
                "Run state feels detached from the active module.",
                "Semantic claims have no seam provenance.",
            ],
            source_refs: vec![doc_ref(
                "docs/uxpass/10_user_journeys.md",
                "W035 user journeys",
            )],
        },
        UxPersona {
            id: "migration_reviewer",
            title: "Migration Reviewer",
            role: "Developer reviewing inherited VBA during modernization.",
            job_pressure: "Needs meaning, provenance, and navigation without breaking host truth.",
            goals: vec![
                "See which semantic facts are available and which are not.",
                "Trace project and document facts back to OxVba-shaped seams.",
            ],
            constraints: vec![
                "Cannot accept UI copy as semantic truth.",
                "Needs explicit unavailable data rather than guessed semantics.",
            ],
            delight_target: "The interface is honest about current and future seam ownership.",
            failure_modes: vec![
                "Adapter output implies unsupported semantics.",
                "Missing seam data is hidden in polished UI copy.",
            ],
            source_refs: vec![doc_ref(
                "docs/firehorse_mockups/UX_PROJECTION_CONTRACT.md",
                "Fire Horse projection contract",
            )],
        },
        UxPersona {
            id: "debug_responder",
            title: "Debug Responder",
            role: "User diagnosing a live macro failure.",
            job_pressure: "Needs paused-state clarity, stack/local visibility, and safe recovery.",
            goals: vec![
                "Understand where execution is paused.",
                "Inspect call stack, locals, watches, and Immediate affordances together.",
            ],
            constraints: vec![
                "Debug data is still a fixture until W080 owns the real contract.",
                "Step controls must be visible without overwhelming source context.",
            ],
            delight_target: "Paused debug state feels controlled, recoverable, and precise.",
            failure_modes: vec![
                "Paused line and stack relationship is unclear.",
                "Watches and locals look like a generic dump.",
            ],
            source_refs: vec![doc_ref(
                "docs/firehorse_mockups/refined_05_debug_cockpit.png",
                "Debug Cockpit mockup",
            )],
        },
        UxPersona {
            id: "terminal_power_user",
            title: "Terminal Power User",
            role: "Developer choosing OxIde because terminal workflow is faster.",
            job_pressure: "Needs dense, precise, low-friction command and viewport behavior.",
            goals: vec![
                "Use high-end terminal width as a strength.",
                "See command affordances and disabled reasons without modal confusion.",
                "Scale down honestly without flattening the primary desktop experience.",
            ],
            constraints: vec![
                "Truecolor and terminal-cell craft matter.",
                "Compact mode is a fallback posture, not the design baseline.",
            ],
            delight_target: "The TUI feels more efficient and emotionally sharper than a comparable GUI.",
            failure_modes: vec![
                "High-end layouts waste space.",
                "Command rows feel like old console menus.",
                "Compact compromises leak into Studio and First-class design.",
            ],
            source_refs: vec![doc_ref(
                "docs/DESIGN_TUI_2026_FIRE_HORSE.md",
                "Fire Horse UX doctrine",
            )],
        },
    ]
}

fn scenarios() -> Vec<UxAuditScenario> {
    vec![
        scenario(
            "audit-launchpad-cold-start",
            "firehorse-launchpad-standard",
            "terminal_power_user",
            "Launchpad cold-start confidence",
            "Cold-start should expose recent work, start actions, and console-fit posture without feeling like a landing page.",
            "studio",
            "Start or reopen work from a capable high-end terminal posture.",
            vec![
                surface(
                    "Identity Rail",
                    "identity",
                    "workspace and readiness posture",
                    "W110",
                ),
                surface(
                    "Start Context",
                    "context_dock",
                    "honest no-project state",
                    "W040",
                ),
                surface(
                    "Key Rail",
                    "key_rail.hints",
                    "start commands with action ids",
                    "W090",
                ),
            ],
            vec!["project.open", "project.create", "app.console_fit"],
            vec![seam("ProjectSession", SeamStatus::Unavailable, "W040")],
            vec![
                image_ref(
                    "docs/firehorse_mockups/refined_01_launchpad.png",
                    "Launchpad mockup",
                ),
                capture_ref(
                    "docs/firehorse_mockups/frankentui_terminal_review/captures/launchpad_studio.txt",
                    "Launchpad Studio capture",
                ),
            ],
        ),
        scenario(
            "audit-editing-lens-pricing",
            "firehorse-editing-lens-standard",
            "pricing_maintainer",
            "Editing Lens pricing loop",
            "The main high-end editing scene should preserve source dominance while surfacing diagnostics, hover provenance, activity state, and command affordances.",
            "studio",
            "Maintain a pricing function with semantic confidence.",
            vec![
                surface(
                    "Code Canvas",
                    "code_canvas",
                    "source-centered editing and lens",
                    "W050",
                ),
                surface(
                    "Context Dock",
                    "context_dock.cards",
                    "diagnostic and symbol cards",
                    "W060",
                ),
                surface(
                    "Activity Deck",
                    "activity_deck",
                    "Problems/Output/References task surface",
                    "W060",
                ),
                surface(
                    "Key Rail",
                    "key_rail.hints",
                    "save, semantic, run, command actions",
                    "W090",
                ),
            ],
            vec![
                "editor.save",
                "command.lens.open",
                "semantic.hover",
                "run.start",
            ],
            vec![
                seam(
                    "HostWorkspaceSession::diagnostics",
                    SeamStatus::Real,
                    "W060",
                ),
                seam("HostWorkspaceSession::hover", SeamStatus::Real, "W060"),
            ],
            vec![
                image_ref(
                    "docs/firehorse_mockups/refined_02_editing_lens.png",
                    "Editing Lens mockup",
                ),
                capture_ref(
                    "docs/firehorse_mockups/frankentui_terminal_review/captures/editing_lens_studio.txt",
                    "Editing Lens Studio capture",
                ),
            ],
        ),
        scenario(
            "audit-command-lens-run",
            "firehorse-command-lens-standard",
            "terminal_power_user",
            "Command Lens run selection",
            "Command search should feel modern and fast, with preview, action ids, and disabled reasons visible.",
            "first-class",
            "Filter to run commands and understand what can execute.",
            vec![
                surface(
                    "Overlay",
                    "overlay.CommandLens",
                    "filter, rows, preview, disabled reasons",
                    "W090",
                ),
                surface(
                    "Code Canvas",
                    "code_canvas",
                    "backing source remains legible",
                    "W050",
                ),
                surface(
                    "Key Rail",
                    "key_rail.hints",
                    "overlay-specific actions",
                    "W090",
                ),
            ],
            vec!["run.start", "run.stop", "target.configure"],
            vec![seam("ActionRegistry", SeamStatus::Future, "W090")],
            vec![
                image_ref(
                    "docs/firehorse_mockups/refined_03_command_lens.png",
                    "Command Lens mockup",
                ),
                capture_ref(
                    "docs/firehorse_mockups/frankentui_terminal_review/captures/command_lens_first-class.txt",
                    "Command Lens First-class capture",
                ),
            ],
        ),
        scenario(
            "audit-run-lane-progress",
            "firehorse-run-lane-standard",
            "pricing_maintainer",
            "Run Lane progress review",
            "Run progress should keep source continuity while showing staged build/execute state and Immediate/output affordances.",
            "studio",
            "Run pricing code and know exactly which stage is active.",
            vec![
                surface(
                    "Run Timeline",
                    "activity_deck.rows",
                    "staged run events",
                    "W070",
                ),
                surface(
                    "Code Canvas",
                    "code_canvas",
                    "source continuity during run",
                    "W050",
                ),
                surface(
                    "Run Context",
                    "context_dock.cards",
                    "active run status",
                    "W070",
                ),
            ],
            vec!["run.stop", "immediate.focus", "scene.return_edit"],
            vec![seam("WebHostEvent", SeamStatus::Real, "W070")],
            vec![
                image_ref(
                    "docs/firehorse_mockups/refined_04_run_lane.png",
                    "Run Lane mockup",
                ),
                capture_ref(
                    "docs/firehorse_mockups/frankentui_terminal_review/captures/run_lane_studio.txt",
                    "Run Lane Studio capture",
                ),
            ],
        ),
        scenario(
            "audit-debug-cockpit-paused",
            "firehorse-debug-cockpit-standard",
            "debug_responder",
            "Debug Cockpit paused-state review",
            "Debug state should make pause location, stack, locals, watches, and step controls precise without burying source.",
            "studio",
            "Diagnose a paused macro failure and choose the next debug action.",
            vec![
                surface(
                    "Code Canvas",
                    "code_canvas.execution_line",
                    "paused execution line",
                    "W080",
                ),
                surface(
                    "Debug Context",
                    "context_dock.cards",
                    "call stack, locals, watches",
                    "W080",
                ),
                surface(
                    "Activity Deck",
                    "activity_deck",
                    "Immediate and watch trace surfaces",
                    "W080",
                ),
            ],
            vec![
                "debug.continue",
                "debug.step",
                "debug.step_out",
                "immediate.focus",
            ],
            vec![seam(
                "W080 debug contract audit",
                SeamStatus::Future,
                "W080",
            )],
            vec![
                image_ref(
                    "docs/firehorse_mockups/refined_05_debug_cockpit.png",
                    "Debug Cockpit mockup",
                ),
                capture_ref(
                    "docs/firehorse_mockups/frankentui_terminal_review/captures/debug_cockpit_studio.txt",
                    "Debug Cockpit Studio capture",
                ),
            ],
        ),
        scenario(
            "audit-console-fit-light",
            "firehorse-console-fit-light",
            "terminal_power_user",
            "Console Fit capability review",
            "Capability guidance should educate without weakening the source-first IDE direction.",
            "studio",
            "Check terminal fit and understand any visual capability constraints.",
            vec![
                surface(
                    "Terminal Fit",
                    "terminal_fit.rows",
                    "capability results and recommendations",
                    "W100",
                ),
                surface(
                    "Key Rail",
                    "key_rail.hints",
                    "rerun/report/return actions",
                    "W090",
                ),
            ],
            vec!["app.console_fit", "scene.return_edit"],
            vec![seam("TerminalCapabilityProbe", SeamStatus::Future, "W100")],
            vec![
                image_ref(
                    "docs/firehorse_mockups/refined_06_console_fit.png",
                    "Console Fit mockup",
                ),
                capture_ref(
                    "docs/firehorse_mockups/frankentui_terminal_review/captures/console_fit_studio.txt",
                    "Console Fit Studio capture",
                ),
            ],
        ),
        scenario(
            "audit-compact-focus-degradation",
            "firehorse-focus-compact",
            "terminal_power_user",
            "Compact Focus degradation review",
            "Small terminal mode should stay source-first and honest without becoming the design baseline.",
            "compact",
            "Work in compact source-first mode with explicit side-surface affordances.",
            vec![
                surface(
                    "Code Canvas",
                    "code_canvas",
                    "source-first compact posture",
                    "W050",
                ),
                surface(
                    "Key Rail",
                    "key_rail.hints",
                    "dock affordances in compact mode",
                    "W100",
                ),
            ],
            vec![
                "focus.project",
                "focus.code",
                "focus.context",
                "focus.activity",
            ],
            vec![seam("LayoutPolicy", SeamStatus::Future, "W100")],
            vec![
                image_ref(
                    "docs/firehorse_mockups/refined_07_compact_focus_mode.png",
                    "Compact Focus mockup",
                ),
                capture_ref(
                    "docs/firehorse_mockups/frankentui_terminal_review/captures/compact_focus_default_compact.txt",
                    "Compact Focus compact capture",
                ),
            ],
        ),
        scenario(
            "audit-real-editing-adapter-honesty",
            REAL_EDITING_SCENARIO_ID,
            "migration_reviewer",
            "Real Editing adapter honesty",
            "Adapter output should show current shell state while marking missing seams explicitly.",
            "studio",
            "Review real thin-slice state without mistaking proof data for semantics.",
            vec![
                surface(
                    "Project Spine",
                    "project_spine.rows",
                    "real mounted workspace rows",
                    "W040",
                ),
                surface(
                    "Code Canvas",
                    "code_canvas.lines",
                    "active buffer from shell state",
                    "W050",
                ),
                surface(
                    "Context Dock",
                    "context_dock.cards.Unavailable",
                    "missing seams stated explicitly",
                    "W060",
                ),
            ],
            vec!["editor.save", "semantic.hover", "run.start"],
            vec![
                seam("DocumentSession", SeamStatus::Future, "W050"),
                seam(
                    "HostWorkspaceSession::diagnostics",
                    SeamStatus::Unavailable,
                    "W060",
                ),
            ],
            vec![
                image_ref(
                    "docs/firehorse_mockups/refined_02_editing_lens.png",
                    "Editing Lens mockup reference",
                ),
                capture_ref(
                    "docs/firehorse_mockups/frankentui_terminal_review/captures/real_editing_adapter_studio.txt",
                    "Real Editing Adapter Studio capture",
                ),
            ],
        ),
    ]
}

fn scenario(
    id: &'static str,
    firehorse_scenario_id: &'static str,
    persona_id: &'static str,
    title: &'static str,
    intent: &'static str,
    default_viewport: &'static str,
    user_intent: &'static str,
    expected_surfaces: Vec<UxSurfaceExpectation>,
    expected_actions: Vec<&'static str>,
    seam_refs: Vec<UxSeamRef>,
    reference_artifacts: Vec<AuditArtifactRef>,
) -> UxAuditScenario {
    UxAuditScenario {
        id,
        firehorse_scenario_id,
        persona_id,
        title,
        intent,
        default_viewport,
        steps: vec![UxJourneyStep {
            id: "primary",
            title: "Primary audit posture",
            user_intent,
            expected_surfaces,
            expected_actions,
            state_refs: vec![UxStateRef {
                owner: "FireHorseProjection",
                field: "scenario_id",
                downstream_workset: "W039",
            }],
            seam_refs,
        }],
        reference_artifacts,
    }
}

fn criteria() -> Vec<UxAuditCriterion> {
    vec![
        criterion(
            "functional.persona_fit",
            AuditCategory::PersonaFit,
            "Does the scene serve the named persona pressure?",
            EvaluationMode::Functional,
        ),
        criterion(
            "functional.journey_fit",
            AuditCategory::JourneyFit,
            "Are expected surfaces, state tokens, and next actions visible?",
            EvaluationMode::Functional,
        ),
        criterion(
            "functional.command_clarity",
            AuditCategory::CommandClarity,
            "Do visible commands have action ids and disabled reasons where needed?",
            EvaluationMode::Functional,
        ),
        criterion(
            "functional.state_ownership",
            AuditCategory::StateOwnership,
            "Can visible state be traced to projection fields and downstream owners?",
            EvaluationMode::Functional,
        ),
        criterion(
            "functional.seam_honesty",
            AuditCategory::SeamHonesty,
            "Do VBA/project facts map to OxVba seams or explicit future/unavailable seams?",
            EvaluationMode::Functional,
        ),
        criterion(
            "functional.degradation",
            AuditCategory::Degradation,
            "Does compact/fallback preserve the task without defining the high-end baseline?",
            EvaluationMode::Functional,
        ),
        criterion(
            "aesthetic.hierarchy",
            AuditCategory::Hierarchy,
            "Can source, context, activity, command, and status priority be read immediately?",
            EvaluationMode::Aesthetic,
        ),
        criterion(
            "aesthetic.density",
            AuditCategory::Density,
            "Do Studio and First-class use space richly without filler or crowding?",
            EvaluationMode::Aesthetic,
        ),
        criterion(
            "aesthetic.balance",
            AuditCategory::Balance,
            "Do rails, docks, canvas, overlays, and lower surfaces carry intentional visual weight?",
            EvaluationMode::Aesthetic,
        ),
        criterion(
            "aesthetic.tone_color",
            AuditCategory::ToneAndColor,
            "Does truecolor output preserve graphite/ember energy while remaining readable?",
            EvaluationMode::Aesthetic,
        ),
        criterion(
            "aesthetic.terminal_craft",
            AuditCategory::TerminalCraft,
            "Do box drawing, padding, clipping, and line rhythm feel deliberate?",
            EvaluationMode::Aesthetic,
        ),
        criterion(
            "aesthetic.reference_fidelity",
            AuditCategory::ReferenceFidelity,
            "Can the result cite what it keeps, changes, or rejects from the colourful mockup?",
            EvaluationMode::Aesthetic,
        ),
        criterion(
            "aesthetic.text_fit",
            AuditCategory::TextFit,
            "Do labels, code, commands, and status text avoid incoherent overlap?",
            EvaluationMode::Aesthetic,
        ),
        criterion(
            "aesthetic.emotional_fit",
            AuditCategory::EmotionalFit,
            "Does the scene feel modern, capable, calm, and not nostalgia-console?",
            EvaluationMode::Aesthetic,
        ),
    ]
}

fn criterion(
    id: &'static str,
    category: AuditCategory,
    question: &'static str,
    evaluation_mode: EvaluationMode,
) -> UxAuditCriterion {
    UxAuditCriterion {
        id,
        category,
        question,
        severity_if_failed: AuditSeverity::Fail,
        evidence_required: match evaluation_mode {
            EvaluationMode::Functional => {
                "projection path, action id, seam ref, or downstream owner"
            }
            EvaluationMode::Aesthetic => "terminal capture or ANSI artifact plus reference mockup",
            EvaluationMode::Mixed => "functional evidence plus aesthetic artifact",
        },
        evaluation_mode,
    }
}

fn rubrics() -> Vec<UxAuditRubric> {
    vec![
        UxAuditRubric {
            id: "functional",
            title: "Functional IDE Fit",
            evaluation_mode: EvaluationMode::Functional,
            criteria: vec![
                "functional.persona_fit",
                "functional.journey_fit",
                "functional.command_clarity",
                "functional.state_ownership",
                "functional.seam_honesty",
                "functional.degradation",
            ],
        },
        UxAuditRubric {
            id: "aesthetic",
            title: "Fire Horse Aesthetic Fit",
            evaluation_mode: EvaluationMode::Aesthetic,
            criteria: vec![
                "aesthetic.hierarchy",
                "aesthetic.density",
                "aesthetic.balance",
                "aesthetic.tone_color",
                "aesthetic.terminal_craft",
                "aesthetic.reference_fidelity",
                "aesthetic.text_fit",
                "aesthetic.emotional_fit",
            ],
        },
    ]
}

fn must_preserve_for(firehorse_scenario_id: &str) -> Vec<&'static str> {
    match firehorse_scenario_id {
        "firehorse-editing-lens-standard" => vec![
            "source canvas remains the primary visual object",
            "Context Dock diagnostic remains visible",
            "semantic hover/lens cites HostWorkspaceSession::hover",
            "Studio and First-class remain high-end targets",
        ],
        "firehorse-command-lens-standard" => vec![
            "overlay shows filter, rows, preview, and disabled reasons",
            "action ids remain stable",
            "backing source remains legible enough to orient the user",
        ],
        "firehorse-debug-cockpit-standard" => vec![
            "paused state is unmistakable",
            "call stack, locals, watches, and step controls remain grouped",
            "debug semantics remain marked as W080-owned future contract",
        ],
        _ => vec![
            "projection ownership remains one-way",
            "reference artifact and terminal capture stay linked",
            "no production behavior is implied by fixture data",
        ],
    }
}

fn render_commands(scenario_id: &str, viewport: &str) -> Vec<String> {
    vec![
        format!(
            "target/release/oxide-uxlab.exe --suite firehorse --scenario {scenario_id} --viewport {viewport} --once --mockup"
        ),
        format!(
            "target/release/oxide-uxlab.exe --suite firehorse --scenario {scenario_id} --viewport {viewport} --once --mockup --ansi"
        ),
    ]
}

fn evaluation_commands(scenario_id: &str, viewport: &str) -> Vec<String> {
    vec![
        format!(
            "target/release/oxide-uxlab.exe --audit --suite firehorse --scenario {scenario_id} --viewport {viewport} --once --json"
        ),
        format!(
            "target/release/oxide-uxlab.exe --audit --suite firehorse --scenario {scenario_id} --viewport {viewport} --evaluate functional,aesthetic --json"
        ),
    ]
}

fn surface(
    surface: &'static str,
    projection_path: &'static str,
    visible_contract: &'static str,
    owner_workset: &'static str,
) -> UxSurfaceExpectation {
    UxSurfaceExpectation {
        surface,
        projection_path,
        visible_contract,
        owner_workset,
    }
}

fn seam(source: &'static str, status: SeamStatus, downstream_workset: &'static str) -> UxSeamRef {
    UxSeamRef {
        source,
        status,
        downstream_workset,
    }
}

fn doc_ref(path: &'static str, title: &'static str) -> AuditArtifactRef {
    artifact("doc", path, title, true)
}

fn image_ref(path: &'static str, title: &'static str) -> AuditArtifactRef {
    artifact("image", path, title, true)
}

fn capture_ref(path: &'static str, title: &'static str) -> AuditArtifactRef {
    artifact("terminal_capture", path, title, true)
}

fn artifact(
    kind: &'static str,
    path: &'static str,
    title: &'static str,
    authority: bool,
) -> AuditArtifactRef {
    AuditArtifactRef {
        kind,
        path,
        title,
        authority,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shell::uxlab::firehorse::FIRE_HORSE_SCENARIOS;

    #[test]
    fn firehorse_audit_suite_covers_every_firehorse_scenario() {
        let suite = firehorse_audit_suite();

        for scenario in FIRE_HORSE_SCENARIOS {
            assert!(
                suite.find_scenario(scenario.id).is_some(),
                "missing audit scenario for {}",
                scenario.id
            );
        }
    }

    #[test]
    fn every_audit_scenario_has_persona_and_artifacts() {
        let suite = firehorse_audit_suite();

        for scenario in &suite.scenarios {
            assert!(
                suite.find_persona(scenario.persona_id).is_some(),
                "{} references unknown persona {}",
                scenario.id,
                scenario.persona_id
            );
            assert!(!scenario.reference_artifacts.is_empty());
            assert!(!scenario.steps.is_empty());
            assert!(!scenario.steps[0].expected_surfaces.is_empty());
        }
    }

    #[test]
    fn every_criterion_declares_evaluation_mode() {
        let suite = firehorse_audit_suite();

        assert!(
            suite
                .criteria
                .iter()
                .any(|criterion| criterion.evaluation_mode == EvaluationMode::Functional)
        );
        assert!(
            suite
                .criteria
                .iter()
                .any(|criterion| criterion.evaluation_mode == EvaluationMode::Aesthetic)
        );
    }

    #[test]
    fn design_brief_contains_agent_commands_and_preservation_contracts() {
        let suite = firehorse_audit_suite();
        let brief = design_brief_for(&suite, "firehorse-editing-lens-standard", "studio")
            .expect("editing brief should exist");

        assert_eq!(brief.scenario_id, "audit-editing-lens-pricing");
        assert!(
            brief
                .must_preserve
                .contains(&"source canvas remains the primary visual object")
        );
        assert!(
            brief
                .render_commands
                .iter()
                .any(|command| command.contains("--mockup"))
        );
        assert!(
            brief
                .evaluation_commands
                .iter()
                .any(|command| command.contains("--once --json"))
        );
    }

    #[test]
    fn matrix_rows_include_downstream_worksets() {
        let suite = firehorse_audit_suite();
        let rows = matrix_rows(&suite);
        let editing = rows
            .iter()
            .find(|row| row.firehorse_scenario_id == "firehorse-editing-lens-standard")
            .expect("editing row");

        assert!(editing.downstream_worksets.contains(&"W060"));
        assert!(editing.criteria.contains(&"functional.seam_honesty"));
    }
}
