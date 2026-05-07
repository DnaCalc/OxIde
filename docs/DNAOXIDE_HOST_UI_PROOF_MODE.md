# DNA OxIde Host UI Proof Mode

Status: `w345_static_frontend_host_fixture_selected`
Date: 2026-05-07
Workset: [`W345_dnaoxide_live_host_ui_proof.md`](worksets/W345_dnaoxide_live_host_ui_proof.md)

## Decision

The first W345 proof mode is a **static frontend host fixture** mounted by `apps/dna-oxide/src/main.js` in the DnaOxIde app path.

This mode is selected because the app currently has a Tauri-ready Rust command boundary and a frontend command client, but it does not yet have a real Tauri/WebView runtime dependency or an e2e browser driver. The proof should therefore make the DnaOxIde host shell reviewable without pretending that desktop IPC, WebView automation, or real OxVba runtime/debug/Immediate/COM execution has been driven.

## What Is Driven

The W345 static host fixture drives:

- DnaOxIde frontend entry mounting into `#dna-oxide-root`;
- DNA OxIde branding and app-specific host chrome;
- a ThinSliceHello / `Module1.bas` host shell model;
- frontend command-client bucket projection from W344;
- lifecycle command availability for proven OxIde-only paths;
- disabled/subset/fixture labels for compile/reference/COM/runtime/Immediate/debug/watch/breakpoint commands;
- native-service-missing and pending-hardening text in visible panes;
- no-claim data attributes for runtime, native runtime, COM runtime, fake Immediate responses, and fake debug data.

## What Is Not Driven

This proof mode does **not** drive or claim:

- live Tauri/WebView IPC;
- Playwright/WebDriver/browser click-key automation;
- full DOM accessibility audit;
- real OxVba compile/check execution;
- real runtime execution;
- real Immediate evaluation;
- real debug/watch/breakpoint session data;
- COM runtime invocation;
- real DnaOneCalc host mount.

## Shared UI Boundary

The static frontend host fixture is app glue around the W342/W343/W344 contracts, not a new owner of shared UI truth:

- shared Rust components remain in `crates/oxide-ui-leptos`;
- host bridge command availability remains in `crates/oxide-host-bridge`;
- DnaOxIde command names and frontend client buckets remain in `apps/dna-oxide/src/command-client.js`;
- the static frontend renderer must preserve the same evidence labels and no-claim flags so W345 can be reviewed before live IPC/e2e work begins.

W345-B01/B02/B03 may add a frontend renderer for this proof, but it must stay thin and host-specific. Reusable/shared UI architecture remains in W342/W343 crates and future app/wasm integration work.

## Verification Commands

Planned W345 proof commands:

```powershell
npm --prefix apps/dna-oxide run host-ui:check
npm --prefix apps/dna-oxide run command-client:check
npm --prefix apps/dna-oxide run scaffold:check
cargo test --manifest-path crates/Cargo.toml -p oxide-ui-leptos
cargo test --manifest-path crates/Cargo.toml -p oxide-host-bridge
```

Acceptance should also run anti-overclaim greps and a fixture mutation guard for `examples/thin-slice/Module1.bas`.
