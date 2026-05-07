# Handoff — W310 DnaOneCalc Web Shell Hosting

Status: `handoff_ready`
Date: 2026-05-07
Source workset: W300 — Mounted Web Shell Adapter

## Summary

W300 proved a thin OxIde web-shell adapter over `GuiShellPacket` plus parsed-HTML DOM smoke coverage for the static shell, command palette, and no-mouse/accessibility projections.

Recommended next step: W310 should deepen DnaOneCalc host embedding from the OxIde side first, then coordinate any DnaOneCalc repository changes separately. The immediate goal is a host contract that says exactly what DnaOneCalc would mount and what OxIde still owns.

## W300 Results To Preserve

W300 added these lab scenarios:

```text
gui-web-shell-adapter-boundary
gui-web-shell-dom-smoke
gui-web-command-palette-dom-smoke
gui-web-no-mouse-accessibility-dom-smoke
```

W300 evidence showed:

- `oxide-webshell` consumes `oxide-core::GuiShellPacket`,
- the adapter renders project tree, editor, diagnostics, lifecycle, run output, COM capability, command summary, focus/accessibility summary, and capability footer,
- parsed HTML smoke verifies packet-derived DOM attributes/text,
- command DOM smoke verifies command IDs, gestures, and disabled reasons,
- no-mouse/accessibility DOM smoke verifies route order, editor restoration, labels, descriptions, and disabled reasons,
- no browser runtime, DOM accessibility audit, filesystem persistence, native runtime, or COM runtime is claimed.

## Recommended W310 Direction

Start W310 as an OxIde-side DnaOneCalc web-shell hosting contract:

1. Add an OxIde-host packet that combines `EmbeddedIdePacket` and `GuiShellPacket`/web-shell expectations without duplicating DnaOneCalc types.
2. Add a GUI-lab scenario that renders the DnaOneCalc host boundary plus W300 web-shell DOM smoke status.
3. Document the exact DnaOneCalc-side mount API needed before any sibling repo modification.
4. Keep all DnaOneCalc repository changes as handoff items unless explicitly authorized.
5. Preserve browser-safe limitations: no filesystem persistence, no native runtime/debug/Immediate, and no COM runtime unless future host tests prove them.

## Guardrails For W310

- OxIde may write only inside the OxIde repo unless the user explicitly authorizes sibling changes.
- DnaOneCalc remains a consuming host, not the owner of OxIde IDE state.
- OxVba remains semantic/runtime/debug/Immediate/COM truth owner.
- Do not duplicate DnaOneCalc or OxVba types locally.
- Do not route OxIde internal semantics through LSP.
- Do not import parked TUI shell/state/widgets/keymaps.
- Do not claim real browser runtime hosting until a DnaOneCalc or browser-host smoke proves it.

## Candidate W310 Beads

1. Register W310 DnaOneCalc web-shell hosting contract.
2. Add an OxIde host-facing web-shell packet that composes `EmbeddedIdePacket` with W300 web-shell status.
3. Add `gui-dnaonecalc-web-shell-host-contract` GUI-lab scenario.
4. Add DnaOneCalc handoff with exact host API and smoke-test expectations.
5. Accept W310 and decide whether W320 is paired DnaOneCalc implementation, native filesystem persistence, or OxVba native runtime/service integration.

## Evidence From W300 Acceptance

Accepted commands:

```powershell
cargo test --manifest-path crates/Cargo.toml --workspace
```

Rendered W210-W300 scenarios were collected in:

```text
target/w300-acceptance-renders.txt
```

Token checks were collected in:

```text
target/w300-acceptance-grep.txt
```

Observed result: nested workspace tests passed; all W210-W300 lab scenarios rendered; W300 adapter, parsed DOM smoke, command-palette DOM smoke, no-mouse/accessibility DOM smoke, and no-claim tokens were present. Frozen OxVba `cfg(kani)` warnings remain non-blocking.
