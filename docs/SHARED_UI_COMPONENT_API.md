# OxIde Shared UI Component API

Status: `w342_shared_ui_layer_accepted`
Date: 2026-05-07
Primary crate target: `oxide-ui-leptos`
Primary consumers: **DnaOxIde / DNA OxIde**, DnaOneCalc, `oxide-guilab`

## Purpose

`oxide-ui-leptos` is the planned shared OxIde IDE component crate. It exists so **DNA OxIde** can host a full desktop IDE shell and DnaOneCalc can present the same IDE surface without depending on the `apps/dna-oxide` product app.

The crate is a UI/component layer only. It does not own project semantics, OxVba DTO truth, native filesystem policy, Tauri IPC, runtime execution, Immediate evaluation, debug sessions, COM discovery, or DnaOneCalc product placement.

## Crate Name And Feature Plan

Chosen crate name: `oxide-ui-leptos`.

Initial feature plan:

| Feature | Default | Purpose | Gate |
| --- | --- | --- | --- |
| `html-strings` | yes | deterministic component-like HTML rendering for W342 tests and `oxide-guilab` review | no browser/WebView claim |
| `leptos-components` | no | future real Leptos component exports | added only when dependency/toolchain evidence exists |
| `hydration` | no | future browser/WASM hydration | W345/W346 live host proof, not W342-B00 |
| `tauri-client` | no | intentionally not part of this crate | Tauri client code belongs in DnaOxIde app or later adapter crate |

W342 starts with deterministic rendering helpers so the API boundary can compile and be tested without selecting the full browser/runtime stack prematurely.

## Dependency Boundary

Allowed dependencies:

- `oxide-core` for GUI-neutral packets/view models such as `GuiShellPacket`, `GuiCommandPalette`, `GuiFocusGraph`, `GuiAccessibilityProjection`, `RuntimeServicePacket`, `ImmediateServicePacket`, `DebugServicePacket`, and `ComCapabilityProfile`.
- `oxide-bridge` for host-boundary packets such as DnaOneCalc embedding/web-shell contracts when a consumer frame is rendered.
- small serialization/string helpers only when needed and justified by tests.

Forbidden dependencies:

- `apps/dna-oxide` or any app folder,
- Tauri crates or JavaScript invoke APIs,
- parked TUI `src/shell/*` substrate,
- sibling repo source paths as direct implementation dependencies,
- OxVba transport/CLI parsing,
- DnaOneCalc product shell code.

## Input Model

Initial W342 inputs should be OxIde-owned packets or thin wrappers around them:

```rust
pub struct SharedIdeSurfaceModel {
    pub shell: GuiShellPacket,
    pub runtime: RuntimeServicePacket,
    pub immediate: ImmediateServicePacket,
    pub debug: DebugServicePacket,
    pub provenance: UiDataProvenance,
}

pub enum UiDataProvenance {
    ProvenOxideState,
    OxVbaAvailableSubset { surface: &'static str, evidence: &'static str },
    PendingOxVbaHardening { gap: &'static str },
}
```

The exact Rust names may change in B01/B02, but the concept is fixed:

1. render from view models/packets;
2. carry provenance labels;
3. distinguish unavailable, available-subset, and full future states;
4. do not duplicate final OxVba-owned DTOs.

## Command Dispatch Boundary

Shared components may emit abstract command intents, not host calls.

Example command categories:

- `project.open`,
- `document.save`,
- `document.reload`,
- `runtime.run`,
- `runtime.stop`,
- `runtime.immediate`,
- `runtime.debug`,
- `references.show`,
- `shell.command_palette`.

The shared UI crate may expose command IDs and presentation metadata already owned by `oxide-core`, but it must not call:

- `window.__TAURI__`,
- `@tauri-apps/api`,
- `invoke(...)`,
- DnaOxIde-specific command wrappers,
- DnaOneCalc-specific callbacks.

W343 owns the host bridge facade. W344 owns DnaOxIde Tauri command boundaries.

## Component Targets

Initial W342 component/render targets:

1. shell frame,
2. project spine,
3. editor pane placeholder/boundary,
4. diagnostics panel,
5. lifecycle save/reload state,
6. run/output panel,
7. command palette surface,
8. focus/accessibility labels,
9. runtime service state,
10. Immediate service state,
11. debug service state,
12. COM capability panel.

## Provenance Labels

Every rendered surface with non-static service data should carry a visible and machine-checkable provenance label:

| Label | Meaning |
| --- | --- |
| `proven-oxide-state` | state is fully proven inside OxIde tests |
| `oxvba-available-subset` | data comes from a current OxVba direct Rust subset, but full DnaOxIde hardening is not complete |
| `pending-oxvba-hardening` | capability or DTO is still waiting for OxVba work/evidence |
| `unavailable-no-claim` | capability is unavailable and no fake data is allowed |

These labels prevent subset-backed adapter work from becoming an accidental full runtime/debug/COM claim.

## No-Claim Rules

The shared UI must preserve these claim boundaries until tests prove otherwise:

- `data-real-execution="false"` unless real OxVba execution is proven in OxIde,
- `data-native-runtime="false"` unless native runtime is proven in OxIde,
- `data-com-runtime="false"` unless COM runtime invocation is proven in OxIde,
- `data-fake-responses="false"` always; fake Immediate responses are not allowed,
- `data-fake-debug-data="false"` always; fake callstacks/locals/watches/breakpoints are not allowed,
- no full DOM accessibility audit claim from parsed/static renders alone,
- no live Tauri/WebView claim from deterministic string renders alone.

## DnaOneCalc Reuse Path

DnaOneCalc should be able to consume the same shared UI by:

1. providing `GuiShellPacket` / bridge packets or compatible host bridge responses;
2. mounting the shared component tree in its own product shell;
3. owning its own persistence/product placement policy;
4. leaving OxIde as the IDE surface owner;
5. leaving OxVba as project/language/runtime truth owner.

No DnaOneCalc repo writes are required for W342. W348 owns the OxIde-only reuse proof.

## Verification Expectations

W342-B00 verification checks:

```powershell
rg -n "oxide-ui-leptos|GuiShellPacket|DnaOneCalc|Tauri|UiDataProvenance|pending-oxvba-hardening|data-real-execution=\"false\"|data-com-runtime=\"false\"" docs/SHARED_UI_COMPONENT_API.md
rg -n "apps/dna-oxide|@tauri-apps|window.__TAURI__|invoke\(" crates
```

The second command is expected to return no implementation dependency hits in shared crates. Documentation references are allowed only in docs and app scaffold notes.
