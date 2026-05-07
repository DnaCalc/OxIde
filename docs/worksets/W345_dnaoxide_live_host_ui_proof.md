# Workset W345 — DnaOxIde Live Host UI Proof

## Ambition

Make **DnaOxIde / DNA OxIde** display the accepted OxIde GUI shell through the real app host path, using shared UI components and host bridge/Tauri command boundaries, while preserving native-service-missing or pending-hardening states for OxVba features that do not yet have OxIde-side adapter evidence.

This workset is the first live host UI proof. It must remain clear whether the proof is a Tauri/WebView runtime, a static hosted build, or a Rust-rendered host shell; do not claim browser/runtime/accessibility coverage that is not actually driven.

## Dependencies

- W341 — DnaOxIde Tauri app scaffold.
- W342 — shared IDE UI component layer.
- W343 — OxIde host bridge facade.
- W344 — DnaOxIde Tauri command boundary stubs.
- [`docs/HANDOFF_W344_DNAOXIDE_COMMAND_BOUNDARY.md`](../HANDOFF_W344_DNAOXIDE_COMMAND_BOUNDARY.md).
- [`docs/DNAOXIDE_HOST_UI_PROOF_MODE.md`](../DNAOXIDE_HOST_UI_PROOF_MODE.md).
- [`docs/HANDOFF_DNAOXIDE_OXVBA_FEEDBACK_ALIGNMENT.md`](../HANDOFF_DNAOXIDE_OXVBA_FEEDBACK_ALIGNMENT.md).

## Design

The live host proof should mount the shared shell and show at least:

- DNA OxIde branding,
- ThinSliceHello project identity,
- `Module1.bas`,
- editable source pane state or accepted editor projection,
- diagnostics panel,
- lifecycle/save/reload state,
- command palette availability,
- focus/accessibility labels already proven by W280/W300,
- runtime/Immediate/debug native-service-missing, available-subset, or OxVba-fixture-evidenced states,
- COM unavailable/native-service-missing, subset-backed, or capability-profile fixture-evidenced status.

The proof may start with a deterministic fixture project, then move to open-from-disk once W344 commands are ready. Where current OxVba direct APIs are wired, the proof may show available-subset or ThinSliceHello fixture-evidenced data; those panes must be labeled as subset/fixture-backed until OxIde adapter tests prove consumption and full host UX/source-span/event/native-boundary claims are ready. Any disk writes use temp project copies.

## Beads

### W345-B00 — Host UI proof mode decision

Goal:
  Decide the first executable proof mode and its exact claim boundary.

Design:
  - Choose between Tauri dev build, static app build, Rust render, or hybrid smoke depending on available toolchain.
  - Record what is and is not driven.

Tests:
  - Documentation grep for proof mode and no-claim tokens.

Evidence:
  - [`docs/DNAOXIDE_HOST_UI_PROOF_MODE.md`](../DNAOXIDE_HOST_UI_PROOF_MODE.md).
  - `target/w345-b00-host-ui-proof-mode.txt`.

Closure:
  - [ ] Proof mode is explicit.
  - [ ] Runtime/browser/accessibility claims are bounded.
  - [ ] Test command is known.

### W345-B01 — Mount shared shell in DnaOxIde

Goal:
  Make the DnaOxIde frontend mount the shared UI shell.

Design:
  - Use shared components from W342.
  - Source state from host bridge/client fixtures or W344 commands.
  - Keep app-specific glue minimal.

Tests:
  - Frontend/build/render test depending on chosen proof mode.
  - Grep for ThinSliceHello, Module1.bas, DNA OxIde in rendered output.

Evidence:
  - `target/w345-host-shell-render.txt`.

Closure:
  - [ ] Shared shell mounts in DnaOxIde path.
  - [ ] Accepted project/module state is visible.
  - [ ] App-specific code remains thin.

### W345-B02 — Host lifecycle proof

Goal:
  Show the host path can surface open/save/reload/session state using proven filesystem/session behavior.

Design:
  - Use test-owned temp project copies.
  - Exercise host bridge/Tauri command path if available.
  - Show dirty/clean and saved/reloaded state.

Tests:
  - Host lifecycle smoke.
  - Checked-in fixture mutation guard.

Evidence:
  - `target/w345-host-lifecycle-proof.txt`.

Closure:
  - [ ] Lifecycle state appears in host UI proof.
  - [ ] Disk writes are test-owned.
  - [ ] Fixtures are unchanged.

### W345-B03 — Unavailable, subset-backed, and fixture-evidenced runtime/debug/COM proof

Goal:
  Confirm the live host UI keeps pending OxVba/OxIde-adoption gaps visibly unavailable while permitting explicitly labeled available-subset or OxVba-fixture-evidenced adapter panes.

Design:
  - Render runtime native-service-missing, subset-backed, or fixture-evidenced state.
  - Render Immediate native-service-missing, subset-backed, or fixture-evidenced state.
  - Render debug native-service-missing, subset-backed, or fixture-evidenced state.
  - Render COM discovery/capability-profile subset/fixture/unavailable state and COM runtime unavailable as appropriate.

Tests:
  - Host render grep for disabled reasons, subset/fixture labels, and no-claim flags.
  - Anti-fake-data scan.

Evidence:
  - `target/w345-unavailable-service-proof.txt`.

Closure:
  - [ ] Runtime/Immediate/debug disabled, subset-backed, or fixture-evidenced states are visible.
  - [ ] COM runtime is not claimed.
  - [ ] No fake debug/Immediate data appears.

### W345-B04 — W345 acceptance

Goal:
  Accept the first live host UI proof as a reviewable DnaOxIde milestone.

Design:
  - Update GUI fixtures/lab documentation.
  - Link W346 interaction/e2e harness work.

Tests:
  - Host proof tests.
  - Workspace tests if code changed.
  - Anti-overclaim scan.

Evidence:
  - `target/w345-acceptance.txt`.
  - [`docs/HANDOFF_W345_DNAOXIDE_HOST_UI_PROOF.md`](../HANDOFF_W345_DNAOXIDE_HOST_UI_PROOF.md).

Closure:
  - [ ] DnaOxIde host UI proof is reviewable.
  - [ ] Claim boundaries are documented.
  - [ ] Interaction harness work is next.

## Out-of-scope

- Full click/key automation; W346 owns it.
- Real OxVba compile/runtime/debug/Immediate/COM behavior beyond explicitly tested adapter evidence.
- Full DOM accessibility audit.
- Installer packaging.
- DnaOneCalc real host mount.
