# Workset W365 — DnaOxIde Runtime And Immediate Adapter

## Ambition

Connect the shared OxIde runtime and Immediate Window UX to OxVba direct Rust APIs after W350-W360 prove live editing, build/check, and reference/COM status.

A user should be able to run saved source according to the active host profile, receive typed runtime session status, attach/open Immediate where supported, submit a typed Immediate request, and see OxVba-backed output without fake responses. Browser/WASM runtime is in scope only through OxVba's wasm-safe runtime profile; native desktop runtime is in scope through linked native Rust host commands.

## Dependencies

- W350 — live editable source app.
- W352 — Tauri/WebView native command spine for desktop evidence.
- W355 — compile/build adapter profiles.
- W360 — reference/COM adapter.
- OxVba `EmbeddedBuildRunHost`, `EmbeddedRunSession`, runtime command status/events, `ImmediateSession`, and Immediate result DTOs.

## Design

W365 should adopt OxVba runtime/Immediate DTOs through direct Rust adapters and profile-aware host packets:

- browser/WASM runtime handles or typed unavailable state from OxVba wasm-safe profile;
- native desktop runtime session handles through Tauri/native Rust command spine;
- run request IDs and runtime session IDs;
- run lifecycle events and typed failure states;
- runtime command availability;
- Immediate session IDs and runtime correlation;
- typed Immediate evaluation requests/results;
- no-session disabled states;
- no fake responses.

## Beads

### W365-B00 — Runtime/Immediate adapter contract lock

Goal:
  Lock the OxVba runtime and Immediate DTOs plus DnaOxIde packet boundaries.

Design:
  - Map run request, runtime session, lifecycle events, command status, Immediate session, and response DTOs.
  - Define no-session disabled state.

Tests:
  - Documentation grep for runtime/Immediate DTO names, IDs, command statuses, and no-fake-response rules.

Evidence:
  - `docs/DNAOXIDE_RUNTIME_IMMEDIATE_ADAPTER_CONTRACT.md`.
  - `target/w365-b00-runtime-immediate-contract.txt`.

Closure:
  - [ ] Runtime/Immediate DTO ownership is clear.
  - [ ] No-session state is defined.
  - [ ] Fake responses remain forbidden.

### W365-B01 — Runtime command adapter

Goal:
  DnaOxIde can start a runtime session for a saved temp project copy and return typed status/events.

Design:
  - Wire OxVba wasm-safe runtime APIs or native `EmbeddedBuildRunHost`/`EmbeddedRunSession` behind host commands according to profile.
  - Preserve typed disabled/failed states, especially native-only operations in browser/WASM.
  - Keep source-span/runtime-error gaps explicit.

Tests:
  - Command tests over temp project copies.
  - Failure/unavailable tests.

Evidence:
  - `target/w365-b01-runtime-adapter.txt`.

Closure:
  - [ ] Runtime session command is adapter-backed.
  - [ ] IDs/events are visible where available.
  - [ ] Failure/unavailable states are typed.

### W365-B02 — Immediate command adapter

Goal:
  DnaOxIde can attach/open an Immediate session from an active runtime and return typed response packets.

Design:
  - Wire `EmbeddedRunSession::into_immediate_session` and `ImmediateSession`.
  - Render typed value/printed/reset/empty/diagnostic outputs.
  - Keep no-runtime disabled state visible.

Tests:
  - Command tests for attached session and no-session disabled state.
  - No fake response scan.

Evidence:
  - `target/w365-b02-immediate-adapter.txt`.

Closure:
  - [ ] Immediate session is adapter-backed.
  - [ ] No-session state is tested.
  - [ ] No fake Immediate rows are shown.

### W365-B03 — Runtime/Immediate UI and interaction proof

Goal:
  The live DnaOxIde flow can edit, save, build/run, open Immediate, submit a request, and show typed output or typed unavailable state.

Design:
  - Use the real hosted seam for the active profile: browser WASM host or Tauri/native command host.
  - Assert runtime IDs/events and Immediate output tokens where available.
  - Keep COM/debug claims false.

Tests:
  - Live interaction check, command tests, anti-overclaim scan.

Evidence:
  - `target/w365-b03-runtime-immediate-interaction.txt`.

Closure:
  - [ ] Runtime/Immediate flow is driven.
  - [ ] Output is OxVba-backed or typed unavailable.
  - [ ] Debug/COM claims remain false.

### W365-B04 — W365 acceptance

Goal:
  Accept runtime/Immediate adapter readiness for debug/watch/breakpoint work.

Design:
  - Run W350-W365 regression and update W370 handoff.

Tests:
  - Full runtime/Immediate regression and no-claim scan.

Evidence:
  - `target/w365-acceptance.txt`.
  - `docs/HANDOFF_W365_RUNTIME_IMMEDIATE_ADAPTER.md`.

Closure:
  - [ ] Runtime/Immediate adapter is accepted.
  - [ ] W370 debug work is unblocked.
  - [ ] No fake runtime/Immediate data exists.

## Out-of-scope

- Full debug/watch/breakpoint UI.
- COM runtime invocation.
- Browser/WASM runtime claims beyond OxVba's wasm-safe profile.
