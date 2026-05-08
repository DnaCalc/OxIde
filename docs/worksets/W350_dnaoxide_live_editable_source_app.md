# Workset W350 — DnaOxIde Live Editable Source App

## Ambition

Make **DNA OxIde / DnaOxIde** run as a live editable host surface where a user can change a module source text buffer, see dirty lifecycle state update, save the change to a test-owned project copy, and reload it without mutating checked-in fixtures.

W350 also establishes the instrumentation loop required for safe progress: every meaningful app step must be visible through both visual artifacts and DOM-like state snapshots, and automation must be able to inject interactions and observe before/after effects.

W350 exists because W341-W349 proved the host runway, command boundaries, static UI proof, and reuse path, but did not yet provide a live app/editor loop.

## Dependencies

- W220 — editable module and diagnostics model/lab proof.
- W230 — document lifecycle state and session restore.
- W320 — native filesystem/session persistence over temp project copies.
- W344 — DnaOxIde Rust-callable command boundary for project/document/session lifecycle.
- W345 — DnaOxIde static frontend host UI proof.
- W346 — DnaOxIde interaction harness.
- W349 — DnaOxIde while-OxVba acceptance and readiness report.
- [`docs/DNAOXIDE_OXVBA_INTEGRATION_READINESS.md`](../DNAOXIDE_OXVBA_INTEGRATION_READINESS.md).

## Design

W350 is the bridge from static proof to live source editing. It should:

1. select a concrete live proof mode: browser DOM first if Tauri/WebView is not ready, live Tauri/WebView only if the toolchain can be proven locally;
2. add app observability before editing: visual snapshot, DOM-like snapshot, state transition log, command log, and interaction injection API;
3. add an editable source component or host pane that accepts source text input events;
4. keep source editing state in OxIde-owned editor/lifecycle models, not in OxVba-owned project truth;
5. call the W344 command boundary for open/load/save/reload/session operations over temp project copies;
6. write only under `target/` or another explicitly test-owned temp directory;
7. keep checked-in fixtures unchanged;
8. keep runtime/debug/Immediate/COM claims false.

The first accepted live app may still be simple: one project, one module, one editable source pane, lifecycle controls, command/status output, and instrumentation artifacts. The goal is a real edit/save/reload loop with a repeatable feedback harness, not full IDE polish.

## Instrumentation Contract

W350 must produce enough app instrumentation to support agentic feedback loops:

- **visual artifact** — stable HTML/screenshot-equivalent capture under `target/` for human review;
- **DOM-like snapshot** — machine-readable state or HTML with stable roles/data attributes for project, module, editor text, dirty state, lifecycle commands, command results, focus, and no-claim flags;
- **event log** — ordered input/state/command events, including source edits and lifecycle transitions;
- **command log** — host command invocations, request IDs where available, disabled reasons, and returned packet category;
- **interaction injection** — a driver API/script that can focus the source editor, type text, trigger save/reload/session commands, and capture before/after snapshots;
- **no-claim guard** — every artifact keeps runtime/debug/Immediate/COM, fake data, DOM-audit, and sibling-write claims false unless separately proven.

If Playwright/WebDriver is available locally, use it. If not, the selected proof mode must still provide deterministic injection and before/after observation through a bounded DOM-like driver.

## Beads

### W350-B00 — Live editable proof mode decision

Goal:
  Decide and document the first live app proof mode for DnaOxIde editable source text and instrumentation.

Design:
  - Choose browser DOM, Tauri/WebView, or a staged hybrid based on available local toolchain evidence.
  - Define what is actually driven: DOM input, command calls, temp filesystem writes, reload, visual capture, DOM-like snapshot, event log, and command trace.
  - Define what remains unclaimed: runtime/debug/Immediate/COM, full accessibility audit, real DnaOneCalc mount.

Tests:
  - Documentation grep for selected proof mode, editable source, instrumentation artifacts, interaction injection, temp project copy, and no-claim flags.

Evidence:
  - `docs/DNAOXIDE_LIVE_EDITABLE_PROOF_MODE.md`.
  - `target/w350-b00-live-editable-proof-mode.txt`.

Closure:
  - [ ] Live proof mode is explicit.
  - [ ] Instrumentation contract is explicit.
  - [ ] Toolchain dependency is explicit.
  - [ ] No unsupported capability is claimed.

### W350-B01 — App observability and interaction instrumentation

Goal:
  Add full DnaOxIde app instrumentation so automation can see visual output, DOM-like state, command/event traces, and inject user-like interactions before live editing work begins.

Design:
  - Define stable data attributes, DOM snapshot shape, visual artifact, command log, state transition log, and interaction injection API for the DnaOxIde host path.
  - Instrument app host glue without importing Tauri globals into shared UI.
  - Capture before/after snapshots for injected focus/command/input probes.
  - Keep runtime/debug/Immediate/COM claims false.

Tests:
  - Instrumentation verifier captures visual artifact, DOM-like snapshot, command/event log, and injected no-op/focus/command effect.
  - Anti-overclaim scan.

Evidence:
  - `docs/DNAOXIDE_APP_INSTRUMENTATION.md`.
  - `target/w350-b01-app-instrumentation.txt`.

Closure:
  - [ ] Visual snapshot is generated.
  - [ ] DOM-like state snapshot is generated.
  - [ ] Interaction injection produces observable before/after effects.
  - [ ] Command/event traces are captured.

### W350-B02 — Editable source component/model boundary

Goal:
  Provide a reusable editable source pane boundary that can receive text changes and report dirty lifecycle state.

Design:
  - Reuse `oxide-editor-core` and `oxide-core` lifecycle types where possible.
  - Add shared UI or host view-model fields for editable source, dirty flag, active module, and command availability.
  - Keep shared UI free of direct Tauri imports/globals.
  - Preserve W350 instrumentation hooks for DOM-like snapshots and event logs.

Tests:
  - Unit tests for edit event application, dirty state, revert/reload state, instrumentation snapshot state, and no app/Tauri coupling.

Evidence:
  - `target/w350-b02-editable-source-boundary.txt`.

Closure:
  - [ ] Editable source state is modeled.
  - [ ] Dirty state changes on edit.
  - [ ] Shared boundary remains host-neutral.

### W350-B03 — DnaOxIde live editable host mount

Goal:
  Mount a live DnaOxIde source editor pane that accepts user text input and is visible through W350 instrumentation.

Design:
  - Extend `apps/dna-oxide/src/host-shell.js` or successor host glue with an actual editable control.
  - Render ThinSliceHello / Module1.bas from a temp project copy.
  - Wire input/change events to update working source, dirty lifecycle display, DOM-like snapshot, and event log.
  - Do not mutate checked-in fixture files.

Tests:
  - Live DOM/WebView or chosen driver smoke proves text input changes rendered/source state.
  - Instrumentation captures before/after DOM-like snapshots and visual artifact.
  - Fixture mutation guard.

Evidence:
  - `target/w350-b03-live-editable-host.txt`.
  - Optional rendered/live capture artifact under `target/`.

Closure:
  - [ ] User-editable source pane is live.
  - [ ] Dirty state updates after input.
  - [ ] Instrumentation sees the input effect.
  - [ ] Checked-in fixture remains unchanged.

### W350-B04 — Save/reload through DnaOxIde command boundary

Goal:
  Saving from the live app persists edited module text to a temp project copy, and reload reads it back, with before/after effects visible through instrumentation.

Design:
  - Use W344 Rust-callable commands or a thin command-client bridge around them.
  - Keep temp project roots under `target/`.
  - Show command results and disabled reasons in the host UI.
  - Capture command traces and DOM-like state transitions.
  - Preserve session snapshot save/load where practical.

Tests:
  - Command tests and live interaction tests for edit -> save -> reload.
  - Instrumentation asserts command log and source text before/after.
  - Fixture mutation guard for `examples/thin-slice/Module1.bas`.

Evidence:
  - `target/w350-b04-live-save-reload.txt`.

Closure:
  - [ ] Save writes edited text to temp project copy.
  - [ ] Reload returns saved text.
  - [ ] Command/event traces prove the flow.
  - [ ] Session/lifecycle state is honest.

### W350-B05 — Live edit feedback-loop regression

Goal:
  Add repeatable automation that drives the live edit/save/reload loop and records visual, DOM-like, command, and event evidence for every run.

Design:
  - Prefer Playwright/WebDriver if available and local; otherwise use the bounded live-driver chosen in W350-B00.
  - Drive user-like input into the source pane.
  - Assert dirty, saved, reloaded, and no-claim states by stable tokens/data attributes.
  - Store machine-readable feedback artifacts under `target/`.

Tests:
  - `npm --prefix apps/dna-oxide run live-edit:check` or equivalent.
  - Anti-overclaim scan.

Evidence:
  - `target/w350-b05-live-edit-feedback-loop.txt`.

Closure:
  - [ ] Automation drives live input.
  - [ ] Save/reload is covered.
  - [ ] Visual/DOM/event artifacts are produced.
  - [ ] No unsupported capability is claimed.

### W350-B06 — W350 acceptance

Goal:
  Accept DnaOxIde as a live editable source app baseline with visual/DOM-like instrumentation and automated interaction feedback loop.

Design:
  - Run W350 instrumentation, live edit, save/reload, and feedback-loop checks.
  - Run W344 command tests, W345 host UI checks, W346 interaction checks, and W349 no-claim scans.
  - Add a handoff for W355 compile/build adapter work.

Tests:
  - Full W350 verifier set.
  - `cargo test --manifest-path apps/dna-oxide/src-tauri/Cargo.toml`.
  - `cargo test --manifest-path crates/Cargo.toml --workspace`.
  - Final fixture mutation and anti-overclaim scans.

Evidence:
  - `target/w350-acceptance.txt`.
  - `docs/HANDOFF_W350_LIVE_EDITABLE_SOURCE_APP.md`.

Closure:
  - [ ] DnaOxIde has a live editable source loop.
  - [ ] Instrumentation can see visual/DOM-like state and injected effects.
  - [ ] Save/reload uses temp project copies.
  - [ ] W355 compile/build adapter work is unblocked.

## Out-of-scope

- Full language-service UX beyond already proven diagnostics/seams.
- Full OxVba build/runtime/debug/Immediate/COM execution.
- COM runtime invocation.
- Real DnaOneCalc mount.
- Writing to checked-in fixtures or sibling repos.
