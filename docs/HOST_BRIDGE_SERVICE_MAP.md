# OxIde Host Bridge Service Map

Status: `w343_service_map`
Date: 2026-05-07
Planned crate: `oxide-host-bridge`
Primary consumers: `oxide-ui-leptos`, **DnaOxIde / DNA OxIde**, DnaOneCalc host mounts, browser/WASM review fixtures

## Purpose

The OxIde host bridge facade lets shared UI issue host-neutral requests without depending on Tauri, DnaOxIde app code, DnaOneCalc product code, or OxVba implementation details.

The facade separates every service into one of three evidence states:

1. **proven-oxide-only** — implementable now from OxIde state and tests;
2. **oxvba-available-subset** — a current direct OxVba Rust surface exists, but DnaOxIde hardening or OxIde adapter evidence is still partial;
3. **pending-oxvba-hardening** — the UI/command exists but must return unavailable/pending states until OxVba closes the tracked gap.

## Service Categories

| Service category | Purpose | Current state | Authoritative owner |
| --- | --- | --- | --- |
| `HostProjectApi` | open/list/inspect workspace/project/module roster | `proven-oxide-only` for fixture packets; `oxvba-available-subset` for `HostWorkspaceSession` / `inspect_workspace_target` | OxVba project truth; OxIde host orchestration |
| `HostDocumentApi` | load/save/reload active documents and sessions | `proven-oxide-only` for W320 filesystem/session persistence | OxIde host policy for persistence; OxVba document identity |
| `HostLanguageApi` | diagnostics, hover, symbols, definition, references | `oxvba-available-subset` through `HostWorkspaceSession` and language-service direct APIs | OxVba |
| `HostCompileApi` | compile options, build/check, run target selection | build/check is `oxvba-available-subset`; compile options/run targets are `pending-oxvba-hardening` | OxVba |
| `HostReferenceApi` | active references, COM candidates, reference plans | `oxvba-available-subset` through `ComSelectionService`; capability profile/reorder/native boundary is pending | OxVba |
| `HostRuntimeApi` | run/reset/invoke/stop lifecycle | `oxvba-available-subset` through `EmbeddedBuildRunHost` / `EmbeddedRunSession`; stable IDs/events/source spans pending | OxVba |
| `HostImmediateApi` | Immediate request/response against active runtime | `oxvba-available-subset` through `ImmediateSession`; stable attach/session IDs/taxonomy pending | OxVba |
| `HostDebugApi` | debug attach, continue/step/stop, locals/callstack/watches/breakpoints | `oxvba-available-subset` for `DebugSession` pause/step/locals; watch/breakpoint DTOs/source spans pending | OxVba |
| `HostSettingsApi` | host preferences, app settings, UI policy | `proven-oxide-only` placeholders only | Host app policy |
| `HostCapabilityApi` | capability profile and disabled reasons across all services | `proven-oxide-only` for existing OxIde packets; shared stable taxonomy is `pending-oxvba-hardening` | Shared contract/OxVba + OxIde presentation |

## Shared UI Command Map

| Shared UI command | Host API category | W343 state | Notes |
| --- | --- | --- | --- |
| `project.open` | `HostProjectApi` | `proven-oxide-only` / `oxvba-available-subset` | DnaOxIde can open paths; OxVba owns project identity |
| `project.inspect` | `HostProjectApi` | `oxvba-available-subset` | maps to project/session roster surfaces |
| `document.save` | `HostDocumentApi` | `proven-oxide-only` | W320 native filesystem/session proof covers temp project copies |
| `document.reload` | `HostDocumentApi` | `proven-oxide-only` | W320 proof; browser profile remains disabled |
| `document.revert` | `HostDocumentApi` | `proven-oxide-only` | pure state command |
| `language.diagnostics` | `HostLanguageApi` | `oxvba-available-subset` | direct language service; no LSP routing |
| `language.hover` | `HostLanguageApi` | `oxvba-available-subset` | direct language service |
| `language.definition` | `HostLanguageApi` | `oxvba-available-subset` | direct language service |
| `language.references` | `HostLanguageApi` | `oxvba-available-subset` | direct language service |
| `compile.options` | `HostCompileApi` | `pending-oxvba-hardening` | project properties/compile options DTOs pending |
| `compile.check` | `HostCompileApi` | `oxvba-available-subset` | typed build/check subset exists, request IDs/events pending |
| `references.show` | `HostReferenceApi` | `oxvba-available-subset` | active roster/candidate subset exists |
| `references.com.search` | `HostReferenceApi` | `oxvba-available-subset` | `ComSelectionService` subset; native boundary status pending |
| `runtime.run` | `HostRuntimeApi` | `oxvba-available-subset` | full claim gated on stable IDs/events/source spans |
| `runtime.stop` | `HostRuntimeApi` | `pending-oxvba-hardening` | stop/cancel availability pending |
| `runtime.immediate` | `HostImmediateApi` | `oxvba-available-subset` | attach/session hardening pending |
| `runtime.debug` | `HostDebugApi` | `oxvba-available-subset` | watch/breakpoint/source-span DTOs pending |
| `debug.continue` | `HostDebugApi` | `oxvba-available-subset` | direct debug subset exists |
| `debug.step_into` | `HostDebugApi` | `oxvba-available-subset` | direct debug subset exists |
| `debug.step_over` | `HostDebugApi` | `oxvba-available-subset` | direct debug subset exists |
| `debug.step_out` | `HostDebugApi` | `oxvba-available-subset` | direct debug subset exists |
| `watch.upsert` | `HostDebugApi` | `pending-oxvba-hardening` | watch registry DTOs pending |
| `breakpoint.set` | `HostDebugApi` | `pending-oxvba-hardening` | breakpoint bind/source remap DTOs pending |
| `settings.open` | `HostSettingsApi` | `proven-oxide-only` placeholder | host policy, no OxVba truth |
| `capability.show` | `HostCapabilityApi` | `proven-oxide-only` / `pending-oxvba-hardening` | existing packets now; stable taxonomy pending |
| `shell.command_palette` | `HostCapabilityApi` | `proven-oxide-only` | presentation command only |

## OxVba Available-Subset Adapter Targets

Confirmed current OxVba direct Rust surfaces that W343 should prepare adapters for:

- `HostWorkspaceSession`,
- `inspect_workspace_target`,
- `ComSelectionService`,
- `EmbeddedBuildRunHost`,
- `EmbeddedRunSession`,
- `ImmediateSession`,
- `DebugSession`.

These names identify adapter targets only. `oxide-host-bridge` should not duplicate final OxVba DTO ownership.

## Pending-Hardening Gates

The host bridge must keep these as unavailable/partial until OxVba closes the corresponding work and OxIde adapter tests verify it:

- stable workspace/project/module/runtime/debug/watch/breakpoint IDs,
- shared capability/error taxonomy,
- unified workspace/project/module DTO with revisions and `Attribute VB_Name` state,
- project properties / compile options / run target DTOs,
- build/run request IDs,
- build/run lifecycle event stream,
- command availability DTOs,
- runtime error source-span mapping,
- Immediate/debug attach from `EmbeddedRunSession`,
- watch registry DTOs,
- breakpoint bind/unbind/source-remap DTOs,
- COM capability profile,
- reference reorder plans,
- COM bitness/apartment/native boundary status,
- COM runtime invocation evidence,
- ThinSliceHello fixture ladder evidence for each claim.

## Ownership Boundaries

- **OxIde** owns IDE UI state, shared component rendering, command presentation, no-claim labels, fixture/test harnesses, and DnaOxIde host orchestration.
- **DnaOxIde** owns the standalone Windows desktop product shell, Tauri packaging/configuration, and host policy.
- **DnaOneCalc** owns its product shell, placement, and persistence policy when it consumes the shared IDE surface.
- **OxVba** owns project, language, compile/build, runtime, Immediate, debug, watch, breakpoint, COM/reference, and source mapping truth.

## No-Claim Rules

The host bridge must not turn subset or pending responses into full capability claims.

Required defaults until proven otherwise:

- real execution claimed: `false`,
- native runtime claimed: `false`,
- COM runtime claimed: `false`,
- fake Immediate responses: `false`,
- fake debug data: `false`,
- DnaOneCalc real host mount: `false`.

## Verification

W343-B00 verification checks should include:

```powershell
rg -n "HostProjectApi|HostDocumentApi|HostLanguageApi|HostCompileApi|HostReferenceApi|HostRuntimeApi|HostImmediateApi|HostDebugApi|HostSettingsApi|HostCapabilityApi" docs/HOST_BRIDGE_SERVICE_MAP.md
rg -n "proven-oxide-only|oxvba-available-subset|pending-oxvba-hardening|HostWorkspaceSession|inspect_workspace_target|ComSelectionService|EmbeddedBuildRunHost|EmbeddedRunSession|ImmediateSession|DebugSession" docs/HOST_BRIDGE_SERVICE_MAP.md
rg -n "apps/dna-oxide|@tauri-apps|window.__TAURI__|invoke\(" crates
```

The final command is expected to find no implementation dependency from shared crates to app/Tauri host APIs.
