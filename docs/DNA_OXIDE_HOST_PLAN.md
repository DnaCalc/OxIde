# DNA OxIde Host Plan

Status: `active_host_plan`
Date: 2026-05-07
Product name: **DNA OxIde**
Internal project/app name: `DnaOxIde`
Primary target: Windows desktop standalone host
Secondary target: standalone browser/WASM host profile where capabilities allow

## 1. Decision

Add **DnaOxIde / DNA OxIde** as the standalone product host for the OxIde GUI IDE. Its first-class target is a full Windows desktop host built on Tauri, with a browser/WASM build of the same IDE surface as a secondary, capability-limited host.

This is a **fast-track full-scope host lane**, not a long exploratory runway. The optional standalone WASM host must not delay the Windows desktop path. The sequencing should get to real OxVba project/build/runtime/debug/COM integration as soon as the necessary OxVba/shared DTO seams are authorized and available.

The host should be desktop-first because the full OxVba scope requires native capabilities:

- native filesystem/project/session persistence,
- native OxVba build/run/runtime sessions,
- Immediate Window execution,
- debug sessions,
- COM reference discovery and COM runtime invocation on Windows,
- installer/packaging and WebView2 deployment behavior.

The UI and IDE shell must not become DnaOxIde-specific. DnaOneCalc should be able to present the same OxIde IDE surface by consuming shared UI components and host-boundary packets.

## 2. Current Baseline

Accepted OxIde work already proves:

- `GuiShellPacket` as the shell state contract,
- deterministic `oxide-guilab` scenarios through W330,
- static web-shell rendering and parsed HTML DOM smoke,
- DnaOneCalc host contract packets,
- native filesystem/session persistence against test-owned temp project copies,
- runtime/Immediate/debug service contract packets with no fake data,
- explicit unavailable/no-claim states for browser runtime, full DOM audit, real native runtime/debug/Immediate, COM runtime, and real DnaOneCalc host mount.

DnaOxIde should start by hosting these accepted packets/components, then progressively replace native-service-missing states with tested Windows-native service implementations.

## 3. Recommended Repository Layout

Use an `apps/` directory for branded product hosts and keep reusable IDE/UI code under `crates/`.

```text
apps/
  README.md

  dna-oxide/
    README.md
    package.json                 # frontend scripts once scaffolded
    Trunk.toml or frontend config # if using Leptos/Trunk
    index.html                   # WASM frontend entry
    src/                         # app frontend entry glue only
    src-tauri/
      Cargo.toml                 # Tauri host crate
      tauri.conf.json            # productName = DNA OxIde
      capabilities/              # Tauri v2 permissions/capabilities
      icons/
      binaries/                  # sidecar native service binaries if used
      src/
        main.rs                  # Tauri builder/commands/bootstrap
        commands.rs              # IPC command registration only
        services.rs              # host service wiring/adapters
        windows.rs               # main/editor/settings windows if split
    e2e/                         # later Tauri/WebView interaction tests

crates/
  oxide-domain/                  # host-independent vocabulary
  oxide-core/                    # app state, commands, reducers, packets
  oxide-editor-core/             # editor model, text, selections, overlays
  oxide-oxvba/                   # direct OxVba adapter boundary
  oxide-bridge/                  # serde host/UI boundary DTOs
  oxide-webshell/                # HTML/web adapter over GuiShellPacket
  oxide-guilab/                  # deterministic review lab

  oxide-ui-leptos/               # new shared UI components
  oxide-host-bridge/             # new host service traits + IPC client facade
  oxide-native-service/          # new native service client/server protocol shell
  oxide-tauri-adapter/           # optional shared Tauri command glue if DnaOneCalc later hosts via Tauri too
```

### Why `apps/dna-oxide/` rather than `crates/dna-oxide`?

DnaOxIde is a branded deliverable, not a reusable library. It owns packaging, installer metadata, icons, native host configuration, app windows, and platform permissions. Reusable behavior belongs in crates.

### Why still keep a WASM build?

The same shared UI should be compilable as a standalone browser/WASM host for review, docs, and browser-capable workflows. That host must remain capability-limited:

- no direct filesystem persistence unless browser APIs are deliberately implemented and tested,
- no native OxVba runtime/debug/Immediate,
- no Windows COM.

## 4. Tauri Layout Recommendation

Use Tauri v2 for the desktop host and keep frontend assets/framework-agnostic. Tauri is frontend-agnostic and hosts HTML/CSS/JS/WASM in the platform webview while Rust owns native integration.

Recommended split:

```text
apps/dna-oxide/
  src/                 # Leptos app entry and DnaOxIde-specific shell mounting
  src-tauri/           # native host, commands, capabilities, sidecars
crates/oxide-ui-leptos # shared UI components used by DnaOxIde and DnaOneCalc
crates/oxide-host-bridge # typed host API consumed by the UI
```

Use Tauri config hooks for frontend builds:

```jsonc
{
  "productName": "DNA OxIde",
  "identifier": "com.dnacalc.dnaoxide",
  "build": {
    "beforeDevCommand": "trunk serve --config Trunk.toml",
    "devUrl": "http://localhost:8080",
    "beforeBuildCommand": "trunk build --release --config Trunk.toml",
    "frontendDist": "../dist"
  }
}
```

The exact frontend tool may change, but the contract should stay:

- development loads a local dev URL,
- production bundles static frontend assets,
- Tauri commands expose host services through typed DTOs,
- native services are capability-gated.

## 5. Sidecar vs In-Process Native Service

Use two native integration levels.

### In-process Tauri commands

Best for:

- app bootstrap,
- settings,
- native filesystem dialogs,
- session file read/write,
- opening project paths,
- command/event plumbing.

### Sidecar or dedicated native service process

Recommended for high-risk/full-scope OxVba runtime work:

- COM discovery and invocation,
- STA/MTA apartment/threading policy,
- 32-bit/64-bit Office/COM compatibility seams,
- long-running debug/runtime sessions,
- crash isolation,
- future elevation or broker policies.

Tauri supports bundling sidecars through `bundle.externalBin`. For DNA OxIde, a sidecar boundary is likely cleaner for COM-capable runtime/debug/Immediate than putting everything directly in the WebView host process.

## 6. Shared UI And Host Factoring

DnaOxIde must not become the only owner of UI components. The factoring target is:

```text
oxide-ui-leptos
  IDE shell, panels, editor, dialogs, command palette, debug panes
  depends on host-independent view models and host bridge traits

oxide-host-bridge
  trait/DTO facade used by UI
  examples:
    HostProjectApi
    HostDocumentApi
    HostLanguageApi
    HostCompileApi
    HostRuntimeApi
    HostImmediateApi
    HostDebugApi
    HostReferenceApi
    HostSettingsApi

apps/dna-oxide/src-tauri
  implements host bridge through Tauri commands and native services

DnaOneCalc repo
  can consume oxide-ui-leptos + bridge DTOs or serialized packets
  implements only its host policy / placement / persistence choices
```

The shared UI crate should render from OxIde view models and dispatch typed commands. It should not call Tauri, OxVba, or DnaOneCalc directly.

## 7. Full OxVba Scope To Expose

DNA OxIde should eventually expose the full OxVba-authoritative surface. The source of truth remains OxVba or shared DTOs coordinated with OxVba.

### Project and compile options

UI surfaces:

- project properties,
- module list and include paths,
- compile target/entrypoint list,
- conditional compilation constants,
- reference list,
- Option Explicit / project-level policy indicators,
- build profile/runtime profile selector,
- warnings/errors panel.

Host/service needs:

- authoritative project metadata from OxVba,
- compile options DTOs,
- build/check command,
- build output and diagnostics stream,
- disabled reasons for unsupported host profiles.

### Editor and language service

UI surfaces:

- source editor,
- diagnostics squiggles and panel,
- hover,
- completion,
- go to definition,
- find references,
- symbol outline,
- rename/code-action planning where OxVba supports it.

Host/service needs:

- direct OxVba language-service session,
- document synchronization,
- debounced semantic refresh,
- source span mapping,
- no LSP indirection for internal OxIde semantics.

### COM references and type libraries

UI surfaces:

- references dialog,
- COM reference selector,
- search/filter installed type libraries,
- reference details pane,
- missing/unavailable COM explanation,
- Windows-native-only capability banner,
- project reference diff/apply preview.

Host/service needs:

- Windows native COM discovery service,
- registry/type-library enumeration,
- selected reference identity DTO,
- compatibility/bitness status,
- project reference update through OxVba-owned project semantics,
- COM runtime invocation availability.

### Runtime and run output

UI surfaces:

- run target selector,
- build/run command buttons,
- output/activity timeline,
- stop/cancel,
- runtime errors with source spans,
- host capability footer.

Host/service needs:

- runtime session id,
- run target enumeration,
- build/run lifecycle events,
- output/error stream,
- cancellation,
- native-service-missing and browser-disabled errors.

### Immediate Window

UI surfaces:

- Immediate input prompt,
- command history,
- response stream,
- values/output/errors,
- session state banner,
- disabled native-service-missing state.

Host/service needs:

- Immediate session tied to runtime session,
- request/response DTOs,
- evaluation error taxonomy,
- availability when running/paused/stopped,
- no fake responses.

### Debug, watches, breakpoints

UI surfaces:

- debug toolbar: continue, break, step into/over/out, stop, restart,
- callstack pane,
- locals pane,
- watches pane,
- breakpoints pane,
- source gutter breakpoints,
- current execution line,
- breakpoint bind/unbound status,
- runtime error pause state.

Host/service needs:

- debug session id,
- execution state,
- command availability,
- callstack DTOs,
- locals/watch value DTOs,
- watch expression evaluate/update,
- breakpoint bind/unbind/status DTOs,
- source span mapping,
- no fake debug data.

## 8. Fast-Track Workset Roadmap

The goal is to reach full standalone host scope soon. Treat the roadmap below as an aggressive vertical path: each workset must land user-visible host capability, subset-backed adapter evidence, or a concrete host-driving test. Shared UI factoring is mandatory, but it must happen just-in-time around the desktop host rather than delaying the desktop host.

OxVba feedback on 2026-05-07 confirms several available-subset direct Rust surfaces already exist. Therefore the OxIde-side plan is not pure waiting/stubbing: W343-W347 should consume those subsets where dependency wiring is ready, while keeping remaining stable-ID/taxonomy/watch/breakpoint/COM-runtime/full-debug gaps visible.

### W340 — DnaOxIde standalone host foundation

Goal: create the branded host lane and full-scope plan without real runtime claims.

Deliverables:

- `apps/dna-oxide/` planning scaffold,
- DNA OxIde product/architecture plan,
- OxVba requirements and feedback alignment notes,
- W341-W349 continuation worksets.

### W341 — DnaOxIde Tauri app scaffold

Goal: create the Tauri-ready app skeleton and product metadata.

Deliverables:

- `apps/dna-oxide` frontend/native scaffold,
- product name `DNA OxIde`,
- Tauri config/bootstrap placeholders,
- scaffold verification and no-claim scan.

### W342 — Shared IDE UI component layer

Goal: make accepted OxIde GUI panes reusable by DnaOxIde and DnaOneCalc.

Deliverables:

- shared component crate target, expected `oxide-ui-leptos`,
- shell/project/editor/diagnostics/lifecycle/runtime/Immediate/debug/COM panes,
- visible unavailable and available-subset labels,
- `oxide-guilab` deterministic render path.

### W343 — OxIde host bridge facade

Goal: separate shared UI from host/service implementations.

Deliverables:

- host-neutral service categories for project, document, language, compile, references, runtime, Immediate, debug, settings, and capability,
- available-subset adapter map for current OxVba surfaces,
- pending-hardening unavailable states for remaining OxVba gaps,
- no Tauri dependency in shared bridge/UI crates.

### W344 — DnaOxIde Tauri command boundary stubs

Goal: define Tauri IPC command boundaries over the host bridge.

Deliverables:

- proven OxIde-only project/document/session commands,
- available-subset OxVba adapter commands where wired,
- pending-hardening unavailable responses for missing DTOs/events/source spans/watch/breakpoint/COM-runtime gaps,
- no fake runtime/debug/Immediate data.

### W345 — DnaOxIde live host UI proof

Goal: make the app host path display the shared IDE shell.

Deliverables:

- DNA OxIde host render/proof mode,
- ThinSliceHello and `Module1.bas` visible,
- lifecycle/save-reload state visible,
- runtime/Immediate/debug/COM unavailable or subset-backed states visible,
- no unproven full capability claim.

### W346 — DnaOxIde interaction and e2e harness

Goal: add deterministic host interaction coverage.

Deliverables:

- command palette/keyboard interaction tests,
- focus/no-mouse route tests,
- open/save/reload/session interactions on temp projects,
- blocked and subset-backed service interaction tests,
- no full accessibility audit claim unless separately tested.

### W347 — Compile options and reference UI placeholders/subset panels

Goal: build the full-scope compile/options/reference surfaces now.

Deliverables:

- project properties and compile options panels,
- subset-backed project/reference/build panels where current OxVba APIs provide data,
- COM candidate/active-selection subset panels where wired,
- pending states for compile options/run targets/stable taxonomy/COM native boundary gaps.

### W348 — DnaOneCalc shared UI reuse path

Goal: prove inside OxIde that the same shared UI/host bridge path is reusable by DnaOneCalc.

Deliverables:

- DnaOneCalc-like host profile/fixture,
- shared UI render under a DnaOneCalc-like frame,
- ownership labels for DnaOneCalc/OxIde/OxVba,
- no sibling repo writes or real DnaOneCalc mount claim.

### W349 — DnaOxIde while-OxVba acceptance

Goal: consolidate W341-W348 into a coherent runway ready for OxVba full-scope integration.

Deliverables:

- cross-workset evidence audit,
- full continuation regression,
- OxVba integration readiness report,
- remaining gap and authorization list.

### Later real-capability integration worksets

After W349, or in parallel where direct adapter evidence is ready, the next capability worksets can proceed as:

- W355 — compile/build UX over OxVba project properties, compile options, run targets, and typed build/check results;
- W360 — COM references over OxVba COM/reference capability profile and candidate/repair/apply surfaces;
- W365 — runtime + Immediate over stable OxVba runtime and Immediate session IDs/events/responses;
- W370 — debug/watch/breakpoints over stable OxVba debug/session/watch/breakpoint DTOs;
- W375 — packaging and full host regression.

Those worksets may adopt available-subset OxVba APIs early, but full claims remain gated on OxIde-side tests over matching OxVba evidence.

## 9. Test Strategy

Keep the current evidence ladder and add host-driving layers only when the host exists.

1. Unit tests in `oxide-core`, `oxide-editor-core`, `oxide-bridge`.
2. `oxide-guilab` deterministic scenario renders for every accepted behavior.
3. `oxide-webshell` parsed DOM smoke for shared components.
4. DnaOxIde Tauri command tests for host IPC and filesystem/session behavior.
5. Tauri/WebView interaction smoke only after the scaffold exists.
6. Windows-native COM/runtime/debug tests behind explicit platform/capability gates.
7. Installer smoke only after packaging exists.

No test should mutate checked-in fixtures. Disk tests use test-owned temp project copies.

## 10. Capability Honesty Matrix

| Capability | Browser/WASM host | DnaOxIde Windows desktop | DnaOneCalc embedded host |
| --- | --- | --- | --- |
| Project open/read | proven through fixture render | planned native host implementation | contract exists, real mount pending |
| Editing/diagnostics | proven as state/render slices | planned shared UI + native lifecycle | shared component target |
| Direct filesystem save/reload | disabled | W320 proof exists, host wiring pending | host policy owned by DnaOneCalc |
| Compile options/build check | not yet complete | planned W355 | shared UI/contract target |
| Run output | simulated/disabled only | planned W365 | host-dependent |
| Immediate | disabled/contract only | planned W365 | host-dependent |
| Debug/watch/breakpoints | disabled/contract only | planned W370 | host-dependent |
| COM reference discovery | unavailable | planned Windows-native W360 | host-dependent/native only |
| COM runtime | unavailable | later, only with tested native service | host-dependent/native only |
| Full DOM accessibility audit | not claimed | planned later | planned later |

## 11. Cross-Repo Gates

DnaOxIde can be scaffolded inside OxIde now. Full runtime/debug/Immediate/COM support still requires coordinated OxVba interfaces or shared DTOs. Do not duplicate OxVba-owned types in OxIde to avoid coordination.

Required handoff/authorization points:

- OxVba runtime/debug/Immediate DTOs,
- OxVba compile options/build target DTOs,
- OxVba COM reference/type-library DTOs,
- DnaOneCalc consumption of `oxide-ui-leptos` or serialized host packets,
- any sibling repo writes.

See [`HANDOFF_DNAOXIDE_OXVBA_REQUIREMENTS.md`](HANDOFF_DNAOXIDE_OXVBA_REQUIREMENTS.md) for the fast-track OxVba requirements note, and [`HANDOFF_DNAOXIDE_OXVBA_FEEDBACK_ALIGNMENT.md`](HANDOFF_DNAOXIDE_OXVBA_FEEDBACK_ALIGNMENT.md) for the confirmed OxVba feedback that splits current work into available-subset adapter proofs versus remaining stable-ID/taxonomy/watch/breakpoint/COM-runtime/full-debug gaps.

## 12. External References Checked

- Tauri v2 overview/start docs: https://v2.tauri.app/start/
- Tauri frontend configuration: https://v2.tauri.app/start/frontend/
- Tauri Trunk frontend configuration: https://v2.tauri.app/start/frontend/trunk/
- Tauri configuration file hooks: https://v2.tauri.app/develop/configuration-files/
- Tauri sidecar/external binary docs: https://v2.tauri.app/develop/sidecar/
- Tauri WebView2 note for Windows: https://v2.tauri.app/reference/webview-versions/
