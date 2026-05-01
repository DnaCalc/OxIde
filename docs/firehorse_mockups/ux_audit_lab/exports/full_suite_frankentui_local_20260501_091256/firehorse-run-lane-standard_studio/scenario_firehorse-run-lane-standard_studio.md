# Run Lane progress review - firehorse-run-lane-standard

Suite: `firehorse`  
Viewport: `studio`  
Gate: `Concern`

## Persona

- `pricing_maintainer`: Excel/VBA maintainer responsible for pricing logic.
- Pressure: Needs fast confidence while editing and running business-critical code.
- Delight target: The IDE feels dense, fast, and calm enough to trust during pricing changes.

## Journey

### primary - Primary audit posture

Intent: Run pricing code and know exactly which stage is active.

Expected surfaces:
- Run Timeline -> `activity_deck.rows` -> staged run events -> W070
- Code Canvas -> `code_canvas` -> source continuity during run -> W050
- Run Context -> `context_dock.cards` -> active run status -> W070

Actions: `run.stop`, `immediate.focus`, `scene.return_edit`

Seams:
- WebHostEvent -> Real -> W070

## Functional Scorecard

- `functional.persona_fit` -> Pass: persona_fit checked against audit fixture audit-run-lane-progress
- `functional.journey_fit` -> Pass: journey_fit checked against audit fixture audit-run-lane-progress
- `functional.command_clarity` -> Pass: command_clarity checked against audit fixture audit-run-lane-progress
- `functional.state_ownership` -> Pass: state_ownership checked against audit fixture audit-run-lane-progress
- `functional.seam_honesty` -> Pass: seam_honesty checked against audit fixture audit-run-lane-progress
- `functional.degradation` -> Pass: degradation checked against audit fixture audit-run-lane-progress

## Aesthetic Scorecard

- `aesthetic.hierarchy` -> Pass: required surfaces are present and grouped by the Fire Horse layout contract
- `aesthetic.density` -> Pass: render has enough dense rows for high-end review
- `aesthetic.balance` -> Pass: structured aesthetic preflight produced no blocker
- `aesthetic.tone_color` -> Pass: ANSI stream contains terminal styling
- `aesthetic.terminal_craft` -> Pass: structured aesthetic preflight produced no blocker
- `aesthetic.reference_fidelity` -> Pass: reference mockup and terminal capture are both linked
- `aesthetic.text_fit` -> Pass: rendered lines stay within fixed viewport width
- `aesthetic.emotional_fit` -> Concern: emotional fit requires cited human or agent rationale against the reference mockup

## Local Findings

No local findings were marked in this export.

## Reproduction

- `target/release/oxide-uxlab.exe --suite firehorse --scenario firehorse-run-lane-standard --viewport studio --once --mockup`
- `target/release/oxide-uxlab.exe --suite firehorse --scenario firehorse-run-lane-standard --viewport studio --once --mockup --ansi`

## Artifacts

- image | Run Lane mockup | `docs/firehorse_mockups/refined_04_run_lane.png`
- terminal_capture | Run Lane Studio capture | `docs/firehorse_mockups/frankentui_terminal_review/captures/run_lane_studio.txt`
