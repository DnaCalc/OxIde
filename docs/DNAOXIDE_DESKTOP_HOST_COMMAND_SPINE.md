# DNA OxIde Desktop Host Command Spine

Status: `w352_b00_decision`
Date: 2026-05-08
Workset: W352 — DnaOxIde Tauri/WebView Product Host And Automation

## Decision

Use **Tauri v2** as the DnaOxIde desktop product host path.

Default native backend means **linked Rust code in the Tauri app crate**, not a separate service process:

```text
DnaOxIde Tauri app
  ├─ WebView UI
  └─ src-tauri Rust crate
      ├─ #[tauri::command] wrappers
      ├─ apps/dna-oxide/src-tauri/src/commands.rs
      ├─ OxIde crates
      └─ OxVba crates/adapters where linked
```

A separate process remains out-of-scope unless a later COM/runtime isolation workset explicitly chooses it.

## Local Tool Evidence

W352-B00 local checks show:

```text
node: v25.2.0
npm: 11.12.1
cargo: 1.94.1
rustc: 1.94.1
installed Rust targets include wasm32-unknown-unknown and x86_64-pc-windows-msvc
Tauri CLI: unavailable
Tauri npm packages: unavailable in apps/dna-oxide
Trunk: unavailable
Tauri WebDriver wrapper tauri-driver: unavailable
Microsoft Edge WebView2 Runtime: present, 147.0.3912.98
Microsoft Edge WebDriver: present at C:\Programs\EdgeDriver\msedgedriver.exe
```

Full transcript:

```text
target/w352-b00-tool-transcript.txt
```

## Official Tooling Basis

Official Tauri v2 references used for this decision:

- CLI install/dev path: <https://v2.tauri.app/reference/cli/>
- Frontend-to-Rust command invocation: <https://v2.tauri.app/develop/calling-rust/>
- WebDriver / `tauri-driver`: <https://v2.tauri.app/develop/tests/webdriver/>

Relevant implications:

- Add the Tauri CLI to this project with npm dev dependency in B01.
- Add the Tauri JavaScript API package where host-specific frontend code needs `invoke`.
- Register Rust commands with `tauri::Builder::invoke_handler(...)`.
- Use `tauri-driver` plus platform WebDriver for WebView automation after desktop shell startup is proven.

## B01 Dependency Path

B01 may add dependencies explicitly with reviewable package/Cargo changes:

```powershell
npm --prefix apps/dna-oxide install --save-dev @tauri-apps/cli@^2
npm --prefix apps/dna-oxide install @tauri-apps/api@^2
```

Rust-side B01 dependency direction:

```toml
[build-dependencies]
tauri-build = { version = "2", features = [] }

[dependencies]
tauri = { version = "2", features = [] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

B01 should add `apps/dna-oxide/src-tauri/build.rs` with `tauri_build::build()` and convert `src-tauri/src/main.rs` from scaffold print mode to `tauri::Builder` mode.

## Frontend Serving Path

Current `tauri.conf.json` has:

```json
"devUrl": "http://127.0.0.1:1420",
"frontendDist": "../dist"
```

Because Trunk is unavailable and the current frontend is JS/HTML, B01 should add a minimal app-local static dev server script rather than introducing Leptos/Trunk yet.

Recommended B01 scripts:

```json
"desktop:serve": "node ./scripts/serve-desktop-dev.mjs",
"tauri:dev": "tauri dev",
"tauri:build": "tauri build"
```

Then update `tauri.conf.json`:

```json
"beforeDevCommand": "npm run desktop:serve",
"devUrl": "http://127.0.0.1:1420",
"beforeBuildCommand": "npm run desktop:dist",
"frontendDist": "../dist"
```

`desktop:dist` can copy the current `index.html` and `src/` assets into `apps/dna-oxide/dist/` until the shared Rust/WASM UI build replaces it.

## First Native Command Spine

B01 should prove this real product seam:

```text
WebView UI button/test call
  -> @tauri-apps/api/core invoke(...)
  -> #[tauri::command] in src-tauri
  -> linked Rust function in apps/dna-oxide/src-tauri/src/commands.rs
  -> typed serializable result
  -> rendered/logged in WebView instrumentation
```

First command:

```text
dna_oxide_desktop_host_capabilities_probe
```

Suggested Rust wrapper behavior:

1. Accept `project_path: String` for a test-owned temp project copy.
2. Call existing linked Rust code: `commands::dna_oxide_get_host_capabilities(project_path)`.
3. Return a small `serde::Serialize` DTO:

```rust
DesktopHostCommandSpinePacket {
    command_name: "dna_oxide_desktop_host_capabilities_probe",
    provider: "tauri-linked-rust",
    command_count,
    enabled_count,
    linked_oxide_crates: true,
    linked_oxvba_adapter_crate: true,
    native_runtime_claimed: false,
    com_runtime_claimed: false,
    fake_responses: false,
    fake_debug_data: false,
}
```

This is deliberately not a runtime/build/COM claim. It proves the UI can call linked native Rust and that linked Rust can reach existing OxIde/OxVba adapter crate code.

## B02 Automation Path

After B01 starts the shell, B02 should install/use WebView automation if still locally feasible:

```powershell
cargo install tauri-driver --locked
```

Windows automation requires Microsoft Edge WebDriver matching the installed Edge/WebView2 version. Local checks found `msedgedriver.exe`, but B02 must verify version compatibility before claiming repeatable WebView automation.

If WebView automation blocks, B02 may document the limitation, but B03/B04 desktop product acceptance still requires real Tauri/native command evidence and must not fall back to browser-only injected services as a substitute.

## No-Claim Boundaries

W352-B00 does not claim:

- Tauri app starts;
- WebView automation works;
- native OxVba compile/build/runtime execution;
- Immediate evaluation;
- debug/watch/breakpoint behavior;
- COM runtime invocation;
- DnaOneCalc mount;
- production packaging.

Those are gated by W352-B01+ and later adapter work.
