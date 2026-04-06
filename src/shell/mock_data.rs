use super::state::{
    FocusRegion, PaletteCommandState, PanelContentState, ShellScene, ShellState, WorkspaceState,
};
use super::theme;

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

fn top_bar_text(state: &ShellState) -> String {
    let workspace = &state.runtime.workspace;
    let project = workspace.project_name.as_deref().unwrap_or("No project");
    let target = workspace.target_name.as_str();
    let layout = workspace.layout.preset.label();
    let views = workspace.visible_view_count();
    let hidden_buffers = workspace.hidden_buffer_count();
    let cursor = workspace
        .active_view()
        .map(|view| {
            format!(
                "Ln {} Col {}",
                view.surface.cursor.line, view.surface.cursor.column
            )
        })
        .unwrap_or_else(|| String::from("No cursor"));

    match state.scene {
        ShellScene::Empty => format!(
            "{project} | {layout} | Focus {} | {} | Truecolor ready",
            state.runtime.focus.label(),
            state.runtime.width_class.label()
        ),
        ShellScene::Editing => format!(
            "{project} | {target} | {layout} | Focus {} | Views {views} | {cursor} | Hidden {hidden_buffers}",
            state.runtime.focus.label(),
        ),
        ShellScene::Semantic => format!(
            "{project} | {target} | {layout} | Focus {} | Views {views} | {cursor} | Hover active",
            state.runtime.focus.label()
        ),
        ShellScene::BuildRun => format!(
            "{project} | {target} | {layout} | Focus {} | {cursor} | {} / {}",
            state.runtime.focus.label(),
            state.runtime.execution.build_status,
            state.runtime.execution.runtime_status
        ),
        ShellScene::Palette => format!(
            "{project} | {target} | Palette | Overlay focus | {}",
            theme::palette_name(),
        ),
        ShellScene::ComReference => {
            format!("{project} | {target} | COM reference helper | Overlay focus")
        }
    }
}

fn explorer_text(state: &ShellState) -> String {
    match state.scene {
        ShellScene::Empty => launcher_text(state),
        _ => project_explorer_text(&state.runtime.workspace),
    }
}

fn editor_title(state: &ShellState) -> String {
    match state.scene {
        ShellScene::Empty => String::from("Welcome"),
        _ => {
            let workspace = &state.runtime.workspace;
            let active_view = workspace.active_view();
            let active_buffer = workspace.active_buffer();
            match (active_buffer, active_view) {
                (Some(buffer), Some(view)) => {
                    format!("{} | {} View", buffer.title, view.kind.label())
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

fn inspector_text(state: &ShellState) -> String {
    let mut text = format_panel_content(&state.runtime.content.inspector);
    if matches!(state.scene, ShellScene::Empty) {
        if !text.is_empty() {
            text.push('\n');
            text.push('\n');
        }
        text.push_str("Tokens\n");
        text.push_str(&theme::token_summary());
    }
    text
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
    for command in &palette.commands {
        text.push_str(&format_palette_command(command));
    }

    text.push_str("\nMockup States\n");
    for command in &palette.state_commands {
        text.push_str(&format!("  {:<24}{}\n", command.label, command.shortcut));
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

fn project_explorer_text(workspace: &WorkspaceState) -> String {
    let project = workspace.project_name.as_deref().unwrap_or("No project");
    let mut text = format!("Project\n> {project}\n");

    if let Some(project_state) = &workspace.project {
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
                text.push_str(&format!(
                    "> {} [{}]\n",
                    module.logical_name,
                    module.kind.label()
                ));
                text.push_str(&format!("  include {}\n", module.include));
                text.push_str(&format!("  source {}\n", module.source_path.display()));
                if let Some(declared_name) = &module.declared_name {
                    text.push_str(&format!("  declared {declared_name}\n"));
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

    text.push_str("\nOpen Buffers\n");

    for buffer in &workspace.buffers {
        let visible = workspace
            .visible_views()
            .iter()
            .any(|view| view.buffer_id == buffer.id);
        let dirty = if buffer.dirty { " *" } else { "" };
        let marker = if visible { ">" } else { " " };
        let visibility = if visible { "view" } else { "open" };
        text.push_str(&format!(
            "{marker} {} [{}:{visibility}]{dirty}\n",
            buffer.title,
            buffer.kind.label()
        ));
    }

    text.push_str("\nViews\n");
    for view in workspace.visible_views() {
        let active = if view.id == workspace.layout.active_view {
            ">"
        } else {
            " "
        };
        let buffer_title = workspace
            .buffer(view.buffer_id)
            .map(|buffer| buffer.title.as_str())
            .unwrap_or("Unknown");
        text.push_str(&format!(
            "{active} {} view -> {buffer_title}\n",
            view.kind.label()
        ));
    }

    text.push_str(&format!(
        "\nTarget\n> {}\n\nLayout\n  {}\n",
        workspace.target_name,
        workspace.layout.preset.label()
    ));
    text
}

fn launcher_text(state: &ShellState) -> String {
    let launcher = &state.runtime.content.launcher;
    let mut text = String::from("Recent\n");
    if launcher.recent_projects.is_empty() {
        text.push_str("  No discovered .basproj files\n");
    } else {
        for project in &launcher.recent_projects {
            text.push_str(&format!("{project}\n"));
        }
    }

    text.push_str("\nStart\n");
    for action in &launcher.actions {
        text.push_str(&format!("  {action}\n"));
    }
    text
}

fn launcher_editor_text(state: &ShellState) -> String {
    let launcher = &state.runtime.content.launcher;
    let mut text = format!(
        "{}\n\nA terminal-native IDE for OxVba.\n\nOpen\n",
        state
            .runtime
            .workspace
            .active_buffer()
            .map(|buffer| buffer.title.as_str())
            .unwrap_or("OxIde")
    );
    for (index, action) in launcher.actions.iter().enumerate() {
        let marker = if index == 0 { ">" } else { " " };
        text.push_str(&format!("  {marker} {action}\n"));
    }

    text.push_str("\nCapability\n");
    for capability in &launcher.capabilities {
        text.push_str(&format!("  {capability}\n"));
    }
    text.push_str(&format!("\nHint\n  {}\n", launcher.hint));
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

fn format_palette_command(command: &PaletteCommandState) -> String {
    format!("  {:<24}{}\n", command.label, command.shortcut)
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
        BufferId, BufferKind, BufferState, CursorPosition, EditorSurfaceState, LayoutPreset,
        ViewId, ViewKind, ViewState, WorkspaceLayoutState, WorkspaceProjectModuleKind,
        WorkspaceProjectModuleState, WorkspaceProjectReferenceKind, WorkspaceProjectReferenceState,
        WorkspaceProjectState, WorkspaceProjectTargetKind,
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
}
