# Command Lens run selection - firehorse-command-lens-standard

Suite: `firehorse`  
Viewport: `first-class`  
Gate: `Concern`

## Persona

- `terminal_power_user`: Developer choosing OxIde because terminal workflow is faster.
- Pressure: Needs dense, precise, low-friction command and viewport behavior.
- Delight target: The TUI feels more efficient and emotionally sharper than a comparable GUI.

## Journey

### primary - Primary audit posture

Intent: Filter to run commands and understand what can execute.

Expected surfaces:
- Overlay -> `overlay.CommandLens` -> filter, rows, preview, disabled reasons -> W090
- Code Canvas -> `code_canvas` -> backing source remains legible -> W050
- Key Rail -> `key_rail.hints` -> overlay-specific actions -> W090

Actions: `run.start`, `run.stop`, `target.configure`

Seams:
- ActionRegistry -> Future -> W090

## Functional Scorecard

- `functional.persona_fit` -> Pass: persona_fit checked against audit fixture audit-command-lens-run
- `functional.journey_fit` -> Pass: journey_fit checked against audit fixture audit-command-lens-run
- `functional.command_clarity` -> Pass: command_clarity checked against audit fixture audit-command-lens-run
- `functional.state_ownership` -> Pass: state_ownership checked against audit fixture audit-command-lens-run
- `functional.seam_honesty` -> Pass: seam_honesty checked against audit fixture audit-command-lens-run
- `functional.degradation` -> Pass: degradation checked against audit fixture audit-command-lens-run

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

- `target/release/oxide-uxlab.exe --suite firehorse --scenario firehorse-command-lens-standard --viewport first-class --once --mockup`
- `target/release/oxide-uxlab.exe --suite firehorse --scenario firehorse-command-lens-standard --viewport first-class --once --mockup --ansi`

## Artifacts

- image | Command Lens mockup | `docs/firehorse_mockups/refined_03_command_lens.png`
- terminal_capture | Command Lens First-class capture | `docs/firehorse_mockups/frankentui_terminal_review/captures/command_lens_first-class.txt`
