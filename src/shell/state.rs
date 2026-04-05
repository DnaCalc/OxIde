#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellScene {
    Empty,
    Editing,
    Semantic,
    BuildRun,
    Palette,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusRegion {
    TopBar,
    Explorer,
    Editor,
    Inspector,
    LowerSurface,
    Palette,
}

impl FocusRegion {
    pub fn label(self) -> &'static str {
        match self {
            Self::TopBar => "Top",
            Self::Explorer => "Explorer",
            Self::Editor => "Editor",
            Self::Inspector => "Inspector",
            Self::LowerSurface => "Lower",
            Self::Palette => "Palette",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InspectorMode {
    Summary,
    Diagnostics,
    Symbols,
    Hover,
    RunStatus,
}

impl InspectorMode {
    pub fn label(self) -> &'static str {
        match self {
            Self::Summary => "Summary",
            Self::Diagnostics => "Diagnostics",
            Self::Symbols => "Symbols",
            Self::Hover => "Hover",
            Self::RunStatus => "RunStatus",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LowerSurfaceMode {
    Launcher,
    Problems,
    Output,
    Immediate,
    References,
    BuildLog,
}

impl LowerSurfaceMode {
    pub fn label(self) -> &'static str {
        match self {
            Self::Launcher => "Launcher",
            Self::Problems => "Problems",
            Self::Output => "Output",
            Self::Immediate => "Immediate",
            Self::References => "References",
            Self::BuildLog => "BuildLog",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidthClass {
    Wide,
    Standard,
    Narrow,
}

impl WidthClass {
    pub fn from_width(width: u16) -> Self {
        if width >= 160 {
            Self::Wide
        } else if width >= 120 {
            Self::Standard
        } else {
            Self::Narrow
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Wide => "Wide",
            Self::Standard => "Standard",
            Self::Narrow => "Narrow",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufferId(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ViewId(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferKind {
    Welcome,
    Source,
    Class,
}

impl BufferKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Welcome => "Welcome",
            Self::Source => "Source",
            Self::Class => "Class",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BufferState {
    pub id: BufferId,
    pub title: String,
    pub kind: BufferKind,
    pub dirty: bool,
    pub lines: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewKind {
    Primary,
    Secondary,
}

impl ViewKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Primary => "Primary",
            Self::Secondary => "Secondary",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorPosition {
    pub line: u16,
    pub column: u16,
}

impl CursorPosition {
    pub const fn new(line: u16, column: u16) -> Self {
        Self { line, column }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectionRange {
    pub anchor: CursorPosition,
    pub head: CursorPosition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditorSurfaceState {
    pub cursor: CursorPosition,
    pub selection: Option<SelectionRange>,
    pub scroll_top: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewState {
    pub id: ViewId,
    pub buffer_id: BufferId,
    pub kind: ViewKind,
    pub surface: EditorSurfaceState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutPreset {
    Project,
    Edit,
    SplitEdit,
    Run,
}

impl LayoutPreset {
    pub fn label(self) -> &'static str {
        match self {
            Self::Project => "Project",
            Self::Edit => "Edit",
            Self::SplitEdit => "SplitEdit",
            Self::Run => "Run",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceLayoutState {
    pub preset: LayoutPreset,
    pub visible_views: Vec<ViewId>,
    pub active_view: ViewId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceState {
    pub project_name: Option<String>,
    pub target_name: String,
    pub buffers: Vec<BufferState>,
    pub recent_buffers: Vec<BufferId>,
    pub views: Vec<ViewState>,
    pub layout: WorkspaceLayoutState,
}

impl WorkspaceState {
    pub fn active_view(&self) -> Option<&ViewState> {
        self.views
            .iter()
            .find(|view| view.id == self.layout.active_view)
    }

    pub fn visible_views(&self) -> Vec<&ViewState> {
        self.layout
            .visible_views
            .iter()
            .filter_map(|id| self.views.iter().find(|view| view.id == *id))
            .collect()
    }

    pub fn buffer(&self, id: BufferId) -> Option<&BufferState> {
        self.buffers.iter().find(|buffer| buffer.id == id)
    }

    pub fn active_buffer(&self) -> Option<&BufferState> {
        self.active_view()
            .and_then(|view| self.buffer(view.buffer_id))
    }

    pub fn open_buffer_count(&self) -> usize {
        self.buffers.len()
    }

    pub fn visible_view_count(&self) -> usize {
        self.layout.visible_views.len()
    }

    pub fn hidden_buffer_count(&self) -> usize {
        self.buffers
            .iter()
            .filter(|buffer| {
                !self.layout.visible_views.iter().any(|view_id| {
                    self.views
                        .iter()
                        .find(|view| view.id == *view_id)
                        .is_some_and(|view| view.buffer_id == buffer.id)
                })
            })
            .count()
    }

    pub fn cycle_active_view(&mut self) {
        let Some(index) = self
            .layout
            .visible_views
            .iter()
            .position(|view_id| *view_id == self.layout.active_view)
        else {
            return;
        };

        self.layout.active_view =
            self.layout.visible_views[(index + 1) % self.layout.visible_views.len()];
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PanelSectionState {
    pub title: &'static str,
    pub lines: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PanelContentState {
    pub sections: Vec<PanelSectionState>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherContentState {
    pub recent_projects: Vec<&'static str>,
    pub actions: Vec<&'static str>,
    pub capabilities: Vec<&'static str>,
    pub hint: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaletteCommandState {
    pub label: &'static str,
    pub shortcut: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaletteContentState {
    pub filter_hint: &'static str,
    pub commands: Vec<PaletteCommandState>,
    pub state_commands: Vec<PaletteCommandState>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellContentState {
    pub launcher: LauncherContentState,
    pub editor_notes: Vec<String>,
    pub inspector: PanelContentState,
    pub lower_surface: PanelContentState,
    pub palette: PaletteContentState,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShellLayoutPolicy {
    pub explorer_width_percent: f32,
    pub editor_width_percent: f32,
    pub lower_surface_height: Option<u16>,
    pub inspector_collapsed: bool,
    pub shows_lower_surface: bool,
}

impl ShellLayoutPolicy {
    fn derive(scene: ShellScene, width_class: WidthClass) -> Self {
        let shows_lower_surface = !matches!(scene, ShellScene::Empty);
        let lower_surface_height = if shows_lower_surface {
            Some(match (scene, width_class) {
                (ShellScene::BuildRun, WidthClass::Wide) => 11,
                (ShellScene::BuildRun, _) => 10,
                (ShellScene::Semantic, WidthClass::Wide) => 10,
                (ShellScene::Semantic, _) => 9,
                (_, WidthClass::Wide) => 8,
                _ => 7,
            })
        } else {
            None
        };

        let (explorer_width_percent, editor_width_percent) = match (width_class, scene) {
            (WidthClass::Wide, ShellScene::Empty) => (16.0_f32, 58.0_f32),
            (WidthClass::Wide, ShellScene::BuildRun) => (18.0_f32, 56.0_f32),
            (WidthClass::Wide, _) => (20.0_f32, 58.0_f32),
            (WidthClass::Standard, ShellScene::Empty) => (18.0_f32, 56.0_f32),
            (WidthClass::Standard, ShellScene::BuildRun) => (18.0_f32, 57.0_f32),
            (WidthClass::Standard, _) => (18.0_f32, 62.0_f32),
            (WidthClass::Narrow, _) => (20.0_f32, 80.0_f32),
        };

        Self {
            explorer_width_percent,
            editor_width_percent,
            lower_surface_height,
            inspector_collapsed: width_class == WidthClass::Narrow && scene != ShellScene::Palette,
            shows_lower_surface,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShellRuntimeState {
    pub focus: FocusRegion,
    pub inspector_mode: InspectorMode,
    pub lower_mode: LowerSurfaceMode,
    pub width_class: WidthClass,
    pub size: (u16, u16),
    pub layout: ShellLayoutPolicy,
    pub workspace: WorkspaceState,
    pub session_workspace: Option<WorkspaceState>,
    pub content: ShellContentState,
    pub previous_focus: FocusRegion,
    pub previous_scene: ShellScene,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShellState {
    pub scene: ShellScene,
    pub runtime: ShellRuntimeState,
}

impl Default for ShellState {
    fn default() -> Self {
        let width_class = WidthClass::Standard;
        let scene = ShellScene::Editing;
        let workspace = workspace_for_scene(scene);
        let mut state = Self {
            scene,
            runtime: ShellRuntimeState {
                focus: FocusRegion::Editor,
                inspector_mode: InspectorMode::Diagnostics,
                lower_mode: LowerSurfaceMode::Problems,
                width_class,
                size: (120, 40),
                layout: ShellLayoutPolicy::derive(scene, width_class),
                workspace: workspace.clone(),
                session_workspace: None,
                content: content_for_scene(scene, &workspace),
                previous_focus: FocusRegion::Editor,
                previous_scene: scene,
            },
        };
        state.apply_scene(scene);
        state
    }
}

impl ShellState {
    pub fn apply_scene(&mut self, scene: ShellScene) {
        self.scene = scene;
        self.runtime.layout = ShellLayoutPolicy::derive(scene, self.runtime.width_class);
        self.runtime.workspace = match (scene, self.runtime.session_workspace.clone()) {
            (ShellScene::Empty, _) => workspace_for_scene(scene),
            (_, Some(workspace)) => workspace_for_scene_from_loaded(scene, workspace),
            (_, None) => workspace_for_scene(scene),
        };
        self.runtime.content = content_for_scene(scene, &self.runtime.workspace);
        if scene != ShellScene::Palette {
            self.runtime.previous_scene = scene;
        }
        match scene {
            ShellScene::Empty => {
                self.runtime.inspector_mode = InspectorMode::Summary;
                self.runtime.lower_mode = LowerSurfaceMode::Launcher;
                self.runtime.focus = FocusRegion::Editor;
            }
            ShellScene::Editing => {
                self.runtime.inspector_mode = InspectorMode::Diagnostics;
                self.runtime.lower_mode = LowerSurfaceMode::Problems;
                self.runtime.focus = FocusRegion::Editor;
            }
            ShellScene::Semantic => {
                self.runtime.inspector_mode = InspectorMode::Hover;
                self.runtime.lower_mode = LowerSurfaceMode::References;
                self.runtime.focus = FocusRegion::Inspector;
            }
            ShellScene::BuildRun => {
                self.runtime.inspector_mode = InspectorMode::RunStatus;
                self.runtime.lower_mode = LowerSurfaceMode::Output;
                self.runtime.focus = FocusRegion::LowerSurface;
            }
            ShellScene::Palette => {
                self.runtime.inspector_mode = InspectorMode::Symbols;
                self.runtime.lower_mode = LowerSurfaceMode::Problems;
                self.runtime.focus = FocusRegion::Palette;
            }
        }
    }

    pub fn mount_workspace(&mut self, workspace: WorkspaceState) {
        self.runtime.session_workspace = Some(workspace.clone());
        self.runtime.workspace = workspace_for_scene_from_loaded(self.scene, workspace);
        self.runtime.content = content_for_scene(self.scene, &self.runtime.workspace);
    }

    pub fn update_size(&mut self, width: u16, height: u16) {
        self.runtime.size = (width, height);
        self.runtime.width_class = WidthClass::from_width(width);
        self.runtime.layout = ShellLayoutPolicy::derive(self.scene, self.runtime.width_class);
        if self.runtime.focus == FocusRegion::Inspector && self.inspector_is_collapsed() {
            self.runtime.focus = FocusRegion::LowerSurface;
        }
        if self.runtime.focus == FocusRegion::LowerSurface && !self.shows_lower_surface() {
            self.runtime.focus = FocusRegion::Editor;
        }
    }

    pub fn cycle_focus(&mut self) {
        let regions = self.available_focus_regions();
        let index = regions
            .iter()
            .position(|region| *region == self.runtime.focus)
            .unwrap_or(0);
        self.runtime.focus = regions[(index + 1) % regions.len()];
    }

    pub fn focus_region(&mut self, region: FocusRegion) {
        if self.available_focus_regions().contains(&region) {
            self.runtime.focus = region;
        }
    }

    pub fn toggle_palette(&mut self) {
        if self.palette_active() {
            self.apply_scene(self.runtime.previous_scene);
            self.focus_region(self.runtime.previous_focus);
            return;
        }

        self.runtime.previous_scene = self.scene;
        self.runtime.previous_focus = self.runtime.focus;
        self.apply_scene(ShellScene::Palette);
    }

    pub fn cycle_active_editor_view(&mut self) {
        self.runtime.workspace.cycle_active_view();
    }

    pub fn palette_active(&self) -> bool {
        self.scene == ShellScene::Palette
    }

    pub fn inspector_is_collapsed(&self) -> bool {
        self.runtime.layout.inspector_collapsed
    }

    pub fn shows_lower_surface(&self) -> bool {
        self.runtime.layout.shows_lower_surface
    }

    pub fn lower_surface_height(&self) -> Option<u16> {
        self.runtime.layout.lower_surface_height
    }

    pub fn explorer_width_percent(&self) -> f32 {
        self.runtime.layout.explorer_width_percent
    }

    pub fn editor_width_percent(&self) -> f32 {
        self.runtime.layout.editor_width_percent
    }

    pub fn available_focus_regions(&self) -> Vec<FocusRegion> {
        if self.palette_active() {
            return vec![FocusRegion::Palette];
        }

        let mut regions = vec![
            FocusRegion::TopBar,
            FocusRegion::Explorer,
            FocusRegion::Editor,
        ];
        if !self.inspector_is_collapsed() {
            regions.push(FocusRegion::Inspector);
        }
        if self.shows_lower_surface() {
            regions.push(FocusRegion::LowerSurface);
        }
        regions
    }
}

fn workspace_for_scene(scene: ShellScene) -> WorkspaceState {
    const VIEW_WELCOME: ViewId = ViewId(1);
    const VIEW_MAIN: ViewId = ViewId(2);
    const VIEW_SPLIT: ViewId = ViewId(3);

    const BUFFER_WELCOME: BufferId = BufferId(1);
    const BUFFER_MAIN: BufferId = BufferId(2);
    const BUFFER_HELPERS: BufferId = BufferId(3);
    const BUFFER_INVOICE: BufferId = BufferId(4);

    match scene {
        ShellScene::Empty => WorkspaceState {
            project_name: None,
            target_name: String::from("None"),
            buffers: vec![BufferState {
                id: BUFFER_WELCOME,
                title: String::from("Welcome"),
                kind: BufferKind::Welcome,
                dirty: false,
                lines: lines(&[
                    "OxIde",
                    "",
                    "A terminal-native IDE for OxVba.",
                    "",
                    "Open",
                    "  > Open Project",
                    "    Create Project",
                    "    Recent Projects",
                ]),
            }],
            recent_buffers: vec![BUFFER_WELCOME],
            views: vec![ViewState {
                id: VIEW_WELCOME,
                buffer_id: BUFFER_WELCOME,
                kind: ViewKind::Primary,
                surface: EditorSurfaceState {
                    cursor: CursorPosition::new(1, 1),
                    selection: None,
                    scroll_top: 0,
                },
            }],
            layout: WorkspaceLayoutState {
                preset: LayoutPreset::Project,
                visible_views: vec![VIEW_WELCOME],
                active_view: VIEW_WELCOME,
            },
        },
        ShellScene::Editing | ShellScene::Palette => WorkspaceState {
            project_name: Some(String::from("Payroll.basproj")),
            target_name: String::from("Exe"),
            buffers: vec![
                BufferState {
                    id: BUFFER_MAIN,
                    title: String::from("MainModule.bas"),
                    kind: BufferKind::Source,
                    dirty: false,
                    lines: lines(&[
                        "01  Option Explicit",
                        "02",
                        "03  Public Sub Main()",
                        "04      Dim total As Integer",
                        "05      total = 40 + 2",
                        "06      Debug.Print total",
                        "07  End Sub",
                        "08",
                        "09  Public Function BuildReport() As String",
                        "10      BuildReport = \"ready\"",
                        "11  End Function",
                    ]),
                },
                BufferState {
                    id: BUFFER_HELPERS,
                    title: String::from("Helpers.bas"),
                    kind: BufferKind::Source,
                    dirty: true,
                    lines: lines(&[
                        "01  Option Explicit",
                        "02",
                        "03  Public Function ComputeAnswer() As Integer",
                        "04      ComputeAnswer = 42",
                        "05  End Function",
                    ]),
                },
                BufferState {
                    id: BUFFER_INVOICE,
                    title: String::from("Invoice.cls"),
                    kind: BufferKind::Class,
                    dirty: false,
                    lines: lines(&[
                        "01  Option Explicit",
                        "02",
                        "03  Private currentId As String",
                        "04",
                        "05  Public Property Get Id() As String",
                        "06      Id = currentId",
                        "07  End Property",
                    ]),
                },
            ],
            recent_buffers: vec![BUFFER_MAIN, BUFFER_HELPERS, BUFFER_INVOICE],
            views: vec![ViewState {
                id: VIEW_MAIN,
                buffer_id: BUFFER_MAIN,
                kind: ViewKind::Primary,
                surface: EditorSurfaceState {
                    cursor: CursorPosition::new(5, 7),
                    selection: None,
                    scroll_top: 0,
                },
            }],
            layout: WorkspaceLayoutState {
                preset: LayoutPreset::Edit,
                visible_views: vec![VIEW_MAIN],
                active_view: VIEW_MAIN,
            },
        },
        ShellScene::Semantic => WorkspaceState {
            project_name: Some(String::from("Payroll.basproj")),
            target_name: String::from("Exe"),
            buffers: vec![
                BufferState {
                    id: BUFFER_MAIN,
                    title: String::from("MainModule.bas"),
                    kind: BufferKind::Source,
                    dirty: false,
                    lines: lines(&[
                        "01  Option Explicit",
                        "02",
                        "03  Public Sub Main()",
                        "04      Dim total As Integer",
                        "05      total = ComputeAnswer()",
                        "06      Debug.Print total",
                        "07  End Sub",
                        "08",
                        "09  Public Function ComputeAnswer() As Integer",
                        "10      ComputeAnswer = 42",
                        "11  End Function",
                    ]),
                },
                BufferState {
                    id: BUFFER_HELPERS,
                    title: String::from("Helpers.bas"),
                    kind: BufferKind::Source,
                    dirty: true,
                    lines: lines(&[
                        "01  Option Explicit",
                        "02",
                        "03  Public Function ComputeAnswer() As Integer",
                        "04      ComputeAnswer = 42",
                        "05  End Function",
                    ]),
                },
                BufferState {
                    id: BUFFER_INVOICE,
                    title: String::from("Invoice.cls"),
                    kind: BufferKind::Class,
                    dirty: false,
                    lines: lines(&[
                        "01  Option Explicit",
                        "02",
                        "03  Private currentId As String",
                        "04",
                        "05  Public Property Get Id() As String",
                        "06      Id = currentId",
                        "07  End Property",
                    ]),
                },
            ],
            recent_buffers: vec![BUFFER_MAIN, BUFFER_HELPERS, BUFFER_INVOICE],
            views: vec![
                ViewState {
                    id: VIEW_MAIN,
                    buffer_id: BUFFER_MAIN,
                    kind: ViewKind::Primary,
                    surface: EditorSurfaceState {
                        cursor: CursorPosition::new(5, 13),
                        selection: None,
                        scroll_top: 0,
                    },
                },
                ViewState {
                    id: VIEW_SPLIT,
                    buffer_id: BUFFER_MAIN,
                    kind: ViewKind::Secondary,
                    surface: EditorSurfaceState {
                        cursor: CursorPosition::new(9, 1),
                        selection: Some(SelectionRange {
                            anchor: CursorPosition::new(9, 1),
                            head: CursorPosition::new(10, 20),
                        }),
                        scroll_top: 0,
                    },
                },
            ],
            layout: WorkspaceLayoutState {
                preset: LayoutPreset::SplitEdit,
                visible_views: vec![VIEW_MAIN, VIEW_SPLIT],
                active_view: VIEW_MAIN,
            },
        },
        ShellScene::BuildRun => WorkspaceState {
            project_name: Some(String::from("Payroll.basproj")),
            target_name: String::from("Exe"),
            buffers: vec![
                BufferState {
                    id: BUFFER_MAIN,
                    title: String::from("MainModule.bas"),
                    kind: BufferKind::Source,
                    dirty: false,
                    lines: lines(&[
                        "01  Option Explicit",
                        "02",
                        "03  Public Sub Main()",
                        "04      Dim total As Integer",
                        "05  >   total = ComputeAnswer()",
                        "06      Debug.Print total",
                        "07  End Sub",
                        "08",
                        "09  Public Function ComputeAnswer() As Integer",
                        "10      ComputeAnswer = 42",
                        "11  End Function",
                    ]),
                },
                BufferState {
                    id: BUFFER_HELPERS,
                    title: String::from("Helpers.bas"),
                    kind: BufferKind::Source,
                    dirty: false,
                    lines: lines(&[
                        "01  Option Explicit",
                        "02",
                        "03  Public Function ComputeAnswer() As Integer",
                        "04      ComputeAnswer = 42",
                        "05  End Function",
                    ]),
                },
                BufferState {
                    id: BUFFER_INVOICE,
                    title: String::from("Invoice.cls"),
                    kind: BufferKind::Class,
                    dirty: false,
                    lines: lines(&[
                        "01  Option Explicit",
                        "02",
                        "03  Private currentId As String",
                        "04",
                        "05  Public Property Get Id() As String",
                        "06      Id = currentId",
                        "07  End Property",
                    ]),
                },
            ],
            recent_buffers: vec![BUFFER_MAIN, BUFFER_HELPERS, BUFFER_INVOICE],
            views: vec![ViewState {
                id: VIEW_MAIN,
                buffer_id: BUFFER_MAIN,
                kind: ViewKind::Primary,
                surface: EditorSurfaceState {
                    cursor: CursorPosition::new(5, 5),
                    selection: None,
                    scroll_top: 0,
                },
            }],
            layout: WorkspaceLayoutState {
                preset: LayoutPreset::Run,
                visible_views: vec![VIEW_MAIN],
                active_view: VIEW_MAIN,
            },
        },
    }
}

fn workspace_for_scene_from_loaded(
    scene: ShellScene,
    mut workspace: WorkspaceState,
) -> WorkspaceState {
    match scene {
        ShellScene::Empty => return workspace_for_scene(ShellScene::Empty),
        ShellScene::Editing | ShellScene::Palette => {
            workspace.layout.preset = LayoutPreset::Edit;
        }
        ShellScene::Semantic => {
            workspace.layout.preset = LayoutPreset::SplitEdit;
        }
        ShellScene::BuildRun => {
            workspace.layout.preset = LayoutPreset::Run;
        }
    }
    workspace
}

fn lines(input: &[&str]) -> Vec<String> {
    input.iter().map(|line| String::from(*line)).collect()
}

fn workspace_symbols(workspace: &WorkspaceState) -> Vec<String> {
    let mut symbols = workspace
        .active_buffer()
        .map(|buffer| {
            buffer
                .lines
                .iter()
                .filter_map(|line| parse_symbol_name(line))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    if symbols.is_empty() {
        symbols.push(String::from("Main"));
    }

    symbols
}

fn parse_symbol_name(line: &str) -> Option<String> {
    let trimmed = line.trim();
    for marker in [
        "Public Sub ",
        "Private Sub ",
        "Public Function ",
        "Private Function ",
    ] {
        if let Some(rest) = trimmed.strip_prefix(marker) {
            let name = rest
                .split(['(', ' '])
                .next()
                .filter(|value| !value.is_empty())?;
            return Some(String::from(name));
        }
    }
    None
}

fn content_for_scene(scene: ShellScene, workspace: &WorkspaceState) -> ShellContentState {
    let active_buffer_title = workspace
        .active_buffer()
        .map(|buffer| buffer.title.as_str())
        .unwrap_or("current buffer");
    let active_cursor = workspace
        .active_view()
        .map(|view| {
            format!(
                "{}:{}",
                view.surface.cursor.line, view.surface.cursor.column
            )
        })
        .unwrap_or_else(|| String::from("-"));
    let dirty_buffers = workspace
        .buffers
        .iter()
        .filter(|buffer| buffer.dirty)
        .count();
    let shared_buffer = if workspace.visible_view_count() > 1 {
        "yes"
    } else {
        "no"
    };
    let selection = workspace
        .visible_views()
        .iter()
        .find_map(|view| view.surface.selection.map(|_| "present"))
        .unwrap_or("none");
    let symbols = workspace_symbols(workspace);
    let hidden_buffer_note = if workspace.hidden_buffer_count() > 0 {
        String::from("Hidden buffers remain switchable without tabs")
    } else {
        String::from("The active project is currently mounted into a single visible buffer")
    };

    let launcher = LauncherContentState {
        recent_projects: vec!["Payroll.basproj", "Ledger.basproj"],
        actions: vec!["Open Project", "Create Project", "Browse Recent"],
        capabilities: vec![
            "Truecolor detected",
            "Unicode coverage good",
            "Keyboard routing ready",
        ],
        hint: "F2 Empty  F3 Edit  F4 Semantic  F5 Run  F6 Palette",
    };

    let palette = PaletteContentState {
        filter_hint: match scene {
            ShellScene::Empty => "Start from empty shell",
            ShellScene::Editing => "Editing shell commands",
            ShellScene::Semantic => "Semantic shell commands",
            ShellScene::BuildRun => "Build/run shell commands",
            ShellScene::Palette => "Palette overlay active",
        },
        commands: vec![
            PaletteCommandState {
                label: "Open Project",
                shortcut: "Ctrl+O",
            },
            PaletteCommandState {
                label: "New Project",
                shortcut: "Ctrl+N",
            },
            PaletteCommandState {
                label: "Focus Explorer",
                shortcut: "Alt+1",
            },
            PaletteCommandState {
                label: "Focus Editor",
                shortcut: "Alt+2",
            },
            PaletteCommandState {
                label: "Focus Inspector",
                shortcut: "Alt+3",
            },
            PaletteCommandState {
                label: "Focus Lower Surface",
                shortcut: "Alt+4",
            },
            PaletteCommandState {
                label: "Cycle Editor View",
                shortcut: "Ctrl+Tab",
            },
            PaletteCommandState {
                label: "Toggle Palette",
                shortcut: "F6",
            },
        ],
        state_commands: vec![
            PaletteCommandState {
                label: "Empty",
                shortcut: "F2",
            },
            PaletteCommandState {
                label: "Editing",
                shortcut: "F3",
            },
            PaletteCommandState {
                label: "Semantic",
                shortcut: "F4",
            },
            PaletteCommandState {
                label: "Build/Run",
                shortcut: "F5",
            },
            PaletteCommandState {
                label: "Palette",
                shortcut: "F6",
            },
        ],
    };

    match scene {
        ShellScene::Empty => ShellContentState {
            launcher,
            editor_notes: vec![
                String::from("Open a project or create a new one to begin."),
                String::from("The shell remains keyboard-first from the first screen."),
            ],
            inspector: PanelContentState {
                sections: vec![
                    PanelSectionState {
                        title: "Capabilities",
                        lines: vec![
                            String::from("Truecolor: yes"),
                            String::from("Unicode: yes"),
                            String::from("Mouse: optional"),
                        ],
                    },
                    PanelSectionState {
                        title: "Theme",
                        lines: vec![
                            String::from("Mockup-derived instrument palette"),
                            String::from("High-contrast panel hierarchy is active"),
                        ],
                    },
                ],
            },
            lower_surface: PanelContentState {
                sections: Vec::new(),
            },
            palette,
        },
        ShellScene::Editing | ShellScene::Palette => ShellContentState {
            launcher,
            editor_notes: vec![
                format!("Primary editor view mounted on {active_buffer_title}"),
                hidden_buffer_note,
                String::from("Ctrl+Tab rotates visible views when splits exist"),
            ],
            inspector: PanelContentState {
                sections: vec![
                    PanelSectionState {
                        title: "Diagnostics",
                        lines: vec![String::from("0 errors"), String::from("1 warning")],
                    },
                    PanelSectionState {
                        title: "Symbols",
                        lines: symbols.clone(),
                    },
                    PanelSectionState {
                        title: "Session",
                        lines: vec![
                            format!("Dirty buffers: {dirty_buffers}"),
                            format!("Visible views: {}", workspace.visible_view_count()),
                            format!("Hidden buffers: {}", workspace.hidden_buffer_count()),
                            format!("Active cursor: {active_cursor}"),
                        ],
                    },
                ],
            },
            lower_surface: PanelContentState {
                sections: vec![PanelSectionState {
                    title: "Problems",
                    lines: vec![
                        String::from("warning: BuildReport is not yet called"),
                        String::from(
                            "hint: keep Problems compact until execution or semantic work needs the space",
                        ),
                    ],
                }],
            },
            palette,
        },
        ShellScene::Semantic => ShellContentState {
            launcher,
            editor_notes: vec![
                String::from("Split layout keeps a secondary view on the same buffer"),
                String::from("Ctrl+Tab rotates the active visible view"),
                String::from("Inspector owns semantic context while the editor stays source-first"),
            ],
            inspector: PanelContentState {
                sections: vec![
                    PanelSectionState {
                        title: "Hover",
                        lines: vec![
                            String::from("ComputeAnswer() As Integer"),
                            String::from("Returns the canonical answer used by the semantic pass."),
                        ],
                    },
                    PanelSectionState {
                        title: "Symbols",
                        lines: symbols
                            .iter()
                            .enumerate()
                            .map(|(index, symbol)| {
                                if index == 0 {
                                    format!("> {symbol}")
                                } else {
                                    symbol.clone()
                                }
                            })
                            .collect(),
                    },
                    PanelSectionState {
                        title: "Layout",
                        lines: vec![
                            format!("Preset: {}", workspace.layout.preset.label()),
                            format!("Visible views: {}", workspace.visible_view_count()),
                            format!("Shared buffer: {shared_buffer}"),
                            format!("Selection: {selection}"),
                        ],
                    },
                ],
            },
            lower_surface: PanelContentState {
                sections: vec![
                    PanelSectionState {
                        title: "References",
                        lines: vec![
                            String::from("MainModule.bas:5 ComputeAnswer()"),
                            String::from("Helpers.bas:12 ComputeAnswer()"),
                        ],
                    },
                    PanelSectionState {
                        title: "Immediate",
                        lines: vec![String::from("? ComputeAnswer()"), String::from("42")],
                    },
                ],
            },
            palette,
        },
        ShellScene::BuildRun => ShellContentState {
            launcher,
            editor_notes: vec![String::from(
                "Run layout keeps one primary code view mounted while output owns the lower surface",
            )],
            inspector: PanelContentState {
                sections: vec![
                    PanelSectionState {
                        title: "Run Status",
                        lines: vec![
                            String::from("Build: passing"),
                            String::from("Runtime: active"),
                            String::from("Profile: win-console"),
                            String::from("Last exit: 0"),
                        ],
                    },
                    PanelSectionState {
                        title: "Workspace",
                        lines: vec![
                            format!("Layout: {}", workspace.layout.preset.label()),
                            format!(
                                "Active buffer: {}",
                                workspace
                                    .active_buffer()
                                    .map(|buffer| buffer.title.as_str())
                                    .unwrap_or("None")
                            ),
                            format!("Open buffers: {}", workspace.open_buffer_count()),
                        ],
                    },
                ],
            },
            lower_surface: PanelContentState {
                sections: vec![
                    PanelSectionState {
                        title: "Output",
                        lines: vec![
                            String::from("[build] compiling Payroll.basproj"),
                            String::from("[run] launching Exe target"),
                            String::from("stdout:"),
                            String::from("42"),
                        ],
                    },
                    PanelSectionState {
                        title: "Build Log",
                        lines: vec![
                            String::from("bundle ready"),
                            String::from("stdout attached"),
                        ],
                    },
                ],
            },
            palette,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn editing_scene_keeps_open_buffers_beyond_the_active_view() {
        let state = ShellState::default();
        assert_eq!(state.runtime.workspace.layout.preset, LayoutPreset::Edit);
        assert_eq!(state.runtime.workspace.open_buffer_count(), 3);
        assert_eq!(state.runtime.workspace.visible_view_count(), 1);
        assert_eq!(state.runtime.workspace.hidden_buffer_count(), 2);
    }

    #[test]
    fn semantic_scene_supports_two_views_on_the_same_buffer() {
        let mut state = ShellState::default();
        state.apply_scene(ShellScene::Semantic);

        let visible_views = state.runtime.workspace.visible_views();
        assert_eq!(
            state.runtime.workspace.layout.preset,
            LayoutPreset::SplitEdit
        );
        assert_eq!(visible_views.len(), 2);
        assert_eq!(visible_views[0].buffer_id, visible_views[1].buffer_id);
    }

    #[test]
    fn build_run_scene_switches_to_the_run_layout_preset() {
        let mut state = ShellState::default();
        state.apply_scene(ShellScene::BuildRun);

        assert_eq!(state.runtime.workspace.layout.preset, LayoutPreset::Run);
        assert_eq!(state.runtime.workspace.visible_view_count(), 1);
    }

    #[test]
    fn palette_toggle_restores_prior_scene_and_focus() {
        let mut state = ShellState::default();
        state.focus_region(FocusRegion::Inspector);
        state.toggle_palette();
        assert_eq!(state.scene, ShellScene::Palette);

        state.toggle_palette();
        assert_eq!(state.scene, ShellScene::Editing);
        assert_eq!(state.runtime.focus, FocusRegion::Inspector);
    }

    #[test]
    fn cycle_active_editor_view_rotates_visible_views_when_split() {
        let mut state = ShellState::default();
        state.apply_scene(ShellScene::Semantic);

        let first_view = state.runtime.workspace.layout.active_view;
        state.cycle_active_editor_view();

        assert_ne!(state.runtime.workspace.layout.active_view, first_view);
    }

    #[test]
    fn editing_scene_populates_runtime_owned_content_providers() {
        let state = ShellState::default();

        assert!(!state.runtime.content.inspector.sections.is_empty());
        assert!(!state.runtime.content.lower_surface.sections.is_empty());
        assert!(!state.runtime.content.palette.commands.is_empty());
        assert_eq!(
            state
                .runtime
                .workspace
                .active_buffer()
                .map(|buffer| buffer.lines.len()),
            Some(11)
        );
    }
}
