# Workset W310 — DnaOneCalc Web Shell Hosting

## Ambition

OxIde defines the DnaOneCalc web-shell hosting contract for the W300 `GuiShellPacket`/web adapter: DnaOneCalc can see exactly what it would mount, what OxIde owns, and what remains unavailable, without requiring sibling repository writes in this workset.

W310 is an OxIde-side host contract and handoff workset. It is not a DnaOneCalc implementation workset unless the user explicitly authorizes sibling repo changes.

## Dependencies

- W300 — mounted web shell adapter and parsed DOM smoke coverage.
- W290 — `GuiShellPacket` and static mounted shell proof.
- W250 — DnaOneCalc embedding contract and `EmbeddedIdePacket` boundary.
- [`docs/HANDOFF_W310_DNAONECALC_WEB_SHELL_HOSTING.md`](../HANDOFF_W310_DNAONECALC_WEB_SHELL_HOSTING.md).
- [`docs/HANDOFF_DNAONECALC_EMBEDDING_CONTRACT.md`](../HANDOFF_DNAONECALC_EMBEDDING_CONTRACT.md).
- [`docs/GUI_FIXTURES_AND_LAB.md`](../GUI_FIXTURES_AND_LAB.md).

## Guardrails

1. OxIde may write only inside the OxIde repo unless the user explicitly authorizes sibling changes.
2. DnaOneCalc remains a consuming host, not the owner of OxIde IDE state.
3. OxIde owns the IDE shell packet and web-shell adapter contracts.
4. OxVba remains semantic/runtime/debug/Immediate/COM truth owner.
5. Do not duplicate DnaOneCalc, OxVba, or sibling-repo semantic types.
6. Do not route OxIde internal semantics through LSP.
7. Do not import parked TUI shell/state/widgets/keymaps/command handlers.
8. Do not claim real DnaOneCalc browser hosting until a paired host smoke proves it.
9. Do not claim filesystem persistence, native runtime/debug/Immediate, or COM runtime without tests.

## Design

W310 should compose existing OxIde contracts rather than inventing a host model:

- `oxide-bridge::EmbeddedIdePacket` remains the host embedding boundary.
- `oxide-core::GuiShellPacket` remains the IDE/web shell state boundary.
- `oxide-webshell` remains the packet-to-web-shell adapter and DOM smoke owner.
- A new OxIde-side host contract may combine these facts for DnaOneCalc review.
- Any DnaOneCalc-side API requirements are documented as handoff notes.

## Scenario Plan

W310 should add deterministic GUI-lab scenarios before any sibling repo changes:

```text
gui-dnaonecalc-web-shell-host-contract
gui-dnaonecalc-web-shell-dom-readiness
```

The scenarios should show ownership boundaries, surface slots, expected mount inputs, W300 DOM smoke readiness, and unchanged no-claim flags.

## Beads

### W310-B00 — Register DnaOneCalc web shell hosting workset

Goal:
  Register W310 as the next active GUI workset after W300 acceptance.

Design:
  - Add `docs/worksets/W310_dnaonecalc_web_shell_hosting.md`.
  - Update `docs/WORKSET_REGISTER.md` and `docs/worksets/README.md`.
  - Use `docs/HANDOFF_W310_DNAONECALC_WEB_SHELL_HOSTING.md` as design input.
  - Keep W310 scoped to an OxIde-side DnaOneCalc host contract unless sibling repo writes are explicitly authorized.

Tests:
  - Documentation review against W300 handoff and cross-repo guardrails.

Evidence:
  - Registered W310 workset and executable bead list.

Closure:
  - [ ] W310 is in the active sequence.
  - [ ] W310 has concrete beads.
  - [ ] Guardrails preserve DnaOneCalc as consuming host and OxIde as IDE state owner.

### W310-B01 — OxIde-side DnaOneCalc web host contract packet

Goal:
  Add an OxIde-side packet that composes the existing DnaOneCalc embedding contract with W300 web-shell readiness without duplicating sibling types.

Design:
  - Add the packet in `oxide-bridge` or another host-boundary crate if layering requires it.
  - Include `EmbeddedIdePacket` ownership/surface facts.
  - Include `GuiShellPacket` identity or summary fields needed by the web shell.
  - Include W300 DOM smoke readiness/no-claim flags.
  - Keep DnaOneCalc repo writes false.

Tests:
  - Packet round-trips through JSON.
  - Packet preserves DnaOneCalc/OxIde/OxVba ownership boundaries.
  - Packet includes W300 DOM smoke readiness and no-claim flags.

Evidence:
  - `cargo test --manifest-path crates/Cargo.toml -p oxide-bridge`.
  - `cargo test --manifest-path crates/Cargo.toml --workspace`.

Closure:
  - [ ] Contract packet composes existing bridge/core/web-shell facts.
  - [ ] Sibling repo writes remain false.
  - [ ] No untested capability is claimed.

### W310-B02 — DnaOneCalc web shell host contract scenario

Goal:
  Render a deterministic GUI-lab scenario for the DnaOneCalc host contract.

Design:
  - Add `gui-dnaonecalc-web-shell-host-contract`.
  - Render host identity, ownership boundaries, expected mount inputs, surface slots, DOM smoke readiness, and no-claim flags.
  - State explicitly that no DnaOneCalc repo files were modified.

Tests:
  - Scenario includes DnaOneCalc, OxIde, and OxVba boundaries.
  - Scenario includes `GuiShellPacket`, `EmbeddedIdePacket`, and `oxide-webshell` readiness.
  - Scenario includes no sibling writes and no runtime/filesystem/COM claims.

Evidence:
  - `cargo test --manifest-path crates/Cargo.toml -p oxide-guilab`.
  - Render command for `gui-dnaonecalc-web-shell-host-contract`.

Closure:
  - [ ] Host contract renders deterministically.
  - [ ] Ownership boundaries are visible.
  - [ ] No sibling repo write or untested capability is claimed.

### W310-B03 — DnaOneCalc web shell DOM readiness scenario

Goal:
  Render DnaOneCalc-specific DOM readiness using W300 parsed DOM smoke results.

Design:
  - Add `gui-dnaonecalc-web-shell-dom-readiness`.
  - Reuse W300 DOM smoke reports rather than reimplementing checks.
  - Mark readiness as OxIde-side parsed DOM readiness, not a DnaOneCalc browser host smoke.

Tests:
  - Scenario includes static shell, command palette, and no-mouse/accessibility DOM smoke pass flags.
  - Scenario states browser runtime and DnaOneCalc host smoke are not claimed.
  - Scenario preserves no-claim flags.

Evidence:
  - `cargo test --manifest-path crates/Cargo.toml -p oxide-guilab`.
  - Render command for `gui-dnaonecalc-web-shell-dom-readiness`.

Closure:
  - [ ] W300 DOM smoke readiness is visible for DnaOneCalc.
  - [ ] Browser host smoke remains explicitly unclaimed.
  - [ ] No full accessibility audit is claimed.

### W310-B04 — DnaOneCalc host API handoff

Goal:
  Document the exact DnaOneCalc-side mount API and smoke-test expectations needed for a paired implementation.

Design:
  - Add a handoff note for DnaOneCalc changes.
  - Include expected packet shape, mount function/props, smoke-test tokens, and capability limitations.
  - Keep public/sibling changes unmade from this workset.

Tests:
  - Documentation review against W310 scenarios and W300 evidence.

Evidence:
  - Handoff note with concrete API and test expectations.

Closure:
  - [ ] DnaOneCalc-side prerequisites are documented.
  - [ ] No sibling repo files were modified.
  - [ ] Capability limitations are explicit.

### W310-B05 — W310 acceptance and next workset decision

Goal:
  Accept W310 and decide whether W320 should be paired DnaOneCalc implementation, native filesystem/session persistence, or OxVba runtime/native service integration.

Design:
  - Update `docs/GUI_FIXTURES_AND_LAB.md` with W310 scenario tokens.
  - Add a next-workset handoff.
  - Preserve W210-W310 regression renders.

Tests:
  - `cargo test --manifest-path crates/Cargo.toml --workspace`.
  - Render W210-W310 GUI-lab scenarios.
  - Grep DnaOneCalc host contract, DOM readiness, ownership, no-claim, and no-sibling-write tokens.

Evidence:
  - Full nested workspace tests.
  - Rendered GUI-lab outputs.
  - Handoff note.

Closure:
  - [ ] W310 accepted or explicitly blocked with evidence.
  - [ ] W210-W310 regression scenarios pass.
  - [ ] Next workset prerequisites are documented.

## Out-of-scope

- Writing to the DnaOneCalc repository without explicit authorization.
- Real DnaOneCalc browser host mount.
- Full accessibility audit/compliance claim.
- Real filesystem persistence.
- Real runtime/debug/Immediate execution.
- Native COM service implementation.
- OxVba repo changes.
- Parked TUI substrate changes.
