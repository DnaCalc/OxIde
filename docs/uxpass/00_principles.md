# 00 - Principles

Status: `draft`
Workset: `W035`

## Purpose
Tie-breakers for every later uxpass decision. Grounded in direct
observation of the running W030 shell (see `tests/wtd/goldens/W037/`),
not in abstract ambition.

## Observations That Shaped These Principles
These are the concrete conditions against which W035 is being written.
They are not grievances; they are the starting point.

1. The current Empty scene duplicates its command list in two panes (Launcher and Welcome).
2. The current Environment pane shows hex color tokens to the user - backstage information on a user surface.
3. The current Hint line mixes user bindings (`Ctrl+O`) with dev scene-flips (`F2 Empty`, `F3 Edit`, `F4 Sem`).
4. The current top bar has seven or more pipe-separated fields without visible hierarchy.
5. Valuable information truncates: Recent project names, module include paths, diagnostic messages.
6. The current Inspector column is narrower than the content it is asked to hold.
7. A "Session" sub-pane reports internal counters ("Dirty buffers: 0 / Hidden buffers: 0") rather than user-meaningful state.
8. The current shell shows a three-column body even on Empty, where there is nothing to inspect.

## Principles

### P1 - Every surface is user-facing
No developer telemetry on a user surface. If it is not actionable,
explanatory, or diagnostic to the user, it belongs in a debug mode or a
log - not in the shell. Applies specifically to palette hex codes,
internal buffer counters, "focus region" labels, and width-class names.

Honest-defaults landings _2026-04-17_ — external jargon also counts as
dev telemetry even when it comes from a dependency: `load_semantic_state`
in `src/shell/oxvba.rs` now screens `Integer / Long / Variant / …` and
any `__OxVba…` provenance out of the Symbols pane (J2-c), diagnostics
from any `__OxVba…` document render as `[generated] <sev> <msg>`
(J3-b), and `sanitize_output_text` strips the `" with <N> user slots"`
suffix from every `WebHostEvent::OutputLine` before it reaches the
Output pane or build log (J3-c). The palette overlay is also painted
opaquely over its backing surface — `render_overlay` in `src/shell/view.rs`
clears the overlay rect with `Cell::default()` before the Block renders,
so editor glyphs cannot bleed through (J4-a / J4-d).

### P2 - One source of truth per affordance
When two panes list the same commands, one of them is wrong. Either the
pane teaches (Welcome) or it acts (Launcher), not both. If a decision is
unclear, the pane that drives the keyboard wins.

### P3 - Density means information, not noise
Pipe-separated top-bar fields are not density; they are a list. Real
density is: every glyph on screen carries a user-visible meaning. Fields
that repeat, echo, or restate state go.

### P4 - Never truncate what the user needs to act
Paths, diagnostic text, identifiers, recent-project names - if a region
cannot hold them, either the region grows, the content wraps, the user
gets an affordance to expand, or the content moves to a region that can
hold it. Silent mid-word truncation is a bug.

Dirty-state honesty landing _2026-04-17_ — the editor title gains a
trailing ` *` when the active buffer's `dirty` flag is set, so a typed
edit is visibly uncommitted until save lands (J2-d). Welcome (Empty) is
scene-fixed and never carries the marker. Pinned by
`shell::mock_data::tests::editor_title_gains_dirty_marker_after_edit_and_clears_without_one_otherwise`.

Save-path landing _2026-04-17_ (W050 pre-landing) — the dirty marker is
now an honest handshake, not just a warning. `Ctrl+S` writes
`buffer.lines` back to the buffer's `source_path` (new field on
`BufferState`, populated from the `DocumentSession` seam in
`src/shell/session.rs`) and clears `dirty`; `Ctrl+Shift+S` saves every
dirty buffer. The file's line-ending convention (CRLF / LF) is detected
on load via `LineEnding::detect`, stored on `BufferState`, and
preserved across edits — a round-trip through OxIde does not
gratuitously normalise a checkout's line endings. The trailing-newline
flag is also preserved. A `Ctrl+Z` undo stack (per-buffer
`BufferHistory`, default 64 snapshots, ring-buffer bounded) + `Ctrl+Y`
redo complete the "I can type, I can recover, I can persist" triad.
Discovery is on the always-present bottom status line (D3): the
Editing hint carries `Ctrl+S save` and `Ctrl+Z undo`. Each new
palette binding resolves to a wired `Msg` (J4-e / P6). Pinned by
fifteen new regression tests in `shell::state::tests` and
`shell::model::tests` — see W035 Progress paragraph for the full list.
Overlay scenes (Palette, COM reference) are now non-destructive: the
previous `apply_scene` rebuilt the workspace from the clean session
snapshot on every transition and silently wiped in-flight dirty edits,
a separate P4 violation found and fixed in the same push
(`shell::state::tests::opening_palette_overlay_preserves_in_flight_buffer_edits`).

### P5 - Match shape to task
Empty has nothing to inspect; do not render an Inspector. Editing has no
build output; do not reserve a lower surface. The frame adapts to the
scene, not the other way around. A region that is present must earn its
footprint.

### P6 - Every binding is documented in place
If a keybinding exists, the scene that uses it surfaces it in a visible,
persistent status hint. Users should not need to read `PRODUCT_DIRECTION.md`
to discover `Ctrl+O`. This applies equally to chords and mnemonics when
those exist.

The converse also holds, and landed _2026-04-17_: a binding that the
palette advertises must actually resolve to a wired `Msg`. The
unwired `New Project` / `Ctrl+N` entry has been removed from the
palette command list in `src/shell/state.rs` and will return only
when `Ctrl+N` is implemented (tracked for W040). The remaining
palette bindings (`Ctrl+O`, `Alt+1..4`, `Ctrl+Shift+{M,C,R,T}`,
`Ctrl+Tab`, `F6`) were audited and all resolve today. Pinned by
`shell::state::tests::palette_commands_do_not_advertise_unwired_ctrl_n_binding`.

### P7 - Dev affordances are not user affordances
F2 / F3 / F4 scene-flip demos, theme-token dumps, internal focus labels -
these are developer conveniences. They move behind a dev flag or a debug
scene accessed explicitly, not into the default shell.

### P8 - Honest about the terminal
If the terminal lacks truecolor, say so and degrade gracefully. If the
terminal is narrower than the minimum, say so and refuse to render
something misleading. Capability failures are shell messages, not silent
visual corruption.

### P9 - Keyboard-first, mouse-coherent
The primary input is the keyboard. Mouse support is additive: every
mouse action has a keyboard equivalent, and every keyboard action has a
visible hint. The shell does not depend on the mouse to be usable, and
does not punish it when used.

### P10 - Scene names are for users
`Editing`, `Debug`, `Build` - fine. `Scene::Semantic` is an internal
enum, not a thing the user has opinions about. Internal taxonomy names
do not surface in the UI unless the user can do something with them.

## Decisions
Numbered and imperative. Later uxpass docs cite these by number.

1. The Empty scene does not render an Inspector column. Width goes to a single Welcome surface that is also the launcher. _Fully landed 2026-04-17 (D1a + D1b):_ D1a removed the Inspector column (`Environment` pane) from Empty. D1b then collapsed the remaining two-column body into a single full-width Welcome panel that owns both the launcher role (Recent + Start actions) and the teaching role — `launcher_text` deleted; `launcher_editor_text` in `src/shell/mock_data.rs` now renders `OxIde / A terminal-native IDE for OxVba. / Recent / ... / Start / ...` directly into Welcome; `explorer_text` returns empty on Empty; `render_empty_body` in `src/shell/view.rs` collapsed to a single Welcome panel. `available_focus_regions` on Empty is `[TopBar, Editor]` only, so `Tab` cycles just those two and `Alt+1`/`Alt+3` are no-ops on Empty. Contract pinned by: `shell::state::tests::{empty_scene_focus_ring_is_top_bar_and_editor_only, empty_scene_alt1_focus_request_is_rejected, empty_scene_alt3_focus_request_is_rejected, editing_scene_still_exposes_inspector_in_focus_ring}` and `shell::mock_data::tests::{welcome_owns_the_launcher_role_on_empty, empty_scene_emits_no_explorer_panel_text}`. W037 `empty.txt` golden and `cold_start/00_welcome.txt` + `01_after_tab.txt` + `01_after_ctrl_o.txt` captures re-blessed; Empty now paints `[top bar][single Welcome panel][status line]` — no Explorer, no Inspector, no Launcher pane.
2. The top bar carries, at most, three fields: project identity, scene, and the single most relevant state value for the current scene. Everything else moves to status line or inspector. _Landed 2026-04-17:_ `top_bar_text` rewritten; banned strings (`Focus Editor/Lower/Top`, width-class names, `Views N`, `Hidden N`, `Hover active`, `Overlay focus`, `Mockup-derived instrument palette`, `Truecolor ready`) removed from every scene; five regression tests in `shell::mock_data::tests` pin the contract; W037 goldens and cold_start / thin_slice evidence captures re-blessed.
3. The status line is a dedicated row (bottom) whose contract is "what keystrokes are available right now." It is always present. _Landed 2026-04-17:_ `src/shell/view.rs` now builds the frame as `split_root() = [TopBar(3), Body(Fill), LowerSurface?, StatusLine(1)]` so a 1-row status line is reserved on _every_ scene and every frame, even when there is no lower surface. The row is painted by `render_status_line()` in muted style with no border. Text comes from `ShellState::status_line_hint()` (delegated through `ShellModel::status_line_hint()`), which returns a single static per-scene string — e.g. Empty = `Ctrl+O open project  Up/Down select recent  F6 palette  Ctrl+Q quit`, Editing/Semantic = `F5 run  F6 palette  Ctrl+Tab next view  Tab next focus  Ctrl+Q quit`, BuildRun = `F5 rerun  F6 palette  Tab next focus  Ctrl+Q quit`, and overlays (Palette, ComReference) receive a dedicated overlay hint so the status line always names the keystrokes that are actually live. Contract pinned by `shell::state::tests::{empty_status_line_announces_ctrl_o, editing_status_line_announces_f5_and_palette, build_run_status_line_announces_rerun, palette_overlay_status_line_is_overlay_hint, com_reference_overlay_status_line_is_overlay_hint, status_line_hint_never_exceeds_one_line}`. Visible on every re-blessed W037 golden (`empty.txt`, `thin_slice_loaded.txt`) on row 40.
4. Palette hex codes, token names, and "width class" labels never appear in the shipping shell. _Landed 2026-04-17 (top bar partial) + 2026-04-17 (Inspector sweep) + 2026-04-17 (dead-code reap):_ width-class name and theme name removed from the top bar alongside D2; the Empty Inspector's `Theme` sub-pane (`Mockup-derived instrument palette` / `High-contrast panel hierarchy is active`) and the unconditional `Tokens` hex-code dump (`bg #0A0E14 panel #0D1117 …`) are both deleted. `theme::token_summary()` and the `pub const *_HEX: &str` palette-name constants removed from `src/shell/theme.rs`; `theme` import removed from `src/shell/mock_data.rs`. In the same 2026-04-17 pass `WidthClass::label` was deleted outright so the shell cannot re-introduce a user-visible "Standard / Narrow / Wide" badge by accident. Regression pinned by `shell::state::tests::empty_inspector_shows_only_capabilities`.
5. Internal scene/region identifiers never appear as user-visible labels. `FocusRegion::Editor` is not a label; "Editor" with visible focus ring is. _Landed 2026-04-17 (top bar) + 2026-04-17 (Inspector sweep) + 2026-04-17 (Explorer slim sweep + dead-code reap):_ `Focus Editor/Inspector/Lower/Top/Palette` and `Overlay focus` dropped from the top bar. Inspector sub-panes rewritten: Editing/Palette/ComReference now carry only `Diagnostics` + `Symbols` (former `Session` sub-pane with `Dirty buffers: N / Visible views: N / Hidden buffers: N / Active cursor: L:C` removed); Semantic now carries only `Hover` + `Symbols` (former `Layout` sub-pane with `Preset: SplitEdit` leaking the internal `LayoutPreset` enum removed); BuildRun carries `Run Status` + `Target` (former `Workspace` sub-pane with `Layout: Run / Open buffers: N` removed). Explorer slim sweep on 2026-04-17 also removed the internal taxonomy leaking through the project tree: the `Layout` / `Views` / `Target` sub-panes under Project (carrying `Preset:` and `SplitEdit` labels, the `[Source:view]` role tag on module rows, and the `Primary view` echo on buffers) are all gone; the redundant second "declared" name line is suppressed when the declared name equals the logical module name, and surfaced only when they differ. `FocusRegion::label` (unused once the top-bar region badge was removed) deleted in the same pass so the dev-only taxonomy cannot leak back. Contract pinned by five Inspector tests — `empty_inspector_shows_only_capabilities`, `editing_inspector_carries_diagnostics_and_symbols_only`, `semantic_inspector_carries_hover_and_symbols_only`, `build_run_inspector_carries_run_status_and_target_only`, `palette_inspector_matches_editing_slim_contract` — plus three Explorer tests — `shell::mock_data::tests::{explorer_drops_layout_views_and_target_subpanes, explorer_drops_redundant_declared_name_line, explorer_surfaces_declared_name_when_it_differs_from_logical}`.
6. F2 / F3 / F4 dev scene-flips are removed from the default build. A `--dev-scenes` flag gates them for W035 prototyping. _Landed 2026-04-16:_ gated via `ShellModel::with_dev_scenes`; regression covered by `shell::model::tests::{f2_does_not_change_scene_in_default_build, f3_changes_scene_when_dev_scenes_enabled, welcome_hint_omits_scene_flip_bindings_in_default_build, palette_state_commands_are_empty_in_default_build, palette_state_commands_are_populated_when_dev_scenes_enabled}`; W037 goldens re-blessed.
7. Content that does not fit the region it is assigned is either moved to a region that can hold it, wrapped, or made expandable. Silent truncation is a defect. _Landed 2026-04-17:_ `render_panel` in `src/shell/view.rs` grew an `Option<WrapMode>` parameter; every content region that was observed silently truncating receives `Some(WrapMode::WordChar)` — Explorer, Inspector, Lower Surface, Welcome, and Overlay bodies now wrap on word boundaries (with char-break fallback for long identifiers / paths). The Editor surface intentionally stays `None` because source-code layout must preserve the author's column positions. Evidence: the re-blessed W037 `thin_slice_loaded.txt` golden shows `examples/thin-slice/ThinSliceHello.baspr` wrapping to a second line `oj` in Explorer, and the diagnostic `PMR-E-OPTION-PRIVATE-MODULE-KIND-UNRESOLVED: \`Option Private Module\` requires project/module-kind integration` wrapping across multiple lines in both the Inspector's Diagnostics sub-pane and the Lower Surface's Problems pane rather than mid-word-truncating. `thin_slice/00_loaded.txt` under `docs/uxpass/captures/` carries the same evidence.
8. `Ctrl+O` to open a project is the canonical cold-start action. It is surfaced on the Empty scene's status line, not in a hint paragraph. _Landed 2026-04-17 (co-landed with D3):_ Empty's single-line status hint is `Ctrl+O open project  Up/Down select recent  F6 palette  Ctrl+Q quit` — painted in the dedicated status-line row established by D3, not inside the Welcome pane's prose. The Welcome pane's body therefore no longer needs a hint line at all (the `hint` and `capabilities` fields on `LauncherContentState` were deleted in the same pass). Pinned by `shell::state::tests::empty_status_line_announces_ctrl_o` (verifies the string appears) and `shell::model::tests::empty_status_line_announces_ctrl_o_and_omits_dev_scene_flips` (verifies `F2`/`F3`/`F4` never sneak back in). Visible on the re-blessed W037 `empty.txt` golden row 40 and `cold_start/01_after_tab.txt`.

## Open Questions (forwarded to later docs)
- How the three-column frame adapts to scenes is the subject of [20_frame_and_regions.md](20_frame_and_regions.md), guided by P5.
- Whether Debug is a distinct scene or a layout preset within Editing is the subject of [30_scene_catalogue.md](30_scene_catalogue.md).
- The full keymap including VBA-IDE-compatible profile is the subject of [40_command_model.md](40_command_model.md), guided by P6 / P9.
- Degradation policy under weak terminals is the subject of [50_visual_language.md](50_visual_language.md), guided by P8.
