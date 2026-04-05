use super::state::{FocusRegion, LowerSurfaceMode, MockState, ShellState};
use super::theme;

pub struct ShellPanels {
    pub top_bar: String,
    pub explorer: String,
    pub editor_title: String,
    pub editor: String,
    pub inspector: String,
    pub lower_surface: String,
    pub palette: String,
}

pub fn shell_panels(state: &ShellState) -> ShellPanels {
    ShellPanels {
        top_bar: top_bar_text(state),
        explorer: explorer_text(state),
        editor_title: editor_title(state),
        editor: editor_text(state),
        inspector: inspector_text(state),
        lower_surface: lower_surface_text(state),
        palette: palette_text(state),
    }
}

fn top_bar_text(state: &ShellState) -> String {
    let lower_modes = [
        LowerSurfaceMode::Launcher,
        LowerSurfaceMode::Problems,
        LowerSurfaceMode::Output,
        LowerSurfaceMode::Immediate,
        LowerSurfaceMode::References,
        LowerSurfaceMode::BuildLog,
    ]
    .into_iter()
    .map(LowerSurfaceMode::label)
    .collect::<Vec<_>>()
    .join("/");

    format!(
        "Workspace: Payroll.basproj | Target: Exe | Mode: {} | Focus: {} | Width: {} | Palette: {} | Lower Modes: {}",
        state.mock_state.label(),
        state.focus.label(),
        state.width_class.label(),
        theme::palette_name(),
        lower_modes
    )
}

fn explorer_text(state: &ShellState) -> String {
    match state.mock_state {
        MockState::Empty => String::from(
            "Recent\n> Payroll.basproj\n  Ledger.basproj\n\nCommands\n  New Project\n  Open Project\n  Recent\n",
        ),
        _ => String::from(
            "Project\n> MainModule.bas\n  Helpers.bas\n  Invoice.cls\n  PayrollForm.frm\n\nReferences\n  VBA\n  OxVba Runtime\n\nTargets\n  Exe\n  Library\n",
        ),
    }
}

fn editor_title(state: &ShellState) -> String {
    match state.mock_state {
        MockState::Empty => String::from("Welcome"),
        MockState::Editing => String::from("MainModule.bas"),
        MockState::Semantic => String::from("MainModule.bas"),
        MockState::BuildRun => String::from("MainModule.bas"),
        MockState::Palette => String::from("MainModule.bas"),
    }
}

fn editor_text(state: &ShellState) -> String {
    match state.mock_state {
        MockState::Empty => String::from(
            "OxIde\n\nA terminal-native IDE for OxVba.\n\n> Open Project\n  Create Project\n  Recent Projects\n\nSetup\n  Truecolor: detected\n  Unicode: good\n  Keyboard: full\n\nHint\n  F2 Empty  F3 Edit  F4 Semantic  F5 Run  F6 Palette  Tab Focus  Ctrl+Q Quit\n",
        ),
        MockState::Editing | MockState::Palette => String::from(
            "01  Option Explicit\n02\n03  Public Sub Main()\n04      Dim total As Integer\n05      total = 40 + 2\n06      Debug.Print total\n07  End Sub\n08\n09  Public Function BuildReport() As String\n10      BuildReport = \"ready\"\n11  End Function\n",
        ),
        MockState::Semantic => String::from(
            "01  Option Explicit\n02\n03  Public Sub Main()\n04      Dim total As Integer\n05      total = ComputeAnswer()\n06      Debug.Print total\n07  End Sub\n08\n09  Public Function ComputeAnswer() As Integer\n10      ComputeAnswer = 42\n11  End Function\n",
        ),
        MockState::BuildRun => String::from(
            "01  Option Explicit\n02\n03  Public Sub Main()\n04      Dim total As Integer\n05  >   total = ComputeAnswer()\n06      Debug.Print total\n07  End Sub\n08\n09  Public Function ComputeAnswer() As Integer\n10      ComputeAnswer = 42\n11  End Function\n",
        ),
    }
}

fn inspector_text(state: &ShellState) -> String {
    match state.mock_state {
        MockState::Empty => format!(
            "Setup Summary\n\nCapabilities\n  Truecolor: yes\n  Unicode: yes\n  Mouse: optional\n\nTheme\n  {}\n  {}\n",
            theme::palette_name(),
            theme::token_summary()
        ),
        MockState::Editing => String::from(
            "Diagnostics\n\n0 errors\n1 warning\n\nSymbols\n  Main\n  BuildReport\n\nStatus\n  Dirty: no\n  Views: 1\n",
        ),
        MockState::Semantic => String::from(
            "Hover\n\nComputeAnswer() As Integer\nReturns the canonical answer used by the demo flow.\n\nSymbols\n  Main\n> ComputeAnswer\n\nRefs\n  3 matches\n",
        ),
        MockState::BuildRun => String::from(
            "Run Status\n\nBuild: passing\nRuntime: active\nProfile: win-console\nLast exit: 0\n\nHost\n  direct session planned\n",
        ),
        MockState::Palette => String::from(
            "Symbols\n\n  Main\n  ComputeAnswer\n  BuildReport\n\nHints\n  Palette owns focus\n",
        ),
    }
}

fn lower_surface_text(state: &ShellState) -> String {
    let base = match state.mock_state {
        MockState::Empty => String::from(
            "Launcher\n\nOpen Project\nCreate Project\nRecent: Payroll.basproj\nCapability details available on demand.\n",
        ),
        MockState::Editing => String::from(
            "Problems\n\nwarning: BuildReport is not yet called\nhint: use inspector for symbols and summary\n",
        ),
        MockState::Semantic => String::from(
            "References\n\nMainModule.bas:5 ComputeAnswer()\nHelpers.bas:12 ComputeAnswer()\nImmediate\n? ComputeAnswer()\n42\n",
        ),
        MockState::BuildRun => String::from(
            "Output\n\n[build] compiling Payroll.basproj\n[run] launching Exe target\nstdout:\n42\n\nBuild Log\n  bundle ready\n",
        ),
        MockState::Palette => {
            String::from("Problems\n\nBackground shell is frozen while the palette is active.\n")
        }
    };

    if state.inspector_is_collapsed() && !matches!(state.mock_state, MockState::Empty) {
        format!("{base}\nCollapsed Inspector\n{}\n", inspector_text(state))
    } else {
        base
    }
}

fn palette_text(state: &ShellState) -> String {
    let state_hint = match state.mock_state {
        MockState::Empty => "Start from empty shell",
        MockState::Editing => "Editing shell commands",
        MockState::Semantic => "Semantic shell commands",
        MockState::BuildRun => "Build/run shell commands",
        MockState::Palette => "Palette overlay active",
    };

    format!(
        "Command Palette\n\nFilter\n  > {}\n\nCommands\n  Open Project              Ctrl+O\n  New Project               Ctrl+N\n  Focus Explorer            Alt+1\n  Focus Editor              Alt+2\n  Focus Inspector           Alt+3\n  Focus Lower Surface       Alt+4\n  Toggle Palette            F6\n\nMockup States\n  F2 Empty\n  F3 Editing\n  F4 Semantic\n  F5 Build/Run\n\nCurrent Focus Owner\n  {}\n",
        state_hint,
        match state.focus {
            FocusRegion::Palette => "Palette",
            _ => "Shell",
        }
    )
}
