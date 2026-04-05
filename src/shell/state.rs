#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MockState {
    Empty,
    Editing,
    Semantic,
    BuildRun,
    Palette,
}

impl MockState {
    pub fn label(self) -> &'static str {
        match self {
            Self::Empty => "Empty",
            Self::Editing => "Editing",
            Self::Semantic => "Semantic",
            Self::BuildRun => "Build/Run",
            Self::Palette => "Palette",
        }
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShellState {
    pub mock_state: MockState,
    pub focus: FocusRegion,
    pub inspector_mode: InspectorMode,
    pub lower_mode: LowerSurfaceMode,
    pub width_class: WidthClass,
    pub size: (u16, u16),
}

impl Default for ShellState {
    fn default() -> Self {
        let mut state = Self {
            mock_state: MockState::Editing,
            focus: FocusRegion::Editor,
            inspector_mode: InspectorMode::Diagnostics,
            lower_mode: LowerSurfaceMode::Problems,
            width_class: WidthClass::Standard,
            size: (120, 40),
        };
        state.apply_mock_state(MockState::Editing);
        state
    }
}

impl ShellState {
    pub fn apply_mock_state(&mut self, mock_state: MockState) {
        self.mock_state = mock_state;
        match mock_state {
            MockState::Empty => {
                self.inspector_mode = InspectorMode::Summary;
                self.lower_mode = LowerSurfaceMode::Launcher;
                self.focus = FocusRegion::Editor;
            }
            MockState::Editing => {
                self.inspector_mode = InspectorMode::Diagnostics;
                self.lower_mode = LowerSurfaceMode::Problems;
                self.focus = FocusRegion::Editor;
            }
            MockState::Semantic => {
                self.inspector_mode = InspectorMode::Hover;
                self.lower_mode = LowerSurfaceMode::References;
                self.focus = FocusRegion::Inspector;
            }
            MockState::BuildRun => {
                self.inspector_mode = InspectorMode::RunStatus;
                self.lower_mode = LowerSurfaceMode::Output;
                self.focus = FocusRegion::LowerSurface;
            }
            MockState::Palette => {
                self.inspector_mode = InspectorMode::Symbols;
                self.lower_mode = LowerSurfaceMode::Problems;
                self.focus = FocusRegion::Palette;
            }
        }
    }

    pub fn update_size(&mut self, width: u16, height: u16) {
        self.size = (width, height);
        self.width_class = WidthClass::from_width(width);
        if self.focus == FocusRegion::Inspector && self.inspector_is_collapsed() {
            self.focus = FocusRegion::LowerSurface;
        }
    }

    pub fn cycle_focus(&mut self) {
        let regions = self.available_focus_regions();
        let index = regions
            .iter()
            .position(|region| *region == self.focus)
            .unwrap_or(0);
        self.focus = regions[(index + 1) % regions.len()];
    }

    pub fn palette_active(&self) -> bool {
        self.mock_state == MockState::Palette
    }

    pub fn inspector_is_collapsed(&self) -> bool {
        self.width_class == WidthClass::Narrow && !self.palette_active()
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
        regions.push(FocusRegion::LowerSurface);
        regions
    }
}
