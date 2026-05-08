# DNA OxIde Live Editable Proof Mode

Status: `selected_w350_b00`
Date: 2026-05-08
Source workset: W350 — DnaOxIde Live Editable Source App

## Decision

W350 will use **Option B: browser DOM + Playwright** as the primary live editable proof mode.

A separate **Option C: Tauri/WebView automation** plan is recorded and tracked as future work. Tauri/WebView automation is important for the desktop product lane, but it should not block the first live edit/save/reload feedback loop because the local Tauri CLI/tooling is not currently installed.

## Local Tool Evidence

Observed local tooling during W350-B00:

```text
node: available
npm: available
Playwright CLI: available
cargo/rustc: available
Tauri CLI: unavailable
Trunk: unavailable
```

Implication:

- Browser DOM + Playwright can start now.
- Tauri/WebView automation needs a toolchain bootstrap plan before it can be the primary driver.

## Primary Proof Mode: Browser DOM + Playwright

The first live editable DnaOxIde proof will:

1. serve or load the DnaOxIde frontend in a browser DOM context;
2. mount the DnaOxIde host shell with stable test IDs and `data-*` attributes;
3. expose an app test driver at `window.__DNA_OXIDE_TEST_DRIVER__`;
4. use Playwright to inject user-like interactions;
5. capture visual, DOM-like, command, and event evidence before and after each interaction;
6. write all proof artifacts under `target/`;
7. use temp project copies for edit/save/reload;
8. keep checked-in fixtures unchanged;
9. keep runtime/debug/Immediate/COM and DnaOneCalc mount claims false.

## Required App Test Driver

W350-B01 must provide a stable driver shape similar to:

```js
window.__DNA_OXIDE_TEST_DRIVER__ = {
  version: "w350-v1",
  snapshot() {},
  visualSnapshot() {},
  eventLog() {},
  commandLog() {},
  injectInteraction(action) {},
  resetForTest() {}
};
```

The exact shape may evolve in W350-B01, but it must support:

- focus source editor,
- type/replace source text,
- invoke save,
- invoke reload,
- invoke command palette command where available,
- capture before/after state,
- inspect no-claim flags,
- inspect dirty/saved/reloaded lifecycle state.

## Required Stable Snapshot Fields

The DOM-like snapshot must expose at least:

```text
productName
proofMode
projectName
activeModule
sourceText
sourceTextHash or length
editorFocused
dirty
lastSavedSourceText or saved hash
lastReloadedSourceText or reloaded hash
lifecycleCommandStates
lastCommand
commandLogLength
eventLogLength
noClaimFlags
```

No assertion should depend on fragile prose or list position when a stable role, command ID, module name, or data attribute exists.

## Required Artifacts

W350-B01 and later live checks should write artifacts like:

```text
target/w350-*/app.html
target/w350-*/visual.html or screenshot.png
target/w350-*/snapshot-before.json
target/w350-*/snapshot-after.json
target/w350-*/events.json
target/w350-*/commands.json
target/w350-*/summary.txt
```

A failing run should preserve enough artifacts to diagnose what the app did.

## Browser DOM + Playwright Acceptance Meaning

When W350 accepts under this proof mode, it may claim:

- browser DOM live source editing is driven;
- Playwright or an equivalent browser-DOM driver injects interactions;
- edit/save/reload works over temp project copies;
- visual and DOM-like artifacts prove before/after effects;
- command/event logs show what happened.

It must not claim:

- live Tauri/WebView execution;
- native desktop packaging readiness;
- full DOM accessibility audit;
- real/native OxVba runtime execution;
- real Immediate evaluation;
- real debug/watch/breakpoint behavior;
- COM runtime invocation;
- real DnaOneCalc product mount.

## Fallback If Playwright Cannot Drive The App

If Playwright becomes unavailable or cannot drive the local app deterministically, W350 may use a bounded DOM-like driver only if it still provides:

- real JS event dispatch into the same app model,
- before/after snapshots,
- visual artifact output,
- command/event logs,
- the same no-claim guards.

Fallback use must be explicitly documented in W350 evidence and must not be described as browser event-loop or WebView automation.

## Option C Plan Reference

Tauri/WebView automation is planned separately in:

- [`DNAOXIDE_TAURI_WEBVIEW_AUTOMATION_PLAN.md`](DNAOXIDE_TAURI_WEBVIEW_AUTOMATION_PLAN.md),
- `docs/worksets/W352_dnaoxide_tauri_webview_automation.md`.

W352 should start after W350 proves the browser DOM feedback loop or when the user explicitly prioritizes desktop WebView automation.
