# 20 - Frame and Regions

Status: `draft`
Workset: `W035`

## Purpose
Revisit the canonical frame. Confirm - or replace - the region set, fix
each region's role on a principle-by-principle basis, and pin the shape
the shell takes across scenes and terminal widths. This is the doc that
later uxpass work (scenes, commands, visuals) cites when it assumes
"the Explorer column is X cells wide" or "the Lower Surface is Y rows
at Wide".

Grounded in the re-blessed W037 goldens and the `captures/` evidence
listed below, not in the pre-W030 mockups under `docs/DesignMockup/`.

## Captures This Document Cites

| File | Scene | Terminal | What it pins |
| ---- | ----- | -------- | ------------ |
| [../../tests/wtd/goldens/W037/empty.txt](../../tests/wtd/goldens/W037/empty.txt) | Empty | 120x40 | Single-panel Welcome body; permanent top bar + status line. |
| [../../tests/wtd/goldens/W037/thin_slice_loaded.txt](../../tests/wtd/goldens/W037/thin_slice_loaded.txt) | Editing | 120x40 | Three-column body; Lower Surface Problems below; permanent status line. |
| [captures/cold_start/00_welcome.txt](captures/cold_start/00_welcome.txt) | Empty | 120x40 | Narrative evidence for Empty shape. |
| [captures/thin_slice/00_loaded.txt](captures/thin_slice/00_loaded.txt) | Editing | 120x40 | Narrative evidence for Editing shape. |
| [captures/thin_slice/06_after_f5_run.txt](captures/thin_slice/06_after_f5_run.txt) | BuildRun | 120x40 | BuildRun shape: same three columns, Lower Surface becomes Output. |
| [captures/thin_slice/07_palette_open.txt](captures/thin_slice/07_palette_open.txt) | Palette overlay | 120x40 | Overlay geometry and permanent status-line coexistence. |

## Observations That Shaped These Decisions

These are the conditions on the ground against which this doc is being
written. They are not grievances; they are the starting point, and they
are all drawn from the files cited above.

1. **Vertical layout is already four bands.** The shell today renders
   `[TopBar (3 rows) - Body (flex) - LowerSurface (optional 7..11 rows) - StatusLine (1 row)]`.
   The TopBar and StatusLine are always present; Body fills; LowerSurface
   is scene-scoped. Established by `src/shell/view.rs::split_root` and
   pinned by the re-blessed goldens (the top bar occupies rows 1-3, the
   status line occupies row 40, on every golden).
2. **Body shape is already scene-scoped.** Empty collapses the body to
   a single Welcome panel (D1b). Editing / Semantic / BuildRun use three
   columns `[Explorer | Editor | Inspector]`. Narrow width collapses to
   two columns `[Explorer | Editor]`. There is no width class where
   Editing has fewer than two columns.
3. **Column widths are already scene-aware, not just width-aware.**
   `ShellLayoutPolicy::derive` gives Empty a narrower Explorer (16-18%)
   and a wider Editor (56-58%) than Editing (20% / 58-62%) - but D1b
   makes Empty single-panel, so the Empty column percentages are
   vestigial. `ShellScene::BuildRun` already claims a taller Lower
   Surface (10-11 rows) than Editing or Semantic (7-10 rows).
4. **Overlays paint over body + lower surface but leave the status line
   visible.** `captures/thin_slice/07_palette_open.txt` shows the Palette
   overlay spanning rows 9-32, covering Explorer / Editor / Inspector
   but leaving the LowerSurface border at rows 33-39 and the status
   line at row 40 untouched. The overlay rect is computed by
   `centered_rect` over `frame.width() x frame.height()`, not over the
   body rect - so the overlay *can* mask the lower surface at narrow
   widths, but never masks the top bar or status line.
5. **The top bar is in the focus ring but has no user-actionable
   content.** `ShellState::available_focus_regions` returns
   `[TopBar, ...]` for every scene, which means `Tab` cycles through
   `TopBar` even though the top bar only paints `<project> | <scene> | ...`.
   This was surfaced as open question #4 in `10_user_journeys.md`
   ("Is the `Focus Top` region reachable by Tab a useful surface, or an
   artifact of treating the top bar as a region?").
6. **The five-region name is already misleading.** The pre-W030 docs
   talk about "the five-region frame" counting `TopBar`, `Explorer`,
   `Editor`, `Inspector`, `LowerSurface`. With D3 the status line is a
   sixth region. With D1b the Empty scene has only two regions (TopBar
   and Editor-as-Welcome, plus the status line). The count varies; the
   name does not.
7. **Narrow width is the only degradation axis that changes region
   set.** At `WidthClass::Narrow` (`< 120 cols`) the Inspector is
   collapsed, so the body has `[Explorer(20%) | Editor(80%)]`. The
   palette and COM-reference overlays are the exceptions: they force
   `inspector_collapsed = false` so the overlay's backing body keeps
   its wide shape. No existing width class changes the vertical band
   set.
8. **Immediate panel lives as a LowerSurface mode, not a region.**
   The plan's open question #2 ("Does the Immediate panel live in the
   lower surface, or as a first-class region?") is currently answered
   implicitly: `LowerSurfaceMode` in `src/shell/state.rs` is already a
   cycle through `Problems / Output / Log / Immediate / References`.
   No Immediate region is drawn today.

## Region Catalogue

Every region below has a canonical user-visible label, a fixed role,
and a constraint it must honor. The name "region" is internal; the
label column is what the user sees.

| Region (internal) | User-visible label | Presence | Role |
| ----------------- | ------------------ | -------- | ---- |
| `TopBar`          | none (project identity sits in the band)                | always | Identity and one relevant state value for the current scene. |
| `Explorer`        | `Explorer` | scene-scoped (non-Empty) | Project tree: modules, references, open buffers. |
| `Editor`          | active buffer title (e.g. `Module1.bas | Primary View`) on non-Empty; `Welcome` on Empty | always | Primary reading / typing surface. On Empty it is the launcher. |
| `Inspector`       | `Inspector <Mode>` (e.g. `Inspector Diagnostics`) | scene-scoped (non-Empty), width-scoped (collapsed at Narrow unless an overlay forces it open) | Contextual sub-panes for the active scene. |
| `LowerSurface`    | `Lower Surface <Mode>` (e.g. `Lower Surface Problems`) | scene-scoped (non-Empty) | Ephemeral task surface: problems, output, log, immediate, references. |
| `StatusLine`      | none (the band itself is the affordance) | always | Single row; announces the keystrokes that are live right now. |
| `Overlay`         | overlay title (e.g. `Palette`, `COM Reference`) | modal | Command discovery and focused one-off workflows. |

The TopBar band is three rows (bordered block). The StatusLine band is
one row (no border, muted style). Everything else wraps with
`WrapMode::WordChar` except the Editor, which stays unwrapped so source
column positions survive (D7).

## Width Classes

The thresholds are set in `WidthClass::from_width`:

| Class | Column range | Body shape (non-Empty) | Notes |
| ----- | ------------ | ---------------------- | ----- |
| Wide | `>= 160` | `[Explorer 16-20% | Editor 56-58% | Inspector fill]` | Used for laptop landscape / external monitor. |
| Standard | `120 - 159` | `[Explorer 18% | Editor 57-62% | Inspector fill]` | Default laptop / typical console. |
| Narrow | `< 120` | `[Explorer 20% | Editor 80%]` (Inspector collapsed) | Minimum supported; still usable. |

All three classes keep the same vertical band set. Only the body
decomposes differently.

## Per-Scene Frame Shape

Each scene names its frame below. The status line hint row is always
present; it is documented in D3 (`00_principles.md`) and not repeated
here.

### Empty (no project)

```
+- Top Bar ----------------------------------+
| <project> | Empty                          |
+- Welcome (full width) ---------------------+
| OxIde                                      |
| A terminal-native IDE for OxVba.           |
| Recent                                     |
| > <recent project>                         |
| Start                                      |
|   Open Project                             |
|   ...                                      |
+--------------------------------------------+
| Ctrl+O open project  Up/Down select recent |
+--------------------------------------------+
```

- No Explorer, no Inspector, no Lower Surface (D1b / P5).
- Focus ring is `[TopBar, Editor]`; see D16 for why the TopBar entry
  will be removed.
- Evidence: `W037/empty.txt`, `captures/cold_start/00_welcome.txt`.

### Editing / Semantic (project loaded, editing source)

```
+- Top Bar ----------------------------------+
| <project> | Editing | Ln X Col Y           |
+- Explorer -+- Editor ---------+- Inspector +
|  tree      |  source          |  Diag/Sym  |
|  ...       |  ...             |  ...       |
+------------+------------------+------------+
+- Lower Surface Problems -------------------+
|  ...                                       |
+--------------------------------------------+
| F5 run  F6 palette  Ctrl+Tab next view ... |
+--------------------------------------------+
```

- Three-column body at Wide / Standard; two-column (no Inspector) at
  Narrow.
- Semantic differs from Editing only in Inspector / LowerSurface mode;
  the frame is identical. Top-bar scene label differs
  (`Editing` vs `Semantic`) but P10 says the internal enum name is not
  a user affordance - see `30_scene_catalogue.md` for whether we keep
  two names or one.
- Evidence: `W037/thin_slice_loaded.txt`, `captures/thin_slice/00_loaded.txt`.

### BuildRun (after F5)

```
+- Top Bar ----------------------------------+
| <project> | Run | passing / completed      |
+- Explorer -+- Editor ---------+- Inspector +
|  tree      |  source          |  RunStatus |
|            |                  |  + Target  |
+------------+------------------+------------+
+- Lower Surface Output ---------------------+
|  [workspace] ...                           |
|  [diagnostic] ...                          |
|  [stdout] ...                              |
+--------------------------------------------+
| F5 rerun  F6 palette  Tab next focus ...   |
+--------------------------------------------+
```

- Same three-column body shape as Editing / Semantic. The Editor
  column deliberately stays visible during Run so the user can read
  the source that is producing the output.
- Lower Surface is 10-11 rows (Wide) vs 7-8 rows (Standard / Narrow) -
  Run needs to show more log lines than Editing's Problems view.
- Evidence: `captures/thin_slice/06_after_f5_run.txt`.

### Overlay (Palette, COM Reference)

```
+- Top Bar ----------------------------------+
| <project> | Palette                        |
+- Explorer -+- Editor ---------+- Inspector +
|      +-- Overlay (centered) ------+        |
|      | Command Palette            |        |
|      | Filter                     |        |
|      |  > ...                     |        |
|      | Commands                   |        |
|      |   Open Project    Ctrl+O   |        |
|      |   ...                      |        |
|      +----------------------------+        |
+------------+------------------+------------+
+- Lower Surface Problems -------------------+
|  ... (still visible around the overlay)    |
+--------------------------------------------+
| Esc close  Up/Down select  Enter apply     |
+--------------------------------------------+
```

- Overlay rect is centered over `frame.width() x frame.height()` and
  sized as a percentage of the frame (see `centered_rect` in
  `src/shell/view.rs`).
- The overlay can mask part of the Lower Surface at narrow widths but
  must not mask the top bar or the status line - the top bar carries
  the scene label and the status line carries the overlay-specific
  hint (`Esc close  Up/Down select  Enter apply`).
- Underlying cells are cleared with `Cell::default()` before the
  overlay Block renders (J4-a / P1), so editor glyphs cannot bleed
  through.
- Evidence: `captures/thin_slice/07_palette_open.txt`.

## Decisions

Numbered and imperative. Continues the uxpass-wide numbering from
`00_principles.md` (which ends at D8).

9. **The canonical vertical frame is four bands:**
   `[TopBar(3 rows, bordered) - Body(Fill) - LowerSurface(optional, 7..11 rows, bordered) - StatusLine(1 row, no border)]`.
   TopBar and StatusLine are always present on every scene, every
   width class. Body fills; LowerSurface is scene-scoped (absent on
   Empty, present on Editing / Semantic / BuildRun). The pre-W030
   phrase "five-region frame" is retired in favor of *"four-band
   vertical frame with a scene-scoped body decomposition"*; later
   docs cite regions by role, not by count. Already in force via
   `src/shell/view.rs::split_root`; confirmed by re-blessed W037
   goldens.

10. **Body decomposition is `scene x width_class`, not width alone.**
    Empty's body is a single full-width panel (D1b). Non-Empty bodies
    are three columns `[Explorer | Editor | Inspector]` at Wide /
    Standard and two columns `[Explorer | Editor]` at Narrow. Column
    percentages are selected by `(scene, width_class)` so BuildRun can
    give Editor a slightly wider share than Editing without mutating
    the region set. Encoded in `ShellLayoutPolicy::derive`; this
    decision documents the contract so later scenes can cite it
    rather than re-deriving.

11. **Width-class thresholds are `>= 160` (Wide), `120..=159`
    (Standard), `< 120` (Narrow).** The shell supports Narrow to the
    point of "readable and keyboard-operable with Inspector hidden";
    anything below the smallest supported terminal the user will see
    a capability message, not a silently corrupt frame (cross-refs
    P8; actual capability-message copy lives in
    `50_visual_language.md`). The class names themselves are internal
    (D4); the shell renders no "Wide / Standard / Narrow" badge.

12. **At Narrow width the Inspector collapses, period.** The Lower
    Surface does not collapse at Narrow because the scenes that
    surface errors (Editing, BuildRun) need a reading surface wider
    than the Editor column. If Narrow-width Editing proves unreadable
    in later observation, the corrective action is to make the
    Inspector accessible by `F9` (toggle show/hide) rather than to
    drop the Lower Surface.

13. **On Empty the body is a single full-width Welcome panel; no
    Explorer, Inspector, or Lower Surface is rendered.** Already
    landed as D1b; restated here because it is a frame contract, not
    just an Empty-scene detail. A later scene that *needs* an
    Empty-like "nothing loaded" shape (e.g. project-load failure)
    uses this same body shape.

14. **Overlays float centered over `Body + LowerSurface` but never
    over `TopBar` or `StatusLine`.** The top bar always shows the
    scene label (so the user can see that the overlay has not lost
    them), and the status line always shows the overlay-specific
    keystroke hint (so the user can see how to dismiss it). Overlay
    cells under the overlay rect are cleared before the Block paints
    (J4-a). This is the definition of "modal" in OxIde: scoped to
    body/lower, not to the frame.

15. **The Lower Surface is the canonical ephemeral task surface.**
    Immediate, Problems, Output, Log, References all live here as
    `LowerSurfaceMode` variants. The Immediate panel does not get a
    first-class region. The plan's open question #2 ("Does the
    Immediate panel live in the lower surface, or as a first-class
    region?") is answered here as *lower surface*. Rationale: P5
    (match shape to task) - Immediate is an ephemeral, task-scoped
    surface; promoting it to a permanent region would cost vertical
    real estate on every scene where it is not being used. Scope of
    its keymap and invocation lives in `40_command_model.md`.

16. **The Top Bar is display-only; it is not a focus target.**
    `ShellState::available_focus_regions` drops `TopBar` from the
    ring on every scene; `Tab` cycles through user-actionable
    regions only (`Editor` on Empty; `Explorer / Editor / Inspector /
    LowerSurface` on non-Empty, skipping any region that is absent
    or collapsed in the current layout). `Alt+1..4` already
    addresses the actionable regions by index, which remains the
    direct-focus model. Answers the plan's open question #4. _In
    force in code_ — pinned by
    `shell::state::tests::{top_bar_is_not_in_focus_ring_on_any_non_overlay_scene, top_bar_focus_request_is_rejected_on_every_non_overlay_scene, empty_scene_focus_ring_is_editor_only}`.

17. **Split-editor panes are deferred past W050.** The Editor column
    is one logical surface at the frame level. *Within* the Editor
    column, multiple `ViewId`s already exist (`Primary` / `Secondary`),
    cycled by `Ctrl+Tab`, but they share the same rectangle and
    alternate rather than tile. W060 or W050 can revisit the *inside*
    of the Editor rectangle; the outside (the body decomposition) is
    frozen at three columns / two columns per D10. Answers the plan's
    open question #1.

18. **Region labels are user-facing nouns; internal enum names are
    not.** `Explorer`, `Editor`, `Inspector`, `Lower Surface`,
    `Palette`, `COM Reference` are the canonical strings. `TopBar`,
    `FocusRegion::Editor`, `ShellScene::Semantic`,
    `LowerSurfaceMode::Problems` are internal identifiers. Every
    user-visible title comes through `active_title()` in
    `src/shell/view.rs`; every internal identifier stays behind the
    seam (reinforces P1 / P10).

## Reconciliation Notes (forwarded to 60_reconciliation.md)

- `PRODUCT_DIRECTION.md` and `docs/DESIGN_TUI.md` both use the phrase
  "five-region frame". D9 retires that wording; the reconciliation doc
  should rewrite those passages as "four-band vertical frame with a
  scene-scoped body decomposition" and cite D9.
- `docs/DESIGN_TUI.md` lists the region names as canonical. D18
  confirms that list as user-visible, but also renames `LowerSurface`
  to `Lower Surface` (two words) for the user label - the single-word
  form is internal.
- D15 closes a prior ambiguity in `PRODUCT_DIRECTION.md` about the
  Immediate panel. The reconciliation should reference D15 explicitly.
- D16 changes observable behavior (Tab ring) and needs a regression
  test when it lands in code.
- D17 explicitly defers a question `DESIGN_TUI.md` hinted at; the
  reconciliation should cite D17 next to any remaining "split view"
  language.

## Open Questions (forwarded to later docs)

- What `Inspector` sub-panes exist for each scene, and in what order?
  (`30_scene_catalogue.md`)
- What `Lower Surface` modes exist, what cycles between them, and
  which scenes auto-switch modes? (`30_scene_catalogue.md`)
- How does Debug reshape the body - a new scene with a different body
  decomposition (e.g. Editor / Callstack / Locals) or an Editing-body
  with different Inspector / Lower Surface modes? (`30_scene_catalogue.md`)
- Where does a palette-triggered go-to-file / go-to-symbol surface
  render - the current Overlay rect, or a dedicated input strip under
  the TopBar? (`40_command_model.md`)
- What does the Narrow-width Inspector-collapsed affordance look like
  when the user presses `F9` to re-open it? A temporary overlay, or a
  reshape that pushes Editor narrower? (`30_scene_catalogue.md` +
  `50_visual_language.md`)
- When `columns < 80`, which band is sacrificed first - Explorer or
  LowerSurface? D12 says LowerSurface stays; a future doc has to
  decide the hard lower bound. (`50_visual_language.md`,
  tied to P8.)
