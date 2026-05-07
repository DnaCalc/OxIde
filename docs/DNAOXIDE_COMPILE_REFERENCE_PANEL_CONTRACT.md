# DNA OxIde Compile / Reference Panel Contract

Status: `w347_placeholder_panel_contract`
Date: 2026-05-07
Workset: [`W347_compile_options_reference_placeholders.md`](worksets/W347_compile_options_reference_placeholders.md)

## Purpose

This contract defines the DnaOxIde placeholder panel inputs for project properties, compile options, build/check, run target, references, COM candidate discovery, and reference repair/apply preview.

The contract is intentionally **not** an authoritative OxVba DTO definition. OxVba owns final project properties, compile options, run target, build result, reference, COM, source-span, and command availability DTOs.

## State Labels

Panels must use the same evidence labels as W343-W346:

- `proven-oxide-only` — OxIde-owned project/session identity or lifecycle state proven locally.
- `oxvba-available-subset` — direct OxVba Rust surface exists, but DnaOxIde adapter proof remains partial.
- `oxvba-fixture-evidenced` — OxVba ThinSliceHello fixture evidence exists, but DnaOxIde full adapter proof is pending.
- `pending-oxvba-hardening` — final DTO, event stream, source-span mapping, command availability taxonomy, native boundary, or UX adoption is pending.
- `unavailable-no-claim` — no host capability is available in the current proof mode.

## Panel Inputs

### Project Properties

Allowed now:

- project name from proven OxIde state;
- project file name;
- active module name;
- source policy label such as `DiskOnly` or `WorkspaceOverlay` when it is only a displayed placeholder.

Not owned here:

- final OxVba project property DTOs;
- final compile/run target mutation semantics.

### Compile Options

Allowed now:

- placeholder rows labeled `pending-oxvba-hardening`;
- disabled reason pointing to OxVba-owned compile option DTO adoption;
- no mutation claim.

Not owned here:

- final compile options schema;
- compiler flag semantics;
- mutation/apply behavior.

### Build / Check

Allowed now:

- `oxvba-fixture-evidenced` row for `EmbeddedBuildRunHost::build_workspace` ThinSliceHello evidence;
- disabled reason until a local DnaOxIde adapter test proves the command path;
- no fake diagnostics/build output.

Not owned here:

- final build result DTO;
- runtime/source-span mapping.

### Run Target

Allowed now:

- placeholder entrypoint text such as `Module1.Main` only as a disabled UI hint;
- `pending-oxvba-hardening` state until final run target DTO and command availability are adopted.

Not owned here:

- final run target DTO;
- runtime session creation claim.

### References / COM

Allowed now:

- `oxvba-fixture-evidenced` rows for ThinSliceHello broken reference state and COM capability profile;
- `oxvba-available-subset` note for `ComSelectionService` direct Rust surface;
- explicit separation between COM candidate discovery/capability profile and COM runtime invocation.

Not owned here:

- final reference repair/apply DTOs;
- COM native boundary/bitness/apartment truth;
- COM runtime invocation.

## No-Claim Defaults

All placeholder panel outputs must keep these false unless a future direct adapter test proves otherwise:

- `realExecutionClaimed`,
- `nativeRuntimeClaimed`,
- `comRuntimeClaimed`,
- `fakeResponses`,
- `fakeDebugData`.

HTML renders must keep corresponding data attributes false:

- `data-real-execution="false"`,
- `data-native-runtime="false"`,
- `data-com-runtime="false"`,
- `data-fake-responses="false"`,
- `data-fake-debug-data="false"`,
- `data-com-runtime-invocation="false"`.

## Verification Commands

Planned W347 commands:

```powershell
npm --prefix apps/dna-oxide run compile-panels:check
npm --prefix apps/dna-oxide run reference-panels:check
npm --prefix apps/dna-oxide run placeholder-commands:check
```
