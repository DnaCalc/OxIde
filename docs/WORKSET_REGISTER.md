# OxIde Workset Register

Status: `active_register`
Date: 2026-04-02

## 1. Purpose
This is the live ordered workset register for current OxIde execution.

It defines:
1. the current workset set,
2. dependency order,
3. intended rollout shape for the rewrite/salvage phase.

This file is not an execution-status board.
It owns workset truth, not bead state.

## 2. Planning-Surface Clarification
Planning and execution truth in OxIde is split as follows:
1. [PRODUCT_DIRECTION.md](/C:/Work/DnaCalc/OxIde/PRODUCT_DIRECTION.md) owns product direction and UX authority.
2. [ARCHITECTURE.md](/C:/Work/DnaCalc/OxIde/ARCHITECTURE.md) owns seam and implementation-direction authority.
3. [OPERATIONS.md](/C:/Work/DnaCalc/OxIde/OPERATIONS.md) owns the local execution model.
4. this register owns ordered workset truth.
5. `.beads/` owns epics, beads, readiness, blockers, in-progress state, and closure.
6. individual workset docs under `docs/worksets/` are planning/provenance packets, not live execution state.

## 3. Use Rule
Use this document as:
1. the repo-local workset authority,
2. the source for `workset -> epic -> bead` rollout,
3. the ordered implementation map for the OxIde rebuild.

Do not use this document as:
1. a blocker tracker,
2. a second status board,
3. a replacement for `.beads/`.

## 4. Register Contract
Each workset in this register carries:
1. stable workset id,
2. title,
3. purpose,
4. depends_on,
5. parent doctrine/spec surfaces,
6. closure condition,
7. initial epic lanes,
8. rollout mode.

Rollout mode values:
1. `execution_target`
2. `tracking_anchor`

## 5. Active Workset Sequence

### W001 Execution Doctrine And Rewrite Bootstrap
1. purpose:
   establish the OxIde-local workset/beads execution doctrine and seed the rewrite plan as explicit worksets and beads.
2. depends_on:
   none
3. parent_doctrine_and_spec_surfaces:
   `OPERATIONS.md`, `docs/BEADS.md`, `PRODUCT_DIRECTION.md`, `ARCHITECTURE.md`
4. closure_condition:
   doctrine is live, the workset register exists, the rewrite lanes are explicit, and the first execution beads are seeded.
5. initial_epic_lanes:
   doctrine bootstrap, workset register, initial rollout graph
6. rollout_mode:
   `execution_target`

### W010 Implementation Reset And Salvage Triage
1. purpose:
   turn the current single-file spike into an explicit salvage/rewrite decision surface, preserving only real value.
2. depends_on:
   `W001`
3. parent_doctrine_and_spec_surfaces:
   `PRODUCT_DIRECTION.md`, `ARCHITECTURE.md`
4. closure_condition:
   the current code is classified into retain / port / discard buckets, the salvage targets are explicit, and the rewrite boundary is settled.
5. initial_epic_lanes:
   code inventory, test inventory, salvage packet, removal/replacement plan
6. rollout_mode:
   `execution_target`

### W020 Shell And Action System Rebuild
1. purpose:
   rebuild the shell around the authoritative product direction rather than the prototype’s single-buffer and raw-`:` assumptions.
2. depends_on:
   `W001`, `W010`
3. parent_doctrine_and_spec_surfaces:
   `PRODUCT_DIRECTION.md`, `ARCHITECTURE.md`
4. closure_condition:
   a new shell/action model exists with explicit action registry, command-entry model, focus routing, and panel composition foundations.
5. initial_epic_lanes:
   action registry, input routing, shell frame, command surfaces
6. rollout_mode:
   `execution_target`

### W030 Buffer View Layout Session Model
1. purpose:
   implement the buffer/view/layout architecture required by product direction, including non-visible open buffers and multi-view-on-one-buffer support.
2. depends_on:
   `W001`, `W010`, `W020`
3. parent_doctrine_and_spec_surfaces:
   `PRODUCT_DIRECTION.md`, `ARCHITECTURE.md`
4. closure_condition:
   buffers, views, and layouts are explicit first-class objects and are no longer collapsed into one active editor/document path.
5. initial_epic_lanes:
   buffer model, view model, layout model, session restore hooks
6. rollout_mode:
   `execution_target`

### W040 Editor Surface And Undo Foundation
1. purpose:
   establish the FrankenTui-based editor surface, including buffer-local undo/redo and shared edit history across multiple views on the same buffer.
2. depends_on:
   `W001`, `W020`, `W030`
3. parent_doctrine_and_spec_surfaces:
   `PRODUCT_DIRECTION.md`, `ARCHITECTURE.md`
4. closure_condition:
   editor integration is aligned with the new shell/session model and undo ownership is correct by architecture, not just by widget accident.
5. initial_epic_lanes:
   editor adapter, undo/redo model, cursor/viewport model, multi-view editor behavior
6. rollout_mode:
   `execution_target`

### W050 File Workspace And Project Management Surfaces
1. purpose:
   rebuild file handling, workspace/project management, and project-oriented UX on top of the new shell/session model.
2. depends_on:
   `W001`, `W020`, `W030`
3. parent_doctrine_and_spec_surfaces:
   `PRODUCT_DIRECTION.md`, `ARCHITECTURE.md`
4. closure_condition:
   file lifecycle, workspace loading/reload, project-backed module navigation, and project-management UX exist in the rebuilt shell.
5. initial_epic_lanes:
   file lifecycle, workspace lifecycle, project rail, management commands and surfaces
6. rollout_mode:
   `execution_target`

### W060 Direct OxVba Semantic Editing Integration
1. purpose:
   port and deepen the direct OxVba host-session integration into the rebuilt shell so semantics remain OxVba-owned while editing UX remains OxIde-owned.
2. depends_on:
   `W001`, `W010`, `W020`, `W030`, `W040`, `W050`
3. parent_doctrine_and_spec_surfaces:
   `PRODUCT_DIRECTION.md`, `ARCHITECTURE.md`
4. closure_condition:
   the rebuilt shell drives diagnostics, symbols, hover, completions, and related semantic editing flows through the direct host session rather than the prototype shell.
5. initial_epic_lanes:
   host-session adapter port, diagnostics UX, symbol/hover/completion UX, semantic test migration
6. rollout_mode:
   `execution_target`

### W070 Empty State Console Setup And First Run
1. purpose:
   deliver the early-use surfaces already identified as strategically important: welcome/empty state and console capability/setup guidance.
2. depends_on:
   `W001`, `W020`, `W030`
3. parent_doctrine_and_spec_surfaces:
   `PRODUCT_DIRECTION.md`
4. closure_condition:
   OxIde has a designed empty state and a first-class console diagnostics/setup surface in the rebuilt shell.
5. initial_epic_lanes:
   empty state, console test surface, Windows guidance, setup validation flows
6. rollout_mode:
   `execution_target`
