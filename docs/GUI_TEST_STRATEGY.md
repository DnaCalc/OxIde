# OxIde GUI Test Strategy

Status: `first_pass_test_strategy`
Date: 2026-05-07

## Purpose

This note defines the initial testing posture for the GUI pivot.

The fixture and GUI lab seed plan is [`GUI_FIXTURES_AND_LAB.md`](GUI_FIXTURES_AND_LAB.md).

## Test Layers

1. Pure Rust unit tests
   - editor buffer behavior,
   - cursor and selection movement,
   - undo/redo,
   - command reducer behavior,
   - capability model behavior,
   - DTO serialization,
   - project/session view-model construction.

2. OxVba contract tests
   - workspace load,
   - document mapping,
   - diagnostics,
   - hover,
   - references,
   - build/run,
   - immediate,
   - unsupported COM profile behavior.

3. WASM/browser tests
   - use the DnaOneCalc browser-test approach as an exemplar,
   - include `wasm-bindgen-test` and deterministic component smoke tests where appropriate.

4. Browser visual/scenario tests
   - create an `oxide-guilab` scenario catalogue early,
   - use DOM text snapshots, screenshots, or accessibility snapshots as appropriate.

5. Host capability matrix tests
   - browser WASM: COM unavailable,
   - desktop non-Windows: Windows COM unavailable,
   - Windows native: COM available only through native host service.

6. DnaOneCalc integration smoke
   - eventually prove DnaOneCalc can consume OxIde bridge/component/artifact without owning OxIde semantics.

## DnaOxIde Static Interaction Harness

W346 adds the first DnaOxIde host interaction regression layer:

```powershell
npm --prefix apps/dna-oxide run interaction-command:check
npm --prefix apps/dna-oxide run interaction-focus:check
npm --prefix apps/dna-oxide run interaction-lifecycle:check
npm --prefix apps/dna-oxide run interaction-services:check
```

This layer is a frontend interaction model plus static DOM/token smoke over the W345 rendered host shell. It covers command palette/keyboard routing, no-mouse focus route, lifecycle command sequence, and blocked runtime/Immediate/debug/COM command attempts.

It does not claim live Tauri/WebView IPC, browser event-loop automation, Playwright/WebDriver coverage, full DOM accessibility audit, real OxVba runtime/debug/Immediate/COM behavior, or COM runtime invocation.

## TUI Tests

Existing WTD tests remain useful for the parked TUI lane. They are not the GUI default regression loop.
