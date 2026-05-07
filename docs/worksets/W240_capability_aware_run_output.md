# Workset W240 — Capability-Aware Run And Output

## Ambition

The GUI exposes run/output behavior through explicit host capability profiles: unsupported hosts explain why they cannot run, while supported native paths produce structured run/output events.

## Dependencies

- W230 — document lifecycle baseline.
- OxVba build/run host contracts.
- Capability profile model.

## Design

Run should be a visible IDE workflow, not a raw log append.

Implementation lanes for the first pass:

1. OxIde-owned runtime capability profile with explicit browser-safe disabled reasons,
2. pure run request/event/output model before any native/OxVba execution wiring,
3. deterministic lab output/activity surface for unsupported browser run state,
4. explicitly simulated supported run-output provider for reviewable happy-path UI evidence,
5. acceptance evidence that keeps W210-W230 project/edit/diagnostic/lifecycle scenarios as regressions.

W240 does not need to introduce real native execution or COM. Browser-safe mode must remain unable to run. Any supported run proof in this workset must be labeled simulated unless it is backed by a tested real provider.

## Beads

### W240-B01 — Runtime capability model and disabled reasons

**Infrastructure.**

- **Goal.** `oxide-core` can describe run capability independently of host UI and report why browser-safe mode cannot execute VBA.
- **Design.** Add pure run capability/profile types with explicit labels for browser-safe unsupported, simulated-supported, and future native-supported modes. Keep these as OxIde presentation/orchestration state, not copies of OxVba runtime enums.
- **Tests.** Unit tests for browser-safe disabled reason, simulated-supported availability, and no COM/native claim in browser-safe profile.
- **Evidence.** `cargo test --manifest-path crates/Cargo.toml --workspace` output.
- **Closure.** Run capability state exists; unsupported reasons are explicit; no native execution is claimed.

### W240-B02 — Pure run request/event/output protocol

**Infrastructure.**

- **Goal.** OxIde has a GUI-neutral run request and structured output event model before any real provider wiring.
- **Design.** Add request types for project/module/entrypoint, output event rows for lifecycle/activity/diagnostic/stdout-like messages, and a deterministic simulated provider that emits labeled events. Do not call OxVba execution yet unless the provider is explicitly added and tested.
- **Tests.** Unit tests for request construction, browser-disabled result, simulated provider event order, and structured event labels.
- **Evidence.** GUI workspace test output.
- **Closure.** Run/output state is structured and deterministic; no raw-log-only UI contract.

### W240-B03 — Browser unsupported run lab scenario

**Feature.**

- **Goal.** `oxide-guilab` renders browser-safe run/output state showing that run is unavailable and why.
- **Design.** Add stable scenario ID `gui-run-output-browser-disabled`. Use the existing thin-slice project and current browser-safe capability text. Render project/module context, run command disabled state, disabled reason, and an output/activity region that records the unsupported run request.
- **Tests.** Scenario lookup by ID; render assertions for scenario ID, project/module, run-disabled reason, output/activity region, and unchanged W210-W230 scenario tokens.
- **Evidence.** `cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-run-output-browser-disabled` output.
- **Closure.** Browser unsupported run behavior is reviewable and honest.

### W240-B04 — Simulated supported run-output scenario

**Feature.**

- **Goal.** `oxide-guilab` renders a supported run-output happy path without claiming native execution.
- **Design.** Add stable scenario ID `gui-run-output-simulated-supported`. Use the pure simulated provider from W240-B02 and label the provider as `simulated`. Render request, lifecycle events, deterministic output rows, completion status, and capability/profile label.
- **Tests.** Scenario lookup by ID; render assertions for simulated label, run started/completed events, deterministic output token, and absence of COM/native claims.
- **Evidence.** `cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-run-output-simulated-supported` output.
- **Closure.** Supported run-output UI shape is reviewable while remaining clearly simulated.

### W240-B05 — W240 acceptance and W250 handoff

**Doctrine.**

- **Goal.** W240 closes with evidence for capability-aware run/output behavior and a precise handoff to W250 DnaOneCalc embedding proof.
- **Design.** Update `GUI_FIXTURES_AND_LAB.md`, this workset, and a W250 handoff note with observed commands, tokens, limitations, and embedding prerequisites.
- **Tests.** Rerun GUI workspace tests plus W210-W240 lab render commands.
- **Evidence.** Test output, lab render output, and W250 handoff note.
- **Closure.** W240 acceptance target is satisfied; W250 has explicit prerequisites and no hidden run/output gaps.

## Out-of-scope

- Debugger surfaces.
- Immediate Window.
- Windows COM invocation.
- DnaOneCalc embedding.
