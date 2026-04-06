# W070 - Run Debug Immediate Surfaces

Status: `planned`
Sequence: `7`
Depends on: `W060`

## 1. Purpose
Make execution a first-class workspace state by presenting typed run, debug,
and immediate/evaluation surfaces over OxVba-owned execution contracts.

## 2. Governing Truth
1. [PRODUCT_DIRECTION.md](/C:/Work/DnaCalc/OxIde/PRODUCT_DIRECTION.md)
2. [ARCHITECTURE.md](/C:/Work/DnaCalc/OxIde/ARCHITECTURE.md)
3. [DESIGN_TUI.md](/C:/Work/DnaCalc/OxIde/docs/DESIGN_TUI.md)

## 3. Intended Execution Lanes
1. run/build shell state and output surfaces
2. debug layout, focus policy, and execution-state routing
3. watches, locals, stack, and debug-console presentation
4. immediate evaluation panel over OxVba evaluation helpers

## 4. Rollout Intention
This workset should begin with a rollout bead to inventory the exact OxVba
run/debug/evaluation contracts that OxIde can host directly before the child
execution beads are fixed.

## 5. Closure Condition
This workset closes when edit, run, and debug feel like explicit workspace
states in OxIde and the immediate/debug surfaces are hosted through the
intended OxVba execution seams.
