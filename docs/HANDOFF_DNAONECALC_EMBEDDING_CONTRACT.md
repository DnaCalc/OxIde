# Handoff — DnaOneCalc Consuming The OxIde Embedding Contract

Status: `sibling_repo_handoff`
Date: 2026-05-07

## Source Workset

OxIde W250 produced an OxIde-side embedding proof for DnaOneCalc without editing the DnaOneCalc repo.

Evidence in OxIde:

```powershell
cargo test --manifest-path crates/Cargo.toml --workspace
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-dnaonecalc-embedding-contract
```

Primary OxIde artifacts:

1. `crates/oxide-bridge` — serializable host-boundary packet crate.
2. `EmbeddedIdePacket::dnaonecalc_thin_slice_browser_disabled(...)` — deterministic DnaOneCalc thin-slice contract builder.
3. `gui-dnaonecalc-embedding-contract` — GUI-lab scenario rendering the contract over `examples/thin-slice/ThinSliceHello.basproj`.

## Ownership Boundary

DnaOneCalc should consume the contract without absorbing OxIde identity:

```text
DnaOneCalc
  owns product shell, host policy, persistence policy, and where embedded OxIde appears

OxIde
  owns IDE/editor/project surface, lifecycle UX, run/output presentation, and embedding contract

OxVba
  owns VBA project, language-service, semantic, build/run, Immediate, debug, and runtime truth
```

Do not duplicate OxIde lifecycle/run/session models in DnaOneCalc. Consume `oxide-bridge`/OxIde artifacts or coordinate a shared crate if direct dependency shape is wrong.

## Suggested DnaOneCalc Repo-Scoped Next Steps

1. Decide dependency intake:
   - path dependency to OxIde's `crates/oxide-bridge` for local proof, or
   - coordinated package/shared-crate extraction if the boundary needs to become stable.
2. Add a DnaOneCalc test fixture that deserializes an `EmbeddedIdePacket` JSON sample or calls the builder from a local path dependency.
3. Map `EmbeddedIdeSurfaceKind` slots to DnaOneCalc shell locations without renaming their ownership:
   - `project-spine`,
   - `source-editor`,
   - `diagnostics`,
   - `document-lifecycle`,
   - `run-output`,
   - `capability-footer`.
4. Keep DnaOneCalc mode/shell ownership clear:
   - `Explore`, `Inspect`, and `Workbench` decide where the embedded IDE is visible,
   - OxIde still owns the IDE surface behavior once mounted.
5. Preserve capability honesty:
   - browser-safe execution stays disabled,
   - native execution is not implied,
   - COM is unavailable in pure browser/WASM.
6. Add paired smoke only from a DnaOneCalc-scoped run.

## Suggested First Paired Smoke

A minimal DnaOneCalc-side smoke should prove:

```text
load OxIde EmbeddedIdePacket
  -> identify host as DnaOneCalc
  -> list expected OxIde surface slots
  -> show ThinSliceHello / Module1.bas identity
  -> show browser-disabled run reason
  -> show no native execution and no COM runtime
  -> retain DnaOneCalc shell ownership statement
```

This smoke should not execute VBA, call COM, or mutate OxIde fixtures.

## Known Limitations

1. W250 did not edit DnaOneCalc files.
2. W250 did not prove a real Leptos component mount inside DnaOneCalc.
3. W250 did not prove native OxVba execution.
4. W250 did not prove Windows COM capability.
5. W250 did not define package publishing/versioning for `oxide-bridge`.
6. `EmbeddedIdePacket` is a first proof packet; stabilize/version it only when an external boundary needs that commitment.

## Required Coordination Before Production Use

1. Choose whether `oxide-bridge` remains an OxIde crate, moves to a shared DNA Calc crate, or is consumed as an unpublished path dependency.
2. Decide who owns packet versioning once DnaOneCalc uses it outside a local proof.
3. Coordinate any OxVba public runtime/build DTOs rather than copying them into OxIde or DnaOneCalc.
4. Add DnaOneCalc repo tests from DnaOneCalc's own bead/workset process.
