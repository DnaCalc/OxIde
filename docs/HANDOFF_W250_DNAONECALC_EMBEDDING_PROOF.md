# Handoff — W250 DnaOneCalc Embedded IDE And Runtime Proof

Status: `handoff_note`
Date: 2026-05-07

## W240 Baseline

W240 produced five deterministic GUI lab commands:

```powershell
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-thin-slice-loaded
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-thin-slice-edited-diagnostics
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-thin-slice-lifecycle
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-run-output-browser-disabled
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-run-output-simulated-supported
```

The run/output scenarios prove:

1. browser-safe run is disabled with an explicit native execution unavailable reason,
2. run request/output state is structured as capability, request, transcript, status, and event rows,
3. supported run-output UI shape exists via a clearly labeled simulated provider,
4. simulated output does not claim native execution or COM availability,
5. W210-W230 project/edit/diagnostic/lifecycle/session evidence remains available as regression scenarios.

## W250 Starting Point

W250 should prove DnaOneCalc can consume OxIde IDE surface/contracts or OxIde-authored artifacts without making DnaOneCalc own OxIde semantics.

Recommended first W250 steps:

1. expand `docs/worksets/W250_dnaonecalc_embedding_proof.md` into executable beads,
2. decide dependency direction before implementation,
3. define the smallest OxIde-side artifact/contract DnaOneCalc should consume,
4. keep any DnaOneCalc repo changes as handoff notes unless the user explicitly authorizes cross-repo edits,
5. keep OxVba as semantic/runtime owner for VBA project truth and execution semantics.

## Constraints

1. Do not duplicate OxIde lifecycle/run/session types into DnaOneCalc.
2. Do not make OxIde a DnaOneCalc submodule by accident.
3. Do not make DnaOneCalc own OxIde semantics or OxVba runtime truth.
4. Do not edit sibling repos from an OxIde-scoped agent run without explicit user instruction.
5. Any paired smoke requiring DnaOneCalc changes should be documented as a handoff first.

## Open Questions For W250 Expansion

1. Is the first proof an artifact/runtime packet, an embedded editor surface contract, or a shared component contract?
2. Should the first OxIde artifact be a deterministic `oxide-guilab` scenario manifest, a crate API surface, or generated static assets?
3. What is the minimum DnaOneCalc-side evidence needed: design-only handoff, local consumer fixture, or paired smoke in the DnaOneCalc repo?
