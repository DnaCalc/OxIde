# DNA OxIde Interaction Harness

Status: `w346_static_dom_and_frontend_interaction_model_selected`
Date: 2026-05-07
Workset: [`W346_dnaoxide_interaction_e2e_harness.md`](worksets/W346_dnaoxide_interaction_e2e_harness.md)

## Decision

The first W346 harness layer is a **frontend interaction model plus static DOM/token smoke over the W345 rendered host shell**.

This layer is intentionally lighter than a live browser/WebView driver. It can run now with Node and the existing Rust/NPM checks, and it is deterministic enough to become the first host regression base while Tauri/WebView and Playwright/WebDriver automation remain future work.

## What The Harness Drives

The initial harness drives deterministic frontend interaction state for:

- command palette open/close;
- keyboard shortcut routing to W344 command names;
- command availability and disabled reasons from the W344 command client;
- no-mouse focus route over W345 host panes;
- lifecycle command sequence over target-owned proof files;
- blocked runtime, Immediate, debug, watch, breakpoint, and COM/reference command attempts.

It also performs static DOM/token smoke checks over the W345 rendered host shell output:

- `target/w345-host-shell-render.html`,
- `target/w345-host-lifecycle-proof.html`.

## What The Harness Does Not Drive

W346-B00 does **not** select or claim:

- live Tauri/WebView IPC;
- browser event loop execution;
- Playwright/WebDriver click/key automation;
- full DOM accessibility audit;
- real OxVba compile/check;
- real runtime execution;
- real Immediate evaluation;
- real debug/watch/breakpoint sessions;
- COM runtime invocation;
- real DnaOneCalc mount.

## First Flows

B01 command/keyboard:

- open and close command palette;
- route `Ctrl+Shift+P`, `Ctrl+O`, `Ctrl+S`, `F5`, and debug/Immediate shortcuts to stable command names;
- assert disabled runtime/debug/COM commands remain disabled with labels.

B02 focus/no-mouse:

- walk project, editor, diagnostics, lifecycle, command palette, runtime, Immediate, debug, COM, and claim boundary panes;
- verify labels exist in the W345 static render;
- do not claim full accessibility compliance.

B03 lifecycle:

- exercise open/save/reload/session command sequence in the harness model;
- use target-owned lifecycle proof files;
- keep checked-in fixture mutation guard.

B04 blocked/subset/fixture services:

- trigger runtime, Immediate, debug, watch, breakpoint, references, and COM commands;
- assert `oxvba-available-subset`, `oxvba-fixture-evidenced`, or `pending-oxvba-hardening` labels;
- assert empty service counts and false no-claim flags.

## Verification Commands

Planned W346 commands:

```powershell
npm --prefix apps/dna-oxide run interaction-command:check
npm --prefix apps/dna-oxide run interaction-focus:check
npm --prefix apps/dna-oxide run interaction-lifecycle:check
npm --prefix apps/dna-oxide run interaction-services:check
npm --prefix apps/dna-oxide run host-ui:check
```

Acceptance should also run anti-overclaim greps and the checked-in fixture mutation guard.
