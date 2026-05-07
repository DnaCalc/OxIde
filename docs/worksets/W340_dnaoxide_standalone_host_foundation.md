# Workset W340 — DnaOxIde Standalone Host Foundation

## Ambition

Create **DnaOxIde**, branded as **DNA OxIde**, as the standalone Windows desktop host path for OxIde. W340 establishes the app layout, host boundaries, and first Tauri-ready scaffold plan while preserving shared UI/component factoring for DnaOneCalc.

This is a fast-track host lane: optional standalone WASM support and broad factoring must not delay the Windows desktop path to full OxVba scope. W340 still does not claim real OxVba runtime/debug/Immediate/COM execution; it prepares the host shell and service seams that the next short worksets should fill with tested native implementations.

## Dependencies

- W300 — mounted web shell adapter.
- W310 — DnaOneCalc web shell hosting contract.
- W320 — native filesystem/session persistence proof.
- W330 — runtime/debug/Immediate service contract packets.
- [`docs/DNA_OXIDE_HOST_PLAN.md`](../DNA_OXIDE_HOST_PLAN.md).
- [`docs/HANDOFF_W340_CROSS_REPO_NATIVE_SERVICE_OR_HOST_IMPLEMENTATION.md`](../HANDOFF_W340_CROSS_REPO_NATIVE_SERVICE_OR_HOST_IMPLEMENTATION.md).
- [`ARCHITECTURE.md`](../../ARCHITECTURE.md).
- [`PRODUCT_DIRECTION.md`](../../PRODUCT_DIRECTION.md).

## Guardrails

1. DnaOxIde is a branded host app; reusable IDE behavior belongs in shared crates.
2. Shared UI must be factored so DnaOneCalc can consume it without depending on DnaOxIde.
3. OxVba remains project/language/runtime/debug/Immediate/COM truth owner.
4. Do not duplicate authoritative OxVba or DnaOneCalc types.
5. Do not write to sibling repositories without explicit authorization.
6. Do not claim real runtime/debug/Immediate/COM execution without native-service tests.
7. Keep browser/WASM capability limitations visible.
8. Preserve parked TUI lineage; do not import parked TUI substrate into the GUI host.

## Design

W340 creates the product-host lane:

```text
apps/dna-oxide/          # DNA OxIde branded host
crates/oxide-ui-leptos/  # future shared UI components
crates/oxide-host-bridge/# future host service trait/DTO facade
```

The DnaOxIde app should eventually contain Tauri-specific startup, windows, permissions, packaging, and native service wiring. It should not contain core IDE logic, UI components that DnaOneCalc needs, or OxVba-owned semantics.

The first scaffold should be conservative: host the accepted `GuiShellPacket`/web-shell slices and runtime-service-missing states before any real native execution is claimed.

## Beads

### W340-B00 — DnaOxIde host architecture plan

Goal:
  Capture the DnaOxIde / DNA OxIde standalone host architecture plan and register the next workset without claiming unimplemented runtime/COM/debug capability.

Design:
  - Document recommended app/crate layout for Tauri desktop-first plus optional standalone WASM host.
  - Factor shared UI/components/contracts so DnaOneCalc can consume the same IDE surface.
  - Plan full OxVba scope coverage: compile options, references/COM selection, runtime, Immediate, debug, watch, locals/callstack/breakpoints.
  - Keep sibling OxVba/DnaOneCalc changes as handoff/authorization gates.

Tests:
  - Documentation verification for DnaOxIde, DNA OxIde, Tauri, shared UI, DnaOneCalc, OxVba scope, and no-claim/authorization tokens.

Evidence:
  - `docs/DNA_OXIDE_HOST_PLAN.md` and W340 workset registration.

Closure:
  - [ ] DnaOxIde host plan exists.
  - [ ] W340 workset is registered.
  - [ ] Cross-repo and no-claim gates are explicit.

### W340-B01 — DnaOxIde app directory scaffold

Goal:
  Create the initial `apps/dna-oxide` host scaffold without introducing unreviewed Tauri behavior or runtime claims.

Design:
  - Add Tauri-oriented app directory files.
  - Keep app README and host metadata explicit about DNA OxIde branding.
  - Add placeholder frontend/native directories only when accompanied by tests or render evidence.
  - Do not wire real runtime/COM services yet.

Tests:
  - Workspace/file verification confirms app directory exists.
  - No runtime/COM true-claim tokens appear.

Evidence:
  - `apps/dna-oxide/` scaffold and docs.

Closure:
  - [ ] DnaOxIde app path exists.
  - [ ] Branding is visible.
  - [ ] No real runtime/COM claim is introduced.

### W340-B02 — Shared UI crate planning/scaffold

Goal:
  Establish the shared UI crate boundary that DnaOxIde and DnaOneCalc can both consume.

Design:
  - Add or plan `crates/oxide-ui-leptos`.
  - Component APIs consume OxIde view models/packets, not Tauri commands directly.
  - Reuse accepted W290/W300 shell surfaces.
  - Keep DnaOneCalc host policy outside the shared UI crate.

Tests:
  - Component boundary compiles or, if planning only, documentation grep verifies dependencies and no Tauri coupling.

Evidence:
  - Shared UI crate scaffold or follow-on handoff.

Closure:
  - [ ] Shared UI boundary is clear.
  - [ ] DnaOneCalc consumption path is preserved.
  - [ ] Tauri-specific behavior remains outside shared components.

### W340-B03 — Host bridge scaffold

Goal:
  Establish the host service facade that can be implemented by DnaOxIde Tauri and later by DnaOneCalc.

Design:
  - Define host APIs for project, document, language, compile, references, runtime, Immediate, debug, settings.
  - Reuse existing `RuntimeServicePacket`, `ImmediateServicePacket`, `DebugServicePacket`, and `GuiShellPacket` where appropriate.
  - Keep OxVba-owned DTOs as handoff/shared-type candidates rather than duplicated final truth.

Tests:
  - Packet/trait examples compile or documentation verifies service coverage.
  - No fake runtime/debug/Immediate data is introduced.

Evidence:
  - Host bridge crate or documented API surface.

Closure:
  - [ ] Host services are named and separated.
  - [ ] Runtime/debug/Immediate remain capability-gated.
  - [ ] OxVba ownership is preserved.

### W340-B04 — Tauri scaffold and accepted shell render

Goal:
  Scaffold the Tauri host enough to render accepted shell state without native runtime/COM claims.

Design:
  - Add `src-tauri` app shell.
  - Use product name `DNA OxIde` and identifier plan.
  - Render accepted `GuiShellPacket`/web-shell state in the host frontend.
  - Keep native-service-missing runtime/Immediate/debug states visible.

Tests:
  - Tauri app scaffold check or cargo/npm build if dependencies are introduced.
  - Render/smoke output includes no-claim tokens.

Evidence:
  - Host render or scaffold verification.

Closure:
  - [ ] Tauri scaffold exists.
  - [ ] Accepted shell state is visible.
  - [ ] Runtime/COM claims remain false.

### W340-B05 — W340 acceptance and next workset decision

Goal:
  Accept the DnaOxIde host foundation and choose the shortest next step toward full scope, preferring a combined W345 shared UI + host bridge minimum viable shell over a long abstraction-only runway.

Design:
  - Update `docs/GUI_FIXTURES_AND_LAB.md` or a DnaOxIde-specific evidence doc.
  - Preserve W210-W340 regression evidence.
  - Add handoffs for OxVba/shared DTO needs if discovered.

Tests:
  - `cargo test --manifest-path crates/Cargo.toml --workspace`.
  - DnaOxIde scaffold verification.
  - Grep no-claim tokens.

Evidence:
  - Full tests and scaffold/render evidence.

Closure:
  - [ ] W340 accepted or explicitly blocked.
  - [ ] DnaOxIde foundation is reviewable.
  - [ ] Next workset is documented.

## Out-of-scope

- Real OxVba runtime/debug/Immediate execution.
- Native COM discovery or invocation.
- DnaOneCalc repo changes.
- OxVba repo changes.
- Installer signing and production packaging.
- Full accessibility audit/compliance.
- Parked TUI substrate changes.
