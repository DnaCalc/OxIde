# W020 - Runtime Shell Foundation

Status: `stub`
Sequence: `2`
Depends on: `W010`

## 1. Purpose
Replace mock data and purely demonstrative state with the real shell runtime
foundation that OxIde will build on.

## 2. Governing Truth
1. [PRODUCT_DIRECTION.md](/C:/Work/DnaCalc/OxIde/PRODUCT_DIRECTION.md)
2. [ARCHITECTURE.md](/C:/Work/DnaCalc/OxIde/ARCHITECTURE.md)
3. [DESIGN_TUI.md](/C:/Work/DnaCalc/OxIde/docs/DESIGN_TUI.md)

## 3. Intended Execution Lanes
1. buffer / view / layout state model
2. region-mode routing and focus control
3. shell-owned layout policy and session state
4. editor-surface attachment behind OxIde-owned seams

## 4. Rollout Intention
This workset should begin with an explicit rollout bead once `W010` proves the
mockup shell.

## 5. Closure Condition
This workset closes when the shell is no longer just a mockup and the main
runtime state model is in place behind the proven TUI structure.
