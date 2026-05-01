# Real Editing adapter honesty - firehorse-real-editing

Suite: `firehorse`  
Viewport: `studio`  
Gate: `Concern`

## Persona

- `migration_reviewer`: Developer reviewing inherited VBA during modernization.
- Pressure: Needs meaning, provenance, and navigation without breaking host truth.
- Delight target: The interface is honest about current and future seam ownership.

## Journey

### primary - Primary audit posture

Intent: Review real thin-slice state without mistaking proof data for semantics.

Expected surfaces:
- Project Spine -> `project_spine.rows` -> real mounted workspace rows -> W040
- Code Canvas -> `code_canvas.lines` -> active buffer from shell state -> W050
- Context Dock -> `context_dock.cards.Unavailable` -> missing seams stated explicitly -> W060

Actions: `editor.save`, `semantic.hover`, `run.start`

Seams:
- DocumentSession -> Future -> W050
- HostWorkspaceSession::diagnostics -> Unavailable -> W060

## Functional Scorecard

- `functional.persona_fit` -> Pass: persona_fit checked against audit fixture audit-real-editing-adapter-honesty
- `functional.journey_fit` -> Pass: journey_fit checked against audit fixture audit-real-editing-adapter-honesty
- `functional.command_clarity` -> Pass: command_clarity checked against audit fixture audit-real-editing-adapter-honesty
- `functional.state_ownership` -> Pass: state_ownership checked against audit fixture audit-real-editing-adapter-honesty
- `functional.seam_honesty` -> Pass: seam_honesty checked against audit fixture audit-real-editing-adapter-honesty
- `functional.degradation` -> Pass: degradation checked against audit fixture audit-real-editing-adapter-honesty

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

- `target/release/oxide-uxlab.exe --suite firehorse --scenario firehorse-real-editing --viewport studio --once --mockup`
- `target/release/oxide-uxlab.exe --suite firehorse --scenario firehorse-real-editing --viewport studio --once --mockup --ansi`

## Artifacts

- image | Editing Lens mockup reference | `docs/firehorse_mockups/refined_02_editing_lens.png`
- terminal_capture | Real Editing Adapter Studio capture | `docs/firehorse_mockups/frankentui_terminal_review/captures/real_editing_adapter_studio.txt`
