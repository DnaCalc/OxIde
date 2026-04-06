use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use oxvba_project::{
    BasProjModuleKind, ComSelectionCandidate, ComSelectionConfidence, ComSelectionService,
    ComSelectionSourceKind, FileBackedComSelectionQuery, HostProjectEdit, HostProjectEditPlan,
    OutputType, RegisteredComSelectionQuery, add_module_edit, add_project_reference_edit,
    apply_host_project_edit_plan, parse_basproj_xml, plan_add_com_candidate, plan_new_module,
    prepare_host_project_edit_plan, serialize_basproj_xml,
};

use super::state::{
    ComReferenceCandidateState, ComReferenceHelperState, ComReferenceSearchMode,
    WorkspaceProjectModuleKind, WorkspaceProjectState,
};

pub fn next_module_name(
    project: &WorkspaceProjectState,
    kind: WorkspaceProjectModuleKind,
) -> String {
    let prefix = match kind {
        WorkspaceProjectModuleKind::Module => "Module",
        WorkspaceProjectModuleKind::Class => "Class",
        WorkspaceProjectModuleKind::Document => "Document",
    };

    let mut index = 1_u16;
    loop {
        let candidate = format!("{prefix}{index}");
        if !project.modules.iter().any(|module| {
            module.logical_name.eq_ignore_ascii_case(candidate.as_str())
                || Path::new(module.include.as_str())
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .is_some_and(|stem| stem.eq_ignore_ascii_case(candidate.as_str()))
        }) {
            return candidate;
        }
        index += 1;
    }
}

pub fn add_scaffolded_module(
    workspace_path: &Path,
    kind: WorkspaceProjectModuleKind,
    logical_name: &str,
) -> io::Result<()> {
    let planned = plan_new_module(module_kind(kind), logical_name, Some(logical_name), true)
        .map_err(|source| io::Error::other(source.to_string()))?;
    let plan = prepare_edits(
        workspace_path,
        &[add_module_edit(planned.basproj_item.clone())],
    )?;

    let project_dir = plan
        .project_file
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));
    let source_path = project_dir.join(planned.include.as_str());
    if let Some(parent) = source_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&source_path, planned.source)?;
    apply_host_project_edit_plan(&plan).map_err(|source| io::Error::other(source.to_string()))?;
    Ok(())
}

pub fn add_project_reference(workspace_path: &Path, include: &str) -> io::Result<()> {
    apply_edits(workspace_path, &[add_project_reference_edit(include)])
}

pub struct ComReferenceDiscovery {
    pub candidates: Vec<ComSelectionCandidate>,
    pub helper: ComReferenceHelperState,
}

pub fn discover_com_reference_candidates(
    workspace_path: &Path,
    mode: ComReferenceSearchMode,
    query: &str,
    selection: usize,
) -> io::Result<ComReferenceDiscovery> {
    let service = ComSelectionService;
    let query = query.trim().to_string();
    let mut status_lines = Vec::new();
    let mut candidates = Vec::new();

    if !query.is_empty() {
        match mode {
            ComReferenceSearchMode::Search => {
                match service.discover_registered_candidates(&RegisteredComSelectionQuery {
                    reference_name: query.clone(),
                    requested_coclass: None,
                    import_lib: None,
                    guid: None,
                    version_major: None,
                    version_minor: None,
                    lcid: None,
                }) {
                    Ok(found) => candidates.extend(found),
                    Err(error) => status_lines.push(format!("registered lookup: {error}")),
                }

                match service.discover_prog_id_candidates(query.as_str()) {
                    Ok(found) => candidates.extend(found),
                    Err(error) => status_lines.push(format!("ProgID lookup: {error}")),
                }

                candidates.retain(|candidate| candidate_matches_query(candidate, query.as_str()));
            }
            ComReferenceSearchMode::File => {
                let path = PathBuf::from(query.as_str());
                if !is_supported_file_candidate(path.as_path()) {
                    status_lines.push(String::from(
                        "file mode requires an absolute .tlb, .dll, or .xll path",
                    ));
                } else {
                    match service.discover_file_backed_candidates(&FileBackedComSelectionQuery {
                        carrier_path: path,
                        reference_name: None,
                        requested_coclass: None,
                    }) {
                        Ok(found) => candidates.extend(found),
                        Err(error) => status_lines.push(format!("file lookup: {error}")),
                    }
                }
            }
        }
    }

    dedupe_candidates(&mut candidates);
    let surface = service
        .inspect_workspace_project_state(workspace_path, &candidates)
        .map_err(|source| io::Error::other(source.to_string()))?;
    let selection = clamp_selection(selection, candidates.len());

    if query.is_empty() {
        status_lines.push(match mode {
            ComReferenceSearchMode::Search => {
                String::from("Type an exact library name or ProgID")
            }
            ComReferenceSearchMode::File => {
                String::from("Enter an absolute .tlb, .dll, or .xll path")
            }
        });
    } else if candidates.is_empty() {
        status_lines.push(match mode {
            ComReferenceSearchMode::Search => {
                String::from("No COM typelib candidates matched the exact library-name or ProgID query")
            }
            ComReferenceSearchMode::File => {
                String::from("No COM typelib candidates matched the current file path")
            }
        });
    }

    let helper = ComReferenceHelperState {
        mode,
        query,
        selection,
        candidates: candidates
            .iter()
            .map(map_com_candidate_state)
            .collect::<Vec<_>>(),
        active_reference_lines: surface
            .active_references
            .iter()
            .map(|reference| {
                let mut line = reference.include.clone();
                if let Some(import_lib) = &reference.import_lib {
                    line.push_str(&format!(" [{import_lib}]"));
                }
                line
            })
            .collect(),
        status_lines,
    };

    Ok(ComReferenceDiscovery { candidates, helper })
}

pub fn add_com_reference_candidate(
    workspace_path: &Path,
    candidate: &ComSelectionCandidate,
) -> io::Result<()> {
    let plan = plan_add_com_candidate(candidate, None);
    apply_edits(workspace_path, &plan.edits)
}

pub fn cycle_output_type(workspace_path: &Path) -> io::Result<OutputType> {
    let project_file = prepare_edits(workspace_path, &[])?.project_file;
    let xml = fs::read_to_string(&project_file)?;
    let mut basproj =
        parse_basproj_xml(&xml).map_err(|source| io::Error::other(source.to_string()))?;
    let current = basproj
        .properties
        .output_type
        .ok_or_else(|| io::Error::other("project is missing OutputType"))?;
    let next = next_output_type(current);
    basproj.properties.output_type = Some(next);
    fs::write(project_file, serialize_basproj_xml(&basproj))?;
    Ok(next)
}

fn apply_edits(workspace_path: &Path, edits: &[HostProjectEdit]) -> io::Result<()> {
    let plan = prepare_edits(workspace_path, edits)?;
    apply_host_project_edit_plan(&plan).map_err(|source| io::Error::other(source.to_string()))?;
    Ok(())
}

fn prepare_edits(
    workspace_path: &Path,
    edits: &[HostProjectEdit],
) -> io::Result<HostProjectEditPlan> {
    let plan = prepare_host_project_edit_plan(workspace_path, edits)
        .map_err(|source| io::Error::other(source.to_string()))?;
    if plan.validation.can_apply {
        return Ok(plan);
    }

    let summary = plan
        .validation
        .issues
        .iter()
        .map(|issue| issue.message.as_str())
        .collect::<Vec<_>>()
        .join("; ");
    Err(io::Error::other(summary))
}

fn module_kind(kind: WorkspaceProjectModuleKind) -> BasProjModuleKind {
    match kind {
        WorkspaceProjectModuleKind::Module => BasProjModuleKind::Module,
        WorkspaceProjectModuleKind::Class => BasProjModuleKind::ClassModule,
        WorkspaceProjectModuleKind::Document => BasProjModuleKind::DocumentModule,
    }
}

fn next_output_type(current: OutputType) -> OutputType {
    match current {
        OutputType::Exe => OutputType::Library,
        OutputType::Library => OutputType::Addin,
        OutputType::Addin => OutputType::ComServer,
        OutputType::ComServer => OutputType::HostModule,
        OutputType::HostModule => OutputType::ComExe,
        OutputType::ComExe => OutputType::Exe,
    }
}

fn map_com_candidate_state(candidate: &ComSelectionCandidate) -> ComReferenceCandidateState {
    let mut detail_lines = vec![format!(
        "{} | {} | {}",
        source_kind_label(candidate.source_kind),
        carrier_kind_label(candidate),
        confidence_label(candidate.confidence)
    )];

    if let Some(description) = &candidate.friendly_description {
        detail_lines.push(format!("desc {description}"));
    }
    if !candidate.prog_ids.is_empty() {
        detail_lines.push(format!("progids {}", candidate.prog_ids.join(", ")));
    }
    if let Some(import_lib) = &candidate.identity.import_lib {
        detail_lines.push(format!("carrier {import_lib}"));
    }
    if let Some(guid) = &candidate.identity.guid {
        detail_lines.push(format!("guid {guid}"));
    }

    ComReferenceCandidateState {
        title: candidate.identity.library_name.clone(),
        detail_lines,
    }
}

fn source_kind_label(kind: ComSelectionSourceKind) -> &'static str {
    match kind {
        ComSelectionSourceKind::RegisteredLibrary => "registered",
        ComSelectionSourceKind::ProgIdLookup => "ProgID",
        ComSelectionSourceKind::FileBrowse => "file",
        ComSelectionSourceKind::ProjectActiveReference => "project",
    }
}

fn carrier_kind_label(candidate: &ComSelectionCandidate) -> &'static str {
    match candidate.carrier_kind {
        oxvba_project::ComSelectionCarrierKind::TypeLibrary => "typelib",
        oxvba_project::ComSelectionCarrierKind::DynamicLibrary => "dll",
        oxvba_project::ComSelectionCarrierKind::ActiveXControl => "ocx",
        oxvba_project::ComSelectionCarrierKind::Executable => "exe",
        oxvba_project::ComSelectionCarrierKind::Xll => "xll",
        oxvba_project::ComSelectionCarrierKind::Unknown => "unknown",
    }
}

fn confidence_label(confidence: ComSelectionConfidence) -> &'static str {
    match confidence {
        ComSelectionConfidence::Exact => "exact",
        ComSelectionConfidence::Strong => "strong",
        ComSelectionConfidence::Weak => "weak",
    }
}

fn candidate_matches_query(candidate: &ComSelectionCandidate, query: &str) -> bool {
    let query = query.to_ascii_lowercase();
    candidate
        .identity
        .library_name
        .to_ascii_lowercase()
        .contains(&query)
        || candidate
            .friendly_description
            .as_deref()
            .unwrap_or("")
            .to_ascii_lowercase()
            .contains(&query)
        || candidate
            .prog_ids
            .iter()
            .any(|prog_id| prog_id.to_ascii_lowercase().contains(&query))
        || candidate
            .identity
            .import_lib
            .as_deref()
            .unwrap_or("")
            .to_ascii_lowercase()
            .contains(&query)
}

fn dedupe_candidates(candidates: &mut Vec<ComSelectionCandidate>) {
    let mut seen = std::collections::BTreeSet::new();
    candidates.retain(|candidate| {
        let key = format!(
            "{}|{}|{}|{}|{:?}",
            candidate.identity.library_name.to_ascii_lowercase(),
            candidate
                .identity
                .guid
                .as_deref()
                .unwrap_or("")
                .to_ascii_lowercase(),
            candidate.identity.version_major.unwrap_or_default(),
            candidate.identity.version_minor.unwrap_or_default(),
            candidate.identity.import_lib
        );
        seen.insert(key)
    });
}

fn clamp_selection(selection: usize, len: usize) -> usize {
    if len == 0 {
        0
    } else {
        selection.min(len.saturating_sub(1))
    }
}

fn is_supported_file_candidate(path: &Path) -> bool {
    path.is_absolute()
        && path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| {
                matches!(
                    extension.to_ascii_lowercase().as_str(),
                    "tlb" | "dll" | "xll"
                )
            })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shell::session::ProjectSession;

    const FIXTURE_BASPROJ: &str = r#"<Project Sdk="OxVba.Sdk/0.1.0">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <ProjectName>FixtureApp</ProjectName>
    <EntryPoint>Module1.Main</EntryPoint>
  </PropertyGroup>
  <ItemGroup>
    <Module Include="Module1.bas" />
  </ItemGroup>
</Project>
"#;

    const FIXTURE_MODULE: &str = "Option Explicit\n\nPublic Sub Main()\nEnd Sub\n";

    #[test]
    fn add_scaffolded_module_updates_workspace_and_writes_source() {
        let workspace = seed_workspace("add-module");
        add_scaffolded_module(&workspace, WorkspaceProjectModuleKind::Module, "Module2").unwrap();

        let session = ProjectSession::load(&workspace).unwrap();
        assert!(
            session
                .project
                .modules
                .iter()
                .any(|module| module.include == "Module2.bas")
        );
        assert!(workspace.parent().unwrap().join("Module2.bas").exists());
    }

    #[test]
    fn add_project_reference_updates_workspace_surface() {
        let workspace = seed_workspace("add-project-reference");
        add_project_reference(&workspace, "../Lib/Lib.basproj").unwrap();

        let session = ProjectSession::load(&workspace).unwrap();
        assert!(
            session
                .project
                .references
                .iter()
                .any(|reference| reference.include == "../Lib/Lib.basproj")
        );
    }

    #[test]
    fn cycle_output_type_rewrites_project_file() {
        let workspace = seed_workspace("cycle-output-type");
        let next = cycle_output_type(&workspace).unwrap();
        let session = ProjectSession::load(&workspace).unwrap();

        assert_eq!(next, OutputType::Library);
        assert_eq!(session.target_name, "Library");
    }

    #[test]
    fn discover_search_mode_returns_prog_id_candidates() {
        let workspace = seed_workspace("discover-search");
        let discovery = discover_com_reference_candidates(
            &workspace,
            ComReferenceSearchMode::Search,
            "OxVba.TestDispatch",
            0,
        )
        .unwrap();

        assert!(!discovery.candidates.is_empty());
        assert!(
            discovery
                .helper
                .candidates
                .iter()
                .any(|candidate| candidate.title == "OxVba.TestDispatch")
        );
    }

    #[test]
    fn discover_file_mode_accepts_xll_and_tlb_carriers() {
        let workspace = seed_workspace("discover-file");
        let discovery = discover_com_reference_candidates(
            &workspace,
            ComReferenceSearchMode::File,
            fixture_typelib_hint("OxVba.TestEventServer.tlb").as_str(),
            0,
        )
        .unwrap();

        assert_eq!(discovery.candidates.len(), 1);
        assert!(
            discovery.helper.candidates[0]
                .detail_lines
                .iter()
                .any(|line| line.contains("typelib"))
        );
    }

    #[test]
    fn add_com_reference_candidate_updates_workspace_surface() {
        let workspace = seed_workspace("add-com-reference-candidate");
        let discovery = discover_com_reference_candidates(
            &workspace,
            ComReferenceSearchMode::Search,
            "OxVba.TestDispatch",
            0,
        )
        .unwrap();
        add_com_reference_candidate(&workspace, &discovery.candidates[0]).unwrap();

        let session = ProjectSession::load(&workspace).unwrap();
        assert!(
            session
                .project
                .references
                .iter()
                .any(|reference| reference.include == "OxVba.TestDispatch")
        );
    }

    fn seed_workspace(name: &str) -> PathBuf {
        let root = PathBuf::from("target")
            .join("test-workspaces")
            .join("project-actions")
            .join(name);
        fs::create_dir_all(&root).unwrap();
        let basproj = root.join("FixtureApp.basproj");
        fs::write(&basproj, FIXTURE_BASPROJ).unwrap();
        fs::write(root.join("Module1.bas"), FIXTURE_MODULE).unwrap();
        basproj
    }

    fn fixture_typelib_hint(file_name: &str) -> String {
        std::env::current_dir()
            .unwrap()
            .join(".external")
            .join("oxvba-frozen")
            .join("temp")
            .join("missing")
            .join(file_name)
            .to_string_lossy()
            .into_owned()
    }
}
