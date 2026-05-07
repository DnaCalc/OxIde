# Workset W342 — Shared IDE UI Component Layer

## Ambition

Create the shared OxIde GUI component layer that **DnaOxIde** can host and **DnaOneCalc** can reuse, without coupling reusable UI to Tauri, DnaOxIde product policy, or OxVba implementation details.

This workset moves accepted shell presentation from static/web-shell rendering toward reusable UI components while preserving deterministic GUI-lab evidence.

## Dependencies

- W290 — host-mounted GUI shell.
- W300 — mounted web shell adapter.
- W310 — DnaOneCalc web shell hosting contract.
- W341 — DnaOxIde Tauri app scaffold.
- [`docs/HANDOFF_DNAOXIDE_OXVBA_FEEDBACK_ALIGNMENT.md`](../HANDOFF_DNAOXIDE_OXVBA_FEEDBACK_ALIGNMENT.md).

## Design

W342 should introduce a shared component crate, expected name `oxide-ui-leptos` unless implementation evidence suggests a different Rust/WASM UI crate name.

Confirmed OxVba feedback means components should accept both unavailable/no-claim packets and available-subset adapter packets. Visual labels must distinguish subset-backed OxVba data from full capability claims.

Rules:

1. Components consume OxIde view models/packets such as `GuiShellPacket`.
2. Components dispatch abstract host commands rather than calling Tauri directly.
3. Components render unavailable/no-claim runtime/debug/Immediate/COM states honestly.
4. DnaOneCalc must be able to mount the same component tree or consume the same render packets without depending on `apps/dna-oxide`.
5. `oxide-guilab` remains the deterministic review driver.
6. Available-subset OxVba data must carry visible subset/provenance labels until full stable IDs/taxonomy/source-span/watch/breakpoint/COM-runtime evidence lands.

Initial component targets:

- shell frame,
- project spine,
- source editor placeholder/component boundary,
- diagnostics panel,
- lifecycle/save-reload status,
- run/output panel,
- command palette surface,
- focus/accessibility labels,
- runtime/Immediate/debug unavailable states,
- COM capability panel.

## Beads

### W342-B00 — Component API design

Goal:
  Define the shared component crate API and dependency boundaries.

Design:
  - Decide component crate name and feature flags.
  - Define input view models from `oxide-core`/`oxide-bridge`.
  - Define abstract command dispatch shape without Tauri.

Tests:
  - Dependency grep verifies no app-folder dependency.
  - Documentation check verifies DnaOneCalc reuse path.

Evidence:
  - Component API notes or crate README.

Closure:
  - [ ] Component boundary is documented.
  - [ ] DnaOneCalc reuse path is explicit.
  - [ ] Tauri-specific code is excluded.

### W342-B01 — Shared shell component scaffold

Goal:
  Add the shared UI crate and first shell component around `GuiShellPacket`.

Design:
  - Add crate under `crates/`.
  - Render shell identity, project/module names, lifecycle state, diagnostics summary, and capability footer.
  - Keep initial output deterministic for tests.

Tests:
  - `cargo test --manifest-path crates/Cargo.toml -p oxide-ui-leptos` or workspace equivalent.
  - Snapshot/string tests for accepted shell tokens.

Evidence:
  - Shared shell component test output.

Closure:
  - [ ] Crate exists.
  - [ ] Shell component consumes shared packet state.
  - [ ] Tests pass.

### W342-B02 — Accepted panes as components

Goal:
  Add reusable components for accepted panes without adding real runtime/debug/COM behavior.

Design:
  - Add project spine, diagnostics, lifecycle, run/output, command palette, runtime, Immediate, debug, and COM capability components.
  - Inputs remain pure packets/view models.

Tests:
  - Component unit tests.
  - `oxide-guilab` scenario render still deterministic.

Evidence:
  - Component render evidence for accepted W210-W330 slices.

Closure:
  - [ ] Accepted panes render from components.
  - [ ] No fake runtime/debug/Immediate data is introduced.
  - [ ] COM runtime remains unavailable unless proven.

### W342-B03 — GUI-lab component route

Goal:
  Make `oxide-guilab` render through or alongside the shared components.

Design:
  - Preserve existing scenario IDs.
  - Add component-backed scenario IDs only if needed.
  - Keep parsed/static evidence deterministic.

Tests:
  - Existing GUI-lab scenario renders.
  - Component-backed smoke render.

Evidence:
  - `target/w342-guilab-renders.txt`.

Closure:
  - [ ] Existing renders are not regressed.
  - [ ] Shared component render is reviewable.
  - [ ] Scenario evidence remains deterministic.

### W342-B04 — W342 acceptance

Goal:
  Accept the shared UI layer as the reusable presentation foundation for DnaOxIde and DnaOneCalc.

Design:
  - Update `docs/GUI_FIXTURES_AND_LAB.md`.
  - Link W343 host bridge expectations.

Tests:
  - Workspace tests.
  - Component render tests.
  - Anti-overclaim scan.

Evidence:
  - W342 acceptance outputs.

Closure:
  - [ ] Shared UI is usable by DnaOxIde.
  - [ ] DnaOneCalc reuse remains possible.
  - [ ] No unproven capability is claimed.

## Out-of-scope

- Tauri native command implementation; W344 owns it.
- Live Tauri/WebView smoke; W345/W346 own it.
- Real OxVba runtime/debug/Immediate/COM data.
- DnaOneCalc repo changes.
