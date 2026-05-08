# DNA OxIde Save/Reload Command Boundary

Status: `w350_b04_implemented`
Date: 2026-05-08
Source workset: W350 — DnaOxIde Live Editable Source App

## Purpose

W350-B04 proves that the live browser DnaOxIde app can save edited module text to a test-owned temp project copy and reload that saved text back through a DnaOxIde command boundary.

This is still a browser proof. It does not claim live Tauri/WebView IPC or OxVba runtime/debug/Immediate/COM behavior.

## Command Boundary Shape

The app host can receive injected browser host services through:

```js
window.__DNA_OXIDE_HOST_SERVICES__ = {
  saveActiveModule(packet) {},
  reloadActiveModule(packet) {}
};
```

`apps/dna-oxide/src/main.js` passes this boundary into the instrumented app. Save/reload toolbar clicks call:

```js
app.runHostCommand("save-active-module", { via: "dom-click" })
app.runHostCommand("reload-active-module", { via: "dom-click" })
```

If no host services are injected, the app remains usable in the in-memory browser harness. The W350-B04 verifier injects Playwright-backed host services that write/read a temp project copy under `target/`.

## Temp Project Copy

The verifier creates and writes only under:

```text
target/w350-b04-temp-project/ThinSliceHello/
```

It reads the checked-in thin-slice fixture as the initial source but does not mutate it.

## Evidence Command

Run:

```powershell
npm --prefix apps/dna-oxide run live-save-reload:check
```

This opens the app with Playwright/Edge, injects temp-project host services, edits the source pane, saves, makes a divergent unsaved edit, reloads from the saved temp file, and writes:

```text
target/w350-b04-before-edit.json
target/w350-b04-after-edit.json
target/w350-b04-after-save.json
target/w350-b04-after-divergent-edit.json
target/w350-b04-after-reload.json
target/w350-b04-events.json
target/w350-b04-commands.json
target/w350-b04-live-save-reload.html
target/w350-b04-live-save-reload.png
target/w350-b04-saved-Module1.bas
target/w350-b04-live-save-reload.txt
```

## Assertions

The verifier asserts:

- the app bootstraps from a temp project copy;
- browser input makes the source dirty;
- save writes edited text to `target/w350-b04-temp-project/ThinSliceHello/Module1.bas`;
- save returns the app to clean state;
- a divergent unsaved edit makes the app dirty again;
- reload reads the saved temp file and discards the divergent edit;
- command and event logs record save/reload through `injected-browser-host-service`;
- checked-in `examples/thin-slice/Module1.bas` remains unchanged;
- runtime/debug/Immediate/COM/fake-data claims remain false.

## Boundaries

This bead does not claim:

- OxVba compile/build execution,
- real/native runtime execution,
- real Immediate evaluation,
- real debug/watch/breakpoint behavior,
- COM runtime invocation,
- live Tauri/WebView IPC,
- real DnaOneCalc product mount.
