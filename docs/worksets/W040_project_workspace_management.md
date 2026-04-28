# Workset W040 — Project And Workspace Management

## Ambition

A mounted `.basproj` workspace feels alive in OxIde. The user sees real
project structure in the Explorer (not a rendered mock); runs
project-management actions (add module, add class, add reference,
cycle target) from keyboard or palette with visible, immediate feedback;
switches between projects from a persistent recent list; and returns
to the exact workspace state they left on relaunch.

At the end of W040 an OxVba author uses OxIde as a *project
environment*, not just a file editor.

## Dependencies

- **W030** — direct OxVba host integration (exists).
- **W037** — `wtd` harness (exists; each bead lands a journey).
- **W038** — UX development lab (ideal for authoring bead scenarios;
  not hard-blocking).
- **W039** — Fire Horse terminal UX proof. Use the Project Spine,
  Launchpad, and real-editing adapter outputs as presentation input; do
  not treat fixture rows as project truth.

### W039 Fire Horse Input

W039 proved the Project Spine, Launchpad recent rows, and read-only
`ShellState -> FireHorseProjection` adapter in terminal cells. W040 owns
the real behavior behind those surfaces: `ProjectSession` truth,
Explorer navigation, project mutations, MRU persistence, session
restore, and honest dirty-buffer gates. Any W039 unavailable projection
for project/session data should become either a real W040 seam or an
explicit deferral.

## Design

### Explorer as a real project tree

The Explorer column renders `ProjectSession::project.modules` +
`.references` directly. Each module row carries the module's logical
name, kind (Module / Class / Document), and — when it differs — the
declared VB_Name on a continuation line (D4 / D5 slim rule; no
internal taxonomy such as `Preset:` or `[Source:view]`).

Keyboard:
- `Up` / `Down` selects a row when Explorer has focus.
- `Enter` opens the selected module (active buffer switches; scene
  stays Editing).
- `Ctrl+Shift+M` / `Ctrl+Shift+C` / `Ctrl+Shift+R` already scaffold a
  module / class / reference; their palette dispatch already works.
- `Ctrl+Shift+T` already cycles the project's target kind.

Mouse: left-click a module row to open it; right-click opens a small
context overlay (rename / remove / reveal on disk). Right-click
deferred if time-boxed — keyboard must work regardless.

### Project-management actions

All four write through `oxvba_project` helpers and re-mount the
workspace afterwards so the explorer reflects truth. Contracts already
exist in `src/shell/project_actions.rs`; this workset wires them to
first-class UX (toasts, error popovers, confirmation for destructive
changes).

### Recent projects MRU

A persisted recent list at `%APPDATA%/OxIde/session.json`:

```
{
  "recent_projects": [
    { "path": "C:/…/ThinSliceHello.basproj", "opened_at": "2026-04-18T…" },
    …
  ],
  "last_opened": "C:/…/ThinSliceHello.basproj"
}
```

Welcome's Recent list reads from this file instead of
`discover_projects(cwd)`. `Ctrl+O` opens `last_opened` when the list is
non-empty; otherwise surfaces the honest feedback popover already
shipped.

### Session restore

On launch, if `session.json` records a `last_opened` AND the file
still exists AND `--scene` / project-path arg is not given:
- mount that workspace,
- restore open buffers, active view, cursor, and scroll_top from the
  session record.

On quit, write the session atomically (temp file + rename).

### Dirty-buffer gate on project mutations

Already in force: `apply_project_action` refuses when any buffer is
dirty. W040 surfaces this as a popover ("Save dirty buffers before
modifying project structure") rather than a silent no-op.

## Beads

### W040-B01 — Explorer tree navigation by keyboard

**Feature.**

- **Goal.** In Editing, `Tab` to Explorer, `Down` highlights the next
  module, `Enter` switches the active buffer to that module and cursor
  lands at line 1.
- **Design.** New selection state on Explorer (`selected_module_index`
  on `ShellRuntimeState`). Up/Down cycle the index. Enter dispatches
  `Msg::OpenModule(BufferId)`. Rendered with `>` marker on the
  highlighted row (same convention as palette / Welcome).
- **Tests.**
  - Unit contract: Up/Down cycles the index with wrap;
    `Msg::OpenModule` switches the active view; Enter on Explorer-focus
    dispatches `Msg::OpenModule(selected)`.
  - `wtd` journey: `tests/wtd/journey_explorer_navigate.rs` launches
    thin-slice, Tabs to Explorer, asserts selection visible on
    Module1, then Enter → captures the buffer title in the top bar.
- **Evidence.** Five-minute pass: navigate the full module list; any
  silent-no-op filed as follow-up bead.
- **Closure checklist as usual.**

### W040-B02 — Explorer mouse support

**Feature.**

- **Goal.** Left-clicking a module row in the Explorer switches the
  active buffer to that module. Scroll wheel scrolls the Explorer
  when it has more rows than fit.
- **Design.** `Msg::MouseEvent { button, row, col }` extended to
  dispatch Explorer-local actions based on (row, col) → module index
  lookup. Scroll wheel → Explorer `scroll_top` adjust.
- **Tests.**
  - Unit contract: mouse (row, col) → module index lookup correct
    for all visible rows; scroll up/down wraps clamped.
  - `wtd` journey: `tests/wtd/journey_explorer_mouse.rs` sends a mouse
    click at the Module1 row's pixel position and asserts buffer
    switch.
- **Evidence.** Five-minute pass with mouse: click every module,
  scroll, confirm no silent rows.

### W040-B03 — Real recent-projects MRU at `%APPDATA%/OxIde/session.json`

**Feature.**

- **Goal.** Welcome's Recent list is a real MRU: projects opened via
  `Ctrl+O` / `Ctrl+N` / `--scenario --scenario ... --open <path>` get
  appended; most recent first; duplicates deduped.
- **Design.** New `src/shell/session_store.rs`. Atomic file I/O
  (temp + rename) under `%APPDATA%/OxIde/`. Model calls
  `session_store::record_opened(path)` after each successful mount.
- **Tests.**
  - Unit contract: record/read round-trip; dedup; cap at N entries
    (32); atomicity (a corrupt write does not wipe the whole list).
  - `wtd` journey:
    `tests/wtd/journey_recent_persists_across_runs.rs` launches
    twice, mounts a project on first run, asserts it appears in
    Welcome Recent on the second run.
- **Evidence.** Five-minute pass: open two different projects, quit,
  relaunch — both appear in Recent, most recent first.

### W040-B04 — Session restore (buffers, cursors, layout)

**Feature.**

- **Goal.** On relaunch, OxIde opens the last-opened project, shows
  the same set of open buffers the user had, with the active view
  pointing at the same buffer and the cursor at the same
  `(line, column)`.
- **Design.** Extend `session.json` with `open_buffers`, `active_view`,
  and `cursor` fields per project. On successful mount, overlay the
  stored state onto the fresh `WorkspaceState`.
- **Tests.**
  - Unit contract: overlay round-trip given a known workspace +
    session record.
  - `wtd` journey:
    `tests/wtd/journey_session_restore_cursor.rs` moves cursor to a
    specific position, quits, relaunches, asserts top bar reads the
    same `Ln X Col Y`.
- **Evidence.** Five-minute pass: edit a module, move cursor to a
  distinct position, quit, relaunch, confirm exact restoration.

### W040-B05 — Project-action feedback popovers

**Feature.**

- **Goal.** `Ctrl+Shift+M` / `Ctrl+Shift+C` / `Ctrl+Shift+R` /
  `Ctrl+Shift+T` each produce a visible, honest acknowledgement: a
  brief popover shows what was just added / cycled; failures show a
  popover explaining the failure (permissions, parse error, etc.) and
  leave the project untouched.
- **Design.** Reuse existing hover-popover infra
  (`show_hover_popover`) with a distinct title line ("Added module
  `Module2`"). Auto-dismiss on any next keystroke; explicit `Esc`.
- **Tests.**
  - Unit contract: each project action installs the expected
    popover text on success; error path installs the error popover.
  - `wtd` journey:
    `tests/wtd/journey_add_module_surfaces_popover.rs` launches
    thin-slice, presses `Ctrl+Shift+M`, asserts popover visible with
    the added module's name.
- **Evidence.** Five-minute pass: trigger each action, observe
  popover, confirm the project state change on disk.

### W040-B06 — Dirty-buffer gate surfaces honestly

**Feature.**

- **Goal.** Pressing `Ctrl+Shift+M` / etc. while any buffer is dirty
  opens a feedback popover ("Save dirty buffers before modifying
  project structure") instead of being a silent no-op.
- **Design.** Existing `apply_project_action` dirty-gate path adds a
  popover install. Popover offers `Ctrl+S save all` as the escape
  hatch.
- **Tests.**
  - Unit contract: with any buffer dirty, project-action Msg
    produces the popover and does not mutate the project.
  - `wtd` journey:
    `tests/wtd/journey_project_action_dirty_gate.rs` types into
    Module1, presses `Ctrl+Shift+M`, asserts popover text + that
    `.basproj` content is unchanged on disk.

## Out-of-scope

- **Project creation wizard** — `Ctrl+N` already scaffolds a
  minimal project; a richer wizard (target kind picker, entry-point
  picker) is W110-material polish.
- **Arbitrary path input** — no file-path text input / picker in
  W040. The recent list + CLI arg + `Ctrl+N` cover the common flows.
  A text-input overlay is W090 (command system) territory.
- **Live on-disk change detection** — file watcher + reload prompt
  is W050 (document services).
- **Cross-project references** — project-to-project `Reference`
  resolution remains OxVba-owned; W040 renders what OxVba reports
  but does not implement reference resolution.
