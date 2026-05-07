# DnaOneCalc Shared UI Reuse Proof

Status: `w348_oxide_only_dnaonecalc_reuse_proof_selected`
Date: 2026-05-07
Workset: [`W348_dnaonecalc_shared_ui_reuse_path.md`](worksets/W348_dnaonecalc_shared_ui_reuse_path.md)

## Decision

W348 will prove DnaOneCalc reuse **inside the OxIde repo only** using:

1. an OxIde-owned `dnaonecalc-consumer` profile fixture, and
2. the existing GUI-lab DnaOneCalc web shell host contract render.

This proof must not write to `C:/Work/DnaCalc/DnaOneCalc` and must not claim a real DnaOneCalc product mount.

## Reused OxIde Pieces

The reuse proof consumes or verifies these OxIde-owned surfaces:

- `crates/oxide-ui-leptos` — shared IDE surface renderer;
- `crates/oxide-host-bridge` — host-neutral command/capability facade;
- `crates/oxide-core` — `GuiShellPacket`, runtime/Immediate/debug service packets, and DnaOneCalc packet contracts;
- `crates/oxide-webshell` — static web shell adapter boundary;
- `crates/oxide-guilab` — deterministic render scenarios including `gui-dnaonecalc-web-shell-host-contract`.

## Ownership Boundaries

- DnaOneCalc owns product shell, host policy, formula workflow, persistence policy, and where an embedded OxIde appears.
- OxIde owns the IDE surface, editor/project/lifecycle UX, shared UI components, and host bridge contracts.
- OxVba owns VBA project/language/runtime/debug/Immediate/COM truth.

## Claim Boundaries

This proof does **not** claim:

- writes to the DnaOneCalc sibling repo;
- a real DnaOneCalc host mount;
- DnaOneCalc browser host smoke;
- live Tauri/WebView IPC;
- real OxVba runtime/debug/Immediate/COM behavior;
- COM runtime invocation;
- full DOM accessibility audit.

No-claim tokens must remain visible:

- `data-sibling-repo-writes="false"`,
- `data-host-mount-claimed="false"`,
- `data-native-runtime="false"`,
- `data-com-runtime="false"`,
- `data-dom-audited="false"`.

## Verification Commands

Planned W348 commands:

```powershell
node tools/verify-dnaonecalc-reuse.mjs
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-dnaonecalc-web-shell-host-contract
cargo test --manifest-path crates/Cargo.toml -p oxide-ui-leptos
cargo test --manifest-path crates/Cargo.toml -p oxide-host-bridge
```
