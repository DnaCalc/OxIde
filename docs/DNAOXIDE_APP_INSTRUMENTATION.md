# DNA OxIde App Instrumentation

Status: `w350_b01_implemented`
Date: 2026-05-08
Source workset: W350 — DnaOxIde Live Editable Source App

## Purpose

W350-B01 adds the observability and interaction contract required before the live editable source app is accepted.

The app host path now exposes a browser-oriented test driver shape for automation and review artifacts. The primary proof mode remains browser DOM + Playwright per `docs/DNAOXIDE_LIVE_EDITABLE_PROOF_MODE.md`. The package now carries a local `@playwright/test` dev dependency so later W350 beads can add an automatic browser loop; this bead keeps a fast Node verifier for deterministic instrumentation evidence.

## Installed Driver

The DnaOxIde frontend installs the app driver at:

```js
window.__DNA_OXIDE_TEST_DRIVER__
```

Driver methods:

```js
snapshot()
eventLog()
commandLog()
visualSnapshot()
renderApp()
renderHostMarkup()
injectInteraction(interaction)
runCommand(commandName, payload)
```

## Stable Surfaces

The rendered app includes stable attributes for automation:

- `data-testid="dnaoxide-w350-app"`
- `data-testid="source-editor"`
- `data-testid="dirty-indicator"`
- `data-testid="project-panel"`
- `data-testid="instrumentation-panel"`
- `data-testid="event-count"`
- `data-testid="command-count"`
- `data-testid="last-command"`

The app root records:

- product/app identity,
- project/module identity,
- proof mode,
- dirty state,
- event/command log lengths,
- no-claim runtime/debug/Immediate/COM flags.

## Snapshot Contract

`snapshot()` includes at least:

- `productName`
- `appName`
- `proofMode`
- `projectName`
- `projectFile`
- `activeModule`
- `sourceText`
- `sourceTextLength`
- `sourceTextHash`
- `persistedSourceText`
- `persistedSourceTextLength`
- `persistedSourceTextHash`
- `editorFocused`
- `focusedSurface`
- `dirty`
- `lifecycleStatus`
- `tempProjectRoot`
- `lastCommand`
- `commandLogLength`
- `eventLogLength`
- `lifecycleCommandStates`
- `noClaimFlags`
- `instrumentation`

## Interaction Contract

`injectInteraction(...)` supports:

- `{ type: "focusEditor" }`
- `{ type: "replaceSource", text }`
- `{ type: "appendSource", text }`
- `{ type: "command", commandName, payload }`

`runCommand(...)` currently records and handles browser-harness commands for:

- `focus-editor` / `dna_oxide_focus_editor`,
- `save-active-module` / `dna_oxide_save_active_module`,
- `reload-active-module` / `dna_oxide_reload_active_module`.

Unsupported commands are logged without native dispatch and without runtime/COM/debug/Immediate claims.

## Evidence Command

Run:

```powershell
npm --prefix apps/dna-oxide run app-instrumentation:check
```

This writes:

```text
target/w350-b01-snapshot-before.json
target/w350-b01-snapshot-after.json
target/w350-b01-events.json
target/w350-b01-commands.json
target/w350-b01-app-instrumentation.html
target/w350-b01-app-markup.html
target/w350-b01-app-instrumentation.txt
```

## Boundaries

This bead does not claim:

- live Tauri/WebView IPC,
- full DOM accessibility audit,
- real OxVba runtime execution,
- real Immediate evaluation,
- real debug/watch/breakpoint behavior,
- COM runtime invocation,
- real DnaOneCalc product mount.

All runtime/debug/Immediate/COM and fake-data claim flags remain false.
