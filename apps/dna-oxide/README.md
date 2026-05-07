# DNA OxIde (`DnaOxIde`)

Status: `planned_host_scaffold`
Date: 2026-05-07

`DnaOxIde` is the planned branded standalone host for OxIde, presented to users as **DNA OxIde**.

Primary target:

- Windows desktop host using Tauri.

Secondary target:

- standalone browser/WASM host profile for capability-limited review/use where appropriate.

This directory intentionally starts with planning documentation only. The first implementation workset should scaffold the Tauri app here without claiming real OxVba runtime/debug/Immediate/COM capability until native-service tests prove it.

Planned shape:

```text
apps/dna-oxide/
  package.json
  Trunk.toml
  index.html
  src/
  src-tauri/
    Cargo.toml
    tauri.conf.json
    capabilities/
    icons/
    src/
  e2e/
```

Reusable UI and host contracts should remain outside this app, under `crates/`, so DnaOneCalc can present the same OxIde IDE surface.

See:

- [`../../docs/DNA_OXIDE_HOST_PLAN.md`](../../docs/DNA_OXIDE_HOST_PLAN.md)
- [`../../docs/worksets/W340_dnaoxide_standalone_host_foundation.md`](../../docs/worksets/W340_dnaoxide_standalone_host_foundation.md)
