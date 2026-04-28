//! W037/W038 smoke scenarios: verify OxIde renders committed shell goldens and
//! the UX lab once runner exposes its smoke frame under WinTermDriver.
//!
//! Gated behind the `wtd` cargo feature so the default `cargo test` loop stays
//! fast. Run with:
//!
//!     cargo test --features wtd --test wtd_smoke
//!
//! Re-bless after a reviewed UX change:
//!
//!     UPDATE_GOLDENS=1 cargo test --features wtd --test wtd_smoke
//!
//! See docs/TESTING_WTD.md for the full loop.

#[path = "wtd/backfill.rs"]
mod backfill;
#[path = "wtd/firehorse.rs"]
mod firehorse;
mod support;

use support::LabScenarioJourney;
use support::{Harness, assert_golden_text, assert_golden_vt};

const WORKSPACE_YAML: &str = ".wtd/oxide-smoke.yaml";
const WORKSPACE_NAME: &str = "oxide-smoke";
const WORKSET: &str = "W037";
const UXLAB_WORKSET: &str = "W038";
const UXLAB_WORKSPACE_YAML: &str = ".wtd/oxide-uxlab-smoke.yaml";
const UXLAB_WORKSPACE_NAME: &str = "oxide-uxlab-smoke";

const EMPTY_PANE: &str = "empty/oxide-empty-pane";
const THIN_SLICE_PANE: &str = "thin-slice/oxide-thin-pane";
const UXLAB_SMOKE_PANE: &str = "uxlab-smoke/oxide-uxlab-smoke-pane";
const UXLAB_SMOKE_JOURNEY: LabScenarioJourney = LabScenarioJourney::new(
    UXLAB_WORKSPACE_YAML,
    UXLAB_WORKSPACE_NAME,
    UXLAB_SMOKE_PANE,
    "lab-smoke",
    "lab-smoke-editing",
    "standard",
);

/// The empty-scene welcome pane must render its launcher, welcome body, and
/// environment side pane before we compare against the golden.
#[test]
fn empty_scene_matches_golden() {
    let harness = Harness::open(WORKSPACE_YAML, WORKSPACE_NAME);

    // Wait for the welcome body to paint before capturing.
    harness.wait_for_text(EMPTY_PANE, "A terminal-native IDE for OxVba.");
    harness.wait_for_stable_frame(EMPTY_PANE);

    let text = harness.capture_text(EMPTY_PANE);
    let vt = harness.capture_vt(EMPTY_PANE);

    assert_golden_text(WORKSET, "empty", &text);
    assert_golden_vt(WORKSET, "empty", &vt);
}

/// Opening `ThinSliceHello.basproj` must present the editing scene with the
/// project explorer populated and the `Module1.bas` buffer in the editor.
#[test]
fn thin_slice_loaded_matches_golden() {
    let harness = Harness::open(WORKSPACE_YAML, WORKSPACE_NAME);

    // The thin-slice pane loads a real project through OxVba's HostWorkspaceSession;
    // wait for both the module list and the editor body to be painted.
    harness.wait_for_text(THIN_SLICE_PANE, "Module1 [Module]");
    harness.wait_for_text(THIN_SLICE_PANE, "Public Sub Main()");
    harness.wait_for_stable_frame(THIN_SLICE_PANE);

    let text = harness.capture_text(THIN_SLICE_PANE);
    let vt = harness.capture_vt(THIN_SLICE_PANE);

    assert_golden_text(WORKSET, "thin_slice_loaded", &text);
    assert_golden_vt(WORKSET, "thin_slice_loaded", &vt);
}

/// The W038 Phase 1 lab runner must expose a stable smoke frame through the
/// same release-binary WTD path future Fire Horse scenarios will reuse.
#[test]
fn uxlab_once_smoke_renders_visible_contracts() {
    let harness = Harness::open_lab_once(&UXLAB_SMOKE_JOURNEY);

    let text = harness.capture_lab_once_text(
        &UXLAB_SMOKE_JOURNEY,
        &[
            "Lab Smoke Editing",
            "viewport: standard 120x34",
            "F6 Command Lens",
        ],
    );
    let vt = harness.capture_lab_once_vt(&UXLAB_SMOKE_JOURNEY);
    assert!(text.contains("Project Spine | Code Canvas | Context Dock"));
    assert!(text.contains("Debug.Print \"W038 lab smoke\""));

    assert_golden_text(UXLAB_WORKSET, "uxlab_once_smoke", &text);
    assert_golden_vt(UXLAB_WORKSET, "uxlab_once_smoke", &vt);
}
