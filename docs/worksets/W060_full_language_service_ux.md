# Workset W060 — Full Language-Service UX

## Ambition

OxIde exposes the full `HostWorkspaceSession` surface as first-class
UX. A VBA author navigates by meaning (goto-definition, find
references, symbols picker), reads by meaning (hover popover, inline
diagnostics with squiggles, gutter marks), writes by meaning
(completion popup, signature help, rename), and moves between
projects' semantics without thinking about which subsystem is
answering.

At the end of W060 the language-service capability in OxIde is at or
above the VBA IDE baseline and the only missing pieces are those OxVba
itself has not yet shipped.

## Dependencies

- **W050** — real editor widget + selection model + find-in-buffer.
  Completion and rename hook into the selection + textarea infra.
- **W030 / W037** — host session + `wtd` harness (exist).
- Soft dep on **W035 §30** (scene catalogue) for Inspector sub-pane
  contract under Semantic scene.
- **W039** — Fire Horse Editing Lens proof and projection contract.
  It supplies the source-lens, Context Dock, Activity Deck, and explicit
  unavailable-seam shape; W060 owns the real language-service data.

### W039 Fire Horse Input

W039 proved how diagnostics, source-adjacent lenses, semantic context
cards, references, and key hints should sit around the Code Canvas. The
proof does not implement language-service behavior. W060 replaces
fixture or unavailable projections with real
`HostWorkspaceSession::diagnostics`, hover, goto-definition,
references, symbols, completions, signature help, and rename data while
preserving the W039 source-centered composition.

## Design

### What already landed (W060 pre-landings during W035 / W050 work)

- `F1` hover popover backed by `HostWorkspaceSession::hover`.
- `F12` goto-definition with same-buffer / cross-buffer navigation.
- Inspector's `Diagnostics` and `Symbols` sub-panes read from real
  OxVba output with J2-c intrinsic-type-screen and J3-b generated-
  document tagging.
- Hover popover anchored at cursor, opaque, auto-dismissed on cursor
  move (J4-a treatment).

W060 extends this foundation.

### New surfaces

**Inline diagnostics (squiggles).** Each diagnostic's span renders
with a squiggle underline in the editor body. Severity colour: red
(error), yellow (warning), blue (info), green (hint). Gutter marks
echo the severity (`!` for error, `?` for warning) so the user sees a
scrollable file's error presence.

**Completion popup.** Triggered on `Ctrl+Space` (always) and
opt-in-as-you-type (off by default; a uxpass decision). Popup lists
completion items from `HostWorkspaceSession::completions`. Rendering
is a bordered block below the cursor (flips above if it overflows),
with item detail rendered on the right half when the popup is wide
enough.

**Signature help.** Inside a call-expression's parens, a small
non-intrusive popover shows the procedure's signature with the
current argument highlighted. Triggered automatically by `(` and
comma typing.

**Find references.** `Shift+F12` lists references for the symbol at
the cursor into the Lower Surface (`LowerSurfaceMode::References`)
with file + line + snippet per entry. Enter on an entry navigates.

**Workspace symbols picker.** `Ctrl+T` opens an overlay similar to
the palette listing every `document_symbol` across the project.
Fuzzy filter. Enter navigates.

**Rename.** `F2` on a symbol opens a single-row inline input strip
over the symbol's location. Enter commits the rename through
`HostWorkspaceSession::prepare_rename` + `rename`. Esc cancels.

### Updated Inspector

Semantic scene's Inspector carries:
- `Hover` — current cursor's hover info (same as popover body).
- `Symbols` — project-wide symbols filtered to the active file.
- `References` — reference count for the symbol at cursor (click to
  open the full references list in Lower Surface).

### Squiggle degradation

In 16-colour terminals, squiggles render as coloured underlines
(no wave). Under `--caps monochrome`, squiggles render as `~~~~` on
a line below the source (W100 degradation budget).

## Beads

### W060-B01 — Inline diagnostics (squiggle underlines + gutter marks)

**Feature.**

- **Goal.** A VBA author sees squiggles under every span reported by
  `HostWorkspaceSession::diagnostics` in the Editor; the gutter shows
  a severity mark on any line carrying a diagnostic.
- **Design.** Editor render layer takes the current diagnostics +
  per-line span list; paints cell backgrounds (or underlines, where
  the terminal supports SGR) over the reported ranges; paints gutter
  marks in the muted gutter column.
- **Tests.** Unit: render a known buffer + diagnostic; assert the
  expected cells carry the squiggle style. `wtd` journey: induce a
  diagnostic in thin-slice, assert squiggle presence in the capture.
- **Evidence.** Five-minute pass with a deliberate compile error:
  squiggle visible, gutter mark present; fixing the error clears
  both.

### W060-B02 — Completion popup (`Ctrl+Space`)

**Feature.**

- **Goal.** `Ctrl+Space` anywhere in the editor opens a bordered
  completion popup below the cursor with `HostWorkspaceSession::completions`
  results; `Up` / `Down` select; `Enter` / `Tab` commit;
  `Esc` closes.
- **Design.** New `CompletionPopoverState` with item list + selection
  index. `Msg::ToggleCompletion`, `Msg::MoveCompletionSelection`,
  `Msg::CommitCompletion`. Render similar to hover popover but with
  row selection and per-row items.
- **Tests.** Unit: popover installs, Up/Down cycles, commit inserts
  the selected text. `wtd` journey: press Ctrl+Space on a known
  cursor position and assert the popover contains an expected
  completion.

### W060-B03 — Signature help

**Feature.**

- **Goal.** Typing `(` after a procedure name surfaces a small
  non-intrusive popover with the procedure's signature; the current
  argument is highlighted; typing `,` advances; typing `)` dismisses.
- **Design.** Driven by `HostWorkspaceSession::signature_help`
  (verify this API exists; if not, file a dependency bead). Popover
  renders above or below the cursor. Non-modal; does not steal
  keystrokes.
- **Tests.** Unit: trigger / advance / dismiss. `wtd` journey: type
  `Main(` and assert the signature popover appears.

### W060-B04 — Find references in the Lower Surface

**Feature.**

- **Goal.** `Shift+F12` on a symbol populates
  `LowerSurfaceMode::References` with every call / use site; Enter
  on a row navigates to that location (cross-buffer if needed).
- **Design.** Reuse existing `find_references` call. Lower Surface
  cycle already includes `References`.
- **Tests.** Unit: references list materialises for a known symbol.
  `wtd` journey: Shift+F12 on `Main`, assert `References` mode
  visible with the expected row count, press Enter, assert
  navigation.

### W060-B05 — Workspace symbols picker (`Ctrl+T`)

**Feature.**

- **Goal.** `Ctrl+T` opens an overlay listing every
  `workspace_symbol`; typing filters; Enter navigates.
- **Design.** Overlay based on the palette infra with a different
  data source. Fuzzy filter (prefix + substring ranking).
- **Tests.** Unit: filter narrows correctly. `wtd` journey: Ctrl+T
  → type partial symbol → Enter → assert navigation to the expected
  location.

### W060-B06 — Rename (`F2`) with prepare-rename gate

**Feature.**

- **Goal.** `F2` on a symbol surfaces an inline rename input over the
  symbol; committing dispatches
  `HostWorkspaceSession::prepare_rename` +
  `rename`; every affected file is rewritten on disk and the
  buffers refresh.
- **Design.** Small overlay anchored at the symbol location.
  Validation against `prepare_rename`'s reported range before accepting
  input.
- **Tests.** Unit: valid rename applies; invalid position rejects
  with a popover. `wtd` journey: rename `answer` in thin-slice;
  assert all usages updated on disk.

## Out-of-scope

- **Code actions / quick fixes** — W060 reads diagnostics; acting on
  them is W110 (polish) territory.
- **Multi-cursor editing** — not a W060 concern.
- **Fold / outline regions** — the Inspector's `Symbols` list
  provides the outline; a dedicated fold gutter is later polish.
- **Inlay hints** — deferred.
- **Semantic colouring** — the hand-written VBA lexer is W050 /
  highlight.rs; semantic re-colouring is a later improvement.
