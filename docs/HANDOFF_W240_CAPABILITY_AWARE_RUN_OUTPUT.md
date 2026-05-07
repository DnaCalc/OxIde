# Handoff — W240 Capability-Aware Run And Output

Status: `handoff_note`
Date: 2026-05-07

## W230 Baseline

W230 produced three deterministic GUI lab commands:

```powershell
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-thin-slice-loaded
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-thin-slice-edited-diagnostics
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-thin-slice-lifecycle
```

The lifecycle scenario proves:

1. `oxide-core` owns pure document lifecycle state for persisted/working source and dirty status,
2. browser-limited save/reload are disabled with explicit direct-filesystem reasons,
3. in-memory persistence is available as a deterministic proof seam and is labeled non-filesystem,
4. `GuiSessionSnapshot` can serialize/restore workspace, active module, working source, dirty state, and capability profile,
5. `oxide-guilab` renders lifecycle and session restore evidence without mutating checked-in fixtures.

## W240 Starting Point

W240 should add run/output behavior without weakening W210-W230 evidence.

Recommended first W240 steps:

1. expand `docs/worksets/W240_capability_aware_run_output.md` into executable beads,
2. add an OxIde-owned run capability model with browser-safe disabled reasons,
3. add a pure run request/event/output model before wiring to OxVba execution,
4. add a lab scenario for browser-unsupported run state, likely `gui-run-output-browser-disabled`,
5. add a supported run-output proof only when the provider is explicitly simulated or genuinely supported and labeled as such.

## Constraints

1. Do not claim native execution in browser-safe mode.
2. Do not claim COM availability before W260/native capability work.
3. Do not route run state through parked TUI code.
4. Do not duplicate OxVba runtime/build enums if authoritative shared types can be consumed.
5. Keep dirty/session state visible or intentionally scoped out when showing run behavior.

## Open Questions For W240 Expansion

1. Should W240 use the existing thin-slice source for the first run-disabled scenario, or introduce `run-output-demo` immediately?
2. Should the first supported run proof use a simulated provider in `oxide-core`, or wait for direct OxVba build/run integration in `oxide-oxvba`?
3. What structured output rows are required for later debugger/immediate surfaces: lifecycle events, diagnostics, stdout-like output, or activity timeline entries?
