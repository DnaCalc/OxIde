# Editing Lens pricing loop - firehorse-editing-lens-standard

Suite: `firehorse`  
Viewport: `studio`  
Gate: `Concern`

## Persona

- `pricing_maintainer`: Excel/VBA maintainer responsible for pricing logic.
- Pressure: Needs fast confidence while editing and running business-critical code.
- Delight target: The IDE feels dense, fast, and calm enough to trust during pricing changes.

## Journey

### primary - Primary audit posture

Intent: Maintain a pricing function with semantic confidence.

Expected surfaces:
- Code Canvas -> `code_canvas` -> source-centered editing and lens -> W050
- Context Dock -> `context_dock.cards` -> diagnostic and symbol cards -> W060
- Activity Deck -> `activity_deck` -> Problems/Output/References task surface -> W060
- Key Rail -> `key_rail.hints` -> save, semantic, run, command actions -> W090

Actions: `editor.save`, `command.lens.open`, `semantic.hover`, `run.start`

Seams:
- HostWorkspaceSession::diagnostics -> Real -> W060
- HostWorkspaceSession::hover -> Real -> W060

## Functional Scorecard

- `functional.persona_fit` -> Pass: persona_fit checked against audit fixture audit-editing-lens-pricing
- `functional.journey_fit` -> Pass: journey_fit checked against audit fixture audit-editing-lens-pricing
- `functional.command_clarity` -> Pass: command_clarity checked against audit fixture audit-editing-lens-pricing
- `functional.state_ownership` -> Pass: state_ownership checked against audit fixture audit-editing-lens-pricing
- `functional.seam_honesty` -> Pass: seam_honesty checked against audit fixture audit-editing-lens-pricing
- `functional.degradation` -> Pass: degradation checked against audit fixture audit-editing-lens-pricing

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

- `target/release/oxide-uxlab.exe --suite firehorse --scenario firehorse-editing-lens-standard --viewport studio --once --mockup`
- `target/release/oxide-uxlab.exe --suite firehorse --scenario firehorse-editing-lens-standard --viewport studio --once --mockup --ansi`

## Artifacts

- image | Editing Lens mockup | `docs/firehorse_mockups/refined_02_editing_lens.png`
- terminal_capture | Editing Lens Studio capture | `docs/firehorse_mockups/frankentui_terminal_review/captures/editing_lens_studio.txt`
