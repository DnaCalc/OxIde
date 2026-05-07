# Handoff — W348 DnaOneCalc Shared UI Reuse Path

Status: `accepted_oxide_only`
Date: 2026-05-07
Source workset: W348 — DnaOneCalc Shared UI Reuse Path

## Summary

W348 proves, inside the OxIde repo only, that the shared IDE UI and host bridge route can be consumed by a future DnaOneCalc host without depending on the DnaOxIde app scaffold.

This is not a real DnaOneCalc product mount. No write to `C:/Work/DnaCalc/DnaOneCalc` is authorized or claimed by this handoff.

## Reuse Path

The accepted reuse path is:

```text
DnaOneCalc product shell / host policy
  -> DnaOneCalcWebShellHostPacket
  -> GuiShellPacket + RuntimeServicePacket + ImmediateServicePacket + DebugServicePacket
  -> oxide-host-bridge command/capability state
  -> oxide-ui-leptos shared IDE surface
  -> oxide-webshell static web boundary where needed
```

OxIde-owned proof files:

- `docs/DNAONECALC_SHARED_UI_REUSE_PROOF.md`,
- `docs/fixtures/dnaonecalc-consumer-profile.json`,
- `tools/verify-dnaonecalc-profile.mjs`,
- `tools/verify-dnaonecalc-reuse.mjs`,
- `docs/HANDOFF_DNAONECALC_WEB_SHELL_HOST_API.md` W348 refresh section.

## Evidence

W348 evidence artifacts:

- `target/w348-b00-reuse-proof-design.txt`,
- `target/w348-dnaonecalc-profile-tests.txt`,
- `target/w348-dnaonecalc-reuse-render.txt`,
- `target/w348-dnaonecalc-web-shell-host-contract.html`,
- `target/w348-shared-ui-shell-component.html`,
- `target/w348-handoff-refresh.txt`,
- `target/w348-acceptance.txt`.

Acceptance commands:

```powershell
node tools/verify-dnaonecalc-profile.mjs
node tools/verify-dnaonecalc-reuse.mjs
cargo test --manifest-path crates/Cargo.toml -p oxide-ui-leptos
cargo test --manifest-path crates/Cargo.toml -p oxide-host-bridge
```

## Boundaries Preserved

- DnaOneCalc owns product shell, host placement, host policy, and persistence policy.
- OxIde owns IDE surface, shared UI packet rendering, and host bridge contracts.
- OxVba owns VBA project/language/runtime/Immediate/debug/COM truth.
- DnaOxIde owns only the standalone DNA OxIde host scaffold and Tauri packaging lane.

No W348 evidence claims:

- sibling DnaOneCalc repo writes,
- real DnaOneCalc product mount,
- live DnaOneCalc browser host smoke,
- live Tauri/WebView IPC,
- real OxVba runtime/debug/Immediate/COM execution,
- COM runtime invocation,
- fake runtime/Immediate/debug data,
- full DOM accessibility audit.

## Next Authorization Gate

A real DnaOneCalc host mount requires an explicit user authorization to write to `C:/Work/DnaCalc/DnaOneCalc`, followed by paired DnaOneCalc evidence. Until then, W348 remains an OxIde-only reuse proof.
