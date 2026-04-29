//! W041 Audit Lab journeys.

use crate::support::Harness;

const STUDIO_WORKSPACE_YAML: &str = ".wtd/oxide-audit-lab-studio.yaml";
const STUDIO_WORKSPACE_NAME: &str = "oxide-audit-lab-studio";
const STUDIO_PANE: &str = "audit-lab/oxide-audit-lab-studio-pane";

const FIRST_CLASS_WORKSPACE_YAML: &str = ".wtd/oxide-audit-lab-first-class.yaml";
const FIRST_CLASS_WORKSPACE_NAME: &str = "oxide-audit-lab-first-class";
const FIRST_CLASS_PANE: &str = "audit-lab/oxide-audit-lab-first-class-pane";

#[test]
fn audit_lab_studio_regions_and_selection_update() {
    let harness = Harness::open(STUDIO_WORKSPACE_YAML, STUDIO_WORKSPACE_NAME);

    wait_for_cockpit(&harness, STUDIO_PANE);
    let initial = harness.capture_text(STUDIO_PANE);
    assert!(initial.contains("firehorse-editing-lens-standard"));
    assert!(initial.contains("focus Scenario Atlas"));
    assert!(initial.contains("Audit Dossier | Persona"));

    harness.send_text(STUDIO_PANE, "j");
    harness.wait_for_text(STUDIO_PANE, "Selected firehorse-command-lens-standard");
    harness.send_keys(STUDIO_PANE, &["Enter"]);
    harness.wait_for_text(STUDIO_PANE, "Opened Command Lens");

    let changed = harness.capture_text(STUDIO_PANE);
    assert!(changed.contains("firehorse-command-lens-standard"));
    assert!(changed.contains("run.start"));
    assert!(changed.contains("Run Project"));
}

#[test]
fn audit_lab_first_class_lenses_marks_and_mode_switches() {
    let harness = Harness::open(FIRST_CLASS_WORKSPACE_YAML, FIRST_CLASS_WORKSPACE_NAME);

    wait_for_cockpit(&harness, FIRST_CLASS_PANE);
    harness.wait_for_text(FIRST_CLASS_PANE, "first-class");

    harness.send_text(FIRST_CLASS_PANE, "2");
    harness.wait_for_text(FIRST_CLASS_PANE, "Audit Dossier | Journey");
    harness.wait_for_text(FIRST_CLASS_PANE, "actions:");

    harness.send_text(FIRST_CLASS_PANE, "3");
    harness.wait_for_text(FIRST_CLASS_PANE, "Audit Dossier | Mapping");
    harness.wait_for_text(FIRST_CLASS_PANE, "HostWorkspaceSession::diagnostics");

    harness.send_text(FIRST_CLASS_PANE, "4");
    harness.wait_for_text(FIRST_CLASS_PANE, "Audit Dossier | Checklist");
    harness.send_text(FIRST_CLASS_PANE, "c");
    harness.wait_for_text(FIRST_CLASS_PANE, "Marked functional.persona_fit as Concern");

    harness.send_text(FIRST_CLASS_PANE, "5");
    harness.wait_for_text(FIRST_CLASS_PANE, "Audit Dossier | Findings");
    harness.wait_for_text(FIRST_CLASS_PANE, "functional.persona_fit concern");

    harness.send_text(FIRST_CLASS_PANE, "v");
    harness.wait_for_text(FIRST_CLASS_PANE, "viewport: standard");
    harness.send_text(FIRST_CLASS_PANE, "r");
    harness.wait_for_text(FIRST_CLASS_PANE, "render: contract");
}

fn wait_for_cockpit(harness: &Harness, pane: &str) {
    harness.wait_for_text(pane, "OxIde UX Audit Lab");
    harness.wait_for_text(pane, "Scenario Atlas");
    harness.wait_for_text(pane, "Live Fire Horse Stage");
    harness.wait_for_text(pane, "Audit Dossier");
    harness.wait_for_text(pane, "Evidence Rail");
    harness.wait_for_stable_frame(pane);
}
