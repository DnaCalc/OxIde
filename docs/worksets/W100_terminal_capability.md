# W100 - Terminal Capability And Onboarding

Status: `planned`
Sequence: `10`
Depends on: `W035`

## 1. Purpose
Treat terminal capability as a product-visible concern: probe what the host
terminal can do, communicate findings, guide the user to remediate, and
degrade rendering honestly when capabilities fall short of the instrument
theme.

## 2. Governing Truth
1. [PRODUCT_DIRECTION.md](/C:/Work/DnaCalc/OxIde/PRODUCT_DIRECTION.md)
2. [ARCHITECTURE.md](/C:/Work/DnaCalc/OxIde/ARCHITECTURE.md)
3. [docs/uxpass/50_visual_language.md](/C:/Work/DnaCalc/OxIde/docs/uxpass/50_visual_language.md)

## 3. Intended Execution Lanes
1. first-run capability probe page: truecolor, mouse, Unicode width, size, VT features, with guidance per deficiency (leans on `ftui-core::capabilities`)
2. degradation path: ASCII borders, 16-color palette, reduced decoration when capabilities are insufficient, via FrankenTui's degradation budget hooks
3. status-line actionable hints (always present, always honest about the next available keystroke)
4. light / dark palette toggle with auto-detect where possible

## 4. Rollout Intention
This workset depends on W035 having ruled on the onboarding surface
(first-run page vs status-line hint vs palette command) and on the visual
language doc setting degradation rules. Probe page and degradation path
land as paired beads so the probe can recommend the degradation it will
apply.

## 5. Closure Condition
This workset closes when WTD captures of the probe page under at least
three simulated capability levels match committed goldens, and when the
status line always surfaces an actionable next keystroke in every scene.
