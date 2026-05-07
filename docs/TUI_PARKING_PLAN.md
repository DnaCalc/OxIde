# OxIde TUI Parking Plan

Status: `first_pass_parking_plan`
Date: 2026-05-07

## Purpose

This note defines how the existing FrankenTui direction is retained without letting it interfere with the new Rust/WASM-capable GUI pivot.

## Parking Meaning

Parking means:

1. the current TUI implementation remains available,
2. TUI docs, UX lab material, Fire Horse mockups, and WTD tests remain useful evidence,
3. TUI code is isolated or feature-gated so GUI architecture is not shaped by terminal constraints,
4. W010-W110 are treated as parked TUI lineage,
5. new active execution starts with W200 and the GUI lineage.

Parking does not mean deletion, disavowal, or preventing a future companion TUI.

## Implementation Bias

The parked TUI is not the base for the GUI product. During W200 and later GUI work, existing TUI code should be mined for requirements, behavior examples, UX lessons, and OxVba seam evidence. The default implementation choice should be new GUI-native code.

## Test Posture

WTD tests remain available for parked TUI regression checks. They should not be the default blocker for GUI development. The GUI line should gain its own browser/scenario/contract test loop.
