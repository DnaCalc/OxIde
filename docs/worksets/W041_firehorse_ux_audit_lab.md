# Workset W041 - Fire Horse UX Audit Lab

## Ambition

Build a terminal-native UX Audit Lab inside `oxide-uxlab` so a reviewer
can evaluate the Fire Horse IDE direction against personas, scenario
journeys, visual mockups, terminal captures, state ownership, action ids,
and OxVba seam mappings from one working TUI surface. The workset turns
the current mockup/capture collection into an auditable review cockpit
that can produce evidence for downstream implementation beads without
switching the production `ox-ide` renderer.

The same lab is also an agentic automation surface. A coding agent must
be able to ask the lab what UX design work is needed, render candidate
states, evaluate them against functional and aesthetic criteria, export
machine-readable evidence, and decide whether a downstream implementation
bead is ready, blocked, or needs redesign. The interactive cockpit is one
view over the same audit contracts; it is not the only consumer.

The primary target is the high-end desktop terminal experience:
First-class and Studio viewports should feel close to the colourful Fire
Horse mockups. Compact and fallback terminal postures are review
dimensions, not excuses to flatten the desktop experience.

## Dependencies

- `W035` UX design pass:
  - `docs/uxpass/00_principles.md`
  - `docs/uxpass/10_user_journeys.md`
  - `docs/uxpass/20_frame_and_regions.md`
  - `docs/uxpass/60_reconciliation.md`
- `W038` UX development lab:
  - Phase 1 scenario registry, viewport contract, and
    `oxide-uxlab --once` runner.
  - `W038-B15` FrankenTui Fire Horse mockup renderer, including
    `--mockup` and `--mockup --ansi`.
  - `W038-B09` interactive scenario browser as shared browser substrate
    when it is available. W041 may build its first audit shell directly
    on the same underlying uxlab modules if B09 is not complete yet, but
    it must not create a second renderer.
- `W039` Fire Horse terminal UX proof:
  - `docs/firehorse_mockups/UX_RESET.md`
  - `docs/firehorse_mockups/UX_PROJECTION_CONTRACT.md`
  - `docs/firehorse_mockups/HARDENING_REVIEW.md`
  - Fire Horse scenario ids, projection types, fixture suite, action id
    matrix, and OxVba seam mapping.
- `W090` command system direction, for action id naming only. W041 does
  not implement `ActionRegistry`.
- `W100` terminal capability direction, for degradation review only.
  W041 does not implement terminal probing.

## Design

### Product Shape

The Audit Lab is not another set of screenshots and not a marketing
viewer. It is a working terminal review instrument. The reviewer should
be able to answer six questions for any Fire Horse scene:

1. Which persona and job is this scene serving?
2. Which scenario step is visible, and what outcome should the user get?
3. Which surfaces, commands, and state tokens must be visible?
4. Which projection fields and internal owners feed those surfaces?
5. Which OxVba seam or named future seam owns every VBA/project fact?
6. Is the high-end terminal result strong enough to implement, or does
   the design need correction before downstream beads consume it?
7. What design change should an agent attempt next, and which contracts
   must it preserve?
8. Which objective checks and structured aesthetic judgements support
   the review result?

The lab opens directly into the review surface. It should not show a
landing page or ask the reviewer to read docs before doing useful work.

### Agentic Automation Shape

The Audit Lab must be useful to a coding agent without a human sitting in
the TUI. It therefore exposes the same review state as deterministic,
machine-readable contracts:

```text
discover       -> list suites, personas, scenarios, viewports, criteria
brief          -> produce a design work packet for one scenario/view
render         -> produce contract text, mockup text, ANSI stream, and metadata
evaluate       -> produce functional and aesthetic scorecards
compare        -> compare baseline and candidate artifacts
export         -> write a reproducible local evidence pack
handoff        -> emit downstream gate state and follow-up bead hints
```

Automation entry points must be stable enough for agents and WTD tests:

```text
oxide-uxlab --audit --suite firehorse --list --json
oxide-uxlab --audit --suite firehorse --matrix --json
oxide-uxlab --audit --suite firehorse --scenario <id> --viewport studio --brief --json
oxide-uxlab --audit --suite firehorse --scenario <id> --viewport studio --once --json
oxide-uxlab --audit --suite firehorse --scenario <id> --viewport studio --evaluate functional,aesthetic --json
oxide-uxlab --audit --suite firehorse --batch docs/firehorse_mockups/ux_audit_lab/agent_run.json --json
```

The JSON output is not a debug dump. It is a versioned schema with:

- stable ids for suites, personas, scenarios, viewports, criteria,
  findings, artifacts, actions, and seams;
- absolute or repo-relative artifact paths;
- reproduction commands;
- exit codes suitable for automation;
- evidence links for every pass, concern, fail, or deferred result;
- no ANSI escape data embedded in JSON; ANSI streams are referenced as
  artifact files.

An agent design loop should be possible with no hidden context:

1. Run `--list --json` and select a ready audit scenario.
2. Run `--brief --json` to get persona, journey, visual target,
   must-preserve contracts, likely downstream owner, and current artifact
   refs.
3. Modify the lab renderer, fixture, or downstream implementation slice.
4. Run `--once --json` and `--evaluate functional,aesthetic --json`.
5. Inspect scorecard failures, reproduce captures locally, and iterate.
6. Run `--export` to produce the evidence pack a human can review.

The lab should support design work, not only design inspection. A
design work packet may say "raise source density without hiding the
Context Dock diagnostic," "make Debug Cockpit feel paused and safe," or
"restore Fire Horse command-lens emotional fit at Studio width," but it
must also name the concrete contracts, files, render commands, and
evaluation criteria that make the work checkable.

### UX Shape

The high-end Studio layout is a four-region cockpit:

```text
+---------------------+--------------------------------------+-----------------------------+
| Scenario Atlas      | Live Fire Horse Stage                | Audit Dossier               |
| personas/scenarios  | FrankenTui mockup or contract render | persona / journey / mapping |
| viewport ladder     |                                      | checklist / findings        |
+---------------------+--------------------------------------+-----------------------------+
| Evidence Rail: reference mockup, terminal capture, projection path, commands, export status |
+--------------------------------------------------------------------------------------------+
```

First-class may keep all four regions with narrower proportions. Compact
uses a single live stage with a right-side or lower audit drawer. The
reviewer can switch layout posture without changing the selected
scenario, persona, or finding set.

Region responsibilities:

| Region | Owns | Must not own |
| --- | --- | --- |
| Scenario Atlas | Persona list, scenario list, journey steps, viewport ladder, coverage badges. | Fire Horse projection data or command dispatch. |
| Live Fire Horse Stage | The current `oxide-uxlab` Fire Horse render, with `--mockup` as the review default and contract render as an explicit comparison mode. | Real project, semantic, run, or debug behavior. |
| Audit Dossier | Persona goals, scenario intent, success criteria, state/action/OxVba mapping, checklist, local findings. | Product authority not backed by docs or projection contracts. |
| Evidence Rail | Current artifact refs, reference image path, capture path, ANSI replay command, export status. | Persistent issue tracking or public posting. |

### Review Modes

The Audit Dossier has five modes:

| Mode | Purpose |
| --- | --- |
| Persona | Shows the target user, job pressure, working constraints, delight target, and failure modes. |
| Journey | Shows the selected scenario as steps with expected visible state and command affordances. |
| Mapping | Shows visible surface -> projection field -> state owner -> action id -> OxVba seam -> downstream workset. |
| Checklist | Shows audit criteria grouped by density/layout, information hierarchy, task fit, command clarity, seam honesty, and degradation. |
| Findings | Shows local pass/concern/fail/deferred marks and export-ready notes. |

The modes are not separate applications. They are review lenses over the
same selected scenario.

### Functional And Aesthetic Rubric

The audit rubric has two equal responsibilities.

Functional evaluation checks whether the design serves the IDE job:

| Category | Checks |
| --- | --- |
| Persona fit | The visible scene supports the named user's pressure, goals, constraints, and failure modes. |
| Journey fit | The selected step has the expected surfaces, state tokens, and next action affordances. |
| Command clarity | Visible commands have action ids, enabled/disabled reasons, and no silent affordances. |
| State ownership | Visible state maps to projection fields and current/planned OxIde owners. |
| Seam honesty | VBA/project facts map to OxVba seams or explicit future/unavailable seams. |
| Degradation | Compact/fallback modes preserve the task without becoming the high-end design target. |

Aesthetic evaluation checks whether the terminal design has the Fire
Horse quality bar:

| Category | Checks |
| --- | --- |
| Hierarchy | The eye can find source, context, activity, command, and status priority immediately. |
| Density | Studio and First-class use space richly without crowding or decorative filler. |
| Balance | Rails, docks, canvas, overlays, and lower surfaces carry visual weight intentionally. |
| Tone and color | Truecolor surfaces preserve graphite/ember energy while remaining readable and non-monochrome. |
| Terminal craft | Box drawing, padding, line rhythm, clipping, and ANSI rendering feel deliberate, not retro or broken. |
| Reference fidelity | The terminal result can cite which qualities of the colourful mockup it keeps, changes, or rejects. |
| Text fit | No label, command, code token, or status text overlaps incoherently at the target viewport. |
| Emotional fit | The scene feels fast, capable, calm under pressure, and more like a modern desktop IDE than a nostalgia console. |

The aesthetic checks are not fully objective, but they must be structured.
An agent may fail objective checks automatically, and may propose
`Concern` or `Pass` for aesthetic checks only with a rationale and a
reference artifact. Aesthetic pass claims require a cited capture or ANSI
artifact plus the reference mockup path.

### Command And Key Contract

Interactive entry points:

```text
oxide-uxlab --audit
oxide-uxlab --audit --suite firehorse
oxide-uxlab --audit --suite firehorse --scenario firehorse-editing-lens-standard --viewport studio
```

Non-interactive evidence entry points:

```text
oxide-uxlab --audit --suite firehorse --list
oxide-uxlab --audit --suite firehorse --list --json
oxide-uxlab --audit --suite firehorse --matrix --json
oxide-uxlab --audit --suite firehorse --scenario firehorse-editing-lens-standard --viewport studio --brief --json
oxide-uxlab --audit --suite firehorse --scenario firehorse-editing-lens-standard --viewport studio --once
oxide-uxlab --audit --suite firehorse --scenario firehorse-editing-lens-standard --viewport studio --once --json
oxide-uxlab --audit --suite firehorse --scenario firehorse-editing-lens-standard --viewport studio --evaluate functional,aesthetic --json
oxide-uxlab --audit --suite firehorse --scenario firehorse-editing-lens-standard --viewport studio --export docs/firehorse_mockups/ux_audit_lab/editing_lens_studio.md
oxide-uxlab --audit --suite firehorse --batch docs/firehorse_mockups/ux_audit_lab/agent_run.json --json
```

Interactive keys:

| Key | Action id | Visible outcome |
| --- | --- | --- |
| `Tab` / `Shift+Tab` | `audit.focus.next` / `audit.focus.previous` | Moves focus between Atlas, Stage, Dossier, and Evidence Rail. |
| `j` / `k` or arrows | `audit.selection.move` | Moves within the focused list or checklist. |
| `Enter` | `audit.selection.open` | Opens the selected persona, scenario, step, criterion, or artifact ref. |
| `v` | `audit.viewport.next` | Cycles Studio, First-class, Standard, Compact. |
| `r` | `audit.render.mode.toggle` | Toggles mockup render vs contract render. |
| `1`..`5` | `audit.dossier.mode` | Switches Persona, Journey, Mapping, Checklist, Findings. |
| `p` | `audit.mark.pass` | Marks current criterion pass. |
| `c` | `audit.mark.concern` | Marks current criterion concern. |
| `f` | `audit.mark.fail` | Marks current criterion fail. |
| `d` | `audit.mark.deferred` | Marks current criterion deferred with downstream owner. |
| `e` | `audit.export.review_pack` | Writes the local review pack for selected scope. |
| `?` | `audit.help.toggle` | Shows the audit key sheet. |
| `Esc` | `audit.dismiss_or_return` | Dismisses drawer/help, then returns to the prior focus. |
| `q` | `audit.quit` | Exits the lab. |

The keys are display/test contracts inside uxlab. Real product command
dispatch remains W090-owned.

### Audit Data Model

W041 introduces audit-only types under `src/shell/uxlab/audit/`. They
reference W039 projection ids and artifact paths; they do not own VBA
meaning.

Sketch:

```rust
pub struct UxAuditSuite {
    pub id: &'static str,
    pub title: &'static str,
    pub schema_version: u32,
    pub personas: Vec<UxPersona>,
    pub scenarios: Vec<UxAuditScenario>,
    pub criteria: Vec<UxAuditCriterion>,
    pub rubrics: Vec<UxAuditRubric>,
}

pub struct UxPersona {
    pub id: &'static str,
    pub title: &'static str,
    pub role: &'static str,
    pub job_pressure: &'static str,
    pub goals: Vec<&'static str>,
    pub constraints: Vec<&'static str>,
    pub delight_target: &'static str,
    pub failure_modes: Vec<&'static str>,
    pub source_refs: Vec<AuditArtifactRef>,
}

pub struct UxAuditScenario {
    pub id: &'static str,
    pub firehorse_scenario_id: &'static str,
    pub persona_id: &'static str,
    pub title: &'static str,
    pub intent: &'static str,
    pub default_viewport: &'static str,
    pub steps: Vec<UxJourneyStep>,
    pub reference_artifacts: Vec<AuditArtifactRef>,
}

pub struct UxJourneyStep {
    pub id: &'static str,
    pub title: &'static str,
    pub user_intent: &'static str,
    pub expected_surfaces: Vec<UxSurfaceExpectation>,
    pub expected_actions: Vec<&'static str>,
    pub state_refs: Vec<UxStateRef>,
    pub seam_refs: Vec<UxSeamRef>,
}

pub struct UxSurfaceExpectation {
    pub surface: &'static str,
    pub projection_path: &'static str,
    pub visible_contract: &'static str,
    pub owner_workset: &'static str,
}

pub struct UxAuditCriterion {
    pub id: &'static str,
    pub category: AuditCategory,
    pub question: &'static str,
    pub severity_if_failed: AuditSeverity,
    pub evidence_required: &'static str,
    pub evaluation_mode: EvaluationMode,
}

pub struct UxAuditFinding {
    pub scenario_id: &'static str,
    pub criterion_id: &'static str,
    pub status: AuditStatus,
    pub note: String,
    pub downstream_owner: Option<&'static str>,
}

pub struct UxDesignBrief {
    pub scenario_id: &'static str,
    pub viewport: &'static str,
    pub persona_id: &'static str,
    pub design_intent: &'static str,
    pub must_preserve: Vec<&'static str>,
    pub likely_files: Vec<&'static str>,
    pub render_commands: Vec<String>,
    pub evaluation_commands: Vec<String>,
}

pub struct UxAuditRun {
    pub suite_id: &'static str,
    pub scenario_ids: Vec<&'static str>,
    pub viewports: Vec<&'static str>,
    pub render_modes: Vec<RenderMode>,
    pub criteria_filter: Vec<AuditCategory>,
    pub output_root: String,
}

pub struct UxAuditScorecard {
    pub run_id: String,
    pub scenario_id: String,
    pub viewport: String,
    pub functional: Vec<UxCriterionResult>,
    pub aesthetic: Vec<UxCriterionResult>,
    pub gate: AuditGate,
    pub artifacts: Vec<AuditArtifactRef>,
}

pub struct UxCriterionResult {
    pub criterion_id: String,
    pub status: AuditStatus,
    pub confidence: AuditConfidence,
    pub rationale: String,
    pub evidence: Vec<AuditArtifactRef>,
}
```

Initial personas should be small but reviewable:

| Persona id | Role | Primary pressure |
| --- | --- | --- |
| `pricing_maintainer` | Excel/VBA maintainer responsible for business logic. | Needs fast confidence while editing and running pricing code. |
| `migration_reviewer` | Developer reviewing inherited VBA during modernization. | Needs meaning, provenance, and navigation without breaking host truth. |
| `debug_responder` | User diagnosing a live macro failure. | Needs paused-state clarity, stack/local visibility, and safe recovery. |
| `terminal_power_user` | Developer choosing OxIde because terminal workflow is faster. | Needs dense, precise, low-friction command and viewport behavior. |

### Fire Horse Scenario Mapping

Every W039 Fire Horse scenario must have at least one audit scenario:

| Fire Horse scenario id | Audit angle |
| --- | --- |
| `firehorse-launchpad-standard` | Cold-start confidence, recent work discovery, terminal capability posture. |
| `firehorse-editing-lens-standard` | Main source-centered editing loop for a high-end desktop terminal. |
| `firehorse-command-lens-standard` | Command discovery, disabled reasons, preview clarity, action id honesty. |
| `firehorse-run-lane-standard` | Run progress, output/Immediate relationship, source continuity. |
| `firehorse-debug-cockpit-standard` | Paused debug posture, call stack/locals/watch relationship, step affordances. |
| `firehorse-console-fit-light` | Light theme and capability education without weakening source focus. |
| `firehorse-focus-compact` | Controlled degradation to small-terminal source-first mode. |
| `firehorse-real-editing` | Adapter honesty for current shell state and missing seam data. |

### Mapping Discipline

Mapping rows in the Audit Dossier must use this order:

```text
visible surface
  -> FireHorseProjection path
  -> current or planned OxIde owner
  -> action id if command-bearing
  -> OxVba seam or named future seam if VBA/project meaning appears
  -> downstream implementation workset
  -> evidence artifact
```

Examples:

| Visible surface | Projection path | Owner | Seam | Workset |
| --- | --- | --- | --- | --- |
| Diagnostic card | `context_dock.cards[].Diagnostic` | Inspector mode state | `HostWorkspaceSession::diagnostics` | W060 |
| Source lens hover | `code_canvas.lens` | Editor semantic overlay | `HostWorkspaceSession::hover` | W060 |
| Run timeline row | `activity_deck.rows[]` | Run execution projection | `WebHostEvent` stream, later typed run event seam | W070 |
| Locals table | `context_dock.cards[].Locals` | Debug context dock | W080 debug contract audit | W080 |
| Command Lens row | `overlay.rows[]` | Palette / future `ActionRegistry` view | None unless command preview includes semantic data | W090 |
| Console fit row | `terminal_fit.rows[]` | Terminal capability state | None | W100 |

If a visible VBA/project fact lacks a real or named future seam, the
finding status is `Concern` or `Fail`. The lab must never silently treat
UI copy as semantic truth.

### Evidence Products

W041 produces local review packs under
`docs/firehorse_mockups/ux_audit_lab/` when the reviewer exports:

```text
docs/firehorse_mockups/ux_audit_lab/
  README.md
  audit_suite.json
  audit_schema.json
  agent_brief_<scenario>_<viewport>.json
  audit_run.json
  scorecard.json
  findings.json
  scenario_<id>_<viewport>.md
  captures/
  ansi/
```

The export should include commands sufficient to reproduce each capture.
Agent-facing JSON files must be deterministic, schema-versioned, and free
of terminal escape bytes. The export is local evidence only; it does not
create GitHub issues, comments, or public posts.

### Architecture

W041 extends the existing uxlab modules:

```text
src/shell/uxlab/
  audit/
    mod.rs
    registry.rs
    model.rs
    fixtures.rs
    schema.rs
    score.rs
    automation.rs
    view.rs
    controller.rs
    export.rs
```

Responsibilities:

- `registry.rs` registers audit suites and links audit scenario ids to
  existing uxlab suite/scenario ids.
- `model.rs` owns `UxAuditSuite`, `UxPersona`, `UxAuditScenario`,
  `UxJourneyStep`, `UxAuditCriterion`, `UxAuditFinding`,
  `UxDesignBrief`, `UxAuditRun`, and `UxAuditScorecard`.
- `fixtures.rs` defines the first Fire Horse audit suite from W035,
  W038, W039, and Fire Horse review artifacts.
- `schema.rs` owns versioned JSON serialization contracts and rejects
  unknown schema versions.
- `score.rs` owns functional and aesthetic criterion evaluation,
  including objective preflight checks and structured manual/agent
  judgement slots.
- `automation.rs` owns batch runs, work-packet generation, compare
  inputs, exit codes, and non-interactive agent commands.
- `view.rs` renders the Atlas, Stage, Dossier, and Evidence Rail using
  FrankenTui.
- `controller.rs` owns focus movement, dossier mode changes, viewport
  switching, render mode switching, and local finding state.
- `export.rs` writes Markdown and JSON review packs, including agent
  briefs and scorecards.

The Live Stage reuses the existing Fire Horse mockup/contract renderers.
No W041 module should fork the Fire Horse renderer or create a second
projection type.

### Test And Evidence Strategy

Feature beads keep the normal bead evidence bar:

- Unit tests pin registry completeness, scenario mapping, key/action
  contracts, export shape, and missing-seam detection.
- Unit tests pin JSON schema versioning, stable ids, batch-run parsing,
  scorecard gate calculation, and exit-code behavior.
- WTD journeys launch the release `oxide-uxlab` binary in audit mode and
  assert visible panes, selected scenarios, dossier modes, mapping rows,
  and export feedback.
- Non-interactive automation journeys run the release binary with
  `--brief --json`, `--once --json`, `--evaluate ... --json`, and
  `--batch ... --json`, then assert parseable JSON and stable artifact
  paths.
- Five-minute user passes prioritize Studio and First-class viewports,
  then check compact/fallback degradation.

Doctrine and infrastructure beads use read-through evidence only when
they have no user-observable behavior.

## Beads

### W041-B00 - Audit lab doctrine and bead graph

Type: Doctrine

**Goal.** A reviewer can read this workset and see the full UX Audit Lab
scope, sequencing, dependencies, and bead graph before implementation
starts.

**Design.**

- Add `docs/worksets/W041_firehorse_ux_audit_lab.md`.
- Register W041 in `docs/WORKSET_REGISTER.md`.
- Create live `.beads` entries using `br`, with W041 labels and explicit
  dependencies.
- Set W041 after W040 in the register, because it is a lab/review
  capability that guides downstream W050/W060/W070/W080/W090/W100 work.

**Tests.**

- Read-through check: the workset follows `docs/BEADS.md` section 4.1.
- Read-through check: every bead has Goal, Design, Tests, Evidence, and
  Closure.
- `br list --status open --json` shows the W041 epic and child beads.

**Evidence.**

- Workset file exists.
- Register links the workset file.
- Live bead ids are recorded in `.beads` through `br`.

**Closure.**

- [ ] Workset spec created.
- [ ] Register updated.
- [ ] Live bead graph created through `br`.
- [ ] No `.beads` file edited by hand.
- [ ] Read-through checklist complete.

### W041-B01 - Audit authority inventory and traceability map

Type: Doctrine

**Goal.** A reviewer can open a single traceability document and see
which UX/product documents, Fire Horse artifacts, terminal captures, and
projection contracts feed each audit scenario.

**Design.**

- Add `docs/firehorse_mockups/ux_audit_lab/TRACEABILITY.md`.
- Inventory:
  - W035 principles and journeys;
  - W038 high-end review and FrankenTui terminal review artifacts;
  - W039 projection contract, UX reset, hardening review, scenario ids,
    and action id matrix;
  - colourful Fire Horse PNGs and refined mockups;
  - terminal capture and ANSI review pack paths.
- Define traceability row shape:
  `audit scenario -> persona -> journey source -> Fire Horse scenario ->
  reference mockup -> terminal capture -> projection contract section ->
  downstream owner workset`.
- Identify gaps as explicit "audit inputs missing" rows rather than
  filling them with assumptions.

**Tests.**

- Read-through check: all W039 Fire Horse scenario ids appear.
- Read-through check: every scenario has at least one artifact ref and
  one source document ref.
- Read-through check: gaps are listed separately from confirmed inputs.

**Evidence.**

- Traceability document committed.
- Reviewer can follow at least one complete row from `pricing_maintainer`
  to `firehorse-editing-lens-standard` to W060/W090 owners.

**Closure.**

- [ ] Traceability document exists.
- [ ] All W039 scenario ids covered.
- [ ] Artifact refs point to real repo paths.
- [ ] Missing inputs are explicit.
- [ ] Workset spec updated if scope changes.

### W041-B02 - Audit data model and Fire Horse suite fixtures

Type: Infrastructure

**Goal.** `oxide-uxlab` can load a Fire Horse audit suite containing
personas, audit scenarios, journey steps, criteria, mapping refs,
rubrics, design briefs, and scorecard schema metadata without rendering
the interactive lab yet.

**Design.**

- Add `src/shell/uxlab/audit/model.rs` with the audit-only structs from
  the design section.
- Add `src/shell/uxlab/audit/registry.rs` with suite lookup by id.
- Add `src/shell/uxlab/audit/schema.rs` with a versioned JSON schema
  contract for agent consumers.
- Add `src/shell/uxlab/audit/fixtures.rs` with:
  - `pricing_maintainer`;
  - `migration_reviewer`;
  - `debug_responder`;
  - `terminal_power_user`;
  - audit scenarios for every W039 Fire Horse scenario id;
  - criteria categories for density/layout, information hierarchy, task
    fit, command clarity, seam honesty, and degradation.
- Add design-brief fixtures for each scenario/viewport pair that name
  must-preserve contracts, reference artifacts, render commands,
  evaluation commands, and likely implementation owners.
- Link audit scenarios to existing Fire Horse scenario ids rather than
  duplicating projection data.

**Tests.**

- Unit contract: every W039 Fire Horse scenario id has at least one audit
  scenario.
- Unit contract: every audit scenario references an existing persona.
- Unit contract: every surface expectation has a projection path and
  downstream workset.
- Unit contract: seam-bearing expectations have either an OxVba seam ref
  or an explicit unavailable/future seam ref.
- Unit contract: every criterion declares functional or aesthetic
  evaluation mode.
- Unit contract: JSON schema version, ids, and artifact refs serialize
  deterministically.

**Evidence.**

- `cargo test uxlab` passes.
- Read-through of fixture rows confirms no UI copy is used as semantic
  source truth.

**Closure.**

- [ ] Audit model added under uxlab.
- [ ] Fire Horse audit suite fixtures load.
- [ ] Agent-facing schema and design briefs load.
- [ ] Unit contracts green.
- [ ] No new renderer or production state owner introduced.
- [ ] Workset spec updated if data shape changes.

### W041-B03 - Audit CLI listing and single-scenario export

Type: Feature

**Goal.** Running `oxide-uxlab --audit --suite firehorse --list` lists
audit personas/scenarios/criteria, running `--audit --brief --json`
emits a design work packet, and running `--audit --once` for a scenario
emits either a readable audit dossier or a parseable JSON dossier for
that selected scene.

**Design.**

- Extend `src/bin/oxide-uxlab.rs` CLI parsing with audit flags:
  - `--audit`;
  - `--list`;
  - `--matrix`;
  - `--brief`;
  - `--json`;
  - `--export <path>`.
- Keep normal `--suite`, `--scenario`, `--viewport`, `--once`,
  `--mockup`, and `--ansi` behavior compatible.
- `--audit --list` prints:
  - suite id/title;
  - personas;
  - audit scenarios and linked Fire Horse scenario ids;
  - criteria categories.
- `--audit --list --json` emits machine-readable suite discovery.
- `--audit --matrix --json` emits scenario/persona/viewport/criteria
  coverage for agents choosing the next design target.
- `--audit --brief --json` emits a design work packet for one
  scenario/viewport: persona pressure, intended emotional quality,
  must-preserve contracts, likely files, and commands to run.
- `--audit --once` prints a terminal-cell audit dossier:
  - selected persona;
  - selected scenario and journey step list;
  - required surfaces/actions;
  - state/seam/workset mapping summary;
  - reproduction command for the live Fire Horse render.
- `--audit --once --json` emits the same dossier without ANSI bytes,
  including artifact refs and expected paths.
- `--export` writes Markdown for the selected scenario, creating parent
  directories only when they already sit under the requested output path.

**Tests.**

- Unit contract: audit CLI rejects unknown audit suite ids with an honest
  message.
- Unit contract: `--audit --list` includes every audit scenario id.
- Unit contract: `--audit --once` includes persona, journey, mapping, and
  reproduction command sections.
- Unit contract: `--audit --brief --json` parses as a design work packet
  and includes must-preserve contracts.
- Unit contract: `--audit --once --json` parses as an audit dossier and
  contains no ANSI escape bytes.
- WTD journey: run release `oxide-uxlab.exe --audit --suite firehorse
  --scenario firehorse-editing-lens-standard --viewport studio --once`
  and assert the audit dossier names Editing Lens, Studio, projection
  paths, and downstream worksets.
- Automation journey: run release `oxide-uxlab.exe --audit --suite
  firehorse --scenario firehorse-editing-lens-standard --viewport studio
  --brief --json` and assert stable ids, artifact refs, and reproduction
  commands.

**Evidence.**

- `cargo test uxlab` passes.
- `cargo build --release --bin oxide-uxlab` passes.
- WTD audit dossier journey passes against the release binary.
- Five-minute user pass: run `--audit --list`, one Studio `--once`, one
  First-class `--once`, and one Compact `--once`.

**Closure.**

- [ ] Audit CLI flags work.
- [ ] JSON discovery and design-brief outputs work.
- [ ] Unknown suite/scenario errors are honest.
- [ ] Unit tests green.
- [ ] WTD journey green on release binary.
- [ ] Five-minute user pass complete.

### W041-B04 - Interactive audit lab shell

Type: Feature

**Goal.** Running `oxide-uxlab --audit --suite firehorse` opens an
interactive FrankenTui audit cockpit with Scenario Atlas, Live Fire Horse
Stage, Audit Dossier, and Evidence Rail visible in Studio and First-class
viewports.

**Design.**

- Add `src/shell/uxlab/audit/view.rs` and `controller.rs`.
- Reuse W038 viewport definitions and Fire Horse render entry points.
- Default selected audit scenario:
  `firehorse-editing-lens-standard` in Studio viewport with mockup render
  mode.
- Draw:
  - Scenario Atlas with personas, scenarios, journey steps, and viewport
    ladder;
  - Live Stage using the existing `--mockup` Fire Horse renderer;
  - Audit Dossier in Persona mode;
  - Evidence Rail with reference artifact paths and reproduction command.
- Implement focus movement and selection movement. Selection changes
  update the live stage and dossier without restarting the app.
- Preserve non-interactive `--once` behavior.

**Tests.**

- Unit contract: initial audit state selects Editing Lens, Studio, mockup
  render mode, and Persona dossier mode.
- Unit contract: focus traversal visits Atlas, Stage, Dossier, Evidence
  Rail by semantic region id.
- Unit contract: selecting a scenario updates linked Fire Horse scenario
  id and live stage request.
- WTD journey: launch release audit lab, assert the four regions are
  visible, move selection, press Enter, and assert the scenario title and
  live stage change.

**Evidence.**

- `cargo test uxlab` passes.
- `cargo build --release --bin oxide-uxlab` passes.
- WTD interactive audit shell journey passes.
- Five-minute user pass in Studio and First-class viewports confirms
  text does not overlap and live stage remains readable.

**Closure.**

- [ ] Interactive audit shell launches.
- [ ] Four regions visible at high-end viewports.
- [ ] Scenario selection updates dossier and stage.
- [ ] Unit tests green.
- [ ] WTD journey green on release binary.
- [ ] Five-minute user pass complete.

### W041-B05 - Persona and journey review lenses

Type: Feature

**Goal.** In the interactive audit lab, pressing `1` and `2` switches
between Persona and Journey dossier modes so the reviewer can inspect who
the scene serves and what task steps the scene must support.

**Design.**

- Extend Audit Dossier rendering with Persona and Journey modes.
- Persona mode shows role, pressure, goals, constraints, delight target,
  and failure modes.
- Journey mode shows steps, expected surfaces, expected actions, and
  success outcome.
- Atlas selection can choose a persona, scenario, or journey step. The
  dossier should keep the selected object visible and mark cross-refs to
  the live Fire Horse scenario.
- Evidence Rail updates source refs when the selected persona or journey
  step changes.

**Tests.**

- Unit contract: Persona mode includes role, pressure, goals, and failure
  modes for each persona.
- Unit contract: Journey mode includes every step id and expected action
  for the selected scenario.
- Unit contract: switching dossier modes preserves selected scenario and
  viewport.
- WTD journey: launch audit lab, press `2`, select a journey step, assert
  expected surfaces/actions are visible, press `1`, assert persona goals
  are visible.

**Evidence.**

- `cargo test uxlab` passes.
- WTD persona/journey lens journey passes on release binary.
- Five-minute user pass checks all four initial personas and at least
  three Fire Horse scenarios.

**Closure.**

- [ ] Persona mode complete.
- [ ] Journey mode complete.
- [ ] Cross-refs remain stable while switching modes.
- [ ] Unit tests green.
- [ ] WTD journey green on release binary.
- [ ] Five-minute user pass complete.

### W041-B06 - State, action, and OxVba mapping lens

Type: Feature

**Goal.** Pressing `3` in the Audit Dossier shows the selected scenario's
visible surface mappings all the way through projection paths, OxIde
owners, action ids, OxVba seams, downstream worksets, and evidence refs.

**Design.**

- Add Mapping dossier mode.
- Render mapping rows in the fixed discipline:
  visible surface -> projection path -> owner -> action id -> OxVba seam
  -> downstream workset -> evidence artifact.
- Add missing-seam detection:
  - seam-bearing VBA/project facts without a seam ref are marked concern;
  - known future seams are marked explicit future dependency;
  - non-semantic shell surfaces are marked "no OxVba seam required".
- Allow mapping rows to be filtered by downstream workset using Atlas or
  Dossier selection.

**Tests.**

- Unit contract: every mapping row has visible surface, projection path,
  owner, downstream workset, and evidence ref.
- Unit contract: command-bearing rows include an action id.
- Unit contract: semantic/project rows without a seam ref are reported as
  missing seam concerns.
- WTD journey: press `3` on Editing Lens and assert rows for Diagnostic
  card, Source lens, Command Lens action id, and W060/W090 owners are
  visible.

**Evidence.**

- `cargo test uxlab` passes.
- WTD mapping lens journey passes on release binary.
- Five-minute user pass verifies Editing Lens, Command Lens, Run Lane,
  Debug Cockpit, and Console Fit mappings.

**Closure.**

- [ ] Mapping lens complete.
- [ ] Missing-seam concerns visible.
- [ ] Unit tests green.
- [ ] WTD journey green on release binary.
- [ ] Five-minute user pass complete.

### W041-B07 - Audit checklist and local finding capture

Type: Feature

**Goal.** In Checklist and Findings modes, the reviewer can mark criteria
as pass, concern, fail, or deferred for the current scenario and see those
marks reflected in a local findings summary.

**Design.**

- Add Checklist mode (`4`) and Findings mode (`5`).
- Checklist groups criteria by:
  - density/layout;
  - information hierarchy;
  - persona/task fit;
  - command clarity;
  - seam honesty;
  - degradation.
- Every criterion declares whether it is functional, aesthetic, or mixed.
  Aesthetic criteria require rationale and reference artifacts when
  marked pass or concern.
- Implement in-memory finding marks with note placeholders:
  - `p` pass;
  - `c` concern;
  - `f` fail;
  - `d` deferred.
- Findings mode summarizes marked criteria by scenario and category.
- Findings store enough structured fields to be serialized into a
  scorecard: criterion id, status, confidence, rationale, evidence refs,
  and downstream owner.
- No public posting. No GitHub issue creation. Export remains local only.

**Tests.**

- Unit contract: every criterion category renders with at least one
  criterion.
- Unit contract: aesthetic criteria cannot pass without rationale and a
  reference artifact.
- Unit contract: marking a criterion updates the selected scenario's
  finding summary.
- Unit contract: deferred findings require a downstream owner workset.
- WTD journey: open Checklist, mark one pass and one concern, switch to
  Findings, assert both marks are visible.

**Evidence.**

- `cargo test uxlab` passes.
- WTD checklist/finding journey passes on release binary.
- Five-minute user pass marks criteria for at least Editing Lens and
  Debug Cockpit, then clears or exits without accidental persistence.

**Closure.**

- [ ] Checklist mode complete.
- [ ] Findings mode complete.
- [ ] Marks are local and visible.
- [ ] Aesthetic findings require structured evidence.
- [ ] Deferred findings name a downstream owner.
- [ ] Unit tests green.
- [ ] WTD journey green on release binary.
- [ ] Five-minute user pass complete.

### W041-B08 - Viewport ladder and render-mode comparison

Type: Feature

**Goal.** The reviewer can cycle Studio, First-class, Standard, and
Compact viewports and toggle mockup vs contract render while the audit
selection, dossier, and findings remain stable.

**Design.**

- Implement `v` viewport cycling.
- Implement `r` render mode toggle:
  - mockup mode uses W038-B15 FrankenTui Fire Horse mockup renderer;
  - contract mode uses the W039 contract renderer;
  - the Evidence Rail labels the current mode clearly.
- Add viewport-readiness badges:
  - `High-end target`;
  - `Readable`;
  - `Concern`;
  - `Unsupported`.
- Add objective aesthetic preflight checks for each rendered viewport:
  - text clipping and overlap markers;
  - empty-line and dense-line ratios;
  - required surface presence;
  - color-token availability for ANSI mode;
  - reference artifact link present.
- Keep Studio and First-class as primary review modes. Compact is a
  degradation review, not the baseline design.

**Tests.**

- Unit contract: viewport cycling preserves selected scenario and
  finding state.
- Unit contract: render mode toggle changes stage source but preserves
  audit dossier content.
- Unit contract: Studio and First-class are labeled high-end targets.
- Unit contract: aesthetic preflight reports clipping/overlap concerns
  as machine-readable scorecard rows.
- WTD journey: launch audit lab, cycle viewport, toggle render mode,
  assert viewport/mode labels and selected scenario remain visible.

**Evidence.**

- `cargo test uxlab` passes.
- WTD viewport/render comparison journey passes on release binary.
- Five-minute user pass compares Editing Lens and Command Lens in Studio
  and First-class, then verifies Compact remains honest.

**Closure.**

- [ ] Viewport cycling works.
- [ ] Render mode toggle works.
- [ ] High-end targets are labeled as primary.
- [ ] Objective aesthetic preflight rows are produced.
- [ ] Unit tests green.
- [ ] WTD journey green on release binary.
- [ ] Five-minute user pass complete.

### W041-B09 - Review pack export

Type: Feature

**Goal.** Pressing `e` or running `--audit --export` writes a local review
pack that captures the selected audit scope, findings, scorecards, agent
briefs, artifact refs, reproduction commands, and render captures.

**Design.**

- Add `src/shell/uxlab/audit/export.rs`.
- Export Markdown and JSON:
  - `README.md` for suite-level export;
  - `audit_schema.json`;
  - `audit_suite.json`;
  - `agent_brief_<scenario>_<viewport>.json`;
  - `audit_run.json`;
  - `scorecard.json`;
  - `findings.json`;
  - one `scenario_<id>_<viewport>.md` per exported scenario/viewport.
- Include:
  - persona and scenario summary;
  - journey steps;
  - mapping rows;
  - checklist results;
  - functional and aesthetic scorecards;
  - reproduction commands for mockup and contract render;
  - reference image/capture paths.
- Capture generation may reuse existing `--once --mockup` and
  `--once --mockup --ansi` paths.
- Refuse to overwrite a non-audit-lab file unless the output path is
  explicitly within the requested audit export directory and the command
  reports what it wrote.

**Tests.**

- Unit contract: export includes persona, journey, mapping, findings,
  scorecard, agent brief, and reproduction sections.
- Unit contract: export refuses unknown suite/scenario ids.
- Unit contract: export path guard rejects paths outside the requested
  export root.
- Unit contract: exported JSON is deterministic for the same audit state.
- WTD journey: mark findings, press `e`, assert export completion is
  visible and the expected Markdown/JSON files exist.

**Evidence.**

- `cargo test uxlab` passes.
- WTD export journey passes on release binary.
- Five-minute user pass exports a Studio Editing Lens review pack and a
  First-class Command Lens review pack.

**Closure.**

- [ ] Markdown and JSON export work.
- [ ] Agent briefs and scorecards export.
- [ ] Export reports files written.
- [ ] Path guard works.
- [ ] Unit tests green.
- [ ] WTD journey green on release binary.
- [ ] Five-minute user pass complete.

### W041-B10 - Agent automation runner and scorecard API

Type: Feature

**Goal.** A coding agent can run one command against the release
`oxide-uxlab` binary and receive a machine-readable functional and
aesthetic scorecard that tells it what design work passed, failed, or
needs follow-up.

**Design.**

- Add `src/shell/uxlab/audit/automation.rs` and `score.rs`.
- Implement `--audit --evaluate functional,aesthetic --json` for one
  scenario/viewport.
- Implement `--audit --batch <path> --json` for multi-scenario audit
  runs. Batch input names suite, scenarios, viewports, render modes,
  criteria filters, and output root.
- Define exit codes:
  - `0`: all selected criteria pass or are explicitly deferred;
  - `1`: concern/fail gate reached;
  - `2`: malformed command, missing suite/scenario, or schema mismatch;
  - `3`: render/capture failure.
- Scorecard output includes:
  - schema version;
  - selected suite/scenario/viewport/render mode;
  - functional criteria results;
  - aesthetic criteria results;
  - objective preflight metrics;
  - gate state;
  - artifact refs and reproduction commands.
- The runner must not invoke public posting, GitHub issue creation, or
  destructive file operations.

**Tests.**

- Unit contract: malformed batch input returns schema error and exit-code
  contract.
- Unit contract: functional scorecard includes persona, journey, command,
  state, and seam criteria.
- Unit contract: aesthetic scorecard includes hierarchy, density,
  balance, color/tone, terminal craft, reference fidelity, text fit, and
  emotional fit criteria.
- Unit contract: gate calculation fails on any non-deferred fail and
  reports concerns separately from fails.
- Automation journey: run release `oxide-uxlab.exe --audit --suite
  firehorse --scenario firehorse-editing-lens-standard --viewport studio
  --evaluate functional,aesthetic --json`; parse JSON; assert functional
  and aesthetic sections, gate, and artifact refs.
- Automation journey: run release `oxide-uxlab.exe --audit --suite
  firehorse --batch <fixture> --json`; assert all selected scenarios
  produce scorecards under the requested audit export root.

**Evidence.**

- `cargo test uxlab` passes.
- `cargo build --release --bin oxide-uxlab` passes.
- Both automation journeys pass against the release binary.
- Five-minute agent pass: run list, brief, evaluate, batch, and export
  commands for Editing Lens Studio and Command Lens First-class, then
  inspect the resulting scorecards.

**Closure.**

- [ ] `--evaluate ... --json` works.
- [ ] `--batch ... --json` works.
- [ ] Exit codes are deterministic.
- [ ] Functional and aesthetic scorecards are parseable.
- [ ] Unit tests green.
- [ ] Automation journeys green on release binary.
- [ ] Five-minute agent pass complete.

### W041-B11 - Downstream audit handoff and gate rules

Type: Doctrine

**Goal.** Downstream workset authors can use the UX Audit Lab outputs to
decide which Fire Horse design claims are implementation-ready, which are
deferred, and which need redesign before product code adopts them.

**Design.**

- Add `docs/firehorse_mockups/ux_audit_lab/HANDOFF.md`.
- Define gate states:
  - `Ready for implementation`;
  - `Ready with named downstream dependency`;
  - `Design concern`;
  - `Blocked by missing seam`;
  - `Rejected for scope`.
- Map audit findings to downstream worksets:
  - W050 document lifecycle;
  - W060 semantic UX;
  - W070 run/immediate;
  - W080 debug surfaces;
  - W090 command system;
  - W100 terminal capability;
  - W110 polish/recovery.
- Define how a downstream bead cites audit evidence:
  - audit scenario id;
  - viewport;
  - finding id;
  - scorecard id and gate state;
  - exported review pack path;
  - agent brief path if the downstream bead is implementing a design
    correction;
  - observed release-binary behavior if the downstream bead claims it.
- Update W039 handoff language if the audit lab changes how W039 outputs
  should be consumed.

**Tests.**

- Read-through check: every gate state is defined.
- Read-through check: every W039 scenario maps to at least one downstream
  implementation area or explicit non-product proof status.
- Read-through check: downstream citation format is concrete enough for
  a bead design.
- Read-through check: agent-generated scorecards can be cited without
  replacing the author's release-binary observation duties.

**Evidence.**

- Handoff document exists.
- Workset specs or handoff docs updated only where authority changed.
- Reviewer can trace at least one exported finding into a downstream
  bead citation example.

**Closure.**

- [ ] Handoff document created.
- [ ] Gate states defined.
- [ ] Downstream citation format defined.
- [ ] Agent scorecard citation rule defined.
- [ ] W039 handoff updated if needed.
- [ ] Read-through checklist complete.

### W041-B12 - UX Audit Lab closure review

Type: Doctrine

**Goal.** The workset closes only after the lab has produced a usable
Fire Horse audit pack and the remaining design/product risks have been
filed as follow-up beads.

**Design.**

- Run a full local audit pass over:
  - Launchpad;
  - Editing Lens;
  - Command Lens;
  - Run Lane;
  - Debug Cockpit;
  - Console Fit;
  - Compact Focus;
  - Real Editing Adapter.
- Require Studio and First-class review for all high-end scenarios.
- Require Compact review for Compact Focus and at least one high-end
  scenario to verify degradation.
- Run an agent automation pass over at least Editing Lens Studio,
  Command Lens First-class, Debug Cockpit Studio, and Compact Focus
  Compact. The pass must produce parseable design briefs, scorecards, and
  export artifacts.
- File follow-up beads for:
  - any missing seam that blocks implementation;
  - any high-end layout that does not meet the Fire Horse direction;
  - any aesthetic scorecard concern that blocks the Fire Horse emotional
    target;
  - any action id mismatch with W039/W090;
  - any export or evidence gap.
- Update this workset only if closure reveals scope/design drift.

**Tests.**

- Read-through checklist over exported audit pack.
- `br list --status open --json` confirms follow-up beads exist for any
  unresolved concerns.
- Release-binary audit commands in the exported pack reproduce.
- Agent automation commands in the exported pack reproduce and their
  JSON parses.

**Evidence.**

- Exported suite review pack under
  `docs/firehorse_mockups/ux_audit_lab/`.
- Follow-up beads filed before closure for every unresolved concern.
- Workset closure note cites only evidence actually seen in the release
  binary.

**Closure.**

- [ ] Full Fire Horse audit pack exported.
- [ ] Studio and First-class scenarios reviewed.
- [ ] Compact degradation reviewed.
- [ ] Agent automation pass completed.
- [ ] Follow-up beads filed for unresolved concerns.
- [ ] Reproduction commands verified on release binary.
- [ ] Commit message cross-checked against observed behavior.

## Out-of-scope

- Replacing the production `ox-ide` renderer. W041 is an `oxide-uxlab`
  extension.
- Implementing real project/workspace behavior. W040 owns project truth.
- Implementing file/document lifecycle. W050 owns document services.
- Implementing real diagnostics, hover, references, symbols, or
  completions. W060 owns semantic UX over OxVba.
- Implementing run, Immediate, or debug behavior. W070 and W080 own those
  surfaces and OxVba contracts.
- Implementing the real command registry. W090 owns command dispatch and
  keymap profiles.
- Implementing terminal probing/degradation policy. W100 owns capability
  detection and onboarding.
- Public posting, GitHub issue creation, or external design publication.
  W041 export is local evidence only.
- Deleting older mockups or captures. Older assets remain until the user
  gives an exact delete command under `AGENTS.md`.
