# Handoff — W345 DnaOxIde Host UI Proof

Status: `accepted_dnaoxide_static_host_ui_proof`
Date: 2026-05-07
Workset: [`W345_dnaoxide_live_host_ui_proof.md`](worksets/W345_dnaoxide_live_host_ui_proof.md)

## Summary

W345 makes **DnaOxIde / DNA OxIde** mount a reviewable host shell through the DnaOxIde frontend path using the selected static frontend host fixture proof mode.

This is a host UI proof, not a live desktop IPC proof. It renders DNA OxIde branding, the ThinSliceHello / `Module1.bas` project view, lifecycle state, command availability, and runtime/Immediate/debug/COM disabled states while preserving W343/W344 evidence labels and no-claim attributes.

## Proof Mode

Selected proof mode:

- `static-frontend-host-fixture`

Mode authority:

- [`docs/DNAOXIDE_HOST_UI_PROOF_MODE.md`](DNAOXIDE_HOST_UI_PROOF_MODE.md)

Driven in W345:

- `apps/dna-oxide/src/main.js` mounts into `#dna-oxide-root`;
- `apps/dna-oxide/src/host-shell.js` renders the DnaOxIde host shell over W342/W343/W344 contract labels;
- `apps/dna-oxide/src/command-client.js` supplies W344 command buckets;
- host UI render includes project, editor, diagnostics, lifecycle, command palette, runtime, Immediate, debug, COM, and no-claim boundary panes.

Not driven or claimed:

- live Tauri/WebView IPC,
- Playwright/WebDriver click-key automation,
- full DOM accessibility audit,
- real OxVba compile/check,
- real runtime execution,
- real Immediate evaluation,
- real debug/watch/breakpoint session data,
- COM runtime invocation,
- real DnaOneCalc host mount.

## Delivered Surface

Frontend/static proof files:

- `apps/dna-oxide/src/host-shell.js`
- `apps/dna-oxide/src/main.js`
- `apps/dna-oxide/scripts/verify-host-ui.mjs`
- `apps/dna-oxide/scripts/verify-host-lifecycle.mjs`
- `apps/dna-oxide/scripts/verify-host-services.mjs`

NPM scripts:

```powershell
npm --prefix apps/dna-oxide run host-ui:check
npm --prefix apps/dna-oxide run host-lifecycle:check
npm --prefix apps/dna-oxide run host-services:check
```

Rendered evidence files generated under `target/`:

- `target/w345-host-shell-render.html`
- `target/w345-host-lifecycle-proof.html`

## Visible Host UI Proof

The accepted render includes:

- `role="dnaoxide-host-ui-proof"`,
- `data-proof-mode="static-frontend-host-fixture"`,
- `data-shared-ui-crate="oxide-ui-leptos"`,
- `data-host-bridge-crate="oxide-host-bridge"`,
- `DNA OxIde`,
- `ThinSliceHello`,
- `Module1.bas`,
- `role="host-project-spine"`,
- `role="host-editor-boundary"`,
- `role="host-diagnostics-panel"`,
- `role="host-lifecycle-panel"`,
- `role="host-command-palette"`,
- runtime, Immediate, debug, and COM panels.

Command/service evidence labels remain visible:

- `proven-oxide-only`,
- `oxvba-available-subset`,
- `oxvba-fixture-evidenced`,
- `pending-oxvba-hardening`,
- `native-service-missing`.

## Lifecycle Proof

The lifecycle proof uses target-owned proof files and a checked-in fixture mutation guard. It shows:

- `data-provider="proven-oxide-only-temp-copy"`,
- `data-dirty="true"`,
- `opened-temp-project-copy`,
- `saved-working-source-to-temp-copy`,
- `reloaded-module-from-temp-copy`,
- `session-snapshot-restored-from-temp-copy`,
- `checked-in-fixture-unchanged`.

The verifier reads `examples/thin-slice/Module1.bas`, writes expected target proof files only when absent or identical, and refuses to overwrite differing source/edited proof files.

## Runtime / Immediate / Debug / COM Boundaries

W345 keeps service panes empty and explicit:

- runtime: `data-output-events="0"`, empty runtime ID;
- Immediate: `data-immediate-responses="0"`, empty Immediate session ID;
- debug: `data-callstack-frames="0"`, `data-locals="0"`, `data-watches="0"`, `data-breakpoints="0"`, empty debug session ID;
- COM: `data-com-candidates="0"`, `data-com-runtime-invocation="false"`.

No fake responses, fake debug data, real execution, native runtime, or COM runtime are claimed.

## Acceptance Evidence

Acceptance evidence is captured in `target/w345-acceptance.txt`.

Commands run:

```powershell
npm --prefix apps/dna-oxide run host-ui:check
npm --prefix apps/dna-oxide run host-lifecycle:check
npm --prefix apps/dna-oxide run host-services:check
npm --prefix apps/dna-oxide run command-client:check
npm --prefix apps/dna-oxide run scaffold:check
cargo test --manifest-path crates/Cargo.toml -p oxide-ui-leptos
cargo test --manifest-path crates/Cargo.toml -p oxide-host-bridge
```

Additional acceptance checks:

- rendered-token grep over host shell and lifecycle proof output;
- proof-mode/source token grep;
- checked-in `examples/thin-slice/Module1.bas` mutation guard;
- no direct Tauri import/global in frontend/shared UI/host bridge paths;
- anti-overclaim scan for true runtime/COM/fake-data/live-proof claim tokens.

Observed non-blocking warning class:

- frozen OxVba `unexpected cfg condition name: kani` / dead-code warnings.

## Next Workset

W346 should add the first deterministic interaction/e2e harness over this static host proof. It should begin with the lightest reliable layer and must keep claim boundaries explicit: no full accessibility audit, no live browser/WebView runtime claim, and no real OxVba runtime/debug/Immediate/COM claim unless directly driven and tested.
