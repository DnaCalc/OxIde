# Handoff — W300 Mounted Web Shell Decision

Status: `handoff_ready`
Date: 2026-05-07
Source workset: W290 — Host-Mounted GUI Shell

## Summary

W290 proved that OxIde can assemble a serializable `GuiShellPacket` from the W210-W280 pure GUI projections and render a static mounted-shell proof without importing parked TUI state or claiming untested host capabilities.

Recommended next step: start W300 as a thin mounted web shell adapter over `GuiShellPacket`, preserving `oxide-guilab` as deterministic regression evidence while adding one real browser/DOM smoke only for claims that can be tested.

## W290 Results To Preserve

W290 added these lab scenarios:

```text
gui-shell-packet-baseline
gui-mounted-shell-static
gui-mounted-command-palette
gui-mounted-no-mouse-accessibility
```

The mounted proofs currently state:

- `data-source="GuiShellPacket"` for the static shell,
- `data-source="GuiShellPacket.command_palette"` for mounted command rendering,
- `data-source="GuiShellPacket.focus_graph+accessibility"` for mounted no-mouse/accessibility rendering,
- `data-parked-tui-imported="false"`,
- `data-web-framework-bound="false"`,
- `data-dom-audited="false"`,
- `data-filesystem-persistence="false"`,
- `data-native-runtime="false"`,
- `data-com-runtime="false"`.

## Recommended W300 Direction

Prefer a narrow web-shell adapter workset before DnaOneCalc repo changes or native-service integration:

1. Choose the smallest Rust/WASM web adapter that can mount a `GuiShellPacket` into a browser DOM.
2. Keep `oxide-core` as the owner of GUI state and packet contracts.
3. Keep `oxide-guilab` snapshots as the fast deterministic regression path.
4. Add only tested DOM claims: static mount exists, visible text exists, and selected ARIA/role attributes exist if a DOM smoke verifies them.
5. Keep runtime/debug/Immediate, filesystem persistence, and COM execution unavailable unless a target-host test proves otherwise.

## Guardrails For W300

- Do not fork command, keyboard, focus, accessibility, lifecycle, run, COM, or embedding models.
- Do not import `src/shell/*` parked TUI state, keymaps, widgets, or command handlers.
- Do not duplicate sibling-repo OxVba or DnaOneCalc types.
- Do not mutate DnaOneCalc until a paired host workset is explicitly authorized.
- Do not claim accessibility compliance from packet metadata alone; a DOM/accessibility audit must produce evidence first.
- Do not claim real filesystem persistence without a disk-write test.
- Do not claim native OxVba runtime/debug/Immediate or COM execution without native host/service tests.

## Candidate W300 Beads

1. Register W300 mounted web shell adapter.
2. Add a minimal web-shell crate or adapter module that consumes `GuiShellPacket`.
3. Add one deterministic browser/DOM smoke for a static shell mount.
4. Add command-palette DOM smoke for stable command IDs and disabled reasons.
5. Add no-mouse/accessibility DOM smoke for visible labels/roles, while clearly separating smoke-tested attributes from full accessibility compliance.
6. Accept W300 and decide whether W310 should be DnaOneCalc host embedding, native filesystem/session persistence, or OxVba runtime service integration.

## Evidence From W290 Acceptance

Accepted commands:

```powershell
cargo test --manifest-path crates/Cargo.toml --workspace
```

Rendered W210-W290 scenarios were collected in:

```text
target/w290-acceptance-renders.txt
```

Token checks were collected in:

```text
target/w290-acceptance-grep.txt
```

Observed result: nested workspace tests passed; all W210-W290 lab scenarios rendered; W290 shell-packet, static shell, mounted command-palette, focus, and accessibility tokens were present. Frozen OxVba `cfg(kani)` warnings remain non-blocking.
