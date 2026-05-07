# Workset W343 — OxIde Host Bridge Facade

## Ambition

Create the host bridge facade that lets shared UI talk to a host implementation without knowing whether it is running inside **DnaOxIde**, a standalone browser/WASM review shell, or a future DnaOneCalc mount.

The bridge names the service surfaces needed for full scope while preserving unavailable states until OxVba-backed APIs are ready.

## Dependencies

- W310 — DnaOneCalc web shell hosting contract.
- W330 — runtime/debug/Immediate service contract packets.
- W342 — shared IDE UI component layer.
- [`docs/HANDOFF_DNAOXIDE_OXVBA_REQUIREMENTS.md`](../HANDOFF_DNAOXIDE_OXVBA_REQUIREMENTS.md).
- [`docs/HANDOFF_DNAOXIDE_OXVBA_FEEDBACK_ALIGNMENT.md`](../HANDOFF_DNAOXIDE_OXVBA_FEEDBACK_ALIGNMENT.md).

## Design

Expected crate: `crates/oxide-host-bridge`.

The facade should define host-neutral service categories:

- `HostProjectApi`,
- `HostDocumentApi`,
- `HostLanguageApi`,
- `HostCompileApi`,
- `HostReferenceApi`,
- `HostRuntimeApi`,
- `HostImmediateApi`,
- `HostDebugApi`,
- `HostSettingsApi`,
- `HostCapabilityApi`.

The bridge may define OxIde-side request/response wrappers and capability states. It must not duplicate final OxVba-owned DTOs for compile options, COM references, runtime sessions, Immediate responses, debug frames, watches, or breakpoints. Where OxVba DTOs are missing, use placeholder unavailable responses and handoff references.

Confirmed OxVba feedback changes this workset from pure blank-stub planning to a split bridge:

- available-subset adapter targets: `HostWorkspaceSession`, `inspect_workspace_target`, `ComSelectionService`, `EmbeddedBuildRunHost`, `EmbeddedRunSession`, `ImmediateSession`, and `DebugSession`;
- pending-hardening gaps: stable IDs, shared capability/error taxonomy, unified workspace/project/module DTOs, compile options/run target DTOs, request IDs, event streams, runtime source-span mapping, Immediate/debug attach hardening, watch/breakpoint DTOs, COM capability profile, bitness/apartment/native boundary status;
- claim gates: no full runtime/debug/Immediate/COM claim flips until OxIde has direct adapter tests over matching OxVba evidence.

## Beads

### W343-B00 — Host service map

Goal:
  Document and test the host service category map.

Design:
  - Map each shared UI command to a host API category.
  - Identify which APIs can be implemented now from OxIde/W320 state.
  - Identify which APIs can be implemented as OxVba available-subset adapters.
  - Identify which APIs remain pending OxVba hardening.

Tests:
  - Documentation grep for all API category names.
  - No duplicate OxVba type names unless explicitly wrappers/placeholders.

Evidence:
  - Host bridge service map.

Closure:
  - [ ] All service categories are named.
  - [ ] Implement-now, available-subset adapter, and pending-hardening categories are separated.
  - [ ] Ownership boundaries are explicit.

### W343-B01 — Host bridge crate scaffold

Goal:
  Add the host bridge crate with compile-time role markers and basic API traits/types.

Design:
  - Add `crates/oxide-host-bridge` to the nested GUI workspace.
  - Depend on `oxide-core`/`oxide-bridge` as needed.
  - Keep dependencies host-neutral.

Tests:
  - `cargo test --manifest-path crates/Cargo.toml -p oxide-host-bridge` or workspace equivalent.

Evidence:
  - Host bridge crate test output.

Closure:
  - [ ] Crate exists.
  - [ ] Host-neutral traits compile.
  - [ ] No Tauri dependency is introduced.

### W343-B02 — Proven and available-subset service implementations as fixtures

Goal:
  Provide test/demo implementations for capabilities proven inside OxIde and adapter probes for currently available OxVba subsets.

Design:
  - Implement in-memory/browser-limited fixture host.
  - Implement native-filesystem fixture host against test-owned temp projects if useful.
  - Add adapter-shaped seams for current OxVba direct surfaces where dependency wiring is available.
  - Runtime/Immediate/debug/COM return typed unavailable or subset-backed partial packets until full claim evidence exists.

Tests:
  - Fixture host tests for project/document lifecycle.
  - Available-subset adapter tests where OxVba dependency wiring is ready.
  - Unavailable-state tests for runtime/Immediate/debug/COM gaps.

Evidence:
  - Host bridge fixture test output.

Closure:
  - [ ] Proven project/document paths work in fixtures.
  - [ ] Available-subset adapters are separated from full capability claims.
  - [ ] Runtime/debug/Immediate/COM gap states are preserved where hardening is missing.
  - [ ] No fake data is returned.

### W343-B03 — Shared UI command dispatch integration

Goal:
  Connect shared UI command intents to host bridge request categories.

Design:
  - Use abstract command intents, not Tauri invocations.
  - Keep command availability synchronized with host capability packets.

Tests:
  - Command mapping tests.
  - GUI-lab render shows disabled reasons from bridge state.

Evidence:
  - Command mapping evidence.

Closure:
  - [ ] UI commands map to bridge categories.
  - [ ] Disabled reasons remain visible.
  - [ ] DnaOneCalc can implement the same facade later.

### W343-B04 — W343 acceptance

Goal:
  Accept the host bridge facade as the common service boundary.

Design:
  - Update relevant handoff docs.
  - Link W344 Tauri command stub work.

Tests:
  - Workspace tests.
  - Host bridge tests.
  - Anti-overclaim scan.

Evidence:
  - W343 acceptance outputs.

Closure:
  - [ ] Bridge facade is reviewable.
  - [ ] Shared UI can target it.
  - [ ] OxVba-blocked services stay unavailable.

## Out-of-scope

- DnaOxIde Tauri IPC implementation; W344 owns it.
- Full OxVba compile/runtime/debug/Immediate/COM behavior beyond available-subset adapter proofs.
- Final shared OxVba DTO definitions.
- DnaOneCalc repo implementation.
