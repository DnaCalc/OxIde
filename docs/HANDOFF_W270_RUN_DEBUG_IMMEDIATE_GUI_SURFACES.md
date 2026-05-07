# Handoff — W270 Run, Debug, And Immediate GUI Surfaces

Status: `handoff_note`
Date: 2026-05-07

## W260 Baseline

W260 closed with these capability scenarios in addition to W210-W250 regressions:

```powershell
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-com-reference-browser-unavailable
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-com-reference-nonwindows-unavailable
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-com-reference-native-service-missing
```

W260 proves capability honesty only. It does not prove real COM discovery or invocation.

## W270 Starting Point

W270 should build run/debug/Immediate GUI surfaces over explicit capability gates:

1. W240 simulated run remains simulated-only evidence.
2. Browser-safe run remains disabled.
3. Browser and non-Windows COM remain unavailable.
4. Windows-native COM remains service-missing until a tested native service exists.
5. OxVba owns runtime, debug, and Immediate semantics.

## Required W270 Guardrails

1. Do not invent fake debug truth.
2. Do not route OxIde internal runtime/debug semantics through LSP.
3. Do not claim COM-capable run/debug/Immediate when W260 says the native COM service is missing.
4. Keep source continuity visible while changing run/debug/Immediate postures.
5. If OxVba Immediate/debug APIs are insufficient, create handoffs rather than local duplicate models.

## Suggested W270 Bead Expansion

1. Run timeline refinement over the existing `RunTranscript` model.
2. Immediate panel unavailable/browser-disabled state.
3. Immediate panel future/native-session-required state.
4. Debug paused-state projection with explicit unavailable/future seam if OxVba APIs are not ready.
5. Callstack/locals/watch/breakpoint surface skeletons only when backed by real or explicitly unavailable runtime state.
6. Acceptance scenario that proves the GUI tells the truth across browser-disabled, simulated, and native-service-missing profiles.

## Open Interface Needs

Potential OxVba-facing handoffs for W270:

1. runtime session identity and lifecycle packets,
2. build/run target enumeration and selection,
3. Immediate request/response/event packet shape,
4. debug pause/resume/step command availability,
5. callstack frame projection,
6. locals/watch value projection,
7. breakpoint binding and diagnostic states,
8. runtime error taxonomy shared with COM errors where applicable.

## Regression Scenarios To Preserve

Before W270 acceptance, keep these scenarios rendering:

```text
gui-thin-slice-loaded
gui-thin-slice-edited-diagnostics
gui-thin-slice-lifecycle
gui-run-output-browser-disabled
gui-run-output-simulated-supported
gui-dnaonecalc-embedding-contract
gui-com-reference-browser-unavailable
gui-com-reference-nonwindows-unavailable
gui-com-reference-native-service-missing
```
