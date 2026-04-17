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
4. `W035` - fresh UX design pass, reconciled back into `PRODUCT_DIRECTION.md` and `DESIGN_TUI.md` (see [docs/uxpass/README.md](/C:/Work/DnaCalc/OxIde/docs/uxpass/README.md))
5. `W037` - WinTermDriver test harness foundation (see [docs/TESTING_WTD.md](/C:/Work/DnaCalc/OxIde/docs/TESTING_WTD.md))
6. `W038` - UX development lab (scene flag, scenario driver, VT replay/diff, `oxide-uxlab` TUI)
7. `W040` - project and workspace management shell over OxVba-owned truth
8. `W050` - file and document lifecycle services in the TUI shell
9. `W060` - full language-service UX over direct OxVba semantics
10. `W070` - run/debug/immediate shell surfaces over OxVba execution contracts
11. `W080` - debug surfaces (callstack / locals / watches / breakpoints, step control)
12. `W090` - command system and keymap profiles (unified action registry, VBA-IDE-compatible profile)
13. `W100` - terminal capability and onboarding (probe, degradation, status-line hints)
14. `W110` - polish, accessibility, and recovery; WTD regression suite locked

## 4. Active Worksets

1. [W010_shell_mockup_scaffold.md](/C:/Work/DnaCalc/OxIde/docs/worksets/W010_shell_mockup_scaffold.md)
2. [W020_runtime_shell_foundation.md](/C:/Work/DnaCalc/OxIde/docs/worksets/W020_runtime_shell_foundation.md)
3. [W030_service_integration.md](/C:/Work/DnaCalc/OxIde/docs/worksets/W030_service_integration.md)
4. [W035_ux_design_pass.md](/C:/Work/DnaCalc/OxIde/docs/worksets/W035_ux_design_pass.md)
5. [W037_wtd_harness.md](/C:/Work/DnaCalc/OxIde/docs/worksets/W037_wtd_harness.md)
6. [W038_ux_development_lab.md](/C:/Work/DnaCalc/OxIde/docs/worksets/W038_ux_development_lab.md)
7. [W040_project_workspace_management.md](/C:/Work/DnaCalc/OxIde/docs/worksets/W040_project_workspace_management.md)
8. [W050_file_document_services.md](/C:/Work/DnaCalc/OxIde/docs/worksets/W050_file_document_services.md)
9. [W060_full_language_service_ux.md](/C:/Work/DnaCalc/OxIde/docs/worksets/W060_full_language_service_ux.md)
10. [W070_run_debug_immediate_surfaces.md](/C:/Work/DnaCalc/OxIde/docs/worksets/W070_run_debug_immediate_surfaces.md)
11. [W080_debug_surfaces.md](/C:/Work/DnaCalc/OxIde/docs/worksets/W080_debug_surfaces.md)
12. [W090_command_system.md](/C:/Work/DnaCalc/OxIde/docs/worksets/W090_command_system.md)
13. [W100_terminal_capability.md](/C:/Work/DnaCalc/OxIde/docs/worksets/W100_terminal_capability.md)
14. [W110_polish_and_recovery.md](/C:/Work/DnaCalc/OxIde/docs/worksets/W110_polish_and_recovery.md)
## 5. Use Rule
Use this document as:
1. the repo-local workset authority,
2. the place that records whether active worksets exist,
3. the starting point for the current execution sequence.

Do not use this document as:
1. a blocker tracker,
2. a second status board,
3. a replacement for `.beads/`.
