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
1. `W010` is the active shell-mockup workset.
2. `W020` and `W030` are stub follow-on packets.
3. Live execution state still belongs only in `.beads/`.

## Use These Instead
1. Use [PRODUCT_DIRECTION.md](/C:/Work/DnaCalc/OxIde/PRODUCT_DIRECTION.md) for product and UX authority.
2. Use [ARCHITECTURE.md](/C:/Work/DnaCalc/OxIde/ARCHITECTURE.md) for seam and implementation direction.
3. Use [WORKSET_REGISTER.md](/C:/Work/DnaCalc/OxIde/docs/WORKSET_REGISTER.md) for ordered workset truth.
4. Use [BEADS.md](/C:/Work/DnaCalc/OxIde/docs/BEADS.md) for the local bead method.
