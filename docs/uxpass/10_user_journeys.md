# 10 - User Journeys

Status: `draft`
Workset: `W035`

## Purpose
Concrete journeys, keystroke by keystroke, against the running W030 shell.
Each journey cites the principles from [00_principles.md](00_principles.md)
that the current shell honors or violates, so later uxpass docs
(frame, scenes, commands, visuals) know what to change and why.

## Verification Status

- Landed annotations in this document are interpreted as:
  - **Unit-tested; end-to-end verification pending**, unless a release-binary
    `tests/wtd/backfill.rs` journey now covers the same interaction.
- W045 is the active pass that promotes these notes to interactive evidence.
- As of `2026-04-18`, W045 backfill journeys plus a release-binary
  five-minute user pass cover the core shipped bindings in J1-J4:
  open/create/save/save-all/undo-redo/hover/goto-def/run-return/palette
  dispatch and overlay-preservation flows.

Captures are committed under
[captures/](captures/) (cold_start/, thin_slice/) and referenced by
filename below. They were produced via the W037 wtd harness against a
120x40 terminal; larger width classes are explored separately in
[20_frame_and_regions.md](20_frame_and_regions.md).

Keystrokes below are as they exist today, discovered from
[src/shell/model.rs](../../src/shell/model.rs):
- `Ctrl+O` open project
- `F5` run
- `F6` toggle palette
- `Tab` next focus, `Alt+1..4` direct focus
- `Ctrl+Tab` cycle editor view
- `Ctrl+Shift+M` / `Ctrl+Shift+C` / `Ctrl+Shift+R` / `Ctrl+Shift+T`
- `Escape` close overlay, `Ctrl+Q` quit
- `F2` / `F3` / `F4` are dev scene-flips (removed by D6)

---

## J1 - Cold start to loaded project

**Intent.** Launch OxIde with no project; open the thin-slice project
from the welcome scene using only the keyboard.

### Observed path

1. `cargo run -p ox-ide` (or the packaged binary) -> Empty scene painted.
   Capture: [captures/cold_start/00_welcome.txt](captures/cold_start/00_welcome.txt).
2. `Ctrl+O` -> project loads; scene transitions to Editing; focus
   defaults to the Editor region.
   Capture: [captures/cold_start/01_after_ctrl_o.txt](captures/cold_start/01_after_ctrl_o.txt).

### What the user actually sees

On Empty (`00_welcome.txt`):
- Top bar: `No project | Project | Focus Editor | Standard | Truecolor ready`.
  Seven pipe-separated fields; "Standard" is an internal width-class
  label; "Truecolor ready" restates what the Environment pane already says.
- Three columns: Launcher (left), Welcome (middle), Environment (right).
- Launcher and Welcome both list `Open Project / Create Project / Browse Recent`.
- Environment pane surfaces `Capabilities`, `Theme` (`Mockup-derived
  instrument palette`, `High-contrast panel hierarchy`), and `Tokens`
  (`bg #0A0E14 panel #0D1117 pane...`).
- Welcome hint line: `Ctrl+O open selected  Up/Down select  F2 Empty  F3 Edit  F4 Sem`.

### Findings against the principles

| # | Observation | Principle |
| - | ----------- | --------- |
| J1-a | Launcher and Welcome duplicate the same command list. | P2 - one source of truth per affordance; decision D1 removes the Inspector column on Empty. **Unit-tested + release-binary WTD-verified + five-minute pass `2026-04-18`**: D1a removed the Environment/Inspector column. D1b then collapsed the two-column body into a single full-width Welcome panel — Launcher as a separate pane is gone; Welcome owns both the recent-projects list and the Start actions (`Open Project / Create Project`). Unit-test coverage: `shell::state::tests::empty_scene_focus_ring_is_editor_only`, `shell::mock_data::tests::welcome_owns_the_launcher_role_on_empty`, `shell::mock_data::tests::empty_scene_emits_no_explorer_panel_text`. Re-blessed [captures/cold_start/00_welcome.txt](captures/cold_start/00_welcome.txt) and [captures/cold_start/01_after_tab.txt](captures/cold_start/01_after_tab.txt) show Empty as `[top bar][single Welcome panel][status line]`. |
| J1-a' | Top bar on Empty read `No project \| Project \| Focus Editor \| Standard \| Truecolor ready`. | P1 / P3 / D2 / D4 / D5 — **Unit-tested; end-to-end verification pending**: now reads `No project \| Empty`. |
| J1-b | `Up`/`Down` in the hint do nothing (confirmed live: capture `01_after_tab.txt` shows Tab cycled focus, arrows were a no-op). | P4 - never mislead; P6 - every binding documented in place. After D1b+D16 the focus ring on Empty is `[Editor]` only, so Tab is a no-op and `Alt+1` / `Alt+3` are rejected on Empty (`shell::state::tests::{empty_scene_focus_ring_is_editor_only, empty_scene_alt1_focus_request_is_rejected, empty_scene_alt3_focus_request_is_rejected}`). |
| J1-c | `F2 Empty  F3 Edit  F4 Sem` are dev scene-flips shown to end users. | P7 - dev affordances are not user affordances; decision D6 gates them behind `--dev-scenes`. **Unit-tested; end-to-end verification pending** (dev gate) + **2026-04-17** (status-line rewrite for D1b + D3 + D8): the Welcome pane no longer carries a hint line at all. The canonical status line on Empty is now `Ctrl+O open project  Up/Down select recent  F6 palette  Ctrl+Q quit`, painted in the dedicated bottom row established by D3. Unit-test coverage: `shell::model::tests::empty_status_line_announces_ctrl_o_and_omits_dev_scene_flips`; evidence in [captures/cold_start/00_welcome.txt](captures/cold_start/00_welcome.txt) row 40. |
| J1-d | Environment pane shows hex tokens and mockup taxonomy. | P1 - every surface is user-facing; decision D4 removes these from the shipping shell. **Unit-tested; end-to-end verification pending**: the `Theme` sub-pane and the `Tokens` hex-code dump are deleted; after D1b the whole Environment/Inspector column is gone on Empty, so the issue cannot recur. See [captures/cold_start/00_welcome.txt](captures/cold_start/00_welcome.txt) after re-bless. Unit-test coverage: `shell::state::tests::empty_inspector_shows_only_capabilities`. |
| J1-e | Top bar field `Focus Editor` on Empty, before any editor exists. | P5 - match shape to task; P10 - scene names are for users. |
| J1-f | `Ctrl+O` works, but is not announced in a status line; it is a line of prose inside the Welcome pane. | P6 - every binding documented in place; decision D8 elevates it to the Empty scene's status line. **Unit-tested + release-binary WTD-verified + five-minute pass `2026-04-18`** (co-landed with D3): the Welcome pane no longer carries a prose hint; the binding lives in the permanent bottom status line row as `Ctrl+O open project  Up/Down select recent  F6 palette  Ctrl+Q quit`. Unit-test coverage: `shell::state::tests::empty_status_line_announces_ctrl_o` and `shell::state::tests::status_line_hint_never_exceeds_one_line`. Row 40 of the re-blessed W037 `empty.txt` golden and [captures/cold_start/01_after_tab.txt](captures/cold_start/01_after_tab.txt) carry the evidence. |

### What J1 implies for later docs

- 20_frame_and_regions: on Empty, give Welcome the full width; kill
  Launcher and Environment as separate regions.
- 40_command_model: the canonical entry points for a cold start are
  `Ctrl+O`, `Ctrl+N`, `Ctrl+R` (recent). The status line declares them;
  the palette (`F6`) enumerates them with bindings.
- 50_visual_language: reserve the bottom row as a status line whose sole
  job is "what keystrokes are available right now" (decision D3).

---

## J2 - Edit a module and observe live semantic feedback

**Intent.** With ThinSliceHello loaded, move the cursor into the body of
`Module1.bas`, make an edit that breaks the code, observe the shell's
response, then undo.

### Observed path

1. Start with project loaded, focus Editor.
   Capture: [captures/thin_slice/00_loaded.txt](captures/thin_slice/00_loaded.txt).
   (Here focus has already been Tab-cycled once to Inspector; the
   starting focus after `Ctrl+O` is Editor, per J1 step 2.)
2. `Down Down Down` to reach `Public Sub Main()` (Ln 5).
3. `Right Right Right` to land at Col 3, between `Pu` and `b`.
4. Type `x`. Status line updates to `Ln 5 Col 4`. Source shows
   `Puxblic Sub Main()`. Inspector Symbols pane replaces
   `Main / answer / Integer / Main` with `No symbols discovered in mounted project`.
   Problems pane updates.
   Capture: [captures/thin_slice/05_after_typo.txt](captures/thin_slice/05_after_typo.txt).
5. `Backspace`. Source restored. Symbols return to `Main`. Problems clear.

### What works well

- Cursor movement and status-line Ln/Col readout are correct and
  instantaneous.
- Semantic re-analysis is synchronous: typing a character and the
  resulting diagnostic/symbol state both appear in the same frame.
- The Editor has a visible focus ring (`> Module1.bas | Primary View <`
  arrows flank the title).

### Findings against the principles

| # | Observation | Principle |
| - | ----------- | --------- |
| J2-a | Diagnostic text wraps silently at the Problems pane edge: `...requires project/mo` truncates mid-word. | P4 - never truncate what the user needs to act; decision D7 applies wrap semantics. **Unit-tested; end-to-end verification pending**: `render_panel` in `src/shell/view.rs` now takes `Option<WrapMode>` and Explorer / Inspector / Lower Surface / Welcome / Overlay are all rendered with `WrapMode::WordChar` (word boundary, char-break fallback). Editor stays unwrapped so source-code columns are preserved. Evidence in the re-blessed W037 `thin_slice_loaded.txt` golden: the diagnostic `PMR-E-OPTION-PRIVATE-MODULE-KIND-UNRESOLVED: \`Option Private Module\` requires project/module-kind integration` wraps across multiple lines in both the Inspector Diagnostics sub-pane and the Lower Surface Problems pane, and `examples/thin-slice/ThinSliceHello.baspr` wraps to a following `oj` line in Explorer. Same evidence in [captures/thin_slice/00_loaded.txt](captures/thin_slice/00_loaded.txt). |
| J2-b | Inspector `Session` sub-pane shows `Dirty buffers: 0 / Visible views: 1 / Hidden buffers: 0 / Active cursor: 1:1`. The cursor readout duplicates the top bar; the buffer counts are internal state. | P1 - dev telemetry; P3 - density means information, not noise. **Unit-tested; end-to-end verification pending**: `Session` sub-pane deleted; Editing / Palette / ComReference Inspector now carries only `Diagnostics` + `Symbols` (see [captures/thin_slice/00_loaded.txt](captures/thin_slice/00_loaded.txt), [captures/thin_slice/05_after_typo.txt](captures/thin_slice/05_after_typo.txt), [captures/thin_slice/07_palette_open.txt](captures/thin_slice/07_palette_open.txt) after re-bless). Unit-test coverage: `shell::state::tests::editing_inspector_carries_diagnostics_and_symbols_only` + `palette_inspector_matches_editing_slim_contract`. |
| J2-c | Inspector Symbols lists `Integer` alongside `Main` and `answer`. `Integer` is a type, not a user-declared symbol in this file. | Signals the symbols projection needs filtering; noted for 40_command_model and 60_reconciliation. Not a principle violation, but an accuracy gap. **Unit-tested; end-to-end verification pending**: `load_semantic_state` in `src/shell/oxvba.rs` filters on `SymbolProvenanceKind::SourceModule` _and_ screens out (a) symbols whose `provenance.document_id` starts with `__OxVba` (generated helpers) and (b) symbols whose name is a VBA intrinsic type keyword (`Boolean / Byte / Currency / Date / Decimal / Double / Integer / Long / LongLong / LongPtr / Object / Single / String / Variant`). The intrinsic-name screen is a defensive mitigation for an OxVba language-service defect that reports the type token in `Dim x As Integer` as a `Variable` declared inside the enclosing Sub — without the screen that bug leaks `Integer` straight into the Symbols pane. Unit-test coverage: `shell::oxvba::tests::{vba_intrinsic_type_names_cover_the_builtin_scalar_types, loads_semantic_state_from_real_oxvba_workspace_session}` (the latter now asserts no intrinsic type name reaches `semantic.symbols`). |
| J2-d | No save/dirty indicator on the buffer title when the user has typed. The file remains on disk unchanged; the in-memory buffer diverges silently. | P4 - user needs to know when their change is uncommitted. **Unit-tested + release-binary WTD-verified + five-minute pass `2026-04-18` (marker + save path)**: the marker landed in `editor_title` in `src/shell/mock_data.rs` — appends ` *` when the active buffer's `dirty` flag is set. The W050-C1 save path landed later the same day: `Ctrl+S` writes `buffer.lines` back to `source_path` (preserving the detected `line_ending` and trailing-newline flag) and clears `dirty`; `Ctrl+Shift+S` saves every dirty buffer. The `*` is now an honest handshake, not just a warning: it appears on edit, disappears on save. Unit-test coverage: `shell::mock_data::tests::editor_title_gains_dirty_marker_after_edit_and_clears_without_one_otherwise` plus the save-path test cluster `shell::state::tests::{save_active_buffer_writes_dirty_lines_to_disk_and_clears_dirty, save_all_dirty_buffers_persists_every_dirty_buffer, save_preserves_lf_line_ending_when_original_was_lf, save_preserves_no_trailing_newline_when_original_had_none}` and model-layer dispatch tests `shell::model::tests::{save_dispatches_through_model_and_clears_dirty_marker, maps_ctrl_s_to_save_active_buffer, maps_ctrl_shift_s_to_save_all_dirty_buffers}`. |
| J2-e | No undo/redo. Correction is manual backspace. | **Unit-tested + release-binary WTD-verified + five-minute pass `2026-04-18` (W050 pre-landing)**: per-buffer `BufferHistory` in `src/shell/state.rs` snapshots the pre-edit `lines` + cursor before every `insert_char` / `insert_newline` / `backspace`. `Ctrl+Z` pops from undo onto redo and restores; `Ctrl+Y` pops from redo onto undo and re-applies; a new edit after undo invalidates the redo stack (standard editor convention). Default capacity is 64 snapshots per buffer (a ring buffer so memory stays bounded). The Editing status-line hint now carries `Ctrl+Z undo` next to `Ctrl+S save` so discovery is on the always-present bottom row (D3 / D6). Unit-test coverage: `shell::state::tests::{undo_restores_previous_lines_and_cursor, undo_on_empty_history_is_a_noop, redo_reapplies_undone_edit, new_edit_after_undo_clears_redo_history, buffer_history_capacity_bounds_memory}` and model-level `shell::model::tests::{undo_and_redo_round_trip_through_the_model, maps_ctrl_z_to_undo_active_buffer, maps_ctrl_y_to_redo_active_buffer}`. Selection, clipboard, find-in-buffer, syntax highlighting, and the full `ftui_widgets::TextArea` swap remain W050 proper. |

### What J2 implies for later docs

- 20_frame_and_regions: the Inspector must grow wide enough to hold
  diagnostic text, OR the Problems pane moves to the lower surface where
  it has width, OR diagnostics wrap (decision D7).
- 30_scene_catalogue: define the difference between editing-in-progress
  and editing-committed; the buffer title must reflect dirty state.
- 50_visual_language: pick a non-noisy dirty marker (trailing bullet,
  accent color, etc.); the current shell has nothing.

---

## J3 - Build and run the active project

**Intent.** With ThinSliceHello loaded, press `F5` to run; observe
output; return to editing.

### Observed path

1. Focus Editor, project loaded.
2. `F5` -> scene transitions to BuildRun. Focus moves to Lower. Inspector
   mode changes to RunStatus. Top bar reshapes.
   Capture: [captures/thin_slice/06_after_f5_run.txt](captures/thin_slice/06_after_f5_run.txt).
3. The lower surface fills with `Output`: `[workspace]`, `[diagnostic]`,
   `[stdout]` prefixed lines. Inspector RunStatus shows
   `Build: passing / Runtime: completed / Profile: win-console / Last exit: 0`.
4. `Escape` now returns from BuildRun to Editing (documented in the
   BuildRun status line).

### What works well

- The scene contract is consistent: focus follows the task; inspector
  mode matches the scene; the lower surface becomes the primary reading
  surface for output. This is exactly the shape P5 asks for.
- Per-line prefixes `[workspace]`, `[diagnostic]`, `[stdout]` are a good
  lightweight signal of origin.

### Findings against the principles

| # | Observation | Principle |
| - | ----------- | --------- |
| J3-a | Top bar retains `Ln 1 Col 1` during Run even though the Editor is not the active surface and Ln/Col is not meaningful now. | P3 - fields that restate state or show irrelevant state go. **Unit-tested; end-to-end verification pending**: Run top bar is now `<project> \| Run \| <build> / <runtime>`; Ln/Col is dropped in this scene. |
| J3-b | Build reports `passing` while `Problems` simultaneously lists two errors from `__OxVbaStartupEntryShim`. The errors originate from a tool-generated helper, but the user cannot tell that from the label. | P4 / P10 - honesty about state; "passing" must not co-exist with user-visible error rows without explanation. **Partial landing 2026-04-17** (origin labeling): diagnostics from any `__OxVba…` document now render as `[generated] <severity> <message>` instead of `__OxVbaStartupEntryShim <severity> <message>`, so the user can see at a glance that the row is not about their own module. The deeper `passing` / `Problems` inconsistency (a build status that claims success while generated-helper diagnostics still stand) is a W040 / W060 follow-up — the builder layer is what decides whether generated diagnostics block a `passing` verdict; labeling alone cannot fix it. Unit-test coverage: `shell::oxvba::tests::{is_generated_document_id_matches_oxvba_tool_helpers, format_diagnostic_line_tags_generated_documents_and_preserves_user_ids}`. |
| J3-c | Output line `[stdout] project run completed with 1 user slots` exposes the term "user slots". | P1 - no dev jargon on user surfaces. **Unit-tested; end-to-end verification pending**: `sanitize_output_text` in `src/shell/oxvba.rs` trims the `" with <digits> user slots"` tail from every OxVba `WebHostEvent::OutputLine` before it reaches the Output pane or the build log, so the Output surface now reads `[stdout] project run completed`. Only the specific jargon suffix is stripped; unrelated `with` clauses are preserved. Unit-test coverage: `shell::oxvba::tests::{sanitize_output_text_strips_user_slots_suffix_from_run_completion, sanitize_output_text_preserves_unrelated_with_clauses, runs_project_through_real_oxvba_runtime_contract}` — the last assertion now fails if any output or log line leaks the phrase `user slots`. |
| J3-d | Inspector `Workspace` sub-pane repeats what Explorer already shows (`Entry: Module1.Main`, `Active buffer: Module1.bas`, `Open buffers: 1`). | P2 - one source of truth; P3 - noise. **Unit-tested; end-to-end verification pending**: `Workspace` sub-pane deleted; BuildRun Inspector now carries `Run Status` (build/runtime/profile/last exit) + a new slim `Target` (entry + active buffer — the single piece the explorer does _not_ uniquely own, kept here so the user can see what Run is about to invoke). `Layout: Run` and `Open buffers: N` are gone. See [captures/thin_slice/06_after_f5_run.txt](captures/thin_slice/06_after_f5_run.txt) after re-bless. Unit-test coverage: `shell::state::tests::build_run_inspector_carries_run_status_and_target_only`. |
| J3-e | No status line announces what to do next: "how do I go back to editing?", "how do I re-run?". | P6 - every binding documented in place; decision D3 - status line is always present. **Unit-tested + release-binary WTD-verified + five-minute pass `2026-04-18`**: BuildRun's status line now reads `F5 rerun  Esc return  F6 palette  Tab next focus  Ctrl+Q quit` — painted in the always-present bottom row established by D3. Unit-test coverage: `shell::state::tests::build_run_status_line_announces_rerun`; visible in [captures/thin_slice/06_after_f5_run.txt](captures/thin_slice/06_after_f5_run.txt). |

### What J3 implies for later docs

- 30_scene_catalogue: define BuildRun as a scene with its own status
  line, its own top-bar field set, and an explicit return-to-editing
  affordance (probably `Escape` or `F5` again).
- 30_scene_catalogue: resolve whether the Problems pane is
  scene-scoped (only during Editing/BuildRun that has real diagnostics)
  or always-on, and whether the `__OxVbaStartupEntryShim` diagnostics
  belong in a separate "generated code" bucket.
- 40_command_model: declare the minimum re-run / stop / rerun-with-args
  set and bind them consistently.

---

## J4 - Discover commands via the palette

**Intent.** User does not remember a binding; presses `F6` to find the
action by name.

### Observed path

1. In any scene, `F6` opens the palette overlay.
   Capture: [captures/thin_slice/07_palette_open.txt](captures/thin_slice/07_palette_open.txt).
2. Palette lists `Open Project`, `Create Project`, save/undo actions,
   focus actions, module/reference actions, and `Toggle Palette`.
3. `Escape` closes the palette.

### What works well

- Commands are annotated with their bindings. This is a natural
  reference once the palette is discovered.
- `F6` to open is memorable and mirrors VS Code's `F1`.

### Findings against the principles

| # | Observation | Principle |
| - | ----------- | --------- |
| J4-a | Palette overlay is not opaque. Editor text `Integer` and `answer = 40 + 2` bleeds through the palette frame at the top. | P1 / P4 - the overlay must not produce visually corrupt output. Bug for W050 or earlier. **Unit-tested; end-to-end verification pending**: `render_overlay` in `src/shell/view.rs` now calls `frame.buffer.fill(overlay, Cell::default())` before the Block + Paragraph render, so editor glyphs underneath the overlay rect are erased rather than recoloured in place. (Block's `set_style_area` only changes the cell style; it preserves cell content, which is why the bleed-through happened.) Unit-test coverage: `shell::view::tests::palette_overlay_is_opaque_over_editor_text`, which renders Editing → Palette open at 120×40, computes the overlay rect, and asserts that `Integer` and `answer = 40` are not present on any interior row. As a co-benefit, the J4-d "Command Palettes" artefact (the `s` of `As` bleeding through next to the singular title `Command Palette`) is fixed by this same change — the title in code was always singular. |
| J4-b | Section `Mockup States` with F2/F3 scene-flip entries is listed as a first-class palette group. | P7 - decision D6 removes it from the default build. |
| J4-c | Top bar during overlay: `ThinSliceHello \| Exe \| Palette \| Overlay focus \| Mockup-derived instrument palette`. `Overlay focus` and `Mockup-derived instrument palette` are internal labels. | P1 - decision D4 / D5. **Unit-tested; end-to-end verification pending**: Palette top bar reduced to `<project> \| Palette`. Co-landed with D3: the palette overlay receives a dedicated status-line hint `Esc close  Up/Down select  Enter apply` in the always-present bottom row, so users never have to guess how to dismiss the overlay. Unit-test coverage: `shell::state::tests::palette_overlay_status_line_is_overlay_hint` (palette) and `shell::state::tests::com_reference_overlay_status_line_is_overlay_hint` (COM reference helper). |
| J4-d | Palette header reads `Command Palettes` (plural) followed by `Filter`. | Wording inconsistency; flag for 40_command_model. **Unit-tested; end-to-end verification pending** (co-landed with J4-a): the palette title in code was always singular (`Command Palette`); the trailing `s` observed in `captures/thin_slice/07_palette_open.txt` was the leading `s` of `As` in `Dim answer As Integer` bleeding through from the editor underneath. Once J4-a made the overlay opaque the bleed-through disappears and the title reads as written. Evidence: re-captured overlay row shows only the singular title. |
| J4-e | Project creation is advertised in the palette (`Create Project`, `Ctrl+N`) and must dispatch a real scaffold flow. | P6 - every shown binding must actually work. **Unit-tested; end-to-end verification pending**: `Create Project` remains in the palette and dispatches to `Msg::CreateNewProject`; it is no longer a silent no-op. Unit-test coverage: `shell::state::tests::palette_create_project_is_wired_to_ctrl_n` and `shell::model::tests::maps_ctrl_n_to_create_new_project`. |

### What J4 implies for later docs

- 40_command_model: the palette is the canonical discovery surface.
  Every registered action appears there, annotated with its default
  binding and its VBA-IDE-profile binding.
- 50_visual_language: modal overlay rendering must be opaque and
  framed distinctly from underlying panes (degradation budget allowing).

---

## Open questions surfaced by these journeys

Forwarded to later docs, not answered here.

1. Does the palette ALSO serve as the go-to-file / go-to-symbol /
   go-to-line entry point, or are those separate commands with their own
   overlays? (40_command_model)
2. Where does the status line live during an overlay, and does the
   overlay itself extend the status line? (20_frame_and_regions,
   50_visual_language)
3. Should `__OxVbaStartupEntryShim` diagnostics surface in the user's
   Problems pane at all, or only in a "generated code" view? The current
   behavior is dishonest about what the user's code did or did not
   compile. (30_scene_catalogue, cross-check with OxVba frozen mirror.)
4. Is the `Focus Top` region reachable by Tab a useful surface, or an
   artifact of treating the top bar as a region? (20_frame_and_regions)
5. When Build is `passing` but diagnostics are present, what does the
   user see? A composite state, a qualification ("passing with N
   generated warnings"), or an assertion that `__OxVba...` never surfaces
   as an error? (30_scene_catalogue)
6. Does `F5` during a running project re-run, stop, or no-op?
   (40_command_model)

---

## Handback criterion for J1-J4

These four journeys are the minimum evidence base for rewriting
20_frame_and_regions and 30_scene_catalogue. When those docs land they
must cite specific J-numbered findings. If they do not, this doc did not
do its job and should be expanded first.
