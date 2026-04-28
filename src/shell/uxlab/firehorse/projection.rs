#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FireHorseProjection {
    pub scenario_id: &'static str,
    pub expected_layout: LayoutPosture,
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
    pub seams: SeamFixtureSet,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LayoutPosture {
    Launchpad,
    Editing,
    CommandLens,
    RunLane,
    DebugCockpit,
    ConsoleFit,
    CompactFocus,
}

impl LayoutPosture {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Launchpad => "Launchpad",
            Self::Editing => "Editing",
            Self::CommandLens => "CommandLens",
            Self::RunLane => "RunLane",
            Self::DebugCockpit => "DebugCockpit",
            Self::ConsoleFit => "ConsoleFit",
            Self::CompactFocus => "CompactFocus",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThemeProjection {
    GraphiteEmber,
    PaperEmber,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityRailProjection {
    pub product: &'static str,
    pub workspace_label: String,
    pub scene: LayoutPosture,
    pub target: Option<String>,
    pub health: Vec<StateBadgeProjection>,
    pub cursor: Option<CursorProjection>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateBadgeProjection {
    pub label: String,
    pub tone: BadgeTone,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BadgeTone {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CursorProjection {
    pub line: u32,
    pub column: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectSpineProjection {
    pub posture: SpinePosture,
    pub rows: Vec<ProjectSpineRowProjection>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpinePosture {
    Full,
    CompactPeek,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectSpineRowProjection {
    pub label: String,
    pub kind: ProjectItemKind,
    pub depth: u8,
    pub active: bool,
    pub dirty: bool,
    pub badges: Vec<StateBadgeProjection>,
    pub seam_ref: Option<ProjectSeamRef>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectItemKind {
    Project,
    Module,
    Class,
    Form,
    Reference,
    Target,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectSeamRef {
    pub project_id: String,
    pub item_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodeCanvasProjection {
    pub document_label: String,
    pub language: &'static str,
    pub lines: Vec<SourceLineProjection>,
    pub lens: Option<SourceLensProjection>,
    pub execution_line: Option<u32>,
    pub selection: Option<SourceRangeProjection>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceLineProjection {
    pub number: u32,
    pub text: String,
    pub markers: Vec<GutterMarkerProjection>,
    pub semantic_spans: Vec<SemanticSpanProjection>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GutterMarkerProjection {
    Diagnostic,
    Breakpoint,
    Execution,
    Generated,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticSpanProjection {
    pub range: SourceRangeProjection,
    pub kind: SemanticKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SemanticKind {
    Procedure,
    Variable,
    Literal,
    Generated,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SourceRangeProjection {
    pub start: SourcePositionProjection,
    pub end: SourcePositionProjection,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SourcePositionProjection {
    pub line: u32,
    pub column: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceLensProjection {
    pub anchor: SourceRangeProjection,
    pub title: String,
    pub body: Vec<String>,
    pub actions: Vec<ActionHintProjection>,
    pub source: SeamSourceProjection,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeamSourceProjection {
    pub provider: &'static str,
    pub query: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContextDockProjection {
    pub title: String,
    pub cards: Vec<ContextCardProjection>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContextCardProjection {
    Diagnostic(MockDiagnosticProjection),
    Symbol(MockSymbolProjection),
    RunStatus(RunStatusProjection),
    CallStack(Vec<StackFrameProjection>),
    Locals(Vec<LocalValueProjection>),
    Watches(Vec<WatchProjection>),
    TerminalFit(TerminalFitProjection),
    Unavailable(UnavailableProjection),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RunStatusProjection {
    pub target_id: String,
    pub active_step: RunStepKind,
    pub status: RunStepStatus,
    pub message: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivityDeckProjection {
    pub posture: DeckPosture,
    pub active: ActivityKind,
    pub tabs: Vec<ActivityTabProjection>,
    pub rows: Vec<ActivityRowProjection>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeckPosture {
    Rail,
    Compact,
    Expanded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActivityKind {
    Problems,
    Output,
    RunTimeline,
    Immediate,
    References,
    WatchTrace,
}

impl ActivityKind {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Problems => "Problems",
            Self::Output => "Output",
            Self::RunTimeline => "Run Timeline",
            Self::Immediate => "Immediate",
            Self::References => "References",
            Self::WatchTrace => "Watch/Trace",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivityTabProjection {
    pub kind: ActivityKind,
    pub label: String,
    pub count: Option<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActivityRowProjection {
    Diagnostic(MockDiagnosticProjection),
    Symbol(MockSymbolProjection),
    RunEvent(MockRunEventProjection),
    StackFrame(StackFrameProjection),
    Local(LocalValueProjection),
    Watch(WatchProjection),
    Text { source: &'static str, text: String },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyRailProjection {
    pub hints: Vec<ActionHintProjection>,
    pub no_wrap: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionHintProjection {
    pub label: String,
    pub binding: KeyBindingProjection,
    pub action_id: &'static str,
    pub enabled: bool,
    pub disabled_reason: Option<String>,
    pub display_only_reason: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyBindingProjection {
    pub label: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OverlayProjection {
    CommandLens(CommandLensProjection),
}

impl OverlayProjection {
    pub const fn name(&self) -> &'static str {
        match self {
            Self::CommandLens(_) => "CommandLens",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandLensProjection {
    pub filter: String,
    pub selected_action_id: &'static str,
    pub rows: Vec<CommandRowProjection>,
    pub preview: CommandPreviewProjection,
    pub footer_hints: Vec<ActionHintProjection>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandRowProjection {
    pub action_id: &'static str,
    pub label: String,
    pub binding: Option<KeyBindingProjection>,
    pub enabled: bool,
    pub disabled_reason: Option<String>,
    pub preview: CommandPreviewProjection,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandPreviewProjection {
    pub title: String,
    pub body: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalFitProjection {
    pub summary: String,
    pub rows: Vec<TerminalFitRowProjection>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalFitRowProjection {
    pub signal: &'static str,
    pub result: FitResult,
    pub detail: String,
    pub recommendation: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FitResult {
    Pass,
    Warn,
    Fail,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnavailableProjection {
    pub source: &'static str,
    pub reason: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeamFixtureSet {
    pub diagnostics: Vec<MockDiagnosticProjection>,
    pub symbols: Vec<MockSymbolProjection>,
    pub run_events: Vec<MockRunEventProjection>,
    pub debug_frames: Vec<StackFrameProjection>,
    pub locals: Vec<LocalValueProjection>,
    pub watches: Vec<WatchProjection>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MockDiagnosticProjection {
    pub document_id: String,
    pub range: SourceRangeProjection,
    pub severity: DiagnosticSeverity,
    pub code: String,
    pub message: String,
    pub provenance: DiagnosticProvenanceProjection,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Info,
    Hint,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiagnosticProvenanceProjection {
    pub provider: &'static str,
    pub project_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MockSymbolProjection {
    pub document_id: String,
    pub range: SourceRangeProjection,
    pub kind: SymbolKind,
    pub name: String,
    pub detail: String,
    pub provenance: SymbolProvenanceProjection,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SymbolKind {
    Procedure,
    Function,
    Variable,
    Module,
    Reference,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SymbolProvenanceProjection {
    pub provider: &'static str,
    pub document_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MockRunEventProjection {
    pub target_id: String,
    pub step: RunStepKind,
    pub status: RunStepStatus,
    pub message: String,
    pub emitted_at_ms: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RunStepKind {
    Prepare,
    Analyze,
    Build,
    Execute,
    Result,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RunStepStatus {
    Pending,
    Active,
    Complete,
    Failed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StackFrameProjection {
    pub frame_id: String,
    pub procedure: String,
    pub document_id: String,
    pub line: u32,
    pub source: SeamSourceProjection,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalValueProjection {
    pub name: String,
    pub type_label: String,
    pub value: String,
    pub source: SeamSourceProjection,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WatchProjection {
    pub expression: String,
    pub type_label: String,
    pub value: String,
    pub source: SeamSourceProjection,
}
