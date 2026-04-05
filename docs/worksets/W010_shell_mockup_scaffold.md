# W010 - Shell Mockup Scaffold

Status: `implemented`
Sequence: `1`
Depends on: none

## 1. Purpose
Prove the OxIde TUI shell in its real medium before wiring real project or
semantic integration.

This workset turns [DESIGN_TUI.md](/C:/Work/DnaCalc/OxIde/docs/DESIGN_TUI.md)
into a FrankenTui shell mockup that can be compared directly against the web
design study.

## 2. Governing Truth
1. [PRODUCT_DIRECTION.md](/C:/Work/DnaCalc/OxIde/PRODUCT_DIRECTION.md)
2. [ARCHITECTURE.md](/C:/Work/DnaCalc/OxIde/ARCHITECTURE.md)
3. [DESIGN_TUI.md](/C:/Work/DnaCalc/OxIde/docs/DESIGN_TUI.md)
4. [OPERATIONS.md](/C:/Work/DnaCalc/OxIde/OPERATIONS.md)
5. [WORKSET_REGISTER.md](/C:/Work/DnaCalc/OxIde/docs/WORKSET_REGISTER.md)

## 3. Execution Lanes
1. shell scaffold and module split
2. persistent shell regions and fake-data state rendering
3. region modes, overlays, and theme tokens
4. width adaptation and focus styling
5. comparison and verification against the web mockup

## 4. Intended Rollout
This workset should roll out into:
1. one epic for the shell-mockup lane,
2. direct execution beads for the scaffold and mockup steps,
3. a verification bead that confirms the terminal shell still matches the
   intended design.

## 5. Closure Condition
This workset closes when:
1. a FrankenTui shell mockup renders the five spec states from fake data,
2. the shell frame, region ownership, and width adaptation are visible in code,
3. the result is strong enough to compare meaningfully with the web mockup,
4. follow-on runtime work is ready to proceed from the proven shell shape.
