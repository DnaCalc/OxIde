# Fire Horse FrankenTui Terminal Review Pack

This directory is the current review pack for W038-B15.

It answers the gap found after W039: the earlier W039 renderer is still
useful as a plain text contract renderer, but it is too flat to judge the
Fire Horse UX. These captures come from the real FrankenTui mockup path:

```text
target/release/oxide-uxlab.exe --suite firehorse --scenario <id> --viewport <viewport> --once --mockup
target/release/oxide-uxlab.exe --suite firehorse --scenario <id> --viewport <viewport> --once --mockup --ansi
```

## Contents

- `captures/*.txt` — plain terminal-cell text extracted from the
  FrankenTui frame. Use these for quick review, comments, and diffs.
- `ansi/*.ansi` — styled terminal streams emitted through FrankenTui's
  presenter. Replay or print these in a modern terminal to review color,
  panel tone, and high-end visual rhythm.

Every Fire Horse scenario has First-class and Studio captures:

- Launchpad
- Editing Lens
- Command Lens
- Run Lane
- Debug Cockpit
- Console Fit
- Compact Focus
- Real Editing Adapter

Compact Focus also has `captures/compact_focus_default_compact.txt` to
review the deliberate small-terminal posture at its default compact
viewport.

## Review Position

Use this pack to evaluate density, layout, hierarchy, and emotional fit
against the colourful Fire Horse mockups. Use the W039 text goldens only
for named contract checks.

The mockup path does not switch the production `ox-ide` renderer and
does not implement project, semantic, run, or debug behavior. It is a
fixture/projection-backed UX proof surface.
