use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellScene {
    Empty,
    Editing,
    Semantic,
    BuildRun,
    Palette,
    ComReference,
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

// NOTE: `FocusRegion::label` used to return "Top" / "Explorer" / etc. for
// rendering focus-ring badges in the top bar. Those labels were internal
// taxonomy leaking to a user surface (uxpass P1 / D5) and the top bar no
// longer carries them, so the helper is gone. Focus is now communicated by
// the visible `> ... <` border decoration and by the status line.

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

    // NOTE: `WidthClass::label` used to feed a "Wide" / "Standard" / "Narrow"
    // badge into the top bar. Per uxpass D4, layout width-class names are
    // dev telemetry and must not surface as user-visible labels. Width is
    // still observed (to pick Wide vs Narrow body layouts) but no longer
    // named.
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

/// Newline convention used when writing a buffer back to disk.
///
/// Detected once on load (`LineEnding::detect`) and preserved across
/// edits so a round-trip through OxIde does not gratuitously change
/// the checkout's line endings. Matches the on-disk convention of the
/// file at the moment it was first read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEnding {
    Lf,
    CrLf,
}

impl LineEnding {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Lf => "\n",
            Self::CrLf => "\r\n",
        }
    }

    /// Detect the file's newline convention. CRLF takes precedence if
    /// any CRLF sequence is present; otherwise LF. An empty / single-line
    /// file returns `Lf` by convention (matches `git init` and most
    /// Unix tooling).
    pub fn detect(text: &str) -> Self {
        if text.contains("\r\n") {
            Self::CrLf
        } else {
            Self::Lf
        }
    }
}

/// A snapshot of a buffer's mutable editor state, used as a single
/// undo/redo entry. We snapshot the whole `lines` vector and the
/// cursor before every edit primitive (`insert_char`, `insert_newline`,
/// `backspace_char`). This is simple and correct; the quadratic cost
/// is acceptable for human-sized edits, and the capacity cap on
/// `BufferHistory` bounds worst-case memory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BufferSnapshot {
    pub lines: Vec<String>,
    pub cursor: CursorPosition,
}

/// Per-buffer undo/redo history. Every accepted edit pushes a
/// pre-edit snapshot onto `undo` and clears `redo`. `Undo` pops from
/// `undo` onto `redo`; `Redo` pops from `redo` onto `undo`. The
/// redo stack is invalidated by any new edit (standard editor
/// convention).
///
/// Capacity is bounded to keep memory predictable. When the undo
/// stack is full, the oldest entry is dropped (ring-buffer
/// semantics); the user can still undo the last N edits but not
/// further. 64 entries is small enough to be cheap and large enough
/// to cover a typical intra-session edit burst.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BufferHistory {
    undo: Vec<BufferSnapshot>,
    redo: Vec<BufferSnapshot>,
    capacity: usize,
}

impl BufferHistory {
    pub const DEFAULT_CAPACITY: usize = 64;

    pub fn new() -> Self {
        Self::with_capacity(Self::DEFAULT_CAPACITY)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            undo: Vec::new(),
            redo: Vec::new(),
            capacity,
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }

    /// Push a pre-edit snapshot and invalidate any redo history.
    /// Called by every edit primitive before it mutates the buffer.
    pub fn record(&mut self, snapshot: BufferSnapshot) {
        if self.undo.len() == self.capacity {
            self.undo.remove(0);
        }
        self.undo.push(snapshot);
        self.redo.clear();
    }

    /// Swap the most recent undo entry with the caller-supplied
    /// current snapshot and return the snapshot to apply. `None` if
    /// nothing to undo.
    pub fn undo(&mut self, current: BufferSnapshot) -> Option<BufferSnapshot> {
        let previous = self.undo.pop()?;
        if self.redo.len() == self.capacity {
            self.redo.remove(0);
        }
        self.redo.push(current);
        Some(previous)
    }

    pub fn redo(&mut self, current: BufferSnapshot) -> Option<BufferSnapshot> {
        let next = self.redo.pop()?;
        if self.undo.len() == self.capacity {
            self.undo.remove(0);
        }
        self.undo.push(current);
        Some(next)
    }
}

impl Default for BufferHistory {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BufferState {
    pub id: BufferId,
    pub title: String,
    pub kind: BufferKind,
    pub dirty: bool,
    pub lines: Vec<String>,
    /// Absolute path on disk, if this buffer is backed by a file.
    /// `None` for mock/demo buffers and for the Welcome buffer; save
    /// is a no-op on buffers without a `source_path`.
    pub source_path: Option<PathBuf>,
    /// Newline convention preserved from the file as last read.
    /// Used only when writing back to disk. Defaults to `Lf` for
    /// buffers that were never on disk.
    pub line_ending: LineEnding,
    /// Whether the on-disk file ended with a trailing newline when
    /// loaded. Preserved across edits so the save round-trips
    /// faithfully instead of appending or stripping a terminator.
    pub trailing_newline: bool,
    /// Per-buffer undo/redo history. Edit primitives record a
    /// pre-edit snapshot here; `Msg::UndoActiveBuffer` pops from it.
    pub history: BufferHistory,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceProjectTargetKind {
    BasProj,
    Vbp,
    ConventionDirectory,
}

impl WorkspaceProjectTargetKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::BasProj => "BasProj",
            Self::Vbp => "Vbp",
            Self::ConventionDirectory => "Convention",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceProjectModuleKind {
    Module,
    Class,
    Document,
}

impl WorkspaceProjectModuleKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Module => "Module",
            Self::Class => "Class",
            Self::Document => "Document",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceProjectModuleState {
    pub kind: WorkspaceProjectModuleKind,
    pub include: String,
    pub source_path: PathBuf,
    pub logical_name: String,
    pub declared_name: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceProjectReferenceKind {
    Project,
    Com,
    Native,
}

impl WorkspaceProjectReferenceKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Project => "Project",
            Self::Com => "COM",
            Self::Native => "Native",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceProjectReferenceState {
    pub kind: WorkspaceProjectReferenceKind,
    pub include: String,
    pub referenced_project_name: Option<String>,
    pub path: Option<String>,
    pub guid: Option<String>,
    pub import_lib: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceProjectState {
    pub workspace_kind: WorkspaceProjectTargetKind,
    pub workspace_target: PathBuf,
    pub project_file: Option<PathBuf>,
    pub project_dir: PathBuf,
    pub output_type: String,
    pub modules: Vec<WorkspaceProjectModuleState>,
    pub references: Vec<WorkspaceProjectReferenceState>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceState {
    pub project_name: Option<String>,
    pub target_name: String,
    pub project: Option<WorkspaceProjectState>,
    pub buffers: Vec<BufferState>,
    pub recent_buffers: Vec<BufferId>,
    pub views: Vec<ViewState>,
    pub layout: WorkspaceLayoutState,
    pub semantic: Option<WorkspaceSemanticState>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceSemanticState {
    pub diagnostics: Vec<String>,
    pub symbols: Vec<String>,
    pub hover_lines: Vec<String>,
    pub references: Vec<String>,
}

impl WorkspaceState {
    pub fn active_view(&self) -> Option<&ViewState> {
        self.views
            .iter()
            .find(|view| view.id == self.layout.active_view)
    }

    fn active_view_mut(&mut self) -> Option<&mut ViewState> {
        let active_view = self.layout.active_view;
        self.views.iter_mut().find(|view| view.id == active_view)
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

    fn buffer_mut(&mut self, id: BufferId) -> Option<&mut BufferState> {
        self.buffers.iter_mut().find(|buffer| buffer.id == id)
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

    pub fn move_cursor_left(&mut self) {
        let Some(buffer_id) = self.active_view().map(|view| view.buffer_id) else {
            return;
        };
        let cursor = self
            .active_view()
            .map(|view| view.surface.cursor)
            .unwrap_or(CursorPosition::new(1, 1));

        if cursor.column > 1 {
            if let Some(view) = self.active_view_mut() {
                view.surface.cursor.column -= 1;
            }
            return;
        }

        if cursor.line <= 1 {
            return;
        }

        let previous_line = cursor.line - 1;
        let previous_line_len = self
            .buffer(buffer_id)
            .and_then(|buffer| {
                buffer
                    .lines
                    .get(usize::from(previous_line.saturating_sub(1)))
                    .map(|line| line.chars().count())
            })
            .unwrap_or(0);

        if let Some(view) = self.active_view_mut() {
            view.surface.cursor.line = previous_line;
            view.surface.cursor.column = saturating_editor_column(previous_line_len);
        }
    }

    pub fn move_cursor_right(&mut self) {
        let Some(buffer_id) = self.active_view().map(|view| view.buffer_id) else {
            return;
        };
        let current = self
            .active_view()
            .map(|view| view.surface.cursor)
            .unwrap_or(CursorPosition::new(1, 1));
        let line_len = self
            .buffer(buffer_id)
            .and_then(|buffer| {
                buffer
                    .lines
                    .get(usize::from(current.line.saturating_sub(1)))
                    .map(|line| line.chars().count())
            })
            .unwrap_or(0);
        let max_column = saturating_editor_column(line_len);
        if current.column < max_column {
            if let Some(view) = self.active_view_mut() {
                view.surface.cursor.column += 1;
            }
            return;
        }

        let total_lines = self
            .buffer(buffer_id)
            .map(|buffer| buffer.lines.len())
            .unwrap_or(0);
        if usize::from(current.line) >= total_lines {
            return;
        }

        if let Some(view) = self.active_view_mut() {
            view.surface.cursor.line += 1;
            view.surface.cursor.column = 1;
        }
    }

    pub fn move_cursor_up(&mut self) {
        let Some(buffer_id) = self.active_view().map(|view| view.buffer_id) else {
            return;
        };
        let cursor = self
            .active_view()
            .map(|view| view.surface.cursor)
            .unwrap_or(CursorPosition::new(1, 1));

        if cursor.line <= 1 {
            return;
        }

        let target_line = cursor.line - 1;
        let target_line_len = self
            .buffer(buffer_id)
            .and_then(|buffer| {
                buffer
                    .lines
                    .get(usize::from(target_line.saturating_sub(1)))
                    .map(|line| line.chars().count())
            })
            .unwrap_or(0);

        if let Some(view) = self.active_view_mut() {
            view.surface.cursor.line = target_line;
            view.surface.cursor.column =
                cursor.column.min(saturating_editor_column(target_line_len));
        }
    }

    pub fn move_cursor_down(&mut self) {
        let Some(buffer_id) = self.active_view().map(|view| view.buffer_id) else {
            return;
        };
        let cursor = self
            .active_view()
            .map(|view| view.surface.cursor)
            .unwrap_or(CursorPosition::new(1, 1));
        let total_lines = self
            .buffer(buffer_id)
            .map(|buffer| buffer.lines.len())
            .unwrap_or(0);
        if usize::from(cursor.line) >= total_lines {
            return;
        }

        let target_line = cursor.line + 1;
        let target_line_len = self
            .buffer(buffer_id)
            .and_then(|buffer| {
                buffer
                    .lines
                    .get(usize::from(target_line.saturating_sub(1)))
                    .map(|line| line.chars().count())
            })
            .unwrap_or(0);

        if let Some(view) = self.active_view_mut() {
            view.surface.cursor.line = target_line;
            view.surface.cursor.column =
                cursor.column.min(saturating_editor_column(target_line_len));
        }
    }

    pub fn insert_char(&mut self, ch: char) {
        let Some(buffer_id) = self.active_view().map(|view| view.buffer_id) else {
            return;
        };
        let cursor = self
            .active_view()
            .map(|view| view.surface.cursor)
            .unwrap_or(CursorPosition::new(1, 1));
        self.record_edit_snapshot(buffer_id, cursor);

        if let Some(buffer) = self.buffer_mut(buffer_id) {
            ensure_buffer_line(buffer, cursor.line);
            let line_index = usize::from(cursor.line.saturating_sub(1));
            let column_index = column_to_byte_index(&buffer.lines[line_index], cursor.column);
            buffer.lines[line_index].insert(column_index, ch);
            buffer.dirty = true;
        }

        if let Some(view) = self.active_view_mut() {
            view.surface.cursor.column += 1;
        }
        self.semantic = None;
    }

    pub fn insert_newline(&mut self) {
        let Some(buffer_id) = self.active_view().map(|view| view.buffer_id) else {
            return;
        };
        let cursor = self
            .active_view()
            .map(|view| view.surface.cursor)
            .unwrap_or(CursorPosition::new(1, 1));
        self.record_edit_snapshot(buffer_id, cursor);

        if let Some(buffer) = self.buffer_mut(buffer_id) {
            ensure_buffer_line(buffer, cursor.line);
            let line_index = usize::from(cursor.line.saturating_sub(1));
            let column_index = column_to_byte_index(&buffer.lines[line_index], cursor.column);
            let remainder = buffer.lines[line_index].split_off(column_index);
            buffer.lines.insert(line_index + 1, remainder);
            buffer.dirty = true;
        }

        if let Some(view) = self.active_view_mut() {
            view.surface.cursor.line += 1;
            view.surface.cursor.column = 1;
        }
        self.semantic = None;
    }

    pub fn backspace(&mut self) {
        let Some(buffer_id) = self.active_view().map(|view| view.buffer_id) else {
            return;
        };
        let cursor = self
            .active_view()
            .map(|view| view.surface.cursor)
            .unwrap_or(CursorPosition::new(1, 1));

        if cursor.column > 1 {
            self.record_edit_snapshot(buffer_id, cursor);
            if let Some(buffer) = self.buffer_mut(buffer_id) {
                ensure_buffer_line(buffer, cursor.line);
                let line_index = usize::from(cursor.line.saturating_sub(1));
                let start = column_to_byte_index(
                    &buffer.lines[line_index],
                    CursorPosition::new(cursor.line, cursor.column - 1).column,
                );
                let end = column_to_byte_index(&buffer.lines[line_index], cursor.column);
                buffer.lines[line_index].replace_range(start..end, "");
                buffer.dirty = true;
            }
            if let Some(view) = self.active_view_mut() {
                view.surface.cursor.column -= 1;
            }
            self.semantic = None;
            return;
        }

        if cursor.line <= 1 {
            return;
        }

        let line_index = usize::from(cursor.line.saturating_sub(1));
        let buffer_line_count = self
            .buffer(buffer_id)
            .map(|buffer| buffer.lines.len())
            .unwrap_or(0);
        if line_index >= buffer_line_count {
            return;
        }
        self.record_edit_snapshot(buffer_id, cursor);

        if let Some(buffer) = self.buffer_mut(buffer_id) {
            let removed = buffer.lines.remove(line_index);
            let previous_len = buffer.lines[line_index - 1].chars().count();
            buffer.lines[line_index - 1].push_str(&removed);
            buffer.dirty = true;

            if let Some(view) = self.active_view_mut() {
                view.surface.cursor.line -= 1;
                view.surface.cursor.column = saturating_editor_column(previous_len);
            }
        }
        self.semantic = None;
    }

    /// Push the current (pre-edit) buffer contents onto the buffer's
    /// undo stack. Called by every edit primitive before it mutates
    /// the buffer, so `Undo` always returns to the exact state prior
    /// to the most recent edit. Redo history is cleared because any
    /// new edit invalidates a previously-undone future (standard
    /// editor behaviour).
    fn record_edit_snapshot(&mut self, buffer_id: BufferId, cursor: CursorPosition) {
        let Some(buffer) = self.buffer_mut(buffer_id) else {
            return;
        };
        let snapshot = BufferSnapshot {
            lines: buffer.lines.clone(),
            cursor,
        };
        buffer.history.record(snapshot);
    }

    /// Apply the top undo entry of the active buffer. Returns `true`
    /// if something was undone, `false` if the stack was empty. The
    /// buffer's `dirty` flag is left unchanged — undoing may still
    /// leave the buffer divergent from disk (the common case), and
    /// the user judges "is this the saved state?" from the editor's
    /// dirty marker which they saved.
    pub fn undo_active_buffer(&mut self) -> bool {
        let Some(buffer_id) = self.active_view().map(|view| view.buffer_id) else {
            return false;
        };
        let cursor = self
            .active_view()
            .map(|view| view.surface.cursor)
            .unwrap_or(CursorPosition::new(1, 1));

        let current = BufferSnapshot {
            lines: self
                .buffer(buffer_id)
                .map(|buffer| buffer.lines.clone())
                .unwrap_or_default(),
            cursor,
        };

        let Some(previous) = self
            .buffer_mut(buffer_id)
            .and_then(|buffer| buffer.history.undo(current))
        else {
            return false;
        };

        if let Some(buffer) = self.buffer_mut(buffer_id) {
            buffer.lines = previous.lines;
            buffer.dirty = true;
        }
        if let Some(view) = self.active_view_mut() {
            view.surface.cursor = previous.cursor;
        }
        self.semantic = None;
        true
    }

    pub fn redo_active_buffer(&mut self) -> bool {
        let Some(buffer_id) = self.active_view().map(|view| view.buffer_id) else {
            return false;
        };
        let cursor = self
            .active_view()
            .map(|view| view.surface.cursor)
            .unwrap_or(CursorPosition::new(1, 1));

        let current = BufferSnapshot {
            lines: self
                .buffer(buffer_id)
                .map(|buffer| buffer.lines.clone())
                .unwrap_or_default(),
            cursor,
        };

        let Some(next) = self
            .buffer_mut(buffer_id)
            .and_then(|buffer| buffer.history.redo(current))
        else {
            return false;
        };

        if let Some(buffer) = self.buffer_mut(buffer_id) {
            buffer.lines = next.lines;
            buffer.dirty = true;
        }
        if let Some(view) = self.active_view_mut() {
            view.surface.cursor = next.cursor;
        }
        self.semantic = None;
        true
    }

    /// Return whether the currently-active buffer has any undo
    /// history. Used to gate the `Undo` palette entry / status-line
    /// hint from promising what it cannot deliver (P6).
    pub fn can_undo_active_buffer(&self) -> bool {
        self.active_buffer()
            .map(|buffer| buffer.history.can_undo())
            .unwrap_or(false)
    }

    pub fn can_redo_active_buffer(&self) -> bool {
        self.active_buffer()
            .map(|buffer| buffer.history.can_redo())
            .unwrap_or(false)
    }

    /// Serialize a buffer's current lines to the byte sequence that
    /// should be written to its source file. Preserves the detected
    /// line ending and the original trailing-newline-or-not.
    fn serialize_buffer_for_save(buffer: &BufferState) -> String {
        let mut text = buffer.lines.join(buffer.line_ending.as_str());
        if buffer.trailing_newline {
            text.push_str(buffer.line_ending.as_str());
        }
        text
    }

    /// Write the active buffer's in-memory lines back to its source
    /// file. Returns:
    /// - `Ok(true)` if the buffer was written (dirty was true and it
    ///   had a `source_path`),
    /// - `Ok(false)` if there was nothing to save (no active buffer,
    ///   not dirty, or no `source_path` — the Welcome buffer falls
    ///   in this category), and
    /// - `Err(io::Error)` if the write failed. Callers decide how
    ///   to surface the error; today the message is not surfaced to
    ///   the user (an overlay-based save-error dialog is a follow-up
    ///   belonging to W050/W060 UX polish).
    ///
    /// The dirty flag is cleared on success.
    pub fn save_active_buffer(&mut self) -> std::io::Result<bool> {
        let Some(buffer_id) = self.active_view().map(|view| view.buffer_id) else {
            return Ok(false);
        };
        self.save_buffer(buffer_id)
    }

    /// Iterate every dirty buffer and attempt to save it. Returns a
    /// list of `(buffer_title, error)` pairs for buffers whose save
    /// failed; saves that skipped (not dirty, no source_path) are
    /// silent. Successful saves clear `dirty`.
    pub fn save_all_dirty_buffers(&mut self) -> Vec<(String, std::io::Error)> {
        let ids: Vec<BufferId> = self
            .buffers
            .iter()
            .filter(|buffer| buffer.dirty)
            .map(|buffer| buffer.id)
            .collect();
        let mut errors = Vec::new();
        for id in ids {
            match self.save_buffer(id) {
                Ok(_) => {}
                Err(err) => {
                    if let Some(buffer) = self.buffer(id) {
                        errors.push((buffer.title.clone(), err));
                    }
                }
            }
        }
        errors
    }

    fn save_buffer(&mut self, buffer_id: BufferId) -> std::io::Result<bool> {
        let Some(buffer) = self.buffer(buffer_id) else {
            return Ok(false);
        };
        if !buffer.dirty {
            return Ok(false);
        }
        let Some(path) = buffer.source_path.clone() else {
            // Welcome / in-memory fixtures are not persisted. Silently
            // skip; not a failure, just a no-op.
            return Ok(false);
        };
        let serialized = Self::serialize_buffer_for_save(buffer);
        std::fs::write(&path, serialized)?;
        if let Some(buffer) = self.buffer_mut(buffer_id) {
            buffer.dirty = false;
        }
        Ok(true)
    }
}

fn ensure_buffer_line(buffer: &mut BufferState, line: u16) {
    let index = usize::from(line.saturating_sub(1));
    while buffer.lines.len() <= index {
        buffer.lines.push(String::new());
    }
}

fn column_to_byte_index(line: &str, column: u16) -> usize {
    let char_index = usize::from(column.saturating_sub(1));
    line.char_indices()
        .nth(char_index)
        .map(|(index, _)| index)
        .unwrap_or(line.len())
}

fn saturating_editor_column(char_len: usize) -> u16 {
    u16::try_from(char_len.saturating_add(1)).unwrap_or(u16::MAX)
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

/// Launcher content used by the Welcome / Empty-scene body.
///
/// Uxpass D1b retired the separate `capabilities` and `hint` fields:
/// - `hint` used to be rendered as a trailing `Hint\n  …` paragraph
///   inside Welcome, advertising the same `Ctrl+O` / `Up/Down` /
///   `F6` bindings that the status line already surfaces (D3 / D8).
///   Repeating them is noise (P2 / P3), and the line was also silently
///   truncating on Standard-width frames (D7). The status line owns the
///   contract now.
/// - `capabilities` (`Truecolor detected`, …) was informational, not
///   actionable; terminal capability onboarding is the W100 workset's
///   first-run probe page, not a decoration on the Empty body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherContentState {
    pub recent_projects: Vec<String>,
    pub actions: Vec<&'static str>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComReferenceSearchMode {
    Search,
    File,
}

impl ComReferenceSearchMode {
    pub fn label(self) -> &'static str {
        match self {
            Self::Search => "Search",
            Self::File => "File",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComReferenceCandidateState {
    pub title: String,
    pub detail_lines: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComReferenceHelperState {
    pub mode: ComReferenceSearchMode,
    pub query: String,
    pub selection: usize,
    pub candidates: Vec<ComReferenceCandidateState>,
    pub active_reference_lines: Vec<String>,
    pub status_lines: Vec<String>,
}

impl Default for ComReferenceHelperState {
    fn default() -> Self {
        Self {
            mode: ComReferenceSearchMode::Search,
            query: String::new(),
            selection: 0,
            candidates: Vec::new(),
            active_reference_lines: Vec::new(),
            status_lines: vec![String::from(
                "Type an exact library name or ProgID; file mode accepts absolute .tlb/.dll/.xll paths",
            )],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellContentState {
    pub launcher: LauncherContentState,
    pub editor_notes: Vec<String>,
    pub inspector: PanelContentState,
    pub lower_surface: PanelContentState,
    pub palette: PaletteContentState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionState {
    pub profile: String,
    pub entry_point: String,
    pub build_status: String,
    pub runtime_status: String,
    pub last_exit_code: Option<i32>,
    pub output_lines: Vec<String>,
    pub log_lines: Vec<String>,
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
            inspector_collapsed: width_class == WidthClass::Narrow
                && !matches!(scene, ShellScene::Palette | ShellScene::ComReference),
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
    pub execution: ExecutionState,
    pub recent_projects: Vec<PathBuf>,
    pub launcher_selection: usize,
    pub content: ShellContentState,
    pub com_reference_helper: ComReferenceHelperState,
    pub previous_focus: FocusRegion,
    pub previous_scene: ShellScene,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShellState {
    pub scene: ShellScene,
    /// When `true`, the `F2/F3/F4` scene-flip affordances and the palette's
    /// `Mockup States` section are available. Off by default; enable with
    /// `--dev-scenes` on the CLI. See uxpass `00_principles.md` decision D6.
    pub dev_scenes: bool,
    pub runtime: ShellRuntimeState,
}

impl Default for ShellState {
    fn default() -> Self {
        let width_class = WidthClass::Standard;
        let scene = ShellScene::Editing;
        let dev_scenes = false;
        let workspace = workspace_for_scene(scene);
        let execution = execution_for_workspace(&workspace);
        let mut state = Self {
            scene,
            dev_scenes,
            runtime: ShellRuntimeState {
                focus: FocusRegion::Editor,
                inspector_mode: InspectorMode::Diagnostics,
                lower_mode: LowerSurfaceMode::Problems,
                width_class,
                size: (120, 40),
                layout: ShellLayoutPolicy::derive(scene, width_class),
                workspace: workspace.clone(),
                session_workspace: None,
                execution: execution.clone(),
                recent_projects: Vec::new(),
                launcher_selection: 0,
                content: content_for_scene(scene, &workspace, &execution, &[], 0, dev_scenes),
                com_reference_helper: ComReferenceHelperState::default(),
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
        let previous_scene = self.scene;
        self.scene = scene;
        self.runtime.layout = ShellLayoutPolicy::derive(scene, self.runtime.width_class);
        self.runtime.workspace = match (scene, self.runtime.session_workspace.clone()) {
            (ShellScene::Empty, _) => workspace_for_scene(scene),
            // Overlay scenes are non-destructive: preserve the current
            // workspace so in-flight buffer edits, dirty flags, cursors,
            // and per-buffer undo history all survive opening / closing
            // the overlay. Without this clause, `apply_scene(Palette)`
            // would rebuild the workspace from `session_workspace`
            // (the clean snapshot captured at mount time) and silently
            // discard anything the user typed — a P4 honesty violation
            // because the `*` dirty marker still implied the edit
            // existed after the overlay closed.
            (ShellScene::Palette | ShellScene::ComReference, _) => {
                self.runtime.workspace.clone()
            }
            // Returning from an overlay to the previous scene is also
            // non-destructive for the same reason: the in-flight
            // workspace must survive the round-trip.
            (_, Some(_))
                if matches!(
                    previous_scene,
                    ShellScene::Palette | ShellScene::ComReference
                ) =>
            {
                self.runtime.workspace.clone()
            }
            (_, Some(workspace)) => workspace_for_scene_from_loaded(scene, workspace),
            (_, None) => workspace_for_scene(scene),
        };
        self.refresh_content();
        if !matches!(scene, ShellScene::Palette | ShellScene::ComReference) {
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
            ShellScene::ComReference => {
                self.runtime.inspector_mode = InspectorMode::Symbols;
                self.runtime.lower_mode = LowerSurfaceMode::Problems;
                self.runtime.focus = FocusRegion::Palette;
            }
        }
    }

    pub fn mount_workspace(&mut self, workspace: WorkspaceState) {
        self.runtime.session_workspace = Some(workspace.clone());
        self.runtime.workspace = workspace_for_scene_from_loaded(self.scene, workspace);
        self.refresh_content();
    }

    pub fn set_execution(&mut self, execution: ExecutionState) {
        self.runtime.execution = execution;
        self.refresh_content();
    }

    pub fn set_recent_projects(&mut self, recent_projects: Vec<PathBuf>) {
        self.runtime.recent_projects = recent_projects;
        if self.runtime.launcher_selection >= self.runtime.recent_projects.len() {
            self.runtime.launcher_selection = 0;
        }
        self.refresh_content();
    }

    pub fn cycle_launcher_selection(&mut self, direction: i8) {
        if self.runtime.recent_projects.is_empty() {
            return;
        }

        let len = self.runtime.recent_projects.len();
        let index = self.runtime.launcher_selection;
        self.runtime.launcher_selection = if direction >= 0 {
            (index + 1) % len
        } else if index == 0 {
            len - 1
        } else {
            index - 1
        };
        self.refresh_content();
    }

    pub fn selected_project_path(&self) -> Option<&PathBuf> {
        self.runtime
            .recent_projects
            .get(self.runtime.launcher_selection)
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
            self.close_overlay();
        } else {
            self.runtime.previous_scene = self.scene;
            self.runtime.previous_focus = self.runtime.focus;
            self.apply_scene(ShellScene::Palette);
        }
    }

    pub fn open_com_reference_helper(&mut self) {
        if self.com_reference_helper_active() {
            return;
        }

        self.runtime.previous_scene = self.scene;
        self.runtime.previous_focus = self.runtime.focus;
        self.apply_scene(ShellScene::ComReference);
    }

    pub fn close_overlay(&mut self) {
        if !self.overlay_active() {
            return;
        }

        self.apply_scene(self.runtime.previous_scene);
        self.focus_region(self.runtime.previous_focus);
    }

    pub fn cycle_active_editor_view(&mut self) {
        self.runtime.workspace.cycle_active_view();
    }

    pub fn move_editor_cursor_left(&mut self) {
        self.runtime.workspace.move_cursor_left();
        self.refresh_content();
    }

    pub fn move_editor_cursor_right(&mut self) {
        self.runtime.workspace.move_cursor_right();
        self.refresh_content();
    }

    pub fn move_editor_cursor_up(&mut self) {
        self.runtime.workspace.move_cursor_up();
        self.refresh_content();
    }

    pub fn move_editor_cursor_down(&mut self) {
        self.runtime.workspace.move_cursor_down();
        self.refresh_content();
    }

    pub fn insert_editor_char(&mut self, ch: char) {
        self.runtime.workspace.insert_char(ch);
        self.refresh_content();
    }

    pub fn insert_editor_newline(&mut self) {
        self.runtime.workspace.insert_newline();
        self.refresh_content();
    }

    pub fn backspace_editor_char(&mut self) {
        self.runtime.workspace.backspace();
        self.refresh_content();
    }

    /// Save the active buffer to disk. See `WorkspaceState::save_active_buffer`
    /// for semantics. Dirty marker in the explorer clears on success;
    /// the editor title's trailing `*` (J2-d) disappears accordingly.
    pub fn save_active_buffer(&mut self) -> std::io::Result<bool> {
        let result = self.runtime.workspace.save_active_buffer();
        self.refresh_content();
        result
    }

    pub fn save_all_dirty_buffers(&mut self) -> Vec<(String, std::io::Error)> {
        let errors = self.runtime.workspace.save_all_dirty_buffers();
        self.refresh_content();
        errors
    }

    pub fn undo_active_buffer(&mut self) -> bool {
        let applied = self.runtime.workspace.undo_active_buffer();
        if applied {
            self.refresh_content();
        }
        applied
    }

    pub fn redo_active_buffer(&mut self) -> bool {
        let applied = self.runtime.workspace.redo_active_buffer();
        if applied {
            self.refresh_content();
        }
        applied
    }

    pub fn can_undo_active_buffer(&self) -> bool {
        self.runtime.workspace.can_undo_active_buffer()
    }

    pub fn can_redo_active_buffer(&self) -> bool {
        self.runtime.workspace.can_redo_active_buffer()
    }

    pub fn palette_active(&self) -> bool {
        self.scene == ShellScene::Palette
    }

    pub fn com_reference_helper_active(&self) -> bool {
        self.scene == ShellScene::ComReference
    }

    pub fn overlay_active(&self) -> bool {
        matches!(self.scene, ShellScene::Palette | ShellScene::ComReference)
    }

    pub fn set_com_reference_helper(&mut self, helper: ComReferenceHelperState) {
        self.runtime.com_reference_helper = helper;
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
        if self.overlay_active() {
            return vec![FocusRegion::Palette];
        }

        // Uxpass D16 (`20_frame_and_regions.md`): the top bar is
        // display-only. It carries project identity, scene label, and
        // one relevant state value, none of which the user can act on.
        // It is therefore excluded from the focus ring on every scene,
        // so `Tab` cycles only through regions where keystrokes do
        // something.
        //
        // Uxpass D1 (landed via D1a + D1b): the Empty scene renders a
        // single full-width Welcome panel (which *is* the launcher — it
        // owns both the recent-projects selection and the Start actions).
        // There is no Explorer column, no Inspector column, and no lower
        // surface to focus into, so the focus ring on Empty collapses
        // to a single entry: `Editor`. The Welcome panel is reached via
        // the Editor slot (`panels.editor`), which keeps focus-region
        // plumbing simple.
        if self.scene == ShellScene::Empty {
            return vec![FocusRegion::Editor];
        }

        let mut regions = vec![FocusRegion::Explorer, FocusRegion::Editor];
        if !self.inspector_is_collapsed() {
            regions.push(FocusRegion::Inspector);
        }
        if self.shows_lower_surface() {
            regions.push(FocusRegion::LowerSurface);
        }
        regions
    }

    /// Per-scene keystroke hint rendered in the always-present bottom
    /// status line (uxpass D3 / D8).
    ///
    /// The status line contract is "what keystrokes are available right
    /// now." It is never hidden, never wraps, and never repeats what the
    /// focused panel already shows. Bindings listed here must actually
    /// work in the current scene (P6 — every binding is documented in
    /// place). Each hint starts with the canonical action for the scene:
    /// `Ctrl+O` on Empty (D8 — the cold-start action is on the status
    /// line, not buried in a Hint paragraph), `F5` on run-capable scenes,
    /// etc.
    pub fn status_line_hint(&self) -> &'static str {
        if self.com_reference_helper_active() {
            return "Enter apply  Tab switch mode  Up/Down select  Esc close";
        }
        if self.palette_active() {
            return "Esc close  Up/Down select  Enter apply";
        }
        match self.scene {
            ShellScene::Empty => {
                "Ctrl+O open project  Up/Down select recent  F6 palette  Ctrl+Q quit"
            }
            ShellScene::Editing | ShellScene::Semantic => {
                "Ctrl+S save  F5 run  Ctrl+Z undo  F6 palette  Tab next focus  Ctrl+Q quit"
            }
            ShellScene::BuildRun => {
                "F5 rerun  F6 palette  Tab next focus  Ctrl+Q quit"
            }
            // Overlay scenes are handled by the two early-returns above;
            // these arms keep the match exhaustive without surfacing
            // unreachable hints.
            ShellScene::Palette => "Esc close  Up/Down select  Enter apply",
            ShellScene::ComReference => {
                "Enter apply  Tab switch mode  Up/Down select  Esc close"
            }
        }
    }

    /// Toggle the developer-only affordances (`F2/F3/F4` scene-flips and the
    /// palette's `Mockup States` group). See uxpass decision D6.
    pub fn set_dev_scenes(&mut self, dev_scenes: bool) {
        self.dev_scenes = dev_scenes;
        self.refresh_content();
    }

    fn refresh_content(&mut self) {
        self.runtime.content = content_for_scene(
            self.scene,
            &self.runtime.workspace,
            &self.runtime.execution,
            &self.runtime.recent_projects,
            self.runtime.launcher_selection,
            self.dev_scenes,
        );
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
            project: None,
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
                source_path: None,
                line_ending: LineEnding::Lf,
                trailing_newline: false,
                history: BufferHistory::new(),
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
            semantic: None,
        },
        ShellScene::Editing | ShellScene::Palette | ShellScene::ComReference => WorkspaceState {
            project_name: Some(String::from("Payroll.basproj")),
            target_name: String::from("Exe"),
            project: None,
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
                    source_path: None,
                    line_ending: LineEnding::Lf,
                    trailing_newline: false,
                    history: BufferHistory::new(),
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
                    source_path: None,
                    line_ending: LineEnding::Lf,
                    trailing_newline: false,
                    history: BufferHistory::new(),
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
                    source_path: None,
                    line_ending: LineEnding::Lf,
                    trailing_newline: false,
                    history: BufferHistory::new(),
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
            semantic: None,
        },
        ShellScene::Semantic => WorkspaceState {
            project_name: Some(String::from("Payroll.basproj")),
            target_name: String::from("Exe"),
            project: None,
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
                    source_path: None,
                    line_ending: LineEnding::Lf,
                    trailing_newline: false,
                    history: BufferHistory::new(),
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
                    source_path: None,
                    line_ending: LineEnding::Lf,
                    trailing_newline: false,
                    history: BufferHistory::new(),
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
                    source_path: None,
                    line_ending: LineEnding::Lf,
                    trailing_newline: false,
                    history: BufferHistory::new(),
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
            semantic: None,
        },
        ShellScene::BuildRun => WorkspaceState {
            project_name: Some(String::from("Payroll.basproj")),
            target_name: String::from("Exe"),
            project: None,
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
                    source_path: None,
                    line_ending: LineEnding::Lf,
                    trailing_newline: false,
                    history: BufferHistory::new(),
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
                    source_path: None,
                    line_ending: LineEnding::Lf,
                    trailing_newline: false,
                    history: BufferHistory::new(),
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
                    source_path: None,
                    line_ending: LineEnding::Lf,
                    trailing_newline: false,
                    history: BufferHistory::new(),
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
            semantic: None,
        },
    }
}

fn workspace_for_scene_from_loaded(
    scene: ShellScene,
    mut workspace: WorkspaceState,
) -> WorkspaceState {
    match scene {
        ShellScene::Empty => return workspace_for_scene(ShellScene::Empty),
        ShellScene::Editing | ShellScene::Palette | ShellScene::ComReference => {
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

fn execution_for_workspace(workspace: &WorkspaceState) -> ExecutionState {
    let project_name = workspace.project_name.as_deref().unwrap_or("OxIde");
    let entry_point = workspace
        .buffers
        .iter()
        .flat_map(|buffer| {
            buffer.lines.iter().filter_map(|line| {
                parse_symbol_info(buffer.title.as_str(), line, 0).map(|symbol| {
                    format!("{}.{}", buffer.title.trim_end_matches(".bas"), symbol.name)
                })
            })
        })
        .next()
        .unwrap_or_else(|| String::from("No entry point"));

    ExecutionState {
        profile: execution_profile_for_target(workspace.target_name.as_str()),
        entry_point,
        build_status: String::from("ready"),
        runtime_status: String::from("prepared"),
        last_exit_code: Some(0),
        output_lines: vec![
            format!("[build] project {project_name}"),
            format!("[build] target {}", workspace.target_name),
            format!("[build] open buffers {}", workspace.open_buffer_count()),
            String::from("[run] execution contract not attached yet"),
        ],
        log_lines: vec![
            format!("active layout {}", workspace.layout.preset.label()),
            format!("visible views {}", workspace.visible_view_count()),
        ],
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SymbolInfo {
    name: String,
    signature: String,
    buffer_title: String,
    line: usize,
    kind: &'static str,
}

fn workspace_symbol_infos(workspace: &WorkspaceState) -> Vec<SymbolInfo> {
    workspace
        .buffers
        .iter()
        .flat_map(|buffer| {
            buffer.lines.iter().enumerate().filter_map(|(index, line)| {
                parse_symbol_info(buffer.title.as_str(), line, index + 1)
            })
        })
        .collect()
}

fn workspace_symbols(workspace: &WorkspaceState) -> Vec<String> {
    if let Some(semantic) = &workspace.semantic {
        if !semantic.symbols.is_empty() {
            return semantic.symbols.clone();
        }
    }

    let mut symbols = workspace_symbol_infos(workspace)
        .into_iter()
        .map(|symbol| symbol.name)
        .collect::<Vec<_>>();

    if symbols.is_empty() {
        symbols.push(String::from("No symbols discovered"));
    }

    symbols
}

fn workspace_primary_symbol(workspace: &WorkspaceState) -> Option<SymbolInfo> {
    let symbol_infos = workspace_symbol_infos(workspace);
    let active_buffer = workspace.active_buffer()?;
    let cursor_line = workspace
        .active_view()
        .map(|view| usize::from(view.surface.cursor.line))
        .unwrap_or(1);

    if let Some(source_line) = active_buffer.lines.get(cursor_line.saturating_sub(1)) {
        if let Some(symbol) = symbol_infos
            .iter()
            .find(|symbol| line_contains_symbol_reference(source_line, symbol.name.as_str()))
        {
            return Some(symbol.clone());
        }
    }

    symbol_infos
        .iter()
        .filter(|symbol| symbol.buffer_title == active_buffer.title && symbol.line <= cursor_line)
        .next_back()
        .cloned()
        .or_else(|| {
            symbol_infos
                .iter()
                .find(|symbol| symbol.buffer_title == active_buffer.title)
                .cloned()
        })
        .or_else(|| symbol_infos.first().cloned())
}

fn workspace_hover_lines(workspace: &WorkspaceState) -> Vec<String> {
    if let Some(semantic) = &workspace.semantic {
        if !semantic.hover_lines.is_empty() {
            return semantic.hover_lines.clone();
        }
    }

    let Some(symbol) = workspace_primary_symbol(workspace) else {
        return vec![String::from("No semantic target at the current cursor")];
    };

    vec![
        symbol.signature,
        format!("Defined in {}:{}", symbol.buffer_title, symbol.line),
        format!("Kind: {}", symbol.kind),
    ]
}

fn workspace_references(workspace: &WorkspaceState) -> Vec<String> {
    if let Some(semantic) = &workspace.semantic {
        if !semantic.references.is_empty() {
            return semantic.references.clone();
        }
    }

    let Some(symbol) = workspace_primary_symbol(workspace) else {
        return vec![String::from("No references available")];
    };

    let mut references = workspace
        .buffers
        .iter()
        .flat_map(|buffer| {
            buffer.lines.iter().enumerate().filter_map(|(index, line)| {
                if line_contains_symbol_reference(line, symbol.name.as_str()) {
                    Some(format!(
                        "{}:{} {}",
                        buffer.title,
                        index + 1,
                        normalize_source_line(line)
                    ))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();

    if references.is_empty() {
        references.push(format!("No references found for {}", symbol.name));
    }

    references
}

fn workspace_diagnostics(workspace: &WorkspaceState) -> Vec<String> {
    if let Some(semantic) = &workspace.semantic {
        if !semantic.diagnostics.is_empty() {
            return semantic.diagnostics.clone();
        }
    }

    let mut diagnostics = workspace
        .buffers
        .iter()
        .filter(|buffer| buffer.kind != BufferKind::Welcome)
        .flat_map(|buffer| {
            let has_option_explicit = buffer
                .lines
                .iter()
                .any(|line| normalize_source_line(line).eq_ignore_ascii_case("Option Explicit"));

            let mut lines = Vec::new();
            if !has_option_explicit {
                lines.push(format!(
                    "warning: {} is missing Option Explicit",
                    buffer.title
                ));
            }

            let symbol_count = workspace_symbol_infos(&WorkspaceState {
                project_name: workspace.project_name.clone(),
                target_name: workspace.target_name.clone(),
                project: workspace.project.clone(),
                buffers: vec![buffer.clone()],
                recent_buffers: vec![buffer.id],
                views: workspace
                    .views
                    .iter()
                    .filter(|view| view.buffer_id == buffer.id)
                    .cloned()
                    .collect(),
                layout: workspace.layout.clone(),
                semantic: None,
            })
            .len();
            if symbol_count == 0 {
                lines.push(format!(
                    "info: {} does not expose a discoverable public symbol yet",
                    buffer.title
                ));
            }

            lines
        })
        .collect::<Vec<_>>();

    if diagnostics.is_empty() {
        diagnostics.push(String::from("No diagnostics in mounted workspace"));
    }

    diagnostics
}

fn parse_symbol_info(buffer_title: &str, line: &str, line_number: usize) -> Option<SymbolInfo> {
    let normalized = normalize_source_line(line);
    for (marker, kind) in [
        ("Public Sub ", "Sub"),
        ("Private Sub ", "Sub"),
        ("Public Function ", "Function"),
        ("Private Function ", "Function"),
        ("Public Property Get ", "Property"),
        ("Private Property Get ", "Property"),
    ] {
        if let Some(rest) = normalized.strip_prefix(marker) {
            let name = rest
                .split(['(', ' '])
                .next()
                .filter(|value| !value.is_empty())?;
            return Some(SymbolInfo {
                name: String::from(name),
                signature: normalized.to_string(),
                buffer_title: String::from(buffer_title),
                line: line_number,
                kind,
            });
        }
    }

    None
}

fn normalize_source_line(line: &str) -> &str {
    line.trim_start_matches(|char: char| char.is_ascii_digit() || char == ' ' || char == '>')
        .trim_start()
}

fn line_contains_symbol_reference(line: &str, symbol_name: &str) -> bool {
    normalize_source_line(line)
        .split(|char: char| !(char.is_ascii_alphanumeric() || char == '_'))
        .any(|token| token.eq_ignore_ascii_case(symbol_name))
}

fn execution_profile_for_target(target_name: &str) -> String {
    match target_name {
        "Exe" => String::from("win-console"),
        "Library" => String::from("library"),
        "Addin" => String::from("addin"),
        "ComServer" => String::from("com-server"),
        _ => String::from("host"),
    }
}

fn content_for_scene(
    scene: ShellScene,
    workspace: &WorkspaceState,
    execution: &ExecutionState,
    recent_projects: &[PathBuf],
    launcher_selection: usize,
    dev_scenes: bool,
) -> ShellContentState {
    let active_buffer_title = workspace
        .active_buffer()
        .map(|buffer| buffer.title.as_str())
        .unwrap_or("current buffer");
    // NOTE: cursor, dirty-buffer count, visible-view count, and selection
    // presence all used to feed dev-telemetry sub-panes (`Session`,
    // `Layout`, `Workspace`). The top bar now owns cursor, and the rest
    // were dropped per D4 / D5; see `content_for_scene` below.
    let symbols = workspace_symbols(workspace);
    let diagnostics = workspace_diagnostics(workspace);
    let hover_lines = workspace_hover_lines(workspace);
    let references = workspace_references(workspace);
    let primary_symbol = workspace_primary_symbol(workspace);
    let hidden_buffer_note = if workspace.hidden_buffer_count() > 0 {
        String::from("Hidden buffers remain switchable without tabs")
    } else {
        String::from("The active project is currently mounted into a single visible buffer")
    };

    let launcher = LauncherContentState {
        recent_projects: recent_projects
            .iter()
            .enumerate()
            .map(|(index, path)| {
                let label = path
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .unwrap_or("Unknown Project");
                let marker = if index == launcher_selection {
                    "> "
                } else {
                    "  "
                };
                format!("{marker}{label} ({})", path.display())
            })
            .collect(),
        actions: vec!["Open Project", "Create Project", "Browse Recent"],
    };

    let palette = PaletteContentState {
        filter_hint: match scene {
            ShellScene::Empty => "Start from empty shell",
            ShellScene::Editing => "Editing shell commands",
            ShellScene::Semantic => "Semantic shell commands",
            ShellScene::BuildRun => "Build/run shell commands",
            ShellScene::Palette => "Palette overlay active",
            ShellScene::ComReference => "COM reference helper active",
        },
        commands: vec![
            PaletteCommandState {
                label: "Open Project",
                shortcut: "Ctrl+O",
            },
            // J4-e / P6: `New Project` with `Ctrl+N` used to live here
            // but `Ctrl+N` is not wired in `model.rs`. The palette only
            // advertises bindings that actually work; `New Project` will
            // return once it is implemented (see W040).
            PaletteCommandState {
                label: "Save",
                shortcut: "Ctrl+S",
            },
            PaletteCommandState {
                label: "Save All",
                shortcut: "Ctrl+Shift+S",
            },
            PaletteCommandState {
                label: "Undo",
                shortcut: "Ctrl+Z",
            },
            PaletteCommandState {
                label: "Redo",
                shortcut: "Ctrl+Y",
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
                label: "Add Module",
                shortcut: "Ctrl+Shift+M",
            },
            PaletteCommandState {
                label: "Add Class",
                shortcut: "Ctrl+Shift+C",
            },
            PaletteCommandState {
                label: "Add COM Reference",
                shortcut: "Ctrl+Shift+R",
            },
            PaletteCommandState {
                label: "Cycle Target",
                shortcut: "Ctrl+Shift+T",
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
        // `state_commands` is the dev-only "Mockup States" palette group
        // (scene-flip previews). Suppressed unless --dev-scenes is on; see
        // uxpass decision D6.
        state_commands: if dev_scenes {
            vec![
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
            ]
        } else {
            Vec::new()
        },
    };

    match scene {
        ShellScene::Empty => ShellContentState {
            launcher,
            editor_notes: vec![
                String::from("Open a project or create a new one to begin."),
                String::from("The shell remains keyboard-first from the first screen."),
            ],
            // Empty's Inspector keeps only the `Capabilities` sub-pane
            // (user-actionable under P8). The `Theme` sub-pane and the
            // `Tokens` hex-code dump used to live here; both were dev
            // telemetry (P1 / D4) and were removed together with
            // `theme::token_summary`.
            inspector: PanelContentState {
                sections: vec![PanelSectionState {
                    title: "Capabilities",
                    lines: vec![
                        String::from("Truecolor: yes"),
                        String::from("Unicode: yes"),
                        String::from("Mouse: optional"),
                    ],
                }],
            },
            lower_surface: PanelContentState {
                sections: Vec::new(),
            },
            palette,
        },
        ShellScene::Editing | ShellScene::Palette | ShellScene::ComReference => ShellContentState {
            launcher,
            editor_notes: vec![
                format!("Primary editor view mounted on {active_buffer_title}"),
                hidden_buffer_note,
                String::from("Ctrl+Tab rotates visible views when splits exist"),
            ],
            // `Diagnostics` + `Symbols` are the two user-facing Inspector
            // contracts on Editing: what is wrong with the buffer, and what
            // it contains. The old `Session` sub-pane reported internal
            // counters (dirty / visible / hidden buffer counts, active
            // cursor coordinates that the top bar already owns) — dev
            // telemetry under P1 and P3, dropped per D4 / D5.
            inspector: PanelContentState {
                sections: vec![
                    PanelSectionState {
                        title: "Diagnostics",
                        lines: diagnostics.clone(),
                    },
                    PanelSectionState {
                        title: "Symbols",
                        lines: symbols.clone(),
                    },
                ],
            },
            lower_surface: PanelContentState {
                sections: vec![PanelSectionState {
                    title: "Problems",
                    lines: diagnostics,
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
            // Semantic's Inspector keeps `Hover` + `Symbols` — the two
            // surfaces the user actually reads while navigating semantics.
            // The former `Layout` sub-pane (`Preset: SplitEdit` / `Visible
            // views: 2` / `Shared buffer: yes` / `Selection: none`) leaked
            // the internal `LayoutPreset` enum and reported
            // counts-of-counts. Removed per D4 / D5 / P1.
            inspector: PanelContentState {
                sections: vec![
                    PanelSectionState {
                        title: "Hover",
                        lines: hover_lines,
                    },
                    PanelSectionState {
                        title: "Symbols",
                        lines: symbols
                            .iter()
                            .enumerate()
                            .map(|(index, symbol)| {
                                if primary_symbol
                                    .as_ref()
                                    .is_some_and(|current| current.name == *symbol)
                                    || (primary_symbol.is_none() && index == 0)
                                {
                                    format!("> {symbol}")
                                } else {
                                    symbol.clone()
                                }
                            })
                            .collect(),
                    },
                ],
            },
            lower_surface: PanelContentState {
                sections: vec![
                    PanelSectionState {
                        title: "References",
                        lines: references,
                    },
                    PanelSectionState {
                        title: "Problems",
                        lines: workspace_diagnostics(workspace),
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
            // BuildRun's Inspector now carries just `Run Status` + `Target`
            // — the author's two questions are "is the build/runtime
            // healthy" and "what is about to run". The former `Workspace`
            // sub-pane echoed internal layout enum identifiers and buffer
            // counters (P1 / D4 / D5); it was removed. `Entry` and
            // `Active buffer` moved into the new slim `Target` pane so the
            // user still sees what the Run button is pointed at.
            inspector: PanelContentState {
                sections: vec![
                    PanelSectionState {
                        title: "Run Status",
                        lines: vec![
                            format!("Build: {}", execution.build_status),
                            format!("Runtime: {}", execution.runtime_status),
                            format!("Profile: {}", execution.profile),
                            format!(
                                "Last exit: {}",
                                execution
                                    .last_exit_code
                                    .map(|code| code.to_string())
                                    .unwrap_or_else(|| String::from("-"))
                            ),
                        ],
                    },
                    PanelSectionState {
                        title: "Target",
                        lines: vec![
                            format!("Entry: {}", execution.entry_point),
                            format!(
                                "Active buffer: {}",
                                workspace
                                    .active_buffer()
                                    .map(|buffer| buffer.title.as_str())
                                    .unwrap_or("None")
                            ),
                        ],
                    },
                ],
            },
            lower_surface: PanelContentState {
                sections: vec![
                    PanelSectionState {
                        title: "Output",
                        lines: execution.output_lines.clone(),
                    },
                    PanelSectionState {
                        title: "Build Log",
                        lines: execution.log_lines.clone(),
                    },
                ],
            },
            palette,
        },
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

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

    // J4-e / P6 — every shortcut advertised in the palette must
    // resolve to a wired `Msg` in `model.rs`. `Ctrl+N` (bound to a
    // hypothetical "New Project" action) has no handler today, so the
    // palette must not pretend it does. This test locks the absence
    // in so the entry cannot be reintroduced unwired.
    #[test]
    fn palette_commands_do_not_advertise_unwired_ctrl_n_binding() {
        let state = ShellState::default();
        let commands = &state.runtime.content.palette.commands;

        assert!(
            commands.iter().all(|command| command.shortcut != "Ctrl+N"),
            "palette must not advertise Ctrl+N until it is wired (J4-e): {:?}",
            commands
                .iter()
                .map(|c| (c.label, c.shortcut))
                .collect::<Vec<_>>()
        );
        assert!(
            commands.iter().all(|command| command.label != "New Project"),
            "palette must not advertise `New Project` until it is implemented (J4-e)"
        );
    }

    #[test]
    fn semantic_scene_marks_the_symbol_found_on_the_active_line() {
        let mut state = ShellState::default();
        state.apply_scene(ShellScene::Semantic);

        let symbol_lines = &state.runtime.content.inspector.sections[1].lines;
        assert!(symbol_lines.iter().any(|line| line == "> ComputeAnswer"));
    }

    #[test]
    fn diagnostics_detect_missing_option_explicit() {
        let mut state = ShellState::default();
        state.runtime.workspace.buffers[0].lines.remove(0);
        state.runtime.workspace.buffers[0].lines.remove(0);
        state.refresh_content();

        let diagnostics = &state.runtime.content.inspector.sections[0].lines;
        assert!(
            diagnostics
                .iter()
                .any(|line| line.contains("missing Option Explicit"))
        );
    }

    #[test]
    fn build_run_scene_uses_runtime_execution_state() {
        let mut state = ShellState::default();
        state.set_execution(ExecutionState {
            profile: String::from("win-console"),
            entry_point: String::from("Module1.Main"),
            build_status: String::from("passing"),
            runtime_status: String::from("prepared"),
            last_exit_code: Some(0),
            output_lines: vec![String::from("[run] entry Module1.Main")],
            log_lines: vec![String::from("module Module1.bas ready")],
        });
        state.apply_scene(ShellScene::BuildRun);

        assert_eq!(
            state.runtime.content.inspector.sections[0].lines[1],
            "Runtime: prepared"
        );
        assert!(
            state.runtime.content.lower_surface.sections[0]
                .lines
                .iter()
                .any(|line| line.contains("Module1.Main"))
        );
    }

    #[test]
    fn mounted_workspace_semantics_override_fallback_shell_content() {
        let mut state = ShellState::default();
        state.runtime.workspace.semantic = Some(WorkspaceSemanticState {
            diagnostics: vec![String::from("warning: Module1 implicit variant use")],
            symbols: vec![String::from("Main"), String::from("ComputeAnswer")],
            hover_lines: vec![String::from("Public Sub Main()")],
            references: vec![String::from("Module1:3 Public Sub Main()")],
        });
        state.refresh_content();

        assert_eq!(
            state.runtime.content.inspector.sections[0].lines[0],
            "warning: Module1 implicit variant use"
        );
        assert_eq!(state.runtime.content.inspector.sections[1].lines[0], "Main");
    }

    #[test]
    fn editor_insert_char_marks_buffer_dirty_and_updates_text() {
        let mut state = ShellState::default();
        state.runtime.focus = FocusRegion::Editor;
        state.insert_editor_char('X');

        let buffer = state.runtime.workspace.active_buffer().expect("buffer");
        assert!(buffer.dirty);
        assert!(buffer.lines[4].contains("X"));
    }

    // ---------------------------------------------------------------
    // Inspector slim-pass regression tests (uxpass D4 / D5 / P1).
    //
    // The default-build Inspector is a user surface: every sub-pane
    // must be actionable, explanatory, or diagnostic to the author.
    // Dev telemetry (buffer counters, view counts, internal layout
    // enum names, palette token names) MUST NOT appear. These tests
    // pin the approved set of sub-panes per scene and guard against
    // regressions that sneak dev strings back in.
    // ---------------------------------------------------------------

    fn inspector_section_titles(state: &ShellState) -> Vec<&'static str> {
        state
            .runtime
            .content
            .inspector
            .sections
            .iter()
            .map(|section| section.title)
            .collect()
    }

    fn assert_inspector_is_slim(state: &ShellState) {
        let joined = state
            .runtime
            .content
            .inspector
            .sections
            .iter()
            .flat_map(|section| {
                std::iter::once(section.title.to_string())
                    .chain(section.lines.iter().cloned())
            })
            .collect::<Vec<_>>()
            .join("\n");

        // Sub-pane titles that were dev-only and have been removed.
        for banned in ["Theme", "Tokens", "Session", "Layout", "Workspace"] {
            assert!(
                !joined.contains(banned),
                "inspector contains banned sub-pane {banned:?}:\n{joined}"
            );
        }
        // Free-text dev telemetry that should never surface either.
        for banned in [
            "Mockup-derived",
            "High-contrast panel",
            "Dirty buffers",
            "Visible views",
            "Hidden buffers",
            "Active cursor",
            "Preset: ",
            "Shared buffer",
            "Open buffers",
            "panel-alt",
        ] {
            assert!(
                !joined.contains(banned),
                "inspector contains banned telemetry {banned:?}:\n{joined}"
            );
        }
    }

    #[test]
    fn empty_inspector_shows_only_capabilities() {
        let state = ShellState {
            scene: ShellScene::Empty,
            ..ShellState::default()
        };
        let mut state = state;
        state.apply_scene(ShellScene::Empty);

        assert_eq!(inspector_section_titles(&state), vec!["Capabilities"]);
        assert_inspector_is_slim(&state);
    }

    #[test]
    fn editing_inspector_carries_diagnostics_and_symbols_only() {
        let state = ShellState::default();
        assert_eq!(state.scene, ShellScene::Editing);
        assert_eq!(
            inspector_section_titles(&state),
            vec!["Diagnostics", "Symbols"]
        );
        assert_inspector_is_slim(&state);
    }

    #[test]
    fn semantic_inspector_carries_hover_and_symbols_only() {
        let mut state = ShellState::default();
        state.apply_scene(ShellScene::Semantic);
        assert_eq!(
            inspector_section_titles(&state),
            vec!["Hover", "Symbols"]
        );
        assert_inspector_is_slim(&state);
    }

    #[test]
    fn build_run_inspector_carries_run_status_and_target_only() {
        let mut state = ShellState::default();
        state.apply_scene(ShellScene::BuildRun);
        assert_eq!(
            inspector_section_titles(&state),
            vec!["Run Status", "Target"]
        );
        assert_inspector_is_slim(&state);
    }

    #[test]
    fn palette_inspector_matches_editing_slim_contract() {
        let mut state = ShellState::default();
        state.apply_scene(ShellScene::Palette);
        assert_eq!(
            inspector_section_titles(&state),
            vec!["Diagnostics", "Symbols"]
        );
        assert_inspector_is_slim(&state);
    }

    // ---------------------------------------------------------------
    // Empty-scene frame regression tests (uxpass D1 — D1a + D1b landed;
    // uxpass D16 tightened further).
    //
    // D1a removed the Inspector column from Empty; D1b removed the
    // separate Launcher column and promoted Welcome to a single
    // full-width body panel that owns both the recent-projects
    // selection and the Start actions. D16 (`20_frame_and_regions.md`)
    // then removed the top bar from every focus ring because the band
    // is display-only — the user has no keystroke to bind against it.
    // So `available_focus_regions` now returns `[Editor]` on Empty:
    // `Tab` is a no-op (only one region), `Alt+1` (Explorer), `Alt+3`
    // (Inspector) and `Alt+4` (Lower) are rejected because those
    // regions do not exist on Empty. Non-Empty scenes are unaffected
    // by D1, but also lose `FocusRegion::TopBar` per D16.
    // ---------------------------------------------------------------

    #[test]
    fn empty_scene_focus_ring_is_editor_only() {
        let mut state = ShellState::default();
        state.apply_scene(ShellScene::Empty);

        assert_eq!(
            state.available_focus_regions(),
            vec![FocusRegion::Editor],
            "Empty has no Launcher, Inspector, or Lower surface; \
             D16 also removes the TopBar; focus ring collapses to Editor"
        );
    }

    #[test]
    fn empty_scene_alt3_focus_request_is_rejected() {
        let mut state = ShellState::default();
        state.apply_scene(ShellScene::Empty);
        let before = state.runtime.focus;

        state.focus_region(FocusRegion::Inspector);

        assert_eq!(
            state.runtime.focus, before,
            "Alt+3 / Inspector focus must be a no-op on Empty (D1a)"
        );
    }

    #[test]
    fn empty_scene_alt1_focus_request_is_rejected() {
        // D1b: Launcher column is gone; Alt+1 (Explorer) is a no-op.
        let mut state = ShellState::default();
        state.apply_scene(ShellScene::Empty);
        let before = state.runtime.focus;

        state.focus_region(FocusRegion::Explorer);

        assert_eq!(
            state.runtime.focus, before,
            "Alt+1 / Explorer focus must be a no-op on Empty (D1b)"
        );
    }

    #[test]
    fn editing_scene_still_exposes_inspector_in_focus_ring() {
        let state = ShellState::default();
        assert_eq!(state.scene, ShellScene::Editing);
        assert!(
            state
                .available_focus_regions()
                .contains(&FocusRegion::Inspector),
            "D1 guard must only suppress Inspector on Empty; Editing keeps it"
        );
    }

    // D16 — top bar is display-only on every scene. This tightens the
    // two tests above by pinning the converse: no matter the scene,
    // `FocusRegion::TopBar` must not enter the focus ring, and a
    // direct focus request targeting it must be a no-op.

    #[test]
    fn top_bar_is_not_in_focus_ring_on_any_non_overlay_scene() {
        for scene in [
            ShellScene::Empty,
            ShellScene::Editing,
            ShellScene::Semantic,
            ShellScene::BuildRun,
        ] {
            let mut state = ShellState::default();
            state.apply_scene(scene);
            let ring = state.available_focus_regions();
            assert!(
                !ring.contains(&FocusRegion::TopBar),
                "D16: TopBar must not appear in the focus ring on scene {scene:?}, got {ring:?}"
            );
        }
    }

    #[test]
    fn top_bar_focus_request_is_rejected_on_every_non_overlay_scene() {
        for scene in [
            ShellScene::Empty,
            ShellScene::Editing,
            ShellScene::Semantic,
            ShellScene::BuildRun,
        ] {
            let mut state = ShellState::default();
            state.apply_scene(scene);
            let before = state.runtime.focus;

            state.focus_region(FocusRegion::TopBar);

            assert_eq!(
                state.runtime.focus, before,
                "D16: focus_region(TopBar) must be a no-op on scene {scene:?}"
            );
        }
    }

    // ---------------------------------------------------------------
    // Status-line regression tests (uxpass D3 / D8).
    //
    // Every scene must surface a hint that announces the keystrokes
    // that work in that scene. Empty must surface `Ctrl+O` (D8 — the
    // canonical cold-start action). Overlay scenes (Palette / COM
    // reference helper) surface their own `Esc close` hint.
    // ---------------------------------------------------------------

    #[test]
    fn empty_status_line_announces_ctrl_o() {
        let mut state = ShellState::default();
        state.apply_scene(ShellScene::Empty);
        let hint = state.status_line_hint();
        assert!(
            hint.contains("Ctrl+O"),
            "Empty status line must advertise Ctrl+O (D8), got {hint:?}"
        );
    }

    #[test]
    fn editing_status_line_announces_f5_and_palette() {
        let state = ShellState::default();
        assert_eq!(state.scene, ShellScene::Editing);
        let hint = state.status_line_hint();
        assert!(hint.contains("F5"), "Editing hint must name F5, got {hint:?}");
        assert!(hint.contains("F6"), "Editing hint must name F6, got {hint:?}");
    }

    #[test]
    fn build_run_status_line_announces_rerun() {
        let mut state = ShellState::default();
        state.apply_scene(ShellScene::BuildRun);
        let hint = state.status_line_hint();
        assert!(
            hint.contains("F5"),
            "BuildRun hint must announce F5 rerun, got {hint:?}"
        );
    }

    #[test]
    fn palette_overlay_status_line_is_overlay_hint() {
        let mut state = ShellState::default();
        state.apply_scene(ShellScene::Palette);
        let hint = state.status_line_hint();
        assert!(
            hint.contains("Esc"),
            "Palette overlay hint must advertise Esc, got {hint:?}"
        );
    }

    #[test]
    fn com_reference_overlay_status_line_is_overlay_hint() {
        let mut state = ShellState::default();
        state.apply_scene(ShellScene::ComReference);
        let hint = state.status_line_hint();
        assert!(
            hint.contains("Tab switch mode"),
            "COM reference hint must mention Tab switch mode, got {hint:?}"
        );
    }

    #[test]
    fn status_line_hint_never_exceeds_one_line() {
        // The status line is a single terminal row (Fixed(1) in the
        // frame). A hint that contains a newline would either truncate
        // on render or clip to the first segment — in either case the
        // second half is lost (D7 — silent truncation is a defect).
        for scene in [
            ShellScene::Empty,
            ShellScene::Editing,
            ShellScene::Semantic,
            ShellScene::BuildRun,
            ShellScene::Palette,
            ShellScene::ComReference,
        ] {
            let mut state = ShellState::default();
            state.apply_scene(scene);
            let hint = state.status_line_hint();
            assert!(
                !hint.contains('\n'),
                "status line hint for {scene:?} must fit one row, got {hint:?}"
            );
        }
    }

    #[test]
    fn editing_status_line_announces_ctrl_s_save_and_ctrl_z_undo() {
        let state = ShellState::default();
        assert_eq!(state.scene, ShellScene::Editing);
        let hint = state.status_line_hint();
        assert!(
            hint.contains("Ctrl+S save"),
            "Editing hint must announce Ctrl+S (save path landing), got {hint:?}"
        );
        assert!(
            hint.contains("Ctrl+Z undo"),
            "Editing hint must announce Ctrl+Z (undo history landing), got {hint:?}"
        );
    }

    // ---------------------------------------------------------------
    // Save / Save All round-trip tests.
    //
    // The Welcome buffer never has a `source_path`, so save is always
    // a no-op there. Buffers loaded from a real project carry the
    // path through the DocumentSession → BufferState seam and can be
    // written back. These tests pin the round-trip through a scratch
    // directory (`target/test-workspaces/save-path/`) — matching the
    // existing convention in `model.rs` test fixtures.
    // ---------------------------------------------------------------

    fn seed_save_fixture(name: &str, initial_text: &str) -> PathBuf {
        let root = PathBuf::from("target")
            .join("test-workspaces")
            .join("save-path")
            .join(name);
        std::fs::create_dir_all(&root).expect("create save fixture dir");
        let path = root.join("Module1.bas");
        std::fs::write(&path, initial_text).expect("seed save fixture");
        path
    }

    fn buffer_with_source(path: &Path, text: &str) -> BufferState {
        BufferState {
            id: BufferId(1),
            title: String::from("Module1.bas"),
            kind: BufferKind::Source,
            dirty: false,
            lines: text.lines().map(String::from).collect(),
            source_path: Some(path.to_path_buf()),
            line_ending: LineEnding::detect(text),
            trailing_newline: text.ends_with('\n'),
            history: BufferHistory::new(),
        }
    }

    fn workspace_with_buffer(buffer: BufferState) -> WorkspaceState {
        WorkspaceState {
            project_name: Some(String::from("SaveFixture")),
            target_name: String::from("Exe"),
            project: None,
            recent_buffers: vec![buffer.id],
            views: vec![ViewState {
                id: ViewId(1),
                buffer_id: buffer.id,
                kind: ViewKind::Primary,
                surface: EditorSurfaceState {
                    cursor: CursorPosition::new(1, 1),
                    selection: None,
                    scroll_top: 0,
                },
            }],
            layout: WorkspaceLayoutState {
                preset: LayoutPreset::Edit,
                visible_views: vec![ViewId(1)],
                active_view: ViewId(1),
            },
            buffers: vec![buffer],
            semantic: None,
        }
    }

    #[test]
    fn save_active_buffer_writes_dirty_lines_to_disk_and_clears_dirty() {
        let path = seed_save_fixture(
            "save_active_buffer_writes_dirty_lines_to_disk_and_clears_dirty",
            "Option Explicit\r\n",
        );
        let mut workspace = workspace_with_buffer(buffer_with_source(&path, "Option Explicit\r\n"));
        // Simulate an edit: insert at (1,1) so the line now reads
        // "XOption Explicit".
        workspace.insert_char('X');
        assert!(
            workspace.buffers[0].dirty,
            "pre-check: edit must mark the buffer dirty"
        );

        let wrote = workspace
            .save_active_buffer()
            .expect("save must succeed on a writable path");
        assert!(wrote, "save must report it wrote the dirty buffer");
        assert!(
            !workspace.buffers[0].dirty,
            "save must clear the buffer's dirty flag"
        );

        let actual = std::fs::read_to_string(&path).expect("read back");
        assert_eq!(
            actual, "XOption Explicit\r\n",
            "round trip must preserve the detected CRLF line ending and trailing newline"
        );
    }

    #[test]
    fn save_active_buffer_is_a_noop_on_a_clean_buffer() {
        let path = seed_save_fixture(
            "save_active_buffer_is_a_noop_on_a_clean_buffer",
            "Option Explicit\n",
        );
        let mut workspace = workspace_with_buffer(buffer_with_source(&path, "Option Explicit\n"));
        let wrote = workspace.save_active_buffer().expect("save must not error");
        assert!(
            !wrote,
            "save must report no-op when buffer has no pending edits"
        );
    }

    #[test]
    fn save_active_buffer_is_a_noop_when_no_source_path() {
        // The Welcome buffer and any in-memory fixtures have
        // `source_path: None`; save is silently skipped rather than
        // materialising a file in an unexpected location (P4).
        let mut workspace = workspace_with_buffer(BufferState {
            id: BufferId(1),
            title: String::from("Welcome"),
            kind: BufferKind::Welcome,
            dirty: true, // even a dirty Welcome must not be saved
            lines: vec![String::from("OxIde")],
            source_path: None,
            line_ending: LineEnding::Lf,
            trailing_newline: false,
            history: BufferHistory::new(),
        });
        let wrote = workspace.save_active_buffer().expect("save must not error");
        assert!(
            !wrote,
            "save on a buffer without source_path must be a no-op"
        );
        assert!(
            workspace.buffers[0].dirty,
            "dirty flag must survive a no-op save so the user's edit is not silently marked persisted"
        );
    }

    #[test]
    fn save_preserves_lf_line_ending_when_original_was_lf() {
        let path = seed_save_fixture(
            "save_preserves_lf_line_ending_when_original_was_lf",
            "line one\nline two\n",
        );
        let mut workspace =
            workspace_with_buffer(buffer_with_source(&path, "line one\nline two\n"));
        workspace.insert_char('Z');
        workspace.save_active_buffer().unwrap();
        let actual = std::fs::read(&path).expect("read back as bytes");
        // Bytes must only contain 0x0A for line breaks — no 0x0D.
        assert!(
            !actual.contains(&b'\r'),
            "LF-only file must not gain a CR after round-trip: {:?}",
            String::from_utf8_lossy(&actual)
        );
    }

    #[test]
    fn save_preserves_no_trailing_newline_when_original_had_none() {
        let path = seed_save_fixture(
            "save_preserves_no_trailing_newline_when_original_had_none",
            "line one\nline two",
        );
        let mut workspace = workspace_with_buffer(buffer_with_source(&path, "line one\nline two"));
        workspace.insert_char('Z');
        workspace.save_active_buffer().unwrap();
        let actual = std::fs::read_to_string(&path).expect("read back");
        assert!(
            !actual.ends_with('\n'),
            "file without trailing newline must round-trip without one, got {actual:?}"
        );
    }

    #[test]
    fn save_all_dirty_buffers_persists_every_dirty_buffer() {
        let path_a = seed_save_fixture("save_all_dirty_buffers_a", "A\n");
        let path_b = seed_save_fixture("save_all_dirty_buffers_b", "B\n");

        let mut workspace = WorkspaceState {
            project_name: Some(String::from("SaveAll")),
            target_name: String::from("Exe"),
            project: None,
            recent_buffers: vec![BufferId(1), BufferId(2)],
            views: vec![ViewState {
                id: ViewId(1),
                buffer_id: BufferId(1),
                kind: ViewKind::Primary,
                surface: EditorSurfaceState {
                    cursor: CursorPosition::new(1, 1),
                    selection: None,
                    scroll_top: 0,
                },
            }],
            layout: WorkspaceLayoutState {
                preset: LayoutPreset::Edit,
                visible_views: vec![ViewId(1)],
                active_view: ViewId(1),
            },
            buffers: vec![
                BufferState {
                    dirty: true,
                    ..buffer_with_source(&path_a, "A\n")
                },
                BufferState {
                    id: BufferId(2),
                    dirty: true,
                    lines: vec![String::from("B edited")],
                    ..buffer_with_source(&path_b, "B\n")
                },
            ],
            semantic: None,
        };
        // Mutate the first buffer's lines too so the written content differs.
        workspace.buffers[0].lines = vec![String::from("A edited")];
        let errors = workspace.save_all_dirty_buffers();
        assert!(errors.is_empty(), "all saves should succeed: {errors:?}");
        assert!(
            workspace.buffers.iter().all(|buffer| !buffer.dirty),
            "save-all must clear the dirty flag on every persisted buffer"
        );
        assert_eq!(std::fs::read_to_string(&path_a).unwrap(), "A edited\n");
        assert_eq!(std::fs::read_to_string(&path_b).unwrap(), "B edited\n");
    }

    // ---------------------------------------------------------------
    // Undo / Redo history tests.
    //
    // The edit primitives (`insert_char`, `insert_newline`, `backspace`)
    // each snapshot the buffer's pre-edit state onto the undo stack.
    // `Undo` pops; `Redo` pushes back; a new edit invalidates redo.
    // ---------------------------------------------------------------

    #[test]
    fn undo_restores_previous_lines_and_cursor() {
        let path = seed_save_fixture("undo_restores_previous_lines_and_cursor", "hello\n");
        let mut workspace = workspace_with_buffer(buffer_with_source(&path, "hello\n"));

        workspace.insert_char('X');
        assert_eq!(workspace.buffers[0].lines, vec![String::from("Xhello")]);

        assert!(workspace.undo_active_buffer(), "undo must pop the snapshot");
        assert_eq!(
            workspace.buffers[0].lines,
            vec![String::from("hello")],
            "undo must restore the pre-edit lines"
        );
        assert_eq!(
            workspace.active_view().unwrap().surface.cursor,
            CursorPosition::new(1, 1),
            "undo must restore the pre-edit cursor"
        );
    }

    #[test]
    fn undo_on_empty_history_is_a_noop() {
        let path = seed_save_fixture("undo_on_empty_history_is_a_noop", "hello\n");
        let mut workspace = workspace_with_buffer(buffer_with_source(&path, "hello\n"));
        assert!(
            !workspace.undo_active_buffer(),
            "undo must report false when the stack is empty"
        );
    }

    #[test]
    fn redo_reapplies_undone_edit() {
        let path = seed_save_fixture("redo_reapplies_undone_edit", "hello\n");
        let mut workspace = workspace_with_buffer(buffer_with_source(&path, "hello\n"));
        workspace.insert_char('X');
        workspace.undo_active_buffer();
        assert_eq!(workspace.buffers[0].lines, vec![String::from("hello")]);

        assert!(workspace.redo_active_buffer(), "redo must have an entry");
        assert_eq!(
            workspace.buffers[0].lines,
            vec![String::from("Xhello")],
            "redo must re-apply the edit byte-for-byte"
        );
    }

    #[test]
    fn new_edit_after_undo_clears_redo_history() {
        let path = seed_save_fixture("new_edit_after_undo_clears_redo_history", "hello\n");
        let mut workspace = workspace_with_buffer(buffer_with_source(&path, "hello\n"));
        workspace.insert_char('X');
        workspace.undo_active_buffer();
        workspace.insert_char('Y');
        assert!(
            !workspace.redo_active_buffer(),
            "a new edit after undo must invalidate the redo history \
             (standard editor convention — redoing a stale future would produce inconsistent state)"
        );
    }

    #[test]
    fn buffer_history_capacity_bounds_memory() {
        let mut history = BufferHistory::with_capacity(3);
        for i in 0..10 {
            history.record(BufferSnapshot {
                lines: vec![format!("snapshot-{i}")],
                cursor: CursorPosition::new(1, 1),
            });
        }
        // After 10 records the oldest 7 have been dropped; the stack
        // still has exactly `capacity` entries.
        let dummy = BufferSnapshot {
            lines: vec![String::from("current")],
            cursor: CursorPosition::new(1, 1),
        };
        let popped = history.undo(dummy.clone()).unwrap();
        assert_eq!(popped.lines, vec![String::from("snapshot-9")]);
        let popped = history.undo(dummy.clone()).unwrap();
        assert_eq!(popped.lines, vec![String::from("snapshot-8")]);
        let popped = history.undo(dummy).unwrap();
        assert_eq!(popped.lines, vec![String::from("snapshot-7")]);
        assert!(
            !history.can_undo(),
            "capacity=3 means at most 3 reachable entries after 10 records"
        );
    }

    // ---------------------------------------------------------------
    // LineEnding detection.
    // ---------------------------------------------------------------

    #[test]
    fn line_ending_detect_picks_crlf_when_any_crlf_is_present() {
        assert_eq!(LineEnding::detect("a\r\nb\r\n"), LineEnding::CrLf);
        assert_eq!(LineEnding::detect("a\nb\n"), LineEnding::Lf);
        assert_eq!(LineEnding::detect("no newline"), LineEnding::Lf);
        // Mixed endings: CRLF wins because writing a mixed file back
        // with LF would silently normalise, which is a P4 violation.
        assert_eq!(LineEnding::detect("a\nb\r\nc\n"), LineEnding::CrLf);
    }

    // ---------------------------------------------------------------
    // Palette advertisement regression.
    //
    // J4-e pins that the palette only advertises bindings that are
    // wired. The Save push adds four new entries (Save, Save All,
    // Undo, Redo); all four must resolve to a wired `Msg` in
    // `model.rs`.
    // ---------------------------------------------------------------

    // Opening and closing an overlay must not wipe in-flight buffer
    // edits. Before the save-path push, `apply_scene` rebuilt the
    // workspace from the clean session snapshot on every transition,
    // which silently destroyed unsaved edits — the `*` dirty marker
    // (J2-d) implied the edit existed, but the edit itself was gone.
    // This test pins the non-destructive overlay behavior.

    #[test]
    fn opening_palette_overlay_preserves_in_flight_buffer_edits() {
        let mut state = ShellState::default();
        // Mount a workspace so the Editing scene has real buffers.
        let workspace =
            workspace_with_buffer(buffer_with_source(Path::new("dummy.bas"), "hello\n"));
        state.mount_workspace(workspace);
        state.apply_scene(ShellScene::Editing);

        state.runtime.workspace.insert_char('Z');
        let edited_lines = state.runtime.workspace.buffers[0].lines.clone();
        assert_eq!(edited_lines, vec![String::from("Zhello")]);
        assert!(state.runtime.workspace.buffers[0].dirty);

        state.apply_scene(ShellScene::Palette);
        assert_eq!(
            state.runtime.workspace.buffers[0].lines, edited_lines,
            "opening the palette must preserve in-flight edits"
        );
        assert!(
            state.runtime.workspace.buffers[0].dirty,
            "dirty flag must survive the overlay transition"
        );

        state.apply_scene(ShellScene::Editing);
        assert_eq!(
            state.runtime.workspace.buffers[0].lines, edited_lines,
            "returning to Editing after the overlay must not rebuild from the clean session snapshot"
        );
        assert!(
            state.runtime.workspace.buffers[0].dirty,
            "dirty flag must survive the full overlay round-trip"
        );
    }

    #[test]
    fn palette_advertises_save_all_undo_redo_with_wired_bindings() {
        let state = ShellState::default();
        let labels: Vec<&str> = state
            .runtime
            .content
            .palette
            .commands
            .iter()
            .map(|cmd| cmd.label)
            .collect();
        for expected in ["Save", "Save All", "Undo", "Redo"] {
            assert!(
                labels.contains(&expected),
                "palette must advertise {expected:?} (save-path landing), got {labels:?}"
            );
        }

        let shortcuts: Vec<(&str, &str)> = state
            .runtime
            .content
            .palette
            .commands
            .iter()
            .map(|cmd| (cmd.label, cmd.shortcut))
            .collect();
        assert!(shortcuts.contains(&("Save", "Ctrl+S")));
        assert!(shortcuts.contains(&("Save All", "Ctrl+Shift+S")));
        assert!(shortcuts.contains(&("Undo", "Ctrl+Z")));
        assert!(shortcuts.contains(&("Redo", "Ctrl+Y")));
    }
}
