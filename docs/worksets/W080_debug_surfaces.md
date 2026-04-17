# W080 - Debug Surfaces

Status: `planned`
Sequence: `8`
Depends on: `W070`

## 1. Purpose
Extend the shell with a first-class debug experience: a debug scene with
callstack, locals, watches, and breakpoints, plus editor-gutter breakpoint
control and step/continue semantics over the OxVba runtime.

## 2. Governing Truth
1. [PRODUCT_DIRECTION.md](/C:/Work/DnaCalc/OxIde/PRODUCT_DIRECTION.md)
2. [ARCHITECTURE.md](/C:/Work/DnaCalc/OxIde/ARCHITECTURE.md)
3. [DESIGN_TUI.md](/C:/Work/DnaCalc/OxIde/docs/DESIGN_TUI.md)
4. [docs/uxpass/30_scene_catalogue.md](/C:/Work/DnaCalc/OxIde/docs/uxpass/30_scene_catalogue.md)

## 3. Intended Execution Lanes
1. debug scene and layout preset (distinct scene, not overlay, per uxpass §30)
2. editor-gutter breakpoint toggling persisted per document session
3. step-in / step-over / step-out / continue wired to the OxVba runtime (likely requires a `WebHostCommand` extension upstream in OxVba)
4. exception / halt handling with a "go to offending line" affordance

## 4. Rollout Intention
This workset depends on W070 run surfaces being wired, and on the uxpass
scene catalogue having ruled on debug-as-scene-vs-overlay. Upstream
coordination with OxVba on the runtime debug contract should start as a
preparatory bead.

## 5. Closure Condition
This workset closes when the WTD debug scenario can place a breakpoint,
launch a run, hit the breakpoint, step through statements, and capture the
suspended debug layout against a committed golden.
