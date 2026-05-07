# Handoff — W290 Host-Mounted GUI Shell

Status: `next_workset_handoff`
Date: 2026-05-07

## Source Workset

W280 accepted pure command, keyboard, focus, and accessibility projections plus deterministic GUI-lab renders:

```powershell
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-command-palette-baseline
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-keyboard-contexts-baseline
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-focus-graph-no-mouse
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-accessibility-disabled-reasons
```

W290 should prove the first mounted GUI shell without rewriting these pure projections.

## Recommended W290 Ambition

Mount a thin browser/desktop GUI shell that consumes OxIde projection state and renders the thin-slice IDE surface with command palette, keyboard/focus metadata, and accessibility labels wired as visible DOM/app state.

This should be a mounted-shell proof, not a runtime/native-service proof.

## Regression Baseline

Start with the full W210-W280 lab set:

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
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-command-palette-baseline
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-keyboard-contexts-baseline
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-focus-graph-no-mouse
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-accessibility-disabled-reasons
```

## Suggested W290 Beads

### W290-B00 — Register And Expand Workset

Create the W290 workset spec, add it to the register, and define concrete mounted-shell beads.

### W290-B01 — Shell Projection Packet

Add a single serializable GUI shell packet that combines the existing project spine, source, diagnostics, lifecycle, run output, COM capability, command palette, keyboard map, focus graph, and accessibility projection.

### W290-B02 — Mounted Shell Smoke

Add the smallest mounted GUI crate or host harness that can render the shell packet. The proof may be text/HTML snapshot first if a browser runner is not yet chosen.

### W290-B03 — Command Palette Mount

Wire command palette state into the mounted shell and prove disabled reasons remain visible.

### W290-B04 — Keyboard/Focus/A11y Mount

Wire keyboard/focus/accessibility metadata into the mounted shell and prove no-mouse route and labels survive projection-to-view.

### W290-B05 — Acceptance And Host Handoff

Run full W210-W290 regression evidence and document any DnaOneCalc or OxVba coordination needs.

## Guardrails

1. Do not replace `oxide-guilab`; keep it as deterministic regression evidence.
2. Do not move OxIde IDE state ownership into DnaOneCalc.
3. Do not claim real filesystem persistence without disk-write tests.
4. Do not claim DOM accessibility compliance until a mounted DOM audit exists.
5. Do not claim native runtime, debug, Immediate, or COM execution beyond existing capability labels.
6. Do not import parked TUI command/key/focus state.
7. Prefer a thin mounted shell over broad framework decisions.

## First Scenario Ideas

```text
gui-shell-packet-baseline
gui-mounted-shell-static
gui-mounted-command-palette
gui-mounted-no-mouse-accessibility
```

The first W290 slice should be narrow: prove one shell packet that combines existing projections before choosing or broadening mounted UI framework work.
