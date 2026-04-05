# OxIde Workset Register

Status: `active`
Date: 2026-04-05

## 1. Purpose
This is the live ordered workset register for OxIde.

At the moment it records the first green-field implementation sequence:
1. active worksets now exist,
2. the ordered execution sequence is defined below,
3. live execution state remains in `.beads/`.

This file is not an execution-status board.
It owns workset truth, not bead state.

## 2. Planning-Surface Clarification
Planning and execution truth in OxIde is split as follows:
1. [PRODUCT_DIRECTION.md](/C:/Work/DnaCalc/OxIde/PRODUCT_DIRECTION.md) owns product direction and UX authority.
2. [ARCHITECTURE.md](/C:/Work/DnaCalc/OxIde/ARCHITECTURE.md) owns seam and implementation-direction authority.
3. [OPERATIONS.md](/C:/Work/DnaCalc/OxIde/OPERATIONS.md) owns the local execution model.
4. this register owns ordered workset truth.
5. `.beads/` owns epics, beads, readiness, blockers, in-progress state, and closure.

## 3. Current State
The current ordered workset sequence is:

1. `W010` - shell mockup scaffold and design proof in FrankenTui
2. `W020` - runtime shell foundation on top of the proven mockup
3. `W030` - project/document/OxVba service integration into the new shell

## 4. Active Worksets

1. [W010_shell_mockup_scaffold.md](/C:/Work/DnaCalc/OxIde/docs/worksets/W010_shell_mockup_scaffold.md)
2. [W020_runtime_shell_foundation.md](/C:/Work/DnaCalc/OxIde/docs/worksets/W020_runtime_shell_foundation.md)
3. [W030_service_integration.md](/C:/Work/DnaCalc/OxIde/docs/worksets/W030_service_integration.md)
## 5. Use Rule
Use this document as:
1. the repo-local workset authority,
2. the place that records whether active worksets exist,
3. the starting point for the current execution sequence.

Do not use this document as:
1. a blocker tracker,
2. a second status board,
3. a replacement for `.beads/`.
