# Handoff — GUI Workset Rollout

Status: `handoff_note`
Date: 2026-05-07

## Purpose

This note records the intended post-W200 GUI workset rollout for planning and cross-repo coordination.

## Proposed Sequence

1. `W210` — GUI project-open spine
   - browser GUI opens the thin-slice fixture,
   - project/module/source/capability state is visible.

2. `W220` — editable module and diagnostics
   - minimal editor surface,
   - document snapshot into OxVba,
   - diagnostics visible.

3. `W230` — save, reload, and session restore
   - dirty/save/reload/revert,
   - persistence capability honesty.

4. `W240` — capability-aware run/output path
   - runtime capability profile,
   - run request/events,
   - output/activity surface.

5. `W250` — DnaOneCalc embedded IDE/runtime proof
   - dependency direction decision,
   - shared component/bridge consumption.

6. `W260` — Windows COM capability proof
   - COM available/unavailable states,
   - native service contract or OxVba handoff.

7. `W270` — run/debug/immediate GUI surfaces
   - Immediate and debug surfaces over real/future seams.

8. `W280` — command, keyboard, accessibility, polish
   - command palette,
   - focus graph,
   - accessibility checks,
   - keyboard-first finish.

## Rule

Each workset should be vertical and reviewable. Avoid broad infrastructure-only worksets once W200 is complete.
