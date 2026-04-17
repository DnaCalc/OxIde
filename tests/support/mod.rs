//! WinTermDriver test harness for OxIde.
//!
//! Drives the `wtd` CLI to launch OxIde in a ConPTY under `wtd-host`, capture
//! the rendered screen, and compare against committed goldens.
//!
//! Owned by W037. See `docs/TESTING_WTD.md` for the development loop.

#![allow(dead_code)]

use std::{
    env,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::{Mutex, MutexGuard, OnceLock},
    thread,
    time::{Duration, Instant},
};

/// Serializes all `Harness` instances across a test binary.
///
/// wtd workspaces are keyed by name; two tests opening the same name with
/// `--recreate` in parallel would tear down each other's panes. Tests can
/// still run as a parallel suite (the guard is only held for the lifetime of
/// one `Harness`), but within each test the workspace is owned exclusively.
fn harness_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// Default capture polling interval.
const POLL_INTERVAL: Duration = Duration::from_millis(200);

/// Default overall timeout for `wait_for_text` and `wait_for_stable_frame`.
const DEFAULT_WAIT_TIMEOUT: Duration = Duration::from_secs(20);

/// Stable-frame detection compares this many consecutive captures.
const STABLE_WINDOW: Duration = Duration::from_millis(800);

/// Whether the caller wants to overwrite goldens instead of asserting them.
pub fn update_goldens() -> bool {
    env::var("UPDATE_GOLDENS")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Path to the OxIde repo root.
pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Path to `tests/wtd/goldens/<workset>/`.
pub fn goldens_dir(workset: &str) -> PathBuf {
    repo_root().join("tests").join("wtd").join("goldens").join(workset)
}

/// A running wtd workspace for a single test.
///
/// On `Drop` the workspace is closed so panes do not leak between tests.
/// Holds a process-wide mutex for its lifetime so concurrent tests do not
/// tear down each other's workspace with `--recreate`.
pub struct Harness {
    workspace_name: String,
    _guard: MutexGuard<'static, ()>,
}

impl Harness {
    /// Open the workspace definition at `yaml_path` (relative to the repo root),
    /// returning a handle whose target strings are addressed as
    /// `<workspace_name>/<tab>/<pane>`.
    pub fn open(yaml_path: impl AsRef<Path>, workspace_name: &str) -> Self {
        let guard = harness_lock();

        let yaml = repo_root().join(yaml_path.as_ref());
        assert!(
            yaml.exists(),
            "workspace definition not found: {}",
            yaml.display()
        );

        // Close any lingering instance first so reruns are deterministic.
        let _ = wtd_command(&["close", workspace_name]);

        let out = wtd_command(&[
            "open",
            "--file",
            yaml.to_str().expect("non-utf8 yaml path"),
            "--recreate",
        ]);
        assert!(
            out.status.success(),
            "wtd open failed: stdout={} stderr={}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );

        Self {
            workspace_name: workspace_name.to_string(),
            _guard: guard,
        }
    }

    /// Capture the visible screen of `target_suffix` (e.g. `"empty/oxide-empty-pane"`).
    pub fn capture_text(&self, target_suffix: &str) -> String {
        let target = self.target(target_suffix);
        let out = wtd_command(&["capture", &target]);
        assert!(
            out.status.success(),
            "wtd capture failed for {target}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8_lossy(&out.stdout).into_owned()
    }

    /// Capture a replayable VT snapshot of `target_suffix`.
    pub fn capture_vt(&self, target_suffix: &str) -> Vec<u8> {
        let target = self.target(target_suffix);
        let out = wtd_command(&["capture", "--vt", &target]);
        assert!(
            out.status.success(),
            "wtd capture --vt failed for {target}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        out.stdout
    }

    /// Poll `target_suffix` until `needle` appears or `DEFAULT_WAIT_TIMEOUT` elapses.
    pub fn wait_for_text(&self, target_suffix: &str, needle: &str) {
        self.wait_for_text_with_timeout(target_suffix, needle, DEFAULT_WAIT_TIMEOUT);
    }

    pub fn wait_for_text_with_timeout(
        &self,
        target_suffix: &str,
        needle: &str,
        timeout: Duration,
    ) {
        let start = Instant::now();
        let mut last = String::new();
        while start.elapsed() < timeout {
            last = self.capture_text(target_suffix);
            if last.contains(needle) {
                return;
            }
            thread::sleep(POLL_INTERVAL);
        }
        panic!(
            "wait_for_text timeout after {:?} waiting for {:?} in {target_suffix}.\n--- last capture ---\n{last}",
            timeout, needle
        );
    }

    /// Wait until two consecutive captures taken `STABLE_WINDOW` apart are
    /// identical, or until `DEFAULT_WAIT_TIMEOUT` elapses.
    pub fn wait_for_stable_frame(&self, target_suffix: &str) {
        let start = Instant::now();
        let mut previous = self.capture_text(target_suffix);
        while start.elapsed() < DEFAULT_WAIT_TIMEOUT {
            thread::sleep(STABLE_WINDOW);
            let current = self.capture_text(target_suffix);
            if current == previous {
                return;
            }
            previous = current;
        }
        panic!(
            "wait_for_stable_frame timeout after {:?} for {target_suffix}.\n--- last capture ---\n{previous}",
            DEFAULT_WAIT_TIMEOUT
        );
    }

    fn target(&self, suffix: &str) -> String {
        format!("{}/{}", self.workspace_name, suffix)
    }
}

impl Drop for Harness {
    fn drop(&mut self) {
        let _ = wtd_command(&["close", &self.workspace_name]);
    }
}

fn wtd_command(args: &[&str]) -> Output {
    Command::new("wtd")
        .args(args)
        .output()
        .unwrap_or_else(|err| {
            panic!(
                "failed to invoke `wtd {}`: {err}. Is wtd on PATH and the host running?",
                args.join(" ")
            )
        })
}

/// Assert that `actual` matches the text golden at `goldens/<workset>/<name>.txt`.
///
/// Set `UPDATE_GOLDENS=1` to overwrite instead of asserting.
pub fn assert_golden_text(workset: &str, name: &str, actual: &str) {
    let path = goldens_dir(workset).join(format!("{name}.txt"));

    if update_goldens() {
        std::fs::create_dir_all(path.parent().unwrap())
            .expect("create goldens dir");
        std::fs::write(&path, actual).expect("write golden");
        eprintln!("updated golden: {}", path.display());
        return;
    }

    let expected = std::fs::read_to_string(&path).unwrap_or_else(|err| {
        panic!(
            "golden not found at {}: {err}. Run with UPDATE_GOLDENS=1 to create it.",
            path.display()
        )
    });

    if expected != actual {
        let diff = simple_line_diff(&expected, actual);
        panic!(
            "golden mismatch for {name}.txt\ngolden:   {}\ncaptured: (see below)\n\n{diff}",
            path.display()
        );
    }
}

/// Assert that `actual_vt` matches the VT golden at `goldens/<workset>/<name>.vt`.
///
/// VT comparison is byte-exact. Set `UPDATE_GOLDENS=1` to overwrite.
pub fn assert_golden_vt(workset: &str, name: &str, actual_vt: &[u8]) {
    let path = goldens_dir(workset).join(format!("{name}.vt"));

    if update_goldens() {
        std::fs::create_dir_all(path.parent().unwrap())
            .expect("create goldens dir");
        std::fs::write(&path, actual_vt).expect("write golden");
        eprintln!("updated VT golden: {}", path.display());
        return;
    }

    let expected = std::fs::read(&path).unwrap_or_else(|err| {
        panic!(
            "VT golden not found at {}: {err}. Run with UPDATE_GOLDENS=1 to create it.",
            path.display()
        )
    });

    if expected != actual_vt {
        panic!(
            "VT golden mismatch for {name}.vt at {} (byte-exact).\n\
             Re-inspect via `wtd capture --vt ...` and compare the paired .txt golden \
             for a human-readable view. Run UPDATE_GOLDENS=1 to re-bless after review.",
            path.display()
        );
    }
}

/// Produce a compact line-level diff suitable for test panics.
/// Prefix `-` = expected, `+` = actual, ` ` = matching.
fn simple_line_diff(expected: &str, actual: &str) -> String {
    let expected_lines: Vec<&str> = expected.lines().collect();
    let actual_lines: Vec<&str> = actual.lines().collect();
    let mut out = String::new();
    let max = expected_lines.len().max(actual_lines.len());
    for i in 0..max {
        let e = expected_lines.get(i).copied().unwrap_or("");
        let a = actual_lines.get(i).copied().unwrap_or("");
        if e == a {
            out.push_str("  ");
            out.push_str(e);
        } else {
            out.push_str("- ");
            out.push_str(e);
            out.push('\n');
            out.push_str("+ ");
            out.push_str(a);
        }
        out.push('\n');
    }
    out
}
