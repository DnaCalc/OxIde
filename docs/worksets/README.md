# OxIde Worksets

This folder is the compact active map for workset-level OxIde planning and
provenance.

It is not a live execution tracker.
For ordered workset truth use [WORKSET_REGISTER.md](/C:/Work/DnaCalc/OxIde/docs/WORKSET_REGISTER.md).
For live execution state use [.beads/issues.jsonl](/C:/Work/DnaCalc/OxIde/.beads/issues.jsonl) through `br`.

## Current Rules
1. Worksets are planning and provenance packets, not the owner of ready or blocked state.
2. `.beads/` owns live execution truth.
3. Worksets explain scope, dependency order, and rollout intent.
4. Closed bootstrap/provenance worksets may remain listed here for context even
   when active execution has moved on.

## Active Workset Set
1. [W001_EXECUTION_DOCTRINE_AND_REWRITE_BOOTSTRAP.md](/C:/Work/DnaCalc/OxIde/docs/worksets/W001_EXECUTION_DOCTRINE_AND_REWRITE_BOOTSTRAP.md)
2. [W010_IMPLEMENTATION_RESET_AND_SALVAGE_TRIAGE.md](/C:/Work/DnaCalc/OxIde/docs/worksets/W010_IMPLEMENTATION_RESET_AND_SALVAGE_TRIAGE.md)
3. [W020_SHELL_AND_ACTION_SYSTEM_REBUILD.md](/C:/Work/DnaCalc/OxIde/docs/worksets/W020_SHELL_AND_ACTION_SYSTEM_REBUILD.md)
4. [W030_BUFFER_VIEW_LAYOUT_SESSION_MODEL.md](/C:/Work/DnaCalc/OxIde/docs/worksets/W030_BUFFER_VIEW_LAYOUT_SESSION_MODEL.md)
5. [W040_EDITOR_SURFACE_AND_UNDO_FOUNDATION.md](/C:/Work/DnaCalc/OxIde/docs/worksets/W040_EDITOR_SURFACE_AND_UNDO_FOUNDATION.md)
6. [W050_FILE_WORKSPACE_AND_PROJECT_MANAGEMENT_SURFACES.md](/C:/Work/DnaCalc/OxIde/docs/worksets/W050_FILE_WORKSPACE_AND_PROJECT_MANAGEMENT_SURFACES.md)
7. [W060_DIRECT_OXVBA_SEMANTIC_EDITING_INTEGRATION.md](/C:/Work/DnaCalc/OxIde/docs/worksets/W060_DIRECT_OXVBA_SEMANTIC_EDITING_INTEGRATION.md)
8. [W070_EMPTY_STATE_CONSOLE_SETUP_AND_FIRST_RUN.md](/C:/Work/DnaCalc/OxIde/docs/worksets/W070_EMPTY_STATE_CONSOLE_SETUP_AND_FIRST_RUN.md)

## Use These Instead
1. Use [PRODUCT_DIRECTION.md](/C:/Work/DnaCalc/OxIde/PRODUCT_DIRECTION.md) for product and UX authority.
2. Use [ARCHITECTURE.md](/C:/Work/DnaCalc/OxIde/ARCHITECTURE.md) for seam and implementation direction.
3. Use [WORKSET_REGISTER.md](/C:/Work/DnaCalc/OxIde/docs/WORKSET_REGISTER.md) for ordered workset truth.
4. Use [BEADS.md](/C:/Work/DnaCalc/OxIde/docs/BEADS.md) for the local bead method.
