use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

const SESSION_FILE_NAME: &str = "session.json";
const MAX_RECENT_PROJECTS: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SessionWorkspaceRestore {
    pub project_path: String,
    pub open_buffers: Vec<String>,
    pub active_buffer: Option<String>,
    pub cursor_line: u16,
    pub cursor_column: u16,
    pub scroll_top: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SessionSnapshot {
    pub recent_projects: Vec<String>,
    pub last_opened: Option<String>,
    pub last_workspace: Option<SessionWorkspaceRestore>,
}

impl SessionSnapshot {
    pub fn recent_project_paths(&self) -> Vec<PathBuf> {
        self.recent_projects.iter().map(PathBuf::from).collect()
    }

    pub fn last_opened_path(&self) -> Option<PathBuf> {
        self.last_opened.as_ref().map(PathBuf::from)
    }

    pub fn record_opened(&mut self, project_path: &Path) {
        let project = project_path.to_string_lossy().to_string();
        self.recent_projects.retain(|existing| existing != &project);
        self.recent_projects.insert(0, project.clone());
        self.recent_projects.truncate(MAX_RECENT_PROJECTS);
        self.last_opened = Some(project);
    }
}

pub fn load() -> io::Result<SessionSnapshot> {
    load_from_dir(&session_dir()?)
}

pub fn save(snapshot: &SessionSnapshot) -> io::Result<()> {
    save_to_dir(&session_dir()?, snapshot)
}

pub fn load_from_dir(dir: &Path) -> io::Result<SessionSnapshot> {
    let path = dir.join(SESSION_FILE_NAME);
    let raw = match fs::read_to_string(&path) {
        Ok(raw) => raw,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(SessionSnapshot::default()),
        Err(error) => return Err(error),
    };
    let parsed = serde_json::from_str::<SessionSnapshot>(&raw).map_err(io::Error::other)?;
    Ok(parsed)
}

pub fn save_to_dir(dir: &Path, snapshot: &SessionSnapshot) -> io::Result<()> {
    fs::create_dir_all(dir)?;
    let path = dir.join(SESSION_FILE_NAME);
    let temp = path.with_extension("json.tmp");
    let payload = serde_json::to_vec_pretty(snapshot).map_err(io::Error::other)?;
    fs::write(&temp, payload)?;
    if path.exists() {
        // Windows does not allow `rename` over an existing target.
        // Replace in two steps after the fully-written temp file
        // exists, so the original file remains intact on write errors.
        fs::remove_file(&path)?;
    }
    fs::rename(&temp, &path)?;
    Ok(())
}

fn session_dir() -> io::Result<PathBuf> {
    if let Some(appdata) = env::var_os("APPDATA") {
        return Ok(PathBuf::from(appdata).join("OxIde"));
    }
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "APPDATA is not set; cannot resolve OxIde session store path",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seed_temp_dir(name: &str) -> PathBuf {
        let dir = PathBuf::from("target")
            .join("test-workspaces")
            .join("session-store")
            .join(name);
        fs::create_dir_all(&dir).expect("create session-store fixture dir");
        dir
    }

    #[test]
    fn round_trip_persists_recent_and_workspace_restore() {
        let dir = seed_temp_dir("round-trip");
        let mut snapshot = SessionSnapshot::default();
        snapshot.record_opened(Path::new("C:/tmp/A.basproj"));
        snapshot.last_workspace = Some(SessionWorkspaceRestore {
            project_path: String::from("C:/tmp/A.basproj"),
            open_buffers: vec![String::from("Module1.bas"), String::from("Helpers.bas")],
            active_buffer: Some(String::from("Helpers.bas")),
            cursor_line: 7,
            cursor_column: 3,
            scroll_top: 2,
        });

        save_to_dir(&dir, &snapshot).expect("save session snapshot");
        let loaded = load_from_dir(&dir).expect("load session snapshot");
        assert_eq!(loaded, snapshot);
    }

    #[test]
    fn record_opened_is_mru_deduped_and_capped() {
        let mut snapshot = SessionSnapshot::default();
        for i in 0..40 {
            let path = format!("C:/tmp/{i}.basproj");
            snapshot.record_opened(Path::new(&path));
        }
        snapshot.record_opened(Path::new("C:/tmp/39.basproj"));

        assert_eq!(snapshot.recent_projects.len(), MAX_RECENT_PROJECTS);
        assert_eq!(
            snapshot.recent_projects.first().map(String::as_str),
            Some("C:/tmp/39.basproj")
        );
        assert_eq!(
            snapshot.last_opened.as_deref(),
            Some("C:/tmp/39.basproj")
        );
        let dedup_count = snapshot
            .recent_projects
            .iter()
            .filter(|entry| entry.as_str() == "C:/tmp/39.basproj")
            .count();
        assert_eq!(dedup_count, 1, "MRU should contain deduped entries");
    }

    #[test]
    fn load_missing_file_returns_default_snapshot() {
        let dir = seed_temp_dir("load-missing");
        let file = dir.join(SESSION_FILE_NAME);
        if file.exists() {
            fs::remove_file(&file).expect("remove stale session file");
        }
        let snapshot = load_from_dir(&dir).expect("load from missing session file");
        assert_eq!(snapshot, SessionSnapshot::default());
    }
}
