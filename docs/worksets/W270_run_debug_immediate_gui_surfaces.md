# Workset W270 — Run, Debug, And Immediate GUI Surfaces

## Ambition

Run, Immediate, and debug surfaces become coherent GUI IDE workflows over real OxVba seams or explicit future/unavailable seams.

W270 should improve the GUI IDE surface without faking runtime/debug truth. If OxVba APIs are not ready, the GUI must show precise unavailable or native-runtime-required states.

## Dependencies

- W240 — capability-aware run/output path.
- W260 — COM capability proof where COM-sensitive runtime state matters.
- [`docs/HANDOFF_W270_RUN_DEBUG_IMMEDIATE_GUI_SURFACES.md`](../HANDOFF_W270_RUN_DEBUG_IMMEDIATE_GUI_SURFACES.md).
- OxVba runtime, Immediate, and debug session APIs or explicit handoffs.

## Design

### Runtime truth policy

W270 must preserve:

1. W240 simulated run is simulated-only evidence.
2. Browser-safe run remains disabled.
3. Browser and non-Windows COM remain unavailable.
4. Windows-native COM remains service-missing until a tested native service exists.
5. OxVba owns runtime, debug, and Immediate semantics.

The GUI may render:

1. structured run timeline from existing `RunTranscript` events,
2. Immediate panel unavailable/browser-disabled state,
3. Immediate panel native-runtime-required state,
4. debug surface unavailable/browser-disabled state,
5. debug surface native-runtime-required or future seam state.

The GUI must not render:

1. fake callstack frames,
2. fake locals/watches,
3. fake breakpoints as bound to runtime,
4. COM-capable runtime state while W260 says native COM service is missing.

### Planned scenarios

W270 should add deterministic lab scenarios:

```text
gui-run-timeline-simulated
gui-immediate-browser-disabled
gui-debug-browser-disabled
```

Optional later W270 scenarios may add native-runtime-required states if the pure capability model is ready without claiming real runtime support.

## Beads

### W270-B00 — Expand run/debug/Immediate workset

Goal:
  Make W270 executable by replacing the scaffold with concrete vertical beads and explicit unavailable-seam policy.

Design:
  - Record runtime truth guardrails.
  - Name first lab scenarios.
  - Preserve W240/W260 capability constraints.

Tests:
  - Documentation review against product/architecture/W270 handoff docs.

Evidence:
  - Expanded `docs/worksets/W270_run_debug_immediate_gui_surfaces.md`.

Closure:
  - [ ] W270 has concrete beads.
  - [ ] Immediate/debug unavailable seam policy is explicit.
  - [ ] COM/native limitations remain explicit.

### W270-B01 — Run timeline view model refinement

Goal:
  Add a pure run timeline projection over existing structured run transcripts.

Design:
  - Add `RunTimeline`, `RunTimelineEntry`, and timeline status/provenance labels in `oxide-core`.
  - Build timelines from existing `RunTranscript` without reinterpreting OxVba runtime semantics.
  - Preserve simulated-provider and browser-disabled distinctions.

Tests:
  - Simulated transcript produces ordered timeline entries.
  - Browser-disabled transcript produces requested + diagnostic entries.
  - Timeline keeps provider/status/native/COM flags visible.

Evidence:
  - `cargo test --manifest-path crates/Cargo.toml -p oxide-core`.

Closure:
  - [ ] Timeline projection is pure.
  - [ ] Simulated remains labeled simulated.
  - [ ] Browser-disabled remains disabled.

### W270-B02 — Immediate and debug capability models

Goal:
  Add pure capability state for Immediate and debug surfaces that can honestly render unavailable/future seams.

Design:
  - Add `ImmediateCapabilityProfile` and `DebugCapabilityProfile` in `oxide-core`.
  - Model browser-disabled, native-runtime-required, and future-supported labels.
  - Do not model fake Immediate responses, callstacks, locals, watches, or breakpoints.

Tests:
  - Browser Immediate disabled reason mentions native runtime/session unavailable.
  - Browser debug disabled reason mentions debug session unavailable.
  - Native-runtime-required profiles are distinct from future-supported.
  - Future-supported profiles do not imply COM availability.

Evidence:
  - `cargo test --manifest-path crates/Cargo.toml -p oxide-core`.

Closure:
  - [ ] Immediate/debug availability is explicit.
  - [ ] No fake debug truth is introduced.
  - [ ] COM availability is not implied.

### W270-B03 — Run timeline and Immediate/debug unavailable lab scenarios

Goal:
  Render first W270 GUI-lab evidence for run timeline plus unavailable Immediate/debug surfaces.

Design:
  - Add `gui-run-timeline-simulated` over W240 simulated transcript.
  - Add `gui-immediate-browser-disabled`.
  - Add `gui-debug-browser-disabled`.
  - Reuse thin-slice project spine and capability footer.
  - Show source continuity and disabled reasons.

Tests:
  - Scenario registry finds the new IDs.
  - Rendered run timeline includes simulated provider, ordered events, and answer 42 output.
  - Immediate/debug scenarios show disabled commands and no fake runtime/debug data.
  - W210-W260 scenarios remain green.

Evidence:
  - `cargo test --manifest-path crates/Cargo.toml -p oxide-guilab`.
  - Render commands for all new W270 scenarios.

Closure:
  - [ ] Run timeline scenario is registered.
  - [ ] Immediate disabled scenario is registered.
  - [ ] Debug disabled scenario is registered.
  - [ ] No fake runtime/debug data is rendered.

### W270-B04 — OxVba runtime/debug/Immediate interface handoff

Goal:
  Capture any required OxVba runtime/debug/Immediate interfaces instead of duplicating runtime truth in OxIde.

Design:
  - Document needed runtime session identity, target enumeration, Immediate request/response/events, debug command availability, callstack/locals/watches, breakpoint binding, and runtime error taxonomy.
  - Reference W260 COM service constraints.
  - State what OxIde can render from pure capability state today.

Tests:
  - Documentation review.
  - Full nested workspace tests remain green.

Evidence:
  - Handoff note.
  - `cargo test --manifest-path crates/Cargo.toml --workspace`.

Closure:
  - [ ] Required OxVba interfaces are specific.
  - [ ] OxIde limitations are explicit.
  - [ ] No local duplicate runtime/debug model is introduced.

### W270-B05 — W270 acceptance and W280 handoff

Goal:
  Accept W270 with regression evidence and prepare W280 command/keyboard/accessibility polish.

Design:
  - Update GUI fixture/lab docs with W270 scenarios and expected tokens.
  - Update this workset with acceptance evidence.
  - Add W280 handoff note for command, keyboard, focus, and accessibility polish.

Tests:
  - `cargo test --manifest-path crates/Cargo.toml --workspace`.
  - Render W210-W270 GUI-lab scenarios.

Evidence:
  - Test output and rendered scenario token checks.
  - W280 handoff note.

Closure:
  - [ ] W270 accepted or explicitly blocked with evidence.
  - [ ] W280 prerequisites documented.
  - [ ] Runtime/debug/Immediate limitations remain explicit.

## Out-of-scope

- Full debugger parity if OxVba seams are not ready.
- Fake debug truth.
- Real COM-capable runtime execution without a tested native service.
- General telemetry.
- Command/keybinding/accessibility polish; this belongs to W280.
