# Handoff — W280 Command, Keyboard, Accessibility, And Polish

Status: `next_workset_handoff`
Date: 2026-05-07

## Source Workset

W270 accepted the first runtime-surface GUI-lab proofs:

```powershell
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-run-timeline-simulated
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-immediate-browser-disabled
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-debug-browser-disabled
```

W280 should polish the GUI as an IDE without importing parked TUI architecture.

## Regression Baseline

Start W280 with the full W210-W270 lab set:

```powershell
cargo test --manifest-path crates/Cargo.toml --workspace
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-thin-slice-loaded
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-thin-slice-edited-diagnostics
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-thin-slice-lifecycle
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-run-output-browser-disabled
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-run-output-simulated-supported
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-dnaonecalc-embedding-contract
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-com-reference-browser-unavailable
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-com-reference-nonwindows-unavailable
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-com-reference-native-service-missing
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-run-timeline-simulated
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-immediate-browser-disabled
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-debug-browser-disabled
```

## Recommended W280 Beads

### W280-B00 — Expand Workset

Replace the scaffold with executable vertical beads and scenario names.

### W280-B01 — Command Registry

Add a pure command registry with stable IDs, labels, descriptions, default availability, and disabled reasons.

Initial commands should include existing proven behaviours:

1. open project,
2. save,
3. revert,
4. reload,
5. run,
6. stop/cancel if unavailable,
7. open Immediate,
8. open Debug,
9. show COM capability,
10. show command palette.

### W280-B02 — Keyboard Contexts

Add keybinding contexts and collision checks for browser-safe GUI state. Keep host-specific bindings configurable by future host layers.

### W280-B03 — Focus Graph

Model a deterministic focus graph for project tree, editor, diagnostics, run output, Immediate, debug, and command palette surfaces.

### W280-B04 — Accessibility Projection

Add semantic roles/labels/tokens for GUI-lab render output. This should prove disabled reasons, panel labels, and keyboard paths are visible without a mouse.

### W280-B05 — Acceptance

Run full workspace tests plus W210-W280 lab renders and grep for command/focus/accessibility tokens.

## Guardrails

1. Do not revive or reshape parked TUI code into the GUI substrate.
2. Do not claim real runtime/debug/Immediate where W270 only proves unavailable or simulated seams.
3. Do not claim COM-capable support while W260 native service remains missing.
4. Keep command availability capability-aware.
5. Keep DnaOneCalc as host/consumer, not owner of OxIde IDE state.
6. Keep OxVba as semantic/runtime authority.

## Suggested First Scenario IDs

```text
gui-command-palette-baseline
gui-keyboard-contexts-baseline
gui-focus-graph-no-mouse
gui-accessibility-disabled-reasons
```

The first W280 slice should be narrow: one command palette render with stable command IDs and availability reasons is enough to prove direction before adding keyboard/focus/a11y breadth.
