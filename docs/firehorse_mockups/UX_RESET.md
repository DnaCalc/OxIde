# OxIde UX Reset: Fire Horse Terminal Direction

Status: current doctrine
Type: W039 reset note
Date: 2026-04-28

## Product Decision

The Fire Horse terminal UX is the current front-end direction for
OxIde.

It is a complete replacement of the previous OxIde front-end,
rendering, and visual UI direction. From W039 onward, contributors and
agents should not maintain a mixed old/new UI plan, should not extend
older visual mockups as current product truth, and should not preserve
older pane/frame choices by inertia.

The replacement is about product direction and authority. It does not
mean old files are deleted, and it does not mean every internal helper is
discarded. Useful framework code can be reused after review when it
serves the Fire Horse projection and terminal-cell renderer.

## Current UX Authority

Authoritative Fire Horse inputs are:

- `docs/DESIGN_TUI_2026_FIRE_HORSE.md`
- `docs/firehorse_mockups/HARDENING_REVIEW.md`
- `docs/firehorse_mockups/UX_RESET.md`
- `docs/firehorse_mockups/UX_PROJECTION_CONTRACT.md`
- `docs/worksets/W039_firehorse_terminal_ux_proof.md`
- the approved `docs/firehorse_mockups/refined_*.png` image set
- future W039 UX-lab terminal captures and goldens

Older UI documents and assets remain useful as history, but they are not
current authority when they conflict with the Fire Horse direction.

## Legacy Surface Inventory

### Historical UI Direction

These surfaces are historical design provenance:

- `docs/DESIGN_TUI.md` where it describes the earlier pane/frame
  direction;
- `docs/DESIGN_MOCKUP_WEB.md` and earlier web-presented mockup language;
- W010/W020/W030-era shell mockup references that describe proving or
  carrying forward the older mockup shape;
- older uxpass notes that reconciled `PRODUCT_DIRECTION.md` and
  `docs/DESIGN_TUI.md` before the Fire Horse reset.

They may explain how OxIde arrived here. They should not be used as the
current source for visual composition, scene geometry, palette, or
front-end priorities.

### Non-Authoritative Assets And Captures

Non-canonical mockup exports, exploratory screenshots, and old WTD
goldens can remain in the repository as test history or provenance.
They are non-authoritative unless a later W039 bead explicitly blesses
them as a current terminal-cell capture.

The approved visual image set is the refined Fire Horse set named in
`HARDENING_REVIEW.md`. Future terminal authority comes from W039
UX-lab captures, not from earlier mockup assets.

### Existing Renderer Paths

Existing OxIde shell rendering code is production reality, but not the
future visual contract. Treat it in two different ways:

- old pane/frame presentation choices are legacy UI;
- stable internal rendering infrastructure may be a reuse candidate.

Do not polish the old UI shape as a parallel product direction. If a
W039 or later bead reuses existing code, the bead should name the helper
or module and explain why it is infrastructure rather than legacy visual
design.

### Reusable Inner Framework Candidates

The following kinds of code may be candidates for reuse after explicit
review:

- terminal buffer, cell, color, and layout primitives;
- WTD launch, capture, and golden helpers;
- shell state read models that expose existing truth without owning new
  VBA meaning;
- small formatting helpers that are visual-mechanics code rather than
  old pane/frame policy.

Reuse criteria:

- keeps OxVba/OxIde ownership boundaries intact;
- serves the `FireHorseProjection -> terminal-cell render` path;
- does not preserve old pane/frame UX by inertia;
- is covered by tests or can be covered before reuse;
- has no hidden dependency on production command dispatch for lab-only
  mockups.

### Deletion Candidates

Some old files may later become deletion candidates if they confuse
contributors or preserve a mixed UX direction. No deletion is authorized
by this reset note.

Repo-local rules require the user to give the exact delete command in
the current session before any file or directory can be removed. Until
that happens, old assets and captures are retained and classified here as
historical or non-authoritative.

## Agent Rule

For W039 and later UX work:

- do not extend old front-end UI designs as current product direction;
- do not cite old mockups as current authority when Fire Horse doctrine
  exists;
- do not build a second mockup runner outside W038 Phase 1;
- reuse older internals only when the bead explicitly classifies them as
  framework and explains the tests or evidence protecting that reuse;
- keep VBA/project meaning tied to OxVba-owned or OxVba-shaped seams.

When uncertain, choose the Fire Horse projection contract and W039
UX-lab output over older pane/frame documentation.

## Transition Rule

W039 is the transition artifact.

It turns the Fire Horse visual direction into terminal-cell scenarios,
projection types, action ids, fixture data, WTD captures, and downstream
handoff notes. Until a later workset ships the product renderer
replacement, the existing `ox-ide` UI remains production code, but it is
not the product-design target for new UX work.
