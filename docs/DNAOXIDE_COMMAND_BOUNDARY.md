# DNA OxIde Command Boundary Contract

Status: `w344_command_boundary_contract`
Date: 2026-05-07
Host app: **DNA OxIde** / `DnaOxIde`
Upstream facade: [`docs/HANDOFF_W343_HOST_BRIDGE_FACADE.md`](HANDOFF_W343_HOST_BRIDGE_FACADE.md)

## Purpose

This contract names the first DnaOxIde command boundary over the accepted W343 host bridge.

The command boundary is Tauri-ready but not Tauri-specific. Rust-callable command functions can exist before a live WebView IPC path. Shared UI still targets `oxide-host-bridge` concepts; DnaOxIde owns app packaging and native host policy.

## Bucket Labels

Every command must report exactly one of these evidence buckets:

1. `proven-oxide-only` — command can be implemented from OxIde-owned state/evidence.
2. `oxvba-available-subset` — OxVba direct Rust surface exists, but DnaOxIde adapter proof is partial or disabled.
3. `oxvba-fixture-evidenced` — OxVba ThinSliceHello fixture evidence exists, but DnaOxIde adapter proof is still required before enabling real behavior.
4. `pending-oxvba-hardening` — DTOs, event streams, source spans, command availability, native boundary, or UX adoption are still pending.

No command may synthesize fake runtime, Immediate, debug, watch, breakpoint, or COM data.

## Command Table

| DnaOxIde command | Host bridge command | Host API | W344 bucket | Initial behavior |
| --- | --- | --- | --- | --- |
| `dna_oxide_get_host_capabilities` | `capability.show` | `HostCapabilityApi` | `proven-oxide-only` | Return capability/profile rows from host bridge state. |
| `dna_oxide_open_project_path` | `project.open` | `HostProjectApi` | `proven-oxide-only` | Open an explicit path or test-owned fixture copy through OxIde project shell state. |
| `dna_oxide_inspect_project` | `project.inspect` | `HostProjectApi` | `oxvba-available-subset` | Disabled/subset-labeled until direct OxVba adapter is wired. |
| `dna_oxide_load_active_module` | `project.inspect` / `document.reload` | `HostProjectApi` / `HostDocumentApi` | `proven-oxide-only` | Load active module state from proven fixture/temp project path. |
| `dna_oxide_save_active_module` | `document.save` | `HostDocumentApi` | `proven-oxide-only` | Save source to a test-owned/temp project copy only. |
| `dna_oxide_reload_active_module` | `document.reload` | `HostDocumentApi` | `proven-oxide-only` | Reload source from a test-owned/temp project copy only. |
| `dna_oxide_revert_active_module` | `document.revert` | `HostDocumentApi` | `proven-oxide-only` | Revert in-memory working source without mutating checked-in fixtures. |
| `dna_oxide_save_session_snapshot` | `document.save` | `HostDocumentApi` | `proven-oxide-only` | Save OxIde session snapshot to test-owned/native path. |
| `dna_oxide_load_session_snapshot` | `document.reload` | `HostDocumentApi` | `proven-oxide-only` | Load OxIde session snapshot from test-owned/native path. |
| `dna_oxide_language_diagnostics` | `language.diagnostics` | `HostLanguageApi` | `oxvba-available-subset` | Disabled/subset-labeled until direct adapter is wired. |
| `dna_oxide_language_hover` | `language.hover` | `HostLanguageApi` | `oxvba-available-subset` | Disabled/subset-labeled until direct adapter is wired. |
| `dna_oxide_language_definition` | `language.definition` | `HostLanguageApi` | `oxvba-available-subset` | Disabled/subset-labeled until direct adapter is wired. |
| `dna_oxide_language_references` | `language.references` | `HostLanguageApi` | `oxvba-available-subset` | Disabled/subset-labeled until direct adapter is wired. |
| `dna_oxide_get_compile_options` | `compile.options` | `HostCompileApi` | `pending-oxvba-hardening` | Return typed unavailable/pending state. |
| `dna_oxide_apply_compile_options` | `compile.options` | `HostCompileApi` | `pending-oxvba-hardening` | Return typed unavailable/pending state. |
| `dna_oxide_build_check` | `compile.check` | `HostCompileApi` | `oxvba-fixture-evidenced` | Disabled/fixture-labeled until DnaOxIde adapter test proves `EmbeddedBuildRunHost::build_workspace`. |
| `dna_oxide_get_references` | `references.show` | `HostReferenceApi` | `oxvba-fixture-evidenced` | Disabled/fixture-labeled until DnaOxIde adapter test proves reference state consumption. |
| `dna_oxide_find_com_candidates` | `references.com.search` | `HostReferenceApi` | `oxvba-available-subset` / `oxvba-fixture-evidenced` | Disabled/fixture-labeled; COM runtime invocation remains unclaimed. |
| `dna_oxide_apply_reference_plan` | `references.com.search` | `HostReferenceApi` | `pending-oxvba-hardening` | Return typed unavailable/pending state until plan DTO/adoption is proven. |
| `dna_oxide_run_project` | `runtime.run` | `HostRuntimeApi` | `oxvba-fixture-evidenced` | Disabled/fixture-labeled until DnaOxIde adapter test proves `EmbeddedBuildRunHost::run_project`. |
| `dna_oxide_stop_runtime` | `runtime.stop` | `HostRuntimeApi` | `pending-oxvba-hardening` | Return typed unavailable/pending state. |
| `dna_oxide_reset_runtime` | `runtime.stop` | `HostRuntimeApi` | `pending-oxvba-hardening` | Return typed unavailable/pending state. |
| `dna_oxide_evaluate_immediate` | `runtime.immediate` | `HostImmediateApi` | `oxvba-fixture-evidenced` | Disabled/fixture-labeled until adapter proof over `EmbeddedRunSession::into_immediate_session` / `ImmediateSession`. |
| `dna_oxide_debug_attach` | `runtime.debug` | `HostDebugApi` | `oxvba-fixture-evidenced` | Disabled/fixture-labeled until adapter proof over `EmbeddedRunSession::into_debug_session`. |
| `dna_oxide_debug_continue` | `debug.continue` | `HostDebugApi` | `oxvba-available-subset` | Disabled/subset-labeled until direct adapter is wired. |
| `dna_oxide_debug_step_into` | `debug.step_into` | `HostDebugApi` | `oxvba-available-subset` | Disabled/subset-labeled until direct adapter is wired. |
| `dna_oxide_debug_step_over` | `debug.step_over` | `HostDebugApi` | `oxvba-available-subset` | Disabled/subset-labeled until direct adapter is wired. |
| `dna_oxide_debug_step_out` | `debug.step_out` | `HostDebugApi` | `oxvba-available-subset` | Disabled/subset-labeled until direct adapter is wired. |
| `dna_oxide_debug_stop` | `runtime.stop` | `HostRuntimeApi` | `pending-oxvba-hardening` | Return typed unavailable/pending state. |
| `dna_oxide_watch_upsert` | `watch.upsert` | `HostDebugApi` | `oxvba-fixture-evidenced` | Disabled/fixture-labeled until adapter proof over `DebugSession::add_watch` / `evaluate_watches`. |
| `dna_oxide_watch_remove` | `watch.upsert` | `HostDebugApi` | `pending-oxvba-hardening` | Return typed unavailable/pending state until remove DTO/adoption exists. |
| `dna_oxide_breakpoint_set` | `breakpoint.set` | `HostDebugApi` | `oxvba-fixture-evidenced` | Disabled/fixture-labeled until adapter proof over `DebugSession::set_source_breakpoint`. |
| `dna_oxide_breakpoint_clear` | `breakpoint.set` | `HostDebugApi` | `pending-oxvba-hardening` | Return typed unavailable/pending state until clear/unbind DTO/adoption exists. |
| `dna_oxide_open_settings` | `settings.open` | `HostSettingsApi` | `proven-oxide-only` | Return host settings placeholder/profile state. |
| `dna_oxide_open_command_palette` | `shell.command_palette` | `HostCapabilityApi` | `proven-oxide-only` | Presentation command; shared UI can display it without native service claims. |

## OxVba Fixture-Evidenced Seams

The following OxVba seams are fixture-evidenced in `../OxVba/docs/evidence/DNAOXIDE_THIN_SLICE_HELLO_FIXTURE_2026-05-07.md` and may be targeted by W344 adapters only after DnaOxIde-side tests are added:

- `EmbeddedBuildRunHost::build_workspace`,
- `EmbeddedBuildRunHost::run_project`,
- `EmbeddedRunSession::into_immediate_session`,
- `ImmediateSession` overlay evaluation,
- `EmbeddedRunSession::into_debug_session`,
- `DebugSession::add_watch`,
- `DebugSession::evaluate_watches`,
- `DebugSession::set_source_breakpoint`,
- stable frame/watch/breakpoint/runtime IDs,
- `ComSelectionService::inspect_workspace_project_state`,
- `ComSelectionService::capability_profile`.

## Request/Response Rules

- Proven project/document/session commands must use explicit paths and test-owned/temp project copies in tests.
- Checked-in fixtures, especially `examples/thin-slice/Module1.bas`, must not be mutated.
- Unavailable commands return typed unavailable/pending responses, not panics or fake data.
- Fixture-evidenced commands must name their evidence source and remain disabled until DnaOxIde adapter tests prove consumption.
- Available-subset commands must be labeled as subset-backed and remain disabled unless the adapter is tested.
- Shared UI remains Tauri-free; DnaOxIde app code owns any eventual `invoke` binding.

## Required No-Claim Defaults

All commands default to:

```text
real_execution_claimed=false
native_runtime_claimed=false
com_runtime_claimed=false
fake_responses=false
fake_debug_data=false
```

These flags may flip only for the exact command path proven by an OxIde/DnaOxIde adapter test.

## W344 Initial Implementation Order

1. Proven project/document/session commands over test-owned temp paths.
2. Capability command returning host bridge status rows.
3. Unavailable/subset/fixture-evidenced service commands returning typed disabled responses.
4. Frontend command client shim, still keeping shared UI independent from Tauri.
