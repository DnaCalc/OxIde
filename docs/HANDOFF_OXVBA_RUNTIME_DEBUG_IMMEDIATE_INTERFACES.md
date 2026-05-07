# Handoff — OxVba Runtime, Debug, And Immediate Interfaces

Status: `cross_repo_handoff`
Date: 2026-05-07

## Source Workset

OxIde W270 added GUI-lab surfaces for:

```powershell
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-run-timeline-simulated
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-immediate-browser-disabled
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-debug-browser-disabled
```

These scenarios are capability/UI proofs only. They do not claim real OxVba runtime, Immediate, or debug sessions.

## What OxIde Can Render Now

OxIde can render:

1. structured run timeline from `RunTranscript`,
2. simulated run timeline with deterministic output,
3. browser-disabled Immediate panel,
4. browser-disabled debug panel,
5. native-runtime-required/future-supported capability labels in pure state,
6. no-fake-data empty states for Immediate/debug surfaces.

OxIde cannot yet prove:

1. real OxVba runtime session identity,
2. real build/run target enumeration from OxVba,
3. Immediate request/response/event execution,
4. debug pause/resume/step state,
5. callstack frame projection,
6. locals/watch value projection,
7. breakpoint binding against runtime code,
8. runtime errors over COM-capable execution.

## Required Authoritative Inputs

Prefer OxVba or a shared DNA Calc crate to own these interfaces rather than duplicating them in OxIde.

### Runtime Session

Needed:

1. stable runtime session id,
2. project/workspace id correlation,
3. target/entrypoint enumeration,
4. run command availability and disabled reasons,
5. runtime event stream with lifecycle/activity/output/error events,
6. cancellation/stop command availability.

### Immediate Window

Needed:

1. Immediate session id tied to runtime session,
2. request packet for entered text,
3. response/event packet for output, value, diagnostic, and runtime error,
4. command availability when runtime is stopped/running/paused,
5. host capability reasons for browser-disabled and native-runtime-required states.

### Debug Session

Needed:

1. debug session id,
2. paused/running/stopped state,
3. command availability for continue, break, step into, step over, step out, restart, stop,
4. callstack frame DTOs,
5. locals projection DTOs,
6. watch expression request/result DTOs,
7. breakpoint bind/unbind/status DTOs,
8. source span mapping for runtime locations.

### Error Taxonomy

Needed deterministic labels for:

1. runtime unavailable,
2. build failed,
3. run target missing,
4. native service not configured,
5. COM discovery unavailable,
6. COM runtime unavailable,
7. Immediate evaluation failed,
8. debug session unavailable,
9. breakpoint unresolved,
10. user-denied/blocked by host policy.

## W260 COM Constraint

W260 currently proves:

1. browser COM discovery/runtime unavailable,
2. non-Windows COM unavailable,
3. Windows native COM service missing.

Until a tested native COM service exists, W270+ surfaces must not claim COM-capable run/debug/Immediate.

## Suggested Next OxVba-Scoped Bead

```text
Expose minimal runtime session packet
  -> list run targets for ThinSliceHello
  -> report browser/native command availability
  -> emit structured run lifecycle/output/error events
  -> expose Immediate/debug unavailable states with typed reasons
```

Once that exists, OxIde can replace its W270 pure unavailable/simulated scenarios with real OxVba-backed runtime/Immediate/debug scenarios while preserving the same GUI surface contracts.
