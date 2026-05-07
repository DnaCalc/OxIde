# Workset W347 — Compile Options And Reference UI Placeholders

## Ambition

Build the **DNA OxIde** compile/options/reference panels and unavailable/loading states now, using available-subset OxVba project/reference/build surfaces where they are already consumable, so remaining OxVba DTOs can be connected quickly when they arrive.

This workset delivers user-visible surfaces for full-scope features without pretending the remaining OxVba compile/options/run-target/stable-taxonomy/COM-runtime services are complete.

## Dependencies

- W342 — shared IDE UI component layer.
- W343 — OxIde host bridge facade.
- W344 — DnaOxIde Tauri command boundary stubs.
- [`docs/HANDOFF_DNAOXIDE_OXVBA_REQUIREMENTS.md`](../HANDOFF_DNAOXIDE_OXVBA_REQUIREMENTS.md).
- [`docs/HANDOFF_DNAOXIDE_OXVBA_FEEDBACK_ALIGNMENT.md`](../HANDOFF_DNAOXIDE_OXVBA_FEEDBACK_ALIGNMENT.md).

## Design

Panels to add as shared UI components:

- project properties,
- compile options,
- build/check status,
- run target/entrypoint selector placeholder,
- references roster,
- COM candidate search placeholder,
- reference repair/apply preview placeholder,
- unavailable/native-service-missing banners.

Inputs should be host bridge placeholder packets, existing OxIde capability states, or available-subset OxVba adapter packets. Do not define final authoritative OxVba DTOs locally. If shape is needed before OxVba lands it, mark it as an OxIde placeholder wrapper and link the requirements/feedback notes.

Confirmed available-subset inputs to prefer where dependency wiring is ready:

- `inspect_workspace_target` / host project surface for project/module/reference roster,
- `ComSelectionService` for COM candidate and active-selection subset,
- `EmbeddedBuildRunHost` for typed build/check subset,
- `HostWorkspaceSession` snapshots for `DiskOnly` / `WorkspaceOverlay` source policy.

Still pending from OxVba:

- unified project properties / compile options DTOs,
- run target DTOs,
- stable request/session IDs,
- command availability taxonomy,
- COM capability profile, bitness/apartment/native boundary status,
- COM runtime invocation claim evidence.

## Beads

### W347-B00 — Placeholder and available-subset data contract

Goal:
  Define placeholder and available-subset panel inputs without duplicating final OxVba DTO ownership.

Design:
  - Separate `PendingOxVba`/`Unavailable` state from subset-backed data state.
  - Include disabled reasons and source links to requirements/feedback.
  - Keep future real DTO slots clear.

Tests:
  - Contract tests for placeholder/unavailable states.
  - Contract tests for subset-backed state labels where implemented.
  - Grep for ownership disclaimers.

Evidence:
  - Placeholder contract docs/tests.

Closure:
  - [ ] Placeholder and subset-backed inputs exist or are documented.
  - [ ] OxVba ownership is explicit.
  - [ ] No final DTO duplication is introduced.

### W347-B01 — Project properties and compile options panels

Goal:
  Render project properties and compile options surfaces with pending/unavailable state.

Design:
  - Show workspace/project identity from proven state.
  - Show compile options as pending OxVba DTOs where unavailable.
  - Show build/check disabled reason or available-subset build result where wired.

Tests:
  - Component render tests.
  - GUI-lab scenario for pending compile/options panel.

Evidence:
  - `target/w347-compile-options-render.txt`.

Closure:
  - [ ] Project properties panel renders.
  - [ ] Compile options panel renders.
  - [ ] Build/check remains disabled or explicitly subset-backed pending full OxVba hardening.

### W347-B02 — Reference and COM placeholder panels

Goal:
  Render references and COM candidate/repair surfaces with honest unavailable states.

Design:
  - Show active reference facts when available from current projections or OxVba adapter subset.
  - Show COM discovery subset/unavailable/native-service-missing state.
  - Separate discovery availability from runtime invocation availability.

Tests:
  - Component render tests.
  - GUI-lab scenario for reference/COM placeholder panel.
  - Anti-overclaim scan for COM runtime.

Evidence:
  - `target/w347-reference-com-render.txt`.

Closure:
  - [ ] Reference panel renders.
  - [ ] COM discovery unavailable or subset-backed state is visible.
  - [ ] COM runtime is not claimed.

### W347-B03 — Host commands for placeholder panels

Goal:
  Wire placeholder panels to host bridge/Tauri stubs.

Design:
  - Compile/build and reference/COM commands return subset-backed responses where wired and pending/unavailable responses where hardening is missing.
  - UI shows disabled reasons consistently.

Tests:
  - Command stub tests.
  - Interaction test if W346 harness is available.

Evidence:
  - `target/w347-placeholder-command-tests.txt`.

Closure:
  - [ ] Placeholder panels use host bridge commands.
  - [ ] Disabled reasons are stable.
  - [ ] No fake compile/reference data is returned.
  - [ ] Subset-backed data is labeled as such.

### W347-B04 — W347 acceptance

Goal:
  Accept compile/options/reference placeholders as ready for real OxVba DTO connection.

Design:
  - Update requirements cross-links if any missing OxVba needs are discovered.
  - Link future OxVba integration workset.

Tests:
  - Workspace tests.
  - Component/render tests.
  - Anti-overclaim scan.

Evidence:
  - W347 acceptance outputs.

Closure:
  - [ ] Placeholder panels are reviewable.
  - [ ] OxVba integration points are clear.
  - [ ] No unimplemented capability is claimed.

## Out-of-scope

- Full compile/build execution claims beyond available-subset adapter evidence.
- Real compile option mutation.
- Full COM discovery/repair claims beyond available-subset adapter evidence, or any COM runtime invocation claim.
- OxVba repo changes.
