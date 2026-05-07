# Workset W260 — Windows COM Capability Proof

## Ambition

COM-dependent OxVba projects are handled honestly: browser and non-Windows hosts show unavailable states, while the Windows-native path has a designed and tested capability route.

## Dependencies

- W240 — capability-aware run/output path.
- OxVba COM reference/runtime contracts.
- Windows-native host service decision or OxVba handoff.

## Design

Pure browser/WASM cannot call Windows COM. The architecture must distinguish project truth, reference discovery, and runtime invocation capability.

Likely implementation lanes:

1. COM reference present fixture,
2. browser/non-Windows unavailable UI and disabled reasons,
3. Windows native capability surface,
4. native COM service contract or OxVba handoff,
5. tests for capability matrix behavior.

## Beads

This scaffold is not execution-ready. Expand before implementation.

Expected bead lanes:

1. W260-B01 — COM fixture and capability states.
2. W260-B02 — browser/non-Windows unavailable scenarios.
3. W260-B03 — Windows native service contract.
4. W260-B04 — OxVba/shared handoff for COM service gaps.
5. W260-B05 — capability matrix acceptance.

## Out-of-scope

- Full COM runtime parity.
- Non-Windows COM substitutes.
- Browser-only COM execution.
