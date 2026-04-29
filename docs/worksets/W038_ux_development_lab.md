# Workset W038 — UX Development Lab

## Ambition

Reading and verifying a terminal scene is as cheap as reading a unit
test.

A contributor working on any UX bead can run a named scenario, render it
at a known terminal size, capture the exact VT output that `wtd` will
assert, replay or diff that capture locally, and then use an interactive
lab surface to inspect or bless goldens. The first slice exists to
unblock W039 Fire Horse proof work; the fuller generic lab follows after
W039 has proved the new UX direction.

This workset makes the five-minute user pass cheap. It turns the working
method in `docs/BEADS.md` Section 3 from a discipline into a repeatable
terminal workflow.

## Dependencies

- **W037** — the `wtd` harness crate, scenario YAMLs, and goldens
  directory must exist.
- **W039** — not a prerequisite for Phase 1. The later generic W038
  phases intentionally follow W039 so they are shaped by the Fire Horse
  proof rather than by the previous shell UI.
- Soft dependency on **W035 / Fire Horse doctrine** — W039 consumes the
  W038 Phase 1 substrate for the Fire Horse scenario suite.

## Design

### Phasing

W038 is split into four phases so W039 can move early without waiting
for the entire lab:

| Phase | Name | Purpose | Relationship to W039 |
| --- | --- | --- | --- |
| 1 | Minimum UX-lab substrate | Provide named lab scenarios, fixed viewport contracts, a single-scenario runner, and WTD capture plumbing. | Hard prerequisite for W039 fixture and renderer beads. |
| 2 | Shell state scenario loops | Add `ox-ide --scene` and `ox-ide --scenario` for generic shell work. | Starts after W039 proves the Fire Horse direction. |
| 3 | VT review tools | Add replay, diff, and bless commands for committed captures. | Starts after W039; uses real Fire Horse captures as reference cases. |
| 3a | Fire Horse high-end terminal review | Calibrate first-class and studio desktop targets against real terminal output, not flattened text goldens. | Starts after `ox-vt replay`; protects the high-end UX promise before downstream implementation. |
| 4 | Interactive lab | Build the three-pane `oxide-uxlab` workflow for browsing, comparing, and blessing scenarios. | Starts after W039 and the Phase 2/3 foundations. |

The sequencing rule is:

```text
W038 Phase 1 -> W039 -> W038 Phases 2-4
```

W039 may still perform doctrine work before Phase 1 closes, but W039's
fixture and terminal-rendering beads depend on Phase 1.

### Phase 1: minimum UX-lab substrate

Phase 1 is deliberately narrow. It should be just enough to let W039
register Fire Horse projections and verify their terminal captures.

It introduces a lab-only scenario contract:

```rust
pub struct LabScenarioDescriptor {
    pub id: &'static str,
    pub suite: &'static str,
    pub title: &'static str,
    pub purpose: &'static str,
    pub default_viewport: ViewportClass,
    pub tags: &'static [&'static str],
}

pub enum ViewportClass {
    Standard, // 120x34 unless WTD chooses a nearer supported size
    Compact,  // 92x30
    Wide,     // 160x40, optional for side-by-side review
}
```

The Phase 1 runner must support a command shape close to:

```text
cargo run --release --bin oxide-uxlab -- \
  --suite lab-smoke \
  --scenario lab-smoke-editing \
  --viewport standard \
  --once
```

`--once` is non-interactive: it renders one scenario and exits only when
the final frame is visible/captured by the harness. This is the path W039
uses for WTD journeys. The interactive browser is not part of Phase 1.

Phase 1 owns:

- a scenario registry that can accept future W039 providers;
- stable viewport names and dimensions;
- a tiny built-in smoke scenario that proves the runner without Fire
  Horse code;
- a WTD capture journey for the smoke scenario;
- docs that tell W039 how to register its Fire Horse suite.

Phase 1 does not own:

- Fire Horse projections or fixtures;
- command dispatch;
- `ShellModel::update` script replay;
- VT diff/bless tooling;
- a polished interactive lab.

### Phase 2: shell state scenario loops

After W039, W038 adds generic shell loops for contributors working on
shipping OxIde behavior:

```text
ox-ide --scene <id>
ox-ide --scenario <yaml>
```

`--scene` boots directly into named shell scenes without a real project
mount where possible. Initial scene IDs:

- `empty` — no project, Welcome surface.
- `editing` — thin-slice mounted, cursor at line 1.
- `build-success` — thin-slice plus post-F5 success posture.
- `build-failure` — thin-slice plus diagnostic posture.
- `palette` — Editing plus palette open.
- `hover-popover` — Editing plus semantic popover if the seam exists.
- `debug-suspended` — deferred until W080 exposes a real debug state.

`--scenario` replays a deterministic script of `Msg` values through
`ShellModel::update` without keyboard event translation. This is a
middle-loop tool for model behavior, not the Fire Horse projection proof.

### Phase 3: VT review tools

The review tool is split into small commands so each behavior is testable
and useful on its own:

```text
ox-vt replay <file.vt>
ox-vt diff <a.vt> <b.vt> [--strict]
ox-vt bless <capture> --golden <path>
```

Policy:

- `replay` writes the captured terminal stream to stdout/current
  terminal without interpreting it as product truth.
- `diff` defaults to text-first region/line changes and offers strict
  cell diff for color or glyph regressions.
- `bless` is explicit about source capture and target golden. It must
  refuse ambiguous names and print the exact path it updates.

### Phase 3a: Fire Horse high-end terminal review

W039 proved that Fire Horse surfaces can exist in terminal cells. Phase
3a asks the stronger product question: do the high-end terminal targets
feel close to the colourful Fire Horse mockups, with desktop density,
hierarchy, and emotional force intact?

This phase explicitly prioritizes the expected desktop usage profile.
Compact, fallback, and hostile terminal support remain requirements, but
they must not drag the first-class and studio targets down into a
minimum-viable layout. The best OxIde experience should assume a modern
desktop terminal with truecolor, Unicode line drawing, enough width, and
low-latency redraw.

Review viewport classes:

| Class | Suggested cells | Review meaning |
| --- | --- | --- |
| Compact | 92x30 | Works at all; intentional Focus Mode. |
| Standard | 120x34 | Usable baseline, not the product ceiling. |
| First-class | 144x40 or 160x42 | Primary desktop review target. |
| Studio | 190x48+ | Premium near-GUI terminal IDE target. |

The visual review pack should compare:

- approved colourful mockup PNG;
- replayed or live terminal output from the FrankenTui mockup path;
- plain text golden for contract review;
- short notes on what was kept, what was lost, and what needs redesign.

Editing Lens, Command Lens, Run Lane, and Debug Cockpit are the first
review scenes. Launchpad, Console Fit, and Compact Focus follow after
the core working surfaces feel right.

The W039 text renderer remains useful as a contract surface, but it is
not sufficient UX evidence. Phase 3a therefore adds a separate
`oxide-uxlab --mockup` path that draws Fire Horse scenes with real
FrankenTui widgets and can emit either plain cell text or an ANSI
terminal stream with `--ansi`.

### Phase 4: interactive lab

The final W038 lab is a separate FrankenTui binary with three panes:

- **Scenario list** — enumerates lab providers, `.wtd/*.yaml`, and
  built-in shell scenes.
- **Live render** — runs the selected scenario at the chosen viewport.
- **Golden/diff** — shows committed golden, latest capture, or diff.

Hotkeys:

| Key | Behavior |
| --- | --- |
| `Enter` | Run selected scenario. |
| `c` | Capture current render. |
| `g` | Toggle golden view. |
| `d` | Toggle diff mode. |
| `b` | Bless an explicitly selected capture. |
| `r` | Reset scenario. |
| `1` / `2` / `3` | Switch Standard / Compact / Wide viewport. |
| `q` | Quit. |

Why separate binary: production `ox-ide` stays clean. The lab may grow
multiple providers, capture history, and side-by-side review without
polluting the shipping shell.

## Beads

### W038-B00 — Lab phasing and W039 substrate contract

**Doctrine.**

- **Goal.** A contributor opens this workset and knows exactly which
  W038 beads unblock W039, which beads wait until after W039, and how
  the lab avoids preserving old UI assumptions.
- **Design.** Refactor the W038 spec into phased delivery. Add the
  Phase 1 contract, viewport names, runner command shape, and handoff
  rule to W039. Update W039 to cite Phase 1 as the hard infrastructure
  dependency for fixture/render beads.
- **Tests.** Doctrine bead. No unit tests.
- **Evidence.** Read-through checklist:
  - Phase 1 is visibly smaller than the full W038 ambition;
  - W039 depends on Phase 1, not all of W038;
  - later W038 phases follow W039;
  - no deletion or legacy UI maintenance is implied.
- **Closure.**
  - [ ] W038 spec has explicit phases.
  - [ ] W039 spec cites W038 Phase 1 infrastructure.
  - [ ] Live bead graph matches the phased sequencing.

### W038-B01 — Lab scenario registry and viewport contract

**Infrastructure.**

- **Goal.** Lab code can list named scenarios from registered providers,
  including a built-in smoke provider, and every scenario declares a
  stable viewport class.
- **Design.** Add a lab-only module, expected to live near
  `src/shell/uxlab/`, with:
  - `LabScenarioDescriptor`;
  - `ViewportClass`;
  - `LabScenarioProvider`;
  - a registry builder that combines providers without depending on
    production command dispatch.
  The built-in smoke provider registers `lab-smoke-editing` with
  `suite = "lab-smoke"` and `ViewportClass::Standard`.
- **Tests.**
  - Unit contract: the smoke provider appears in the registry by id.
  - Unit contract: duplicate scenario ids fail with a named error.
  - Unit contract: `Standard` and `Compact` resolve to fixed WTD sizes.
- **Evidence.**
  - A developer can run a list command or unit helper and see the smoke
    scenario with suite, id, title, tags, and viewport.
- **Closure.**
  - [ ] Registry structs exist in lab-only code.
  - [ ] Smoke scenario registers successfully.
  - [ ] Duplicate ids are rejected.
  - [ ] Viewport sizes are documented and tested.

### W038-B02 — Non-interactive `oxide-uxlab --once` runner

**Infrastructure.**

- **Goal.** `oxide-uxlab --suite lab-smoke --scenario lab-smoke-editing
  --viewport standard --once` renders one registered scenario at the
  requested viewport without launching the interactive browser.
- **Design.** Add `src/bin/oxide-uxlab.rs` with a minimal CLI parser and
  a single-scenario render path. The runner asks the registry for the
  scenario, creates its lab projection/renderable object, paints one
  FrankenTui frame, then stays compatible with WTD capture timing before
  exit. Unknown suites/scenarios print available ids.
- **Tests.**
  - Unit contract: CLI selection resolves the smoke scenario by suite
    and id.
  - Unit contract: unknown scenario errors include valid alternatives.
  - WTD journey: `tests/wtd/journey_uxlab_once_smoke.rs` launches the
    release `oxide-uxlab` smoke scenario and asserts the smoke title,
    viewport label, and key rail are visible.
- **Evidence.**
  - Five-minute pass: run the smoke scenario at Standard and Compact;
    both produce a stable terminal frame.
- **Closure.**
  - [ ] Binary builds.
  - [ ] `--once` path works for the smoke scenario.
  - [ ] Unknown scenario output is helpful.
  - [ ] WTD journey green against the release binary.

### W038-B03 — WTD capture contract for lab scenarios

**Infrastructure.**

- **Goal.** WTD can capture a lab scenario through `oxide-uxlab --once`
  using a named viewport, and future W039 journeys can reuse the same
  helper without custom harness code.
- **Design.** Add a WTD helper under `tests/support/`:
  `LabScenarioJourney`, `Harness::open_lab_once`,
  `Harness::capture_lab_once_text`, and
  `Harness::capture_lab_once_vt`. The helper launches a lab scenario
  with suite/id/viewport through a `.wtd/` workspace and captures the
  final frame. Document the helper in `docs/TESTING_WTD.md`. Commit a
  smoke golden under a W038 path so the capture/golden directory
  convention is concrete.
- **Tests.**
  - Unit or integration contract: helper builds the expected command
    line for `oxide-uxlab --once`.
  - WTD journey: smoke capture compares against its golden or asserts
    stable visible tokens if golden comparison is not yet available.
- **Evidence.**
  - W039 can cite the helper name and command shape for Fire Horse
    journeys.
  - Five-minute pass: deliberately run the smoke WTD journey after a
    release build and replay the captured frame in the terminal if the
    harness supports it.
- **Closure.**
  - [ ] Shared WTD lab helper exists.
  - [ ] Smoke capture path is documented.
  - [ ] W038 smoke journey green.
  - [ ] W039 fixture/render beads can depend on this bead.

### W038-B04 — `ox-ide --scene` scenario loader

**Feature.**

- **Goal.** `cargo run --release -- --scene empty` boots OxIde directly
  into the Empty scene with no project required; `--scene editing`
  boots into Editing with thin-slice mounted; `--scene palette` boots
  into Editing and opens the palette.
- **Design.** New `src/shell/scenarios.rs` module defining a scenario
  enum with the initial shell set: Empty, Editing, BuildSuccess,
  BuildFailure, Palette, HoverPopover. `main.rs` gains a `--scene <id>`
  flag parsed ahead of any project path. The loader calls
  `ShellModel::new` and applies a sequence of model messages or state
  constructors to reach the target state. Scene IDs are kebab-case;
  unknown IDs print the list and exit non-zero.
- **Tests.**
  - Unit contract: each scenario produces expected shell scene, focus,
    and optional overlay.
  - WTD journey: `tests/wtd/journey_scene_flag.rs` launches
    `ox-ide --scene palette` and asserts the palette is open.
- **Evidence.**
  - Five-minute user pass: launch each initial scene id and verify the
    visible state without project setup.
- **Closure.**
  - [ ] `--scene <id>` flag parsed in `main.rs`.
  - [ ] Scenario enum covers the initial set.
  - [ ] Per-scenario unit tests green.
  - [ ] WTD journey green.
  - [ ] Five-minute pass clean.

### W038-B05 — `ox-ide --scenario` Msg-script replayer

**Feature.**

- **Goal.** `cargo run --release -- --scenario .wtd/journey_palette_save.yaml`
  replays listed `Msg` values through `ShellModel::update` and exits
  with the final render visible. No keyboard events are involved.
- **Design.** Define a YAML schema with a `script:` array of explicit
  message variants. Loader at `src/shell/scenario_replay.rs`
  deserializes via serde and feeds messages through the model. Unknown
  message variants fail at parse time with a listed valid set and the
  offending line where available.
- **Tests.**
  - Unit contract: representative YAML parses.
  - Unit contract: scripted message sequence reaches expected state.
  - Unit contract: unknown variant failure names valid variants.
  - WTD journey: scripted scenario captures the same visible result as
    the equivalent keystroke scenario.
- **Evidence.**
  - Round-trip: one `wtd` key sequence and equivalent `--scenario`
    script produce matching visible contracts.
- **Closure.**
  - [ ] YAML schema documented in this spec and `docs/TESTING_WTD.md`.
  - [ ] Parser failures cite offending input.
  - [ ] WTD journey green.
  - [ ] Five-minute pass clean.

### W038-B06 — `ox-vt replay`

**Feature.**

- **Goal.** `ox-vt replay <file.vt>` pipes a committed VT snapshot to
  the current terminal so a developer sees what a WTD test captured.
- **Design.** New binary `src/bin/ox-vt.rs` with a `replay`
  subcommand. It validates file existence, streams bytes without
  product interpretation, and refuses directories or non-files with a
  clear error. It should work with W037 and W039 capture paths.
- **Tests.**
  - Unit contract: replay rejects missing and directory paths.
  - Unit contract: replay streams a fixture byte-for-byte to a test
    writer.
  - WTD journey optional: capture replay output and verify expected
    tokens remain visible.
- **Evidence.**
  - Five-minute pass: replay one W037 golden and one W039 Fire Horse
    golden after W039 exists.
- **Closure.**
  - [ ] Binary builds.
  - [ ] Replay streams fixture bytes exactly.
  - [ ] Errors are clear for missing/invalid paths.

### W038-B07 — `ox-vt diff`

**Feature.**

- **Goal.** `ox-vt diff <a.vt> <b.vt>` prints a readable terminal
  capture diff; `--strict` reports cell-level differences when text
  projection is insufficient.
- **Design.** Add text-first diff over normalized line projections.
  Changed, added, and removed lines are grouped with line numbers and
  enough context to review a layout regression. Strict mode compares
  cell/glyph/color tuples when the renderer exposes them; if strict
  mode cannot be supported yet, it fails honestly with a follow-up bead.
- **Tests.**
  - Unit contract: added, removed, and changed lines are reported by
    category.
  - Unit contract: identical fixtures produce "no differences".
  - Unit contract: strict mode either reports a known color difference
    or returns the documented unsupported error.
- **Evidence.**
  - Five-minute pass: alter a copied capture manually outside the repo
    working tree and verify diff reports the expected visible change.
- **Closure.**
  - [ ] Diff subcommand builds.
  - [ ] Text-first diff covers changed/added/removed lines.
  - [ ] Strict behavior is implemented or explicitly deferred.

### W038-B08 — `ox-vt bless`

**Feature.**

- **Goal.** `ox-vt bless <capture> --golden <path>` promotes an
  explicitly selected capture to an explicitly selected golden path,
  refusing ambiguous or missing paths.
- **Design.** Add `bless` to `ox-vt`. It must print source and target
  paths before writing, require an explicit `--golden`, and preserve the
  repo deletion rule by never removing old files. If a golden exists,
  the command overwrites only that exact user-provided target path and
  records enough console output for a reviewer to see what changed.
- **Tests.**
  - Unit contract: missing `--golden` fails.
  - Unit contract: source path must exist and be a file.
  - Unit contract: target path is updated with source bytes in a temp
    fixture directory.
- **Evidence.**
  - Five-minute pass: bless a temp fixture target, then run `ox-vt diff`
    to verify no differences remain.
- **Closure.**
  - [ ] Bless requires explicit source and target.
  - [ ] No deletion is performed.
  - [ ] Temp fixture bless path is tested.

### W038-B12 — Fire Horse high-end viewport doctrine

**Doctrine.**

- **Goal.** A contributor can read the W038/W039 handoff and know that
  First-class and Studio are the primary desktop-quality Fire Horse
  review targets, while Compact and fallback support are degradation
  targets rather than the design ceiling.
- **Design.** Add the high-end viewport policy, review meaning, and
  scene priority list to W038. Cross-reference W039 outputs and the
  hardening review. Make explicit that terminal-cell proof at 120x34 is
  not sufficient evidence for the promised near-GUI/high-performance TUI
  experience.
- **Tests.** Doctrine bead. No unit tests.
- **Evidence.** Read-through checklist:
  - First-class and Studio are named as desktop review targets;
  - Compact and fallback remain requirements but not product ceiling;
  - W039 colourful mockups and terminal captures are both cited;
  - the next visual review scenes are named.
- **Closure.**
  - [ ] High-end viewport policy is documented.
  - [ ] Review targets are tied to Fire Horse mockups and W039 captures.
  - [ ] `ox-vt replay` is named as a prerequisite.

### W038-B13 — Fire Horse first-class/studio review pack

**Infrastructure.**

- **Goal.** Reviewers can inspect Editing Lens, Command Lens, Run Lane,
  and Debug Cockpit at First-class and Studio desktop targets using
  side-by-side artifacts: colourful mockup, terminal replay/render,
  text contract, and concise review notes.
- **Design.** Produce a review directory under `docs/firehorse_mockups/`
  or another documented path. For each scene, include the approved
  `refined_*.png`, the replayed/rendered terminal capture, the paired
  `.txt` golden link, and a note with `Kept`, `Lost`, and
  `Needs redesign` sections. Use `ox-vt replay` for committed `.vt`
  captures and live `oxide-uxlab --once` output where a larger viewport
  render is needed.
- **Tests.** Doctrine/infrastructure bead. No unit tests unless a helper
  script is added.
- **Evidence.** Five-minute review pass: open the four review notes and
  replay at least one W039 `.vt` capture in a real terminal.
- **Closure.**
  - [ ] Four core scenes have review notes.
  - [ ] First-class and Studio targets are represented.
  - [ ] Review artifacts compare against the colourful mockups, not only
        text goldens.

### W038-B14 — Fire Horse high-end density and layout refinement

**Feature (UX-lab renderer).**

- **Goal.** The Fire Horse UX-lab renderer feels materially closer to
  the colourful mockups at First-class and Studio sizes: denser useful
  content, clearer rail hierarchy, stronger source-centered composition,
  and less table-like flattening.
- **Design.** Tune only the UX-lab Fire Horse renderer and fixture
  viewport behavior. Candidate changes: richer wide Project Spine,
  less wasted vertical blank space, stronger Code Canvas/lens hierarchy,
  Activity Deck timeline density, Context Dock card rhythm, and
  first-class/studio-specific layout rules. Do not switch the shipping
  renderer or invent real project/semantic/run/debug behavior.
- **Tests.**
  - Unit contract: First-class/Studio viewport layout choices are
    deterministic.
  - WTD or capture evidence: refreshed W039-style captures for changed
    scenarios.
- **Evidence.** Review pass compares refined terminal output against
  the W038-B13 notes and records which losses were recovered.
- **Closure.**
  - [ ] Fire Horse renderer has explicit high-end layout behavior.
  - [ ] Updated captures/goldens are reviewed.
  - [ ] No product renderer path changes.

### W038-B15 — Fire Horse real FrankenTui mockup renderer

**Feature (UX-lab renderer).**

- **Goal.** Reviewers can run every Fire Horse scenario through
  `oxide-uxlab --mockup` and see a real FrankenTui terminal mockup,
  with Block/Flex/Paragraph layout, first-class/studio density, and an
  optional ANSI stream suitable for live terminal review.
- **Design.** Keep the existing W039 string renderer as the stable text
  contract path for goldens. Add a separate mockup render mode on the
  W038 lab provider contract:
  - `--mockup` selects the FrankenTui mockup renderer;
  - `--ansi` emits the styled terminal stream from FrankenTui's
    presenter;
  - without `--ansi`, `--mockup` emits plain cell text for review
    artifacts and simple diffs.
  The Fire Horse provider owns the mockup implementation and covers
  Launchpad, Editing Lens, Command Lens, Run Lane, Debug Cockpit,
  Console Fit, Compact Focus, and the real Editing adapter. No
  production renderer path changes.
- **Tests.**
  - Unit contract: `--mockup` produces different output from the W039
    contract renderer and includes FrankenTui block chrome.
  - Unit contract: `--mockup --ansi` emits terminal escape sequences
    and Fire Horse identity text.
  - Unit contract: every Fire Horse scenario renders through the
    mockup path with review surfaces visible.
- **Evidence.**
  - Fresh release binary can render at least Editing Lens Studio with
    `oxide-uxlab --mockup`.
  - Review captures under
    `docs/firehorse_mockups/frankentui_terminal_review/` include text
    and ANSI artifacts for the complete Fire Horse scenario set.
- **Closure.**
  - [ ] `--mockup` and `--mockup --ansi` work through `oxide-uxlab`.
  - [ ] All Fire Horse scenarios render through the mockup path.
  - [ ] Review artifacts cover the complete scenario set.
  - [ ] Existing W039 text-contract path remains intact.
  - [ ] No production renderer path changes.

### W038-B09 — Interactive `oxide-uxlab` scenario browser

**Feature.**

- **Goal.** `cargo run --release --bin oxide-uxlab` opens a FrankenTui
  three-pane TUI where a contributor can browse scenarios, run one, and
  inspect its live render at the selected viewport.
- **Design.** Extend the Phase 1 binary with interactive mode. Left pane
  lists providers and scenario ids. Middle pane renders the selected
  scenario. Right pane initially shows scenario metadata and later hosts
  golden/diff. Keyboard-only first version: `Enter`, `r`, viewport keys,
  and `q`.
- **Tests.**
  - Unit contract: scenario list includes registered providers and marks
    load errors as rows, not silent omissions.
  - WTD journey: launch interactive lab, move selection, press Enter,
    and assert middle pane renders the selected scenario.
- **Evidence.**
  - Five-minute pass: run W039 Fire Horse scenarios from the browser
    and switch Standard/Compact viewports.
- **Closure.**
  - [ ] Interactive binary opens.
  - [ ] Three-pane layout renders at 120x40.
  - [ ] Selection and run keys work.
  - [ ] WTD journey green.

### W038-B10 — Interactive golden and diff pane

**Feature.**

- **Goal.** From `oxide-uxlab`, a contributor can toggle golden view,
  latest capture, and diff for the selected scenario without leaving the
  terminal.
- **Design.** Wire the right pane to the Phase 3 `ox-vt` diff/replay
  logic as library code where practical. Hotkeys: `c` capture, `g`
  golden, `d` diff, `b` bless. Bless must require an explicit selected
  capture/golden pair and print a confirmation row before writing.
- **Tests.**
  - Unit contract: pane state transitions among metadata, golden,
    capture, and diff.
  - Unit contract: bless is disabled without explicit source/target.
  - WTD journey: open lab on a scenario with a golden, toggle diff, and
    assert diff/golden labels are visible.
- **Evidence.**
  - Five-minute pass: navigate to a W039 golden, capture, diff, and
    bless a temp target.
- **Closure.**
  - [ ] Capture/golden/diff pane states work.
  - [ ] Bless guardrails are visible.
  - [ ] WTD journey green.

### W038-B11 — Lab documentation and downstream verification recipe

**Doctrine.**

- **Goal.** Contributors working on later UX worksets can read one
  concise recipe and know when to use `--scene`, `--scenario`,
  `oxide-uxlab --once`, interactive `oxide-uxlab`, and `ox-vt`.
- **Design.** Update `docs/TESTING_WTD.md` and any relevant workset
  references with:
  - command examples;
  - viewport policy;
  - capture/golden directory policy;
  - Fire Horse scenario examples from W039;
  - five-minute pass integration.
- **Tests.** Doctrine bead. No unit tests.
- **Evidence.** Read-through checklist:
  - every W038 command has a copy/pasteable example;
  - W039 Fire Horse scenarios are cited as real examples;
  - deletion and blessing guardrails are documented;
  - downstream worksets can cite this recipe.
- **Closure.**
  - [ ] `docs/TESTING_WTD.md` covers the lab workflow.
  - [ ] W039 examples are linked.
  - [ ] Guardrails are explicit.

## Out-of-scope

- **Shipping renderer replacement.** W038 provides lab tooling. W039 and
  later product worksets own the Fire Horse product renderer.
- **Mouse support in `oxide-uxlab`.** The initial TUI is keyboard-only.
- **Remote capture playback.** `ox-vt replay` targets the local
  terminal only.
- **Golden-review UI on GitHub.** CI artifact upload remains outside
  this workset.
- **Deleting older captures or mockups.** No W038 bead deletes files.
  Repo-local deletion rules still require an exact user command.
