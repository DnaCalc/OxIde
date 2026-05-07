# OxIde GUI Pivot Codebase Review

Status: `first_pass_review`
Date: 2026-05-07

## Purpose

This note records how the current codebase should be approached during the GUI pivot.

The main rule is:

```text
Bias strongly toward new implementation.
Use current code as evidence and behavior reference, not as the GUI foundation.
```

## First-Pass Classification

| Area | Treatment | Notes |
|---|---|---|
| `src/main.rs` | park with TUI | TUI entrypoint |
| `src/bin/oxide-uxlab.rs` | park / inspire GUI lab | useful scenario-lab pattern, TUI runtime |
| `src/shell/view.rs` | park only | terminal renderer |
| `src/shell/firehorse_design.rs` | park / inspire GUI lab | design-screen selector pattern useful |
| `src/shell/state.rs` | rewrite from behavior | useful editor/session ideas, too TUI-shaped |
| `src/shell/model.rs` | rewrite from behavior | useful command/update concepts, monolithic TUI model |
| `src/shell/session.rs` | rewrite/extract cautiously | project/document projection evidence |
| `src/shell/oxvba.rs` | rewrite into adapter | strong OxVba seam proof, not final architecture |
| `src/shell/project_actions.rs` | rewrite into services/adapters | useful behavior, avoid UI-level filesystem/platform coupling |
| `src/shell/session_store.rs` | rewrite | APPDATA/local-desktop assumptions need capability-aware persistence |
| `src/shell/uxlab/*` | park and mine for scenarios | useful scenario catalogue/audit approach |
| `tests/wtd/*` | parked TUI regression suite | opt-in for TUI, not GUI default |

## Cross-Repo Rule

When current OxIde code duplicates a concept that belongs in OxVba, DnaOneCalc, Foundation, or another DNA Calc repo, prefer a coordinated handoff or upstream/shared change over preserving a local copy.

This repo-scoped agent may only write inside OxIde, so sibling-repo changes must be recorded as handoffs for external coordination.
