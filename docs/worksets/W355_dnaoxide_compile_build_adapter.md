# Workset W355 — DnaOxIde Compile And Build Adapter

## Ambition

Connect DnaOxIde compile/build UX to OxVba direct Rust DTOs/APIs after W350 proves live editable source text and temp-copy save/reload.

A user should be able to edit source in DnaOxIde, save to a temp project copy, request build/check, and see typed build status/diagnostics without CLI text parsing or fake output.

## Dependencies

- W350 — DnaOxIde live editable source app.
- W347 — compile/options/reference placeholders.
- W349 — OxVba integration readiness report.
- OxVba direct host surfaces for compile options, build request IDs, command status, lifecycle events, and diagnostics.

## Design

W355 should adopt OxVba-owned compile/build DTOs through a thin OxIde adapter:

- project properties / compile options / run target surface;
- typed build/check request packets;
- command availability and disabled reasons;
- build lifecycle event packets;
- typed diagnostics mapped to existing editor/diagnostics panes.

The adapter must operate on the same temp project copy/save policy proven by W350. It must not define authoritative `.basproj` or compile semantics locally.

## Beads

### W355-B00 — Compile/build adapter contract lock

Goal:
  Lock the OxVba compile/build DTOs and OxIde packet shape before implementation.

Design:
  - Map OxVba compile options, run targets, build requests, status, events, and diagnostics to host bridge categories.
  - Keep local wrappers clearly non-authoritative.

Tests:
  - Documentation grep for DTO names, command IDs, no CLI/LSP fallback, and no fake build output.

Evidence:
  - `docs/DNAOXIDE_COMPILE_BUILD_ADAPTER_CONTRACT.md`.
  - `target/w355-b00-compile-build-contract.txt`.

Closure:
  - [ ] OxVba-owned DTOs are named.
  - [ ] OxIde wrapper boundaries are clear.
  - [ ] No runtime/debug/COM claims are introduced.

### W355-B01 — Compile options and build command adapter

Goal:
  DnaOxIde commands can return compile options/run-target/build-check packets from OxVba direct APIs for a temp project copy.

Design:
  - Wire a narrow adapter behind W344 command names.
  - Preserve disabled reasons when data is unavailable.
  - Keep checked-in fixtures unchanged.

Tests:
  - Command tests over temp project copies.
  - Fixture mutation guard.

Evidence:
  - `target/w355-b01-command-adapter.txt`.

Closure:
  - [ ] Compile options are adapter-backed.
  - [ ] Build/check command returns typed status.
  - [ ] No checked-in fixture mutation occurs.

### W355-B02 — Compile/build UI panel adoption

Goal:
  W347 placeholder panels display adapter-backed compile/build data where available.

Design:
  - Replace placeholder-only rows with adapter packets.
  - Preserve pending/unavailable states for any missing OxVba data.
  - Show request IDs and build lifecycle events where available.

Tests:
  - Panel render tests and token greps.
  - Anti-overclaim scan.

Evidence:
  - `target/w355-b02-ui-panels.txt`.

Closure:
  - [ ] UI shows adapter-backed compile/build status.
  - [ ] Request/event identity is visible.
  - [ ] Unavailable states remain honest.

### W355-B03 — Edit-save-build interaction proof

Goal:
  The live DnaOxIde flow can edit source, save, run build/check, and show typed results.

Design:
  - Extend W350 live interaction harness.
  - Drive edit -> save -> build/check.
  - Assert typed result and no fake output.

Tests:
  - Live interaction check.
  - Workspace tests.

Evidence:
  - `target/w355-b03-edit-save-build.txt`.

Closure:
  - [ ] Edit-save-build flow is driven.
  - [ ] Output is OxVba-backed or typed unavailable.
  - [ ] No fake build data is shown.

### W355-B04 — W355 acceptance

Goal:
  Accept compile/build adapter readiness for COM/runtime follow-on work.

Design:
  - Run W350/W355 checks and update handoff for W360.

Tests:
  - Full compile/build regression and no-claim scan.

Evidence:
  - `target/w355-acceptance.txt`.
  - `docs/HANDOFF_W355_COMPILE_BUILD_ADAPTER.md`.

Closure:
  - [ ] Compile/build adapter is accepted.
  - [ ] W360 references/COM work is unblocked.
  - [ ] No runtime/debug/COM runtime claim is introduced.

## Out-of-scope

- COM reference repair/runtime invocation.
- Live runtime execution beyond build/check.
- Debug/Immediate behavior.
