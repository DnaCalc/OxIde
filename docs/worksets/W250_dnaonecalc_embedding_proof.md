# Workset W250 — DnaOneCalc Embedded IDE And Runtime Proof

## Ambition

DnaOneCalc can consume an OxIde-owned IDE surface/contract or OxIde-authored artifact without owning OxIde semantics, without making OxIde a DnaOneCalc submodule by accident, and without duplicating OxVba truth.

The W250 proof is OxIde-side first: produce a deterministic embedding contract and GUI-lab rendering that a DnaOneCalc repo-scoped run can consume later.

## Dependencies

- W240 — capability-aware run/output path.
- [`PRODUCT_DIRECTION.md`](../../PRODUCT_DIRECTION.md) §3, §4, and §13.
- [`ARCHITECTURE.md`](../../ARCHITECTURE.md) §5, §9, and §10.
- [`docs/DNA_CALC_HOST_INTEGRATION.md`](../DNA_CALC_HOST_INTEGRATION.md).
- DnaOneCalc charter and implementation architecture as read-only sibling-repo inputs.
- Clean dependency-direction decision before code.

## Design

### Dependency direction

The first W250 implementation lane is:

```text
OxIde
  owns an embedding contract / artifact packet
  owns IDE state, source projection, lifecycle, run capability, and output transcript
  publishes deterministic Rust/serde shapes and GUI-lab evidence

DnaOneCalc
  remains a read-only sibling repo during this OxIde-scoped run
  may later consume the OxIde contract from its own repo-scoped work
  owns when/where the embedded IDE surface appears in its product shell

OxVba
  remains semantic/project/runtime authority
```

This deliberately avoids:

1. editing the DnaOneCalc repo from W250,
2. duplicating OxIde lifecycle/run/session types inside DnaOneCalc,
3. routing OxIde semantics through LSP,
4. making DnaOneCalc own OxVba project/runtime truth,
5. treating simulated run evidence as native execution.

### Contract shape

W250 should add an OxIde-owned bridge/contract layer only for the host boundary. The packet may summarize IDE state for serialization/review, but it should consume existing OxIde core state rather than creating a second lifecycle/run/session model.

Minimum packet contents:

1. host/consumer descriptor for `DnaOneCalc`,
2. embedded surface descriptors for `project-spine`, `source-editor`, `diagnostics`, `document-lifecycle`, `run-output`, and `capability-footer`,
3. source/document identity for the thin-slice project,
4. browser-safe lifecycle and session capability summary,
5. browser-disabled run capability and transcript summary,
6. explicit ownership strings showing `DnaOneCalc` owns product shell/policy, `OxIde` owns IDE experience, and `OxVba` owns semantics/runtime truth,
7. explicit limitations and sibling-repo handoff requirements.

### GUI-lab proof

W250 should add a deterministic scenario:

```text
gui-dnaonecalc-embedding-contract
```

The scenario should render:

1. `DnaOneCalc` host identity,
2. `ThinSliceHello` and `Module1.bas`,
3. embedded surface slots,
4. OxIde/OxVba/DnaOneCalc ownership boundaries,
5. browser-safe COM-unavailable capability text,
6. browser run disabled state with native execution unavailable reason,
7. statement that no DnaOneCalc repo files were modified.

### Handoff posture

W250 should close by producing a DnaOneCalc handoff note that says exactly what a DnaOneCalc repo-scoped run can consume next and what remains blocked by sibling-repo coordination.

## Beads

### W250-B00 — Expand DnaOneCalc embedding proof workset

Goal:
  Make W250 executable by replacing the scaffold with concrete vertical beads and an explicit dependency-direction decision.

Design:
  - Record OxIde-side-first contract/artifact proof as the first lane.
  - Preserve DnaOneCalc read-only boundary for this repo-scoped run.
  - Name the planned lab scenario and contract contents.

Tests:
  - Documentation review against product/architecture/DNA Calc host integration docs and DnaOneCalc read-only inputs.

Evidence:
  - Expanded `docs/worksets/W250_dnaonecalc_embedding_proof.md`.

Closure:
  - [ ] W250 has concrete beads.
  - [ ] Dependency direction is explicit.
  - [ ] Sibling-repo write boundary is explicit.

### W250-B01 — OxIde bridge contract for embedded-host packets

Goal:
  Add a pure, serializable OxIde-owned contract packet that can carry IDE embedding proof state to a DNA Calc host boundary.

Design:
  - Add `oxide-bridge` to the nested GUI workspace.
  - Depend on `oxide-core` for existing lifecycle/run/session state instead of duplicating those concepts.
  - Model only OxIde-owned host-boundary vocabulary: consumer identity, surface slots, ownership boundaries, limitations, and a composed embedding packet.
  - Provide a deterministic DnaOneCalc thin-slice packet builder using W240 browser-disabled run evidence.

Tests:
  - Contract packet round-trips through JSON.
  - DnaOneCalc packet includes required surface slots.
  - Packet preserves browser-disabled run capability and transcript status.
  - Ownership boundaries name DnaOneCalc, OxIde, and OxVba distinctly.

Evidence:
  - `cargo test --manifest-path crates/Cargo.toml -p oxide-bridge`.

Closure:
  - [ ] `oxide-bridge` exists in the nested workspace.
  - [ ] Packet uses existing OxIde core state where relevant.
  - [ ] No sibling-repo types are copied.

### W250-B02 — GUI-lab DnaOneCalc embedding contract scenario

Goal:
  Render the embedding contract as deterministic GUI-lab evidence for review and future DnaOneCalc consumption.

Design:
  - Add stable scenario ID `gui-dnaonecalc-embedding-contract`.
  - Reuse the thin-slice project-open spine.
  - Render contract surfaces, ownership boundaries, run disabled state, capability text, and no-sibling-write note.
  - Preserve W210-W240 scenarios as regressions.

Tests:
  - Scenario registry finds the new scenario.
  - Rendered output includes DnaOneCalc host identity, embedded surface slots, ThinSliceHello, Module1.bas, browser-disabled run evidence, COM-unavailable capability text, and no sibling repo writes.

Evidence:
  - `cargo test --manifest-path crates/Cargo.toml -p oxide-guilab`.
  - `cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-dnaonecalc-embedding-contract`.

Closure:
  - [ ] Scenario is registered.
  - [ ] Render proves the host boundary without claiming DnaOneCalc execution.
  - [ ] Existing scenario IDs remain intact.

### W250-B03 — DnaOneCalc handoff note and integration checklist

Goal:
  Produce a sibling-repo handoff that tells a future DnaOneCalc repo-scoped run how to consume the OxIde contract without duplicating semantics.

Design:
  - Add a handoff note focused on DnaOneCalc-side consumption steps.
  - Reference the `oxide-bridge` packet and GUI-lab scenario.
  - State expected DnaOneCalc app shell responsibilities and OxIde/OxVba ownership boundaries.
  - Include a checklist for a future paired smoke, but do not edit DnaOneCalc.

Tests:
  - Documentation review.
  - Full nested workspace tests remain green.

Evidence:
  - New handoff note.
  - `cargo test --manifest-path crates/Cargo.toml --workspace`.

Closure:
  - [ ] DnaOneCalc-side next actions are specific.
  - [ ] No sibling-repo files are modified.
  - [ ] Known limitations are explicit.

### W250-B04 — W250 acceptance and W260 handoff

Goal:
  Accept W250 with regression evidence and prepare the Windows COM capability proof workset.

Design:
  - Update GUI fixture/lab docs with the W250 scenario and expected tokens.
  - Update this workset with acceptance evidence.
  - Add W260 handoff notes for native Windows/COM capability proof.

Tests:
  - `cargo test --manifest-path crates/Cargo.toml --workspace`.
  - Render W210-W250 GUI-lab scenarios.

Evidence:
  - Test output and rendered scenario token checks.
  - W260 handoff note.

Closure:
  - [ ] W250 accepted.
  - [ ] W260 prerequisites documented.
  - [ ] Browser/native/COM limitations remain explicit.

## Acceptance Evidence

W250 was accepted with:

```powershell
cargo test --manifest-path crates/Cargo.toml --workspace
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-thin-slice-loaded
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-thin-slice-edited-diagnostics
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-thin-slice-lifecycle
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-run-output-browser-disabled
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-run-output-simulated-supported
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-dnaonecalc-embedding-contract
```

The accepted W250 lab output contains `gui-dnaonecalc-embedding-contract`,
`DnaOneCalc`, `ThinSliceHello`, `Module1.bas`, OxIde-owned embedded surface
slots, distinct `DnaOneCalc` / `OxIde` / `OxVba` ownership boundaries,
`browser-unsupported` run state, `data-status="disabled"`,
`data-native-execution="false"`, `data-com-runtime="false"`,
`native execution provider unavailable`, `did not modify DnaOneCalc files`,
and browser-safe `COM unavailable` capability text.

## Out-of-scope

- Editing sibling DnaOneCalc repo files from this OxIde-scoped run.
- General third-party host embedding.
- Making DnaOneCalc own OxIde semantics.
- Making DnaOneCalc own OxVba project/runtime truth.
- Real native execution.
- Windows COM capability proof; this belongs to W260.
- Debugger and Immediate Window GUI surfaces; these belong to W270.
