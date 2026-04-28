//! W039 Fire Horse UX-lab journeys.

use crate::support::{Harness, LabScenarioJourney, assert_golden_text, assert_golden_vt};

const WORKSET: &str = "W039";

const EDITING_LENS: LabScenarioJourney = LabScenarioJourney::new(
    ".wtd/oxide-firehorse-editing.yaml",
    "oxide-firehorse-editing",
    "editing-lens/oxide-firehorse-editing-pane",
    "firehorse",
    "firehorse-editing-lens-standard",
    "standard",
);

const COMMAND_LENS: LabScenarioJourney = LabScenarioJourney::new(
    ".wtd/oxide-firehorse-command.yaml",
    "oxide-firehorse-command",
    "command-lens/oxide-firehorse-command-pane",
    "firehorse",
    "firehorse-command-lens-standard",
    "standard",
);

const RUN_LANE: LabScenarioJourney = LabScenarioJourney::new(
    ".wtd/oxide-firehorse-run.yaml",
    "oxide-firehorse-run",
    "run-lane/oxide-firehorse-run-pane",
    "firehorse",
    "firehorse-run-lane-standard",
    "standard",
);

const DEBUG_COCKPIT: LabScenarioJourney = LabScenarioJourney::new(
    ".wtd/oxide-firehorse-debug.yaml",
    "oxide-firehorse-debug",
    "debug-cockpit/oxide-firehorse-debug-pane",
    "firehorse",
    "firehorse-debug-cockpit-standard",
    "standard",
);

const LAUNCHPAD: LabScenarioJourney = LabScenarioJourney::new(
    ".wtd/oxide-firehorse-launchpad.yaml",
    "oxide-firehorse-launchpad",
    "launchpad/oxide-firehorse-launchpad-pane",
    "firehorse",
    "firehorse-launchpad-standard",
    "standard",
);

const CONSOLE_FIT: LabScenarioJourney = LabScenarioJourney::new(
    ".wtd/oxide-firehorse-console.yaml",
    "oxide-firehorse-console",
    "console-fit/oxide-firehorse-console-pane",
    "firehorse",
    "firehorse-console-fit-light",
    "standard",
);

const COMPACT_FOCUS: LabScenarioJourney = LabScenarioJourney::new(
    ".wtd/oxide-firehorse-focus.yaml",
    "oxide-firehorse-focus",
    "compact-focus/oxide-firehorse-focus-pane",
    "firehorse",
    "firehorse-focus-compact",
    "compact",
);

const REAL_EDITING: LabScenarioJourney = LabScenarioJourney::new(
    ".wtd/oxide-firehorse-real-editing.yaml",
    "oxide-firehorse-real-editing",
    "real-editing/oxide-firehorse-real-editing-pane",
    "firehorse",
    "firehorse-real-editing",
    "standard",
);

#[test]
fn firehorse_editing_lens_matches_golden() {
    let harness = Harness::open_lab_once(&EDITING_LENS);

    let text = harness.capture_lab_once_text(
        &EDITING_LENS,
        &[
            "Project Spine",
            "Code Canvas",
            "PriceFor",
            "Context Dock",
            "Activity: Problems",
            "F6 Command Lens",
        ],
    );
    let vt = harness.capture_lab_once_vt(&EDITING_LENS);

    assert!(text.contains("Identity Rail"));
    assert!(text.contains("Source Lens"));
    assert!(text.contains("HostWorkspaceSession::hover"));

    assert_golden_text(WORKSET, "firehorse_editing_lens_standard", &text);
    assert_golden_vt(WORKSET, "firehorse_editing_lens_standard", &vt);
}

#[test]
fn firehorse_command_lens_matches_golden() {
    let harness = Harness::open_lab_once(&COMMAND_LENS);

    let text = harness.capture_lab_once_text(
        &COMMAND_LENS,
        &[
            "Overlay: Command Lens",
            "Run Project",
            "Stop Run",
            "No active run",
            "Enter run",
        ],
    );
    let vt = harness.capture_lab_once_vt(&COMMAND_LENS);

    assert!(text.contains("run.start"));
    assert!(text.contains("run.stop"));
    assert!(text.contains("Preview: Run Project"));

    assert_golden_text(WORKSET, "firehorse_command_lens_standard", &text);
    assert_golden_vt(WORKSET, "firehorse_command_lens_standard", &vt);
}

#[test]
fn firehorse_run_lane_matches_golden() {
    let harness = Harness::open_lab_once(&RUN_LANE);

    let text = harness.capture_lab_once_text(
        &RUN_LANE,
        &[
            "Run Lane",
            "Activity: Run Timeline",
            "ExcelDesktop",
            "Build",
            "F8 Stop Run",
        ],
    );
    let vt = harness.capture_lab_once_vt(&RUN_LANE);

    assert!(text.contains("Prepare"));
    assert!(text.contains("Analyze"));
    assert!(text.contains("Execute"));

    assert_golden_text(WORKSET, "firehorse_run_lane_standard", &text);
    assert_golden_vt(WORKSET, "firehorse_run_lane_standard", &vt);
}

#[test]
fn firehorse_debug_cockpit_matches_golden() {
    let harness = Harness::open_lab_once(&DEBUG_COCKPIT);

    let text = harness.capture_lab_once_text(
        &DEBUG_COCKPIT,
        &[
            "Debug Cockpit",
            "paused PriceFor.bas:8",
            "Call Stack",
            "Locals",
            "Watches",
            "F5 Continue",
        ],
    );
    let vt = harness.capture_lab_once_vt(&DEBUG_COCKPIT);

    assert!(text.contains("F8 Step"));
    assert!(text.contains("Esc Return"));
    assert!(text.contains("OxVba debug contract"));

    assert_golden_text(WORKSET, "firehorse_debug_cockpit_standard", &text);
    assert_golden_vt(WORKSET, "firehorse_debug_cockpit_standard", &vt);
}

#[test]
fn firehorse_launchpad_matches_golden() {
    let harness = Harness::open_lab_once(&LAUNCHPAD);

    let text = harness.capture_lab_once_text(
        &LAUNCHPAD,
        &[
            "Identity Rail",
            "Open Project",
            "NorthwindPricing",
            "ExcelDesktop",
            "F10 Console Fit",
        ],
    );
    let vt = harness.capture_lab_once_vt(&LAUNCHPAD);

    assert!(text.contains("Launchpad"));
    assert!(text.contains("SessionStore fixture"));
    assert!(text.contains("ProjectSession unavailable"));

    assert_golden_text(WORKSET, "firehorse_launchpad_standard", &text);
    assert_golden_vt(WORKSET, "firehorse_launchpad_standard", &vt);
}

#[test]
fn firehorse_console_fit_matches_golden() {
    let harness = Harness::open_lab_once(&CONSOLE_FIT);

    let text = harness.capture_lab_once_text(
        &CONSOLE_FIT,
        &[
            "Identity Rail",
            "Labels visible without color",
            "truecolor",
            "box-glyphs",
            "Prefer ASCII rail fallback",
        ],
    );
    let vt = harness.capture_lab_once_vt(&CONSOLE_FIT);

    assert!(text.contains("Console Fit"));
    assert!(text.contains("pass"));
    assert!(text.contains("warn"));
    assert!(text.contains("not live probes"));

    assert_golden_text(WORKSET, "firehorse_console_fit_light", &text);
    assert_golden_vt(WORKSET, "firehorse_console_fit_light", &vt);
}

#[test]
fn firehorse_compact_focus_matches_golden() {
    let harness = Harness::open_lab_once(&COMPACT_FOCUS);

    let text = harness.capture_lab_once_text(
        &COMPACT_FOCUS,
        &[
            "Identity Rail",
            "Project Spine: hidden",
            "Code Canvas | PriceFor.bas",
            "Activity Rail: Problems",
            "Alt+1 Project",
        ],
    );
    let vt = harness.capture_lab_once_vt(&COMPACT_FOCUS);

    assert!(text.contains("Compact Focus"));
    assert!(text.contains("Context Dock: hidden"));
    assert!(text.contains("F6 Command"));

    assert_golden_text(WORKSET, "firehorse_focus_compact", &text);
    assert_golden_vt(WORKSET, "firehorse_focus_compact", &vt);
}

#[test]
fn firehorse_real_editing_adapter_matches_golden() {
    let harness = Harness::open_lab_once(&REAL_EDITING);

    let text = harness.capture_lab_once_text(
        &REAL_EDITING,
        &[
            "Identity Rail",
            "ThinSliceHello",
            "Module1.bas",
            "Code Canvas",
            "Unavailable seam",
        ],
    );
    let vt = harness.capture_lab_once_vt(&REAL_EDITING);

    assert!(text.contains("Project Spine"));
    assert!(text.contains("Public Sub Main"));
    assert!(text.contains("Ctrl+S Save"));

    assert_golden_text(WORKSET, "firehorse_real_editing_adapter", &text);
    assert_golden_vt(WORKSET, "firehorse_real_editing_adapter", &vt);
}
