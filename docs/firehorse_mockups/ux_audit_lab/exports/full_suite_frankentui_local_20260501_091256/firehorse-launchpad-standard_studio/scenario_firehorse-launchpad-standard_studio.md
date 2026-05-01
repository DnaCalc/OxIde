# Launchpad cold-start confidence - firehorse-launchpad-standard

Suite: `firehorse`  
Viewport: `studio`  
Gate: `Concern`

## Persona

- `terminal_power_user`: Developer choosing OxIde because terminal workflow is faster.
- Pressure: Needs dense, precise, low-friction command and viewport behavior.
- Delight target: The TUI feels more efficient and emotionally sharper than a comparable GUI.

## Journey

### primary - Primary audit posture

Intent: Start or reopen work from a capable high-end terminal posture.

Expected surfaces:
- Identity Rail -> `identity` -> workspace and readiness posture -> W110
- Start Context -> `context_dock` -> honest no-project state -> W040
- Key Rail -> `key_rail.hints` -> start commands with action ids -> W090

Actions: `project.open`, `project.create`, `app.console_fit`

Seams:
- ProjectSession -> Unavailable -> W040

## Functional Scorecard

- `functional.persona_fit` -> Pass: persona_fit checked against audit fixture audit-launchpad-cold-start
- `functional.journey_fit` -> Pass: journey_fit checked against audit fixture audit-launchpad-cold-start
- `functional.command_clarity` -> Pass: command_clarity checked against audit fixture audit-launchpad-cold-start
- `functional.state_ownership` -> Pass: state_ownership checked against audit fixture audit-launchpad-cold-start
- `functional.seam_honesty` -> Pass: seam_honesty checked against audit fixture audit-launchpad-cold-start
- `functional.degradation` -> Pass: degradation checked against audit fixture audit-launchpad-cold-start

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

- `target/release/oxide-uxlab.exe --suite firehorse --scenario firehorse-launchpad-standard --viewport studio --once --mockup`
- `target/release/oxide-uxlab.exe --suite firehorse --scenario firehorse-launchpad-standard --viewport studio --once --mockup --ansi`

## Artifacts

- image | Launchpad mockup | `docs/firehorse_mockups/refined_01_launchpad.png`
- terminal_capture | Launchpad Studio capture | `docs/firehorse_mockups/frankentui_terminal_review/captures/launchpad_studio.txt`
