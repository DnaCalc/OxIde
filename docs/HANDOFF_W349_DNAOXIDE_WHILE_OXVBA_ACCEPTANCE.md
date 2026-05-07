# Handoff — W349 DnaOxIde While-OxVba Acceptance

Status: `accepted`
Date: 2026-05-07
Source workset: W349 — DnaOxIde While-OxVba Acceptance

## Summary

W349 accepts the OxIde-side W341-W348 continuation runway for **DNA OxIde / DnaOxIde** while OxVba continues or hardens direct host APIs.

The accepted runway includes:

1. W341 — DnaOxIde Tauri-ready app scaffold.
2. W342 — reusable `oxide-ui-leptos` shared IDE UI layer.
3. W343 — host-neutral `oxide-host-bridge` facade.
4. W344 — DnaOxIde Rust-callable/Tauri-ready command boundary stubs.
5. W345 — DnaOxIde static frontend host UI proof.
6. W346 — DnaOxIde frontend interaction model/static DOM token harness.
7. W347 — compile/options/reference/COM placeholder panels.
8. W348 — OxIde-only DnaOneCalc shared UI reuse path.

This is a consolidation acceptance, not a full runtime/debug/COM capability claim.

## Evidence

Primary W349 evidence:

- `target/w349-evidence-audit.txt`,
- `target/w349-regression.txt`,
- `target/w349-readiness-report.txt`,
- `target/w349-acceptance.txt`,
- `docs/DNAOXIDE_OXVBA_INTEGRATION_READINESS.md`.

Regression evidence from W349-B01:

```powershell
cargo test --manifest-path crates/Cargo.toml --workspace
cargo test --manifest-path apps/dna-oxide/src-tauri/Cargo.toml
npm --prefix apps/dna-oxide run scaffold:check
npm --prefix apps/dna-oxide run command-client:check
npm --prefix apps/dna-oxide run host-ui:check
npm --prefix apps/dna-oxide run host-lifecycle:check
npm --prefix apps/dna-oxide run host-services:check
npm --prefix apps/dna-oxide run interaction-command:check
npm --prefix apps/dna-oxide run interaction-focus:check
npm --prefix apps/dna-oxide run interaction-lifecycle:check
npm --prefix apps/dna-oxide run interaction-services:check
npm --prefix apps/dna-oxide run compile-panels:check
npm --prefix apps/dna-oxide run reference-panels:check
npm --prefix apps/dna-oxide run placeholder-commands:check
node tools/verify-dnaonecalc-profile.mjs
node tools/verify-dnaonecalc-reuse.mjs
```

Accepted GUI-lab render checks:

```powershell
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-shared-ui-shell-component
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-host-bridge-command-dispatch
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-dnaonecalc-web-shell-host-contract
```

## Claim Boundaries

Still unclaimed in OxIde/DnaOxIde after W349:

- live Tauri/WebView IPC execution,
- browser event-loop click/key automation or Playwright/WebDriver coverage,
- full DOM accessibility audit,
- real/native OxVba runtime execution in DnaOxIde,
- real Immediate Window evaluation from DnaOxIde,
- real debug/watch/breakpoint panes from DnaOxIde,
- COM runtime invocation,
- real DnaOneCalc product mount,
- sibling repo writes,
- fake runtime/Immediate/debug rows as substitutes for missing services.

W349 evidence keeps true-token claim scans clear for native runtime, COM runtime, COM runtime invocation, fake responses, fake debug data, DOM audit, DnaOneCalc host mount, and sibling writes.

## OxVba Readiness Position

OxVba has available-subset and ThinSliceHello fixture-evidenced direct Rust surfaces for the DNA OxIde sequence:

1. W355 compile/build UX,
2. W360 COM/reference/native boundary,
3. W365 runtime + Immediate,
4. W370 debug/watch/breakpoints.

OxIde has UI surfaces, command stubs, host bridge states, and verification harnesses ready for adapter work. Full claims remain gated on OxIde-side direct adapter tests over OxVba DTOs/APIs, using temp project copies and no CLI/LSP fallback for core semantics.

## Recommended Next Step

Start a new OxIde direct-adapter workset only if the OxVba direct APIs are stable enough to consume locally:

1. compile/build adapter over project options, run targets, request/event/build result DTOs;
2. COM/reference adapter over roster, candidate, repair/reorder, capability, and runtime-availability DTOs;
3. runtime/Immediate adapter over runtime sessions and Immediate request/response DTOs;
4. debug/watch/breakpoint adapter over debug session, command status, frame/local/watch/breakpoint/source mapping DTOs.

If adapter work is not authorized or not ready, continue DnaOxIde packaging/polish only and keep all runtime/debug/Immediate/COM claim flags false.
