//! W045/W035 backfill journeys: interactive `wtd` demos for already-shipped
//! bindings and affordances that previously had only unit coverage.

use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::support::{Harness, repo_root};

const WORKSPACE_YAML: &str = ".wtd/oxide-backfill.yaml";
const WORKSPACE_NAME: &str = "oxide-backfill";

const EMPTY_PANE: &str = "empty/oxide-empty-pane";
const THIN_SLICE_PANE: &str = "thin-slice/oxide-thin-pane";

fn backfill_root() -> PathBuf {
    repo_root()
        .join("target")
        .join("test-workspaces")
        .join("wtd-backfill")
}

fn seeded_module_path() -> PathBuf {
    backfill_root().join("Module1.bas")
}

fn seed_backfill_workspace() {
    let root = backfill_root();
    fs::create_dir_all(&root).expect("create WTD backfill workspace");

    let basproj_src = repo_root()
        .join("examples")
        .join("thin-slice")
        .join("ThinSliceHello.basproj");
    let basproj_dst = root.join("ThinSliceHello.basproj");
    copy_text_file(&basproj_src, &basproj_dst);

    // Purpose-built fixture for the WTD backfill journeys:
    // - keeps `Main` + `ComputeAnswer` so F12 goto-def has a resolvable target,
    // - still matches the expected thin-slice project shape.
    fs::write(
        root.join("Module1.bas"),
        r#"Attribute VB_Name = "Module1"
Option Explicit

Public Sub Main()
    Dim answer As Integer
    answer = ComputeAnswer()
    Debug.Print answer
End Sub

Public Function ComputeAnswer() As Integer
    ComputeAnswer = 40 + 2
End Function
"#,
    )
    .expect("seed Module1.bas for WTD backfill");
}

fn copy_text_file(src: &Path, dst: &Path) {
    let text = fs::read_to_string(src)
        .unwrap_or_else(|err| panic!("read fixture {}: {err}", src.display()));
    fs::write(dst, text).unwrap_or_else(|err| panic!("write fixture {}: {err}", dst.display()));
}

fn open_backfill_harness() -> Harness {
    seed_backfill_workspace();
    Harness::open(WORKSPACE_YAML, WORKSPACE_NAME)
}

fn wait_for_empty_ready(harness: &Harness) {
    harness.wait_for_text(EMPTY_PANE, "A terminal-native IDE for OxVba.");
    harness.wait_for_text(EMPTY_PANE, "Open Project (Ctrl+O)");
    harness.wait_for_stable_frame(EMPTY_PANE);
}

fn wait_for_loaded_ready(harness: &Harness) {
    harness.wait_for_text(THIN_SLICE_PANE, "ThinSliceHello | Editing");
    harness.wait_for_text(THIN_SLICE_PANE, "Module1.bas | Primary View");
    harness.wait_for_stable_frame(THIN_SLICE_PANE);
}

fn send_repeated_key(harness: &Harness, pane: &str, key: &str, count: usize) {
    for _ in 0..count {
        harness.send_keys(pane, &[key]);
    }
}

fn select_palette_command(harness: &Harness, pane: &str, label: &str) {
    harness.wait_for_text(pane, "Command Palette");
    for _ in 0..32 {
        let capture = harness.capture_text(pane);
        if capture
            .lines()
            .any(|line| line.contains(&format!("> {label}")))
        {
            harness.send_keys(pane, &["Enter"]);
            return;
        }
        harness.send_keys(pane, &["Down"]);
    }
    panic!("palette command {label:?} was never selected after 32 Down presses");
}

#[test]
fn journey_ctrl_o_opens_selected_project_from_empty_scene() {
    let harness = open_backfill_harness();
    wait_for_empty_ready(&harness);

    harness.send_keys(EMPTY_PANE, &["Ctrl+O"]);
    harness.wait_for_text(EMPTY_PANE, " | Editing |");
    harness.wait_for_text(EMPTY_PANE, "Module1.bas | Primary View");

    let capture = harness.capture_text(EMPTY_PANE);
    assert!(
        !capture.contains("A terminal-native IDE for OxVba."),
        "Ctrl+O must leave the Welcome-only Empty scene and mount a workspace:\n{capture}"
    );
}

#[test]
fn journey_ctrl_n_scaffolds_and_mounts_new_project_from_empty_scene() {
    let harness = open_backfill_harness();
    wait_for_empty_ready(&harness);

    harness.send_keys(EMPTY_PANE, &["Ctrl+N"]);
    harness.wait_for_text(EMPTY_PANE, " | Editing |");
    harness.wait_for_text(EMPTY_PANE, "Public Sub Main()");
    let capture = harness.capture_text(EMPTY_PANE);
    assert!(
        capture.contains("NewProject"),
        "Ctrl+N must mount a scaffolded NewProject* workspace:\n{capture}"
    );
}

#[test]
fn journey_ctrl_s_persists_active_buffer_and_clears_dirty_marker() {
    let harness = open_backfill_harness();
    wait_for_loaded_ready(&harness);

    harness.send_text(THIN_SLICE_PANE, "Z");
    harness.wait_for_text(THIN_SLICE_PANE, "Module1.bas * | Primary View");

    harness.send_keys(THIN_SLICE_PANE, &["Ctrl+S"]);
    harness.wait_for_text(THIN_SLICE_PANE, "Module1.bas | Primary View");

    let capture = harness.capture_text(THIN_SLICE_PANE);
    assert!(
        !capture.contains("Module1.bas * | Primary View"),
        "Ctrl+S must clear the dirty marker:\n{capture}"
    );

    let disk = fs::read_to_string(seeded_module_path()).expect("read seeded module after Ctrl+S");
    assert!(
        disk.starts_with('Z'),
        "Ctrl+S should persist the typed prefix to disk, got:\n{disk}"
    );
}

#[test]
fn journey_ctrl_shift_s_save_all_dispatches_and_clears_dirty_marker() {
    let harness = open_backfill_harness();
    wait_for_loaded_ready(&harness);

    harness.send_text(THIN_SLICE_PANE, "Y");
    harness.wait_for_text(THIN_SLICE_PANE, "Module1.bas * | Primary View");

    harness.send_keys(THIN_SLICE_PANE, &["Ctrl+Shift+S"]);
    harness.wait_for_text(THIN_SLICE_PANE, "Module1.bas | Primary View");

    let capture = harness.capture_text(THIN_SLICE_PANE);
    assert!(
        !capture.contains("Module1.bas * | Primary View"),
        "Ctrl+Shift+S must clear dirty marker via Save All:\n{capture}"
    );

    let disk =
        fs::read_to_string(seeded_module_path()).expect("read seeded module after Ctrl+Shift+S");
    assert!(
        disk.starts_with('Y'),
        "Ctrl+Shift+S should persist the typed prefix to disk, got:\n{disk}"
    );
}

#[test]
fn journey_ctrl_z_ctrl_y_undo_and_redo_in_editor() {
    let harness = open_backfill_harness();
    wait_for_loaded_ready(&harness);

    harness.send_text(THIN_SLICE_PANE, "Q");
    harness.wait_for_text(THIN_SLICE_PANE, "QAttribute VB_Name");

    harness.send_keys(THIN_SLICE_PANE, &["Ctrl+Z"]);
    harness.wait_for_text(THIN_SLICE_PANE, "Attribute VB_Name = \"Module1\"");
    let after_undo = harness.capture_text(THIN_SLICE_PANE);
    assert!(
        !after_undo.contains("QAttribute VB_Name"),
        "Ctrl+Z must remove the inserted prefix:\n{after_undo}"
    );

    harness.send_keys(THIN_SLICE_PANE, &["Ctrl+Y"]);
    harness.wait_for_text(THIN_SLICE_PANE, "QAttribute VB_Name");
}

#[test]
fn journey_f1_hover_supports_fallback_and_resolvable_targets() {
    let harness = open_backfill_harness();
    wait_for_loaded_ready(&harness);

    // Fallback: line 3 is blank in the seeded fixture.
    send_repeated_key(&harness, THIN_SLICE_PANE, "Down", 2);
    harness.send_keys(THIN_SLICE_PANE, &["F1"]);
    harness.wait_for_text(
        THIN_SLICE_PANE,
        "No hover information available at this position.",
    );
    harness.send_keys(THIN_SLICE_PANE, &["Esc"]);

    // Resolvable: `Main` on `Public Sub Main()`.
    send_repeated_key(&harness, THIN_SLICE_PANE, "Down", 1);
    send_repeated_key(&harness, THIN_SLICE_PANE, "Right", 12);
    harness.send_keys(THIN_SLICE_PANE, &["F1"]);
    harness.wait_for_text(THIN_SLICE_PANE, "Public Sub Main()");
}

#[test]
fn journey_f12_goto_definition_surfaces_fallback_when_unresolved() {
    let harness = open_backfill_harness();
    wait_for_loaded_ready(&harness);

    // Line 3 is blank in the seeded fixture; F12 here should produce
    // explicit fallback feedback instead of a silent no-op.
    send_repeated_key(&harness, THIN_SLICE_PANE, "Down", 2);
    harness.send_keys(THIN_SLICE_PANE, &["F12"]);
    harness.wait_for_text(
        THIN_SLICE_PANE,
        "No definition target available at this position.",
    );
    harness.send_keys(THIN_SLICE_PANE, &["Esc"]);

    // Resolvable position smoke-check: on `Main` in `Public Sub Main()`,
    // F12 should keep the scene stable and must not re-open the fallback.
    send_repeated_key(&harness, THIN_SLICE_PANE, "Down", 1);
    send_repeated_key(&harness, THIN_SLICE_PANE, "Right", 12);
    harness.send_keys(THIN_SLICE_PANE, &["F12"]);
    harness.wait_for_text(THIN_SLICE_PANE, "ThinSliceHello | Editing |");
    let capture = harness.capture_text(THIN_SLICE_PANE);
    assert!(
        !capture.contains("No definition target available at this position."),
        "F12 on a symbol should not fall back to unresolved message:\n{capture}"
    );
}

#[test]
fn journey_f5_runs_and_esc_returns_to_editing_scene() {
    let harness = open_backfill_harness();
    wait_for_loaded_ready(&harness);

    harness.send_keys(THIN_SLICE_PANE, &["F5"]);
    harness.wait_for_text(THIN_SLICE_PANE, "ThinSliceHello | Run |");
    harness.wait_for_text(THIN_SLICE_PANE, "Lower Surface Output");

    harness.send_keys(THIN_SLICE_PANE, &["Esc"]);
    harness.wait_for_text(THIN_SLICE_PANE, "ThinSliceHello | Editing |");
    harness.wait_for_text(THIN_SLICE_PANE, "Lower Surface Problems");
}

#[test]
fn journey_f6_palette_enter_dispatches_selected_command() {
    let harness = open_backfill_harness();
    wait_for_loaded_ready(&harness);

    harness.send_text(THIN_SLICE_PANE, "S");
    harness.wait_for_text(THIN_SLICE_PANE, "Module1.bas * | Primary View");

    harness.send_keys(THIN_SLICE_PANE, &["F6"]);
    select_palette_command(&harness, THIN_SLICE_PANE, "Save");

    harness.wait_for_text(THIN_SLICE_PANE, "ThinSliceHello | Editing |");
    let capture = harness.capture_text(THIN_SLICE_PANE);
    assert!(
        !capture.contains("Module1.bas * | Primary View"),
        "palette Enter-dispatch on Save must clear dirty marker:\n{capture}"
    );
}

#[test]
fn journey_empty_scene_palette_preserves_welcome_as_overlay_backing() {
    let harness = open_backfill_harness();
    wait_for_empty_ready(&harness);

    harness.send_keys(EMPTY_PANE, &["F6"]);
    harness.wait_for_text(EMPTY_PANE, "Command Palette");

    let capture = harness.capture_text(EMPTY_PANE);
    assert!(
        capture.contains("A terminal-native IDE for OxVba."),
        "palette opened from Empty must keep Welcome as the backing scene:\n{capture}"
    );
    assert!(
        capture.contains("Start"),
        "Empty backing should still show the Welcome Start section:\n{capture}"
    );
    assert!(
        !capture.contains("Payroll.basproj"),
        "Empty + F6 must not leak the mock Editing backing scene:\n{capture}"
    );
}

#[test]
fn journey_overlay_and_run_transitions_preserve_in_flight_dirty_state() {
    let harness = open_backfill_harness();
    wait_for_loaded_ready(&harness);

    harness.send_text(THIN_SLICE_PANE, "P");
    harness.wait_for_text(THIN_SLICE_PANE, "Module1.bas * | Primary View");

    harness.send_keys(THIN_SLICE_PANE, &["F6"]);
    harness.wait_for_text(THIN_SLICE_PANE, "Command Palette");
    harness.send_keys(THIN_SLICE_PANE, &["Esc"]);
    harness.wait_for_text(THIN_SLICE_PANE, "ThinSliceHello | Editing |");

    harness.send_keys(THIN_SLICE_PANE, &["F5"]);
    harness.wait_for_text(THIN_SLICE_PANE, "ThinSliceHello | Run |");
    harness.send_keys(THIN_SLICE_PANE, &["Esc"]);
    harness.wait_for_text(THIN_SLICE_PANE, "ThinSliceHello | Editing |");

    let capture = harness.capture_text(THIN_SLICE_PANE);
    assert!(
        capture.contains("Module1.bas * | Primary View"),
        "dirty marker must survive overlay open/close and F5/Esc scene transitions:\n{capture}"
    );
}
