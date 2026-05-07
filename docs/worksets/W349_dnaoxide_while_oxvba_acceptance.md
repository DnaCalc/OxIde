# Workset W349 — DnaOxIde While-OxVba Acceptance

## Ambition

Accept the OxIde-side continuation runway that can proceed while OxVba implements and hardens the requested full-scope APIs. W349 is a consolidation workset, not a new capability lane: it verifies W341-W348 compose into a coherent DnaOxIde host direction, including available-subset OxVba adapter evidence where present, without overclaiming pending OxVba behavior.

## Dependencies

- W341 — DnaOxIde Tauri app scaffold.
- W342 — shared IDE UI component layer.
- W343 — OxIde host bridge facade.
- W344 — DnaOxIde Tauri command boundary stubs.
- W345 — DnaOxIde live host UI proof.
- W346 — DnaOxIde interaction and e2e harness.
- W347 — compile options and reference UI placeholders/subset panels.
- W348 — DnaOneCalc shared UI reuse path.
- [`docs/HANDOFF_DNAOXIDE_OXVBA_FEEDBACK_ALIGNMENT.md`](../HANDOFF_DNAOXIDE_OXVBA_FEEDBACK_ALIGNMENT.md).

## Design

W349 gathers evidence that the eight continuation areas are ready for OxVba integration and aligned with the confirmed OxVba feedback:

1. DnaOxIde scaffold exists and is branded.
2. Shared UI renders accepted IDE slices.
3. Host bridge facade separates UI from host services.
4. Tauri command stubs cover proven lifecycle, available-subset OxVba adapters, and pending-hardening unavailable services.
5. Live host UI proof is reviewable.
6. Interaction/e2e harness covers command/focus/lifecycle/blocked-service and subset-backed flows.
7. Compile/options/reference placeholders/subset panels are ready for real OxVba DTOs.
8. DnaOneCalc reuse path is preserved without sibling writes.

## Beads

### W349-B00 — Cross-workset evidence audit

Goal:
  Gather W341-W348 evidence and identify any missing acceptance artifacts.

Design:
  - Review target evidence files.
  - Confirm fixture mutation guards.
  - Confirm no-claim scans exist.
  - Confirm subset-backed adapter evidence does not flip full capability claims.

Tests:
  - Evidence file grep.
  - Workset coverage grep for items 1-8.

Evidence:
  - `target/w349-evidence-audit.txt`.

Closure:
  - [ ] W341-W348 evidence is present.
  - [ ] Missing artifacts are documented.
  - [ ] Claim boundaries and subset labels are intact.

### W349-B01 — Full continuation regression

Goal:
  Run the full OxIde-side regression suite for the continuation runway.

Design:
  - Run nested GUI workspace tests.
  - Run GUI-lab renders for accepted scenarios and new host/reuse scenarios.
  - Run interaction harness if available.

Tests:
  - `cargo test --manifest-path crates/Cargo.toml --workspace`.
  - DnaOxIde scaffold/host checks.
  - GUI-lab renders.
  - Anti-overclaim scan.

Evidence:
  - `target/w349-regression.txt`.

Closure:
  - [ ] Workspace tests pass.
  - [ ] Host/reuse renders pass.
  - [ ] Anti-overclaim scan passes.

### W349-B02 — OxVba integration readiness report

Goal:
  Summarize exactly what can be connected when OxVba APIs arrive.

Design:
  - Map W355/W360/W365/W370 OxVba requirements to existing UI/host surfaces.
  - Map confirmed available-subset OxVba surfaces to adopted OxIde adapters.
  - List remaining blockers and required authorization.

Tests:
  - Documentation grep for compile/build, COM, runtime, Immediate, debug/watch/breakpoint readiness.

Evidence:
  - Readiness report doc or section update.

Closure:
  - [ ] Readiness report exists.
  - [ ] OxVba integration and available-subset adoption points are mapped.
  - [ ] Authorization gates remain explicit.

### W349-B03 — W349 acceptance

Goal:
  Close the while-OxVba runway and choose the next workset based on OxVba readiness.

Design:
  - If OxVba APIs are ready/authorized, proceed to real integration work.
  - If not, continue polish/packaging only without false runtime claims.

Tests:
  - W349 regression suite.
  - Final no-claim scan.

Evidence:
  - W349 acceptance outputs.

Closure:
  - [ ] W341-W348 are accepted or explicitly blocked.
  - [ ] Next integration step is known.
  - [ ] No fake capability has landed.

## Out-of-scope

- New feature implementation beyond consolidating W341-W348.
- Real OxVba runtime/debug/Immediate/COM behavior unless separately authorized and tested.
- Sibling repo writes.
