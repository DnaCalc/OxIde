# Thin-Slice VBA Workflow

This sample is the smallest workflow that fits the current `OxIde` thin slice:

- one active document at a time
- edit/save through the editor surface
- build/run through the `OxVbaServices` seam
- execution results surfaced in the `OxVba Output` pane

## Files

- `examples/thin-slice/Module1.bas` - editable VBA module
- `examples/thin-slice/ThinSliceHello.basproj` - project file used by `:build`
  and `:run`

## Preconditions

- `../OxVba` exists relative to this repo, or `OXVBA_DIR` points at an
  `OxVba` checkout
- `cargo` is available

## Happy Path

Start `OxIde` on the module so the editor opens directly on VBA source:

```bash
cargo run -- examples/thin-slice/Module1.bas
```

Inside `OxIde`:

1. Make a small edit in `Module1.bas`.
2. Save with `Ctrl-S` or `:write`.
3. Open the project file with `:open examples/thin-slice/ThinSliceHello.basproj`.
4. Run `:build`.
5. Confirm the footer reports a successful build.
6. Check the `OxVba Output` pane for:
   - `Action: Build`
   - `Target: .../examples/thin-slice/ThinSliceHello.basproj`
   - `Success: yes`
7. Run `:run`.
8. Confirm the footer reports a successful run and the output pane updates to
   `Action: Run`.

This is intentionally a little manual. The current thin slice has a single
active `DocumentSession`, so project build/run works by making the `.basproj`
file the active document after saving module edits.

## Output-Pane Proof

To verify that compile errors reach the output pane:

1. Re-open the module with `:open examples/thin-slice/Module1.bas`.
2. Introduce a syntax error, for example:

```vb
Public Sub Main()
    Dim answer As Integer
    answer = 40 +
End Sub
```

3. Save the broken module.
4. Open `examples/thin-slice/ThinSliceHello.basproj` again.
5. Run `:build`.
6. Confirm the footer reports build failure and `OxVba Output` shows the
   compiler error text under `Stderr`.

Restore the valid module text below before the next successful run.

## Valid Module Text

```vb
Attribute VB_Name = "Module1"

Option Explicit

Public Sub Main()
    Dim answer As Integer
    answer = 40 + 2
End Sub
```
