# W060 Direct OxVba Semantic Editing Integration

Status: `planned`
Date: 2026-04-02

## 1. Purpose
This workset ports and deepens the direct OxVba host-session integration into
the rebuilt shell.

Its scope is:
1. direct `HostWorkspaceSession` integration,
2. diagnostics while editing,
3. symbols, hover, and completions in rebuilt surfaces,
4. migration of semantic tests from the prototype where still valuable.

## 2. Salvage Rule
This workset should aggressively reuse:
1. direct host-session loading patterns,
2. document-id mapping patterns,
3. unsaved-text semantic update patterns,
4. high-value semantic tests.

It should not reuse:
1. prototype shell assumptions,
2. raw `:` command UX,
3. one-buffer application structure.

## 3. Closure Condition
This workset closes when:
1. the rebuilt shell drives direct semantic editing through OxVba,
2. diagnostics are first-class while editing,
3. at least the intended semantic surfaces are ported cleanly,
4. semantic behavior is validated by migrated or replacement tests.

## 4. Initial Epic Lanes
1. host-session adapter port
2. diagnostics UX
3. symbols and hover/completion UX
4. semantic test migration
