# Workset W355 — OxVba Compile/Build Adapter Profiles

## Ambition

Connect the shared OxIde/DnaOxIde compile/build UX to real OxVba compiler APIs across the product host profiles established in the charter:

1. browser website / DnaOneCalc WASM host with an OxVba wasm-safe compiler/runtime profile;
2. standalone DnaOxIde desktop with linked native Rust commands;
3. DnaOneCalc Windows desktop host embedding OxIde and exposing native OxVba services.

A user should be able to edit source, save according to host policy, request compile/check, and see typed build status/diagnostics without CLI text parsing or fake output. In browser/WASM mode this must be a real wasm-safe OxVba path or a typed unavailable state; in desktop mode this must flow through the Tauri/native Rust command layer after W352.

## Dependencies

- W350 — DnaOxIde live editable source app and instrumentation harness.
- W352 — DnaOxIde Tauri/WebView product host and native command spine, for desktop-profile acceptance.
- W347 — compile/options/reference placeholders.
- W349 — OxVba integration readiness report.
- [`CHARTER.md`](../../CHARTER.md).
- [`docs/OXIDE_TARGET_STACK_SCENARIOS.md`](../OXIDE_TARGET_STACK_SCENARIOS.md).
- OxVba direct host surfaces for wasm-safe compile/check, native compile/build, compile options, run targets, request IDs, command status, lifecycle events, and diagnostics.

## Design

W355 should adopt OxVba-owned compile/build DTOs through a thin OxIde adapter. It must distinguish host profiles rather than pretending all hosts have the same abilities.

Profile expectations:

- **browser/DnaOneCalc WASM profile** — compile/check through OxVba wasm-safe APIs and, where OxVba supports it, produce an invokable browser runtime artifact/interpreter handle for DnaOneCalc. Native-only build outputs are typed unavailable.
- **DnaOxIde desktop profile** — UI command reaches Tauri native Rust, then linked OxIde/OxVba crates perform compile/build/check over temp or selected project copies.
- **DnaOneCalc desktop profile** — same shared UI/packet contract, with DnaOneCalc desktop host policy exposing native OxVba services.

The adapter must not define authoritative `.basproj`, language, compile, runtime, or diagnostic semantics locally. Missing OxVba capabilities should create explicit handoffs or typed unavailable states, not fake build output.

## Beads

### W355-B00 — Compile/build adapter contract lock

Goal:
  Lock the OxVba compile/build DTOs, host capability profiles, and OxIde packet shape before implementation.

Design:
  - Name OxVba-owned DTOs for wasm-safe compile/check, native build/check, options, run targets, requests, status, events, diagnostics, and browser-runtime invocation handles where available.
  - Define OxIde wrapper packets only at real UI/host serialization boundaries.
  - Define browser/WASM, DnaOxIde desktop, and DnaOneCalc desktop command paths.
  - Keep local wrappers clearly non-authoritative.
  - Record any required OxVba upstream changes as handoffs.

Tests:
  - Documentation grep for DTO names, host profiles, command IDs, no CLI/LSP fallback, no fake build output, and no native-only claim in browser/WASM mode.

Evidence:
  - `docs/DNAOXIDE_COMPILE_BUILD_ADAPTER_CONTRACT.md`.
  - `target/w355-b00-compile-build-contract.txt`.

Closure:
  - [ ] OxVba-owned DTOs are named.
  - [ ] Browser/WASM and native desktop profiles are explicit.
  - [ ] OxIde wrapper boundaries are clear.
  - [ ] Required OxVba work is listed rather than duplicated locally.
  - [ ] No runtime/debug/COM claims are introduced.

### W355-B01 — Browser/WASM compile/check adapter path

Goal:
  Prove or explicitly block the DnaOneCalc browser/WASM compile/check path through OxVba wasm-safe APIs.

Design:
  - Compile only wasm-safe OxVba crates/features.
  - Drive source text into the wasm-safe compile/check API.
  - Return typed compile status/diagnostics or a typed unavailable packet naming the missing OxVba seam.
  - If runnable browser artifacts/interpreter handles exist, expose the typed handle for host invocation; otherwise record the missing OxVba requirement.

Tests:
  - WASM-target or wasm-safe feature build/check where locally available.
  - No native filesystem/process/COM requirement in browser profile.
  - No fake compile/run data.

Evidence:
  - `target/w355-b01-browser-wasm-compile-adapter.txt`.
  - Optional OxVba handoff if the wasm-safe seam is missing.

Closure:
  - [ ] Browser/WASM compile/check path is adapter-backed or explicitly blocked by a named OxVba gap.
  - [ ] Browser profile has typed unavailable states for native-only outputs.
  - [ ] No fake build/runtime data is shown.

### W355-B02 — Desktop native compile/build command adapter

Goal:
  DnaOxIde desktop commands can return compile options/run-target/build-check packets from OxVba direct APIs through the Tauri/native Rust command spine.

Design:
  - Wire a narrow adapter behind W344/W352 command names.
  - UI command must reach linked native Rust in the desktop host before calling OxVba.
  - Preserve disabled reasons when data is unavailable.
  - Keep checked-in fixtures unchanged.

Tests:
  - Command tests over temp project copies.
  - Desktop command-spine test from W352.
  - Fixture mutation guard.

Evidence:
  - `target/w355-b02-desktop-command-adapter.txt`.

Closure:
  - [ ] Compile options are adapter-backed in the desktop profile.
  - [ ] Build/check command returns typed status or typed unavailable.
  - [ ] UI->Tauri->Rust->OxVba path is evidenced.
  - [ ] No checked-in fixture mutation occurs.

### W355-B03 — Compile/build UI adoption and hosted interaction proof

Goal:
  Shared compile/build panels display adapter-backed compile/build data for the active host profile, and at least one real hosted edit-save-compile flow is driven.

Design:
  - Replace placeholder-only rows with adapter packets where available.
  - Preserve pending/unavailable states for any missing OxVba data.
  - Show request IDs, profile label, and build lifecycle events where available.
  - Keep the component host-neutral so DnaOxIde desktop, DnaOneCalc browser, and DnaOneCalc desktop can consume it.
  - For desktop, use the W352 Tauri/WebView native command host.
  - For browser/WASM, use the DnaOneCalc-compatible wasm-safe command path where available.
  - Assert typed result and no fake output.

Tests:
  - Panel render tests and token greps.
  - Browser profile and desktop profile packet fixtures.
  - Hosted interaction check for at least one real product host profile.
  - Workspace tests.
  - No fake output and no native-only browser claim scans.

Evidence:
  - `target/w355-b03-ui-and-interaction.txt`.

Closure:
  - [ ] UI shows adapter-backed compile/build status by profile.
  - [ ] Request/event identity is visible.
  - [ ] Unavailable states remain honest.
  - [ ] Shared UI remains host-neutral.
  - [ ] Edit-save-compile flow is driven through a real host seam.
  - [ ] Output is OxVba-backed or typed unavailable.

### W355-B04 — W355 acceptance

Goal:
  Accept compile/build adapter readiness for reference/COM and runtime follow-on work.

Design:
  - Run W350/W352/W355 checks and update handoff for W360/W365.
  - Confirm browser/WASM and native desktop capability distinctions are reflected in evidence.

Tests:
  - Full compile/build regression and no-claim scan.

Evidence:
  - `target/w355-acceptance.txt`.
  - `docs/HANDOFF_W355_COMPILE_BUILD_ADAPTER.md`.

Closure:
  - [ ] Compile/build adapter is accepted for at least one real product host profile.
  - [ ] Browser/WASM support is proven or blocked on named OxVba work.
  - [ ] W360 references/COM and W365 runtime work are unblocked according to profile.
  - [ ] No runtime/debug/COM runtime claim is introduced.

## Out-of-scope

- COM reference repair/runtime invocation.
- Full debug/Immediate behavior.
- Native binary wrapping unless OxVba already exposes a typed supported path.
- Browser native filesystem/process/COM assumptions.
