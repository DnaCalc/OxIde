# Workset W352 — DnaOxIde Tauri/WebView Product Host And Automation

## Ambition

Move DnaOxIde from the W350 browser harness onto the real desktop product path: a Tauri/WebView host with a linked native Rust command layer by default.

W352 should prove that the DnaOxIde UI runs inside the desktop shell, that UI commands can reach native Rust code in the Tauri app, and that automation can observe enough state to keep the desktop host regression-safe.

This is not an optional exhibition lane. It is the desktop product-host lane required before compile/build/runtime/debug/COM work is accepted as product work.

## Dependencies

- W350 — DnaOxIde live editable source app and instrumentation.
- [`CHARTER.md`](../../CHARTER.md).
- [`docs/OXIDE_TARGET_STACK_SCENARIOS.md`](../OXIDE_TARGET_STACK_SCENARIOS.md).
- [`docs/DNAOXIDE_LIVE_EDITABLE_PROOF_MODE.md`](../DNAOXIDE_LIVE_EDITABLE_PROOF_MODE.md).
- [`docs/DNAOXIDE_TAURI_WEBVIEW_AUTOMATION_PLAN.md`](../DNAOXIDE_TAURI_WEBVIEW_AUTOMATION_PLAN.md).

## Design

W352 should:

1. bootstrap Tauri/WebView tooling explicitly;
2. start DnaOxIde as a desktop app host;
3. mount the shared/W350-instrumented app shell only as a stepping stone toward shared Rust/WASM UI;
4. wire at least one UI command through the WebView into linked native Rust code in the Tauri app;
5. keep save/reload over temp project copies in the native Rust command layer, not Playwright-injected browser services;
6. automate or inspect the WebView through a documented driver;
7. capture visual, DOM-like, command, and event artifacts;
8. keep runtime/debug/Immediate/COM claims false until the later native adapter work proves them.

Default native backend meaning:

```text
DnaOxIde Tauri app
  ├─ WebView UI
  └─ Rust app crate linked with OxIde/OxVba crates
```

A separate service process is out-of-scope unless a later workset explicitly chooses it for COM/runtime isolation.

## Beads

### W352-B00 — Desktop host toolchain and native command spine plan

Goal:
  Decide the exact Tauri/WebView tooling path needed for local desktop execution, automation, and linked native Rust command calls.

Design:
  - Inspect local Tauri, Rust, Node, WebView2, and driver availability.
  - Decide whether to use Tauri CLI, npm Tauri CLI, WebDriver, WebView2 debugging, or another supported route.
  - Define the first native command spine: UI command -> Tauri command -> linked Rust function -> typed result.
  - Document installation/network requirements before adding dependencies.

Tests:
  - Tool availability command transcript.
  - Documentation grep for chosen tooling, linked native Rust backend meaning, and no-claim boundaries.

Evidence:
  - `docs/DNAOXIDE_TAURI_WEBVIEW_AUTOMATION_PLAN.md` update.
  - `target/w352-b00-tauri-toolchain-plan.txt`.

Closure:
  - [ ] Tooling path is explicit.
  - [ ] Install/network requirements are visible.
  - [ ] First native command spine is specified.
  - [ ] No desktop/native capability claim is made before proof.

### W352-B01 — Tauri dev shell starts with native command spine

Goal:
  Launch DnaOxIde in a Tauri/WebView shell and prove a UI command reaches linked native Rust code in the Tauri app.

Design:
  - Add the minimal Tauri dependency path only after B00 selects it.
  - Start the shell with no runtime/debug/Immediate/COM claims.
  - Expose a simple typed native command such as host capabilities or save/reload echo.
  - Ensure `window.__DNA_OXIDE_TEST_DRIVER__` or successor instrumentation is available inside the WebView.

Tests:
  - Tauri dev/build smoke as selected in B00.
  - UI/WebView command invocation reaches Rust and returns typed result.
  - Anti-overclaim scan.

Evidence:
  - `target/w352-b01-tauri-shell-start.txt`.

Closure:
  - [ ] Desktop shell starts.
  - [ ] UI command reaches linked native Rust.
  - [ ] Instrumentation is mounted or successor is documented.
  - [ ] No unsupported capability is claimed.

### W352-B02 — WebView automation bridge

Goal:
  Capture visual and DOM-like state from the DnaOxIde WebView and inject at least one interaction against the real desktop shell.

Design:
  - Use the selected WebView automation route.
  - Capture before/after snapshot and event log.
  - If only partial automation is possible, label the limitation precisely.
  - Do not substitute browser-only Playwright host services for desktop native command evidence.

Tests:
  - WebView inspection/injection smoke.
  - Native command result remains visible in the inspected state.
  - Anti-overclaim scan.

Evidence:
  - `target/w352-b02-webview-automation.txt`.

Closure:
  - [ ] WebView state is observable.
  - [ ] At least one interaction is injectable or limitation is documented.
  - [ ] Artifacts are written under `target/`.
  - [ ] Evidence comes from the desktop host, not a static HTML snapshot.

### W352-B03 — Tauri edit/save/reload through native Rust commands

Goal:
  Drive the W350 edit/save/reload flow through the Tauri/WebView host with save/reload handled by linked native Rust commands over temp project copies.

Design:
  - Use temp project copies.
  - Reuse W350 snapshot/event/command log semantics where still applicable.
  - Replace Playwright-injected browser host services with Tauri native commands.
  - Compare core state fields with W350 browser DOM proof artifacts only as regression reference.

Tests:
  - Desktop interaction smoke for edit/save/reload.
  - Native Rust command evidence for save/reload.
  - Fixture mutation guard.

Evidence:
  - `target/w352-b03-tauri-edit-save-reload.txt`.

Closure:
  - [ ] Desktop edit/save/reload is driven.
  - [ ] Save/reload are native Rust command backed.
  - [ ] Before/after artifacts are captured.
  - [ ] Checked-in fixtures remain unchanged.

### W352-B04 — W352 acceptance

Goal:
  Accept Tauri/WebView as the desktop product host regression lane and unblock native adapter work.

Design:
  - Run W352 checks.
  - Confirm at least one UI->Tauri->linked Rust command path.
  - Update downstream handoffs to identify desktop-host evidence separately from browser DOM/W350 harness evidence.

Tests:
  - W352 regression checks.
  - Final no-claim scan.

Evidence:
  - `target/w352-acceptance.txt`.
  - `docs/HANDOFF_W352_TAURI_WEBVIEW_AUTOMATION.md`.

Closure:
  - [ ] Desktop host execution is repeatable.
  - [ ] Evidence distinguishes WebView/native command path from browser DOM harness.
  - [ ] W355 native compile/build adapter work is unblocked.
  - [ ] No fake capability has landed.

## Out-of-scope

- Production installer/package quality.
- Full DOM accessibility audit.
- Real OxVba compile/build/runtime/debug/Immediate/COM behavior.
- COM runtime invocation.
- DnaOneCalc browser or desktop implementation.
