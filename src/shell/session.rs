use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use oxvba_project::{
    BasProjModuleKind, HostProjectModuleInfo, HostProjectReferenceInfo, HostProjectReferenceKind,
    HostProjectSurface, HostWorkspaceTargetKind, inspect_workspace_target, load_basproj,
};

use super::oxvba::{load_execution_state, load_semantic_state};
use super::state::{
    BufferId, BufferKind, BufferState, CursorPosition, EditorSurfaceState, ExecutionState,
    LayoutPreset, ViewId, ViewKind, ViewState, WorkspaceLayoutState, WorkspaceProjectModuleKind,
    WorkspaceProjectModuleState, WorkspaceProjectReferenceKind, WorkspaceProjectReferenceState,
    WorkspaceProjectState, WorkspaceProjectTargetKind, WorkspaceState,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectSession {
    pub project_path: PathBuf,
    pub project_name: String,
    pub target_name: String,
    pub entry_point: String,
    pub project: WorkspaceProjectState,
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
        let surface = inspect_workspace_target(&project_path)
            .map_err(|source| io::Error::other(source.to_string()))?;
        let loaded =
            load_basproj(&project_path).map_err(|source| io::Error::other(source.to_string()))?;
        let project_name = surface.project_name.clone();
        let target_name = output_type_label(surface.output_type);
        let entry_point = loaded
            .entry_point
            .unwrap_or_else(|| String::from("Main.Main"));

        let mut documents = surface
            .modules
            .iter()
            .map(DocumentSession::load_from_module)
            .collect::<io::Result<Vec<_>>>()?;

        documents.sort_by(|left, right| left.title.cmp(&right.title));

        Ok(Self {
            project_path,
            project_name,
            target_name,
            entry_point,
            project: map_project_surface(surface),
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
        let default_cursor = CursorPosition::new(1, 1);
        let semantic = buffers.first().and_then(|buffer| {
            load_semantic_state(
                &self.project_path,
                Some(buffer.title.as_str()),
                &buffer.lines,
                default_cursor,
            )
        });

        WorkspaceState {
            project_name: Some(self.project_name.clone()),
            target_name: self.target_name.clone(),
            project: Some(self.project.clone()),
            recent_buffers: buffers.iter().map(|buffer| buffer.id).collect(),
            views: vec![ViewState {
                id: ViewId(1),
                buffer_id: active_buffer_id,
                kind: ViewKind::Primary,
                surface: EditorSurfaceState {
                    cursor: default_cursor,
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
            semantic,
        }
    }

    pub fn discover_projects(root: impl AsRef<Path>) -> io::Result<Vec<PathBuf>> {
        let mut projects = Vec::new();
        discover_projects_recursive(root.as_ref(), 0, &mut projects)?;
        projects.sort();
        Ok(projects)
    }

    pub fn execution_state(&self) -> ExecutionState {
        load_execution_state(
            &self.project_path,
            execution_profile_for_target(self.target_name.as_str()),
            self.entry_point.clone(),
        )
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

    fn load_from_module(module: &HostProjectModuleInfo) -> io::Result<Self> {
        let mut document = Self::load(module.source_path.clone())?;
        document.kind = buffer_kind_for_module_kind(module.kind);
        Ok(document)
    }
}

fn buffer_kind_for_path(path: &Path) -> BufferKind {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("cls") => BufferKind::Class,
        Some("bas") => BufferKind::Source,
        _ => BufferKind::Source,
    }
}

fn buffer_kind_for_module_kind(kind: BasProjModuleKind) -> BufferKind {
    match kind {
        BasProjModuleKind::Module => BufferKind::Source,
        BasProjModuleKind::ClassModule | BasProjModuleKind::DocumentModule => BufferKind::Class,
    }
}

fn map_project_surface(surface: HostProjectSurface) -> WorkspaceProjectState {
    WorkspaceProjectState {
        workspace_kind: map_workspace_target_kind(surface.workspace_kind),
        workspace_target: surface.workspace_target,
        project_file: surface.project_file,
        project_dir: surface.project_dir,
        output_type: output_type_label(surface.output_type),
        modules: surface
            .modules
            .into_iter()
            .map(map_project_module)
            .collect(),
        references: surface
            .references
            .into_iter()
            .map(map_project_reference)
            .collect(),
    }
}

fn map_workspace_target_kind(kind: HostWorkspaceTargetKind) -> WorkspaceProjectTargetKind {
    match kind {
        HostWorkspaceTargetKind::BasProj => WorkspaceProjectTargetKind::BasProj,
        HostWorkspaceTargetKind::Vbp => WorkspaceProjectTargetKind::Vbp,
        HostWorkspaceTargetKind::ConventionDirectory => {
            WorkspaceProjectTargetKind::ConventionDirectory
        }
    }
}

fn map_project_module(module: HostProjectModuleInfo) -> WorkspaceProjectModuleState {
    WorkspaceProjectModuleState {
        kind: match module.kind {
            BasProjModuleKind::Module => WorkspaceProjectModuleKind::Module,
            BasProjModuleKind::ClassModule => WorkspaceProjectModuleKind::Class,
            BasProjModuleKind::DocumentModule => WorkspaceProjectModuleKind::Document,
        },
        include: module.include,
        source_path: module.source_path,
        logical_name: module.identity.effective_name,
        declared_name: module.identity.declared_vb_name,
    }
}

fn map_project_reference(reference: HostProjectReferenceInfo) -> WorkspaceProjectReferenceState {
    WorkspaceProjectReferenceState {
        kind: match reference.kind {
            HostProjectReferenceKind::Project => WorkspaceProjectReferenceKind::Project,
            HostProjectReferenceKind::Com => WorkspaceProjectReferenceKind::Com,
            HostProjectReferenceKind::Native => WorkspaceProjectReferenceKind::Native,
        },
        include: reference.include,
        referenced_project_name: reference.referenced_project_name,
        path: reference.path,
        guid: reference.guid,
        import_lib: reference.import_lib,
    }
}

fn output_type_label(output_type: oxvba_project::OutputType) -> String {
    match output_type {
        oxvba_project::OutputType::HostModule => String::from("HostModule"),
        oxvba_project::OutputType::Library => String::from("Library"),
        oxvba_project::OutputType::Exe => String::from("Exe"),
        oxvba_project::OutputType::Addin => String::from("Addin"),
        oxvba_project::OutputType::ComServer => String::from("ComServer"),
        oxvba_project::OutputType::ComExe => String::from("ComExe"),
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
        assert_eq!(
            workspace
                .project
                .as_ref()
                .map(|project| project.workspace_kind),
            Some(WorkspaceProjectTargetKind::BasProj)
        );
        assert_eq!(
            workspace
                .project
                .as_ref()
                .map(|project| project.modules.len()),
            Some(1)
        );
        assert_eq!(
            workspace
                .project
                .as_ref()
                .map(|project| project.modules[0].include.as_str()),
            Some("Module1.bas")
        );
        assert_eq!(workspace.buffers.len(), 1);
        assert_eq!(workspace.buffers[0].title, "Module1.bas");
        assert!(workspace.semantic.is_some());
        assert!(
            workspace.buffers[0]
                .lines
                .iter()
                .any(|line| line.contains("Public Sub Main"))
        );
        assert!(
            workspace
                .semantic
                .as_ref()
                .is_some_and(|semantic| semantic.symbols.iter().any(|symbol| symbol == "Main"))
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
        assert_eq!(execution.build_status, "passing");
        assert_eq!(execution.runtime_status, "prepared");
        assert!(
            execution
                .output_lines
                .iter()
                .any(|line| line.contains("runtime reset to a fresh prepared session"))
        );
        assert!(
            execution
                .log_lines
                .iter()
                .any(|line| line.contains("workspace loaded"))
        );
    }
}
