# Workset W090 — Command System And Keymap Profiles

## Ambition

Every command OxIde can execute lives in one unified action
registry. The registry backs keybindings, chords, mnemonic menus,
mouse actions, and palette entries through a single namespace. Two
profiles ship out of the box: OxIde default (Windows-native) and
VBA-IDE-compatible. User overrides are honoured from a local config.

At the end of W090 a VBA author who is muscle-memoried into the VBA
IDE switches the profile once, types the bindings they already know,
and the shell responds exactly as they expect. No ad-hoc F-key match
statements remain in `model.rs`.

## Dependencies

- **W035 §40 `command_model.md`** — decides the shape of chords,
  mnemonics, profile schema. W090 implements the decisions.
- **W040 – W080** — features exist to bind to; W090 regroups them.
- **W039** — Fire Horse Command Lens proof and action-id matrix. It
  supplies visible command rows, disabled reasons, previews, and stable
  ids; W090 owns dispatch and profile behavior.

### W039 Fire Horse Input

W039 proved Command Lens as a product surface with action ids, binding
labels, enabled/disabled state, disabled reasons, preview text, and
overlay-specific Key Rail. W090 should consume those ids and projection
rules when building the real `ActionRegistry`, chords, mnemonic menus,
VBA-compatible profile, and user override file. W039 did not implement
action dispatch or keymap loading.

## Design

### `ActionRegistry` shape

```rust
pub struct Action {
    pub id: ActionId,              // stable, e.g. "editor.save"
    pub label: &'static str,       // "Save"
    pub context: ActionContext,    // Global / Scene(ShellScene) / Focus(FocusRegion)
    pub default_binding: Binding,  // Key or Chord
    pub vba_ide_binding: Binding,  // alternative for VBA-IDE profile
    pub dispatch: fn(&mut ShellModel),
}
```

All existing `Msg` variants that are user-triggerable become `Action`
entries. `Msg::from(Event)` becomes a thin wrapper over the registry's
resolution function.

### Chord state machine

`Ctrl+K Ctrl+O` style: a two-stage match. First key arms the chord,
second key completes it. Armed state surfaces in the status line
("`Ctrl+K` …"). Timeout (~2 seconds) cancels.

### Mnemonic sequences

`Alt+I, M` opens a virtual menu (Inspector → Module) that executes
the action. The menu is a transient overlay built from the registry,
filtered by the current context.

### Profile loading order

1. Built-in default profile (shipped).
2. Built-in VBA-IDE profile (shipped), overlaid if user selects it.
3. `%APPDATA%/OxIde/keymap.json` overrides, if present.

### Profile switch at runtime

New palette entry "Switch Keymap" opens a profile picker; applied
immediately.

## Beads

### W090-B01 — Introduce `ActionRegistry`; migrate existing bindings

**Feature (infrastructure-flavoured).**

- **Goal.** `model.rs` has zero ad-hoc F-key or Ctrl/Alt match arms.
  Every user-triggerable binding routes through the registry. All
  existing tests and `wtd` journeys still pass.
- **Design.** New `src/shell/actions.rs`. Every current key handler
  in `model.rs` becomes an `Action` entry. `Msg::from` calls
  `ActionRegistry::resolve(event, context)` and dispatches the
  returned action.
- **Tests.** Full existing test suite still green. New unit:
  registry resolution for every shipped binding. `wtd` journey:
  `tests/wtd/journey_registry_preserves_bindings.rs` replays every
  binding the Editing status line advertises and confirms the
  expected action.

### W090-B02 — Chord state machine (`Ctrl+K Ctrl+O` style)

**Feature.**

- **Goal.** A user presses `Ctrl+K`, sees the status line show
  "`Ctrl+K` …", presses `Ctrl+O` within the timeout, and the chord-
  bound action dispatches. Pressing Esc or letting the timeout
  elapse cancels.
- **Design.** `ChordState { armed: Option<Key>, armed_at: Instant }`
  on runtime. Registry honours chord bindings.
- **Tests.** Unit: arm / complete / timeout / Esc cancel paths.
  `wtd` journey: a scripted chord (once a real one is bound —
  verify in the tests a test-only action with a `Ctrl+K Ctrl+T` binding).

### W090-B03 — Mnemonic sequences (Alt+letter navigation)

**Feature.**

- **Goal.** `Alt+I` opens a transient Inspector menu; selecting `M`
  (for Module) navigates or opens; `Alt+F` opens a File menu; etc.
  The menu is not a fixed widget — it's built from the registry.
- **Design.** Mnemonic entries on `Action`. Transient overlay
  rendering the current mnemonic level. Timeout / Esc to cancel.
- **Tests.** Unit: mnemonic tree resolution. `wtd` journey:
  `tests/wtd/journey_mnemonic_file_new.rs` presses `Alt+F` then
  `N` and asserts the new-project flow triggers.

### W090-B04 — VBA-IDE profile

**Feature.**

- **Goal.** Loading the VBA-IDE profile makes every VBA-IDE default
  binding (e.g. `F8` step-into, `Shift+F8` step-over, `F2` object
  browser) resolve to the corresponding OxIde action.
- **Design.** `vba_ide_binding` populated on every registered
  `Action`. Profile swap overlays that binding over the default.
  Palette entry "Switch Keymap → VBA-IDE".
- **Tests.** Unit: every `Action` has a VBA-IDE binding or an
  explicit "no analog" marker. `wtd` journey: switch profile,
  press `F8` on a breakpoint row, assert step-in.

### W090-B05 — User override file at `%APPDATA%/OxIde/keymap.json`

**Feature.**

- **Goal.** A user can drop a `keymap.json` and override bindings
  without modifying shipped code. Invalid entries surface a startup
  popover listing the offending lines; valid entries override.
- **Design.** Load on startup; merge into registry after profile
  overlay. JSON schema: list of `{ id, binding }` entries.
- **Tests.** Unit: merge correctness; invalid entry reporting.
  `wtd` journey:
  `tests/wtd/journey_user_override_rebinds_save.rs` writes a
  `keymap.json` rebinding `Save` to `Ctrl+W`, launches, asserts the
  new binding works and the old is free.

## Out-of-scope

- **Command recording / macro playback.** Later workset if ever.
- **Dynamic binding help text.** The palette already shows bindings
  per entry; richer help is W110 polish.
- **Multi-stage chords beyond two keys.** Design pass to revisit if
  ever needed.
