# Workset W348 — DnaOneCalc Shared UI Reuse Path

## Ambition

Prove, inside the OxIde repo, that the shared OxIde UI and host bridge are not locked to **DnaOxIde** and can be consumed by a future **DnaOneCalc** host mount.

This workset preserves DnaOneCalc reuse while avoiding sibling repo writes.

## Dependencies

- W310 — DnaOneCalc web shell hosting contract.
- W342 — shared IDE UI component layer.
- W343 — OxIde host bridge facade.
- W345 — DnaOxIde live host UI proof.
- W346 — DnaOxIde interaction and e2e harness.
- W347 — compile options and reference UI placeholders.
- [`docs/HANDOFF_W347_COMPILE_REFERENCE_PLACEHOLDERS.md`](../HANDOFF_W347_COMPILE_REFERENCE_PLACEHOLDERS.md).
- [`docs/DNAONECALC_SHARED_UI_REUSE_PROOF.md`](../DNAONECALC_SHARED_UI_REUSE_PROOF.md).
- [`docs/HANDOFF_DNAOXIDE_OXVBA_FEEDBACK_ALIGNMENT.md`](../HANDOFF_DNAOXIDE_OXVBA_FEEDBACK_ALIGNMENT.md).

## Design

W348 should create an OxIde-owned reuse proof that behaves like a DnaOneCalc consumer without modifying `C:/Work/DnaCalc/DnaOneCalc`.

Possible proof forms:

- a host profile fixture named `dnaonecalc-consumer`,
- a shared UI render using `DnaOneCalcWebShellHostPacket`,
- a component story/scenario that mounts the same shell under a DnaOneCalc product frame,
- a DOM-smoke check over the DnaOneCalc host contract packet.

The proof must show:

- shared UI components do not depend on DnaOxIde app code,
- host bridge interfaces can be implemented by a non-Tauri consumer,
- the same interface can carry unavailable, available-subset OxVba adapter, OxVba-fixture-evidenced adapter, or full future OxVba states,
- DnaOneCalc owns product shell/placement/persistence policy,
- OxIde owns IDE surface,
- OxVba owns language/project/runtime truth,
- real DnaOneCalc repo mount remains pending explicit authorization.

## Beads

### W348-B00 — Reuse proof design

Goal:
  Choose the OxIde-only DnaOneCalc reuse proof form.

Design:
  - Decide fixture/scenario/story shape.
  - Map reused components and host bridge services.
  - Keep sibling repo untouched.

Tests:
  - Documentation grep for DnaOneCalc ownership and no sibling writes.

Evidence:
  - Reuse proof design note.

Closure:
  - [ ] Proof form is selected.
  - [ ] Sibling-write boundary is explicit.
  - [ ] Reused components are listed.

### W348-B01 — DnaOneCalc consumer fixture/profile

Goal:
  Add an OxIde-owned host fixture/profile representing DnaOneCalc consumption.

Design:
  - Use existing `DnaOneCalcWebShellHostPacket` where possible.
  - Include DnaOneCalc product-shell ownership labels.
  - Use the same shared UI component inputs as DnaOxIde.

Tests:
  - Fixture/profile unit tests.
  - Packet serialization tests if applicable.

Evidence:
  - `target/w348-dnaonecalc-profile-tests.txt`.

Closure:
  - [ ] DnaOneCalc consumer profile exists.
  - [ ] Shared UI inputs are reused.
  - [ ] Ownership labels are correct.

### W348-B02 — Shared UI reuse render

Goal:
  Render the shared IDE shell in a DnaOneCalc-like host frame.

Design:
  - Mount shared components, not DnaOxIde app components.
  - Show ThinSliceHello and Module1.bas.
  - Preserve browser/native/runtime/COM disabled, subset-backed, or fixture-evidenced states as appropriate.

Tests:
  - GUI-lab or webshell render for DnaOneCalc reuse.
  - Parsed DOM smoke if static HTML is produced.

Evidence:
  - `target/w348-dnaonecalc-reuse-render.txt`.

Closure:
  - [ ] Shared UI renders under DnaOneCalc-like frame.
  - [ ] DnaOxIde dependency is absent.
  - [ ] Disabled, subset-backed, and fixture-evidenced states remain honest.

### W348-B03 — Reuse contract handoff refresh

Goal:
  Update DnaOneCalc handoff docs with the shared UI/host bridge consumption path.

Design:
  - Point to shared UI crate.
  - Point to host bridge facade.
  - Preserve explicit authorization requirement for sibling writes.

Tests:
  - Documentation grep for crate names, DnaOneCalc, and authorization gate.

Evidence:
  - Updated handoff docs.

Closure:
  - [ ] Handoff reflects shared UI route.
  - [ ] Host bridge route is documented.
  - [ ] Real DnaOneCalc mount remains unclaimed.

### W348-B04 — W348 acceptance

Goal:
  Accept the DnaOneCalc reuse path as proven inside OxIde.

Design:
  - Update GUI fixture/lab docs.
  - Link external DnaOneCalc authorization step if needed.

Tests:
  - Workspace tests.
  - Reuse render tests.
  - Anti-overclaim scan.

Evidence:
  - W348 acceptance outputs.

Closure:
  - [ ] Reuse path is reviewable.
  - [ ] No sibling repo writes occurred.
  - [ ] Real DnaOneCalc host mount remains pending authorization.

## Out-of-scope

- Writing to `C:/Work/DnaCalc/DnaOneCalc`.
- Real DnaOneCalc product mount.
- DnaOneCalc persistence policy implementation.
- Real OxVba runtime/debug/Immediate/COM behavior.
