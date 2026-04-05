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
            "{project} | {target} | {layout} | Focus {} | {cursor} | Output live",
            state.runtime.focus.label()
        ),
        ShellScene::Palette => format!(
            "{project} | {target} | Palette | Overlay focus | {}",
            theme::palette_name(),
        ),
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
    if matches!(state.scene, ShellScene::Palette) {
        if !base.is_empty() {
            base.push('\n');
            base.push('\n');
        }
        base.push_str("Palette\n");
        base.push_str("  Background shell is frozen while the palette is active.");
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

fn project_explorer_text(workspace: &WorkspaceState) -> String {
    let project = workspace.project_name.as_deref().unwrap_or("No project");
    let mut text = format!("Project\n> {project}\n\nOpen Buffers\n");

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
    for (index, project) in launcher.recent_projects.iter().enumerate() {
        let marker = if index == 0 { ">" } else { " " };
        text.push_str(&format!("{marker} {project}\n"));
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
    let mut text = state
        .runtime
        .workspace
        .active_buffer()
        .map(|buffer| buffer.lines.join("\n"))
        .unwrap_or_else(|| String::from("No buffer mounted."));

    if !state.runtime.content.editor_notes.is_empty() {
        text.push_str("\n\nView\n");
        for note in &state.runtime.content.editor_notes {
            text.push_str(&format!("  {note}\n"));
        }
    }

    text
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
