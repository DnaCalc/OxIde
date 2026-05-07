# Third-Party Research And Licenses

Status: `first_pass_ledger`
Date: 2026-05-07

## Purpose

This ledger tracks third-party systems considered for the GUI/editor pivot.

The goal is MIT-compatible production code without licensing surprises. The project is not dogmatic, but license posture must be deliberate.

## Rules

1. Record whether a project is a dependency candidate, reference only, or inspiration only.
2. Prefer MIT/Apache-compatible dependencies.
3. Do not copy restrictive-license implementation code into OxIde.
4. Prefer shared DNA Calc implementations over third-party dependencies when that gives a cleaner long-term system.

## Initial Ledger

| Project | Use | License posture | Notes |
|---|---|---|---|
| Leptos | dependency candidate | permissive ecosystem; verify exact crate licenses before adoption | primary GUI candidate, aligned with DnaOneCalc |
| Tauri | dependency candidate | verify exact crate licenses before adoption | desktop host candidate |
| CodeMirror 6 | reference or dependency candidate | permissive; verify package licenses if used | strong browser-editor reference |
| Monaco | reference or dependency candidate | permissive; verify package licenses if used | large TypeScript-centric editor |
| Zed | reference only | editor code license requires care | useful architecture/product reference, not primary substrate |
| GPUI | reference or native-only candidate | more permissive than Zed editor; verify before use | native UI reference, not browser-first |
| Ropey/Crop | dependency candidates | verify crate licenses | text-buffer candidates |
| Tree-sitter | dependency candidate only if needed | verify grammar and crate licenses | avoid unless OxVba integration warrants it |
