# OxIde GUI Workspace Layout

Status: `active_layout_plan`
Date: 2026-05-07

## 1. Purpose

This note defines how the OxIde repo should be prepared for the Rust/WASM-capable GUI implementation without letting the parked FrankenTui code shape the new architecture by accident.

It is a layout plan, not a code move. W200 records the intended shape so later implementation beads can make small, reviewable workspace changes.

## 2. Layout Principles

1. Start GUI implementation cleanly.
2. Treat current TUI code as parked prototype/evidence, not as GUI substrate.
3. Avoid deleting files during the layout transition.
4. Prefer small crate additions over a large one-shot move.
5. Keep product behavior out of host wrapper crates.
6. Consume authoritative DNA Calc cross-repo types where layering permits.
7. Use handoffs for sibling-repo changes rather than local compatibility sprawl.

## 3. Target Workspace Shape

The target shape remains:

```text
crates/
  oxide-domain/
  oxide-core/
  oxide-editor-core/
  oxide-oxvba/
  oxide-bridge/
  oxide-ui-leptos/
  oxide-guilab/
  oxide-host-browser/
  oxide-host-tauri/
  oxide-tui-frankentui/
```

Meaning:

- `oxide-domain` owns pure product vocabulary and view-model concepts that are genuinely OxIde-owned.
- `oxide-core` owns state transitions, command registry, and orchestration.
- `oxide-editor-core` owns rendering-independent text editing behavior.
- `oxide-oxvba` owns direct OxVba integration.
- `oxide-bridge` owns serialization boundaries only where needed.
- `oxide-ui-leptos` owns GUI components and design system.
- `oxide-guilab` owns browser scenario review surfaces.
- `oxide-host-browser` and `oxide-host-tauri` own startup/packaging.
- `oxide-tui-frankentui` owns parked TUI code once code movement is explicitly performed.

## 4. Current Root Handling

The current root crate remains the TUI implementation until a later layout bead moves or isolates it.

Because repo rules forbid deletion without explicit exact commands, future code movement must be planned carefully. A move from `src/` into `crates/oxide-tui-frankentui/` is logically correct, but it removes the old paths and therefore must be handled as an explicit, reviewed file-move operation.

Until that move happens:

1. the root crate is considered parked TUI implementation,
2. new GUI crates should be added beside it, not inside `src/shell`,
3. GUI code should not import `src/shell/*` as foundation,
4. any reusable behavior from current code should be rewritten into new crates unless it is clearly pure and not terminal-shaped.

## 5. Staging Order

Recommended layout rollout:

### Stage A — workspace shell

- Convert `Cargo.toml` to an explicit workspace root while keeping the current root package buildable.
- Add empty/new GUI crates only when the first implementation bead needs them.
- Keep WTD tests opt-in.

### Stage B — first GUI crates

Add only the crates needed for W210:

```text
crates/oxide-domain
crates/oxide-core
crates/oxide-editor-core
crates/oxide-oxvba
crates/oxide-ui-leptos
crates/oxide-guilab
crates/oxide-host-browser
```

Do not add `oxide-host-tauri` until desktop host work begins.

### Stage C — TUI isolation

Move the TUI implementation into `crates/oxide-tui-frankentui` under explicit reviewed commands. Keep any old binary names or launcher compatibility only if still useful.

### Stage D — desktop/native host

Add `oxide-host-tauri` and native capability services when W240/W260 requires them.

## 6. Cargo Policy

- Keep root `cargo test` meaningful throughout the transition.
- Do not let GUI dependencies make parked TUI builds slower or less reliable.
- Gate WTD and host-specific tests behind explicit features or scripts.
- Prefer small crates with clear seams over a single GUI monolith, but do not split so early that development becomes ceremony.

## 7. Done For W200

W200 is complete for layout preparation when:

1. this layout plan exists,
2. `ARCHITECTURE.md` points to it,
3. W200 records that code movement is deferred to a later explicit bead,
4. no code has been moved or deleted as part of W200-B03.
