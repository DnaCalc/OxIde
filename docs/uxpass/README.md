# OxIde UX Design Pass

Status: `in progress`
Owned by: `W035`

## 1. Purpose
This directory holds the deliverables of the W035 UX design pass: a
deliberate, time-boxed "blank sheet" re-derivation of the OxIde shell UX,
run on top of the W010-W030 foundation and reconciled back into
`PRODUCT_DIRECTION.md` and `docs/DESIGN_TUI.md`.

W035 is active. Work-in-progress documents appear below as they land;
the reconciliation file is the exit gate.

## 2. Planned Deliverables
1. [00_principles.md](00_principles.md) - design principles distilled from first principles; used as tie-breakers for every later decision. **Draft landed.**
2. [10_user_journeys.md](10_user_journeys.md) - concrete user journeys with the exact keystrokes and visible state transitions, cross-referenced to principle numbers. **Draft landed with J1-J4.**
3. [20_frame_and_regions.md](20_frame_and_regions.md) - the canonical frame revisited (retires "five-region" in favor of a four-band vertical frame with a scene-scoped body decomposition, pinned width-class thresholds, and D9-D18 decisions). **Draft landed.**
4. `30_scene_catalogue.md` - scenes and modes (Empty, Editing, Semantic, BuildRun, Debug, Palette, overlay stack) with data, inputs, and transitions. _Next._
5. `40_command_model.md` - unified action registry, palette behavior, keymap profiles (default and VBA-IDE-compatible), chord and mnemonic handling, mouse mapping.
6. `50_visual_language.md` - palette (light and dark), typography, density, border language, motion and animation policy, degradation rules for weak terminals.
7. `60_reconciliation.md` - explicit diff vs `PRODUCT_DIRECTION.md` and `docs/DESIGN_TUI.md`; lists what changes, what stays, what gets amended.

Supporting evidence lives under [captures/](captures/): one
subdirectory per journey, each file a `wtd capture` of a specific state
referenced by `10_user_journeys.md`.

## 3. Method
- **Observe the running shell first, sketch second.** Every claim in these docs is grounded in a committed capture under `captures/`; ASCII target sketches come after observation, not before.
- **Decide explicitly.** Each file ends with a numbered, imperative "decisions" section. Later worksets cite those decision numbers rather than re-debating them.
- **Reconcile, don't fork.** `60_reconciliation.md` is the authoritative diff back into the existing canonical docs; the pass completes only when those docs are amended to match.

## 4. Open Questions For The Pass
These are questions the current pre-W030 docs answer implicitly and that
W035 should answer explicitly:

- Does the editor live in a single pane, or are split panes a day-one feature?
- Does the Immediate panel live in the lower surface, or as a first-class region?
- Does the command palette also serve as the "go to file / symbol / line" entry point, or are those separate?
- How does Debug reshape the layout - replacement scene or overlay?
- What is the minimum-viable mouse story: focus clicks only, selection drag, scroll, or full parity?
- What is the terminal-capability onboarding surface - a first-run page, a status-line hint, or a palette command?
