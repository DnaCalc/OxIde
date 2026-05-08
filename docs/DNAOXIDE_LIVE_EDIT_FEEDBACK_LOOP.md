# DNA OxIde Live Edit Feedback Loop

Status: `w350_b05_implemented`
Date: 2026-05-08
Source workset: W350 — DnaOxIde Live Editable Source App

## Purpose

W350-B05 adds the repeatable automatic test loop for the basic live editable DnaOxIde app.

The loop verifies the instrumentation contract, editable source boundary, Playwright browser input, and save/reload through the temp-project command boundary. It then audits the produced artifacts instead of trusting command success alone.

## Command

Run:

```powershell
npm --prefix apps/dna-oxide run live-edit:check
```

This runs:

```text
app-instrumentation:check
editable-source-boundary:check
live-host-mount:check
live-save-reload:check
```

Then it verifies the actual artifacts for:

- browser DOM input changed source text;
- dirty state moved clean → dirty;
- save moved dirty → clean;
- divergent unsaved edit moved clean → dirty;
- reload restored saved temp-file text and returned dirty → clean;
- command and event logs contain save/reload;
- visual HTML/PNG artifacts exist and are non-empty;
- checked-in thin-slice fixture remains unchanged;
- no runtime/debug/Immediate/COM/fake-data overclaim tokens are present.

## Evidence

The loop writes:

```text
target/w350-b05-live-edit-feedback-loop-commands.txt
target/w350-b05-live-edit-feedback-loop.txt
```

It also verifies the B01-B04 artifacts, including:

```text
target/w350-b03-live-editable-host.html
target/w350-b03-live-editable-host.png
target/w350-b04-live-save-reload.html
target/w350-b04-live-save-reload.png
target/w350-b04-saved-Module1.bas
```

## Meaning of Green

A passing `live-edit:check` means the current browser proof can be reviewed as a basic live source app and can be automatically driven through:

```text
render app → type source text → observe dirty → save temp copy → edit divergently → reload saved text → inspect visual/DOM/log artifacts
```

## Boundaries

This loop does not claim:

- live Tauri/WebView IPC,
- native desktop packaging readiness,
- real OxVba compile/build/runtime/debug/Immediate/COM behavior,
- real DnaOneCalc product mount,
- full DOM accessibility audit.
