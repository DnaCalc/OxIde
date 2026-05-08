# OxIde Architecture

Status: `active_architecture_direction`
Date: 2026-05-08

This document is subordinate to [`CHARTER.md`](CHARTER.md) and [`PRODUCT_DIRECTION.md`](PRODUCT_DIRECTION.md). It records the implementation seams and ownership boundaries for the active Rust/WASM-capable GUI pivot.

## 1. Architectural Position

`OxIde` is the OxVba IDE surface for the DNA Calc program.

The active architecture should optimize for:

1. a shared Rust IDE core,
2. a Rust/WASM-capable GUI surface,
3. browser website and desktop host shells,
4. embedded consumption by DNA Calc hosts such as `DnaOneCalc`,
5. a wasm-safe OxVba compiler/runtime profile for browser-hosted DnaOneCalc,
6. a native OxVba compiler/runtime/debug/COM profile for Windows desktop hosts,
7. direct typed OxVba integration for project/language/runtime truth,
8. explicit host capability profiles,
9. greenfield GUI implementation rather than mutation of the current TUI codebase.

The parked FrankenTui implementation is retained as prototype/evidence lineage, not as the active architectural substrate. See [`docs/TUI_PARKING_PLAN.md`](docs/TUI_PARKING_PLAN.md).

## 2. Core Rule

```text
OxVba owns VBA/project/runtime truth.
OxIde owns IDE experience.
DNA Calc hosts consume, embed, and run where appropriate.
```

Architectural consequences:

1. do not duplicate OxVba project semantics,
2. do not duplicate OxVba language semantics,
3. do not route OxIde internal semantics through LSP,
4. do not create long-lived local copies of sibling-repo enums/types when authoritative types can be consumed,
5. prefer coordinated cross-repo interface improvements over compatibility adapter sprawl,
6. keep host capability truth explicit and testable.

## 3. Cross-Repo Architecture Assumption

OxIde is one repo in the coordinated DNA Calc product family. The architecture may recommend changes in OxVba, DnaOneCalc, Foundation, or other sibling repos when that gives a cleaner final system.

Execution boundary:

- OxIde repo-scoped agents write only inside OxIde,
- sibling repos may be read freely,
- needed sibling-repo changes are captured as handoff notes or done in separate repo-scoped runs.

This matters because architecture should not contort around avoidable compatibility layers. If a type belongs in OxVba or a shared DNA Calc crate, move/share it through coordinated work rather than duplicating it in OxIde.

See [`docs/HANDOFF_DNA_CALC_GUI_PIVOT_COORDINATION.md`](docs/HANDOFF_DNA_CALC_GUI_PIVOT_COORDINATION.md).

## 4. Target Stack Direction

The preferred stack direction is:

- Rust for shared application/domain/editor/session/compiler-adapter logic;
- Leptos or a similar Rust/WASM-friendly UI framework for shared GUI surfaces;
- browser/WASM host support for the DnaOneCalc website scenario, including an OxVba wasm-safe compiler/runtime profile that can run supported code inside the browser-hosted DnaOneCalc app;
- Tauri or equivalent local desktop host for standalone DnaOxIde and DnaOneCalc desktop operation;
- linked native Rust command layers inside desktop hosts by default;
- optional separate native service processes only when COM apartment policy, crash isolation, long-lived runtime isolation, or multi-host sharing requires them;
- native Windows host capability where COM-capable OxVba execution is required;
- OxVba direct Rust APIs and shared DTOs for project/language/compiler/runtime/debug/COM integration.

This is a direction, not a dependency lock. Candidate dependencies must pass engineering, testing, and license review. See [`docs/EDITOR_SUBSTRATE_RESEARCH.md`](docs/EDITOR_SUBSTRATE_RESEARCH.md), [`docs/THIRD_PARTY_RESEARCH_AND_LICENSES.md`](docs/THIRD_PARTY_RESEARCH_AND_LICENSES.md), and [`docs/OXIDE_TARGET_STACK_SCENARIOS.md`](docs/OXIDE_TARGET_STACK_SCENARIOS.md).

### Target Product Scenarios

The same IDE surface must support:

1. **DnaOneCalc website / browser WASM** — DnaOneCalc loads from a website, opens OxIde in the browser, compiles/checks through OxVba wasm-safe APIs, and runs/invokes supported functions inside the DnaOneCalc WASM host. Native-only features are unavailable with typed disabled reasons.
2. **Standalone Windows DnaOxIde desktop** — DnaOxIde runs in a desktop shell with the same UI and a native Rust command layer linked into the app by default. That command layer can call OxIde/OxVba crates directly and expose native filesystem, runtime/debug, wrapped native execution, and Windows COM where supported.
3. **DnaOneCalc Windows desktop host** — DnaOneCalc owns the desktop product shell, embeds/opens OxIde, and exposes native OxVba services to the shared IDE surface.

### Meaning of Native Rust Backend

In this architecture, "native Rust backend" for a Tauri app means the Rust side of the Tauri application crate by default:

```text
Tauri desktop app
  ├─ WebView UI
  └─ linked Rust app code
      ├─ OxIde host commands
      ├─ OxIde adapters
      └─ OxVba crates / native services
```

It does not mean a separate process unless a workset explicitly chooses that for isolation or COM/runtime policy.

### Product-Seam Rule

Fast feedback remains valuable, but future work must exercise a real endpoint seam: shared Rust/WASM UI, DnaOneCalc browser WASM host integration, Tauri/WebView UI to linked native Rust command, native Rust to OxVba adapter, or real host capability reporting. Static HTML snapshots are review artifacts only and must not become a substitute implementation track.

## 5. Proposed Workspace Shape

The final crate split may change, but the target shape is:

```text
crates/
  oxide-domain/
    pure IDs, view models, capability model, host-independent vocabulary

  oxide-core/
    app state, command registry, reducers, session orchestration

  oxide-editor-core/
    text buffer, selections, caret, decorations, editor commands

  oxide-oxvba/
    OxVba adapter over project, language-service, wasm-safe compile/run,
    native build/run, immediate, debug, COM, and capability-sensitive runtime paths

  oxide-bridge/
    serde request/event DTOs where a host/UI boundary requires serialization

  oxide-ui-leptos/
    GUI components, design system, panes, dialogs, editor surface integration

  oxide-guilab/
    browser scenario catalogue and visual feedback harness

  oxide-host-browser/
    browser/WASM entrypoint for the DnaOneCalc website-compatible profile

  oxide-host-tauri/
    standalone desktop entrypoint with linked native Rust commands by default

  oxide-host-dnaonecalc/
    optional shared host-facing crate only if needed to avoid duplication across
    DnaOneCalc browser and desktop embedding

  oxide-tui-frankentui/
    parked TUI implementation, feature-gated or otherwise isolated
```

Important: `oxide-bridge` should not become a dumping ground for duplicated upstream models. It should contain boundary DTOs only when direct authoritative types are not appropriate across the boundary.

A dedicated `oxide-host-dnaonecalc` crate is not assumed initially. The likely cleaner path is shared components/contracts that DnaOneCalc consumes from its side, avoiding circular dependencies.

## 6. Major Seams

### `oxide-domain`

Owns host-independent product vocabulary:

- IDE IDs and references,
- view models that are genuinely OxIde-owned,
- capability/profile projection if not owned upstream,
- host-independent command descriptors,
- UI-neutral status and availability concepts.

It should stay pure and deterministic.

### `oxide-core`

Owns application behavior:

- project/session orchestration,
- command registry and dispatch,
- reducers/state transitions,
- open document coordination,
- active host profile selection,
- layout-independent shell state,
- coordination between editor, OxVba adapter, persistence, and UI.

It should not depend on concrete Leptos widgets.

### `oxide-editor-core`

Owns rendering-independent editor behavior:

- text buffer model,
- selection and caret model,
- editor commands,
- undo/redo,
- decorations and overlay ranges,
- diagnostics/completion/hover attachment points,
- source snapshot production for OxVba.

It should not own project semantics or call OxVba directly.

### `oxide-oxvba`

Owns OxIde-side integration with OxVba:

- workspace/project loading through authoritative OxVba APIs,
- mapping OxIde document/session concepts to OxVba document identity,
- semantic query orchestration,
- wasm-safe compile/check/run invocation where OxVba exposes it,
- native build/run/immediate/debug session access where the host exposes it,
- COM capability reporting and native-runtime routing where applicable,
- translation to OxIde-owned view models only where needed.

It should avoid duplicating OxVba-owned enums and contracts. If OxVba lacks a clean wasm-safe/native capability split, OxIde should record and coordinate that upstream work rather than replacing OxVba truth locally.

### `oxide-bridge`

Owns serializable request/event boundaries when needed:

- browser/native boundary packets,
- host capability snapshots where serialization is required,
- runtime and document events for UI-host separation,
- DnaOneCalc/OxIde component boundary packets if direct Rust types are not viable.

If a DTO is actually an OxVba public host contract, prefer a coordinated move to OxVba or a shared crate.

### `oxide-ui-leptos`

Owns GUI presentation:

- design tokens,
- layout primitives,
- IDE shell composition,
- editor component rendering,
- project spine,
- context dock,
- activity surfaces,
- dialogs/overlays,
- command palette,
- accessibility and focus surfaces.

It should depend on view models and commands, not on OxVba directly.

### Host crates

Host crates own startup, packaging, and platform integration only:

- `oxide-host-browser` boots browser/WASM mode and must remain compatible with the DnaOneCalc website scenario,
- `oxide-host-tauri` boots standalone desktop mode and exposes linked native Rust commands by default,
- host-specific native services are exposed through typed capability-aware seams,
- a separate native service process is an explicit workset decision, not the default meaning of backend.

Product behavior should not live first in host wrappers.

### `oxide-guilab`

Owns fast visual and interaction feedback for GUI scenarios:

- deterministic scenario catalogue,
- browser-rendered review surfaces,
- snapshot/a11y/DOM text checks,
- successor to the TUI UX lab for active GUI work.

## 7. Capability Profile Architecture

Capability profiles are first-class architecture.

They should answer:

- what platform is running,
- whether the runtime is WASM or native,
- whether filesystem access is available,
- whether OxVba semantic services are available,
- whether OxVba execution is available,
- whether COM reference discovery is available,
- whether COM runtime invocation is available,
- what persistence mode is available,
- what host owns retained evidence or artifact handoff.

Conceptual shape:

```rust
HostCapabilityProfile {
    host_kind,
    platform,
    ui_runtime,                 // browser WASM, desktop WebView, native shell
    oxvba_compiler_profile,     // wasm-safe, native, unavailable
    oxvba_runtime_profile,      // wasm-safe, native, unavailable
    filesystem_access,
    oxvba_semantics,
    oxvba_execution,
    com_reference_discovery,
    com_runtime_invocation,
    persistence_mode,
}
```

The actual type should be shared or upstreamed if the concept belongs in OxVba, DnaOneCalc, Foundation, or another DNA Calc crate.

## 8. Windows COM Architecture

Pure browser/WASM cannot call Windows COM.

COM-capable execution requires a native Windows runtime layer. The intended architecture is:

```text
Leptos / GUI surface
  owns editor UI, panes, commands, presentation

Native Windows host service
  owns COM-capable OxVba runtime/session
  owns COM apartment/threading policy
  owns registry/type-library discovery
  owns actual COM calls

OxVba
  owns project, language, runtime, immediate/debug semantics
```

The native Windows service should be deliberately owned and tested. It should not be an incidental async callback from UI code.

Likely responsibilities:

- initialize and own COM apartment policy,
- discover registered and file-backed type libraries,
- create COM objects,
- marshal calls/results/errors,
- enforce trust/capability policy,
- emit typed runtime events back to the GUI.

If OxVba already owns or should own this service, OxIde should create a handoff rather than duplicating it.

## 9. Host Protocol / Bridge Architecture

A typed request/event protocol should be designed early because the same IDE surface must operate across host modes.

Likely packet families:

```text
IdeRequest / IdeEvent
ProjectRequest / ProjectEvent
DocumentRequest / DocumentEvent
RuntimeRequest / RuntimeEvent
ImmediateRequest / ImmediateEvent
DebugRequest / DebugEvent
CapabilitySnapshot
DiagnosticPacket
CompletionPacket
RunPacket
```

Rules:

1. use direct Rust APIs when UI and service live in the same process/crate boundary,
2. use serializable DTOs only at real serialization boundaries,
3. avoid DTOs that duplicate authoritative OxVba types long-term,
4. version the host protocol once external host consumption requires stability,
5. make capability and disabled-reason reporting part of the protocol from the start.

## 10. DnaOneCalc Integration Architecture

DnaOneCalc is the first exemplar host, but OxIde should not become a DnaOneCalc submodule by accident.

Integration proof ladder:

1. artifact/runtime proof,
2. embedded editor proof,
3. shared component proof.

The dependency direction should be chosen to avoid cycles. Likely pattern:

```text
shared OxIde editor/session/UI components
  consumed by OxIde standalone
  consumed by DnaOneCalc host surface where appropriate

OxVba authoritative APIs/types
  consumed by both through clean adapter layers
```

DnaOneCalc-specific product behavior remains in DnaOneCalc. OxIde-specific IDE behavior remains in OxIde.

See [`docs/DNA_CALC_HOST_INTEGRATION.md`](docs/DNA_CALC_HOST_INTEGRATION.md).

## 11. Persistence Architecture

Persistence must be capability-aware.

Do not preserve the current APPDATA-only assumption as an architectural rule.

Host profiles may include:

- browser local/session storage,
- browser file picker / user-granted handles,
- desktop filesystem access,
- DnaOneCalc workspace persistence,
- retained evidence bundles,
- read-only/demo scenarios.

OxIde should distinguish:

- project source persistence,
- IDE session restore,
- host capability persistence,
- retained run/debug evidence,
- user preferences/keybindings.

## 12. Testing Architecture

Testing must be created early, not after the GUI grows.

Primary layers:

1. pure Rust unit tests,
2. OxVba contract tests,
3. WASM/browser tests,
4. browser visual/scenario tests,
5. host capability matrix tests,
6. DnaOneCalc integration smoke tests.

Existing WTD tests remain parked TUI tests and should not be the GUI default loop.

See [`docs/GUI_TEST_STRATEGY.md`](docs/GUI_TEST_STRATEGY.md).

## 13. Current Codebase Policy

The existing TUI codebase is valuable but not the GUI foundation.

Policy:

```text
rewrite behavior deliberately in GUI-native architecture
reuse existing code only when it is pure and not terminal-shaped
preserve TUI code as parked lineage
```

Known posture:

- `src/shell/view.rs`: park only,
- `src/shell/model.rs`: rewrite from behavior,
- `src/shell/state.rs`: rewrite from behavior,
- `src/shell/session.rs`: rewrite/extract cautiously,
- `src/shell/oxvba.rs`: rewrite into `oxide-oxvba`,
- `src/shell/project_actions.rs`: rewrite into services/adapters,
- `src/shell/uxlab/*`: mine for scenario-lab patterns.

See [`docs/GUI_PIVOT_CODEBASE_REVIEW.md`](docs/GUI_PIVOT_CODEBASE_REVIEW.md) and [`docs/GUI_WORKSPACE_LAYOUT.md`](docs/GUI_WORKSPACE_LAYOUT.md).

## 14. Accessibility And Command Architecture

The GUI architecture must retain keyboard-first discipline.

Required architectural support:

- command registry with stable IDs,
- keybinding contexts,
- command palette,
- visible disabled reasons,
- focus graph and focus restoration,
- screen-reader labels and semantic roles,
- high-contrast token support,
- no-mouse critical path support,
- platform shortcut remapping.

Command availability should be computed from state and capability, not scattered through widgets.

## 15. First Vertical Implementation Path

After W200 foundation work, implementation should proceed through vertical slices:

1. fixture project opens in GUI,
2. editable module and diagnostics,
3. save/reload/session restore,
4. capability-aware run/output,
5. DnaOneCalc embedded IDE/runtime proof,
6. Windows COM capability proof,
7. run/debug/immediate GUI surfaces,
8. polish/accessibility/packaging.

Each workset should produce runnable or reviewable evidence.

## 16. Live Architecture Docs

Supporting live docs:

- [`docs/GUI_DIRECTION.md`](docs/GUI_DIRECTION.md)
- [`docs/DNA_CALC_HOST_INTEGRATION.md`](docs/DNA_CALC_HOST_INTEGRATION.md)
- [`docs/GUI_PIVOT_CODEBASE_REVIEW.md`](docs/GUI_PIVOT_CODEBASE_REVIEW.md)
- [`docs/GUI_TEST_STRATEGY.md`](docs/GUI_TEST_STRATEGY.md)
- [`docs/EDITOR_SUBSTRATE_RESEARCH.md`](docs/EDITOR_SUBSTRATE_RESEARCH.md)
- [`docs/THIRD_PARTY_RESEARCH_AND_LICENSES.md`](docs/THIRD_PARTY_RESEARCH_AND_LICENSES.md)
- [`docs/TUI_PARKING_PLAN.md`](docs/TUI_PARKING_PLAN.md)
- [`docs/worksets/W200_gui_pivot_foundation.md`](docs/worksets/W200_gui_pivot_foundation.md)

## 17. Non-Goals

Current non-goals:

- full GUI implementation in W200,
- arbitrary third-party embedded-host support,
- generic editor platform architecture,
- internal LSP indirection,
- copying restrictive-license editor code,
- mutating the current TUI code into the GUI product,
- deleting the parked TUI lineage.
