# Handoff — DNA Calc GUI Pivot Coordination

Status: `handoff_note`
Date: 2026-05-07

## Purpose

This note captures cross-repo implications discovered while planning the OxIde GUI pivot.

The OxIde repo-scoped agent may read sibling repos but may only write inside OxIde. Changes below require human coordination or separate repo-scoped runs.

## Candidate Cross-Repo Coordination Items

1. OxVba shared host/capability types
   - Determine whether host capability profiles and runtime-location concepts belong in OxVba or a shared DNA Calc crate.
   - Avoid duplicating these as long-lived OxIde-only enums.

2. OxVba serializable host protocol DTOs
   - Review existing `oxvba-web-host` DTOs for reuse or relocation.
   - Prefer shared authoritative request/event packets over OxIde-local copies.

3. DnaOneCalc embedded IDE consumption
   - Decide dependency direction for consuming OxIde UI/editor/bridge components.
   - Avoid circular repo dependencies.

4. Shared DNA Calc design system
   - Explore whether DnaOneCalc and OxIde should share Leptos design tokens or component primitives.

5. Windows native COM capability service
   - Determine whether the COM-capable native runtime service belongs in OxVba, a shared host crate, or host-specific crates.

## Rule

Prefer simple coordinated cross-repo changes over compatibility bridges inside OxIde when the project-wide final design is cleaner.
