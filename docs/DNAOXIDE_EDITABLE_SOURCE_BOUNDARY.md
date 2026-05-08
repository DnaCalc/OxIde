# DNA OxIde Editable Source Boundary

Status: `w350_b02_implemented`
Date: 2026-05-08
Source workset: W350 — DnaOxIde Live Editable Source App

## Purpose

W350-B02 adds a host-neutral editable source model boundary for the basic DnaOxIde app. The boundary receives source input events, exposes dirty lifecycle state, and reports save/reload command availability without importing Tauri globals or claiming OxVba semantic/runtime truth.

The browser app uses this boundary through `apps/dna-oxide/src/app-instrumentation.js`, so the instrumentation snapshot now carries the same source lifecycle state that the editable pane will render and automate.

## Boundary File

```text
apps/dna-oxide/src/editable-source-boundary.js
```

Public shape:

```js
createEditableSourceBoundary(options)
verifyEditableSourceBoundaryContract()
lifecycleCommandStates(isDirty)
stableSourceHash(text)
```

A boundary instance exposes:

```js
snapshot()
replaceSource(text, metadata)
appendSource(text, metadata)
applyInputEvent(event, metadata)
saveToPersisted(metadata)
reloadFromPersisted(metadata)
revertToPersisted(metadata)
```

## Snapshot Fields

The source boundary snapshot includes:

- `projectName`
- `projectFile`
- `activeModule`
- `sourceText`
- `sourceTextLength`
- `sourceTextHash`
- `persistedSourceText`
- `persistedSourceTextLength`
- `persistedSourceTextHash`
- `lastReloadedSourceText`
- `lastReloadedSourceTextHash`
- `dirty`
- `lifecycleStatus`
- `editRevision`
- `savedRevision`
- `reloadedRevision`
- `commandStates`
- `noClaimFlags`
- `boundary`

## Command State Contract

The lifecycle command state is explicit and stable:

- `saveActiveModule`: `enabled-dirty` or `enabled-clean-noop`
- `reloadActiveModule`: `enabled-temp-copy`
- `revertActiveModule`: `enabled-dirty` or `enabled-clean-noop`
- runtime/debug/Immediate/COM commands remain unavailable/no-claim.

## Host-Neutrality

The boundary is plain JavaScript model code. It must not contain:

- `@tauri-apps`,
- `__TAURI__`,
- direct `invoke(` calls,
- DnaOneCalc product-host code,
- OxVba semantic/runtime/debug/COM execution code.

## Rust Core Reuse

The verifier also runs Rust editor/lifecycle core tests as a compatibility smoke:

```powershell
cargo test --manifest-path crates/Cargo.toml -p oxide-editor-core
cargo test --manifest-path crates/Cargo.toml -p oxide-core
```

This keeps the W350 browser model aligned with the existing OxIde editor/lifecycle concepts while avoiding direct shared-UI or Tauri coupling.

## Evidence Command

Run:

```powershell
npm --prefix apps/dna-oxide run editable-source-boundary:check
```

This writes:

```text
target/w350-b02-editable-source-before.json
target/w350-b02-editable-source-after-edit.json
target/w350-b02-editable-source-after-save.json
target/w350-b02-editable-source-after-reload.json
target/w350-b02-instrumented-app-source-snapshot.json
target/w350-b02-editable-source-boundary.txt
```

## Boundaries

This bead does not claim:

- live Tauri/WebView IPC,
- OxVba semantic/project/runtime/debug/COM truth,
- real Immediate evaluation,
- real debug/watch/breakpoint behavior,
- COM runtime invocation,
- real DnaOneCalc product mount.
