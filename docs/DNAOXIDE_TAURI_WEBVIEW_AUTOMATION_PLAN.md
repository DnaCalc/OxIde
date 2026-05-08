# DNA OxIde Tauri/WebView Automation Plan

Status: `planned_after_w350_browser_dom`
Date: 2026-05-08
Related workset: W352 — DnaOxIde Tauri/WebView Automation

## Purpose

Option C is the desktop-product proof lane: run **DNA OxIde / DnaOxIde** in a real Tauri/WebView host and automate the WebView enough to see and drive the same app behavior proven by W350 browser DOM + Playwright.

This plan exists now so W350 can use browser DOM + Playwright without losing the desktop host target.

## Current Local Tool State

Observed during W350-B00:

```text
cargo/rustc: available
node/npm: available
Playwright CLI: available
Tauri CLI: unavailable
Trunk: unavailable
```

Therefore Tauri/WebView automation is not the first W350 proof mode. It needs a toolchain/bootstrap bead before it can be used as a regression lane.

## Target Capability

The Tauri/WebView lane should eventually prove:

1. DnaOxIde starts as a desktop app host;
2. the WebView mounts the same app shell and test driver as the browser DOM proof;
3. automation can capture a visual/WebView artifact;
4. automation can capture DOM-like state from inside the WebView;
5. automation can inject focus/type/save/reload interactions;
6. command/event logs are captured;
7. artifacts can be compared against the W350 browser DOM proof;
8. no runtime/debug/Immediate/COM claims flip without adapter evidence.

## Proposed W352 Beads

### W352-B00 — Tauri/WebView toolchain bootstrap plan

- Decide exact Tauri CLI and dependency installation path.
- Keep installation explicit and reviewable.
- Do not hide network/toolchain requirements.

### W352-B01 — Tauri dev shell starts with no runtime claims

- Add or enable the minimal Tauri dependency path.
- Launch DnaOxIde desktop shell with the W350-instrumented app.
- Keep native runtime, COM runtime, and fake data claims false.

### W352-B02 — WebView automation bridge

- Choose driver strategy for WebView inspection/injection.
- Prefer a supported WebView automation route if available.
- If direct WebView automation is not available, document the bounded alternate and what it cannot claim.

### W352-B03 — Tauri edit/save/reload parity smoke

- Drive the same edit/save/reload loop as W350 through the desktop host.
- Capture before/after visual, DOM-like, command, and event artifacts.
- Compare key state fields with browser DOM proof output.

### W352-B04 — Tauri/WebView acceptance

- Accept only when desktop host automation is repeatable and artifacts are sufficient for debugging.
- Preserve all no-claim boundaries.

## Relationship To W350

W350 is the primary next step because browser DOM + Playwright is available now and gives the fastest feedback loop.

W352 should reuse W350 instrumentation rather than inventing a parallel test substrate. The same driver concepts should exist in both lanes:

```text
snapshot()
visualSnapshot()
eventLog()
commandLog()
injectInteraction(action)
resetForTest()
```

## Relationship To W355-W370

W355-W370 adapter work may proceed after W350 if Tauri/WebView tooling is still unavailable. However, adapter acceptance must remain honest about which host was driven:

- browser DOM proof: claim browser DOM app behavior only;
- Tauri/WebView proof: claim desktop host behavior only after W352 evidence exists.

## No-Claim Boundaries

W352 does not by itself prove:

- real/native OxVba runtime execution,
- Immediate evaluation,
- debug/watch/breakpoint behavior,
- COM runtime invocation,
- DnaOneCalc mount,
- full DOM accessibility audit,
- production installer/package quality.

Those claims remain gated by separate adapter and packaging evidence.
