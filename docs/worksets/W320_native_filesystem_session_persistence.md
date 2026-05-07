# Workset W320 — Native Filesystem And Session Persistence

## Ambition

OxIde proves real native filesystem/session persistence for the GUI line using test-owned temporary project copies. A user can see save, reload, and session restore as disk-backed capabilities when a native filesystem service is available, while browser/WASM builds continue to report direct filesystem persistence as unavailable.

W320 is an OxIde-only persistence workset. It does not implement DnaOneCalc hosting, OxVba runtime/debug/Immediate execution, or native COM discovery/invocation.

## Dependencies

- W230 — browser-limited lifecycle/session restore seam.
- W290 — `GuiShellPacket` mounted shell state boundary.
- W300 — web-shell no-claim policy for filesystem/native runtime/COM.
- W310 — DnaOneCalc host contract acceptance and next-workset decision.
- [`docs/HANDOFF_W320_NATIVE_FILESYSTEM_SESSION_PERSISTENCE.md`](../HANDOFF_W320_NATIVE_FILESYSTEM_SESSION_PERSISTENCE.md).
- [`docs/GUI_FIXTURES_AND_LAB.md`](../GUI_FIXTURES_AND_LAB.md).

## Guardrails

1. Do not mutate checked-in fixtures, especially `examples/thin-slice/Module1.bas`.
2. Use test-owned temporary directories for all disk-write evidence.
3. Do not delete repo files or directories. Do not run destructive cleanup commands.
4. Keep browser/WASM direct filesystem persistence disabled and explicit.
5. Do not claim native runtime/debug/Immediate, COM runtime, DnaOneCalc browser hosting, or DOM accessibility audit.
6. Do not import parked TUI session-store code; mine behavior only if needed.
7. OxVba owns project/language/runtime truth; OxIde persists GUI document/session state without redefining `.basproj` semantics.

## Design

W320 adds a native filesystem persistence seam in the GUI crate stack and proves it with disk-backed tests that copy the existing thin-slice fixture into a test-owned temporary directory.

The implementation should keep the existing W230 browser-limited profile intact and add a separate native-filesystem profile. Save/reload/session restore can claim disk persistence only where tests write and read from a temporary project copy. GUI-lab output must identify the provider and must state that checked-in fixtures were not mutated.

The core model should continue to separate:

- working source,
- persisted source acknowledgement,
- dirty state,
- reload source,
- session snapshot state.

The native persistence layer may serialize OxIde session snapshots, but it must not duplicate or replace OxVba project semantics.

## Scenario Plan

W320 should add deterministic GUI-lab scenarios:

```text
gui-native-save-reload-disk
gui-native-session-restore-disk
gui-browser-filesystem-still-disabled
```

Required no-claim and proof tokens include:

```text
data-provider="native-filesystem"
data-filesystem-persistence="true"
data-test-owned-temp-project="true"
data-checked-in-fixture-mutated="false"
data-native-runtime="false"
data-com-runtime="false"
data-provider="browser-limited"
data-filesystem-persistence="false"
```

## Beads

### W320-B00 — Register native filesystem/session persistence workset

Goal:
  Register W320 as the next OxIde-only GUI workset after W310 acceptance.

Design:
  - Add `docs/worksets/W320_native_filesystem_session_persistence.md` from the W320 handoff.
  - Update `docs/WORKSET_REGISTER.md` and `docs/worksets/README.md`.
  - Keep paired DnaOneCalc and OxVba native runtime out of scope.

Tests:
  - Documentation review against `HANDOFF_W320_NATIVE_FILESYSTEM_SESSION_PERSISTENCE.md` and W310 acceptance constraints.

Evidence:
  - Registered W320 workset and executable bead list.

Closure:
  - [ ] W320 is in the active sequence.
  - [ ] W320 has concrete beads.
  - [ ] Guardrails require disk-write tests before filesystem persistence is claimed.

### W320-B01 — Native filesystem service seam

Goal:
  Add a native filesystem service seam that writes and reads a test-owned thin-slice project copy without touching checked-in fixtures.

Design:
  - Add a native/filesystem provider type in the GUI crate stack.
  - Copy fixture files into a test-owned temporary project directory.
  - Save edited module text through the provider.
  - Reload module text from disk and prove it matches the saved edit.
  - Keep browser-limited persistence as a separate unsupported provider.

Tests:
  - Native provider writes `Module1.bas` in a temp project copy.
  - Reload returns the saved source.
  - Checked-in fixture content remains unchanged.
  - Browser-limited provider still reports no direct filesystem persistence.

Evidence:
  - `cargo test --manifest-path crates/Cargo.toml -p oxide-core` or the crate that owns the service seam.
  - Targeted test names proving disk write/read/reload.

Closure:
  - [ ] Disk write/read is covered by tests.
  - [ ] Checked-in fixture mutation is guarded by tests.
  - [ ] Browser-limited filesystem remains disabled.

### W320-B02 — Native save/reload GUI projection

Goal:
  Project native disk-backed save/reload lifecycle state into the GUI model.

Design:
  - Add save/reload projection fields that identify native filesystem support.
  - Clear dirty state only after disk-backed save acknowledgement.
  - Show reload source and save acknowledgement as disk-backed evidence.
  - Preserve W230 in-memory/browser-limited lifecycle behavior.

Tests:
  - Dirty edit becomes clean after native disk save.
  - Reload uses disk-backed source.
  - Browser profile still disables save/reload with the existing reason.
  - No native runtime/COM claims are introduced.

Evidence:
  - `cargo test --manifest-path crates/Cargo.toml -p oxide-core`.
  - Targeted lifecycle projection tests.

Closure:
  - [ ] Native save/reload projection is deterministic.
  - [ ] W230 browser lifecycle remains intact.
  - [ ] Runtime/COM no-claim flags remain false.

### W320-B03 — Native session snapshot persistence

Goal:
  Persist and restore an OxIde GUI session snapshot through the native filesystem service.

Design:
  - Serialize a session snapshot for the temp project copy.
  - Reload it through the native provider.
  - Restore project path, active module, working source, persisted source, and dirty state accurately.
  - Keep the session format OxIde-owned; do not redefine `.basproj` semantics.

Tests:
  - Session snapshot writes to disk in a temp location.
  - Restored session preserves active module and source state.
  - Dirty and clean session variants round-trip.
  - Checked-in fixtures remain unchanged.

Evidence:
  - `cargo test --manifest-path crates/Cargo.toml -p oxide-core` or persistence owner crate.
  - Targeted session persistence tests.

Closure:
  - [ ] Session snapshot persistence is disk-backed.
  - [ ] Restore state is deterministic.
  - [ ] Session format remains OxIde-owned and host-independent.

### W320-B04 — Native persistence GUI-lab scenarios

Goal:
  Render deterministic GUI-lab scenarios for native save/reload and session restore, plus a browser profile proving filesystem remains unavailable there.

Design:
  - Add `gui-native-save-reload-disk`.
  - Add `gui-native-session-restore-disk`.
  - Add `gui-browser-filesystem-still-disabled`.
  - Render provider, temp-project, checked-in-fixture, native runtime, and COM no-claim tokens.

Tests:
  - Scenario registry finds all W320 scenario IDs.
  - Native save/reload scenario contains disk-backed persistence tokens.
  - Native session restore scenario contains disk-backed session tokens.
  - Browser scenario preserves filesystem disabled reason.

Evidence:
  - `cargo test --manifest-path crates/Cargo.toml -p oxide-guilab`.
  - Render commands for the three W320 scenarios.

Closure:
  - [ ] GUI-lab can review native persistence evidence.
  - [ ] Browser profile remains honest.
  - [ ] Native runtime/COM remain unclaimed.

### W320-B05 — W320 acceptance and next handoff

Goal:
  Accept W320 and decide whether W330 should be paired DnaOneCalc host implementation or OxVba native runtime/service integration.

Design:
  - Update `docs/GUI_FIXTURES_AND_LAB.md` with W320 scenario tokens.
  - Add a next-workset handoff.
  - Preserve W210-W320 regression renders.

Tests:
  - `cargo test --manifest-path crates/Cargo.toml --workspace`.
  - Render W210-W320 GUI-lab scenarios.
  - Grep native persistence, checked-in-fixture, browser disabled, no-runtime, and no-COM tokens.

Evidence:
  - Full nested workspace tests.
  - Rendered GUI-lab outputs.
  - Handoff note.

Closure:
  - [ ] W320 accepted or explicitly blocked with evidence.
  - [ ] W210-W320 regression scenarios pass.
  - [ ] Next workset prerequisites are documented.

## Out-of-scope

- Writing to the DnaOneCalc repository without explicit authorization.
- Real DnaOneCalc browser host mount.
- Full browser runtime/DOM accessibility audit.
- OxVba runtime/debug/Immediate execution.
- Native COM service implementation.
- OxVba repo changes.
- Parked TUI substrate changes.
