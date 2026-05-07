# Workset W230 — Save, Reload, And Session Restore

## Ambition

The GUI has honest document lifecycle behavior: dirty state, save, reload, revert, and session restore work where the host capability profile permits them.

## Dependencies

- W220 — editable module and diagnostics.
- Capability profile model sufficient to distinguish browser-limited and filesystem-capable hosts.

## Design

Document lifecycle must be capability-aware. The same UI must not pretend browser-only mode and desktop filesystem mode have identical persistence power.

Implementation lanes for the first pass:

1. pure dirty-state model and lifecycle command availability,
2. persistence service seam with in-memory supported profile and browser-limited disabled reasons,
3. deterministic lab lifecycle scenario over the W220 thin-slice edit without mutating fixture files,
4. pure session restore model for open project/module/source/dirty state,
5. acceptance evidence that distinguishes supported in-memory persistence from unsupported browser/file-system persistence.

W230 does not need to perform real disk writes to `examples/thin-slice` in its first pass. Real filesystem save can follow once a temporary-fixture strategy is designed that does not hide destructive behavior or mutate canonical examples.

## Beads

### W230-B01 — Pure document lifecycle state and commands

**Infrastructure.**

- **Goal.** `oxide-core` owns a GUI-native document lifecycle model for clean, dirty, saved, reloaded, and reverted source snapshots without importing parked TUI session code.
- **Design.** Add pure types for document lifecycle state, persisted source, working source, dirty flag, and lifecycle command availability. Model commands as state transitions only; no filesystem or OxVba session changes in this bead.
- **Tests.** Unit tests for open clean document, edit makes dirty, save acknowledgement makes clean, reload replaces working text, revert restores persisted text, and unsupported commands report disabled reasons.
- **Evidence.** `cargo test --manifest-path crates/Cargo.toml --workspace` output.
- **Closure.** Lifecycle state is pure, deterministic, and independent of parked TUI code.

### W230-B02 — Persistence capability seam with memory provider and browser-disabled profile

**Infrastructure.**

- **Goal.** OxIde can distinguish persistence-supported and browser-limited hosts without pretending browser-only mode can write local files.
- **Design.** Add a small persistence abstraction in `oxide-core` or a new narrow module as needed. Provide an in-memory provider for deterministic tests and a browser-limited capability response that returns disabled reasons for save/reload/revert where appropriate. Do not mutate checked-in fixtures.
- **Tests.** Unit tests for in-memory save/reload/revert behavior; browser profile returns disabled reasons; persisted and working source remain distinct.
- **Evidence.** GUI workspace test output.
- **Closure.** Capability-aware persistence seam exists; no real disk writes are required or claimed.

### W230-B03 — GUI lab lifecycle scenario rendering

**Feature.**

- **Goal.** `oxide-guilab` renders a deterministic lifecycle scenario that makes dirty state, save/reload/revert availability, and host limitations visible.
- **Design.** Add stable scenario ID `gui-thin-slice-lifecycle`. Reuse the W220 edited source as the dirty working document. Render project/module/source state, dirty/clean labels, lifecycle command availability, and browser-safe capability text. If in-memory save/reload evidence is shown, label it as an in-memory provider rather than filesystem persistence.
- **Tests.** Scenario lookup by ID; render assertions for dirty state, command availability/disabled reasons, in-memory provider label if present, and unchanged W210/W220 scenario tokens.
- **Evidence.** `cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-thin-slice-lifecycle` output.
- **Closure.** Lifecycle behavior is reviewable in the lab without claiming real filesystem save.

### W230-B04 — Session restore model and lab smoke

**Feature.**

- **Goal.** OxIde can serialize/restore the minimal GUI session state needed to reopen the project/module and reconstruct working source/dirty state.
- **Design.** Add pure session snapshot types for workspace path, active module, working source hash or text as appropriate for the first pass, dirty state, and capability profile. Rehydrate into lifecycle state without reading parked TUI session stores.
- **Tests.** Unit tests for snapshot round trip; restored dirty session preserves working source and active module; restored clean session does not invent edits.
- **Evidence.** GUI workspace test output and/or lifecycle lab output showing restored state.
- **Closure.** Session restore model exists as pure GUI state; no multi-project restore claimed.

### W230-B05 — W230 acceptance and W240 handoff

**Doctrine.**

- **Goal.** W230 closes with evidence for honest document lifecycle/session behavior and a precise handoff to W240 capability-aware run/output work.
- **Design.** Update `GUI_FIXTURES_AND_LAB.md`, this workset, and a W240 handoff note with observed commands, tokens, limitations, and run-output prerequisites.
- **Tests.** Rerun GUI workspace tests plus W210/W220/W230 lab render commands.
- **Evidence.** Test output, lab render output, and W240 handoff note.
- **Closure.** W230 acceptance target is satisfied; W240 has explicit prerequisites and no hidden persistence gaps.

## Out-of-scope

- Multi-project workspace restore.
- Advanced conflict resolution.
- Cloud sync or arbitrary host persistence systems.
