# DNA OxIde Tauri/WebView Product Host And Automation Plan

Status: `active_next_desktop_product_lane`
Date: 2026-05-08
Related workset: W352 — DnaOxIde Tauri/WebView Product Host And Automation

## Purpose

W352 is now the desktop product-host lane, not an optional exhibition proof.

The goal is to run **DNA OxIde / DnaOxIde** in a real Tauri/WebView host, prove that the UI can call linked native Rust commands in the Tauri app, and keep enough automation/instrumentation to regress edit/save/reload and later OxVba adapters.

## Current Local Tool State

Observed during W352-B00:

```text
node: v25.2.0
npm: 11.12.1
cargo: 1.94.1
rustc: 1.94.1
installed Rust targets: wasm32-unknown-unknown, wasm32-wasip1, wasm32-wasip2, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu
Tauri CLI: unavailable
Tauri npm packages in apps/dna-oxide: unavailable
Trunk: unavailable
tauri-driver: unavailable
Microsoft Edge WebView2 Runtime: present
Microsoft Edge WebDriver: present at C:\\Programs\\EdgeDriver\\msedgedriver.exe
```

Full transcript: `target/w352-b00-tool-transcript.txt`.

W352-B00 decision details are recorded in [`DNAOXIDE_DESKTOP_HOST_COMMAND_SPINE.md`](DNAOXIDE_DESKTOP_HOST_COMMAND_SPINE.md).

## Target Product Capability

The Tauri/WebView lane should prove:

1. DnaOxIde starts as a desktop app host;
2. the WebView mounts the shared DnaOxIde IDE surface;
3. at least one UI command reaches linked native Rust code in the Tauri app;
4. save/reload over temp project copies is handled by native Rust commands, not Playwright-injected browser host services;
5. automation can capture a visual/WebView artifact;
6. automation can capture DOM-like state from inside the WebView;
7. automation can inject at least one interaction or document the precise limitation;
8. command/event logs are captured;
9. performance and footprint numbers are captured from the start so the desktop app stays svelte, strong, and zippy rather than sluggish or bloated;
10. artifacts distinguish desktop-host evidence from W350 browser harness evidence;
11. runtime/debug/Immediate/COM claims remain false until later adapter work proves them.

## Default Native Backend Meaning

In W352, native backend means linked Rust code in the Tauri app by default:

```text
DnaOxIde Tauri app
  ├─ WebView UI
  └─ Rust app crate
      ├─ Tauri command functions
      ├─ OxIde adapter crates
      └─ OxVba crates/services where linked
```

A separate native service process is not the default. It is only chosen by a later explicit workset if COM apartment policy, runtime isolation, crash containment, or multi-host sharing requires it.

## Proposed W352 Beads

### W352-B00 — Desktop host toolchain and native command spine plan

Decision:

- Use Tauri v2.
- Add `@tauri-apps/cli` as an app-local npm dev dependency in B01.
- Add `@tauri-apps/api` as an app-local npm dependency in B01 for host-specific frontend invoke code.
- Add `tauri`, `tauri-build`, `serde`, and `serde_json` to `apps/dna-oxide/src-tauri` in B01.
- Use the existing JS/HTML frontend with an app-local static dev server first; do not introduce Trunk/Leptos conversion in W352.
- First native command spine: WebView UI -> `@tauri-apps/api/core.invoke` -> `#[tauri::command]` -> `commands::dna_oxide_get_host_capabilities(...)` -> serializable `DesktopHostCommandSpinePacket`.

### W352-B01 — Tauri dev shell starts with native command spine

- Add or enable the minimal Tauri dependency path selected in B00.
- Add a minimal static dev server and dist-copy script for the current JS/HTML frontend.
- Launch DnaOxIde desktop shell.
- Invoke `dna_oxide_desktop_host_capabilities_probe` from the UI/WebView.
- Keep native runtime, COM runtime, and fake data claims false.

### W352-B02 — WebView automation bridge

- Choose driver strategy for WebView inspection/injection.
- Prefer a supported WebView automation route if available.
- If direct WebView automation is not available, document the bounded alternate and what it cannot claim.
- Do not replace native command evidence with browser-only injected services.
- Record the initial desktop performance/size baseline: release executable size, dist/bundle size, cold-start-to-first-observable-UI timing, native host-probe round-trip latency, and idle process memory.

### W352-B03 — Tauri edit/save/reload through native Rust commands

- Drive edit/save/reload through the desktop host.
- Save/reload must execute through Tauri native commands over temp project copies.
- Capture before/after visual, DOM-like, command, and event artifacts.

### W352-B04 — Tauri/WebView product-host acceptance

- Accept only when desktop host execution is repeatable and artifacts are sufficient for debugging.
- Confirm UI -> linked native Rust command evidence.
- Preserve all no-claim boundaries.
- Carry the W352-B02 performance/size baseline into downstream desktop adapter work and file follow-up beads for any sluggishness, bloat, or startup/interaction regressions.

## Relationship To W350

W350 is complete and remains useful as an instrumentation and regression harness. Its static HTML artifacts are review evidence only; they are not the product app.

W352 may reuse W350 instrumentation concepts:

```text
snapshot()
visualSnapshot()
eventLog()
commandLog()
injectInteraction(action)
resetForTest()
```

but W352 acceptance must come from the real Tauri/WebView host and native Rust command path.

## Relationship To W355-W370

W355-W370 adapter acceptance must exercise real product seams:

- browser website/DnaOneCalc WASM profile for wasm-safe OxVba compiler/runtime work;
- Tauri/WebView -> linked native Rust command -> OxVba adapter for desktop native work;
- host-profile-specific typed unavailable states where a capability is absent.

Browser-only Playwright harnesses may remain regression aids, but they must not substitute for the required product-host evidence.

## No-Claim Boundaries

W352 does not by itself prove:

- real/native OxVba compile/build/runtime execution,
- Immediate evaluation,
- debug/watch/breakpoint behavior,
- COM runtime invocation,
- DnaOneCalc mount,
- full DOM accessibility audit,
- production installer/package quality.

Those claims remain gated by separate adapter and packaging evidence.
