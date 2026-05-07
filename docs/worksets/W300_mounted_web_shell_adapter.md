# Workset W300 — Mounted Web Shell Adapter

## Ambition

OxIde gets its first real browser/DOM-mounted GUI shell adapter over the W290 `GuiShellPacket`: a thin web surface mounts packet-derived state into a testable DOM without moving ownership out of `oxide-core`, importing parked TUI state, or claiming untested filesystem/runtime/debug/Immediate/COM capability.

W300 is a mounting-adapter workset. It is not a native runtime, COM service, filesystem persistence, or DnaOneCalc repository integration workset.

## Dependencies

- W290 — host-mounted GUI shell packet and static mounted proof.
- W280 — command, keyboard, focus, accessibility, and polish projections.
- W270 — runtime/debug/Immediate surfaces and honest unavailable seams.
- W260 — COM capability proof and native-service-missing seams.
- W250 — DnaOneCalc embedding contract and host boundary.
- [`docs/HANDOFF_W300_MOUNTED_WEB_SHELL_DECISION.md`](../HANDOFF_W300_MOUNTED_WEB_SHELL_DECISION.md).
- [`docs/GUI_FIXTURES_AND_LAB.md`](../GUI_FIXTURES_AND_LAB.md).

## Guardrails

1. `oxide-core::GuiShellPacket` remains the state contract for mounted shell rendering.
2. The web adapter consumes packet state; it must not fork command, keyboard, focus, accessibility, lifecycle, run, COM, or embedding models.
3. Do not import parked TUI shell/state/widgets/keymaps/command handlers.
4. Do not duplicate OxVba, DnaOneCalc, or sibling-repo semantic types.
5. Keep `oxide-guilab` deterministic as the fast regression evidence path.
6. Claim only DOM behavior that has a DOM/browser test in W300 evidence.
7. Do not claim accessibility compliance from packet metadata alone; distinguish DOM smoke-tested roles/labels from a full accessibility audit.
8. Do not claim real filesystem persistence without disk-write tests.
9. Do not claim native runtime, debug, Immediate, or COM execution beyond existing capability labels.
10. Do not mutate DnaOneCalc or OxVba repos from this OxIde workset; capture handoffs instead.

## Design

W300 should add the thinnest viable web-shell adapter around `GuiShellPacket`:

- a small crate or module that renders the packet into DOM-like shell markup,
- a browser/DOM smoke harness that verifies mounted text/attributes rather than only string snapshots,
- deterministic GUI-lab scenarios retained as regression evidence,
- explicit capability flags for browser-safe limitations,
- one path for command-palette DOM projection,
- one path for no-mouse/focus/accessibility DOM projection.

The adapter can choose a framework only if the first bead keeps the choice reversible and evidence-driven. A static DOM renderer or minimal WASM mount is acceptable if it proves the browser seam without committing broad architecture.

## Scenario Plan

W300 should extend, not replace, W290 scenarios:

```text
gui-web-shell-dom-smoke
gui-web-command-palette-dom-smoke
gui-web-no-mouse-accessibility-dom-smoke
```

The scenario names should only claim DOM smoke coverage. They should not claim full accessibility compliance, persistent storage, native runtime, debug/Immediate execution, or COM execution.

## Beads

### W300-B00 — Register mounted web shell adapter workset

Goal:
  Register W300 as the next active GUI workset after W290 acceptance.

Design:
  - Add `docs/worksets/W300_mounted_web_shell_adapter.md`.
  - Update `docs/WORKSET_REGISTER.md` and `docs/worksets/README.md`.
  - Use `docs/HANDOFF_W300_MOUNTED_WEB_SHELL_DECISION.md` as design input.
  - Keep W300 scoped to a thin web/DOM adapter over `GuiShellPacket`.

Tests:
  - Documentation review against W290 handoff and guardrails.

Evidence:
  - Registered W300 workset and executable bead list.

Closure:
  - [ ] W300 is in the active sequence.
  - [ ] W300 has concrete beads.
  - [ ] Guardrails preserve `GuiShellPacket` ownership and no untested capability claims.

### W300-B01 — Web shell adapter boundary

Goal:
  Add the smallest rendering boundary that turns a `GuiShellPacket` into web-shell markup or DOM instructions without selecting a broad architecture prematurely.

Design:
  - Add a focused crate/module only if layering requires it.
  - Accept `GuiShellPacket` as input.
  - Render project tree, editor, diagnostics, lifecycle, run output, COM capability, command summary, focus/accessibility metadata, and capability footer.
  - Keep no-claim flags visible: DOM audit false unless tested, filesystem false, native runtime false, COM runtime false.

Tests:
  - Boundary consumes `GuiShellPacket` rather than rebuilding state.
  - Render includes all major W290 shell surfaces.
  - No parked TUI modules are imported.

Evidence:
  - `cargo test --manifest-path crates/Cargo.toml --workspace`.
  - Render/snapshot command for the adapter boundary.

Closure:
  - [ ] Adapter boundary consumes `GuiShellPacket`.
  - [ ] Major shell surfaces render.
  - [ ] No untested host capability is claimed.

### W300-B02 — Browser/DOM static shell smoke

Goal:
  Prove that the web shell adapter can mount packet-derived static shell state into a browser/DOM-testable surface.

Design:
  - Add a deterministic DOM smoke harness appropriate to the selected minimal adapter.
  - Verify visible project/module/source text and stable data attributes.
  - Keep full accessibility compliance out-of-scope unless audited.

Tests:
  - DOM contains `ThinSliceHello`, `Module1.bas`, and shell surface roles/attributes.
  - DOM contains no-claim flags for filesystem/native runtime/COM.
  - DOM smoke result is deterministic in CI/local agent runs.

Evidence:
  - `cargo test --manifest-path crates/Cargo.toml --workspace`.
  - DOM smoke output for `gui-web-shell-dom-smoke`.

Closure:
  - [ ] Static shell mounts into DOM-testable output.
  - [ ] DOM smoke verifies visible shell text/attributes.
  - [ ] No DOM accessibility audit is claimed.

### W300-B03 — Web command palette DOM smoke

Goal:
  Prove mounted command palette rows, gestures, availability, and disabled reasons survive through the web/DOM adapter.

Design:
  - Consume `GuiShellPacket.command_palette` and `GuiShellPacket.keyboard_map`.
  - Render stable command IDs and disabled reasons as DOM-testable attributes/text.
  - Preserve `data-parked-tui-imported="false"`.

Tests:
  - DOM contains `project.open`, `document.save`, `runtime.run`, `runtime.immediate`, `runtime.debug`, and `shell.command_palette`.
  - DOM contains `Ctrl+S`, `F5`, `Enter`, `F10`, and `Ctrl+Shift+P` where packet bindings provide them.
  - DOM contains runtime/Immediate/debug disabled reasons.

Evidence:
  - `cargo test --manifest-path crates/Cargo.toml --workspace`.
  - DOM smoke output for `gui-web-command-palette-dom-smoke`.

Closure:
  - [ ] Command IDs and labels survive DOM mounting.
  - [ ] Keyboard gestures survive DOM mounting.
  - [ ] Disabled reasons remain visible.

### W300-B04 — Web no-mouse/accessibility DOM smoke

Goal:
  Prove the no-mouse route and accessible labels/descriptions become DOM-testable without claiming full accessibility compliance.

Design:
  - Consume `GuiShellPacket.focus_graph` and `GuiShellPacket.accessibility`.
  - Render route order, restoration hints, labels, descriptions, and disabled reasons.
  - Label the test as DOM smoke, not audit/compliance.

Tests:
  - DOM contains route order through project tree, editor, diagnostics, run output, Immediate, debug, COM, command palette, and editor restoration.
  - DOM contains labels/descriptions for major surfaces.
  - DOM contains disabled reasons for runtime/Immediate/debug/COM surfaces.

Evidence:
  - `cargo test --manifest-path crates/Cargo.toml --workspace`.
  - DOM smoke output for `gui-web-no-mouse-accessibility-dom-smoke`.

Closure:
  - [ ] Focus route survives DOM mounting.
  - [ ] Accessible labels/descriptions survive DOM mounting.
  - [ ] DOM smoke remains distinct from full accessibility audit.

### W300-B05 — W300 acceptance and next integration decision

Status: Accepted 2026-05-07.

Goal:
  Accept W300 with regression evidence and decide whether the next workset should deepen DnaOneCalc host embedding, native filesystem/session persistence, or OxVba runtime/native service integration.

Design:
  - Update `docs/GUI_FIXTURES_AND_LAB.md` with W300 scenario tokens.
  - Add a handoff for the selected next workset.
  - Keep W210-W300 regression renders available.

Tests:
  - `cargo test --manifest-path crates/Cargo.toml --workspace`.
  - Render W210-W300 GUI-lab scenarios.
  - Run W300 DOM smoke commands.
  - Grep shell packet, DOM mount, command palette, focus, accessibility, and no-claim tokens.

Evidence:
  - `cargo test --manifest-path crates/Cargo.toml --workspace` passed.
  - W210-W300 GUI-lab renders collected in `target/w300-acceptance-renders.txt`.
  - Adapter, parsed DOM smoke, command-palette DOM smoke, no-mouse/accessibility DOM smoke, and no-claim token checks collected in `target/w300-acceptance-grep.txt`.
  - Next integration handoff: [`docs/HANDOFF_W310_DNAONECALC_WEB_SHELL_HOSTING.md`](../HANDOFF_W310_DNAONECALC_WEB_SHELL_HOSTING.md).
  - Frozen OxVba `cfg(kani)` warnings remain non-blocking.

Closure:
  - [x] W300 accepted or explicitly blocked with evidence.
  - [x] W210-W300 regression scenarios pass.
  - [x] Next integration prerequisites are documented.

## Out-of-scope

- Broad framework architecture beyond what the thin adapter proves.
- Full accessibility audit/compliance claim.
- Real filesystem persistence.
- Real runtime/debug/Immediate execution.
- Native COM service implementation.
- DnaOneCalc repo changes.
- OxVba repo changes.
- Parked TUI substrate changes.
