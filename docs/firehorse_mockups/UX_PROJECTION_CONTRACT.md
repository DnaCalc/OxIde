# Fire Horse UX Projection Contract

Status: current W039 contract
Type: doctrine
Date: 2026-04-28

## Purpose

This document is the contract between the Fire Horse visual direction,
the W038 UX lab, and W039 terminal proof code.

The renderer should be buildable from this document without re-reading
the HTML mockup. The contract is intentionally one-way:

```text
Shell state, lab fixtures, or OxVba-shaped seam data
        -> FireHorseProjection
        -> terminal-cell render
        -> WTD capture / golden
```

There is no reverse path from projection into project truth, editor
truth, command dispatch, or VBA semantics.

## Do Not Own VBA Meaning

Fire Horse projection types describe what the IDE shows. They do not
become the owner of VBA meaning.

Rules:

- OxVba owns VBA/project meaning.
- OxIde owns workflow, presentation, editor state, command policy, and
  when/how OxVba results are surfaced.
- W039 fixture data must be named after seam concepts, not arbitrary UI
  copy.
- Missing seam data is represented explicitly as unavailable data with a
  reason; it is not guessed.
- W039 action ids are display/test contracts only until W090 implements
  the real action registry.

## Root Projection

Rust-like sketch:

```rust
pub struct FireHorseProjection {
    pub scenario_id: &'static str,
    pub identity: IdentityRailProjection,
    pub project_spine: Option<ProjectSpineProjection>,
    pub code_canvas: CodeCanvasProjection,
    pub context_dock: Option<ContextDockProjection>,
    pub activity_deck: ActivityDeckProjection,
    pub key_rail: KeyRailProjection,
    pub overlay: Option<OverlayProjection>,
    pub theme: ThemeProjection,
    pub terminal_fit: Option<TerminalFitProjection>,
    pub layout: LayoutPosture,
}

pub enum LayoutPosture {
    Launchpad,
    Editing,
    CommandLens,
    RunLane,
    DebugCockpit,
    ConsoleFit,
    CompactFocus,
}

pub enum ThemeProjection {
    GraphiteEmber,
    PaperEmber,
}

pub struct UnavailableProjection {
    pub source: &'static str,
    pub reason: &'static str,
}
```

Every scenario must carry enough data for Identity Rail, Activity Deck,
and Key Rail. Compact Focus may omit Project Spine and Context Dock, but
must keep explicit dock affordances in the Key Rail or source lens.

## Surface Contracts

### Identity Rail

Owns:

- product mark;
- project/session label;
- scene posture;
- target/run/debug summary;
- dirty summary;
- cursor position where relevant;
- console capability posture where relevant.

Must not own:

- canonical project model;
- focus routing;
- command dispatch.

```rust
pub struct IdentityRailProjection {
    pub product: &'static str,
    pub workspace_label: String,
    pub scene: ScenePosture,
    pub target: Option<String>,
    pub health: Vec<StateBadgeProjection>,
    pub cursor: Option<CursorProjection>,
}
```

### Project Spine

Owns user-facing project rows and their display badges.

Must not own `.basproj` truth or module membership.

```rust
pub struct ProjectSpineProjection {
    pub posture: SpinePosture,
    pub rows: Vec<ProjectSpineRowProjection>,
}

pub struct ProjectSpineRowProjection {
    pub label: String,
    pub kind: ProjectItemKind,
    pub depth: u8,
    pub active: bool,
    pub dirty: bool,
    pub badges: Vec<StateBadgeProjection>,
    pub seam_ref: Option<ProjectSeamRef>,
}
```

### Code Canvas

Owns visible source lines, markers, selection, lens placement, and
execution/diagnostic highlighting.

Must not own text editing semantics, symbol meaning, or semantic query
logic.

```rust
pub struct CodeCanvasProjection {
    pub document_label: String,
    pub language: &'static str,
    pub lines: Vec<SourceLineProjection>,
    pub lens: Option<SourceLensProjection>,
    pub execution_line: Option<u32>,
    pub selection: Option<SourceRangeProjection>,
}

pub struct SourceLineProjection {
    pub number: u32,
    pub text: String,
    pub markers: Vec<GutterMarkerProjection>,
    pub semantic_spans: Vec<SemanticSpanProjection>,
}

pub struct SourceLensProjection {
    pub anchor: SourceRangeProjection,
    pub title: String,
    pub body: Vec<String>,
    pub actions: Vec<ActionHintProjection>,
    pub source: SeamSourceProjection,
}
```

### Context Dock

Owns current contextual cards: diagnostics, symbol detail, run status,
call stack, locals, watches, or selected reference detail.

Must not become a generic dashboard.

```rust
pub struct ContextDockProjection {
    pub title: String,
    pub cards: Vec<ContextCardProjection>,
}

pub enum ContextCardProjection {
    Diagnostic(MockDiagnosticProjection),
    Symbol(MockSymbolProjection),
    RunStatus(RunStatusProjection),
    CallStack(Vec<StackFrameProjection>),
    Locals(Vec<LocalValueProjection>),
    Watches(Vec<WatchProjection>),
    Unavailable(UnavailableProjection),
}
```

### Activity Deck

Owns task surfaces: Problems, Output, Run Timeline, Immediate,
References, and Watch/Trace.

Must not degrade to an unstructured dump as the only source of truth.

```rust
pub struct ActivityDeckProjection {
    pub posture: DeckPosture,
    pub active: ActivityKind,
    pub tabs: Vec<ActivityTabProjection>,
    pub rows: Vec<ActivityRowProjection>,
}
```

### Key Rail

Owns the visible key hints for the current posture.

Must not dispatch commands. Rows reference action ids only.

```rust
pub struct KeyRailProjection {
    pub hints: Vec<ActionHintProjection>,
    pub no_wrap: bool,
}

pub struct ActionHintProjection {
    pub label: String,
    pub binding: KeyBindingProjection,
    pub action_id: &'static str,
    pub enabled: bool,
    pub disabled_reason: Option<String>,
    pub display_only_reason: Option<String>,
}
```

### Overlay

Owns Command Lens and future picker projection state.

Must not become an independent command registry.

```rust
pub enum OverlayProjection {
    CommandLens(CommandLensProjection),
}

pub struct CommandLensProjection {
    pub filter: String,
    pub selected_action_id: &'static str,
    pub rows: Vec<CommandRowProjection>,
    pub preview: CommandPreviewProjection,
    pub footer_hints: Vec<ActionHintProjection>,
}

pub struct CommandRowProjection {
    pub action_id: &'static str,
    pub label: String,
    pub binding: Option<KeyBindingProjection>,
    pub enabled: bool,
    pub disabled_reason: Option<String>,
    pub preview: CommandPreviewProjection,
}
```

### Terminal Fit

Owns capability-shaped display rows and recommendations.

Must not perform real terminal probing. W100 owns probing.

```rust
pub struct TerminalFitProjection {
    pub summary: String,
    pub rows: Vec<TerminalFitRowProjection>,
}

pub struct TerminalFitRowProjection {
    pub signal: &'static str,
    pub result: FitResult,
    pub detail: String,
    pub recommendation: String,
}
```

## OxIde State Mapping

| Fire Horse surface | OxIde owner | Later workset |
| --- | --- | --- |
| Identity Rail | `OxIdeShell` / shell runtime state | W039 proof, W110 polish |
| Project Spine | `ProjectSession` presentation + Explorer state | W040 |
| Code Canvas | `EditorSurface` + `DocumentSession` + semantic projection | W050 / W060 |
| Context Dock | Inspector mode state | W060 / W070 / W080 |
| Activity Deck | Lower Surface mode state | W070 / W080 |
| Key Rail | status-line hint, later `ActionRegistry` | W090 |
| Command Lens | palette overlay, later `ActionRegistry` view | W090 |
| Run Lane | execution event projection | W070 |
| Debug Cockpit | debug scene/projection | W080 |
| Console Fit | terminal capability state | W100 |
| Compact Focus | layout policy / width adaptation | W100 / W110 |

## OxVba Seam Mapping

| UX data | Seam source |
| --- | --- |
| Project modules, forms, classes, references, targets | `ProjectSession` over OxVba project truth |
| Document identity and active source text | `DocumentSession` mapped to OxVba `DocumentId` |
| Diagnostics | `HostWorkspaceSession::diagnostics` or current equivalent |
| Hover/source lens | `HostWorkspaceSession::hover` |
| Definition location | `HostWorkspaceSession::goto_definition` or current equivalent |
| References | `HostWorkspaceSession::references` |
| Symbols | document/workspace symbol APIs |
| Completion rows | completion API |
| Run timeline | current `WebHostEvent` stream, later typed run event seam |
| Immediate results | current/future evaluate API from OxVba runtime |
| Call stack, locals, watches, stepping | W080 audit of OxVba debug contract |
| Generated-code tagging | OxVba provenance surfaced through OxIde formatting |
| Terminal capability rows | W100 terminal capability probe result |

## Fixture Naming Rules

Fixture structs use seam-shaped names:

```rust
pub struct MockDiagnosticProjection {
    pub document_id: String,
    pub range: SourceRangeProjection,
    pub severity: DiagnosticSeverity,
    pub code: String,
    pub message: String,
    pub provenance: DiagnosticProvenanceProjection,
}

pub struct MockSymbolProjection {
    pub document_id: String,
    pub range: SourceRangeProjection,
    pub kind: SymbolKind,
    pub name: String,
    pub detail: String,
    pub provenance: SymbolProvenanceProjection,
}

pub struct MockRunEventProjection {
    pub target_id: String,
    pub step: RunStepKind,
    pub status: RunStepStatus,
    pub message: String,
    pub emitted_at_ms: u64,
}
```

Avoid fixture names like `SidebarString`, `PrettyProblem`, or
`OrangeCard`. The source concept should be visible in the type name.

## Action Id Matrix

Action ids are stable display/test contracts for W039. Dispatch remains
owned by existing model code until W090.

| Visible command or key | Binding | Action id | Applies to | Notes |
| --- | --- | --- | --- | --- |
| Open Project | `Ctrl+O` | `project.open` | Launchpad, Command Lens, Focus | W040 implements behavior. |
| Open selected recent/project row | `Enter` | `project.open_selected` | Launchpad | Selection-driven variant. |
| Create Project | `Ctrl+N` | `project.create` | Launchpad, Command Lens | W040 implements behavior. |
| Clone / Import | `Ctrl+Shift+O` | `project.import` | Launchpad | Future project import. |
| Quit | `Ctrl+Q` | `app.quit` | Launchpad | Existing shell policy may differ until W090. |
| Console Fit | `F10` | `app.console_fit` | Global, Launchpad, Focus | W100 implements behavior. |
| Command Lens | `F6` | `command.lens.open` | Global | Opens overlay in product work. |
| Type to filter | text input | `command.filter.update` | Command Lens | Overlay-local input. |
| Run selected command | `Enter` | `command.execute_selected` | Command Lens | Preview row supplies selected action id. |
| Focus preview | `Tab` | `command.preview.focus` | Command Lens | Overlay-local focus move. |
| Alternate command | `Ctrl+Enter` | `command.execute_alternate` | Command Lens | Uses selected row alternate. |
| Close overlay | `Esc` | `overlay.close` | Command Lens | Cascades before scene return. |
| Save | `Ctrl+S` | `editor.save` | Editing | W050 implements behavior. |
| Semantic Lens / help | `F1` | `semantic.hover` | Editing, diagnostics | Pin state is display-only in W039. |
| Pin semantic lens | `F1` | `semantic.lens.pin` | Editing lens | Future lens interaction. |
| Go Definition | `F12` | `semantic.goto_definition` | Editing | W060 implements behavior. |
| References | `Shift+F12` | `semantic.references` | Editing, source lens | W060 implements behavior. |
| Completion | `Ctrl+Space` | `semantic.completion` | Editing | W060 implements behavior. |
| Quick Fix | `Ctrl+.` | `diagnostic.quick_fix` | Problems/diagnostics | W060/W090 integration. |
| Pin Watch | `Ctrl+W` | `watch.pin` | Editing, Debug | W080 implements behavior. |
| Run Project | `F5` | `run.start` | Editing, Command Lens | W070 implements behavior. |
| Run With... | `Ctrl+F5` | `run.start_with` | Command Lens | Target/profile choice. |
| Rerun | `F5` | `run.rerun` | Run Lane | Valid after a run target exists. |
| Stop Run | `F8` | `run.stop` | Run Lane, Command Lens | Disabled reason required when no active run. |
| Configure Target | `Ctrl+K T` | `target.configure` | Command Lens | W040/W070 target model. |
| Switch Target | `Ctrl+K Shift+T` | `target.switch` | Command Lens | W040/W070 target model. |
| Return To Edit | `Esc` | `scene.return_edit` | Run, Debug, Console Fit | Scene-level return. |
| Immediate | `Ctrl+G` | `immediate.focus` | Run, Debug | W070/W080. |
| Toggle Breakpoint | `F9` | `debug.breakpoint.toggle` | Editing, Debug | W080. |
| Continue | `F5` | `debug.continue` | Debug | Debug context wins over run. |
| Step | `F8` | `debug.step` | Debug | Debug context wins over stop. |
| Step Out | `Shift+F8` | `debug.step_out` | Debug | W080. |
| Focus Project | `Alt+1` | `focus.project` | Editing, Compact Focus | Opens temporary dock in compact mode. |
| Focus Code | `Alt+2` | `focus.code` | Editing | Full focus map. |
| Focus Context | `Alt+3` | `focus.context` | Editing, Compact Focus | Opens temporary dock in compact mode. |
| Focus Activity | `Alt+4` | `focus.activity` | Editing, Compact Focus | Opens temporary dock in compact mode. |
| Open workspace chord | `Ctrl+K O` | `project.open` | Chord help | Alias of open project. |
| References chord | `Ctrl+K R` | `semantic.references` | Chord help | Alias of references. |
| Diagnostics chord | `Ctrl+K D` | `diagnostics.focus` | Chord help | Focus Problems/diagnostics. |
| Keymap chord | `Ctrl+K K` | `keymap.open` | Chord help | W090. |
| Open detailed Console Fit report | `Enter` | `console_fit.report.open` | Console Fit action row | W100. |
| Rerun Console Fit checks | `Enter` | `console_fit.rerun` | Console Fit key rail | Scenario may choose either report or rerun as active. |

No visible command row in the refined mockups should appear without an
action id. If a future mockup includes purely explanatory text in a
key-like style, the renderer must set `display_only_reason`.

## Scenario Requirements

| Scenario id | Required projection posture | Default viewport |
| --- | --- | --- |
| `firehorse-launchpad-standard` | Launchpad | Standard |
| `firehorse-editing-lens-standard` | Editing | Standard |
| `firehorse-command-lens-standard` | CommandLens with `OverlayProjection::CommandLens` | Standard |
| `firehorse-run-lane-standard` | RunLane | Standard |
| `firehorse-debug-cockpit-standard` | DebugCockpit | Standard |
| `firehorse-console-fit-light` | ConsoleFit with `ThemeProjection::PaperEmber` | Standard |
| `firehorse-focus-compact` | CompactFocus | Compact |

Every scenario registers through the W038 lab registry under
`suite = "firehorse"`.

## Renderer Test Contracts

W039 renderer tests should assert region and action contracts by name,
not by row position:

- Identity Rail visible with the expected scene posture.
- Required regions named in the test are present.
- Key Rail contains required action ids.
- Disabled command rows include a reason.
- Compact Focus hides Project Spine and Context Dock but exposes
  `focus.project`, `focus.context`, and `focus.activity`.
- Console Fit rows include text labels for pass/warn/fail states; color
  alone is insufficient.
