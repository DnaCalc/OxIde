# DNA OxIde (`DnaOxIde`)

Status: `scaffold_design_locked`
Date: 2026-05-07
Product name: **DNA OxIde**
Internal app name: `DnaOxIde`
Primary host target: Windows desktop, Tauri-ready
Secondary host target: browser/WASM review profile where capabilities allow

`DnaOxIde` is the branded standalone host for OxIde, presented to users as **DNA OxIde**.

This app folder owns product-host concerns only: app bootstrap, packaging metadata, Tauri configuration, host command adapters, and product branding. Reusable IDE UI, host contracts, project/language/runtime semantics, and DnaOneCalc-reusable components belong under `../../crates/`.

## Capability Boundary

The W341 scaffold does **not** claim real OxVba runtime/debug/Immediate/COM capability.

Current scaffold claim values:

- real execution: `false`
- native runtime: `false`
- COM runtime: `false`
- fake Immediate responses: `false`
- fake debug data: `false`
- real DnaOneCalc host mount: `false`

Available-subset OxVba adapter work is planned in W343-W347. Until adapter tests exist inside OxIde, the app must render pending/unavailable or subset-labeled states rather than full capability claims.

## Locked W341 Scaffold Shape

```text
apps/dna-oxide/
  README.md
  package.json
  Trunk.toml
  index.html
  src/
    main.js
    styles.css
  src-tauri/
    Cargo.toml
    tauri.conf.json
    capabilities/
      default.json
    icons/
      README.md
    src/
      main.rs
      commands.rs
      services.rs
  e2e/
    README.md
```

## File Roles

| Path | Role in W341 | Build participation in W341 | Notes |
| --- | --- | --- | --- |
| `README.md` | product-host scaffold contract | documentation | owns this design lock and no-claim language |
| `package.json` | frontend script metadata | static metadata only | must not require npm install for W341 acceptance |
| `Trunk.toml` | future Trunk/WASM host config | static metadata only | points at `index.html`; later W342/W345 may make it executable |
| `index.html` | frontend host entry | static render/check input | may load `src/main.js` and `src/styles.css`; no shared UI implementation here |
| `src/main.js` | frontend bootstrap placeholder | static render/check input | app-specific glue only; shared UI moves to W342 |
| `src/styles.css` | minimal app shell styling | static render/check input | no product logic |
| `src-tauri/Cargo.toml` | native scaffold crate metadata | `cargo test` for scaffold crate | intentionally no `tauri` dependency in W341 so checks stay deterministic/offline |
| `src-tauri/tauri.conf.json` | Tauri product metadata | static config check | contains `productName = DNA OxIde`; real Tauri runtime wiring comes later |
| `src-tauri/capabilities/default.json` | future Tauri v2 capability placeholder | static config check | keeps permissions conservative |
| `src-tauri/icons/README.md` | icon placeholder | documentation | avoids fake production asset claims |
| `src-tauri/src/main.rs` | native bootstrap placeholder | `cargo test` for scaffold crate | reports scaffold metadata only; not a Tauri runtime claim |
| `src-tauri/src/commands.rs` | command registration shell | `cargo test` for scaffold crate | lists command names and disabled claim flags; implementations come W344 |
| `src-tauri/src/services.rs` | service boundary shell | `cargo test` for scaffold crate | records unavailable runtime/debug/Immediate/COM states |
| `e2e/README.md` | future interaction harness placeholder | documentation | W346 owns real interaction/e2e tests |

## Verification Commands

W341 scaffold verification should not require network access or sibling repo writes.

```powershell
cargo test --manifest-path apps/dna-oxide/src-tauri/Cargo.toml
rg -n "DNA OxIde|DnaOxIde|Tauri|real execution: `false`|native runtime: `false`|COM runtime: `false`|fake Immediate responses: `false`|fake debug data: `false`" apps/dna-oxide
# Anti-overclaim scan: search for the same claim labels above set to an affirmative value.
# The anti-overclaim scan is expected to return no matches.
```

## Workset Links

- [`../../docs/DNA_OXIDE_HOST_PLAN.md`](../../docs/DNA_OXIDE_HOST_PLAN.md)
- [`../../docs/HANDOFF_DNAOXIDE_OXVBA_REQUIREMENTS.md`](../../docs/HANDOFF_DNAOXIDE_OXVBA_REQUIREMENTS.md)
- [`../../docs/HANDOFF_DNAOXIDE_OXVBA_FEEDBACK_ALIGNMENT.md`](../../docs/HANDOFF_DNAOXIDE_OXVBA_FEEDBACK_ALIGNMENT.md)
- [`../../docs/worksets/W341_dnaoxide_tauri_app_scaffold.md`](../../docs/worksets/W341_dnaoxide_tauri_app_scaffold.md)
- [`../../docs/worksets/W342_shared_ide_ui_component_layer.md`](../../docs/worksets/W342_shared_ide_ui_component_layer.md)
- [`../../docs/worksets/W343_oxide_host_bridge_facade.md`](../../docs/worksets/W343_oxide_host_bridge_facade.md)
