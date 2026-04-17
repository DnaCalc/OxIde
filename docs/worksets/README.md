# OxIde Worksets

This folder contains the ordered workset packets for the current green-field
implementation sequence.

It is not a live execution tracker.
For ordered workset truth use [WORKSET_REGISTER.md](/C:/Work/DnaCalc/OxIde/docs/WORKSET_REGISTER.md).
For live execution state use [.beads/issues.jsonl](/C:/Work/DnaCalc/OxIde/.beads/issues.jsonl) through `br`.

## Current Rules
1. Worksets are planning and provenance packets, not the owner of ready or blocked state.
2. `.beads/` owns live execution truth.
3. Worksets explain scope, dependency order, and rollout intent.
4. New workset packets should be added only when the next execution sequence has been defined.

## Current State
1. `W010` through `W030` record the implemented foundation sequence.
2. `W035`, `W037`, and `W038` are prerequisite worksets before W040 (UX re-derivation, WTD harness, UX development lab).
3. `W040` through `W070` define the original forward execution sequence.
4. `W080` through `W110` define the follow-on sequence toward a feature-complete shell.
5. Live execution state still belongs only in `.beads/`.

## Ordered Sequence
1. `W010` - shell mockup scaffold and design proof
2. `W020` - runtime shell foundation
3. `W030` - service integration
4. `W035` - fresh UX design pass
5. `W037` - WinTermDriver test harness foundation
6. `W038` - UX development lab
7. `W040` - project and workspace management
8. `W050` - file and document services
9. `W060` - full language-service UX
10. `W070` - run/debug/immediate shell surfaces
11. `W080` - debug surfaces
12. `W090` - command system and keymap profiles
13. `W100` - terminal capability and onboarding
14. `W110` - polish, accessibility, and recovery

## Use These Instead
1. Use [PRODUCT_DIRECTION.md](/C:/Work/DnaCalc/OxIde/PRODUCT_DIRECTION.md) for product and UX authority.
2. Use [ARCHITECTURE.md](/C:/Work/DnaCalc/OxIde/ARCHITECTURE.md) for seam and implementation direction.
3. Use [WORKSET_REGISTER.md](/C:/Work/DnaCalc/OxIde/docs/WORKSET_REGISTER.md) for ordered workset truth.
4. Use [BEADS.md](/C:/Work/DnaCalc/OxIde/docs/BEADS.md) for the local bead method.
