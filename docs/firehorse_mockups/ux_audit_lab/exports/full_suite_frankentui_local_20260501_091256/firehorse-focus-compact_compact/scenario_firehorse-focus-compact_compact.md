# Compact Focus degradation review - firehorse-focus-compact

Suite: `firehorse`  
Viewport: `compact`  
Gate: `Concern`

## Persona

- `terminal_power_user`: Developer choosing OxIde because terminal workflow is faster.
- Pressure: Needs dense, precise, low-friction command and viewport behavior.
- Delight target: The TUI feels more efficient and emotionally sharper than a comparable GUI.

## Journey

### primary - Primary audit posture

Intent: Work in compact source-first mode with explicit side-surface affordances.

Expected surfaces:
- Code Canvas -> `code_canvas` -> source-first compact posture -> W050
- Key Rail -> `key_rail.hints` -> dock affordances in compact mode -> W100

Actions: `focus.project`, `focus.code`, `focus.context`, `focus.activity`

Seams:
- LayoutPolicy -> Future -> W100

## Functional Scorecard

- `functional.persona_fit` -> Pass: persona_fit checked against audit fixture audit-compact-focus-degradation
- `functional.journey_fit` -> Pass: journey_fit checked against audit fixture audit-compact-focus-degradation
- `functional.command_clarity` -> Pass: command_clarity checked against audit fixture audit-compact-focus-degradation
- `functional.state_ownership` -> Pass: state_ownership checked against audit fixture audit-compact-focus-degradation
- `functional.seam_honesty` -> Pass: seam_honesty checked against audit fixture audit-compact-focus-degradation
- `functional.degradation` -> Pass: degradation checked against audit fixture audit-compact-focus-degradation

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

- `target/release/oxide-uxlab.exe --suite firehorse --scenario firehorse-focus-compact --viewport compact --once --mockup`
- `target/release/oxide-uxlab.exe --suite firehorse --scenario firehorse-focus-compact --viewport compact --once --mockup --ansi`

## Artifacts

- image | Compact Focus mockup | `docs/firehorse_mockups/refined_07_compact_focus_mode.png`
- terminal_capture | Compact Focus compact capture | `docs/firehorse_mockups/frankentui_terminal_review/captures/compact_focus_default_compact.txt`
