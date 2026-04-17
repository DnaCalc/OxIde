//! W037 smoke scenario: verify OxIde renders the empty scene and the thin-slice
//! loaded scene under WinTermDriver, and that both match committed goldens.
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

mod support;

use support::{Harness, assert_golden_text, assert_golden_vt};

const WORKSPACE_YAML: &str = ".wtd/oxide-smoke.yaml";
const WORKSPACE_NAME: &str = "oxide-smoke";
const WORKSET: &str = "W037";

const EMPTY_PANE: &str = "empty/oxide-empty-pane";
const THIN_SLICE_PANE: &str = "thin-slice/oxide-thin-pane";

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
