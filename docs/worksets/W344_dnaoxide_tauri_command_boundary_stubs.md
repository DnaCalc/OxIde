# Workset W344 — DnaOxIde Tauri Command Boundary Stubs

## Ambition

Define and implement the first **DnaOxIde** Tauri command boundary over the host bridge, using capabilities already proven in OxIde, available-subset OxVba adapters where dependency wiring is ready, OxVba ThinSliceHello fixture-evidenced adapter targets, and typed unavailable states for still-pending adoption/hardening gaps.

This workset creates the IPC seam that the live desktop host will use while OxIde adopts the new OxVba fixture-evidenced seams and while remaining taxonomy, compile/run target DTO, event stream, source-span, host UX, and COM native boundary details are hardened.

## Dependencies

- W320 — native filesystem/session persistence proof.
- W341 — DnaOxIde Tauri app scaffold.
- W343 — OxIde host bridge facade.
- [`docs/HANDOFF_DNAOXIDE_OXVBA_FEEDBACK_ALIGNMENT.md`](../HANDOFF_DNAOXIDE_OXVBA_FEEDBACK_ALIGNMENT.md).
- [`docs/HANDOFF_W343_HOST_BRIDGE_FACADE.md`](../HANDOFF_W343_HOST_BRIDGE_FACADE.md).
- [`docs/DNAOXIDE_COMMAND_BOUNDARY.md`](../DNAOXIDE_COMMAND_BOUNDARY.md).

## Design

Tauri commands should be thin adapters over host bridge services. They may initially be Rust-callable command functions even before a full WebView smoke exists. The stable command naming table lives in [`docs/DNAOXIDE_COMMAND_BOUNDARY.md`](../DNAOXIDE_COMMAND_BOUNDARY.md).

Safe/proven commands to implement first:

- open project path from an explicit path,
- load active module from disk/test copy,
- save module source,
- reload module source,
- save session snapshot,
- load session snapshot,
- return host capability profile,
- return runtime/Immediate/debug/COM unavailable packets.

Commands should be classified into four buckets:

1. **proven OxIde-only** — project/document/session lifecycle from W320;
2. **available-subset OxVba adapters** — workspace/editor, project authoring, COM selection subset, build/run subset, Immediate subset, and debug subset when direct dependency wiring is ready;
3. **OxVba fixture-evidenced adapter targets** — ThinSliceHello evidence covers overlay build, runtime session creation, Immediate attach/evaluation, debug attach, watch evaluation, breakpoint binding, stable runtime/debug/watch/breakpoint IDs, broken COM reference state, and COM capability profile, but OxIde command tests must still prove consumption;
4. **pending-hardening unavailable** — compile options/run target DTOs, lifecycle event streams, source-span mapping, command availability taxonomy, COM native boundary status, COM runtime invocation, and full runtime/debug/COM host UX claims.

Named commands should still exist for:

- build/check,
- compile options fetch/apply,
- COM candidate discovery/apply,
- run/stop/reset,
- Immediate evaluate,
- debug attach/continue/step/stop,
- watch add/update/remove/evaluate,
- breakpoint set/clear/enable/disable.

Unavailable or partial commands must return typed unavailable/pending-hardening responses and must not synthesize fake data.

## Beads

### W344-B00 — IPC command naming contract

Goal:
  Define stable command names and request/response categories before implementation.

Design:
  - Name commands by host service category.
  - Mark each command as proven OxIde-only, available-subset OxVba adapter, OxVba fixture-evidenced adapter target, or pending-hardening unavailable.
  - Keep names stable for frontend/e2e tests.

Tests:
  - Documentation/static grep for command names and blocked/proven labels.

Evidence:
  - [`docs/DNAOXIDE_COMMAND_BOUNDARY.md`](../DNAOXIDE_COMMAND_BOUNDARY.md).

Closure:
  - [ ] Command names are listed.
  - [ ] Proven, available-subset, fixture-evidenced, and pending-hardening categories are clear.
  - [ ] No fake service behavior is implied.

### W344-B01 — Project/document/session commands

Goal:
  Implement Rust-callable/Tauri-ready commands for proven filesystem and session lifecycle.

Design:
  - Use W320 native filesystem/session persistence models.
  - Use test-owned temp project copies.
  - Return serializable responses suitable for the frontend.

Tests:
  - Command unit tests open/save/reload/session-restore temp projects.
  - Checked-in fixture mutation guard.

Evidence:
  - `target/w344-project-document-command-tests.txt`.

Closure:
  - [ ] Project/document commands work on temp copies.
  - [ ] Session commands persist and restore.
  - [ ] Checked-in fixtures remain unchanged.

### W344-B02 — Capability, available-subset, and unavailable service commands

Goal:
  Implement command stubs/adapters for runtime, Immediate, debug, build/check, compile options, and COM/reference services.

Design:
  - Return existing `RuntimeServicePacket`, `ImmediateServicePacket`, `DebugServicePacket`, and COM unavailable states where full OxIde evidence is missing.
  - Add available-subset or fixture-evidenced adapter responses for current OxVba direct surfaces where dependency wiring is ready.
  - Add build/compile/reference unavailable responses if no shared DTO exists yet.

Tests:
  - Adapter tests verify available-subset or fixture-evidenced behavior where wired.
  - Stub tests verify unavailable states and disabled reasons for pending-hardening gaps.
  - Anti-fake-data tests for empty or explicitly subset-backed debug/Immediate data.

Evidence:
  - `target/w344-unavailable-service-command-tests.txt`.

Closure:
  - [ ] Available-subset and fixture-evidenced commands are labeled and tested.
  - [ ] Pending-hardening commands return typed unavailable states.
  - [ ] No fake responses/callstacks/locals/watches/breakpoints appear.
  - [ ] COM runtime remains unclaimed.

### W344-B03 — Frontend command client shim

Goal:
  Add the frontend-side command client abstraction without binding shared UI to Tauri.

Design:
  - DnaOxIde app may call Tauri invoke.
  - Shared UI talks to host bridge/client traits.
  - Browser review profile can use fixture client.

Tests:
  - Client shim unit/static tests.
  - Dependency grep verifies shared UI has no Tauri dependency.

Evidence:
  - Command client tests or grep output.

Closure:
  - [ ] DnaOxIde has a command client shim.
  - [ ] Shared UI remains Tauri-free.
  - [ ] Browser fixture path remains available.

### W344-B04 — W344 acceptance

Goal:
  Accept the command boundary as ready for live host UI proof.

Design:
  - Update docs with command table and evidence.
  - Link W345 live host UI proof.

Tests:
  - Workspace tests.
  - Tauri command/unit tests.
  - Anti-overclaim scan.

Evidence:
  - W344 acceptance outputs.

Closure:
  - [ ] Proven commands are implemented.
  - [ ] Available-subset services and pending-hardening stubs are honest.
  - [ ] Live host UI proof is unblocked.

## Out-of-scope

- Full OxVba build/check, runtime, Immediate, debug, watch, breakpoint, or COM behavior beyond explicit available-subset or fixture-evidenced adapter proofs.
- Full WebView/e2e automation; W345/W346 own it.
- DnaOneCalc command implementation.
