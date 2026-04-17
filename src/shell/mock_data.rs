use super::state::{
    FocusRegion, PaletteCommandState, PanelContentState, ShellScene, ShellState, WorkspaceState,
};

/// User-facing scene label for the top bar. Distinct from `ShellScene`'s
/// internal `Debug` name (which leaks `Semantic`, `BuildRun`, etc.) — the
/// rule from uxpass P10 / D5 is that internal taxonomy does not appear on
/// user surfaces.
fn scene_label(scene: ShellScene) -> &'static str {
    match scene {
        ShellScene::Empty => "Empty",
        ShellScene::Editing => "Editing",
        ShellScene::Semantic => "Semantic",
        ShellScene::BuildRun => "Run",
        ShellScene::Palette => "Palette",
        ShellScene::ComReference => "COM reference",
    }
}

pub struct ShellPanels {
    pub top_bar: String,
    pub explorer: String,
    pub editor_title: String,
    pub editor: String,
    pub inspector: String,
    pub lower_surface: String,
    pub palette: String,
    pub com_reference_helper: String,
}

pub fn shell_panels(state: &ShellState) -> ShellPanels {
    ShellPanels {
        top_bar: top_bar_text(state),
        explorer: explorer_text(state),
        editor_title: editor_title(state),
        editor: editor_text(state),
        inspector: inspector_text(state),
        lower_surface: lower_surface_text(state),
        palette: palette_text(state),
        com_reference_helper: com_reference_helper_text(state),
    }
}

/// Build the top-bar status line.
///
/// Uxpass D2 cap: at most three fields. Order is always
/// `<project> | <scene> [| <state>]`. Focus region, width class,
/// view/buffer counts, and theme/palette names are all dev telemetry and
/// MUST NOT appear here (P1 / P3 / D4 / D5). Project `target` (Exe,
/// Library, ...) is project metadata and belongs in the Explorer, not the
/// top bar — it is dropped from every scene.
///
/// The `<state>` field, when present, is the single piece of data that is
/// most relevant to the current scene:
/// - `Editing` / `Semantic`: cursor position (`Ln L Col C`).
/// - `Run`: composite build+runtime status (`passing / completed`).
/// - `Empty` / `Palette` / `COM reference`: omitted; the Welcome surface or
///   overlay carries its own state and adding a third field would be noise.
fn top_bar_text(state: &ShellState) -> String {
    let workspace = &state.runtime.workspace;
    let project = workspace.project_name.as_deref().unwrap_or("No project");
    let scene = scene_label(state.scene);

    let cursor = workspace.active_view().map(|view| {
        format!(
            "Ln {} Col {}",
            view.surface.cursor.line, view.surface.cursor.column
        )
    });

    match state.scene {
        ShellScene::Empty | ShellScene::Palette | ShellScene::ComReference => {
            format!("{project} | {scene}")
        }
        ShellScene::Editing | ShellScene::Semantic => match cursor {
            Some(cursor) => format!("{project} | {scene} | {cursor}"),
            None => format!("{project} | {scene}"),
        },
        ShellScene::BuildRun => format!(
            "{project} | {scene} | {} / {}",
            state.runtime.execution.build_status,
            state.runtime.execution.runtime_status
        ),
    }
}

fn explorer_text(state: &ShellState) -> String {
    // Uxpass D1b: Empty no longer renders an Explorer (Launcher) column.
    // The Welcome surface owns the launcher role now, so explorer_text is
    // only consulted for non-Empty scenes. Kept defensively so callers
    // that cache `panels.explorer` unconditionally still get a harmless
    // empty string on Empty.
    match state.scene {
        ShellScene::Empty => String::new(),
        _ => project_explorer_text(&state.runtime.workspace),
    }
}

/// Compose the Editor panel title.
///
/// Uxpass J2-d (P4 — honest about state): when the active buffer is
/// dirty, a trailing ` *` is appended so the title carries the same
/// "uncommitted change" signal the Explorer's `Open Buffers` list
/// already uses (`project_explorer_text` already appends ` *` per
/// buffer). Without this, a user could type into the editor, look at
/// the title, and see nothing indicating the file on disk no longer
/// matches the buffer.
fn editor_title(state: &ShellState) -> String {
    match state.scene {
        ShellScene::Empty => String::from("Welcome"),
        _ => {
            let workspace = &state.runtime.workspace;
            let active_view = workspace.active_view();
            let active_buffer = workspace.active_buffer();
            match (active_buffer, active_view) {
                (Some(buffer), Some(view)) => {
                    let dirty = if buffer.dirty { " *" } else { "" };
                    format!("{}{dirty} | {} View", buffer.title, view.kind.label())
                }
                _ => String::from("Editor"),
            }
        }
    }
}

fn editor_text(state: &ShellState) -> String {
    match state.scene {
        ShellScene::Empty => launcher_editor_text(state),
        _ => editor_buffer_text(state),
    }
}

/// Render the Inspector column.
///
/// The Inspector is a user surface (P1): every section that lives here must
/// be actionable, explanatory, or diagnostic to the author. The previous
/// `Tokens` hex-code dump on the Empty scene was dev telemetry (P1 / D4) and
/// has been removed. The sub-panes now come entirely from
/// `state::content_for_scene`, which D5 also trims.
fn inspector_text(state: &ShellState) -> String {
    format_panel_content(&state.runtime.content.inspector)
}

fn lower_surface_text(state: &ShellState) -> String {
    let mut base = format_panel_content(&state.runtime.content.lower_surface);
    if matches!(state.scene, ShellScene::Palette | ShellScene::ComReference) {
        if !base.is_empty() {
            base.push('\n');
            base.push('\n');
        }
        if state.scene == ShellScene::Palette {
            base.push_str("Palette\n");
            base.push_str("  Background shell is frozen while the palette is active.");
        } else {
            base.push_str("COM Reference Helper\n");
            base.push_str("  Background shell is frozen while the COM reference helper is active.");
        }
    }

    if state.inspector_is_collapsed() && !matches!(state.scene, ShellScene::Empty) {
        if !base.is_empty() {
            base.push('\n');
            base.push('\n');
        }
        base.push_str("Collapsed Inspector\n");
        base.push_str(&indent_block(&inspector_text(state), 2));
    }

    base
}

fn palette_text(state: &ShellState) -> String {
    let palette = &state.runtime.content.palette;
    let current_focus_owner = match state.runtime.focus {
        FocusRegion::Palette => "Palette",
        _ => "Shell",
    };

    let mut text = format!(
        "Command Palette\n\nFilter\n  > {}\n\nCommands\n",
        palette.filter_hint
    );
    // Paint a `>` marker on the currently-selected command row so
    // Up / Down selection and the Enter-dispatch handler give the
    // user visible, predictable feedback. Identical marker semantics
    // to the COM reference helper (`com_reference_helper_text`) and
    // the Empty-scene launcher (`launcher_editor_text`).
    let selection = state.runtime.palette_selection;
    for (index, command) in palette.commands.iter().enumerate() {
        let marker = if index == selection { '>' } else { ' ' };
        text.push_str(&format_palette_command(command, marker));
    }

    // `state_commands` is empty in the default build (uxpass D6). The dev
    // build (--dev-scenes) repopulates it; only then emit the group header.
    // The Mockup States group has no selection of its own — it's a
    // reference list for dev-mode preview keys — so every row renders
    // with a space marker.
    if !palette.state_commands.is_empty() {
        text.push_str("\nMockup States\n");
        for command in &palette.state_commands {
            text.push_str(&format_palette_command(command, ' '));
        }
    }

    text.push_str(&format!("\nCurrent Focus Owner\n  {current_focus_owner}\n"));
    text
}

fn com_reference_helper_text(state: &ShellState) -> String {
    let helper = &state.runtime.com_reference_helper;
    let mut text = format!(
        "COM Reference Helper\n\nMode\n  {}\n\nQuery\n  > {}\n\nCandidates\n",
        helper.mode.label(),
        helper.query
    );

    if helper.candidates.is_empty() {
        text.push_str("  No candidates\n");
    } else {
        for (index, candidate) in helper.candidates.iter().enumerate() {
            let marker = if index == helper.selection { ">" } else { " " };
            text.push_str(&format!("  {marker} {}\n", candidate.title));
            for line in &candidate.detail_lines {
                text.push_str(&format!("    {line}\n"));
            }
        }
    }

    text.push_str("\nActive References\n");
    if helper.active_reference_lines.is_empty() {
        text.push_str("  No COM references declared\n");
    } else {
        for line in &helper.active_reference_lines {
            text.push_str(&format!("  {line}\n"));
        }
    }

    text.push_str("\nStatus\n");
    for line in &helper.status_lines {
        text.push_str(&format!("  {line}\n"));
    }

    text.push_str("\nKeys\n  Enter apply  Tab switch mode  Up/Down select  Esc close\n");
    text
}

/// Render the Explorer column (non-Empty scenes).
///
/// The Explorer is a user surface (P1): it answers "what is this
/// project, what modules does it have, what references, and what is
/// open right now." Four sub-panes, no more. Previously it also
/// carried `Views`, `Target`, and `Layout` sub-panes that echoed the
/// `BasProj | Exe` line and leaked internal `LayoutPreset` /
/// `ViewKind` enum names — dev telemetry under P1, duplication under
/// P2 / P3, internal identifiers under D5. All three are removed; the
/// active visible buffer is still marked with `>` in Open Buffers.
///
/// Path display stays untruncated here; the view layer wraps long
/// lines (D7 — silent mid-word truncation is a defect).
fn project_explorer_text(workspace: &WorkspaceState) -> String {
    let project = workspace.project_name.as_deref().unwrap_or("No project");
    let mut text = format!("Project\n> {project}\n");

    if let Some(project_state) = &workspace.project {
        // `BasProj | Exe` is the canonical identity line; the former
        // `Target` sub-pane below echoed `Exe` and is dropped (P2 / P3).
        text.push_str(&format!(
            "  {} | {}\n",
            project_state.workspace_kind.label(),
            project_state.output_type
        ));
        text.push_str(&format!(
            "  target {}\n",
            project_state.workspace_target.display()
        ));
        if let Some(project_file) = &project_state.project_file {
            text.push_str(&format!("  file {}\n", project_file.display()));
        }

        text.push_str("\nModules\n");
        if project_state.modules.is_empty() {
            text.push_str("  No modules discovered\n");
        } else {
            for module in &project_state.modules {
                // `Module1 [Module]` is the heading; the former
                // `declared Module1` line restated the logical name
                // and is dropped (P2). `declared_name` only matters
                // when it differs from `logical_name`; that case is
                // kept explicit.
                text.push_str(&format!(
                    "> {} [{}]\n",
                    module.logical_name,
                    module.kind.label()
                ));
                text.push_str(&format!("  include {}\n", module.include));
                text.push_str(&format!("  source {}\n", module.source_path.display()));
                if let Some(declared_name) = &module.declared_name {
                    if declared_name != &module.logical_name {
                        text.push_str(&format!("  declared {declared_name}\n"));
                    }
                }
            }
        }

        text.push_str("\nReferences\n");
        if project_state.references.is_empty() {
            text.push_str("  No references declared\n");
        } else {
            for reference in &project_state.references {
                text.push_str(&format!(
                    "> {} [{}]\n",
                    reference.include,
                    reference.kind.label()
                ));
                if let Some(project_name) = &reference.referenced_project_name {
                    text.push_str(&format!("  project {project_name}\n"));
                }
                if let Some(path) = &reference.path {
                    text.push_str(&format!("  path {path}\n"));
                }
                if let Some(guid) = &reference.guid {
                    text.push_str(&format!("  guid {guid}\n"));
                }
                if let Some(import_lib) = &reference.import_lib {
                    text.push_str(&format!("  import {import_lib}\n"));
                }
            }
        }
    }

    // Open Buffers: `>` marks a buffer whose content is mounted into
    // a visible view; absent marker means the buffer is held open but
    // not currently shown. The former `[Source:view]` / `[Source:open]`
    // suffix leaked `BufferKind::Source` taxonomy (P1 / D5) and
    // duplicated what the marker already conveys (P3); removed.
    // Dirty state is surfaced with a trailing `*` (unchanged).
    text.push_str("\nOpen Buffers\n");
    for buffer in &workspace.buffers {
        let visible = workspace
            .visible_views()
            .iter()
            .any(|view| view.buffer_id == buffer.id);
        let dirty = if buffer.dirty { " *" } else { "" };
        let marker = if visible { ">" } else { " " };
        text.push_str(&format!("{marker} {}{dirty}\n", buffer.title));
    }

    text
}

/// Render the Welcome panel that covers the Empty scene body.
///
/// Uxpass D1b (Welcome IS the launcher): the Welcome panel is now a
/// single full-width surface that owns both the recent-projects
/// selection (with `>` marker driven by `launcher_selection`) and the
/// Start command list. Two prior surfaces were retired here:
/// - The separate `Launcher` column on the left is gone; its content is
///   now the `Recent` block below.
/// - The trailing `Hint` paragraph (`Ctrl+O open selected  …`) is gone;
///   that affordance now lives in the always-present status line (D3 /
///   D8) and repeating it here would violate P2 / P3.
/// The `Capability` block (`Truecolor detected`, …) is also retired:
/// capability onboarding is tracked as the W100 workset (first-run
/// probe page / degradation policy); a static three-line list on the
/// Empty body was informational, not actionable (P1 / P3).
fn launcher_editor_text(state: &ShellState) -> String {
    let launcher = &state.runtime.content.launcher;
    let mut text = String::from("OxIde\n\nA terminal-native IDE for OxVba.\n\nRecent\n");

    if launcher.recent_projects.is_empty() {
        text.push_str("  No discovered .basproj files\n");
    } else {
        for project in &launcher.recent_projects {
            // `launcher.recent_projects` already carries the `> ` / `  `
            // selection prefix for each row (see
            // `content_for_scene`). Emit them verbatim.
            text.push_str(&format!("{project}\n"));
        }
    }

    text.push_str("\nStart\n");
    for action in &launcher.actions {
        // Start commands are informational; the status line announces
        // the keystroke that actually triggers them. No in-body
        // selection marker, to avoid suggesting a second selection
        // cursor competing with Recent.
        text.push_str(&format!("  {action}\n"));
    }
    text
}

fn editor_buffer_text(state: &ShellState) -> String {
    state
        .runtime
        .workspace
        .active_buffer()
        .map(|buffer| buffer.lines.join("\n"))
        .unwrap_or_else(|| String::from("No buffer mounted."))
}

fn format_panel_content(content: &PanelContentState) -> String {
    let mut parts = Vec::new();
    for section in &content.sections {
        let mut block = String::from(section.title);
        for line in &section.lines {
            block.push('\n');
            block.push_str(line);
        }
        parts.push(block);
    }
    parts.join("\n\n")
}

fn format_palette_command(command: &PaletteCommandState, marker: char) -> String {
    format!("{marker} {:<24}{}\n", command.label, command.shortcut)
}

fn indent_block(text: &str, spaces: usize) -> String {
    let pad = " ".repeat(spaces);
    text.lines()
        .map(|line| format!("{pad}{line}"))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::shell::state::{
        BufferHistory, BufferId, BufferKind, BufferState, CursorPosition, EditorSurfaceState,
        LayoutPreset, LineEnding, ViewId, ViewKind, ViewState, WorkspaceLayoutState,
        WorkspaceProjectModuleKind, WorkspaceProjectModuleState, WorkspaceProjectReferenceKind,
        WorkspaceProjectReferenceState, WorkspaceProjectState, WorkspaceProjectTargetKind,
    };

    #[test]
    fn explorer_text_prefers_real_workspace_project_structure() {
        let workspace = WorkspaceState {
            project_name: Some(String::from("App")),
            target_name: String::from("Exe"),
            project: Some(WorkspaceProjectState {
                workspace_kind: WorkspaceProjectTargetKind::BasProj,
                workspace_target: PathBuf::from("examples/thin-slice/ThinSliceHello.basproj"),
                project_file: Some(PathBuf::from("examples/thin-slice/ThinSliceHello.basproj")),
                project_dir: PathBuf::from("examples/thin-slice"),
                output_type: String::from("Exe"),
                modules: vec![WorkspaceProjectModuleState {
                    kind: WorkspaceProjectModuleKind::Module,
                    include: String::from("Module1.bas"),
                    source_path: PathBuf::from("examples/thin-slice/Module1.bas"),
                    logical_name: String::from("Module1"),
                    declared_name: None,
                }],
                references: vec![WorkspaceProjectReferenceState {
                    kind: WorkspaceProjectReferenceKind::Com,
                    include: String::from("Scripting"),
                    referenced_project_name: None,
                    path: None,
                    guid: Some(String::from("{420B2830-E718-11CF-893D-00A0C9054228}")),
                    import_lib: Some(String::from("scrrun.dll")),
                }],
            }),
            buffers: vec![BufferState {
                id: BufferId(1),
                title: String::from("Module1.bas"),
                kind: BufferKind::Source,
                dirty: false,
                lines: vec![String::from("Option Explicit")],
                source_path: None,
                line_ending: LineEnding::Lf,
                trailing_newline: false,
                history: BufferHistory::new(),
            }],
            recent_buffers: vec![BufferId(1)],
            views: vec![ViewState {
                id: ViewId(1),
                buffer_id: BufferId(1),
                kind: ViewKind::Primary,
                surface: EditorSurfaceState {
                    cursor: CursorPosition::new(1, 1),
                    selection: None,
                    scroll_top: 0,
                },
            }],
            layout: WorkspaceLayoutState {
                preset: LayoutPreset::Edit,
                visible_views: vec![ViewId(1)],
                active_view: ViewId(1),
            },
            semantic: None,
        };

        let text = project_explorer_text(&workspace);

        assert!(text.contains("BasProj | Exe"));
        assert!(text.contains("Modules"));
        assert!(text.contains("> Module1 [Module]"));
        assert!(text.contains("References"));
        assert!(text.contains("> Scripting [COM]"));
    }

    // ------------------------------------------------------------------
    // Explorer slim sweep (uxpass P1 / P2 / P3 / D5).
    //
    // The Explorer used to carry three additional sub-panes — `Layout`,
    // `Views`, `Target` — whose content was either internal taxonomy
    // (LayoutPreset / ViewKind enum names, leaked as user-facing labels
    // under D5), direct duplication of data rendered elsewhere (Target
    // echoing `BasProj | Exe`, P2 / P3), or restating the logical
    // module name (`declared Module1` line, P2). All three are removed;
    // the active visible buffer is still marked with `>`, dirty state
    // is still marked with `*`, and paths are rendered untruncated (D7
    // handles fit via wrap mode in the view layer).
    // ------------------------------------------------------------------

    /// The Explorer must not surface any of the dev-telemetry sub-pane
    /// headings or internal-taxonomy tags that used to live here.
    fn assert_explorer_is_slim(text: &str) {
        for banned in [
            // Sub-pane headings that were dev-only and have been
            // removed entirely.
            "\nLayout\n",
            "\nViews\n",
            "\nTarget\n",
            // Internal LayoutPreset names leaking to the user surface.
            "Preset: ",
            "SplitEdit",
            // `[Source:view]` / `[Source:open]` taxonomy that leaked
            // `BufferKind::Source` and duplicated the `>` marker.
            "[Source:view]",
            "[Source:open]",
            // The `> Primary view -> Module1.bas` echo used to sit
            // under the `Views` sub-pane; `Primary view` is the
            // ViewKind::Primary enum name, not something the user
            // types or cares about.
            "Primary view",
            "Secondary view",
        ] {
            assert!(
                !text.contains(banned),
                "explorer leaked banned token {banned:?}:\n{text}"
            );
        }
    }

    #[test]
    fn explorer_drops_layout_views_and_target_subpanes() {
        let workspace = WorkspaceState {
            project_name: Some(String::from("ThinSliceHello")),
            target_name: String::from("Exe"),
            project: Some(WorkspaceProjectState {
                workspace_kind: WorkspaceProjectTargetKind::BasProj,
                workspace_target: PathBuf::from(
                    "examples/thin-slice/ThinSliceHello.basproj",
                ),
                project_file: Some(PathBuf::from(
                    "examples/thin-slice/ThinSliceHello.basproj",
                )),
                project_dir: PathBuf::from("examples/thin-slice"),
                output_type: String::from("Exe"),
                modules: vec![WorkspaceProjectModuleState {
                    kind: WorkspaceProjectModuleKind::Module,
                    include: String::from("Module1.bas"),
                    source_path: PathBuf::from("examples/thin-slice/Module1.bas"),
                    logical_name: String::from("Module1"),
                    declared_name: None,
                }],
                references: vec![],
            }),
            buffers: vec![BufferState {
                id: BufferId(1),
                title: String::from("Module1.bas"),
                kind: BufferKind::Source,
                dirty: false,
                lines: vec![String::from("Option Explicit")],
                source_path: None,
                line_ending: LineEnding::Lf,
                trailing_newline: false,
                history: BufferHistory::new(),
            }],
            recent_buffers: vec![BufferId(1)],
            views: vec![ViewState {
                id: ViewId(1),
                buffer_id: BufferId(1),
                kind: ViewKind::Primary,
                surface: EditorSurfaceState {
                    cursor: CursorPosition::new(1, 1),
                    selection: None,
                    scroll_top: 0,
                },
            }],
            layout: WorkspaceLayoutState {
                preset: LayoutPreset::Edit,
                visible_views: vec![ViewId(1)],
                active_view: ViewId(1),
            },
            semantic: None,
        };

        let text = project_explorer_text(&workspace);

        assert_explorer_is_slim(&text);
        assert!(
            text.contains("\nOpen Buffers\n"),
            "Open Buffers sub-pane survives the slim sweep: {text:?}"
        );
        // The `>` marker on a visible buffer survives; `[Source:view]`
        // was its dev-telemetry suffix and is gone.
        assert!(
            text.contains("> Module1.bas\n"),
            "visible buffer must keep its `>` marker, got: {text:?}"
        );
    }

    /// The `declared <name>` line only belongs in the Explorer when
    /// the declared `Attribute VB_Name` differs from the logical name;
    /// otherwise it restates the heading (P2 duplication).
    #[test]
    fn explorer_drops_redundant_declared_name_line() {
        let workspace = WorkspaceState {
            project_name: Some(String::from("ThinSliceHello")),
            target_name: String::from("Exe"),
            project: Some(WorkspaceProjectState {
                workspace_kind: WorkspaceProjectTargetKind::BasProj,
                workspace_target: PathBuf::from("thin.basproj"),
                project_file: Some(PathBuf::from("thin.basproj")),
                project_dir: PathBuf::from("."),
                output_type: String::from("Exe"),
                modules: vec![WorkspaceProjectModuleState {
                    kind: WorkspaceProjectModuleKind::Module,
                    include: String::from("Module1.bas"),
                    source_path: PathBuf::from("Module1.bas"),
                    logical_name: String::from("Module1"),
                    declared_name: Some(String::from("Module1")),
                }],
                references: vec![],
            }),
            buffers: vec![],
            recent_buffers: vec![],
            views: vec![],
            layout: WorkspaceLayoutState {
                preset: LayoutPreset::Edit,
                visible_views: vec![],
                active_view: ViewId(0),
            },
            semantic: None,
        };

        let text = project_explorer_text(&workspace);

        assert_explorer_is_slim(&text);
        assert!(
            !text.contains("declared Module1"),
            "`declared` line must be suppressed when equal to logical name, got: {text:?}"
        );
    }

    /// When the declared name actually differs from the logical name,
    /// surface it — the user needs to see the mismatch.
    #[test]
    fn explorer_surfaces_declared_name_when_it_differs_from_logical() {
        let workspace = WorkspaceState {
            project_name: Some(String::from("ThinSliceHello")),
            target_name: String::from("Exe"),
            project: Some(WorkspaceProjectState {
                workspace_kind: WorkspaceProjectTargetKind::BasProj,
                workspace_target: PathBuf::from("thin.basproj"),
                project_file: Some(PathBuf::from("thin.basproj")),
                project_dir: PathBuf::from("."),
                output_type: String::from("Exe"),
                modules: vec![WorkspaceProjectModuleState {
                    kind: WorkspaceProjectModuleKind::Module,
                    include: String::from("Module1.bas"),
                    source_path: PathBuf::from("Module1.bas"),
                    logical_name: String::from("Module1"),
                    declared_name: Some(String::from("RenamedModule")),
                }],
                references: vec![],
            }),
            buffers: vec![],
            recent_buffers: vec![],
            views: vec![],
            layout: WorkspaceLayoutState {
                preset: LayoutPreset::Edit,
                visible_views: vec![],
                active_view: ViewId(0),
            },
            semantic: None,
        };

        let text = project_explorer_text(&workspace);

        assert!(
            text.contains("declared RenamedModule"),
            "`declared` line must appear when declared name diverges, got: {text:?}"
        );
    }

    // ------------------------------------------------------------------
    // Welcome-as-launcher (uxpass D1b).
    //
    // Empty renders a single full-width Welcome panel. It must carry
    // the Recent projects list (with `>` selection marker) AND the
    // Start action list, and it must NOT carry the old `Capability`
    // or `Hint` paragraphs — those were either informational (P1 / P3)
    // or duplicated the status-line hint (P2).
    // ------------------------------------------------------------------

    #[test]
    fn welcome_owns_the_launcher_role_on_empty() {
        let state = state_in_scene(ShellScene::Empty);
        let panels = shell_panels(&state);

        assert!(
            panels.editor_title == "Welcome",
            "Empty body title must be Welcome, got {:?}",
            panels.editor_title
        );
        assert!(
            panels.editor.contains("\nRecent\n"),
            "Welcome must carry the Recent section (D1b): {:?}",
            panels.editor
        );
        assert!(
            panels.editor.contains("\nStart\n"),
            "Welcome must carry the Start section (D1b): {:?}",
            panels.editor
        );
        // Capability / Hint paragraphs are retired.
        for banned in ["Capability\n", "Hint\n", "Truecolor detected"] {
            assert!(
                !panels.editor.contains(banned),
                "Welcome must not carry {banned:?} anymore: {:?}",
                panels.editor
            );
        }
    }

    #[test]
    fn empty_scene_emits_no_explorer_panel_text() {
        // D1b: the Explorer (Launcher) column does not render on Empty.
        // `explorer_text` returns an empty string so any caller that
        // still reads `panels.explorer` gets a harmless empty body.
        let state = state_in_scene(ShellScene::Empty);
        let panels = shell_panels(&state);

        assert_eq!(
            panels.explorer, "",
            "Empty must not produce explorer text (D1b), got {:?}",
            panels.explorer
        );
    }

    // J2-d / P4 — a clean editor title carries no dirty marker. After
    // an edit, the active buffer flips to dirty and the title gains a
    // trailing ` *` so the user has honest feedback about unsaved work.
    // Welcome (Empty scene) is never dirty by definition; the marker
    // only applies to buffer-backed scenes.
    #[test]
    fn editor_title_gains_dirty_marker_after_edit_and_clears_without_one_otherwise() {
        let mut state = state_in_scene(ShellScene::Editing);

        let clean = shell_panels(&state);
        assert!(
            !clean.editor_title.contains(" *"),
            "clean editor title must not carry dirty marker: {:?}",
            clean.editor_title
        );

        state.runtime.focus = FocusRegion::Editor;
        state.insert_editor_char('X');

        let dirty = shell_panels(&state);
        assert!(
            dirty.editor_title.contains(" *"),
            "dirty editor title must carry ` *` marker (J2-d): {:?}",
            dirty.editor_title
        );

        // Welcome title is scene-fixed; it never carries the dirty marker.
        let empty = state_in_scene(ShellScene::Empty);
        let welcome = shell_panels(&empty);
        assert_eq!(
            welcome.editor_title, "Welcome",
            "Welcome title is scene-fixed and must not carry a dirty marker"
        );
    }

    // ------------------------------------------------------------------
    // Top-bar slim pass (uxpass D2 / D4 / D5).
    // The top bar must carry at most three fields and must not surface
    // focus-region labels, width-class names, theme names, view counts,
    // or truecolor status. Each test below pins one banned string on one
    // representative scene; together they form the regression surface.
    // ------------------------------------------------------------------

    use crate::shell::state::{ExecutionState, FocusRegion, ShellScene, ShellState};

    fn state_in_scene(scene: ShellScene) -> ShellState {
        let mut state = ShellState::default();
        state.apply_scene(scene);
        state
    }

    /// Lightweight assertion helper: the top bar contains none of the
    /// banned tokens, and has at most three `|`-separated fields.
    fn assert_top_bar_is_slim(top_bar: &str) {
        for banned in [
            "Focus",           // D5: internal region label
            "Standard",        // D4: width-class name
            "Wide",
            "Narrow",
            "Truecolor ready", // redundant with Environment pane
            "Views ",          // P1: dev telemetry
            "Hidden ",         // P1: dev telemetry
            "Hover active",    // P1: internal mode label
            "Overlay focus",   // D5: internal focus label
            "Mockup",          // D4: theme-token name
        ] {
            assert!(
                !top_bar.contains(banned),
                "top bar leaked banned token {banned:?}: {top_bar:?}"
            );
        }

        let field_count = top_bar.split('|').count();
        assert!(
            field_count <= 3,
            "top bar has {field_count} fields (D2 cap is 3): {top_bar:?}"
        );
    }

    #[test]
    fn top_bar_on_empty_scene_is_two_fields_without_dev_telemetry() {
        let state = state_in_scene(ShellScene::Empty);
        let panels = shell_panels(&state);

        assert_top_bar_is_slim(&panels.top_bar);
        assert_eq!(panels.top_bar, "No project | Empty");
    }

    #[test]
    fn top_bar_on_editing_scene_reports_cursor_only() {
        let state = state_in_scene(ShellScene::Editing);
        let panels = shell_panels(&state);

        assert_top_bar_is_slim(&panels.top_bar);
        assert!(
            panels.top_bar.contains(" | Editing | Ln "),
            "top bar must carry the cursor on Editing, got {:?}",
            panels.top_bar
        );
    }

    #[test]
    fn top_bar_on_build_run_scene_reports_build_runtime_status() {
        let mut state = state_in_scene(ShellScene::BuildRun);
        state.set_execution(ExecutionState {
            build_status: String::from("passing"),
            runtime_status: String::from("completed"),
            ..state.runtime.execution.clone()
        });
        let panels = shell_panels(&state);

        assert_top_bar_is_slim(&panels.top_bar);
        assert!(
            panels.top_bar.ends_with("Run | passing / completed"),
            "Run top bar must end with status composite, got {:?}",
            panels.top_bar
        );
    }

    #[test]
    fn top_bar_on_palette_overlay_omits_internal_focus_label() {
        let state = state_in_scene(ShellScene::Palette);
        let panels = shell_panels(&state);

        assert_top_bar_is_slim(&panels.top_bar);
        assert!(
            panels.top_bar.ends_with(" | Palette"),
            "Palette scene must end with ' | Palette', got {:?}",
            panels.top_bar
        );
    }

    #[test]
    fn top_bar_on_com_reference_scene_uses_user_facing_label() {
        let state = state_in_scene(ShellScene::ComReference);
        let panels = shell_panels(&state);

        assert_top_bar_is_slim(&panels.top_bar);
        assert!(
            panels.top_bar.ends_with(" | COM reference"),
            "COM reference scene must end with ' | COM reference', got {:?}",
            panels.top_bar
        );
    }
}
