# Workset W050 — File And Document Services

## Ambition

Editable buffers in OxIde behave like real IDE documents.

A user types, undoes, redoes, saves, selects, cuts, copies, pastes,
finds-in-buffer, reloads after an on-disk change, splits the editor
into two views of the same buffer or of different buffers, and
observes every change exactly as any modern editor presents it — with
the frame contract from W035 D9-D18 honoured.

At the end of W050 the Editor surface is no longer a `Paragraph` over
`buffer.lines` — it is backed by `ftui_widgets::TextArea` +
`ftui_text::Rope` for correctness at scale, selection, and clipboard
operations.

## Dependencies

- **W030** — document identity / host session exists.
- **W040** — project lifecycle, recent MRU, session restore
  foundations. W050 extends the session record with buffer-level
  detail.
- Soft dep on **W037 / W038** — every bead here ships a `wtd`
  journey.

## Design

### What already landed (W050 pre-landings during W035 work)

The following pieces of W050 are already in force and continue to
apply. They are listed here so W050 does not re-build them:

- `BufferState` carries `source_path`, `line_ending`,
  `trailing_newline`, `BufferHistory` (undo/redo ring buffer).
- `Ctrl+S` / `Ctrl+Shift+S` save the active / all dirty buffers with
  line-ending preservation.
- `Ctrl+Z` / `Ctrl+Y` undo / redo using per-buffer history.
- Dirty marker ` *` on the editor title.
- `apply_scene` preserves in-flight edits across overlay transitions
  (critical for the Save/Undo flow).

### What W050 lands

**Real editor widget.** Replace the `Paragraph(buffer.lines)` render
path with a `ftui_widgets::TextArea` bound to a `ftui_text::Rope` per
buffer. The Rope owns the text; `BufferState` holds metadata
(path, line_ending, dirty, history). Cursor and selection become
first-class.

**Selection.** Shift+arrow / Shift+Home / Shift+End / Shift+PgUp /
Shift+PgDn extends selection from cursor. Mouse drag from cursor
sets selection. Double-click selects a word; triple-click selects a
line. Selection painted with the `theme::selection()` background.

**Clipboard.** `Ctrl+C`, `Ctrl+X`, `Ctrl+V` via `crossterm`'s system
clipboard feature. Cut without selection cuts the current line. Copy
without selection copies the current line.

**Find-in-buffer.** `Ctrl+F` opens a one-row input strip between the
editor body and the lower surface. Typing filters matches, which are
highlighted in the editor body. `F3` / `Shift+F3` move to next /
previous match. `Esc` closes the strip.

**Reload-on-disk-change.** A file watcher per open buffer's
`source_path`. When the on-disk file changes and the buffer is clean,
silently reload. When the buffer is dirty, surface a popover: "File
changed on disk. Reload / Keep mine / Merge (Diff view)."

**Split panes and buffer roster.** Two canonical splits: horizontal
(`Ctrl+\`) and vertical (`Ctrl+-`). Each pane is a `ftui_layout::pane`
leaf. `Ctrl+Tab` cycles focus between panes (replacing the current
"cycle views on the same buffer" meaning). Buffer roster overlay
(`Ctrl+P`) is a fuzzy-filtered list of all open buffers; Enter
switches the active pane to that buffer.

**Extended session restore.** `session.json` now records per-buffer
open state, cursor, scroll_top, selection, and the layout (split
structure). W040-B04's basic session restore extends to the full
editor state.

## Beads

### W050-B01 — TextArea + Rope-backed editor

**Feature.**

- **Goal.** The Editor panel is backed by `ftui_widgets::TextArea` +
  `ftui_text::Rope`. Every existing edit primitive (insert_char,
  insert_newline, backspace) still works; undo/redo still works;
  syntax highlighting still works.
- **Design.** Replace `BufferState.lines: Vec<String>` with
  `BufferState.rope: Rope`. The highlight path lexes line-by-line
  from the Rope on render. All existing `ShellState` edit methods
  now delegate to the Rope. Undo history snapshots the Rope
  (via lightweight clone); existing `BufferHistory` API preserved.
- **Tests.** Full existing test suite still green (140+). New unit
  contract: Rope + lines() round-trip; edit primitive equivalence
  against the old `Vec<String>` semantics. New `wtd` journey:
  `tests/wtd/journey_edit_large_file.rs` opens a 10k-line fixture,
  edits mid-file, confirms responsiveness.
- **Evidence.** Five-minute pass exercising Ctrl+S / Ctrl+Z / Ctrl+Y
  + typing on the thin-slice. No regressions vs prior behaviour.

### W050-B02 — Selection model + Shift+arrow + drag

**Feature.**

- **Goal.** Shift+arrow extends the cursor into a selection; mouse
  drag from the editor creates a selection; the selection renders
  with `theme::selection()` background; Esc collapses the selection;
  typing replaces the selection.
- **Design.** `SelectionRange` on `EditorSurfaceState` already exists
  as a stub — populate it. New `Msg::ExtendSelection{Left,Right,Up,Down,
  Home,End,PgUp,PgDn}` for Shift+arrows. Render layer paints cells
  inside the selection range with the selection background.
- **Tests.**
  - Unit contract: shift-left/right from a known cursor produces
    the expected range; Esc clears; typing a character replaces.
  - `wtd` journey:
    `tests/wtd/journey_select_shift_arrow.rs` drives Shift+Right x5
    on line 5 and asserts 5 cells render in selection colour.
- **Evidence.** Five-minute pass covering all Shift+arrow variants +
  a mouse drag (once mouse drag lands; see B04).

### W050-B03 — Clipboard copy/cut/paste

**Feature.**

- **Goal.** `Ctrl+C` copies the current selection (or current line if
  no selection); `Ctrl+X` cuts; `Ctrl+V` pastes at the cursor
  (replacing any selection).
- **Design.** Use `crossterm`'s system clipboard integration (already
  a dep via `ftui`'s `crossterm` feature) or `arboard` if that's
  cleaner. `Msg::CopyToClipboard` / `CutToClipboard` / `PasteFromClipboard`.
- **Tests.**
  - Unit contract: copy round-trips text through a mock clipboard
    handle; cut removes + clips; paste inserts at cursor.
  - `wtd` journey:
    `tests/wtd/journey_copy_paste.rs` selects a run of characters,
    `Ctrl+C`, moves cursor, `Ctrl+V`, asserts the duplicated text
    is present.
- **Evidence.** Five-minute pass including cross-app paste into and
  out of a system clipboard app.

### W050-B04 — Mouse selection and click placement

**Feature.**

- **Goal.** Left-click in the editor body places the cursor; left-drag
  creates a selection; double-click selects a word; triple-click
  selects a line.
- **Design.** Mouse events routed to the editor when focus is Editor
  and the mouse is inside the editor rect. Cursor/selection math
  accounts for the gutter (`highlight::gutter_total_width`).
- **Tests.**
  - Unit contract: (mouse row, col) → (buffer line, column)
    conversion; double-click expands to word boundary; triple-click
    expands to line.
  - `wtd` journey:
    `tests/wtd/journey_mouse_click_in_editor.rs` clicks at a known
    cell; asserts cursor in the top bar.
- **Evidence.** Five-minute mouse pass.

### W050-B05 — Find-in-buffer (Ctrl+F)

**Feature.**

- **Goal.** `Ctrl+F` opens a one-row find strip between editor and
  lower surface. Typing filters matches in the active buffer;
  matches render with a distinct highlight; `F3` and `Shift+F3`
  cycle forward / backward; `Esc` closes.
- **Design.** New `FindBarState { query, match_cursor, case_insensitive }`
  on `ShellRuntimeState`. New `FocusRegion::FindBar`. View adds a
  one-row section above the Lower Surface when active. Match
  highlights paint in `theme::find_highlight()`.
- **Tests.**
  - Unit contract: match enumeration over known buffers; next/prev
    wraps; Esc closes; Enter jumps to the next match without
    closing.
  - `wtd` journey:
    `tests/wtd/journey_find_next_match.rs` opens thin-slice,
    `Ctrl+F`, types `answer`, asserts cursor lands on a match and
    the find strip shows match count.
- **Evidence.** Five-minute pass including empty-query,
  no-match, and wrap cases.

### W050-B06 — Reload-on-disk-change

**Feature.**

- **Goal.** If a buffer's `source_path` changes on disk while the
  buffer is clean, OxIde silently reloads. If dirty, OxIde surfaces
  a popover offering Reload / Keep Mine / Diff.
- **Design.** `notify` crate or equivalent file watcher per open
  source path. Debounced. Popover uses the hover-popover infra with
  three-action prompt (the three-option choice is the first modal
  that accepts user selection on a popover).
- **Tests.**
  - Unit contract: clean-buffer reload path applies on-disk content
    without asking; dirty-buffer reload path installs the three-
    action popover.
  - `wtd` journey:
    `tests/wtd/journey_external_edit_reloads_clean.rs` opens
    thin-slice, writes a modification to `Module1.bas` on disk from
    the test, asserts the buffer picks up the change.
- **Evidence.** Five-minute pass including dirty-reload conflict.

### W050-B07 — Split panes + buffer roster overlay

**Feature.**

- **Goal.** `Ctrl+\` splits the editor horizontally into two panes;
  `Ctrl+-` splits vertically; `Ctrl+Tab` cycles focus between panes;
  `Ctrl+W` closes the focused pane (merging neighbouring). `Ctrl+P`
  opens a buffer roster overlay with fuzzy filter; Enter switches
  the active pane to the selected buffer.
- **Design.** Use `ftui_layout::pane` for the split tree. Replace
  the existing `ViewState / WorkspaceLayoutState` view-cycle model
  with a pane tree whose leaves are views. Buffer roster overlay
  reuses the palette infra with a different data source.
- **Tests.**
  - Unit contract: split inserts a new pane leaf; close merges;
    focus cycle walks the tree depth-first.
  - `wtd` journey:
    `tests/wtd/journey_split_and_switch.rs` opens thin-slice,
    `Ctrl+\`, asserts two editor rects visible, `Ctrl+P` open the
    roster, selects `Module1.bas` in the second pane, asserts both
    panes show the same buffer content.
- **Evidence.** Five-minute pass covering split / close / focus
  cycle with both splits.

### W050-B08 — Full session restore (buffers / cursors / selection / layout)

**Feature.**

- **Goal.** `session.json` extends to record the full editor state:
  every open buffer, each buffer's cursor, scroll_top, and selection;
  and the current pane-tree layout. On relaunch, all of it restores.
- **Design.** Extend the session schema. Atomic write on every
  close-editor-transition (debounced so each keypress doesn't
  write). Overlay logic on startup.
- **Tests.**
  - Unit contract: round-trip a fully-populated session record.
  - `wtd` journey: `tests/wtd/journey_full_session_restore.rs`
    creates a split, opens two buffers, selects a range, quits,
    relaunches, asserts everything restored.
- **Evidence.** Five-minute pass with a multi-buffer, split-pane,
  mid-selection state.

## Out-of-scope

- **Syntax highlighting for languages other than VBA** — the
  `highlight::tokenize` lexer stays VBA-specific.
- **Tree-sitter upgrade** — the current hand-written lexer is
  sufficient for W050; a tree-sitter VBA grammar is a later
  improvement, not a W050 bead.
- **Collaborative editing** — single-author editing only.
- **Language-service actions** — hover, goto-def already exist as
  pre-landings; the full navigation surface (find references,
  symbols picker, rename, code actions) is W060.
