# OxIde TUI Design 2026: Fire Horse Console IDE

Status: proposal
Type: doctrine
Date: 2026-04-24

This is a fresh design document for the OxIde TUI IDE. It keeps roughly
the scope of `docs/DESIGN_TUI.md` - shell frame, region roles, scenes,
width adaptation, focus, overlays, visual language, and mockup targets -
but rethinks the user experience from a 2026 console-first perspective.

It is not a patch over the current plan. It is a competing direction:
OxIde should feel like a contemporary terminal-native IDE for VBA work,
not like a careful recreation of older console software and not like a
desktop IDE squeezed into monospace cells.

## 1. Design Verdict On The Current Plan

The current TUI plan has the right discipline: project awareness,
terminal honesty, a command registry, a strong editor center, and
explicit capability degradation. The weak point is its emotional and
visual posture. It reads as a useful pane layout, not yet as a product
with a memorable operating feel.

The current plan tends toward:

- a fixed rectangular shell where every state is another pane
  arrangement;
- a strong concern for avoiding mistakes, but less concern for making
  the product feel alive;
- command surfaces that are correct but plain;
- an Inspector / Lower Surface split that can push semantic meaning away
  from the code instead of making the source itself feel intelligent;
- a dark palette that is competent but close to many other developer
  dashboards;
- Empty and Run states that are functional but not yet signature
  moments.

The replacement direction should keep the rigor and raise the ambition.

## 2. North Star

OxIde should feel like a **living instrument for VBA projects**:

- code remains the primary surface;
- project state is always visible but never heavy;
- semantic intelligence appears where the user's eye already is;
- build, run, and debug feel like staged workflows, not just log panes;
- command discovery feels fast, rich, and trustworthy;
- the terminal surface is used as a modern medium with rhythm, color,
  density, and motion.

The 2026 "Fire Horse" reference should not become theme decoration.
Use it as a product temperament:

- forward motion;
- heat where the system is active;
- bright state changes;
- confident contrast;
- a shell that is responsive and energetic without becoming noisy.

## 3. Contemporary Terminal Cues

The design should learn from modern terminal work without cloning any
single tool.

- Modern TUI frameworks such as Textual and Ratatui show that terminal
  UIs can be structured, styled, responsive applications rather than
  curses-era forms.
- Zellij's command panes and layout automation show that build/run/test
  commands can be treated as first-class workspace objects instead of
  anonymous terminal output.
- Modern terminal emulators such as Ghostty expose richer protocols:
  truecolor, synchronized rendering, advanced keyboard protocols,
  light/dark notifications, ligatures, and image/graphics protocols.
  OxIde must not depend on these features, but it should benefit from
  them when present.
- Contemporary terminals such as Warp have normalized IDE-like command
  editing, command palettes, themes, and reusable workflows inside the
  terminal. OxIde should apply that standard specifically to VBA
  projects, not to general shell commands.

External references reviewed:

- Textual: https://textual.textualize.io/
- Ratatui: https://ratatui.rs/
- Zellij features: https://zellij.dev/features/
- Ghostty terminal features: https://ghostty.org/docs/features
- Warp modern terminal UX: https://www.warp.dev/windows-terminal
- 2026 Fire Horse reference: https://www.si.edu/spotlight/lunar-year-horse

## 4. Product Identity

### 4.1 Positioning

OxIde is not:

- a terminal-themed VS Code;
- a retro BASIC environment;
- a Vim clone with VBA plugins;
- a generic project dashboard.

OxIde is:

- a terminal-native IDE;
- editor-fast but IDE-shaped;
- project-aware by default;
- VBA-compatible in workflow memory;
- direct-hosted over OxVba semantics;
- visually modern, dense, and kinetic.

### 4.2 Feeling

The shell should feel:

- warm under active work;
- quiet during reading;
- sharp during command entry;
- visibly stateful during build/run/debug;
- composed at small sizes;
- premium at large sizes.

The best description is **"console cockpit, not console form."**

## 5. Core UX Principles

### P1. The Source Is The Stage

The editor is not just a text pane. It is where semantic, diagnostic,
execution, and debug state should first appear.

Use the Context Dock and Activity Deck for expansion and history. Use the
editor gutter, inline markers, near-cursor lenses, and source-adjacent
badges for immediate understanding.

### P2. Rails Over Boxes

Older console apps rely on everything being boxed. OxIde should rely on
rails, separators, edge-light, spacing, and color temperature.

Borders still matter, but the screen should not look like a grid of
containers. Strong boxes are reserved for overlays, focused drawers, and
dangerous confirmations.

### P3. Every State Has A Signature

Empty, Editing, Build, Debug, Command Lens, and Console Fit should each
have a recognizable composition and color temperature. A user should be
able to glance at the screen and know what kind of work is happening.

### P4. Command Discovery Is A Primary Surface

The command palette is not a fallback for forgotten keys. It is the
Command Lens: search, preview, execute, and learn.

Every action is discoverable there. Important commands expose:

- default binding;
- VBA-compatible binding where different;
- current availability;
- preview of what will change;
- recovery or undo notes when relevant.

### P5. Build And Debug Are Workflows, Not Panes

Run/debug state is not "bottom pane has output." It is a staged workflow:

1. prepare;
2. analyze;
3. build;
4. execute;
5. inspect result;
6. return or continue.

The UI should show that choreography.

### P6. Motion Is A State Cue

TUI motion is allowed, but only for state communication:

- build progress pulse;
- diagnostics count change;
- command lens open/close;
- debug step movement;
- background semantic analysis completion.

No motion while typing ordinary source text. No decorative animation in
the editor. Respect reduced-motion settings.

### P7. Terminal Capability Is Part Of The Product

The app owns the first-run terminal fit experience. A beautiful TUI that
silently breaks under a weak terminal is not beautiful.

The product should test:

- truecolor;
- Unicode line drawing;
- braille/block glyph width;
- mouse reporting;
- modified keyboard events;
- synchronized rendering support if detectable;
- terminal size;
- font quality for box drawing and ligatures.

### P8. VBA Muscle Memory Is Welcome, Not Binding

Default key choices should respect VBA IDE and Visual Studio memory when
that memory maps cleanly. Visual structure should remain terminal-native.

The product should say, by feel: "Your VBA habits still matter, but this
is a sharper instrument."

## 6. Shell Grammar

The proposed shell is not a four-band frame with static rectangles. It
is a **code canvas with rails and docks**.

```text
 OxIde  Payroll.basproj  target: win-console  clean        Ln 42 Col 17
---------------- project spine --+---------------- code canvas ----------------+-- context dock --+
  Modules                       > |  38  Public Sub Main()                      | Diagnostics      |
  Forms                           |  39      Dim answer As Integer              | 0 errors         |
  References                      |  40      answer = PriceFor(invoiceId)       | 2 warnings       |
  Targets                         |  41      Debug.Print answer                 |                  |
                                  |  42  End Sub                                | Symbol           |
                                  |                                            | Main             |
                                  |  [F1 lens]  PriceFor(invoiceId) As Currency |                  |
----------------------------------+--------------------------------------------+------------------+
 Activity Deck: Problems          2 warnings  last run: passed 3s ago
 F5 run  F6 Command Lens  Ctrl+S save  Ctrl+G Immediate  Alt+1 Project  Alt+2 Code  Alt+3 Context
```

### 6.1 Persistent Surfaces

The shell has six conceptual surfaces:

| Surface | Role | Default presence |
| --- | --- | --- |
| Identity Rail | Project, target, scene temperature, current editor state | Always |
| Project Spine | Project structure, modules, references, targets, open buffers | Expanded on standard/wide, collapsed on small |
| Code Canvas | Editor, gutter, source lenses, inline diagnostics, execution point | Always |
| Context Dock | Current semantic/debug/run context, details, quick actions | Adaptive |
| Activity Deck | Problems, output, run timeline, references, immediate, watch/trace | Adaptive |
| Key Rail | Live keys, chord state, selected command affordances | Always |

This is one more conceptual surface than the old spec, but fewer heavy
rectangles. The distinction is important: a rail may be one row or one
edge, not a framed panel.

### 6.2 Identity Rail

The top rail is one or two rows, not a tall bordered block by default.

It shows:

- product mark and project name;
- active target/profile;
- dirty/running/debugging state;
- editor position when code focus owns the screen;
- concise capability warning when relevant.

It does not show:

- focus region names;
- internal scene enum names;
- width class names;
- duplicate diagnostics counts already shown in the Activity Deck.

Example:

```text
 OxIde  Payroll.basproj  target: win-console  clean  Edit          Ln 42 Col 17
```

When running:

```text
 OxIde  Payroll.basproj  target: win-console  Run  build passed  runtime active
```

When debugging:

```text
 OxIde  Payroll.basproj  Debug  paused at Module1.Main:42  breakpoint hit
```

### 6.3 Project Spine

The Project Spine replaces the feel of a conventional Explorer pane.
It is still a tree, but it behaves like an instrument spine:

- modules, forms, classes, references, and targets are first-class
  groups;
- open buffers appear as small right-edge markers on tree rows;
- dirty files carry a warm marker;
- active run target is visible;
- references and generated helper modules are visibly different from
  user-owned modules.

The spine has three states:

| State | Width | Use |
| --- | --- | --- |
| Full | 24-34 cols | Standard and wide editing |
| Slim | 10-14 cols | Small terminals or focused code |
| Peek | Overlay strip | Temporary project navigation over code |

Slim state uses compact group initials and active-row labels. It must
still be keyboard readable; it is not an icon-only toy.

### 6.4 Code Canvas

The Code Canvas is the product center. It owns:

- source text;
- line numbers;
- gutter markers;
- dirty/saved state at buffer title or rail;
- cursor and selection;
- inline diagnostics;
- semantic lenses;
- execution point;
- breakpoint markers;
- live symbol breadcrumb.

The editor should not feel like a paragraph inside a bordered box. It
should feel like the main visual field. Use:

- muted line numbers;
- a thin active gutter rail;
- semantic color with restraint;
- source-adjacent popovers that respect code columns;
- no wrapping for source unless a deliberate soft-wrap mode is enabled.

The Code Canvas can host temporary lenses:

```text
  40  answer = PriceFor(invoiceId)
       ^^^^^^^^
       PriceFor(id As Long) As Currency
       defined in Pricing.bas:12   F12 go  Shift+F12 refs  F1 pin
```

The lens is not a generic floating card. It is anchored to source.

### 6.5 Context Dock

The Context Dock is a right-edge semantic shelf. It is not always a
permanent Inspector column.

It shows details for the thing the user is doing:

- selected symbol;
- diagnostics near cursor;
- current build target;
- run status;
- call stack / locals / watches in debug;
- reference details after a search.

It has three states:

| State | Behavior |
| --- | --- |
| Hidden | Key Rail and badges announce available context. |
| Shelf | 24-34 cols, quiet persistent context. |
| Focused Dock | 40-56 cols, interactive list/detail mode. |

The dock should be entered by explicit focus or command, not by forcing
the editor to surrender width at every moment.

### 6.6 Activity Deck

The Activity Deck is the bottom task surface. It should feel more like a
timeline and less like a spare log pane.

Modes:

- Problems;
- Output;
- Build Timeline;
- Immediate;
- References;
- Watch / Trace;
- Console Fit.

Deck states:

| State | Height | Use |
| --- | --- | --- |
| Rail | 1 row | counts, last result, selected deck mode |
| Compact | 6-10 rows | normal Problems / Output |
| Expanded | 35-45% screen | focused output, references, Immediate |
| Max | body takeover | long logs, trace, interactive console |

The deck rail is always understandable. Example:

```text
 Activity: Problems  0 errors  2 warnings  last run passed 3s ago
```

During run:

```text
 Run: analyze ok > build ok > execute active  stdout 12 lines  F8 stop
```

### 6.7 Key Rail

The Key Rail is the bottom row. It carries only live affordances.

It should also show chord state:

```text
 Ctrl+K ...   O open  R references  T target  Esc cancel
```

During Command Lens:

```text
 Type to filter  Enter run  Tab preview  Ctrl+Enter run alternate  Esc close
```

The rail must never wrap. If the terminal is too narrow, it drops lower
priority hints and exposes the full list through `F6` Command Lens.

## 7. Scene Designs

### 7.1 Empty: Launchpad

The Empty scene should be a signature first impression. It should not be
a plain list and should not be a marketing page.

Goals:

- open or create a project quickly;
- show recent work with meaningful project metadata;
- test terminal fit without shaming the user;
- communicate that this is an IDE, not a command wrapper.

Composition:

```text
 OxIde                                                           Console fit: good

  Recent work
  > Payroll.basproj             win-console   clean     last opened today
    Inventory.basproj           library       2 warn    last opened Friday
    Scratch.basproj             no target     draft     last opened Apr 18

  Start
    Open Project        Ctrl+O
    Create Project      Ctrl+N
    Clone / Import      Ctrl+Shift+O
    Console Fit         F10

  Terminal
    truecolor ok   glyphs ok   mouse ok   modified keys ok

 F6 Command Lens  Enter open  Ctrl+O open project  Ctrl+N create  Ctrl+Q quit
```

Design notes:

- Recent rows are dense and useful: project, target, health, recency.
- Console fit is visible but quiet when healthy.
- Start actions are rows, not boxed buttons.
- If terminal fit is poor, the Terminal section expands with specific
  fixes.

### 7.2 Editing: Source First

Editing is the default operating state.

Composition:

- Identity Rail: project, target, dirty state, position.
- Project Spine: full or slim.
- Code Canvas: dominant.
- Context Dock: shelf with Diagnostics and Symbol Summary, or hidden
  when clean.
- Activity Deck: rail by default, compact if problems exist.
- Key Rail: save/run/palette/focus hints.

Editing should make clean projects feel calm:

```text
 Activity: clean  last analysis 80ms  last run passed 3s ago
```

Editing should make problems specific without taking over:

```text
 Activity: Problems  1 error  2 warnings  nearest: Option Private Module...
```

The user should not have to leave the code to understand the nearest
diagnostic.

### 7.3 Semantic Focus: Lens, Not Scene

Do not surface "Semantic" as a scene label. Semantic work is a focus
state inside Editing.

Entry points:

- `F1` on identifier: source lens;
- `F12`: goto definition;
- `Shift+F12`: references deck;
- `Ctrl+Space`: completion;
- Command Lens actions.

Semantic Focus can use:

- near-cursor lens for one symbol;
- Context Dock shelf for details;
- Activity Deck for references and search results.

Composition:

```text
  42  total = PriceFor(invoiceId)
               ^^^^^^^^
       PriceFor(id As Long) As Currency
       Pricing.bas:12  7 refs  last analyzed 42ms
```

The old plan's `Semantic-Rich Editing` becomes a family of focus states,
not a mode the user has to name.

### 7.4 Build / Run: Run Lane

Build/Run should be one of the most attractive states in the product.
It is where OxIde proves it is a real IDE.

The Run Lane is a horizontal workflow summary above the Activity Deck or
inside its rail:

```text
 Run: prepare ok > analyze ok > build ok > execute active > result pending
```

When complete:

```text
 Run: prepare ok > analyze ok > build ok > execute ok > exit 0   1.2s
```

Composition:

- Identity Rail switches to warm active state.
- Code Canvas remains visible and marks the entry point.
- Context Dock shows Target and current step details.
- Activity Deck opens compact/expanded Output or Build Timeline.
- Key Rail shows rerun, stop, return, console, and palette.

Build output should be structured:

```text
 12:04:18  analyze   ok       28 modules
 12:04:19  build     ok       target win-console
 12:04:19  stdout    line     project run completed
 12:04:20  runtime   exit 0   1.2s
```

Raw terminal output can appear, but the first-class surface is an event
stream with origin, severity, time, and actionability.

### 7.5 Debug: Debug Cockpit

Debug is a distinct work posture. It should not look like Editing with a
different right pane.

Composition:

```text
 OxIde  Payroll.basproj  Debug  paused  Module1.Main:42  breakpoint hit
---------------- project spine --+---------------- code canvas ----------------+-- debug dock ----+
 Breakpoints                    > |  38  Public Sub Main()                      | Call Stack       |
  Module1:42 *                   |  39      Dim answer As Integer              | > Main           |
 Watches                         |  40      answer = PriceFor(invoiceId)       |   PriceFor       |
                                  |  41      Debug.Print answer                 | Locals           |
                                  |  42> End Sub                                | answer = 42      |
----------------------------------+--------------------------------------------+------------------+
 Debug Deck: Immediate            ? answer        42
 F5 continue  F8 step  Shift+F8 step out  F9 breakpoint  Ctrl+G Immediate  Esc edit
```

Debug-specific behavior:

- execution line has a strong warm gutter mark;
- breakpoints live in the gutter and in the Project Spine's debug group;
- Context Dock becomes Debug Dock: call stack, locals, watches;
- Activity Deck defaults to Immediate or Watch/Trace;
- source remains readable, not dimmed into a backdrop.

### 7.6 Command Lens

The Command Lens replaces the plain palette mental model.

It is an overlay with three columns when width permits:

```text
 +---------------------------- Command Lens -----------------------------+
 | > run target                                                       F5 |
 |                                                                       |
 | Actions                         Preview                               |
 | > Run Project        F5          Runs Payroll.basproj as win-console   |
 |   Run With...        Ctrl+F5     Choose target/profile before running  |
 |   Stop Run           F8          Disabled: no active process           |
 |   Configure Target   Ctrl+K T    Opens target editor                   |
 |                                                                       |
 | Recent commands                                                       |
 |   Open Project       Ctrl+O                                           |
 +-----------------------------------------------------------------------+
 Type to filter  Enter run  Tab preview  Ctrl+Enter alternate  Esc close
```

Command rows carry:

- label;
- binding;
- availability;
- command group;
- short preview;
- consequences.

Disabled commands stay visible when useful, but explain why they cannot
run.

The Command Lens is also the entry point for:

- go to file;
- go to symbol;
- go to line;
- switch target;
- console fit;
- keymap switch;
- recent workflows.

Use submodes inside the lens rather than spawning unrelated overlay
types unless the interaction genuinely differs.

### 7.7 Immediate: Dialog With The Running Project

The Immediate surface should feel native, not like a subprocess console.

It can live in the Activity Deck but has a distinct prompt:

```text
 Immediate  context: Module1.Main  runtime: paused
 ? answer
 = 42
 ? PriceFor(10)
 = 19.95
```

Rules:

- expression input uses editor-grade editing where possible;
- results are typed and copyable;
- history can be searched;
- values can be pinned to Watches;
- errors are diagnostics, not raw stack strings.

### 7.8 Console Fit

Console Fit is both first-run onboarding and a command the user can run
any time.

It should be attractive and practical:

```text
 Console Fit

  Color       truecolor ok        gradients disabled by preference
  Glyphs      line drawing ok     braille ok     block glyphs ok
  Keyboard    F-keys ok           modified keys ok
  Mouse       click ok            wheel ok       drag selection limited
  Render      sync unknown        using conservative redraw
  Size        132 x 42            premium layout available

  Recommended:
  - Use Windows Terminal or another truecolor-capable terminal.
  - Keep width at 120+ columns for the full IDE composition.
  - Enable a font with strong box drawing and programming ligatures.

 F6 Command Lens  Enter rerun checks  Esc return
```

Console Fit should not expose raw escape-sequence trivia unless the user
asks for details.

## 8. Width Strategy

OxIde should be designed for a realistic modern terminal with zoomed
text, not only for full-screen ultrawide screenshots.

| Class | Suggested cells | Composition |
| --- | --- | --- |
| Compact | 92-119 cols, 28+ rows | Slim Project Spine, Code Canvas, Activity Rail, Context as overlay/dock |
| Standard | 120-159 cols, 34+ rows | Full Project Spine, Code Canvas, Context Shelf, Activity Rail/Compact |
| Wide | 160+ cols, 40+ rows | Full Spine, wide Canvas, Context Dock, Activity Deck with richer timeline |
| Studio | 190+ cols, 45+ rows | Premium debug/run layouts, split source views, persistent references |

Below 92 columns or 24 rows, OxIde should switch to a guided compact
message with actions:

```text
 OxIde needs more room for the IDE shell.
 Current: 78 x 22. Recommended: 120 x 34.
 Press F10 for Console Fit, Ctrl+O to open anyway in Focus Mode, Ctrl+Q quit.
```

### 8.1 Focus Mode

Focus Mode is the compact answer to small terminals. It is not a
degraded layout pretending to be the full IDE.

Focus Mode shows:

- Identity Rail;
- Code Canvas;
- Activity Rail;
- Key Rail.

Project, Context, and Activity open as overlays or temporary docks.

### 8.2 Studio Mode

Studio Mode is the premium large-terminal layout.

It can add:

- split source views;
- persistent references beside source;
- wider Debug Dock;
- run timeline plus output side by side;
- richer semantic previews.

Studio Mode must be additive. No feature requires it.

## 9. Visual Language

### 9.1 Palette

Default dark theme: **Graphite Ember**.

| Token | Hex | Use |
| --- | --- | --- |
| Base | `#090B0F` | terminal background |
| Canvas | `#0E1117` | code canvas |
| Raised | `#151A22` | docks and overlays |
| Rail | `#11161F` | identity/activity/key rails |
| Border | `#2A303A` | separators and quiet borders |
| Text | `#E8EAED` | primary text |
| Muted | `#8B96A6` | secondary text |
| Ember | `#FF6B2C` | active run/debug heat, dirty state |
| Gold | `#FFC857` | warnings and pending work |
| Azure | `#45B7F5` | semantic intelligence, commands |
| Mint | `#5EE0A0` | success/pass/ready |
| Rose | `#FF5D7A` | errors and stops |
| Violet | `#9B8CFF` | references and navigation, used sparingly |
| Selection | `#26384A` | active row / selected range |

The theme must avoid becoming all orange. Ember is the Fire Horse accent,
not the whole palette.

### 9.2 Light Theme

A modern terminal IDE needs a credible light theme for offices,
projectors, and accessibility.

Light theme: **Paper Ember**.

| Token | Hex |
| --- | --- |
| Base | `#F7F5EF` |
| Canvas | `#FFFDF8` |
| Raised | `#ECE7DC` |
| Rail | `#F0ECE4` |
| Border | `#C9C1B4` |
| Text | `#1D232A` |
| Muted | `#68727F` |
| Ember | `#D9571E` |
| Gold | `#9A6700` |
| Azure | `#006D9C` |
| Mint | `#1F7A52` |
| Rose | `#C9344F` |
| Violet | `#6857C8` |
| Selection | `#DCEBFA` |

### 9.3 16-Color Fallback

The fallback theme preserves meaning, not exact hue:

- Ember -> bright red/yellow pair depending on background;
- Azure -> bright cyan;
- Mint -> bright green;
- Rose -> bright red;
- Gold -> yellow;
- Muted -> bright black / gray;
- Selection -> reverse video or low-intensity background.

Never rely on color alone. Pair color with:

- glyph marker;
- label;
- row position;
- severity word;
- gutter symbol.

### 9.4 Typography And Glyphs

Preferred glyph use:

- thin separators for quiet structure;
- block/half-block only for active rails and progress;
- braille sparingly for miniature traces or activity graphs;
- no emoji as required UI;
- no box-drawing art that becomes meaningless in fallback.

Suggested markers:

```text
 * dirty
 ! error
 ? warning / question
 > active row / execution line
 o breakpoint
 + added / created
 - removed / disabled
```

Unicode variants may enhance these markers when capability checks pass,
but ASCII variants are canonical.

### 9.5 Border Discipline

Use full boxes only for:

- Command Lens;
- focused modal workflows;
- confirmations;
- Console Fit;
- expanded Activity Deck;
- debug/run detail takeovers.

Use rails and separators for:

- Identity Rail;
- Project Spine;
- Code Canvas;
- Context Shelf;
- Activity Rail;
- Key Rail.

This is one of the biggest differences from earlier-era console apps.

### 9.6 Motion

Motion budget:

- max 8-12 visual updates per second for progress animations;
- no animation during source typing;
- no continuously animated decorative elements;
- synchronized rendering if available, conservative redraw otherwise;
- reduced motion disables pulses and replaces them with static markers.

Examples:

- Run Lane step changes flash Ember once then settle.
- Diagnostics count changes briefly brighten the Activity Rail.
- Command Lens opens with one-frame rail inversion, not a long slide.
- Debug step moves the execution marker with a short highlight trail if
  reduced motion is off.

## 10. Interaction Model

### 10.1 Keybinding Layers

Layer 1: direct keys

- `F5` run / continue;
- `F6` Command Lens;
- `F8` stop / step depending debug context;
- `F9` breakpoint;
- `F10` Console Fit;
- `Ctrl+S` save;
- `Ctrl+G` Immediate;
- `F1` help/lens;
- `F12` goto definition.

Layer 2: chords

- `Ctrl+K T` target;
- `Ctrl+K R` references;
- `Ctrl+K O` open workspace;
- `Ctrl+K D` diagnostics;
- `Ctrl+K K` keymap.

Layer 3: Command Lens

- every action;
- fuzzy labels;
- aliases;
- binding search;
- disabled reasons;
- previews.

Layer 4: VBA-compatible profile

- familiar alternatives where they map cleanly;
- visible in Command Lens;
- configurable without changing action identity.

### 10.2 Focus

Focus regions:

- Project Spine;
- Code Canvas;
- Context Dock when visible;
- Activity Deck when compact/expanded;
- Command Lens when open.

Identity Rail is display-only. Key Rail is display-only.

Direct focus:

- `Alt+1` Project;
- `Alt+2` Code;
- `Alt+3` Context;
- `Alt+4` Activity.

`Tab` cycles only through visible, actionable surfaces.

### 10.3 Mouse

Mouse support is coherent but not required.

Mouse should support:

- focus click;
- tree row selection;
- editor cursor placement;
- text selection where terminal permits;
- wheel scroll in spine, code, dock, and deck;
- drag resize between major docks;
- click status/action entries where unambiguous.

Mouse should not be required for:

- command discovery;
- run/debug;
- project open/create;
- navigation;
- fixes and diagnostics.

### 10.4 Command Preview

Commands that mutate project state should preview consequences:

```text
 Create Module
 Will add: Modules/Module2.bas
 Will update: Payroll.basproj
 Undo: remove created module before save
```

Commands that are unavailable should say why:

```text
 Stop Run
 Disabled: no active run process
```

This is how the TUI feels modern without adding web-style widgets.

## 11. Semantic UX

### 11.1 Diagnostics

Diagnostics appear in three layers:

1. gutter marker and underline in Code Canvas;
2. source lens for nearest/selected diagnostic;
3. Activity Deck Problems list for history and navigation.

The nearest diagnostic is always recoverable without leaving the editor:

```text
  12  Option Private Module
      ^^^^^^^^^^^^^^^^^^^^^
      error PMR-E... requires project/module-kind integration
      F1 details  Enter open problem  Ctrl+. quick fix
```

### 11.2 Symbols

Symbols should be projected as user-authored structure:

- procedures;
- variables when useful;
- classes/forms/modules;
- references;
- target entry points.

Do not surface generated helper names as peer project objects unless the
user asks for generated detail.

### 11.3 Completion

Completion is source-adjacent:

```text
  answer = Pri
           +-----------------------+
           | > PriceFor(id)        |
           |   Print               |
           |   Private             |
           +-----------------------+
```

The Context Dock can show detail for the selected completion. The popup
itself stays small and fast.

### 11.4 Quick Fixes

Quick Fix should be a Command Lens submode anchored to the diagnostic:

```text
 Quick Fix: Option Private Module
 > Add module kind to project metadata
   Remove Option Private Module
   Open project settings
```

Every fix row shows whether it edits source, project metadata, or both.

## 12. Project And Workspace UX

Project management should not feel like a file tree bolted to an editor.

The Project Spine owns project structure, but project workflows use
focused surfaces:

- Reference Manager;
- Target Switcher;
- Module/Form/Class creation;
- Project Settings;
- Recent Work;
- Workspace Health.

Target Switcher example:

```text
 Target Switcher
 > win-console     Exe      entry Module1.Main      default
   library         Dll      entry none              buildable
   tests           Exe      entry TestMain.Main     last failed
```

Reference Manager example:

```text
 References
 > VBA Runtime                  resolved
   Scripting.Dictionary         missing path       F1 fix
   Excel Object Library         optional
```

Project actions must name the project files they mutate.

## 13. Build, Run, And Debug UX

### 13.1 Run Timeline

The run timeline is structured and replayable.

Rows have:

- timestamp or relative time;
- phase;
- severity;
- message;
- optional action.

Example:

```text
 0.00s  prepare   ok       target win-console
 0.05s  analyze   warn     2 warnings
 0.21s  build     ok       emitted Payroll.exe
 0.33s  stdout    line     project run completed
 1.20s  runtime   exit 0
```

Actions:

- rerun;
- stop;
- copy output;
- open diagnostic;
- show generated detail;
- save log.

### 13.2 Debug State

Debug has three temperatures:

| State | Temperature | UI |
| --- | --- | --- |
| Running | Ember pulse | execution active, stop/interrupt prominent |
| Paused | Gold/Ember | current line strong, locals visible |
| Finished | Mint/Rose | result summary, return/rerun prominent |

Debug should preserve source continuity. The user should feel that the
program paused inside the same code they were editing.

### 13.3 Watches

Watches are not just a list. They are pinned semantic expressions.

Rows:

```text
 > answer                42            Integer
   invoice.Total         129.50        Currency
   PriceFor(invoiceId)   error         runtime not at call site
```

A watch can be created from:

- selected source;
- Immediate result;
- Command Lens;
- Debug Dock action.

## 14. Accessibility And Recovery

Accessibility is not a late polish pass.

Requirements:

- all color signals also have text/glyph signals;
- high contrast theme;
- reduced motion;
- ASCII fallback;
- focus order predictable;
- screen capture/golden tests for compact, standard, and wide layouts;
- no hidden state that exists only in hover;
- command alternatives for every mouse gesture.

Recovery requirements:

- unsaved changes always visible;
- run/debug cannot silently discard edits;
- overlays never corrupt backing screen;
- failed project load enters a recoverable Launchpad variant;
- terminal capability failures explain exact next actions.

## 15. Implementation Implications

This proposal can fit the existing architecture if implemented as a
sequence of design and shell refinements rather than a rewrite.

### 15.1 Reuse Existing Authority

Keep:

- OxVba owns project and semantic truth;
- OxIde owns presentation and workflow;
- no LSP detour inside OxIde;
- action registry as command source of truth;
- direct host session for semantic/editor integration;
- WTD journeys as UX evidence.

### 15.2 New Or Renamed Concepts

Map proposal language to existing implementation:

| Proposal | Existing / likely implementation |
| --- | --- |
| Identity Rail | Top bar, reduced to rail style |
| Project Spine | Explorer with richer state and slim/full modes |
| Code Canvas | Editor Area plus gutter/lens semantics |
| Context Dock | Inspector with hidden/shelf/focused states |
| Activity Deck | Lower Surface with rail/compact/expanded states |
| Key Rail | Status Line |
| Command Lens | Command Palette plus preview/detail model |
| Run Lane | Lower Surface Run/Build timeline plus top rail state |
| Console Fit | W100 terminal capability onboarding |

### 15.3 Workset Fit

Likely landing path:

- W038: provide the UX-lab substrate: scenario registry, fixed
  viewports, `oxide-uxlab --once`, WTD capture helper, then later replay,
  diff, bless, and interactive lab tooling.
- W039: prove the Fire Horse direction in terminal cells using
  projection contracts, fixture-backed scenarios, a read-only
  `ShellState` adapter, and WTD goldens.
- W050: strengthen Code Canvas basics: title/dirty, selection, find,
  gutter markers, source lenses plumbing.
- W060: implement semantic lens, diagnostics layers, references deck,
  completion source-adjacent popup.
- W070: replace plain output with Run Lane timeline and structured
  Activity Deck output.
- W080: implement Debug Cockpit with Debug Dock, execution line, watches,
  and Immediate integration.
- W090: promote palette to Command Lens with preview, disabled reasons,
  aliases, and profile-aware binding display.
- W100: implement Console Fit and capability-based visual fallbacks.
- W110: motion, accessibility, recovery, and golden layout lock.

### 15.4 W039 Terminal Proof Status

As of 2026-04-28, W039 has turned this doctrine into a terminal UX-lab
proof. The authoritative proof artifacts are:

- `docs/firehorse_mockups/UX_RESET.md`
- `docs/firehorse_mockups/UX_PROJECTION_CONTRACT.md`
- `docs/worksets/W039_firehorse_terminal_ux_proof.md`
- `src/shell/uxlab/firehorse/`
- `tests/wtd/goldens/W039/`

Future implementation worksets should use the W039 projection contract
and goldens as input. They should not preserve old OxIde front-end UI
shape by default, and they should not treat W039 fixture data as product
behavior. W039 proves the terminal composition, surface mapping, action
ids, and missing-seam placeholders; W040 through W100 own the real
behavior behind those surfaces.

## 16. Mockup Targets

The first design proof should produce static but believable terminal
captures for:

1. Launchpad at standard size;
2. Editing clean project at standard size;
3. Editing with nearest diagnostic lens;
4. Command Lens with preview column;
5. Run Lane with structured output;
6. Debug Cockpit paused at breakpoint;
7. Console Fit healthy terminal;
8. Compact Focus Mode;
9. Wide Studio Mode.

Each mockup must be judged by:

- does source remain central?
- is the state recognizable within one glance?
- are the rails useful without becoming noisy?
- does the design avoid retro-console boxiness?
- does compact mode feel intentional?
- does wide mode feel premium rather than just stretched?
- can a VBA IDE user infer the basic workflow?

## 17. Open Decisions

These should be answered before making this canonical:

1. What minimum terminal size is honestly supported for Focus Mode?
2. Does the Project Spine default to Full or Slim at 120 columns?
3. Does Context Dock auto-open for diagnostics, or only badge the
   Activity Rail until requested?
4. Should Command Lens absorb all go-to-file/symbol/line flows, or do
   some get dedicated overlays?
5. What exact source-lens rendering is legible across Windows Terminal,
   legacy console hosts, Ghostty, Kitty, and terminal multiplexers?
6. How much motion is acceptable in WTD goldens without flaky tests?
7. Does the light theme ship day one or remain a W110 polish target?
8. What is the exact VBA-compatible keymap profile after shortcut
   research?

## 18. Anti-Goals

Do not:

- make the Fire Horse concept literal with horse imagery or novelty
  decoration;
- make every surface orange;
- add animations that interfere with typing;
- bury source under dashboard widgets;
- make Command Lens the only usable workflow;
- copy VS Code's sidebar/status bar literally;
- copy old DOS/VBA visuals literally;
- require advanced terminal graphics for the core product;
- turn Debug into a separate app inside the app;
- expose OxVba generated internals as normal user project objects.

## 19. Doctrine Read-Through Evidence

Read-through result:

- [x] It covers the same broad scope as `docs/DESIGN_TUI.md`.
- [x] It preserves the authority boundaries in `ARCHITECTURE.md`.
- [x] It names which old concepts are kept, renamed, or replaced.
- [x] It gives enough scene detail to build FrankenTui/UX-lab mockups.
- [x] It states visual tokens, fallback policy, and motion policy.
- [x] It maps proposal concepts onto future worksets.
- [x] It lists open decisions instead of hiding them in prose.

## 20. Closing Position

The stronger direction is not "more panes" and not "more retro." The
stronger direction is a source-centered console cockpit: rails instead
of boxes, semantic lenses instead of remote explanations, run/debug
timelines instead of raw output panes, and a Command Lens that makes the
whole IDE feel discoverable.

OxIde should look like it belongs to 2026 because it treats the terminal
as a living application medium, not because it paints nostalgia with
new colors.
