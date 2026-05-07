# Workset W341 — DnaOxIde Tauri App Scaffold

## Ambition

Create the first reviewable **DnaOxIde / DNA OxIde** application scaffold under `apps/dna-oxide/`, shaped for a Windows desktop Tauri host but still honest about unavailable runtime/debug/Immediate/COM capabilities.

This workset turns the current app-directory planning into a concrete host skeleton that later worksets can build and test without waiting for OxVba runtime service work.

## Dependencies

- W340 — DnaOxIde standalone host foundation.
- [`docs/DNA_OXIDE_HOST_PLAN.md`](../DNA_OXIDE_HOST_PLAN.md).
- [`docs/HANDOFF_DNAOXIDE_OXVBA_REQUIREMENTS.md`](../HANDOFF_DNAOXIDE_OXVBA_REQUIREMENTS.md).

## Design

W341 owns the product-host folder and metadata only. It may add Tauri-ready structure, config templates, host README updates, and minimal build scripts. It must not put reusable IDE behavior into the app folder.

Target shape:

```text
apps/dna-oxide/
  README.md
  package.json
  Trunk.toml
  index.html
  src/
    main.rs or app.rs            # frontend entry glue only
  src-tauri/
    Cargo.toml
    tauri.conf.json              # productName = DNA OxIde
    capabilities/
    src/
      main.rs                    # Tauri bootstrap only
      commands.rs                # command registration shell only
      services.rs                # host service wiring shell only
```

The first scaffold may use placeholder frontend/native entry points, but any executable code must be covered by deterministic checks. Runtime, Immediate, debug, and COM commands must remain unavailable/no-claim until OxVba-backed evidence exists.

## Beads

### W341-B00 — App scaffold design lock

Goal:
  Lock the exact app scaffold files and build expectations before adding executable host files.

Design:
  - Confirm Tauri v2-friendly directory shape.
  - Record product name `DNA OxIde` and internal app name `DnaOxIde`.
  - Decide which files are documentation-only versus build-participating.

Tests:
  - Documentation grep for `DNA OxIde`, `DnaOxIde`, `Tauri`, and no-claim language.

Evidence:
  - Updated scaffold notes in `apps/dna-oxide/README.md` or this workset.

Closure:
  - [ ] Scaffold file list is explicit.
  - [ ] Branding is explicit.
  - [ ] No runtime/debug/COM claim is introduced.

### W341-B01 — Frontend shell entry scaffold

Goal:
  Add the frontend entry files needed for a future Tauri/WebView host.

Design:
  - Add minimal `index.html` and frontend source entry.
  - Keep frontend app-specific glue thin.
  - Do not duplicate shared UI components here.

Tests:
  - File existence check.
  - Static grep confirms no direct Tauri/OxVba coupling inside frontend entry beyond host bootstrap naming.

Evidence:
  - `apps/dna-oxide/index.html` and frontend entry files.

Closure:
  - [ ] Frontend entry exists.
  - [ ] Shared UI remains outside the app folder.
  - [ ] No real runtime/debug/COM behavior is wired.

### W341-B02 — Tauri native shell scaffold

Goal:
  Add `src-tauri` scaffold metadata and bootstrap shell.

Design:
  - Add Tauri config with product metadata.
  - Add native command registration placeholders.
  - Keep native services as missing/unavailable until later worksets.

Tests:
  - Tauri config/static validation if dependencies allow.
  - Grep for `productName`, `DNA OxIde`, and disabled service language.

Evidence:
  - `apps/dna-oxide/src-tauri/` scaffold.

Closure:
  - [ ] Tauri app metadata exists.
  - [ ] Native command placeholder exists.
  - [ ] Runtime/debug/Immediate/COM remain unavailable.

### W341-B03 — Scaffold verification command

Goal:
  Add a deterministic verification command or documented check for the app scaffold.

Design:
  - Prefer a repo-local script or cargo task only if it avoids extra toolchain assumptions.
  - Otherwise document exact `rg`/file checks as acceptance evidence.

Tests:
  - Run scaffold verification.
  - Anti-overclaim scan for real execution/native runtime/COM true claims.

Evidence:
  - `target/w341-scaffold-check.txt` or equivalent captured command output.

Closure:
  - [ ] Verification exists.
  - [ ] Verification passes.
  - [ ] Anti-overclaim scan passes.

### W341-B04 — W341 acceptance

Goal:
  Accept the DnaOxIde app scaffold as reviewable groundwork for shared UI and host bridge work.

Design:
  - Update `apps/dna-oxide/README.md` with next workset pointers.
  - Keep W342/W343 dependencies clear.

Tests:
  - Scaffold verification.
  - `git status` review.

Evidence:
  - W341 acceptance output.

Closure:
  - [ ] App scaffold is reviewable.
  - [ ] Next worksets are linked.
  - [ ] No unproven capability is claimed.

## Out-of-scope

- Shared UI implementation; W342 owns it.
- Host bridge traits and DTO facade; W343 owns it.
- Tauri command implementations beyond placeholders; W344 owns them.
- Live WebView/Tauri interaction proof; W345/W346 own it.
- Real OxVba runtime/debug/Immediate/COM execution.
- Sibling repo writes.
