# Console Fit capability review - firehorse-console-fit-light

Suite: `firehorse`  
Viewport: `studio`  
Gate: `Concern`

## Persona

- `terminal_power_user`: Developer choosing OxIde because terminal workflow is faster.
- Pressure: Needs dense, precise, low-friction command and viewport behavior.
- Delight target: The TUI feels more efficient and emotionally sharper than a comparable GUI.

## Journey

### primary - Primary audit posture

Intent: Check terminal fit and understand any visual capability constraints.

Expected surfaces:
- Terminal Fit -> `terminal_fit.rows` -> capability results and recommendations -> W100
- Key Rail -> `key_rail.hints` -> rerun/report/return actions -> W090

Actions: `app.console_fit`, `scene.return_edit`

Seams:
- TerminalCapabilityProbe -> Future -> W100

## Functional Scorecard

- `functional.persona_fit` -> Pass: persona_fit checked against audit fixture audit-console-fit-light
- `functional.journey_fit` -> Pass: journey_fit checked against audit fixture audit-console-fit-light
- `functional.command_clarity` -> Pass: command_clarity checked against audit fixture audit-console-fit-light
- `functional.state_ownership` -> Pass: state_ownership checked against audit fixture audit-console-fit-light
- `functional.seam_honesty` -> Pass: seam_honesty checked against audit fixture audit-console-fit-light
- `functional.degradation` -> Pass: degradation checked against audit fixture audit-console-fit-light

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

- `target/release/oxide-uxlab.exe --suite firehorse --scenario firehorse-console-fit-light --viewport studio --once --mockup`
- `target/release/oxide-uxlab.exe --suite firehorse --scenario firehorse-console-fit-light --viewport studio --once --mockup --ansi`

## Artifacts

- image | Console Fit mockup | `docs/firehorse_mockups/refined_06_console_fit.png`
- terminal_capture | Console Fit Studio capture | `docs/firehorse_mockups/frankentui_terminal_review/captures/console_fit_studio.txt`
