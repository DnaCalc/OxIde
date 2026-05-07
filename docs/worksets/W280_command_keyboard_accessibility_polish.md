# Workset W280 — Command, Keyboard, Accessibility, And Polish

## Ambition

The GUI becomes usable as an IDE rather than a demo shell: commands are unified, keyboard paths are complete, focus is predictable, and accessibility/polish are explicitly tested.

W280 is still a greenfield GUI workset. It may reuse behavioural lessons from the parked TUI lane, but it must not pull parked TUI state, keymaps, widgets, or shell architecture into the GUI substrate.

## Dependencies

- W270 — run/debug/immediate GUI surfaces.
- Earlier GUI scenario lab coverage.
- [`docs/HANDOFF_W280_COMMAND_KEYBOARD_ACCESSIBILITY_POLISH.md`](../HANDOFF_W280_COMMAND_KEYBOARD_ACCESSIBILITY_POLISH.md).
- [`docs/GUI_FIXTURES_AND_LAB.md`](../GUI_FIXTURES_AND_LAB.md).

## Guardrails

1. OxIde owns IDE command/focus/accessibility state.
2. DnaOneCalc may host/consume embedded OxIde surfaces but does not own OxIde IDE state.
3. OxVba remains semantic/runtime/debug/Immediate authority.
4. W240 simulated run stays simulated-only.
5. W260 COM-capable paths remain disabled until a tested native service exists.
6. W270 Immediate/debug surfaces stay unavailable/future seams until authoritative runtime APIs exist.
7. GUI command/key/focus modelling must be pure and host-independent first.
8. Do not introduce a parallel runtime/debug model just to populate command availability.

## Scenario Plan

W280 should add deterministic GUI-lab scenarios in narrow vertical slices:

```text
gui-command-palette-baseline
gui-keyboard-contexts-baseline
gui-focus-graph-no-mouse
gui-accessibility-disabled-reasons
```

The first slice should render a command palette over existing proven behaviours. Later slices should add keyboard context collisions, deterministic focus traversal, and accessibility labels/roles.

## Beads

### W280-B00 — Expand command/keyboard/accessibility workset

Goal:
  Replace the scaffold with executable vertical beads and first scenario names.

Design:
  - Preserve W270 runtime/debug/Immediate capability guardrails.
  - Use GUI-native command/focus/accessibility projections, not parked TUI state.
  - Define B01-B05 around command registry, keyboard contexts, focus graph, accessibility labels, and acceptance.

Tests:
  - Documentation review against W280 handoff and GUI fixture baseline.

Evidence:
  - Expanded `docs/worksets/W280_command_keyboard_accessibility_polish.md`.

Closure:
  - [ ] W280 has concrete beads.
  - [ ] First scenario IDs are named.
  - [ ] TUI lineage remains parked.

### W280-B01 — Pure command registry

Goal:
  Add a pure command registry in `oxide-core` with stable command IDs, labels, descriptions, categories, availability, and disabled reasons.

Design:
  - Model commands for already proven GUI behaviours: open project, save, revert, reload, run, open Immediate, open Debug, show COM capability, and show command palette.
  - Keep availability capability-aware and host-independent.
  - Reuse existing run/Immediate/debug capability profiles instead of duplicating runtime truth.
  - Add a `gui-command-palette-baseline` lab scenario over the registry.

Tests:
  - Command IDs and labels are stable.
  - Browser-safe run/Immediate/debug commands have disabled reasons.
  - Simulated run availability remains labeled simulated.
  - GUI-lab registry finds `gui-command-palette-baseline`.
  - Rendered command palette includes command IDs and disabled reasons.

Evidence:
  - `cargo test --manifest-path crates/Cargo.toml -p oxide-core`.
  - `cargo test --manifest-path crates/Cargo.toml -p oxide-guilab`.
  - `cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-command-palette-baseline`.

Closure:
  - [ ] Commands are pure GUI state.
  - [ ] Disabled reasons are visible.
  - [ ] No TUI command model is imported.

### W280-B02 — Keyboard contexts and collision checks

Goal:
  Add host-independent keyboard context projection for the first command set.

Design:
  - Model keybinding contexts for global shell, editor, project tree, diagnostics, run output, Immediate, debug, and command palette.
  - Define default keyboard gestures as data, with collision detection per context.
  - Keep host-specific overrides out of core.
  - Add `gui-keyboard-contexts-baseline` lab scenario.

Tests:
  - Default keybindings have no collisions within a context.
  - Same gesture may appear in distinct non-overlapping contexts only when explicitly allowed.
  - Disabled commands still render keyboard affordances with disabled reasons.
  - GUI-lab renders context/keybinding tokens.

Evidence:
  - `cargo test --manifest-path crates/Cargo.toml -p oxide-core`.
  - `cargo test --manifest-path crates/Cargo.toml -p oxide-guilab`.
  - Render command for `gui-keyboard-contexts-baseline`.

Closure:
  - [ ] Contexts are explicit.
  - [ ] Collision checks are tested.
  - [ ] No host/browser-specific key trap is hard-coded as product truth.

### W280-B03 — Focus graph and no-mouse route

Goal:
  Add deterministic focus graph state for navigating the current GUI surfaces without a mouse.

Design:
  - Model focus nodes for project tree, editor, diagnostics, lifecycle controls, run output, Immediate, debug, COM capability, and command palette.
  - Define a no-mouse traversal route for the thin-slice IDE surface.
  - Add restoration hints for returning from command palette and panels.
  - Add `gui-focus-graph-no-mouse` lab scenario.

Tests:
  - Focus graph contains all required current surface nodes.
  - No-mouse traversal route starts at project tree and reaches editor, diagnostics, run output, Immediate, debug, and command palette.
  - Disabled panels remain focusable when they contain explanatory disabled reasons.
  - GUI-lab render includes ordered focus tokens.

Evidence:
  - `cargo test --manifest-path crates/Cargo.toml -p oxide-core`.
  - `cargo test --manifest-path crates/Cargo.toml -p oxide-guilab`.
  - Render command for `gui-focus-graph-no-mouse`.

Closure:
  - [ ] Focus graph is deterministic.
  - [ ] No-mouse path is visible.
  - [ ] Disabled reason panels remain reachable.

### W280-B04 — Accessibility labels and disabled-reason projection

Goal:
  Add accessibility metadata that makes panels, commands, disabled reasons, and runtime limitations explicit in GUI-lab output.

Design:
  - Model semantic panel roles and accessible labels in pure GUI state.
  - Attach labels/descriptions to command palette, editor, diagnostics, run output, Immediate, debug, COM capability, and lifecycle surfaces.
  - Add `gui-accessibility-disabled-reasons` lab scenario.
  - Keep this as projection data; do not choose a concrete web framework accessibility API yet.

Tests:
  - Every current major surface has an accessible label.
  - Disabled run/Immediate/debug/COM states include accessible descriptions.
  - GUI-lab render includes semantic labels and disabled-reason tokens.

Evidence:
  - `cargo test --manifest-path crates/Cargo.toml -p oxide-core`.
  - `cargo test --manifest-path crates/Cargo.toml -p oxide-guilab`.
  - Render command for `gui-accessibility-disabled-reasons`.

Closure:
  - [ ] Major surfaces have labels.
  - [ ] Disabled reasons are accessible.
  - [ ] No web-framework commitment is hidden in core.

### W280-B05 — W280 acceptance and next handoff

Goal:
  Accept W280 with full regression evidence and prepare the next GUI workset/handoff.

Design:
  - Update `docs/GUI_FIXTURES_AND_LAB.md` with W280 scenarios and expected tokens.
  - Update this workset with acceptance evidence.
  - Add a next-workset handoff once W280 results clarify the next vertical slice.

Tests:
  - `cargo test --manifest-path crates/Cargo.toml --workspace`.
  - Render W210-W280 GUI-lab scenarios.
  - Grep command, keyboard, focus, and accessibility tokens.

Evidence:
  - Full nested workspace tests.
  - Rendered GUI-lab outputs.
  - Handoff note.

Closure:
  - [ ] W280 accepted or explicitly blocked with evidence.
  - [ ] W210-W280 regression scenarios pass.
  - [ ] Next workset prerequisites are documented.

## Out-of-scope

- Broad theming marketplace.
- General plugin system.
- Telemetry.
- Real runtime/debug/Immediate session implementation.
- Native COM service implementation.
- DnaOneCalc repo changes.
- Parked TUI substrate changes.
