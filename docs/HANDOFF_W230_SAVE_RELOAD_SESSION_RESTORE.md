# Handoff — W230 Save Reload Session Restore

Status: `handoff_note`
Date: 2026-05-07

## W220 Baseline

W220 produced two deterministic GUI lab commands:

```powershell
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-thin-slice-loaded
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-thin-slice-edited-diagnostics
```

The edited diagnostics scenario proves:

1. `oxide-editor-core` can apply an in-memory edit to `Module1.bas`,
2. `oxide-oxvba` submits the edited source to `HostWorkspaceSession` without saving to disk,
3. OxVba returns a stable diagnostic: `use of undeclared variable: answer`,
4. `oxide-guilab` renders edited source, diagnostics, OxVba provenance, and browser-safe capability status.

## W230 Starting Point

W230 should add persistence/session behavior without weakening W210/W220 evidence.

Recommended first W230 steps:

1. add an OxIde-owned document state model for clean/dirty/persisted source text,
2. keep disk source and in-memory OxVba overlay distinct,
3. add lab scenario states for dirty edit, saved edit, reload from disk, and restored session,
4. decide whether session restore replays source snapshots into `HostWorkspaceSession` or reloads OxVba first then reapplies overlays,
5. keep `gui-thin-slice-loaded` and `gui-thin-slice-edited-diagnostics` as regressions.

## Constraints

1. Do not claim real file persistence until a test verifies disk content after save.
2. Do not mutate `examples/thin-slice/Module1.bas` as part of tests unless the test restores it safely without file deletion.
3. Do not route persistence state through parked TUI session code.
4. Do not duplicate OxVba project truth; OxVba remains authoritative for `.basproj` and module identity.
5. Keep browser-safe capability status visible when save is simulated or host-limited.

## Open Questions For W230 Expansion

1. Should W230 use a temporary copy of the thin-slice fixture for real save/reload tests, or add a dedicated persistence fixture?
2. Should session restore live first in `oxide-core` as pure state, with filesystem save delegated later?
3. What is the minimum accepted lab command for persisted state: one scenario with multiple regions or separate scenario IDs per state?
