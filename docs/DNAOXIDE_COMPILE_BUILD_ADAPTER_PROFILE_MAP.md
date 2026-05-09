# DNA OxIde Compile/Build Adapter Profile Map

Status: `working_integration_map`
Date: 2026-05-08
Related workset: W355 — OxVba Compile/Build Adapter Profiles

## Purpose

This document maps the first W355 compile/build adapter pass without freezing the OxVba/OxIde seam as a fixed contract.

The integration rule is agile:

1. use current OxVba surfaces as they are;
2. keep OxIde wrappers thin, serializable, and UI-shaped only at host boundaries;
3. adapt on either side as implementation teaches us more;
4. request OxVba changes only for real gaps encountered while building product paths;
5. never substitute CLI/LSP parsing or fake build output for OxVba-backed data.

## Current OxVba Surfaces Observed

### Project loading and compile input

Current useful surfaces:

- `oxvba_project::load_basproj(path: &Path) -> Result<LoadedProject, BasProjError>`
- `oxvba_project::load_basproj_from_str(xml: &str, project_dir: &Path) -> Result<LoadedProject, BasProjError>`
- `oxvba_project::LoadedProject`
  - `manifest: oxvba_compiler::ProjectManifest`
  - `output_type: oxvba_project::OutputType`
  - `build_target: oxvba_project::BuildTarget`
  - `runtime_flavor: oxvba_project::RuntimeFlavor`
  - `entry_point: Option<String>`
  - `default_runtime_profile`, `default_policy_preset`, `default_root_object`
- `oxvba_project::BuildTarget::{Bundle, WrapperExe, WrapperLibrary}`
- `oxvba_project::OutputType::{HostModule, Library, Exe, Addin, ComServer, ComExe}`

### Compile/check

Current useful surfaces:

- `oxvba_compiler::compile(source: &str) -> Result<Bytecode, CompileError>`
- `oxvba_compiler::compile_with_runtime_metadata(source: &str) -> Result<(Bytecode, BTreeMap<String, ProcedureRuntimeMetadata>), CompileError>`
- `oxvba_compiler::compile_project(manifest: &ProjectManifest) -> Result<CompiledProject, ProjectCompileError>`
- `oxvba_compiler::CompiledProject`
  - `bytecode`
  - `procedure_runtime_metadata`
  - `rewritten_source`
  - `host_exports`
  - `reference_visible_exports`
  - `event_dispatch_bindings`
  - `project_com_withevents_routes`
  - `project_dynamic_objects`
- `oxvba_compiler::ProjectCompileError::code()` for stable-ish diagnostic codes.

For W355, the first desktop build/check packet should use `load_basproj` followed by `compile_project`. This is not a contract freeze; it is the current direct API path.

### Runtime/session surfaces adjacent to compile/build

W355 should not claim runtime execution, but current adjacent surfaces are useful for later W365:

- `oxvba_host::Engine::compile_and_prepare_session(&ProjectManifest)`
- `oxvba_host::Engine::compile_and_prepare_session_from_bundle(&OxBundle)`
- `oxvba_host::Engine::execute_project_with_value_snapshot_phased(&ProjectManifest)`
- `oxvba_host::DiagnosticPhase::{CompileTime, Runtime}`
- `oxvba_host::PhaseDiagnostic::{phase(), message()}`

### Native wrapper/build packaging

Current useful surfaces:

- `oxvba_build::compile::compile_shim(source, output_path, ShimOutputType)`
- `oxvba_build::compile::ShimOutputType::{Exe, Dll}`
- wrapper-generation modules under `oxvba_build::{exe,dll,comserver,comserver_exe,xll,...}`

W355 should treat these as optional native packaging/build outputs. A check-only compile result does not need wrapper generation.

### Browser/WASM and web-host surfaces

Current useful surfaces:

- `oxvba_web_host::WebHostCommand`
  - currently includes run/debug/immediate/document commands, but not an explicit compile/check command.
- `oxvba_web_host::WebHostEvent`
  - includes `DiagnosticsUpdated` and `Error` events.
- `oxvba_languageservice` diagnostics surfaces are web-safe candidates for browser diagnostics, but compile/check through `compile_project` still needs wasm-safe validation in W355-B01.

Current browser/WASM gap, proven by W355-B01:

- no observed `WebHostCommand::CompileCheck` or direct web-host compile/build command in the frozen OxVba web host surface.
- `cargo check -p oxvba-compiler --target wasm32-unknown-unknown` fails because the current dependency graph pulls `oxvba-com` native registry/dynamic-library surfaces.
- `cargo check -p oxvba-web-host --target wasm32-unknown-unknown` and `cargo check -p oxvba-project --target wasm32-unknown-unknown` also fail through native COM/JIT-adjacent dependency paths.
- Until OxVba exposes a wasm-safe compile/check seam, the browser profile must return typed unavailable packets rather than fake compile output.

## Host Profiles

### `browser-wasm-dnaonecalc`

Purpose:
  DnaOneCalc browser-hosted IDE path.

Allowed:
  wasm-safe OxVba compile/check APIs if they build for the browser target; typed diagnostics and typed unavailable states.

Not allowed:
  native filesystem/process/COM assumptions, wrapper binary output claims, or fake compile/run data.

Initial command behavior:
  `compile.check` currently returns a typed unavailable packet naming the missing wasm-safe seam: current OxVba compile/project/web-host paths fail `wasm32-unknown-unknown` checks through native COM and native memory/dynamic-library dependency paths.

### `dnaoxide-desktop-tauri-native`

Purpose:
  DnaOxIde standalone desktop app accepted by W352.

Allowed:
  WebView UI -> Tauri command -> linked Rust -> current OxVba project/compiler/build APIs over temp or selected project copies.

Initial command behavior:
  `compile.check` loads the `.basproj`, calls `compile_project`, and returns typed status/diagnostics/inventory through `dna_oxide_build_check`.
  `compile.options` reports current project properties that are already available from `LoadedProject` (`output_type`, `build_target`, `runtime_flavor`, defaults) through `dna_oxide_get_compile_options` plus typed unavailable states for native wrapper/process/COM runtime options not currently claimed.
  native wrapper build commands stay unavailable until a real wrapper-generation path is intentionally wired.

W355-B02 implementation note:
  This profile is now backed by `oxide-oxvba::compile_options_profile` and `oxide-oxvba::compile_build_check`, called from linked Rust Tauri commands in `apps/dna-oxide/src-tauri/src/commands.rs` after W352 established the WebView -> Tauri -> native Rust command spine.

### `dnaonecalc-desktop-native-host`

Purpose:
  DnaOneCalc Windows desktop host embedding OxIde while exposing native OxVba services.

Allowed:
  Same general packet shape as DnaOxIde desktop where practical, with DnaOneCalc host policy selecting project access, save policy, and runtime exposure.

Initial command behavior:
  Reuse the desktop-native adapter shape once DnaOneCalc desktop host mount policy exists; until then return typed unavailable with `host_mount_pending`.

## Command IDs

Use existing host-bridge command IDs where possible:

- `compile.check` — compile/check active project.
- `compile.options` — inspect compile/build-related project options/profile.
- `runtime.run` — remains out of W355 except as disabled/fixture-evidenced metadata.
- `document.save` / `document.reload` — already product-evidenced through W352 for the desktop profile.

Tauri command names for first implementation pass:

- `dna_oxide_build_check`
- `dna_oxide_get_compile_options`

These names already exist as placeholders in the DnaOxIde command catalog. W355-B02 should replace or wrap their placeholder behavior with current OxVba-backed desktop adapter data where available.

## First-Pass Packet Fields

These are UI/host serialization fields, not OxVba-owned authoritative DTOs.

### Compile/check result packet

Suggested fields:

- `command_name`
- `host_bridge_command = "compile.check"`
- `profile_id`
- `project_path`
- `project_name`
- `status`: `succeeded | failed | unavailable`
- `diagnostics`: list of `{ phase, code, message, source }`
- `compiled_summary` when succeeded:
  - `procedure_count`
  - `host_export_count`
  - `reference_visible_export_count`
  - `event_binding_count`
  - `dynamic_object_count`
  - `rewritten_source_length`
- `disabled_reason` when unavailable
- `request_id` generated by OxIde until OxVba exposes one directly
- `provider_label = "oxvba-current-api"`
- no-claim flags for runtime/debug/COM runtime.

### Compile options/profile packet

Suggested fields:

- `command_name`
- `host_bridge_command = "compile.options"`
- `profile_id`
- `project_path`
- `project_name`
- `output_type`
- `build_target`
- `runtime_flavor`
- `entry_point`
- `default_runtime_profile`
- `default_policy_preset`
- `default_root_object`
- `unavailable_options`: list of missing or not-yet-surfaced options
- `provider_label = "oxvba-current-api"`
- no-claim flags.

## Implementation Notes For Next Beads

1. B01 should probe browser/WASM honestly. If `oxvba_compiler` or project loading is not currently wasm-safe, return typed unavailable and name the gap.
2. B02 should prioritize desktop-native check/options over temp project copies via W352 Tauri commands.
3. B03 should adopt adapter-backed data in UI panels, preserving unavailable rows rather than inventing fake compiler output.
4. B04 should accept at least one real product host profile and hand off remaining profile gaps.

## Explicit Non-Goals For This Map

- no locked DTO contract between repos;
- no CLI output parsing;
- no LSP-shaped compile substitute;
- no fake build status;
- no runtime/debug/COM runtime claim in W355;
- no browser native filesystem/process/COM assumption.
