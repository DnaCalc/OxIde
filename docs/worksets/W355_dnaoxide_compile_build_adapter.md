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
- Current OxVba compile/build-related surfaces as they exist when each bead runs. The OxVba/OxIde seam is intentionally agile at this stage: use what exists, adapt thinly on the OxIde side where reasonable, and request OxVba changes only when the current surface blocks an honest product path.

## Design

W355 should connect OxIde to the current OxVba compile/build shape without treating the seam as frozen. It must distinguish host profiles rather than pretending all hosts have the same abilities, but it should avoid contract-lock ceremony. The rule is: use OxVba as it is, keep OxIde wrappers thin and honest, adapt on either side as learning arrives, and record requested OxVba changes only when needed.

Profile expectations:

- **browser/DnaOneCalc WASM profile** — compile/check through OxVba wasm-safe APIs and, where OxVba supports it, produce an invokable browser runtime artifact/interpreter handle for DnaOneCalc. Native-only build outputs are typed unavailable.
- **DnaOxIde desktop profile** — UI command reaches Tauri native Rust, then linked OxIde/OxVba crates perform compile/build/check over temp or selected project copies.
- **DnaOneCalc desktop profile** — same shared UI/packet shape where practical, with DnaOneCalc desktop host policy exposing native OxVba services.

The adapter must not define authoritative `.basproj`, language, compile, runtime, or diagnostic semantics locally. Missing OxVba capabilities should create explicit handoffs or typed unavailable states, not fake build output. Profiles, commands, and packet fields should be documented enough to guide implementation, but they are working integration notes rather than a locked cross-repo contract.

## Beads

### W355-B00 — Compile/build adapter profile and command map

Goal:
  Establish the current compile/build integration map: host profiles, command IDs, likely OxVba surfaces to call, thin OxIde packet boundaries, and honest unavailable states, without freezing the OxVba/OxIde seam as a fixed contract.

Design:
  - Inspect current OxVba compile/build/check/options/run-target surfaces available to OxIde.
  - Define working browser/WASM, DnaOxIde desktop, and DnaOneCalc desktop profiles.
  - Define command IDs and UI-visible packet fields needed for the first implementation pass.
  - Keep local wrappers clearly non-authoritative and adaptable.
  - Record OxVba requests only for real gaps encountered, not speculative contract completion.
  - Preserve no CLI/LSP fallback and no fake build output.

Tests:
  - Documentation grep for host profiles, command IDs, current OxVba surfaces or named gaps, no CLI/LSP fallback, no fake build output, and no native-only claim in browser/WASM mode.

Evidence:
  - `docs/DNAOXIDE_COMPILE_BUILD_ADAPTER_PROFILE_MAP.md`.
  - `target/w355-b00-compile-build-profile-map.txt`.

Closure:
  - [x] Browser/WASM and native desktop profiles are explicit.
  - [x] First-pass command IDs and packet fields are documented.
  - [x] Current OxVba surfaces or named gaps are recorded.
  - [x] OxIde wrappers remain thin, non-authoritative, and adaptable.
  - [x] No runtime/debug/COM claims are introduced.

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
  - [x] Browser/WASM compile/check path is adapter-backed or explicitly blocked by a named OxVba gap.
  - [x] Browser profile has typed unavailable states for native-only outputs.
  - [x] No fake build/runtime data is shown.

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
  - `target/w355-b02-desktop-native-compile-build-adapter.txt`.

Closure:
  - [x] Compile options are adapter-backed or honestly unavailable in the desktop profile.
  - [x] Build/check command returns typed status or typed unavailable.
  - [x] UI->Tauri->Rust->OxVba path is evidenced.
  - [x] No checked-in fixture mutation occurs.

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
  - [x] UI shows adapter-backed compile/build status by profile.
  - [x] Request/event identity is visible.
  - [x] Unavailable states remain honest.
  - [x] Shared UI remains host-neutral.
  - [x] Edit-save-compile flow is driven through a real host seam.
  - [x] Output is OxVba-backed or typed unavailable.

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
