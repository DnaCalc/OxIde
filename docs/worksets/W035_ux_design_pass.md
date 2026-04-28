# Workset W035 — UX Design Pass

## Ambition

OxIde's UX is re-derived from first principles against the W030 foundation,
reconciled back into the authoritative product and design docs, and
backed by a set of observable user journeys with live `wtd` evidence.

At the end of this workset, a new contributor reading
`PRODUCT_DIRECTION.md` + `docs/DESIGN_TUI.md` sees a shell whose shape,
bindings, and naming match what the running binary actually does — and
every decision in the uxpass doc suite (D1…D18+) is either in force in
code or traceable to the bead that lands it.

## Dependencies

- **W030** — service integration (direct OxVba host embedding). Without
  it there is no real shell to observe.
- **W037** — WinTermDriver harness. Every bead's `wtd` journey depends
  on the harness being present.

## Design

### The uxpass doc suite

The UX design pass produces seven markdown files under
`docs/uxpass/`:

1. `00_principles.md` — tie-breakers (P1…P10). Grounds every later
   decision. *In force.*
2. `10_user_journeys.md` — J1…J4 at minimum, keystroke by keystroke,
   with committed `wtd` captures. *In force.*
3. `20_frame_and_regions.md` — retires the "five-region frame" in
   favour of a four-band vertical frame with a scene-scoped body
   decomposition; D9…D18. *In force.*
4. `30_scene_catalogue.md` — per-scene contract: Inspector sub-panes,
   Lower Surface mode order, status-line tokens, transitions. *Pending.*
5. `40_command_model.md` — unified action registry, chord / mnemonic
   rules, keymap profiles (default + VBA-IDE-compatible). *Pending.*
6. `50_visual_language.md` — palette, density, border language,
   degradation policy. *Pending.*
7. `60_reconciliation.md` — diff vs `PRODUCT_DIRECTION.md` and
   `docs/DESIGN_TUI.md`; lists exactly what changes in those two docs.
   **Exit gate.** *Draft landed.*

### Decisions landed in force

D1…D18 are annotated inline in the uxpass docs (00_principles.md
decisions section, 20_frame_and_regions.md decisions section). New
decisions made in the pending docs continue the numbering.

### The reconciliation exit

The pass is complete when:
- All seven uxpass docs exist and are internally consistent.
- `PRODUCT_DIRECTION.md` and `docs/DESIGN_TUI.md` have been edited to
  match the uxpass output (the retired "five-region frame" language is
  gone; the frame / scene / command / visual statements match
  decisions D9…D18+).
- Every landed decision has a `wtd` journey or pinning test cited
  from its uxpass annotation.

### Evidence already on disk

- `docs/uxpass/captures/cold_start/*.txt` — cold-start journey
  evidence.
- `docs/uxpass/captures/thin_slice/*.txt` — thin-slice journey
  evidence.
- `tests/wtd/goldens/W037/{empty,thin_slice_loaded}.{txt,vt}` —
  baseline goldens that the uxpass decisions re-bless.

Three thin-slice captures (`05_after_typo.txt`, `06_after_f5_run.txt`,
`07_palette_open.txt`) are visually stale for three specific differences
landed in recent code pushes (dirty marker, user-slots jargon strip,
opaque palette overlay) and are scheduled for recapture as one of the
beads below.

## Beads

### W035-B01 — Draft `30_scene_catalogue.md`

**Feature.**

- **Goal.** A contributor opens `docs/uxpass/30_scene_catalogue.md` and
  reads a complete per-scene specification: for each of Empty, Editing,
  Semantic, BuildRun, Palette, ComReference, Debug — the Inspector
  sub-panes in order, the Lower Surface mode in default and its
  transitions, the status-line token set (already in force via D3),
  and the explicit transitions in/out.
- **Design.** New file under `docs/uxpass/`. Section per scene. Each
  scene carries an ASCII frame sketch, a bullet list of Inspector
  sub-panes with their contract, a Lower Surface mode-cycle list, a
  status-line token table, and a transitions table. Cites D9…D18.
  Resolves the three open questions forwarded from
  `20_frame_and_regions.md` (Inspector sub-panes, Lower Surface modes,
  Debug shape).
- **Tests.** Unit contract: none (doctrine bead). The doc is proven by
  its citations — every claim points at a test or capture already in
  the repo.
- **Evidence.** Read-through checklist:
  - Each scene cites at least one `wtd` capture or golden that shows
    the shape.
  - Transitions are exhaustive (no scene lacks an entry path and an
    exit path).
  - Open questions from `20_frame_and_regions.md` are explicitly
    closed with decision numbers (D19, D20, …).
- **Closure.**
  - [ ] File exists and is internally consistent.
  - [ ] Cited captures exist under `docs/uxpass/captures/` or
    `tests/wtd/goldens/`.
  - [ ] `00_principles.md` "Open questions" list updated to note
    which questions this doc closed.

### W035-B02 — Draft `40_command_model.md`

**Feature.**

- **Goal.** A contributor opens `docs/uxpass/40_command_model.md` and
  finds the unified action registry, the palette's role as the
  canonical command-discovery surface, chord / mnemonic rules, and the
  default + VBA-IDE-compatible keymap profiles with the exact
  keystroke ↔ action mapping for every listed affordance.
- **Design.** Sections: action registry shape; palette as discovery +
  dispatch; chord state machine (for `Ctrl+K Ctrl+O` style sequences);
  mnemonic sequences (`Alt+I, M` style); two keymap profiles
  side-by-side; mouse → action overlay.
- **Tests.** Unit contract: none in this doc (doctrine bead). The
  contract lands in W090 when the code is written.
- **Evidence.** Read-through checklist:
  - Every binding listed in any uxpass or workset doc is present in at
    least one profile here.
  - Every palette entry the shell currently ships has an action name.
  - The chord and mnemonic sections cite what W090 must implement.
- **Closure.**
  - [ ] File exists.
  - [ ] Cross-check: every binding the current shell advertises is in
    the default profile table.

### W035-B03 — Draft `50_visual_language.md`

**Feature.**

- **Goal.** A contributor opens `docs/uxpass/50_visual_language.md` and
  finds the canonical palette (truecolor + 16-colour fallback),
  typography conventions, density rules, border language, motion and
  animation policy, and the degradation policy under weak terminals.
- **Design.** Palette tables referencing `src/shell/theme.rs` as the
  source of RGB truth (values live there; the doc cites them). Border
  language (which regions use which style), density budget (what
  counts as chrome), motion policy (what animates, what must not),
  degradation rules (what the shell does at 16-colour, no-mouse,
  no-truecolor).
- **Tests.** Unit contract: none (doctrine bead).
- **Evidence.** Read-through checklist:
  - Palette table matches `src/shell/theme.rs` byte-for-byte.
  - Degradation rules name the `ftui_core::capabilities` hooks W100
    will call.

### W035-B04 — Re-capture stale `thin_slice` evidence

**Feature.**

- **Goal.** The three `docs/uxpass/captures/thin_slice/*.txt` files
  that are visually stale (`05_after_typo.txt`,
  `06_after_f5_run.txt`, `07_palette_open.txt`) are re-blessed against
  the current release binary so the uxpass narrative docs cite
  accurate captures.
- **Design.** Interactive `wtd` pass: launch `oxide-smoke` against
  thin-slice, drive each journey (typo → capture; F5 → capture; F6 →
  capture), write the captures.
- **Tests.** Unit contract: none. The captures themselves are the
  evidence.
- **Evidence.**
  - Each re-blessed capture contains the expected landed behaviour
    (dirty marker visible, no `user slots` jargon, palette title
    singular with no editor bleed-through).
  - Five-minute user pass on the release binary during capture run
    surfaces no new silent affordances.
- **Closure.**
  - [ ] 3 files re-blessed.
  - [ ] Each `10_user_journeys.md` citation of these files rendered
    readable against the new content.

### W035-B05 — Write `60_reconciliation.md`

**Feature.**

- **Goal.** A contributor opens `docs/uxpass/60_reconciliation.md` and
  sees a concrete diff: what changes in `PRODUCT_DIRECTION.md` and
  `docs/DESIGN_TUI.md` to align with the uxpass output, what stays,
  and what gets amended. Lists every "five-region frame" occurrence
  that must retire, every frame/scene/command/visual statement that
  must update, and every new section that must appear.
- **Design.** Single file. Section per target doc
  (`PRODUCT_DIRECTION.md` then `docs/DESIGN_TUI.md`). Each section
  lists concrete edits: "Replace passage X with Y"; "Delete passage
  Z"; "Add new section W citing decision D<N>".
- **Tests.** Unit contract: none.
- **Evidence.** Read-through checklist:
  - Every decision D1…D<latest> either appears in the diff or has an
    explicit "no change required" note.
  - A `rg "five-region" <target docs>` expected-diff is listed.

### W035-B06 — Apply reconciliation to `PRODUCT_DIRECTION.md` and `docs/DESIGN_TUI.md`

**Feature.** (Doctrine bead.)

- **Goal.** `PRODUCT_DIRECTION.md` and `docs/DESIGN_TUI.md` match the
  uxpass output. The retired phrases are gone; the new decisions are
  reflected.
- **Design.** Apply every edit listed in `60_reconciliation.md`
  mechanically. No new reasoning — the reasoning lives in the
  reconciliation doc.
- **Tests.** Unit contract: none (doctrine bead).
- **Evidence.**
  - `rg "five-region" PRODUCT_DIRECTION.md docs/DESIGN_TUI.md` returns
    zero hits.
  - Spot-check: each decision D1…D<latest> is either in force in the
    target doc or explicitly cited in the reconciliation's "no change"
    list.
- **Closure.**
  - [ ] Both target docs updated.
  - [ ] `rg "five-region"` clean across the repo (except inside
    `20_frame_and_regions.md` which discusses the retirement).
  - [ ] Workset W035 can close.

## Out-of-scope

- **Code changes to implement pending decisions.** Decisions already
  in force in code are documented; decisions still to land in code
  (split-editor panes per D17; `--dev-scenes` probe page per W100) are
  captured in later worksets. W035 is the *design* pass, not the
  implementation.
- **Retroactive softening of "landed 2026-04-17" prose in the uxpass
  narrative docs.** Handled by the follow-on cleanup/backfill pass
  (W045), not by the core W035 design-authoring beads.
- **Follow-on UX worksets.** `W038` (UX development lab) owns the
  tooling that makes UX work cheap; `W090` owns the command system
  implementation; `W100` owns capability probing; `W110` owns polish
  and recovery. W035 hands them concrete design inputs and stops.
