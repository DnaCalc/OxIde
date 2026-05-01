# Debug Cockpit paused-state review - firehorse-debug-cockpit-standard

Suite: `firehorse`  
Viewport: `studio`  
Gate: `Concern`

## Persona

- `debug_responder`: User diagnosing a live macro failure.
- Pressure: Needs paused-state clarity, stack/local visibility, and safe recovery.
- Delight target: Paused debug state feels controlled, recoverable, and precise.

## Journey

### primary - Primary audit posture

Intent: Diagnose a paused macro failure and choose the next debug action.

Expected surfaces:
- Code Canvas -> `code_canvas.execution_line` -> paused execution line -> W080
- Debug Context -> `context_dock.cards` -> call stack, locals, watches -> W080
- Activity Deck -> `activity_deck` -> Immediate and watch trace surfaces -> W080

Actions: `debug.continue`, `debug.step`, `debug.step_out`, `immediate.focus`

Seams:
- W080 debug contract audit -> Future -> W080

## Functional Scorecard

- `functional.persona_fit` -> Pass: persona_fit checked against audit fixture audit-debug-cockpit-paused
- `functional.journey_fit` -> Pass: journey_fit checked against audit fixture audit-debug-cockpit-paused
- `functional.command_clarity` -> Pass: command_clarity checked against audit fixture audit-debug-cockpit-paused
- `functional.state_ownership` -> Pass: state_ownership checked against audit fixture audit-debug-cockpit-paused
- `functional.seam_honesty` -> Pass: seam_honesty checked against audit fixture audit-debug-cockpit-paused
- `functional.degradation` -> Pass: degradation checked against audit fixture audit-debug-cockpit-paused

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

- `target/release/oxide-uxlab.exe --suite firehorse --scenario firehorse-debug-cockpit-standard --viewport studio --once --mockup`
- `target/release/oxide-uxlab.exe --suite firehorse --scenario firehorse-debug-cockpit-standard --viewport studio --once --mockup --ansi`

## Artifacts

- image | Debug Cockpit mockup | `docs/firehorse_mockups/refined_05_debug_cockpit.png`
- terminal_capture | Debug Cockpit Studio capture | `docs/firehorse_mockups/frankentui_terminal_review/captures/debug_cockpit_studio.txt`
