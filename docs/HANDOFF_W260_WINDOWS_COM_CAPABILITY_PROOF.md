# Handoff — W260 Windows COM Capability Proof

Status: `handoff_note`
Date: 2026-05-07

## W250 Baseline

W250 closed with an OxIde-side DnaOneCalc embedding contract and the current GUI-lab regression ladder:

```powershell
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-thin-slice-loaded
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-thin-slice-edited-diagnostics
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-thin-slice-lifecycle
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-run-output-browser-disabled
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-run-output-simulated-supported
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-dnaonecalc-embedding-contract
```

The W250 embedding contract preserves the browser-safe capability floor: native execution and COM runtime are not available in pure browser/WASM.

## W260 Starting Point

W260 should prove Windows COM capability honesty without collapsing distinct concepts:

1. COM reference is project truth.
2. COM reference discovery is host/platform capability.
3. COM runtime invocation is native Windows runtime capability.
4. Browser/WASM can show COM facts but cannot directly call COM.
5. Simulated run output is not evidence of native or COM runtime support.

## Recommended W260 Expansion

1. Expand `docs/worksets/W260_windows_com_capability_proof.md` into concrete beads before implementation.
2. Add a COM-reference-present fixture only when the first scenario/test consumes it.
3. Add pure capability-state types before UI rendering:
   - browser COM unavailable,
   - non-Windows native COM unavailable,
   - Windows native discovery available,
   - Windows native runtime available only through an explicit service route.
4. Add GUI-lab unavailable-state scenario before any native-positive scenario.
5. Decide whether COM service contracts belong in OxIde, OxVba, or a shared DNA Calc crate; prefer handoff over local duplication.

## Required Honesty Tokens For W260

W260 scenarios should make these distinctions visible:

- `COM reference present`,
- `COM discovery unavailable in browser-safe profile`,
- `COM runtime unavailable in browser-safe profile`,
- `Windows native host required`,
- `native COM service not configured` when applicable,
- no `COM available` claim in browser/WASM or simulated-run scenarios.

## Known Constraints

1. Pure browser/WASM cannot directly call Windows COM.
2. Windows COM support requires a native Windows host/service layer.
3. OxVba owns COM/runtime semantics; OxIde should not invent a duplicate runtime model.
4. W260 should not mutate DnaOneCalc; DnaOneCalc consumption remains a separate sibling-repo handoff path.
5. Full debugger/Immediate surfaces remain W270.

## Open Interface Questions

1. Does OxVba already expose, or should it expose, COM reference discovery/runtime capability DTOs?
2. Should a native Windows service be an OxIde host service, an OxVba service, or a shared DNA Calc service?
3. What fixture best proves COM reference presence without requiring COM invocation in browser tests?
4. How should W260 represent trust/safety policy for native COM invocation?
