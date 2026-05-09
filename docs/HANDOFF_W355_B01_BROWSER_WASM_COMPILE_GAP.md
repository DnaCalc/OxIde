# Handoff — W355-B01 Browser/WASM Compile/Check Gap

Date: 2026-05-09
Profile: `browser-wasm-dnaonecalc`
Command: `compile.check`

## Result

The DnaOneCalc browser/WASM compile/check path is explicitly blocked in the current frozen OxVba snapshot. OxIde now returns typed unavailable packets for the browser profile instead of fake compile/build data.

## Evidence

Commands run from OxIde repo root:

```text
cargo check -p oxvba-compiler --target wasm32-unknown-unknown
cargo check -p oxvba-web-host --target wasm32-unknown-unknown
cargo check -p oxvba-project --target wasm32-unknown-unknown
node apps/dna-oxide/scripts/verify-command-client.mjs
```

Observed blocking errors:

- `oxvba-compiler` pulls `oxvba-com`; wasm check fails because `resolve_typelib_identity_from_prog_id` is gated to Windows and because `libc::dlopen`, `libc::dlsym`, `libc::dlerror`, `RTLD_NOW`, and `RTLD_LOCAL` are unavailable for `wasm32-unknown-unknown`.
- `oxvba-web-host` also pulls native/JIT or COM-adjacent dependencies; wasm check fails through `oxvba-com` and `region`.
- `oxvba-project` also fails for wasm through `oxvba-com` and `region` dependency paths.
- No explicit `WebHostCommand::CompileCheck` seam was observed in the current `oxvba-web-host` command enum.

## OxIde behavior landed for B01

`apps/dna-oxide/src/command-client.js` now exposes browser-profile unavailable packets:

- `browserWasmCompileCheckUnavailableResponse(payload)`
- `browserWasmCompileOptionsUnavailableResponse(payload)`
- `BROWSER_WASM_COMPILE_PROFILE`

The browser command client returns those packets for:

- `dna_oxide_build_check` / `compile.check`
- `dna_oxide_get_compile_options` / `compile.options`

The packets state:

- `profileId: "browser-wasm-dnaonecalc"`
- `status: "unavailable"`
- `adapterBacked: false`
- no native filesystem requirement;
- no native process requirement;
- no COM runtime requirement;
- no fake responses;
- native-only outputs are unavailable.

## Requested OxVba seam

To enable this browser profile, OxVba needs a wasm-safe compile/check seam that does not pull native COM registry, native process, dynamic library loading, or JIT memory-protection dependencies. A workable shape would be one of:

1. a feature-gated `oxvba-compiler` browser profile where COM/type-library lookup is represented as disabled/typed unavailable or pure data; or
2. a `oxvba-web-host` command such as `CompileCheck` with a wasm-safe implementation and typed diagnostics; or
3. a smaller compile/check crate that accepts source/project-in-memory data and produces diagnostics/compiled summary without native dependencies.

Until that exists, OxIde should keep browser compile/check disabled with typed unavailable packets and continue W355 desktop-native work through Tauri/native Rust.
