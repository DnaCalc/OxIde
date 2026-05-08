# DNA OxIde Live Editable Host Mount

Status: `w350_b03_implemented`
Date: 2026-05-08
Source workset: W350 — DnaOxIde Live Editable Source App

## Purpose

W350-B03 mounts the DnaOxIde browser host as a live editable source app. The source pane is now a real `<textarea>` wired to the W350 editable source boundary and app instrumentation. Browser DOM input changes the model, updates dirty state, and is visible in snapshots, event logs, HTML, and screenshots.

## Mounted App Path

```text
apps/dna-oxide/index.html
apps/dna-oxide/src/main.js
```

`src/main.js` mounts the instrumented app and wires DOM events:

- `input` on `data-testid="source-editor"` calls `injectInteraction({ type: "replaceSource", via: "dom-input" })`.
- focus and toolbar button clicks are routed through the app driver.
- re-rendering preserves stable `data-testid` and no-claim attributes.

## Reviewable Result

Run:

```powershell
npm --prefix apps/dna-oxide run live-host-mount:check
```

This opens the app with Playwright/Edge in headless browser mode, types into the source editor, and writes:

```text
target/w350-b03-live-host-before.json
target/w350-b03-live-host-after-input.json
target/w350-b03-live-host-events.json
target/w350-b03-live-host-commands.json
target/w350-b03-live-editable-host.html
target/w350-b03-live-editable-host.png
target/w350-b03-live-editable-host.txt
```

The `.html` and `.png` artifacts are the reviewable basic app view for this bead.

## Assertions

The verifier asserts:

- `window.__DNA_OXIDE_TEST_DRIVER__` is present,
- `data-testid="source-editor"` is present,
- browser `fill(...)` changes the model source text,
- dirty state flips from `false` to `true`,
- the dirty indicator updates in rendered DOM,
- the event log records `source-replaced` with `via: "dom-input"`,
- runtime/debug/Immediate/COM/fake-data claims remain false,
- checked-in thin-slice fixture mutation is guarded by the wrapper evidence command.

## Boundaries

This bead does not claim:

- save/reload filesystem persistence yet,
- live Tauri/WebView IPC,
- real OxVba runtime execution,
- real Immediate evaluation,
- real debug/watch/breakpoint behavior,
- COM runtime invocation,
- real DnaOneCalc product mount.
