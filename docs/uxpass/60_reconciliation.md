# 60 - Reconciliation

Status: `draft`
Workset: `W035`

## Purpose
Reconcile uxpass decisions back into the two authority docs that still carry
older shell wording:

- `PRODUCT_DIRECTION.md`
- `docs/DESIGN_TUI.md`

This file is the W035-B05 diff plan and W035-B06 application record for the
D9 wording change:

> four-band vertical frame with a scene-scoped body decomposition

## Applied Diff

### PRODUCT_DIRECTION.md

1. Added explicit D9 frame wording in the screen-space strategy section:
   - old: implicit frame description only.
   - new: explicit sentence naming the frame as a
     **four-band vertical frame with a scene-scoped body decomposition**
     and clarifying that only the body decomposition is scene/width scoped.

2. Added explicit D9 wording in the layout presets section so state-specific
   layouts map to body decomposition, not to whole-frame replacement.

### docs/DESIGN_TUI.md

1. Replaced the old canonical-shell frame section with D9 wording and shape:
   - top bar
   - scene-scoped body decomposition
   - lower utility surface
   - status line

2. Updated the frame ASCII sketch to include a dedicated status-line band.

3. Updated region and state sections so the status line is always present and
   scene differences are described as body/lower-surface decomposition rather
   than frame replacement.

## Verification Notes

- `PRODUCT_DIRECTION.md` and `docs/DESIGN_TUI.md` now both use the D9 phrase
  explicitly.
- The retired phrase "five-region frame" no longer appears in these two docs.

## Follow-through

- `00_principles.md`, `10_user_journeys.md`, and `20_frame_and_regions.md`
  remain the source for decision rationale.
- This reconciliation doc is the audit trail for the authority-doc wording
  alignment only.
