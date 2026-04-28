# Workset W110 — Polish, Accessibility, And Recovery

## Ambition

OxIde stops being "the IDE that mostly works" and becomes "the IDE
that doesn't embarrass me in a demo". Every error path surfaces a
recoverable state instead of a panic. Every cold-start affordance
works. Mouse and keyboard parity is audited and any silent
affordance is either wired or retired. The `wtd` regression suite is
locked as the stable definition of "the IDE looks right".

W110 is deliberately cross-cutting. It is the final pass before the
shell is considered feature-complete against the current uxpass
revision.

## Dependencies

- **W100** — capability probing in force; degradation paths available.
- **W035 §50 `visual_language.md`** — animation / motion policy.
- **W037 `wtd` harness** — every bead here closes a regression gap.

## Design

### Four lanes

1. **Error recovery.** Every panic-capable input path becomes a
   Result + popover. Missing project file, corrupt `.basproj`,
   OxVba host crash, filesystem read failure, unicode decode
   failure — all surface a recoverable state, no panic.
2. **Empty-state polish.** Welcome gains a proper new-project
   wizard (target-kind picker, entry-point picker), a capability
   shortcut to re-run the probe, and a tour command.
3. **Mouse parity sweep.** Every advertised mouse affordance
   (focus clicks, selection drag, scroll, hover reveals) is
   exercised and has a keyboard analog.
4. **Motion / density audit.** Against the uxpass §50 policy, any
   animation that violates the density principle is trimmed.

### Sign-off gate

W110 closes when:
- No known panic paths remain (fuzzed inputs don't crash).
- Every advertised affordance on every scene dispatches or surfaces
  honest feedback.
- The full `wtd` regression suite passes cleanly against all three
  width classes and both palette profiles.

## Beads

### W110-B01 — Error-recovery sweep

**Feature.**

- **Goal.** Every panic-capable user input produces a popover and a
  recoverable state instead of a crash. Missing project file,
  corrupt `.basproj`, OxVba crash, filesystem read failure —
  all surface honestly.
- **Design.** Audit `unwrap()` / `expect()` / `panic!()` in
  `src/shell/` and path-handling. Replace with `Result` + popover
  installation. Host-crash recovery path attempts a reconnect once,
  then surfaces.
- **Tests.** Unit: each failure path installs a popover and leaves
  a recoverable `ShellState`. `wtd` journeys covering each recovery
  path.

### W110-B02 — New-project wizard

**Feature.**

- **Goal.** `Ctrl+Shift+N` opens a small wizard: project name,
  target kind (Exe / Library / Addin / ComServer / ComExe /
  HostModule), entry-point pattern. Submit scaffolds and mounts.
- **Design.** Extend the `Ctrl+N` minimal scaffold into an overlay
  with field inputs. Reuses the palette overlay infra with form
  semantics.
- **Tests.** Unit: form validation. `wtd` journey:
  `tests/wtd/journey_new_project_wizard.rs` drives the form and
  asserts the resulting project mounts.

### W110-B03 — Tour command

**Feature.**

- **Goal.** A palette entry "Take the tour" drives the user
  through each scene with a 3-step narration overlay: "This is the
  Editor. This is the Inspector. This is the Lower Surface." Each
  step advances on Enter, Esc exits.
- **Design.** New `TourState` on runtime; overlay rendering +
  step list. Pure UX; no model changes beyond adding the Msg.
- **Tests.** Unit: step progression. `wtd` journey: drive the tour
  to completion.

### W110-B04 — Mouse parity sweep

**Feature.**

- **Goal.** Every scene has a mouse affordance consistent with the
  uxpass §30 scene catalogue. Every advertised mouse affordance is
  tested.
- **Design.** A checklist per scene. Where the affordance doesn't
  fire, wire it. Where it has no keyboard analog, add one.
- **Tests.** `wtd` journeys per scene exercising at least one
  mouse affordance.

### W110-B05 — Motion / density audit

**Feature.**

- **Goal.** The shell has no animation or transition that violates
  the density principle from uxpass §50. Any progress spinner,
  fade, or slide that does is retired or rescoped.
- **Design.** Grep every animation call site; cross-check against
  §50 policy; trim.
- **Tests.** Unit: the audit is itself a checklist with citations
  to §50 — no automated test required beyond visual inspection.

### W110-B06 — Regression suite lock

**Feature (infrastructure).**

- **Goal.** The full `cargo test --features wtd` suite passes
  cleanly at the three canonical widths and both palette profiles.
  CI reports any new scenario added by any bead. The suite is the
  stable definition of shell correctness from here on.
- **Design.** CI config extension: run `wtd` at 100x30, 120x40,
  160x50. Results uploaded as artefacts.
- **Tests.** The suite itself.
- **Evidence.** A clean green CI run across all three widths.

## Out-of-scope

- **Pre-release packaging / installer.** Not a W110 concern.
- **Telemetry.** OxIde does not phone home.
- **Non-Windows platforms.** W110 locks Windows behaviour; porting
  starts as a new workset post-W110.
