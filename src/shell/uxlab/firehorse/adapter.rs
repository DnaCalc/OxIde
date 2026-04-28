//! Read-only adapters from current shell state into the Fire Horse proof model.

use std::path::PathBuf;

use super::projection::{
    ActionHintProjection, ActivityDeckProjection, ActivityKind, ActivityRowProjection,
    ActivityTabProjection, BadgeTone, CodeCanvasProjection, ContextCardProjection,
    ContextDockProjection, CursorProjection, DiagnosticProvenanceProjection, DiagnosticSeverity,
    FireHorseProjection, IdentityRailProjection, KeyBindingProjection, KeyRailProjection,
    LayoutPosture, MockDiagnosticProjection, MockSymbolProjection, ProjectItemKind, ProjectSeamRef,
    ProjectSpineProjection, ProjectSpineRowProjection, SeamFixtureSet, SourceLineProjection,
    SourcePositionProjection, SourceRangeProjection, SpinePosture, StateBadgeProjection,
    SymbolKind, SymbolProvenanceProjection, ThemeProjection, UnavailableProjection,
};
use crate::shell::session::ProjectSession;
use crate::shell::state::{
    BufferKind, CursorPosition, ShellScene, ShellState, WorkspaceProjectModuleKind,
    WorkspaceProjectReferenceKind, WorkspaceState,
};

pub const REAL_EDITING_SCENARIO_ID: &str = "firehorse-real-editing";

impl FireHorseProjection {
    pub fn from_shell_state(state: &ShellState) -> Self {
        let workspace = &state.runtime.workspace;
        let active_view = workspace.active_view();
        let active_buffer = workspace.active_buffer();
        let active_title = active_buffer
            .map(|buffer| buffer.title.as_str())
            .unwrap_or("No active buffer");
        let layout = layout_for_shell_scene(state.scene);
        let diagnostics = diagnostic_rows(workspace, active_title);
        let symbols = symbol_rows(workspace, active_title);
        let references = workspace
            .semantic
            .as_ref()
            .map(|semantic| semantic.references.len())
            .unwrap_or_default();

        Self {
            scenario_id: REAL_EDITING_SCENARIO_ID,
            expected_layout: LayoutPosture::Editing,
            identity: IdentityRailProjection {
                product: "OxIde",
                workspace_label: workspace
                    .project_name
                    .clone()
                    .unwrap_or_else(|| "Project unavailable".to_string()),
                scene: layout,
                target: Some(workspace.target_name.clone()),
                health: vec![StateBadgeProjection {
                    label: "Read-only adapter".to_string(),
                    tone: BadgeTone::Info,
                }],
                cursor: active_view.map(|view| CursorProjection {
                    line: view.surface.cursor.line as u32,
                    column: view.surface.cursor.column as u32,
                }),
            },
            project_spine: project_spine_from_workspace(workspace),
            code_canvas: code_canvas_from_workspace(workspace),
            context_dock: Some(ContextDockProjection {
                title: "Adapter Context".to_string(),
                cards: unavailable_seam_cards(),
            }),
            activity_deck: ActivityDeckProjection {
                posture: super::projection::DeckPosture::Expanded,
                active: ActivityKind::Problems,
                tabs: vec![
                    ActivityTabProjection {
                        kind: ActivityKind::Problems,
                        label: "Problems".to_string(),
                        count: Some(diagnostics.len() as u32),
                    },
                    ActivityTabProjection {
                        kind: ActivityKind::Output,
                        label: "Output".to_string(),
                        count: Some(0),
                    },
                    ActivityTabProjection {
                        kind: ActivityKind::References,
                        label: "References".to_string(),
                        count: Some(references as u32),
                    },
                ],
                rows: activity_rows_from_semantics(workspace),
            },
            key_rail: key_rail(&[
                action("Save", "Ctrl+S", "editor.save"),
                action("Command Lens", "F6", "command.lens.open"),
                action("Semantic Lens", "F1", "semantic.hover"),
                action("Run", "F5", "run.start"),
                action("Quick Fix", "Ctrl+.", "diagnostic.quick_fix"),
            ]),
            overlay: None,
            theme: ThemeProjection::GraphiteEmber,
            terminal_fit: None,
            layout,
            seams: SeamFixtureSet {
                diagnostics,
                symbols,
                run_events: vec![],
                debug_frames: vec![],
                locals: vec![],
                watches: vec![],
            },
        }
    }
}

pub fn thin_slice_editing_projection() -> FireHorseProjection {
    FireHorseProjection::from_shell_state(&thin_slice_shell_state())
}

pub fn thin_slice_shell_state() -> ShellState {
    let project_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("thin-slice")
        .join("ThinSliceHello.basproj");
    let session = ProjectSession::load(project_path)
        .expect("checked-in thin-slice project should load for the UX lab adapter");
    let mut state = ShellState::default();
    state.mount_workspace(session.workspace_state());
    state.apply_scene(ShellScene::Editing);
    state
}

fn layout_for_shell_scene(scene: ShellScene) -> LayoutPosture {
    match scene {
        ShellScene::BuildRun => LayoutPosture::RunLane,
        ShellScene::Palette => LayoutPosture::CommandLens,
        _ => LayoutPosture::Editing,
    }
}

fn project_spine_from_workspace(workspace: &WorkspaceState) -> Option<ProjectSpineProjection> {
    let project = workspace.project.as_ref()?;
    let active_title = workspace
        .active_buffer()
        .map(|buffer| buffer.title.as_str());
    let project_label = workspace
        .project_name
        .clone()
        .unwrap_or_else(|| "Project".to_string());
    let mut rows = Vec::new();

    rows.push(ProjectSpineRowProjection {
        label: project_label.clone(),
        kind: ProjectItemKind::Project,
        depth: 0,
        active: false,
        dirty: workspace.buffers.iter().any(|buffer| buffer.dirty),
        badges: vec![StateBadgeProjection {
            label: project.workspace_kind.label().to_string(),
            tone: BadgeTone::Info,
        }],
        seam_ref: Some(ProjectSeamRef {
            project_id: project_label.clone(),
            item_id: project_label,
        }),
    });

    for module in &project.modules {
        let active = active_title.is_some_and(|title| title == module.include);
        rows.push(ProjectSpineRowProjection {
            label: module.include.clone(),
            kind: match module.kind {
                WorkspaceProjectModuleKind::Module => ProjectItemKind::Module,
                WorkspaceProjectModuleKind::Class => ProjectItemKind::Class,
                WorkspaceProjectModuleKind::Document => ProjectItemKind::Form,
            },
            depth: 1,
            active,
            dirty: workspace
                .buffers
                .iter()
                .any(|buffer| buffer.title == module.include && buffer.dirty),
            badges: if active {
                vec![StateBadgeProjection {
                    label: "active".to_string(),
                    tone: BadgeTone::Info,
                }]
            } else {
                vec![]
            },
            seam_ref: Some(ProjectSeamRef {
                project_id: workspace
                    .project_name
                    .clone()
                    .unwrap_or_else(|| "Project".to_string()),
                item_id: module.logical_name.clone(),
            }),
        });
    }

    rows.push(ProjectSpineRowProjection {
        label: workspace.target_name.clone(),
        kind: ProjectItemKind::Target,
        depth: 1,
        active: false,
        dirty: false,
        badges: vec![],
        seam_ref: Some(ProjectSeamRef {
            project_id: workspace
                .project_name
                .clone()
                .unwrap_or_else(|| "Project".to_string()),
            item_id: "target".to_string(),
        }),
    });

    for reference in &project.references {
        rows.push(ProjectSpineRowProjection {
            label: reference_label(reference.kind, &reference.include),
            kind: ProjectItemKind::Reference,
            depth: 1,
            active: false,
            dirty: false,
            badges: vec![],
            seam_ref: Some(ProjectSeamRef {
                project_id: workspace
                    .project_name
                    .clone()
                    .unwrap_or_else(|| "Project".to_string()),
                item_id: reference.include.clone(),
            }),
        });
    }

    Some(ProjectSpineProjection {
        posture: SpinePosture::Full,
        rows,
    })
}

fn code_canvas_from_workspace(workspace: &WorkspaceState) -> CodeCanvasProjection {
    let active_view = workspace.active_view();
    let active_buffer = workspace.active_buffer();

    CodeCanvasProjection {
        document_label: active_buffer
            .map(|buffer| buffer.title.clone())
            .unwrap_or_else(|| "No active buffer".to_string()),
        language: active_buffer
            .map(|buffer| match buffer.kind {
                BufferKind::Welcome => "workspace-launcher",
                BufferKind::Source | BufferKind::Class => "VBA",
            })
            .unwrap_or("unknown"),
        lines: active_buffer
            .map(|buffer| {
                buffer
                    .lines
                    .iter()
                    .enumerate()
                    .map(|(index, text)| SourceLineProjection {
                        number: (index + 1) as u32,
                        text: text.clone(),
                        markers: vec![],
                        semantic_spans: vec![],
                    })
                    .collect()
            })
            .unwrap_or_default(),
        lens: None,
        execution_line: None,
        selection: active_view.and_then(|view| {
            view.surface
                .selection
                .map(|selection| source_range(selection.anchor, selection.head))
        }),
    }
}

fn activity_rows_from_semantics(workspace: &WorkspaceState) -> Vec<ActivityRowProjection> {
    let Some(semantic) = &workspace.semantic else {
        return vec![unavailable_row(
            "HostWorkspaceSession::diagnostics",
            "semantic data is unavailable on this ShellState",
        )];
    };

    if semantic.diagnostics.is_empty() {
        return vec![unavailable_row(
            "HostWorkspaceSession::diagnostics",
            "no projected diagnostics are available from current ShellState",
        )];
    }

    let mut rows = semantic
        .diagnostics
        .iter()
        .map(|text| ActivityRowProjection::Text {
            source: "WorkspaceSemanticState::diagnostics",
            text: text.clone(),
        })
        .collect::<Vec<_>>();
    rows.push(unavailable_row(
        "HostWorkspaceSession::hover",
        "hover/source-lens rows are not adapted from current ShellState in W039",
    ));
    rows
}

fn diagnostic_rows(
    workspace: &WorkspaceState,
    active_title: &str,
) -> Vec<MockDiagnosticProjection> {
    workspace
        .semantic
        .as_ref()
        .map(|semantic| {
            semantic
                .diagnostics
                .iter()
                .map(|message| MockDiagnosticProjection {
                    document_id: format!("shell://workspace/{active_title}"),
                    range: source_range(CursorPosition::new(1, 1), CursorPosition::new(1, 1)),
                    severity: DiagnosticSeverity::Info,
                    code: "SHELL-SEMANTIC".to_string(),
                    message: message.clone(),
                    provenance: DiagnosticProvenanceProjection {
                        provider: "WorkspaceSemanticState::diagnostics",
                        project_id: workspace
                            .project_name
                            .clone()
                            .unwrap_or_else(|| "Project unavailable".to_string()),
                    },
                })
                .collect()
        })
        .unwrap_or_default()
}

fn symbol_rows(workspace: &WorkspaceState, active_title: &str) -> Vec<MockSymbolProjection> {
    workspace
        .semantic
        .as_ref()
        .map(|semantic| {
            semantic
                .symbols
                .iter()
                .map(|name| MockSymbolProjection {
                    document_id: format!("shell://workspace/{active_title}"),
                    range: source_range(CursorPosition::new(1, 1), CursorPosition::new(1, 1)),
                    kind: SymbolKind::Procedure,
                    name: name.clone(),
                    detail: name.clone(),
                    provenance: SymbolProvenanceProjection {
                        provider: "WorkspaceSemanticState::symbols",
                        document_id: format!("shell://workspace/{active_title}"),
                    },
                })
                .collect()
        })
        .unwrap_or_default()
}

fn unavailable_seam_cards() -> Vec<ContextCardProjection> {
    vec![
        ContextCardProjection::Unavailable(UnavailableProjection {
            source: "HostWorkspaceSession::hover",
            reason: "Unavailable seam: hover is not adapted from current ShellState in W039.",
        }),
        ContextCardProjection::Unavailable(UnavailableProjection {
            source: "HostWorkspaceSession::references",
            reason: "Unavailable seam: references remain an explicit W060 handoff.",
        }),
        ContextCardProjection::Unavailable(UnavailableProjection {
            source: "ActionRegistry",
            reason: "Unavailable seam: action dispatch remains W090-owned.",
        }),
    ]
}

fn unavailable_row(source: &'static str, reason: &str) -> ActivityRowProjection {
    ActivityRowProjection::Text {
        source,
        text: format!("Unavailable seam | {source} | {reason}"),
    }
}

fn reference_label(kind: WorkspaceProjectReferenceKind, include: &str) -> String {
    format!("{} {}", kind.label(), include)
}

fn source_range(anchor: CursorPosition, head: CursorPosition) -> SourceRangeProjection {
    SourceRangeProjection {
        start: SourcePositionProjection {
            line: anchor.line as u32,
            column: anchor.column as u32,
        },
        end: SourcePositionProjection {
            line: head.line as u32,
            column: head.column as u32,
        },
    }
}

fn key_rail(hints: &[ActionHintProjection]) -> KeyRailProjection {
    KeyRailProjection {
        hints: hints.to_vec(),
        no_wrap: true,
    }
}

fn action(label: &str, binding: &str, action_id: &'static str) -> ActionHintProjection {
    ActionHintProjection {
        label: label.to_string(),
        binding: KeyBindingProjection {
            label: binding.to_string(),
        },
        action_id,
        enabled: true,
        disabled_reason: None,
        display_only_reason: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_state_adapter_populates_required_editing_surfaces() {
        let state = thin_slice_shell_state();
        let projection = FireHorseProjection::from_shell_state(&state);

        assert_eq!(projection.scenario_id, REAL_EDITING_SCENARIO_ID);
        assert_eq!(projection.identity.workspace_label, "ThinSliceHello");
        assert_eq!(projection.code_canvas.document_label, "Module1.bas");
        assert!(
            projection
                .project_spine
                .as_ref()
                .is_some_and(|spine| spine.rows.iter().any(|row| row.label == "Module1.bas"))
        );
        assert!(!projection.code_canvas.lines.is_empty());
        assert!(!projection.activity_deck.rows.is_empty());
        assert!(!projection.key_rail.hints.is_empty());
    }

    #[test]
    fn shell_state_adapter_marks_missing_seams_explicitly() {
        let state = thin_slice_shell_state();
        let projection = FireHorseProjection::from_shell_state(&state);
        let context = projection.context_dock.expect("context dock");

        assert!(context.cards.iter().any(|card| matches!(
            card,
            ContextCardProjection::Unavailable(unavailable)
                if unavailable.reason.contains("Unavailable seam")
        )));
        assert!(projection.activity_deck.rows.iter().any(|row| matches!(
            row,
            ActivityRowProjection::Text { text, .. } if text.contains("Unavailable seam")
        )));
    }

    #[test]
    fn shell_state_adapter_is_read_only() {
        let state = thin_slice_shell_state();
        let before = state.clone();

        let _projection = FireHorseProjection::from_shell_state(&state);

        assert_eq!(state, before);
    }
}
