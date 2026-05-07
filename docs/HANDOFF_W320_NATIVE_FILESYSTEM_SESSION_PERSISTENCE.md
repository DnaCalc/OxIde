# Handoff — W320 Native Filesystem And Session Persistence

Status: `ready_for_workset_registration`
Date: 2026-05-07
Source workset: W310 — DnaOneCalc Web Shell Hosting

## Decision

W320 should stay inside the OxIde repository and prove native filesystem/session persistence before any product copy claims real save/reload durability.

Paired DnaOneCalc implementation remains a cross-repo follow-up and requires explicit sibling-repo write authorization. OxVba runtime/native service integration remains a later seam because W270/W300/W310 still expose honest disabled states and no runtime/COM execution is claimed.

## Ambition

Turn the W230 browser-limited lifecycle seam into a native-capability persistence proof:

```text
Open a copy of the thin-slice fixture in a test-owned temporary directory
  -> edit Module1.bas in memory
  -> save through an OxIde native filesystem service
  -> reload from disk
  -> restore a session snapshot that points at the saved project/module
  -> prove checked-in fixtures were not mutated
```

## Guardrails

1. Do not mutate checked-in fixtures, especially `examples/thin-slice/Module1.bas`.
2. Use test-owned temporary directories for disk-write evidence.
3. Do not delete files or directories from the repo. Temporary test directories may be created by test harnesses in OS temp locations; repo files are not removed by agent commands.
4. Keep browser/WASM filesystem limitations explicit.
5. Do not claim native runtime/debug/Immediate or COM capability.
6. Do not import parked TUI session-store code. Mine behavior only if needed.
7. Keep OxVba project semantics authoritative; OxIde should persist text/session state, not redefine `.basproj` truth.

## Suggested Bead Shape

### W320-B00 — Register native persistence workset

- Add `docs/worksets/W320_native_filesystem_session_persistence.md`.
- Update `docs/WORKSET_REGISTER.md` and `docs/worksets/README.md`.
- Keep paired DnaOneCalc and OxVba native runtime work out of scope.

### W320-B01 — Native filesystem service seam

- Add a pure/native persistence abstraction in the GUI crate stack.
- Include browser-limited and native-capable capability labels.
- Unit test write/read/reload against a test-owned temp project copy.

### W320-B02 — Save/reload GUI projection

- Add GUI state/projection for native save/reload success.
- Preserve W230 disabled browser profile.
- Test that dirty state clears only after disk-backed save acknowledgement.

### W320-B03 — Session snapshot persistence

- Persist and reload a session snapshot through the native service.
- Test active module, working source, dirty state, and project path identity after restart.

### W320-B04 — GUI-lab native persistence scenarios

Suggested scenario IDs:

```text
gui-native-save-reload-disk
gui-native-session-restore-disk
gui-browser-filesystem-still-disabled
```

Required evidence tokens:

```text
data-filesystem-persistence="true"
data-provider="native-filesystem"
data-test-owned-temp-project="true"
data-checked-in-fixture-mutated="false"
data-native-runtime="false"
data-com-runtime="false"
```

### W320-B05 — Acceptance and next handoff

- Run full nested workspace tests.
- Render W210-W320 GUI-lab regression scenarios.
- Grep native persistence and no-runtime/COM tokens.
- Decide whether W330 should be paired DnaOneCalc host implementation or OxVba native runtime/service integration.

## Required Tests

```powershell
cargo test --manifest-path crates/Cargo.toml --workspace
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-native-save-reload-disk
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-native-session-restore-disk
cargo run --manifest-path crates/Cargo.toml -p oxide-guilab -- render gui-browser-filesystem-still-disabled
```

## Not Claimed By W320

- real DnaOneCalc host implementation,
- OxVba runtime/debug/Immediate execution,
- COM discovery or invocation,
- full browser DOM runtime,
- full accessibility audit/compliance.
