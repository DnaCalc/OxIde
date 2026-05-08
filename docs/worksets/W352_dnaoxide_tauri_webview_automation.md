# Workset W352 — DnaOxIde Tauri/WebView Automation

## Ambition

Prove the **DNA OxIde / DnaOxIde** desktop host can run the W350-instrumented app in a real Tauri/WebView shell and can be automated enough to capture visual state, DOM-like state, command/event logs, and injected interaction effects.

W352 is the Option C plan. It is important for the desktop product lane, but it should reuse W350 instrumentation and should not block the first browser DOM + Playwright edit/save/reload feedback loop.

## Dependencies

- W350 — DnaOxIde live editable source app and instrumentation.
- [`docs/DNAOXIDE_LIVE_EDITABLE_PROOF_MODE.md`](../DNAOXIDE_LIVE_EDITABLE_PROOF_MODE.md).
- [`docs/DNAOXIDE_TAURI_WEBVIEW_AUTOMATION_PLAN.md`](../DNAOXIDE_TAURI_WEBVIEW_AUTOMATION_PLAN.md).

## Design

W352 should:

1. bootstrap Tauri/WebView tooling explicitly;
2. start DnaOxIde as a desktop app host;
3. mount the same W350-instrumented app shell;
4. automate or inspect the WebView through a documented driver;
5. capture visual, DOM-like, command, and event artifacts;
6. drive edit/save/reload parity with W350 where possible;
7. keep runtime/debug/Immediate/COM claims false unless separately proven.

If direct WebView automation is not available locally, the workset must record that limitation and stop at the highest honest proof level.

## Beads

### W352-B00 — Tauri/WebView toolchain bootstrap plan

Goal:
  Decide the exact Tauri/WebView tooling path needed for local desktop automation.

Design:
  - Inspect local Tauri, Rust, Node, WebView, and driver availability.
  - Decide whether to use Tauri CLI, npm Tauri CLI, WebDriver, WebView2 debugging, or another supported route.
  - Document installation/network requirements before adding dependencies.

Tests:
  - Tool availability command transcript.
  - Documentation grep for chosen tooling and no-claim boundaries.

Evidence:
  - `docs/DNAOXIDE_TAURI_WEBVIEW_AUTOMATION_PLAN.md` update.
  - `target/w352-b00-tauri-toolchain-plan.txt`.

Closure:
  - [ ] Tooling path is explicit.
  - [ ] Install/network requirements are visible.
  - [ ] No desktop automation claim is made before proof.

### W352-B01 — Tauri dev shell starts with W350 instrumentation

Goal:
  Launch DnaOxIde in a Tauri/WebView shell that mounts the W350-instrumented app.

Design:
  - Add the minimal Tauri dependency path only after B00 selects it.
  - Start the shell with no runtime/debug/Immediate/COM claims.
  - Ensure `window.__DNA_OXIDE_TEST_DRIVER__` or equivalent is available inside the WebView.

Tests:
  - Tauri dev/build smoke as selected in B00.
  - DOM-like driver availability check if possible.

Evidence:
  - `target/w352-b01-tauri-shell-start.txt`.

Closure:
  - [ ] Desktop shell starts.
  - [ ] W350 instrumentation is mounted.
  - [ ] No unsupported capability is claimed.

### W352-B02 — WebView automation bridge

Goal:
  Capture visual and DOM-like state from the DnaOxIde WebView and inject at least one interaction.

Design:
  - Use the selected WebView automation route.
  - Capture before/after snapshot and event log.
  - If only partial automation is possible, label the limitation precisely.

Tests:
  - WebView inspection/injection smoke.
  - Anti-overclaim scan.

Evidence:
  - `target/w352-b02-webview-automation.txt`.

Closure:
  - [ ] WebView state is observable.
  - [ ] At least one interaction is injectable or limitation is documented.
  - [ ] Artifacts are written under `target/`.

### W352-B03 — Tauri edit/save/reload parity smoke

Goal:
  Drive the W350 edit/save/reload flow through the Tauri/WebView host.

Design:
  - Use temp project copies.
  - Reuse W350 snapshot/event/command log semantics.
  - Compare core state fields with browser DOM proof artifacts.

Tests:
  - Desktop interaction smoke for edit/save/reload.
  - Fixture mutation guard.

Evidence:
  - `target/w352-b03-tauri-edit-save-reload.txt`.

Closure:
  - [ ] Desktop edit/save/reload is driven.
  - [ ] Before/after artifacts are captured.
  - [ ] Checked-in fixtures remain unchanged.

### W352-B04 — W352 acceptance

Goal:
  Accept Tauri/WebView automation as a desktop host regression lane.

Design:
  - Run W352 checks and compare against W350 browser DOM instrumentation.
  - Update downstream handoffs to identify desktop-host evidence separately from browser DOM evidence.

Tests:
  - W352 regression checks.
  - Final no-claim scan.

Evidence:
  - `target/w352-acceptance.txt`.
  - `docs/HANDOFF_W352_TAURI_WEBVIEW_AUTOMATION.md`.

Closure:
  - [ ] Desktop host automation is repeatable.
  - [ ] Evidence distinguishes WebView from browser DOM.
  - [ ] No fake capability has landed.

## Out-of-scope

- Production installer/package quality.
- Full DOM accessibility audit.
- Real OxVba runtime/debug/Immediate/COM behavior.
- COM runtime invocation.
- DnaOneCalc implementation.
