# OxIde GUI Crates

Status: `initial_w210_workspace_shell`

These crates are the greenfield Rust/WASM-capable GUI implementation lane.

The current root crate remains the parked FrankenTui implementation until a later explicit move isolates it under `crates/oxide-tui-frankentui`.

Initial W210 crates:

- `oxide-domain` — host-independent OxIde vocabulary.
- `oxide-core` — GUI-neutral state and orchestration.
- `oxide-oxvba` — adapter boundary over authoritative OxVba APIs.
- `oxide-guilab` — deterministic GUI scenario lab boundary.

Do not import parked TUI `src/shell/*` code into these crates as implementation substrate. Rewrite behavior deliberately when it is needed.
