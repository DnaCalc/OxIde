# Workset W370 — DnaOxIde Debug Watch And Breakpoint Adapter

## Ambition

Connect DnaOxIde debug, watch, breakpoint, callstack, and locals panes to OxVba direct Rust APIs after W365 proves runtime and Immediate adapter flow.

A user should be able to start/attach a debug session, see typed command availability, inspect callstack/locals where available, add/evaluate watches, set source breakpoints, and see bound/unresolved state without fake debug data.

## Dependencies

- W350 — live editable source app.
- W355 — compile/build adapter.
- W360 — reference/COM adapter.
- W365 — runtime/Immediate adapter.
- OxVba `DebugSession`, debug command status, watch records/evaluations, breakpoint records/binding, pause/frame/local/source-span DTOs.

## Design

W370 should adopt OxVba debug DTOs through direct Rust adapters:

- debug session IDs and runtime correlation;
- command availability for start/continue/step/stop;
- pause state, callstack, frame IDs, locals;
- watch registry and evaluation status;
- source breakpoint records and binding/unresolved state;
- source-span/source mapping fields where available;
- no fake callstack/local/watch/breakpoint rows.

## Beads

### W370-B00 — Debug/watch/breakpoint adapter contract lock

Goal:
  Lock OxVba debug DTOs and DnaOxIde packet boundaries.

Design:
  - Map debug session, command status, frames/locals, watches, breakpoints, pause state, and source mapping.
  - Explicitly document remaining source-span breadth gaps.

Tests:
  - Documentation grep for DebugSession, watch, breakpoint, source mapping, no-fake-debug-data.

Evidence:
  - `docs/DNAOXIDE_DEBUG_ADAPTER_CONTRACT.md`.
  - `target/w370-b00-debug-contract.txt`.

Closure:
  - [ ] Debug DTO ownership is clear.
  - [ ] Source mapping gaps are explicit.
  - [ ] Fake debug data remains forbidden.

### W370-B01 — Debug session command adapter

Goal:
  DnaOxIde can create/attach a debug session and return typed command availability and pause state.

Design:
  - Wire OxVba `DebugSession` behind host commands.
  - Preserve unavailable/disabled states for missing runtime/session.

Tests:
  - Command tests for attach/start/continue/step disabled and available states.

Evidence:
  - `target/w370-b01-debug-session-adapter.txt`.

Closure:
  - [ ] Debug session command path is adapter-backed.
  - [ ] Command availability is typed.
  - [ ] Missing-session state is tested.

### W370-B02 — Watch and breakpoint command adapter

Goal:
  DnaOxIde can add/evaluate watches and set/inspect source breakpoints through OxVba DTOs.

Design:
  - Wire watch registry/evaluation and breakpoint binding records.
  - Preserve stable IDs and unresolved reasons.
  - Avoid fake watch/breakpoint rows.

Tests:
  - Command tests for watch registry/evaluation and breakpoint binding/unresolved state.
  - No fake debug data scan.

Evidence:
  - `target/w370-b02-watch-breakpoint-adapter.txt`.

Closure:
  - [ ] Watches are adapter-backed.
  - [ ] Breakpoints are adapter-backed.
  - [ ] Unresolved states are typed.

### W370-B03 — Debug UI and interaction proof

Goal:
  The live DnaOxIde flow can run/debug, show debug panes, add a watch, set a breakpoint, and display typed OxVba-backed or typed unavailable data.

Design:
  - Extend W350/W365 live interaction harness.
  - Assert callstack/locals/watch/breakpoint tokens by stable IDs or disabled reasons.
  - Preserve COM runtime claim boundary.

Tests:
  - Live debug interaction check, command tests, anti-overclaim scan.

Evidence:
  - `target/w370-b03-debug-interaction.txt`.

Closure:
  - [ ] Debug interaction is driven.
  - [ ] Watch/breakpoint panes are backed or typed unavailable.
  - [ ] No fake debug data appears.

### W370-B04 — W370 acceptance

Goal:
  Accept the debug/watch/breakpoint adapter lane and document remaining polish/packaging work.

Design:
  - Run W350-W370 regression and produce handoff.

Tests:
  - Full debug regression and no-claim scan.

Evidence:
  - `target/w370-acceptance.txt`.
  - `docs/HANDOFF_W370_DEBUG_WATCH_BREAKPOINT_ADAPTER.md`.

Closure:
  - [ ] Debug/watch/breakpoint adapter is accepted.
  - [ ] Remaining source-span/polish gaps are documented.
  - [ ] No fake capability has landed.

## Out-of-scope

- COM runtime invocation.
- Full source-span perfection beyond OxVba DTO coverage.
- DnaOneCalc implementation.
