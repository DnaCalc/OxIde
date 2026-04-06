# W060 - Full Language-Service UX

Status: `planned`
Sequence: `6`
Depends on: `W050`

## 1. Purpose
Expose the full OxVba language-service surface inside the shell so diagnostics,
navigation, completion, and semantic inspection are honest IDE workflows rather
than partial shell projections.

## 2. Governing Truth
1. [PRODUCT_DIRECTION.md](/C:/Work/DnaCalc/OxIde/PRODUCT_DIRECTION.md)
2. [ARCHITECTURE.md](/C:/Work/DnaCalc/OxIde/ARCHITECTURE.md)
3. [DESIGN_TUI.md](/C:/Work/DnaCalc/OxIde/docs/DESIGN_TUI.md)

## 3. Intended Execution Lanes
1. semantic refresh orchestration during active editing
2. completions and editor-adjacent semantic overlays
3. definition, references, symbol search, and semantic navigation actions
4. full diagnostics, hover, and semantic inspector/lower-surface UX

## 4. Rollout Intention
This workset should expand directly into execution beads for semantic refresh,
overlay UX, semantic navigation, and diagnostics/hover presentation.

## 5. Closure Condition
This workset closes when OxIde can perform expected language-intelligence
workflows end-to-end against OxVba services with no heuristic fallback
pretending to be the language service.
