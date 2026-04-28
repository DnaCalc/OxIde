# OxIde Fire Horse TUI Mockups

Full-colour mockups for `docs/DESIGN_TUI_2026_FIRE_HORSE.md`.

Open `index.html` directly in a browser. The file is self-contained:
no build step, package install, or dev server is required.

The hardening pass lives in `HARDENING_REVIEW.md`. Treat that file as
the handoff note before converting these mockups into UX-lab or
FrankenTui targets.

## Included Screens

- Launchpad
- Editing with Semantic Lens
- Command Lens
- Run Lane
- Debug Cockpit
- Console Fit
- Compact Focus Mode

## Preferred Exports

- `refined_complete.png` - complete overview image
- `refined_01_launchpad.png`
- `refined_02_editing_lens.png`
- `refined_03_command_lens.png`
- `refined_04_run_lane.png`
- `refined_05_debug_cockpit.png`
- `refined_06_console_fit.png`
- `refined_07_compact_focus_mode.png`

The north-star screenshot is `refined_02_editing_lens.png`; the
secondary debug north-star is `refined_05_debug_cockpit.png`.

The `refined_*.png` files are the clean per-screen exports. Earlier
non-`refined_` PNGs in this directory are capture attempts or previous
exports and should not be treated as the approved set. They remain on
disk because repo-local rules forbid deleting generated files without
an exact user-approved delete command.

## Design Notes

These are visual targets, not implementation contracts. They are meant
to make the Fire Horse direction concrete enough to judge before
FrankenTui or UX-lab work starts.

The mockups use:

- Graphite Ember dark palette
- Paper Ember light palette for Console Fit
- rails over heavy boxes
- source-centered semantic lenses
- structured run/debug states
- command preview and disabled-command explanations
