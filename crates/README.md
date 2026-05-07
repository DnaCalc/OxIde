# OxIde GUI Crates

Status: `initial_w210_workspace_shell`

These crates are the greenfield Rust/WASM-capable GUI implementation lane.

The current root crate remains the parked FrankenTui implementation until a later explicit move isolates it under `crates/oxide-tui-frankentui`.

Initial W210 crates:

- `oxide-domain` — host-independent OxIde vocabulary.
- `oxide-core` — GUI-neutral state and orchestration.
- `oxide-editor-core` — rendering-independent source snapshot and edit behavior.
- `oxide-oxvba` — adapter boundary over authoritative OxVba APIs.
- `oxide-ui-leptos` — shared IDE UI component boundary for DnaOxIde, DnaOneCalc, and GUI-lab review surfaces.
- `oxide-host-bridge` — host-neutral service facade for DnaOxIde, DnaOneCalc, browser review fixtures, and GUI-lab consumers.
- `oxide-guilab` — deterministic GUI scenario lab boundary.

Do not import parked TUI `src/shell/*` code into these crates as implementation substrate. Rewrite behavior deliberately when it is needed.
