# Workset W100 — Terminal Capability And Onboarding

## Ambition

OxIde is honest about the terminal it's running in. At first launch,
the user sees a short capability probe page that tells them what
works and what doesn't (truecolor? mouse? Unicode? size?). When
capabilities are insufficient for the instrument theme, the shell
degrades visibly and says why. The status line always surfaces the
next available keystroke in the current scene, with no misleading
fiction.

At the end of W100 the shell runs cleanly on Windows Terminal,
ConEmu, classic `cmd.exe`, `ssh` into WSL, and plain 16-colour
terminals — each one with a faithful appearance for what the
terminal actually supports, not a forced instrument look that
corrupts on fallback.

## Dependencies

- **W035 §50 `visual_language.md`** — defines the palette, the
  16-colour fallback, the degradation policy. W100 implements those.
- **`ftui_core::capabilities`** — the probing infra exists in
  FrankenTui; W100 calls into it.
- **W039** — Fire Horse Console Fit and Compact Focus proof. It
  supplies terminal-cell presentation targets; W100 owns live probing
  and fallback policy.

### W039 Fire Horse Input

W039 proved Console Fit and Compact Focus as terminal-cell scenarios
using fixture capability rows and viewport classes. W100 should replace
those fixture rows with live `ftui_core::capabilities` results, apply
truecolor/16-colour/ASCII/mouse fallback policy, and keep status-line
honesty under each capability profile. W039 did not probe the user's
terminal or prove fallback rendering under real degraded hosts.

## Design

### First-run capability probe page

On the very first launch (detected by absence of
`%APPDATA%/OxIde/session.json`), a dedicated scene renders:

```
┌────── OxIde capability probe ──────┐
│  Terminal:        Windows Terminal │
│  Truecolor:       ✓                │
│  Mouse:           ✓                │
│  Unicode wide:    ✓                │
│  Size:            140x42           │
│  VT features:     SGR ✓ CUP ✓      │
│                                    │
│  Ready.                            │
│  Press Enter to continue.          │
└────────────────────────────────────┘
```

Each missing capability surfaces the *specific* remediation ("Enable
'Use legacy console' → off in Windows Terminal settings", etc.).

### Degradation path

When capabilities are insufficient, apply the visual-language
fallback:

- No truecolor → 16-colour palette from theme.rs fallback table.
- No Unicode wide → ASCII borders (`-`, `|`, `+`), no `│` / `└` etc.
- No mouse → status line drops mouse hints; palette shows
  "mouse unavailable".
- Size < W035 D11 Narrow threshold → refuse to render (show a
  capability-message scene telling the user to resize).

### Status-line honesty

Every scene's status line names exactly the bindings that work in
that scene under the current capabilities. A binding that requires
mouse disappears from the status line when mouse is off; a binding
that requires truecolor (e.g. hover popover rendering may need it)
either works or the status line explains the fallback.

### Light / dark palette toggle

`Ctrl+Shift+P` (or a palette entry) toggles between dark (default)
and light. Auto-detection via `COLORFGBG` when available. Persisted
to `session.json`.

## Beads

### W100-B01 — Capability probe scene

**Feature.**

- **Goal.** First-run launch (no `session.json`) lands on a
  capability probe scene. Probed items: truecolor, mouse, Unicode
  wide, size, VT SGR / CUP features. Each item shows ✓ or ✗ with
  guidance on ✗. `Enter` advances to the normal Empty scene.
- **Design.** New `ShellScene::CapabilityProbe` (or a dedicated
  first-run sub-scene). Probing via `ftui_core::capabilities`.
  Session-aware: skipped on subsequent launches unless
  `--probe-caps` or a palette entry invokes it.
- **Tests.** Unit: probe result → rendering cells. `wtd` journey:
  `tests/wtd/journey_first_run_probe.rs` launches with no
  `session.json`, captures the probe page, asserts each expected
  row.

### W100-B02 — Degradation path for truecolor / mouse / Unicode

**Feature.**

- **Goal.** Running in a 16-colour terminal renders OxIde with the
  fallback palette; running without mouse removes mouse hints;
  running without Unicode wide renders borders with ASCII.
- **Design.** `theme.rs` gains a `Palette` enum
  (`Truecolor` / `Ansi16`) and border-style variants. The view
  layer chooses based on probed capabilities. FrankenTui's
  degradation budget hooks route the choice.
- **Tests.** Unit: under each capability profile, the rendered cells
  match the expected fallback. `wtd` journey (if wtd can simulate):
  otherwise a unit + manual note.

### W100-B03 — Status-line honesty per capability

**Feature.**

- **Goal.** Every scene's status line reads honestly under the
  current capability profile — mouse-only bindings disappear when
  mouse is off; truecolor-only effects are replaced or dropped.
- **Design.** `status_line_hint()` takes the current
  `ShellCapabilities` into account and filters tokens.
- **Tests.** Unit: per-scene hint under `{truecolor,16colour} x
  {mouse,no-mouse}` profiles.

### W100-B04 — Light / dark toggle

**Feature.**

- **Goal.** `Ctrl+Shift+L` toggles between light and dark palettes;
  the chosen palette persists to `session.json` and reloads on
  next launch; `COLORFGBG` seeded auto-detection where available.
- **Design.** Palette enum extended with `Light` / `Dark` variants.
  `theme.rs` ships two palette tables. Msg + palette entry.
- **Tests.** Unit: toggle flips palette; persistence round-trip.
  `wtd` journey: toggle and capture both states.

### W100-B05 — Sub-minimum-size refusal

**Feature.**

- **Goal.** When the terminal is below the W035 D11 minimum (e.g.
  a 40x20 cmd), OxIde renders a capability-message scene telling
  the user to resize, rather than silently corrupting the frame.
- **Design.** Early check in `render`. If `width < 80 || height < 20`
  (or the final chosen minima), render a single centred block.
- **Tests.** Unit: under known dimensions, rendering chooses the
  refusal scene. `wtd` journey: capture at the known minimum.

## Out-of-scope

- **True image / sixel support.** Not a W100 concern.
- **High-DPI awareness.** Terminal is cell-based.
- **Accessibility beyond the capability probe.** Screen-reader
  support is future, not W100.
