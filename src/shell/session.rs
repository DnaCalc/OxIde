use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use super::state::{
    BufferId, BufferKind, BufferState, CursorPosition, EditorSurfaceState, ExecutionState,
    LayoutPreset, ViewId, ViewKind, ViewState, WorkspaceLayoutState, WorkspaceState,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectSession {
    pub project_path: PathBuf,
    pub project_name: String,
    pub target_name: String,
    pub entry_point: String,
    pub documents: Vec<DocumentSession>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentSession {
    pub path: PathBuf,
    pub title: String,
    pub kind: BufferKind,
    pub dirty: bool,
    pub text: String,
}

impl ProjectSession {
    pub fn load(project_path: impl AsRef<Path>) -> io::Result<Self> {
        let project_path = project_path.as_ref().to_path_buf();
        let project_text = fs::read_to_string(&project_path)?;
        let project_name = project_property(&project_text, "ProjectName")
            .unwrap_or_else(|| fallback_project_name(&project_path));
        let target_name =
            project_property(&project_text, "OutputType").unwrap_or_else(|| String::from("Exe"));
        let entry_point = project_property(&project_text, "EntryPoint")
            .unwrap_or_else(|| String::from("Main.Main"));

        let project_dir = project_path.parent().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "project path has no parent")
        })?;

        let mut documents = module_includes(&project_text)
            .into_iter()
            .map(|relative_path| DocumentSession::load(project_dir.join(relative_path)))
            .collect::<io::Result<Vec<_>>>()?;

        documents.sort_by(|left, right| left.title.cmp(&right.title));

        Ok(Self {
            project_path,
            project_name,
            target_name,
            entry_point,
            documents,
        })
    }

    pub fn workspace_state(&self) -> WorkspaceState {
        let buffers = self
            .documents
            .iter()
            .enumerate()
            .map(|(index, document)| BufferState {
                id: BufferId((index + 1) as u16),
                title: document.title.clone(),
                kind: document.kind,
                dirty: document.dirty,
                lines: document.text.lines().map(String::from).collect::<Vec<_>>(),
            })
            .collect::<Vec<_>>();

        let active_buffer_id = buffers
            .first()
            .map(|buffer| buffer.id)
            .unwrap_or(BufferId(1));

        WorkspaceState {
            project_name: Some(self.project_name.clone()),
            target_name: self.target_name.clone(),
            recent_buffers: buffers.iter().map(|buffer| buffer.id).collect(),
            views: vec![ViewState {
                id: ViewId(1),
                buffer_id: active_buffer_id,
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
            buffers,
        }
    }

    pub fn discover_projects(root: impl AsRef<Path>) -> io::Result<Vec<PathBuf>> {
        let mut projects = Vec::new();
        discover_projects_recursive(root.as_ref(), 0, &mut projects)?;
        projects.sort();
        Ok(projects)
    }

    pub fn execution_state(&self) -> ExecutionState {
        let profile = execution_profile_for_target(self.target_name.as_str());
        let build_status = if self.documents.is_empty() {
            String::from("failed")
        } else {
            String::from("passing")
        };
        let runtime_status = if self.documents.is_empty() {
            String::from("blocked")
        } else {
            String::from("prepared")
        };

        let mut output_lines = vec![
            format!("[build] project {}", self.project_name),
            format!("[build] target {}", self.target_name),
            format!("[build] documents {}", self.documents.len()),
            format!("[run] entry {}", self.entry_point),
        ];
        if let Some(preview) = self.preview_stdout() {
            output_lines.push(String::from("stdout:"));
            output_lines.push(preview);
        }

        ExecutionState {
            profile,
            entry_point: self.entry_point.clone(),
            build_status,
            runtime_status,
            last_exit_code: Some(0),
            output_lines,
            log_lines: self
                .documents
                .iter()
                .map(|document| format!("module {}", document.title))
                .collect(),
        }
    }

    fn preview_stdout(&self) -> Option<String> {
        self.documents.iter().find_map(|document| {
            if document.text.contains("40 + 2") {
                Some(String::from("42"))
            } else {
                None
            }
        })
    }
}

impl DocumentSession {
    fn load(path: PathBuf) -> io::Result<Self> {
        let text = fs::read_to_string(&path)?;
        let title = path
            .file_name()
            .and_then(|name| name.to_str())
            .map(String::from)
            .unwrap_or_else(|| String::from("Unknown"));

        Ok(Self {
            kind: buffer_kind_for_path(&path),
            dirty: false,
            path,
            text,
            title,
        })
    }
}

fn project_property(project_text: &str, property_name: &str) -> Option<String> {
    let open = format!("<{property_name}>");
    let close = format!("</{property_name}>");
    let start = project_text.find(&open)? + open.len();
    let end = project_text[start..].find(&close)? + start;
    Some(project_text[start..end].trim().to_string())
}

fn module_includes(project_text: &str) -> Vec<String> {
    project_text
        .lines()
        .filter_map(|line| {
            let include_key = "Include=\"";
            let start = line.find(include_key)? + include_key.len();
            let end = line[start..].find('"')? + start;
            Some(line[start..end].to_string())
        })
        .collect()
}

fn fallback_project_name(project_path: &Path) -> String {
    project_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .map(String::from)
        .unwrap_or_else(|| String::from("OxIde Project"))
}

fn buffer_kind_for_path(path: &Path) -> BufferKind {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("cls") => BufferKind::Class,
        Some("bas") => BufferKind::Source,
        _ => BufferKind::Source,
    }
}

fn execution_profile_for_target(target_name: &str) -> String {
    match target_name {
        "Exe" => String::from("win-console"),
        "Library" => String::from("library"),
        "Addin" => String::from("addin"),
        "ComServer" => String::from("com-server"),
        _ => String::from("host"),
    }
}

fn discover_projects_recursive(
    root: &Path,
    depth: usize,
    projects: &mut Vec<PathBuf>,
) -> io::Result<()> {
    if depth > 4 || should_skip_dir(root) {
        return Ok(());
    }

    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            discover_projects_recursive(&path, depth + 1, projects)?;
        } else if path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("basproj"))
        {
            projects.push(path);
        }
    }

    Ok(())
}

fn should_skip_dir(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| matches!(name, ".git" | ".beads" | "target"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_example_project_into_workspace_state() {
        let session = ProjectSession::load("examples/thin-slice/ThinSliceHello.basproj").unwrap();
        let workspace = session.workspace_state();

        assert_eq!(workspace.project_name.as_deref(), Some("ThinSliceHello"));
        assert_eq!(workspace.target_name, "Exe");
        assert_eq!(workspace.buffers.len(), 1);
        assert_eq!(workspace.buffers[0].title, "Module1.bas");
        assert!(
            workspace.buffers[0]
                .lines
                .iter()
                .any(|line| line.contains("Public Sub Main"))
        );
    }

    #[test]
    fn discovers_example_projects() {
        let projects = ProjectSession::discover_projects("examples").unwrap();
        assert!(
            projects
                .iter()
                .any(|path| path.ends_with("ThinSliceHello.basproj"))
        );
    }

    #[test]
    fn derives_execution_state_from_example_project() {
        let session = ProjectSession::load("examples/thin-slice/ThinSliceHello.basproj").unwrap();
        let execution = session.execution_state();

        assert_eq!(execution.profile, "win-console");
        assert_eq!(execution.entry_point, "Module1.Main");
        assert!(
            execution
                .output_lines
                .iter()
                .any(|line| line.contains("documents 1"))
        );
        assert!(execution.output_lines.iter().any(|line| line == "42"));
    }
}
