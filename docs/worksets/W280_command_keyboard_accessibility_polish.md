# Workset W280 — Command, Keyboard, Accessibility, And Polish

## Ambition

The GUI becomes usable as an IDE rather than a demo shell: commands are unified, keyboard paths are complete, focus is predictable, and accessibility/polish are explicitly tested.

## Dependencies

- W270 — run/debug/immediate GUI surfaces.
- Earlier GUI scenario lab coverage.

## Design

The GUI pivot must retain the keyboard discipline learned from the TUI lane while using GUI-native accessibility and interaction patterns.

Likely implementation lanes:

1. command registry and stable IDs,
2. keybinding contexts and command palette,
3. focus graph and restoration,
4. visible disabled reasons,
5. screen-reader labels and semantic roles,
6. high-contrast/theme tokens,
7. no-mouse acceptance journeys.

## Beads

This scaffold is not execution-ready. Expand before implementation.

Expected bead lanes:

1. W280-B01 — command registry and palette.
2. W280-B02 — keybinding contexts and shortcut policy.
3. W280-B03 — focus graph and no-mouse path.
4. W280-B04 — accessibility labels and checks.
5. W280-B05 — polish/regression lock.

## Out-of-scope

- Broad theming marketplace.
- General plugin system.
- Telemetry.
