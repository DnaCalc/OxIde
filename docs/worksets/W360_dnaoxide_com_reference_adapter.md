# Workset W360 — DnaOxIde COM And Reference Adapter

## Ambition

Connect the shared OxIde reference and COM UI to OxVba direct Rust DTOs/APIs for active references, COM candidate discovery, repair/replace/remove/reorder plans, capability profile, and runtime availability status.

W360 must distinguish browser/WASM, DnaOxIde desktop, and DnaOneCalc desktop host profiles. Browser/WASM may inspect project/reference facts where OxVba supports them, but Windows COM discovery/invocation is native-only. COM runtime invocation remains gated until direct native evidence exists.

## Dependencies

- W350 — live editable source app and temp project copy policy.
- W352 — Tauri/WebView native command spine for desktop evidence.
- W355 — compile/build adapter profiles baseline.
- W347 — reference/COM placeholder panels.
- OxVba `ComSelectionService`, `ComCapabilityProfile`, `ComRuntimeInvocationAvailability`, and reference edit/reorder DTOs.

## Design

W360 should replace placeholder-only reference/COM panels with OxVba-backed packets where available. It should preserve host-profile unavailable states and distinguish:

- active reference roster,
- broken/missing/ambiguous/resolved COM references,
- candidate search results,
- add/repair/replace/remove/reorder plans,
- browser/WASM reference visibility vs native COM discovery,
- native capability profile,
- runtime invocation availability as status only unless separately proven.

## Beads

### W360-B00 — COM/reference adapter contract lock

Goal:
  Lock OxVba reference/COM DTO ownership and DnaOxIde packet boundaries.

Design:
  - Map active roster, candidates, repair plans, reorder plans, capability profile, and runtime availability.
  - Document platform gates and no-runtime-invocation claim.

Tests:
  - Documentation grep for COM/reference DTO names, platform gates, and no-claim tokens.

Evidence:
  - `docs/DNAOXIDE_COM_REFERENCE_ADAPTER_CONTRACT.md`.
  - `target/w360-b00-com-reference-contract.txt`.

Closure:
  - [ ] OxVba-owned reference/COM DTOs are named.
  - [ ] Platform gates are explicit.
  - [ ] COM runtime invocation remains unclaimed.

### W360-B01 — Reference/COM command adapter

Goal:
  DnaOxIde commands return adapter-backed reference roster, COM candidates, plans, and capability status for temp project copies.

Design:
  - Wire OxVba `ComSelectionService` behind W344/W352 command names where the active host profile can support it.
  - Preserve unavailable/degraded status in browser/WASM, non-Windows, or missing native capability.
  - Keep edit plans previewable before apply.

Tests:
  - Command tests over temp project copies.
  - Platform/degraded capability tests.
  - Fixture mutation guard.

Evidence:
  - `target/w360-b01-command-adapter.txt`.

Closure:
  - [ ] Reference roster is adapter-backed.
  - [ ] COM candidate/capability packets are typed.
  - [ ] Degraded paths are tested.

### W360-B02 — Reference/COM UI adoption

Goal:
  DnaOxIde reference panels show adapter-backed reference/COM state and typed disabled reasons.

Design:
  - Replace placeholder rows with adapter packets where present.
  - Preserve empty/no-claim UI when unavailable.
  - Show preview/apply state only after command evidence exists.

Tests:
  - Panel render tests and anti-overclaim scan.

Evidence:
  - `target/w360-b02-ui-panels.txt`.

Closure:
  - [ ] Reference/COM UI is adapter-backed where possible.
  - [ ] COM runtime invocation is still false unless proven.
  - [ ] No fake candidate/repair rows are shown.

### W360-B03 — Edit-save-reference interaction proof

Goal:
  The live DnaOxIde flow can open a temp project, inspect references/COM status, and preview supported plans.

Design:
  - Extend W350/W355 interaction harness.
  - Assert active roster/capability/plan tokens.
  - Keep platform-specific disabled reasons visible.

Tests:
  - Live interaction check and command tests.

Evidence:
  - `target/w360-b03-reference-interaction.txt`.

Closure:
  - [ ] Reference/COM interaction is driven.
  - [ ] Platform gates are visible.
  - [ ] No COM runtime invocation is claimed.

### W360-B04 — W360 acceptance

Goal:
  Accept reference/COM adapter readiness for runtime/Immediate work.

Design:
  - Run W350/W355/W360 checks and update W365 handoff.

Tests:
  - Full reference/COM regression and no-claim scan.

Evidence:
  - `target/w360-acceptance.txt`.
  - `docs/HANDOFF_W360_COM_REFERENCE_ADAPTER.md`.

Closure:
  - [ ] Reference/COM adapter is accepted.
  - [ ] W365 runtime/Immediate work is unblocked.
  - [ ] COM runtime invocation remains gated.

## Out-of-scope

- COM runtime invocation unless separately evidenced.
- Runtime/Immediate/debug execution.
- DnaOneCalc implementation beyond shared packet/profile contracts.
