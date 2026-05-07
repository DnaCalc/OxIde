# Workset W290 — Host-Mounted GUI Shell

## Ambition

OxIde gets its first mounted GUI shell proof: a thin host-facing surface consumes the pure GUI projection state proven by W210-W280 and renders a reviewable IDE shell packet without rewriting projection logic or claiming untested native/runtime/browser capabilities.

W290 is a mounting and integration proof. It is not a runtime, COM, filesystem, or DnaOneCalc implementation workset.

## Dependencies

- W280 — command, keyboard, focus, accessibility, and polish projections.
- W270 — runtime/debug/Immediate surfaces and honest unavailable seams.
- W260 — COM capability proof and native-service-missing seams.
- W250 — DnaOneCalc embedding contract and host boundary.
- [`docs/HANDOFF_W290_HOST_MOUNTED_GUI_SHELL.md`](../HANDOFF_W290_HOST_MOUNTED_GUI_SHELL.md).
- [`docs/GUI_FIXTURES_AND_LAB.md`](../GUI_FIXTURES_AND_LAB.md).

## Guardrails

1. Keep `oxide-guilab` as deterministic regression evidence.
2. Consume existing pure projections; do not fork command, keyboard, focus, accessibility, lifecycle, run, COM, or embedding models.
3. Do not import parked TUI shell/state/widgets/keymaps.
4. Do not claim real filesystem persistence without disk-write tests.
5. Do not claim DOM accessibility compliance until a mounted DOM audit exists.
6. Do not claim native runtime, debug, Immediate, or COM execution beyond existing capability labels.
7. Keep DnaOneCalc as a consumer/host boundary, not the owner of OxIde IDE state.
8. Prefer a thin shell packet and static/mounted proof over broad framework selection.

## Scenario Plan

W290 should add deterministic scenarios before any full browser runner commitment:

```text
gui-shell-packet-baseline
gui-mounted-shell-static
gui-mounted-command-palette
gui-mounted-no-mouse-accessibility
```

The first slice should combine the existing projections into one serializable shell packet. A mounted/static shell proof can then render that packet.

## Beads

### W290-B00 — Register and expand host-mounted GUI shell workset

Goal:
  Create and register W290 as the next executable GUI workset for the first host-mounted shell proof.

Design:
  - Add `docs/worksets/W290_host_mounted_gui_shell.md` with concrete beads.
  - Update `docs/WORKSET_REGISTER.md` and `docs/worksets/README.md`.
  - Preserve W280 pure projection contracts and GUI-lab regression baseline.

Tests:
  - Documentation review against `docs/HANDOFF_W290_HOST_MOUNTED_GUI_SHELL.md`.

Evidence:
  - Registered W290 workset and executable bead list.

Closure:
  - [ ] W290 is in the active sequence.
  - [ ] W290 has concrete beads.
  - [ ] Mounted-shell guardrails are explicit.

### W290-B01 — Shell projection packet

Goal:
  Add one serializable shell packet that combines the current OxIde GUI projections into a mounted-shell contract.

Design:
  - Add a shell packet in `oxide-core` or a small bridge crate if layering requires it.
  - Include project identity, module/source summary, diagnostics summary, lifecycle state, run output/timeline, COM capability, command palette, keyboard map, focus graph, accessibility projection, and capability footer text.
  - Reuse existing W210-W280 types; do not duplicate sibling repo or runtime semantics.
  - Add `gui-shell-packet-baseline` GUI-lab scenario.

Tests:
  - Packet serializes and round-trips.
  - Packet reuses existing command/keyboard/focus/accessibility projections.
  - Browser-safe limitations survive in the packet.
  - GUI-lab renders packet ownership/capability tokens.

Evidence:
  - `cargo test --manifest-path crates/Cargo.toml -p oxide-core`.
  - `cargo test --manifest-path crates/Cargo.toml -p oxide-guilab`.
  - Render command for `gui-shell-packet-baseline`.

Closure:
  - [ ] Shell packet is serializable.
  - [ ] Existing projections are reused.
  - [ ] Browser/native/runtime limitations remain visible.

### W290-B02 — Static mounted shell proof

Goal:
  Add the smallest mounted/static GUI shell proof that renders the shell packet.

Design:
  - Prefer a static HTML/text snapshot renderer before broad framework commitment.
  - Render project tree, editor, diagnostics, lifecycle, run output, COM capability, command palette, keyboard/focus/accessibility metadata, and capability footer.
  - Add `gui-mounted-shell-static` scenario.

Tests:
  - Mounted/static render includes all major surfaces.
  - Render consumes the shell packet rather than rebuilding state independently.
  - Render does not claim DOM accessibility, filesystem persistence, native runtime, debug, Immediate, or COM support.

Evidence:
  - `cargo test --manifest-path crates/Cargo.toml -p oxide-guilab`.
  - Render command for `gui-mounted-shell-static`.

Closure:
  - [ ] Shell proof consumes the packet.
  - [ ] Major surfaces render together.
  - [ ] No untested host capability is claimed.

### W290-B03 — Mounted command palette slice

Goal:
  Prove command palette state survives packet-to-mounted-shell rendering.

Design:
  - Render stable command IDs, labels, categories, keyboard gestures, availability, and disabled reasons in the mounted shell.
  - Preserve `data-parked-tui-imported="false"` or equivalent evidence.
  - Add `gui-mounted-command-palette` scenario.

Tests:
  - Runtime/Immediate/debug disabled reasons are visible.
  - Command IDs remain stable.
  - Keyboard gestures from W280 remain associated with commands.

Evidence:
  - `cargo test --manifest-path crates/Cargo.toml -p oxide-guilab`.
  - Render command for `gui-mounted-command-palette`.

Closure:
  - [ ] Command palette renders from packet state.
  - [ ] Disabled reasons remain visible.
  - [ ] No TUI command model is imported.

### W290-B04 — Mounted no-mouse and accessibility slice

Goal:
  Prove focus route and accessibility labels survive packet-to-mounted-shell rendering.

Design:
  - Render the no-mouse route, focus restore hints, accessible labels/descriptions, and disabled-reason descriptions.
  - Keep `data-web-framework-bound="false"` until a concrete DOM audit exists.
  - Add `gui-mounted-no-mouse-accessibility` scenario.

Tests:
  - No-mouse route includes project tree, editor, diagnostics, run output, Immediate, debug, COM, command palette, and editor restoration.
  - Accessible labels and disabled reasons are visible.
  - Mounted proof does not claim audited DOM accessibility.

Evidence:
  - `cargo test --manifest-path crates/Cargo.toml -p oxide-guilab`.
  - Render command for `gui-mounted-no-mouse-accessibility`.

Closure:
  - [ ] Focus route survives mounted rendering.
  - [ ] Accessibility labels survive mounted rendering.
  - [ ] DOM/audit limitations remain explicit.

### W290-B05 — W290 acceptance and host handoff

Status: Accepted 2026-05-07.

Goal:
  Accept W290 with full regression evidence and document the next host/framework decision.

Design:
  - Update `docs/GUI_FIXTURES_AND_LAB.md` with W290 scenario tokens.
  - Update this workset with acceptance evidence.
  - Add handoff for the next workset once W290 results clarify whether to deepen mounted web shell, DnaOneCalc host embedding, or native service integration.

Tests:
  - `cargo test --manifest-path crates/Cargo.toml --workspace`.
  - Render W210-W290 GUI-lab scenarios.
  - Grep shell packet, mounted shell, command palette, focus, and accessibility tokens.

Evidence:
  - `cargo test --manifest-path crates/Cargo.toml --workspace` passed.
  - W210-W290 GUI-lab renders collected in `target/w290-acceptance-renders.txt`.
  - Shell packet, mounted shell, command palette, focus, and accessibility token checks collected in `target/w290-acceptance-grep.txt`.
  - Next host/framework handoff: [`docs/HANDOFF_W300_MOUNTED_WEB_SHELL_DECISION.md`](../HANDOFF_W300_MOUNTED_WEB_SHELL_DECISION.md).
  - Frozen OxVba `cfg(kani)` warnings remain non-blocking.

Closure:
  - [x] W290 accepted or explicitly blocked with evidence.
  - [x] W210-W290 regression scenarios pass.
  - [x] Next host/framework prerequisites are documented.

## Out-of-scope

- Choosing a broad framework architecture without a thin proof.
- Real DOM accessibility audit.
- Real filesystem persistence.
- Real runtime/debug/Immediate execution.
- Native COM service implementation.
- DnaOneCalc repo changes.
- Parked TUI substrate changes.
