# OxIde Working Method: Worksets and Beads

## 1. Purpose

This is the OxIde method doc. It defines two concepts — the **workset**
and the **bead** — and the complete discipline that flows through them.

Read this file before executing anything. The entire working method
lives here:

- what a bead is,
- what a workset is,
- how a bead closes,
- the pre-closure ritual ("five-minute user pass"),
- how to use `br` and `bv` to track live state.

Other docs are narrower:

- [`AGENTS.md`](/C:/Work/DnaCalc/OxIde/AGENTS.md) is repo safety only.
- [`PRODUCT_DIRECTION.md`](/C:/Work/DnaCalc/OxIde/PRODUCT_DIRECTION.md)
  and [`ARCHITECTURE.md`](/C:/Work/DnaCalc/OxIde/ARCHITECTURE.md) are
  authority docs (what OxIde is and how it's shaped).
- [`docs/WORKSET_REGISTER.md`](WORKSET_REGISTER.md) is the ordered
  list of worksets.
- [`docs/worksets/*.md`](worksets/) are per-workset design documents.
- [`docs/TESTING_WTD.md`](TESTING_WTD.md) is the mechanical `wtd`
  reference (how to drive the harness; no prescription).

If any doc contradicts this one on method, this one wins.

## 2. The Bead

A **bead** is the complete, atomic, accountable unit of work.

A bead is not a task card, not a TODO item, not a git commit.
It is the unit that carries the entire working method with it.
Every bead has five sections, and a bead without any of them is
malformed and cannot close.

### 2.1 Bead schema

**Goal** — one observable outcome, stated in user terms.

A goal is a *thing the user can see work* after the bead closes.
Not an internal refactor, not a "plumbing" item, not a vague theme.

- Good: "Pressing `Ctrl+N` from the Empty scene scaffolds a new
  `.basproj` under cwd and mounts it in the Editing scene with
  `Module1.bas` loaded."
- Good: "`F1` on a resolvable identifier in the Editor surfaces a
  hover popover with OxVba's label, detail, and provenance; `F1`
  again or `Esc` dismisses it."
- Bad: "Add new-project wiring." (not user-observable)
- Bad: "Polish the editor." (no concrete outcome)
- Bad: "Make hover better." (no contract)

A bead with a non-user-observable goal (pure internal scaffolding,
refactors) is acceptable only when explicitly labelled `doctrine-only`
or `infrastructure-only` — see §2.3.

**Design** — the concrete change.

Detail sufficient that a reviewer can check the approach without
reading the code. Covers:

- Files and modules touched.
- Contracts added, altered, or preserved (function signatures, enum
  variants, public types, configuration keys).
- State-shape implications (new fields, new enums, preserved
  invariants).
- UI-shape implications (new panels, new overlays, changed
  status-line tokens, new keymap entries).
- Existing utilities reused, with paths.

If the design requires a sketch (layout, flow, state transitions),
the bead includes it inline as an ASCII drawing or a short list.

**Tests** — two mandatory layers.

1. **Unit contract tests.** Assertions that pin behaviour *by name or
   contract*, never by list position or exact prose. Every assertion
   phrased in the language of the bead's goal.

   - Good: `palette_has_save_entry_and_it_dispatches_save_active_buffer()` —
     looks up the "Save" palette row by label, asserts its action is
     `PaletteAction::SaveActiveBuffer`.
   - Bad: `palette_row_2_is_save()` — breaks the day any row is
     inserted above Save.

   The tests fail when the goal breaks; they do not fail for adjacent
   cosmetic changes.

2. **Interactive `wtd` journey.** One scenario script under
   `tests/wtd/` that:

   - spawns the release binary with a known-good workspace,
   - drives the keystrokes a real user would press,
   - captures the resulting screen state,
   - asserts one or more contract properties of that state
     (regions present, text tokens present, overlay visible, cursor
     position in expected range).

   The `wtd` journey is what makes the goal real. If you can't
   write one, the goal is not user-observable, and the bead's goal
   is malformed.

Doctrine-only beads (pure documentation with no user-visible
behaviour change) may replace the `wtd` journey with a read-through
checklist documented in Evidence. This is the only exemption.

**Evidence** — what the author produces to claim closure.

- The `wtd` journey script committed under `tests/wtd/` and passing
  against a **freshly built release binary**. Not a cached binary.
  Not the debug build. The one the user would actually run.
- A **five-minute user pass** (§3) on the release binary. Any silent
  or surprising behaviour found during the pass is filed as a
  follow-up bead *before* this bead closes.
- A commit message describing **only** behaviours observed during the
  user pass. Any claim the author has not personally seen on the
  running binary is a process defect; rewrite the message.
- Updated truth surfaces (the workset spec, the register if scope
  changed, design docs if contracts changed, this method doc if the
  method itself changed).

**Closure checklist** — all of these must tick to close the bead.

- [ ] Goal achieved and user-observable on the release binary.
- [ ] Unit contract tests green.
- [ ] `wtd` journey green against the release binary.
- [ ] Five-minute user pass clean; follow-ups filed as new beads.
- [ ] Touched truth surfaces updated.
- [ ] Commit message cross-checked against observed behaviour.

No single item can be waived. A bead that closes with any item
unchecked is carrying hidden debt, which will surface as a "why
isn't this working" report later.

### 2.2 Worked example

Here is what a real bead looks like, written in the schema above.
This is not a template to copy verbatim; it is a concrete example
of the level of detail a bead carries.

```
### Bead B040-3 — Explorer Ctrl+click mounts a sibling project

Goal
  In the Empty scene, left-clicking a path in the Welcome Recent list
  opens that project in Editing. Cursor lands in the first module;
  top bar reads `<project> | Editing | Ln 1 Col 1`.

Design
  - Files touched:
    - src/shell/model.rs (Msg::MouseEvent handling for Welcome)
    - src/shell/mock_data.rs (launcher_editor_text — map row → path)
    - src/shell/state.rs (launcher_selection → mouse row delta)
  - Contracts:
    - new Msg::OpenRecentAt(usize)
    - new helper on ShellState: recent_project_at(index)
  - Reuses:
    - try_mount_workspace (src/shell/model.rs) — no change
  - UI:
    - Welcome Recent list rows are clickable at body_scene == Empty
    - mouse click on a row selects AND mounts (double-click not required)

Tests
  Unit contract:
    - shell::state::tests::recent_project_at_returns_selected_path
    - shell::model::tests::maps_mouse_click_on_recent_row_to_open_recent_at
  wtd journey:
    - tests/wtd/journey_empty_click_mounts_recent.rs
      — spawns oxide-empty, waits for Welcome, mouse-clicks row 0,
        asserts scene transitions to Editing within timeout.

Evidence
  - wtd journey passes against target/release/ox-ide.exe.
  - Five-minute pass notes:
    - Clicked each row in Recent — all mount ok.
    - Hover during click does not show a tooltip (intentional,
      no hover affordance on Welcome; no follow-up needed).
  - Commit message: "Mount recent project on Welcome-row click".

Closure
  - [x] Goal visible: click a row, land in Editing.
  - [x] 2 unit contract tests green.
  - [x] wtd journey green on release binary.
  - [x] Five-minute pass clean, zero follow-ups.
  - [x] W040 spec's mouse-contract note updated.
  - [x] Commit message matches observed behaviour.
```

### 2.3 Bead types

- **Feature bead.** User-observable goal. Full schema required.
- **Infrastructure bead.** No user-observable goal directly — affects
  subsequent feature beads. Example: "tests/wtd/ harness gains a
  `drive_scenario` helper". `wtd` journey may be a self-test of
  the harness; otherwise replaced by a doctrine-style read-through.
- **Doctrine bead.** Pure documentation. No `wtd` journey. Evidence
  is a read-through checklist of the touched docs.

A bead must declare its type at the top (`Feature`, `Infrastructure`,
`Doctrine`). A bead whose type is not clear is almost certainly two
beads.

## 3. The Five-Minute User Pass

Before any bead closes, the author runs this ritual on the release
binary. It is the cheapest check that catches the bugs unit tests
cannot see.

1. **Build the release binary.** `cargo build --release --bin ox-ide`.
   If the build fails or is blocked by a stale process, stop and
   resolve — the pass is invalid against a stale binary.

2. **Launch in a realistic cwd.** Usually the OxIde repo root or a
   scratch dir. Whatever surfaces the bead's goal.

3. **Scene-tour.** Walk every scene the bead touched.
   For each scene:
   - Read every token in the status-line row.
     For each token: press the keystroke. Either something visible
     happens, or something visible says *why* nothing happened.
     A silent no-op is a follow-up bead.
   - Open the palette (`F6`). Either Enter-dispatch every entry the
     bead touched (and any adjacent ones if time permits), or spot-
     check at least the bead's own entries.
   - Open and close any overlay the bead introduces. Confirm the
     backing scene is intact (not a mock leak). Confirm `Esc`
     cascades (popover first, overlay second).

4. **Cold-start tour if Empty was touched.** Launch from a dir with
   no `.basproj` files. Confirm Welcome shows honestly. Press every
   advertised Start-list affordance. Confirm each either dispatches
   or surfaces an honest feedback popover.

5. **Bug triage.** Every silent, surprising, or misleading result
   becomes a new bead filed *before* this bead closes. The bead
   can still close if its own goal works — but the observed noise
   must be captured.

This ritual takes roughly five minutes for most beads and catches
bugs unit tests architecturally cannot catch (terminal keymap
escapes, focus handler interactions, window sizing, wtd ↔ binary
mismatches, docs-vs-reality gaps).

## 4. The Workset

A **workset** partitions ambition. It groups beads that together
deliver one user capability end-to-end.

Worksets are **ambitious in scope**. A workset aims at a real user
capability ("a loaded project is navigable by meaning", "edits
persist and can be undone", "the debugger breakpoints and steps").
Not an incremental tweak. Worksets are the unit of product promise.

Worksets are **comprehensive in design**. A workset spec reads as a
design document: ambition, dependencies, the shape of the end
state, and the full bead list. No progress log, no "status"
preamble, no "Landed 2026-04-18". The bead list is concrete enough
that an executor starts with nothing ambiguous.

### 4.1 Workset template

Every workset spec at `docs/worksets/W<NNN>_<slug>.md` follows this
shape:

```
# Workset W<NNN> — <Ambition phrase>

## Ambition
One paragraph. The user capability this workset delivers
end-to-end. If you cannot state it in one sentence + one clarifying
sentence, the workset is too broad — split it.

## Dependencies
Upstream worksets and any named beads from other worksets that this
one rests on. Do not duplicate the entire upstream — cite it.

## Design
The shape of the end state. Enough prose + ASCII sketches + contract
examples that a reviewer can evaluate the approach without reading
code. This is where the heavy thinking lives.

Subsections as needed:
  - UX shape (scene / overlay / status-line contract)
  - Architecture (modules, state, contracts)
  - Interactions (per-bead sequence where ordering matters)
  - Open questions (if any — but try to resolve before shipping)

## Beads
Ordered list, one sub-section per bead, each using the schema from
§2.1 (Goal / Design / Tests / Evidence / Closure).

An executor should be able to read top-to-bottom and know exactly
what to build, in what order, with what tests, producing what
evidence.

## Out-of-scope
What this workset deliberately does NOT do, and pointers to the
future workset(s) that will.
```

No `Progress`. No `Status:`. No "Landed 2026-04-18". Progress lives
in git log and in `.beads/` closure records — not in the spec file.

### 4.2 Historical workset specs

Worksets that predate this method doc carry a historical banner at
the top, are not reshaped, and are treated as provenance records,
not current design. New work against a historical workset's scope
starts with a new workset spec in the current template.

## 5. Tool Split: `br` and `bv`

`.beads/` owns live execution state. Do not edit files under `.beads/`
directly. Use the tools.

`br` is the **mutation** tool:

```
br ready                                 # inspect ready work
br show <id>                             # inspect a bead
br create --title "..." --type task --priority 2
br update <id> --status in_progress
br close <id> --reason "Completed"
br dep add <issue> <depends-on>
```

`bv` is the **graph-aware inspection** tool. Read-only from an agent's
perspective:

```
bv ready
bv blockers
bv graph
```

Agent rule: prefer non-interactive calls. Do not use `.beads/` files
as an editable planning surface. Plans live in workset spec docs
(§4); live state lives in `.beads/` via `br` / `bv`.

## 6. Bad Beads

These patterns surface as bugs downstream. Reject or reshape before
execution.

- **Vague activity without a reviewable outcome.**
  "Polish the editor." — no goal. Split into specific user-
  observable goals.

- **Missing tests.**
  Any feature bead without both layers (unit contract + `wtd`
  journey) is malformed. Fix the bead before executing it.

- **Positional tests.**
  Assertions keyed on list position (`palette row 2 is Save`) that
  break on unrelated changes. Rewrite to look up by label/contract
  and assert action / outcome by name.

- **Leap beads.**
  One bead that actually covers four features ("line numbers +
  syntax highlighting + hover + goto-def"). Split. Each user-
  observable outcome is its own bead with its own `wtd` journey.

- **Aspirational evidence.**
  Evidence section that claims behaviour the author has not
  personally seen on the running binary. The author's own
  five-minute pass notes are the minimum bar.

- **Goal is actually a refactor.**
  "Clean up state.rs." — not user-observable. If the refactor has
  no observable surface, it is an Infrastructure bead and declares
  itself so; otherwise it's mis-labelled.

- **Premature closure.**
  Closing a bead because "enough progress happened". The closure
  checklist is a bar, not a suggestion.

## 7. Rollout Rule

A workset in flight rolls into one or more ready beads at a time.

Rollout pattern:

1. If the design in the workset spec is clear, expand the next
   bead(s) into `.beads/` via `br` and start work.
2. If the design still needs thought, file a **rollout bead**
   whose goal is to expand the next tier of beads into the spec.
   Rollout beads are Infrastructure beads: their evidence is the
   updated spec.
3. Keep the graph explicit. Every ready bead traces to a workset;
   every workset traces to the register; every register entry
   traces to an ambition the product direction authorises.

## 8. Relationship To Other Docs

- **[`AGENTS.md`](/C:/Work/DnaCalc/OxIde/AGENTS.md)** — repo safety
  only (file deletion, irreversible actions, public posting). Does
  not duplicate this method doc.
- **[`PRODUCT_DIRECTION.md`](/C:/Work/DnaCalc/OxIde/PRODUCT_DIRECTION.md)**
  — what OxIde is trying to become. Worksets serve it.
- **[`ARCHITECTURE.md`](/C:/Work/DnaCalc/OxIde/ARCHITECTURE.md)** —
  seams and ownership. Bead designs respect it.
- **[`docs/WORKSET_REGISTER.md`](WORKSET_REGISTER.md)** — ordered
  workset list. Keeps sequence truth.
- **[`docs/worksets/*.md`](worksets/)** — per-workset design
  documents in the §4.1 template.
- **[`docs/TESTING_WTD.md`](TESTING_WTD.md)** — how to drive
  `wtd`. Mechanical reference only; prescriptive content lives
  here in §2.1 (Tests layer) and §3 (Five-Minute User Pass).
- **[`docs/uxpass/*.md`](uxpass/)** — the W035 UX design pass
  deliverables (principles, journeys, frame, etc.). These are
  narrative design, not method.
- **`.beads/`** — live execution state. `br` / `bv` only.

If anything in this list drifts from the method defined here,
this file wins and the other is corrected.
