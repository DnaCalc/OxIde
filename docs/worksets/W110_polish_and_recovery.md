# W110 - Polish, Accessibility, And Recovery

Status: `planned`
Sequence: `11`
Depends on: `W100`

## 1. Purpose
Close the gap between "the IDE works" and "the IDE feels finished" by
sweeping error recovery, empty-state affordances, mouse parity, and
animation discipline, and by locking the WTD regression suite as the stable
definition of shell correctness.

## 2. Governing Truth
1. [PRODUCT_DIRECTION.md](/C:/Work/DnaCalc/OxIde/PRODUCT_DIRECTION.md)
2. [ARCHITECTURE.md](/C:/Work/DnaCalc/OxIde/ARCHITECTURE.md)
3. [docs/uxpass/50_visual_language.md](/C:/Work/DnaCalc/OxIde/docs/uxpass/50_visual_language.md)
4. [docs/TESTING_WTD.md](/C:/Work/DnaCalc/OxIde/docs/TESTING_WTD.md)

## 3. Intended Execution Lanes
1. error recovery: missing project file, corrupt `.basproj`, OxVba crash surfaces surface as recoverable state (no panic)
2. empty-state welcome with recent projects, new-project wizard, capability shortcut, tour command
3. mouse parity sweep: hover reveals, focus clicks, selection drag, scroll, per uxpass §30 decisions
4. animation and motion audit against the visual language doc, trimming anything that violates the density principle

## 4. Rollout Intention
This workset is deliberately cross-cutting. Each lane is a small bead that
touches one or two files. The workset acts as the final pass before the
shell is considered feature-complete against the current uxpass revision.

## 5. Closure Condition
This workset closes when the full WTD regression suite (all scenarios from
W037 onward) passes cleanly, when there are no known panic paths from
user-facing inputs, and when every scene has a mouse affordance consistent
with uxpass §30.
