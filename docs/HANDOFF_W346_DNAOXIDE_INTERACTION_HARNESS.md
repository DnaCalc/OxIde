# Handoff — W346 DnaOxIde Interaction Harness

Status: `accepted_dnaoxide_static_interaction_harness`
Date: 2026-05-07
Workset: [`W346_dnaoxide_interaction_e2e_harness.md`](worksets/W346_dnaoxide_interaction_e2e_harness.md)

## Summary

W346 establishes the first deterministic **DnaOxIde / DNA OxIde** interaction regression base.

The accepted harness is a frontend interaction model plus static DOM/token smoke over the W345 rendered host shell. It is intentionally not a live browser/WebView driver and not a full accessibility audit.

## Harness Layer

Selected layer:

- `frontend-interaction-model+static-dom-token-smoke`

Layer authority:

- [`docs/DNAOXIDE_INTERACTION_HARNESS.md`](DNAOXIDE_INTERACTION_HARNESS.md)

Core source:

- `apps/dna-oxide/src/interaction-harness.js`

Verification scripts:

```powershell
npm --prefix apps/dna-oxide run interaction-command:check
npm --prefix apps/dna-oxide run interaction-focus:check
npm --prefix apps/dna-oxide run interaction-lifecycle:check
npm --prefix apps/dna-oxide run interaction-services:check
```

## Covered Interactions

W346 covers these deterministic flows:

- command palette open/close via `Ctrl+Shift+P`;
- keyboard routing for `Ctrl+O`, `Ctrl+S`, `Ctrl+R`, `F5`, `Ctrl+Enter`, `F9`, `F10`, `F11`, `Ctrl+F5`, and `Ctrl+Shift+C`;
- no-mouse focus route over project, editor, diagnostics, lifecycle, command palette, runtime, Immediate, debug, COM, and claim-boundary panes;
- lifecycle command sequence: open, load module, save module, reload module, save session, load session;
- blocked service command attempts for build/check, run, Immediate, debug attach, watch upsert, breakpoint set, COM candidate discovery, compile options, and runtime stop.

## Claim Boundaries

W346 does **not** claim:

- live Tauri/WebView IPC,
- browser event loop execution,
- Playwright/WebDriver click-key automation,
- full DOM accessibility audit,
- real OxVba compile/check,
- real runtime execution,
- real Immediate evaluation,
- real debug/watch/breakpoint sessions,
- COM runtime invocation,
- real DnaOneCalc host mount.

Harness no-claim booleans remain false:

- `liveTauriWebViewIpcDriven`,
- `browserEventLoopDriven`,
- `playwrightOrWebDriverDriven`,
- `fullDomAccessibilityAuditClaimed`,
- `realRuntimeExecutionClaimed`,
- `comRuntimeInvocationClaimed`.

Service/command no-claim flags remain false:

- `realExecutionClaimed`,
- `nativeRuntimeClaimed`,
- `comRuntimeClaimed`,
- `fakeResponses`,
- `fakeDebugData`.

## Evidence

Acceptance evidence is captured in `target/w346-acceptance.txt`.

Commands run:

```powershell
npm --prefix apps/dna-oxide run interaction-command:check
npm --prefix apps/dna-oxide run interaction-focus:check
npm --prefix apps/dna-oxide run interaction-lifecycle:check
npm --prefix apps/dna-oxide run interaction-services:check
npm --prefix apps/dna-oxide run host-ui:check
npm --prefix apps/dna-oxide run host-lifecycle:check
npm --prefix apps/dna-oxide run host-services:check
npm --prefix apps/dna-oxide run command-client:check
npm --prefix apps/dna-oxide run scaffold:check
cargo test --manifest-path crates/Cargo.toml --workspace
```

Additional acceptance checks:

- harness/source token grep;
- rendered/static smoke token grep over W345 host shell outputs;
- checked-in `examples/thin-slice/Module1.bas` mutation guard;
- anti-overclaim scan for live/browser/accessibility/runtime/COM/fake-data claim tokens.

Observed non-blocking warning class:

- frozen OxVba `unexpected cfg condition name: kani` / dead-code warnings.

## Next Workset

W347 should add compile-options and reference/COM placeholder/subset panels on top of the W345 host shell and W346 interaction base. It should keep compile options, reference apply, COM native boundary, and COM runtime invocation unavailable or explicitly subset/fixture labeled unless direct OxIde adapter tests prove more.
