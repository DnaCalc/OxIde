# Workset W230 — Save, Reload, And Session Restore

## Ambition

The GUI has honest document lifecycle behavior: dirty state, save, reload, revert, and session restore work where the host capability profile permits them.

## Dependencies

- W220 — editable module and diagnostics.
- Capability profile model sufficient to distinguish browser-limited and filesystem-capable hosts.

## Design

Document lifecycle must be capability-aware. The same UI must not pretend browser-only mode and desktop filesystem mode have identical persistence power.

Likely implementation lanes:

1. dirty-state model and command availability,
2. save/reload/revert service seam,
3. host capability disabled reasons,
4. session restore model for open project/document/cursor state,
5. browser and desktop-path test strategy.

## Beads

This scaffold is not execution-ready. Expand before implementation.

Expected bead lanes:

1. W230-B01 — dirty-state and lifecycle commands.
2. W230-B02 — save/reload/revert capability seam.
3. W230-B03 — browser-limited persistence honesty.
4. W230-B04 — session restore model and smoke.
5. W230-B05 — acceptance scenarios for supported and unsupported persistence.

## Out-of-scope

- Multi-project workspace restore.
- Advanced conflict resolution.
- Cloud sync or arbitrary host persistence systems.
