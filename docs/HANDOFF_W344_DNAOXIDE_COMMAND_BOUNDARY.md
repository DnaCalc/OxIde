# Handoff — W344 DnaOxIde Tauri Command Boundary Stubs

Status: `accepted_dnaoxide_command_boundary_stubs`
Date: 2026-05-07
Workset: [`W344_dnaoxide_tauri_command_boundary_stubs.md`](worksets/W344_dnaoxide_tauri_command_boundary_stubs.md)

## Summary

W344 establishes the first **DnaOxIde / DNA OxIde** command boundary over the W343 host bridge. The boundary is Tauri-ready and Rust-callable today, but it intentionally does not claim live Tauri/WebView IPC until W345/W346 drive that path.

The delivered commands support proven project/document/session lifecycle operations over test-owned project copies and expose typed unavailable, available-subset, or OxVba-fixture-evidenced packets for compile/options, references/COM, runtime, Immediate, debug, watch, and breakpoint paths.

## Delivered Surface

Native command crate path:

- `apps/dna-oxide/src-tauri`

Frontend command shim:

- `apps/dna-oxide/src/command-client.js`
- `apps/dna-oxide/scripts/verify-command-client.mjs`
- `npm --prefix apps/dna-oxide run command-client:check`

Command naming authority:

- [`docs/DNAOXIDE_COMMAND_BOUNDARY.md`](DNAOXIDE_COMMAND_BOUNDARY.md)

Important Rust-callable command groups:

- lifecycle: `dna_oxide_open_project_path`, `dna_oxide_load_active_module`, `dna_oxide_save_active_module`, `dna_oxide_reload_active_module`, `dna_oxide_revert_active_module`, `dna_oxide_save_session_snapshot`, `dna_oxide_load_session_snapshot`;
- capability: `dna_oxide_get_host_capabilities`;
- compile/reference/COM: `dna_oxide_get_compile_options`, `dna_oxide_apply_compile_options`, `dna_oxide_build_check`, `dna_oxide_get_references`, `dna_oxide_find_com_candidates`, `dna_oxide_apply_reference_plan`;
- runtime/Immediate/debug: `dna_oxide_run_project`, `dna_oxide_stop_runtime`, `dna_oxide_reset_runtime`, `dna_oxide_evaluate_immediate`, `dna_oxide_debug_attach`, step/continue/stop commands, watch commands, and breakpoint commands.

## Evidence States

The W344 command boundary preserves the W343 evidence split:

1. `proven-oxide-only` — enabled lifecycle/session behavior backed by OxIde tests.
2. `oxvba-available-subset` — direct OxVba surface exists, but DnaOxIde still reports disabled/subset state until adapter tests prove the command path.
3. `oxvba-fixture-evidenced` — OxVba ThinSliceHello fixture evidence exists, but DnaOxIde still reports disabled/fixture state until local adapter tests prove consumption.
4. `pending-oxvba-hardening` — DTOs, event streams, source-span mapping, command availability, native boundary, COM runtime, or UX adoption are pending.

## Claim Boundaries

W344 does **not** claim:

- live Tauri/WebView invocation from a desktop window,
- real OxVba compile/check execution from DnaOxIde,
- real runtime execution from DnaOxIde,
- real Immediate evaluation from DnaOxIde,
- real debug/watch/breakpoint data from DnaOxIde,
- COM runtime invocation,
- full source-span mapping,
- full shared capability/error taxonomy adoption,
- real DnaOneCalc host mount.

Runtime, Immediate, and debug commands return native-service-missing packets with empty responses/callstacks/locals/watches/breakpoints unless a future bead proves a real adapter.

No-claim defaults remain false:

- `real_execution_claimed`,
- `native_runtime_claimed`,
- `com_runtime_claimed`,
- `fake_immediate_responses`,
- `fake_debug_data`,
- frontend `realExecutionClaimed`, `nativeRuntimeClaimed`, `comRuntimeClaimed`, `fakeResponses`, and `fakeDebugData`.

## Acceptance Evidence

Acceptance evidence is captured in `target/w344-acceptance.txt`.

Commands run:

```powershell
npm --prefix apps/dna-oxide run command-client:check
npm --prefix apps/dna-oxide run scaffold:check
cargo test --manifest-path apps/dna-oxide/src-tauri/Cargo.toml
cargo test --manifest-path crates/Cargo.toml --workspace
```

Additional checks in the acceptance transcript:

- command/bucket token grep over docs, Rust commands, and frontend command client;
- checked-in `examples/thin-slice/Module1.bas` mutation guard;
- no direct Tauri import/global in frontend/shared UI/host bridge paths;
- anti-overclaim scan for true runtime/COM/fake-data claim tokens.

Observed non-blocking warning class:

- frozen OxVba `unexpected cfg condition name: kani` / dead-code warnings.

## Next Workset

W345 should mount the accepted shared shell through the DnaOxIde host path and consume the W344 command client/command boundary. W345 must state its proof mode precisely and must not claim live WebView IPC, runtime execution, Immediate/debug behavior, COM runtime, or full accessibility coverage unless its own tests drive those paths.
