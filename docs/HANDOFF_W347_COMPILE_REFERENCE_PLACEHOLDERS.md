# Handoff — W347 Compile Options And Reference Placeholders

Status: `accepted_dnaoxide_compile_reference_placeholders`
Date: 2026-05-07
Workset: [`W347_compile_options_reference_placeholders.md`](worksets/W347_compile_options_reference_placeholders.md)

## Summary

W347 adds reviewable **DnaOxIde / DNA OxIde** placeholder panels for project properties, compile options, build/check, run target, references, COM candidate discovery, reference repair/apply preview, and COM runtime boundary.

The panels are mounted in the W345 static host shell and use W343-W346 evidence labels. They do not define final OxVba DTOs and do not claim compile/build, reference repair, COM discovery/repair, or COM runtime execution beyond explicit placeholder/fixture labels.

## Contract

Panel contract authority:

- [`docs/DNAOXIDE_COMPILE_REFERENCE_PANEL_CONTRACT.md`](DNAOXIDE_COMPILE_REFERENCE_PANEL_CONTRACT.md)

Core source:

- `apps/dna-oxide/src/placeholder-panels.js`

Verification scripts:

```powershell
npm --prefix apps/dna-oxide run compile-panels:check
npm --prefix apps/dna-oxide run reference-panels:check
npm --prefix apps/dna-oxide run placeholder-commands:check
```

## Delivered Panels

Compile/project panels:

- `role="host-project-properties-panel"` — `proven-oxide-only` project/module identity.
- `role="host-compile-options-panel"` — `pending-oxvba-hardening`; final OxVba compile options DTO pending.
- `role="host-build-check-panel"` — `oxvba-fixture-evidenced`; ThinSliceHello covers `EmbeddedBuildRunHost::build_workspace`, but DnaOxIde adapter proof is pending.
- `role="host-run-target-panel"` — `pending-oxvba-hardening`; run target DTO and command availability taxonomy pending.

Reference/COM panels:

- `role="host-references-panel"` — `oxvba-fixture-evidenced`; reference state seams fixture-evidenced, local adapter proof pending.
- `role="host-com-candidate-panel"` — `oxvba-fixture-evidenced` plus `ComSelectionService direct Rust surface` available-subset note.
- `role="host-reference-repair-panel"` — `pending-oxvba-hardening`; repair/apply DTO pending.
- `role="host-com-runtime-boundary-panel"` — `unavailable-no-claim`; COM runtime invocation false.

## Claim Boundaries

W347 does **not** claim:

- final OxVba project properties DTO ownership,
- final compile options DTO ownership,
- compile option mutation,
- real build/check execution from DnaOxIde,
- final build result DTO ownership,
- run target DTO ownership,
- runtime session creation,
- reference repair/apply behavior,
- COM native boundary/bitness/apartment truth,
- COM runtime invocation.

Empty/failure-proof markers remain explicit:

- `data-output-rows="0"`,
- `data-roster-rows="0"`,
- `data-candidate-rows="0"`,
- `data-preview-rows="0"`,
- `data-com-runtime-invocation="false"`,
- `data-final-oxvba-dtos-owned-here="false"`.

No-claim attributes remain false:

- `data-real-execution="false"`,
- `data-native-runtime="false"`,
- `data-com-runtime="false"`,
- `data-fake-responses="false"`,
- `data-fake-debug-data="false"`.

## Acceptance Evidence

Acceptance evidence is captured in `target/w347-acceptance.txt`.

Commands run:

```powershell
npm --prefix apps/dna-oxide run compile-panels:check
npm --prefix apps/dna-oxide run reference-panels:check
npm --prefix apps/dna-oxide run placeholder-commands:check
npm --prefix apps/dna-oxide run host-ui:check
npm --prefix apps/dna-oxide run interaction-services:check
npm --prefix apps/dna-oxide run scaffold:check
cargo test --manifest-path crates/Cargo.toml --workspace
```

Additional acceptance checks:

- rendered placeholder token grep;
- source/contract token grep;
- anti-overclaim scan for DTO ownership, fake rows, compile/reference mutation, COM runtime, and execution claims.

Observed non-blocking warning class:

- frozen OxVba `unexpected cfg condition name: kani` / dead-code warnings.

## Next Workset

W348 should prove, inside the OxIde repo only, that DnaOneCalc can reuse shared UI and host bridge contracts without depending on DnaOxIde app code and without sibling repo writes. Real DnaOneCalc mount remains pending explicit authorization.
