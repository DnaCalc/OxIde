use std::collections::HashMap;
use std::path::Path;

use oxvba_languageservice::service::Position;
use oxvba_languageservice::{
    DiagnosticSeverity, DocumentId, HostWorkspaceSession, SymbolProvenanceKind,
};
use oxvba_web_host::{WebDiagnosticSeverity, WebHostCommand, WebHostEvent, WebOutputStream};
use oxvba_web_shell::WebShellSession;

use super::state::{CursorPosition, ExecutionState, WorkspaceSemanticState};

pub fn load_execution_state(
    project_path: &Path,
    profile: String,
    entry_point: String,
) -> ExecutionState {
    execution_state_for_action(
        project_path,
        profile,
        entry_point,
        ExecutionAction::PrepareRuntime,
    )
}

pub fn run_project_state(
    project_path: &Path,
    profile: String,
    entry_point: String,
) -> ExecutionState {
    execution_state_for_action(
        project_path,
        profile,
        entry_point,
        ExecutionAction::RunProject,
    )
}

fn execution_state_for_action(
    project_path: &Path,
    profile: String,
    entry_point: String,
    action: ExecutionAction,
) -> ExecutionState {
    let mut services = OxVbaServices::default();
    match services.execute(project_path, action) {
        Ok(events) => execution_state_from_events(
            profile,
            entry_point,
            project_path,
            &events,
            String::from("passing"),
            String::from(action.runtime_status()),
            Some(0),
        ),
        Err(failure) => execution_state_from_failure(
            profile,
            entry_point,
            project_path,
            failure,
            String::from("failed"),
            String::from(action.failure_runtime_status()),
        ),
    }
}

pub fn load_semantic_state(
    project_path: &Path,
    active_buffer_title: Option<&str>,
    active_buffer_lines: &[String],
    active_cursor: CursorPosition,
) -> Option<WorkspaceSemanticState> {
    let session = HostWorkspaceSession::load_workspace_path(project_path).ok()?;
    let documents = session.documents();
    let active_document = resolve_active_document(&documents, active_buffer_title)?;
    let source_by_document = documents
        .iter()
        .filter_map(|document| {
            session
                .document_source(&document.id)
                .ok()
                .map(|source| (document.id.clone(), source))
        })
        .collect::<HashMap<_, _>>();

    // Uxpass J3-b (P4 / P10): diagnostics whose originating document is
    // a tool-inserted helper (OxVba startup shim, any `__OxVba…`
    // module) are not the author's code. Tagging them `[generated]`
    // keeps them visible (they still matter for a run that fails) but
    // makes the origin honest, so the user can tell at a glance that
    // the squiggles are not coming from their `Module1.bas`.
    let diagnostics = documents
        .iter()
        .flat_map(|document| {
            session
                .diagnostics(&document.id)
                .unwrap_or_default()
                .into_iter()
                .map(|diagnostic| {
                    format_diagnostic_line(
                        &document.id.0,
                        host_diagnostic_severity_label(diagnostic.severity),
                        &diagnostic.message,
                    )
                })
        })
        .collect::<Vec<_>>();

    // Uxpass J2-c / P1 — the Symbols pane is a user surface and must
    // list only symbols the user would recognize as theirs.
    // `document_symbols` returns every symbol the OxVba semantic model
    // knows about. Three classes leak in and we scrub each:
    //
    //   1. Imported type-library projections and project references —
    //      provenance kinds `ImportedTypeLibraryProjection` /
    //      `ProjectReference` / `Generated`. Keep only `SourceModule`.
    //   2. Tool-inserted helper documents (anything `__OxVba…`, e.g.
    //      `__OxVbaStartupEntryShim`) — those carry a second `Main`
    //      that mirrors the entry-point reference, not a user
    //      declaration. Filter by `provenance.document_id`.
    //   3. Type names appearing inside `As <Type>` annotations. OxVba's
    //      semantic model currently tags the type-name token as a
    //      `Variable` declared in the enclosing Sub with the same
    //      provenance as the Sub. That is an OxVba defect (it confuses
    //      the type-reference span with a declaration site); until it
    //      is fixed upstream we screen on the set of VBA intrinsic
    //      type names. A real user variable named `Integer` would be
    //      a syntax error in VBA, so this is safe.
    let all_symbols = documents
        .iter()
        .flat_map(|document| session.document_symbols(&document.id).unwrap_or_default())
        .filter(|symbol| symbol.provenance.kind == SymbolProvenanceKind::SourceModule)
        .filter(|symbol| !is_generated_document_id(&symbol.provenance.document_id))
        .filter(|symbol| !is_vba_intrinsic_type_name(&symbol.name))
        .map(|symbol| symbol.name)
        .collect::<Vec<_>>();

    let document_symbols = session
        .document_symbols(&active_document)
        .unwrap_or_default();
    let cursor_position = cursor_offset(active_buffer_lines, active_cursor);
    let semantic_position = session
        .hover(&active_document, cursor_position)
        .ok()
        .flatten()
        .map(|_| cursor_position)
        .or_else(|| document_symbols.first().map(|symbol| symbol.span.start));

    let hover_lines = semantic_position
        .and_then(|position| {
            session
                .hover(&active_document, position)
                .ok()
                .flatten()
                .map(|hover| hover_lines_from_info(&hover))
        })
        .unwrap_or_else(|| vec![String::from("No semantic target at the current cursor")]);

    let references = semantic_position
        .map(|position| {
            let references = session
                .find_references(&active_document, position)
                .unwrap_or_default();
            if references.is_empty() {
                vec![String::from("No references available")]
            } else {
                references
                    .into_iter()
                    .map(|location| {
                        format_reference(
                            &location.document,
                            location.span.start,
                            &source_by_document,
                        )
                    })
                    .collect()
            }
        })
        .unwrap_or_else(|| vec![String::from("No references available")]);

    Some(WorkspaceSemanticState {
        diagnostics: if diagnostics.is_empty() {
            vec![String::from("No diagnostics in mounted workspace")]
        } else {
            diagnostics
        },
        symbols: if all_symbols.is_empty() {
            vec![String::from("No symbols discovered")]
        } else {
            all_symbols
        },
        hover_lines,
        references,
    })
}

#[derive(Default)]
struct OxVbaServices {
    shell: WebShellSession,
}

#[derive(Debug, Clone, Copy)]
enum ExecutionAction {
    PrepareRuntime,
    RunProject,
}

impl ExecutionAction {
    fn runtime_status(self) -> &'static str {
        match self {
            Self::PrepareRuntime => "prepared",
            Self::RunProject => "completed",
        }
    }

    fn failure_runtime_status(self) -> &'static str {
        match self {
            Self::PrepareRuntime => "blocked",
            Self::RunProject => "failed",
        }
    }
}

impl OxVbaServices {
    fn execute(
        &mut self,
        project_path: &Path,
        action: ExecutionAction,
    ) -> Result<Vec<WebHostEvent>, OxVbaFailure> {
        let mut events = self.handle(
            WebHostCommand::LoadWorkspace {
                path: project_path.display().to_string(),
            },
            "load workspace",
            Vec::new(),
        )?;
        let action_events = match action {
            ExecutionAction::PrepareRuntime => self.handle(
                WebHostCommand::ResetRuntime,
                "prepare runtime",
                events.clone(),
            )?,
            ExecutionAction::RunProject => {
                self.handle(WebHostCommand::RunProject, "run project", events.clone())?
            }
        };
        events.extend(action_events);
        Ok(events)
    }

    fn handle(
        &mut self,
        command: WebHostCommand,
        operation: &'static str,
        prior_events: Vec<WebHostEvent>,
    ) -> Result<Vec<WebHostEvent>, OxVbaFailure> {
        self.shell
            .handle_command(command)
            .map_err(|err| OxVbaFailure {
                operation,
                message: err.to_string(),
                events: prior_events,
            })
    }
}

#[derive(Debug, Clone)]
struct OxVbaFailure {
    operation: &'static str,
    message: String,
    events: Vec<WebHostEvent>,
}

fn execution_state_from_events(
    profile: String,
    entry_point: String,
    project_path: &Path,
    events: &[WebHostEvent],
    build_status: String,
    runtime_status: String,
    last_exit_code: Option<i32>,
) -> ExecutionState {
    let mut output_lines = output_lines_from_events(events);
    if output_lines.is_empty() {
        output_lines.push(format!(
            "[stdout] runtime prepared for {}",
            project_path.display()
        ));
    }

    ExecutionState {
        profile,
        entry_point,
        build_status,
        runtime_status,
        last_exit_code,
        output_lines,
        log_lines: log_lines_from_events(events),
    }
}

fn execution_state_from_failure(
    profile: String,
    entry_point: String,
    project_path: &Path,
    failure: OxVbaFailure,
    build_status: String,
    runtime_status: String,
) -> ExecutionState {
    let mut output_lines = output_lines_from_events(&failure.events);
    output_lines.push(format!(
        "[error] {}: {}",
        failure.operation, failure.message
    ));

    let mut log_lines = log_lines_from_events(&failure.events);
    log_lines.push(format!(
        "[workspace] prepare failed for {}",
        project_path.display()
    ));
    log_lines.push(format!(
        "[error] {}: {}",
        failure.operation, failure.message
    ));

    ExecutionState {
        profile,
        entry_point,
        build_status,
        runtime_status,
        last_exit_code: Some(1),
        output_lines,
        log_lines,
    }
}

fn output_lines_from_events(events: &[WebHostEvent]) -> Vec<String> {
    let mut lines = Vec::new();

    for event in events {
        match event {
            WebHostEvent::WorkspaceLoaded(summary) => {
                lines.push(format!("[workspace] {}", summary.workspace_target));
                lines.push(format!("[workspace] documents {}", summary.documents.len()));
            }
            WebHostEvent::OutputLine { stream, text } => {
                lines.push(format!(
                    "[{}] {}",
                    output_stream_label(*stream),
                    sanitize_output_text(text)
                ));
            }
            WebHostEvent::Error { operation, message } => {
                lines.push(format!("[error] {operation}: {message}"));
            }
            WebHostEvent::DiagnosticsUpdated {
                document_id,
                diagnostics,
            } => {
                lines.extend(diagnostics.iter().map(|diagnostic| {
                    format!(
                        "[diagnostic] {} {} {}",
                        document_id,
                        diagnostic_severity_label(diagnostic.severity),
                        diagnostic.message
                    )
                }));
            }
            WebHostEvent::ImmediateResult(_)
            | WebHostEvent::DebugPaused(_)
            | WebHostEvent::RunStateChanged(_) => {}
        }
    }

    lines
}

fn log_lines_from_events(events: &[WebHostEvent]) -> Vec<String> {
    let mut lines = Vec::new();

    for event in events {
        match event {
            WebHostEvent::WorkspaceLoaded(summary) => {
                lines.push(format!("workspace loaded {}", summary.workspace_target));
                lines.extend(summary.documents.iter().map(|document| {
                    let project_name = document.project_name.as_deref().unwrap_or("workspace");
                    format!(
                        "document {} [{}] {}",
                        document.document_id,
                        project_name,
                        format!("{:?}", document.provenance_kind).to_lowercase()
                    )
                }));
            }
            WebHostEvent::DiagnosticsUpdated {
                document_id,
                diagnostics,
            } => {
                if diagnostics.is_empty() {
                    lines.push(format!("diagnostics {document_id} clean"));
                } else {
                    lines.extend(diagnostics.iter().map(|diagnostic| {
                        format!(
                            "diagnostics {} {} {} ({}..{})",
                            document_id,
                            diagnostic_severity_label(diagnostic.severity),
                            diagnostic.message,
                            diagnostic.start,
                            diagnostic.end
                        )
                    }));
                }
            }
            WebHostEvent::OutputLine { stream, text } => {
                lines.push(format!(
                    "output {} {}",
                    output_stream_label(*stream),
                    sanitize_output_text(text)
                ));
            }
            WebHostEvent::RunStateChanged(state) => {
                lines.push(format!("run-state {:?}", state).to_lowercase());
            }
            WebHostEvent::Error { operation, message } => {
                lines.push(format!("error {operation}: {message}"));
            }
            WebHostEvent::ImmediateResult(_) | WebHostEvent::DebugPaused(_) => {}
        }
    }

    if lines.is_empty() {
        lines.push(String::from("no oxvba host events recorded"));
    }

    lines
}

fn output_stream_label(stream: WebOutputStream) -> &'static str {
    match stream {
        WebOutputStream::Stdout => "stdout",
        WebOutputStream::Stderr => "stderr",
    }
}

fn diagnostic_severity_label(severity: WebDiagnosticSeverity) -> &'static str {
    match severity {
        WebDiagnosticSeverity::Error => "error",
        WebDiagnosticSeverity::Warning => "warning",
    }
}

fn host_diagnostic_severity_label(severity: DiagnosticSeverity) -> &'static str {
    match severity {
        DiagnosticSeverity::Error => "error",
        DiagnosticSeverity::Warning => "warning",
    }
}

/// Format a single diagnostic row as it appears in the Problems pane
/// and the Inspector's Diagnostics sub-pane.
///
/// Uxpass J3-b: when the origin document is a tool-inserted helper
/// (any `__OxVba…` module such as `__OxVbaStartupEntryShim`), the row
/// is prefixed with `[generated]` instead of the raw document id so
/// the user can tell the diagnostic is not about their own code.
/// Otherwise the row is the document id followed by severity and
/// message, unchanged from the pre-J3-b layout.
fn format_diagnostic_line(document_id: &str, severity: &str, message: &str) -> String {
    if is_generated_document_id(document_id) {
        format!("[generated] {severity} {message}")
    } else {
        format!("{document_id} {severity} {message}")
    }
}

/// Tool-inserted helper documents carry a leading double-underscore
/// (e.g. `__OxVbaStartupEntryShim`). Any such id is "generated" for
/// uxpass labeling purposes.
fn is_generated_document_id(document_id: &str) -> bool {
    document_id.starts_with("__OxVba")
}

/// True if `name` is a VBA intrinsic type name. Used defensively to
/// screen out the OxVba-language-service defect where the type-token
/// in `Dim x As Integer` is reported as a `Variable` declaration
/// inside the enclosing Sub (see J2-c in [docs/uxpass/10_user_journeys.md]).
/// A real user variable with any of these names would be a VBA syntax
/// error, so filtering by name does not risk hiding a legitimate
/// author-declared symbol.
fn is_vba_intrinsic_type_name(name: &str) -> bool {
    matches!(
        name,
        "Boolean"
            | "Byte"
            | "Currency"
            | "Date"
            | "Decimal"
            | "Double"
            | "Integer"
            | "Long"
            | "LongLong"
            | "LongPtr"
            | "Object"
            | "Single"
            | "String"
            | "Variant"
    )
}

/// Scrub OxVba-internal jargon from a user-facing output line
/// (uxpass J3-c / P1).
///
/// The OxVba web shell emits `project run completed with N user slots`
/// as a raw stdout line (see `oxvba-web-shell`); "user slots" is the
/// runtime's slot-allocator vocabulary, not an end-user concept. We
/// trim the trailing `" with <digits> user slots"` so the Output
/// surface reads `project run completed` — the count is not
/// actionable. Only the specific `user slots` suffix is stripped;
/// other occurrences of `with` in a stdout line are left untouched.
fn sanitize_output_text(text: &str) -> String {
    const SUFFIX: &str = " user slots";
    if let Some(rest) = text.strip_suffix(SUFFIX) {
        if let Some(with_idx) = rest.rfind(" with ") {
            let digits = &rest[with_idx + " with ".len()..];
            if !digits.is_empty() && digits.chars().all(|c| c.is_ascii_digit()) {
                return rest[..with_idx].to_string();
            }
        }
    }
    text.to_string()
}

fn resolve_active_document(
    documents: &[oxvba_languageservice::HostWorkspaceDocument],
    active_buffer_title: Option<&str>,
) -> Option<DocumentId> {
    let preferred = active_buffer_title
        .and_then(document_id_from_buffer_title)
        .filter(|document_id| documents.iter().any(|document| document.id == *document_id));

    preferred.or_else(|| documents.first().map(|document| document.id.clone()))
}

fn document_id_from_buffer_title(title: &str) -> Option<DocumentId> {
    let title = title.trim();
    if title.is_empty() {
        return None;
    }

    Some(DocumentId::new(
        title
            .rsplit_once('.')
            .map(|(stem, _)| stem)
            .unwrap_or(title),
    ))
}

/// Navigation target produced by `fetch_goto_definition`.
///
/// `target_title` matches the `BufferState::title` the model uses to
/// locate the destination buffer (e.g. `Module1.bas`). `(target_line,
/// target_column)` are 1-based, matching `CursorPosition`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GotoDefinitionTarget {
    pub target_title: String,
    pub target_line: u16,
    pub target_column: u16,
}

/// Fetch hover information at the given cursor position for the
/// active buffer. Returns the rendered lines ready for a popover, or
/// `None` if OxVba reports nothing at that position (or the workspace
/// cannot be loaded).
pub fn fetch_hover_at_cursor(
    project_path: &Path,
    active_buffer_title: &str,
    active_buffer_lines: &[String],
    cursor: CursorPosition,
) -> Option<Vec<String>> {
    let session = HostWorkspaceSession::load_workspace_path(project_path).ok()?;
    let documents = session.documents();
    let active_document = resolve_active_document(&documents, Some(active_buffer_title))?;
    let position = cursor_offset(active_buffer_lines, cursor);
    let hover = session.hover(&active_document, position).ok().flatten()?;
    Some(hover_lines_from_info(&hover))
}

/// Resolve a goto-definition request at the given cursor position.
/// Returns `None` if OxVba has no definition to point to.
///
/// The returned `target_title` is the leaf filename of the resolved
/// document's id, matching the convention `ProjectSession::workspace_state`
/// uses for `BufferState::title`. Callers can look the buffer up by
/// title (same-document navigation stays in-buffer; cross-document
/// navigation switches the active view).
pub fn fetch_goto_definition(
    project_path: &Path,
    active_buffer_title: &str,
    active_buffer_lines: &[String],
    cursor: CursorPosition,
) -> Option<GotoDefinitionTarget> {
    let session = HostWorkspaceSession::load_workspace_path(project_path).ok()?;
    let documents = session.documents();
    let active_document = resolve_active_document(&documents, Some(active_buffer_title))?;
    let position = cursor_offset(active_buffer_lines, cursor);
    let location = session
        .go_to_definition(&active_document, position)
        .ok()
        .flatten()?;
    let target_source = session.document_source(&location.document).ok()?;
    let (target_line, target_column) =
        line_col_for_offset(&target_source, location.span.start);
    Some(GotoDefinitionTarget {
        target_title: document_leaf_title(&location.document.0),
        target_line,
        target_column,
    })
}

/// Convert a byte offset in a source string to a 1-based (line, column)
/// pair. Inverse of `cursor_offset` for this file's purposes; column
/// counts Unicode scalars (chars), matching the cursor model.
fn line_col_for_offset(source: &str, offset: Position) -> (u16, u16) {
    let target = offset as usize;
    let mut line_number = 1u16;
    let mut line_start = 0usize;

    for (index, byte) in source.bytes().enumerate() {
        if index >= target {
            break;
        }
        if byte == b'\n' {
            line_number = line_number.saturating_add(1);
            line_start = index + 1;
        }
    }

    let slice_end = target.min(source.len());
    let column_chars = source
        .get(line_start..slice_end)
        .map(|slice| slice.chars().count())
        .unwrap_or(0) as u16;
    (line_number, column_chars.saturating_add(1))
}

/// Map a `DocumentId` back to the buffer title `ProjectSession`
/// produced. Document ids carry the source path; the title is the
/// file's leaf name (e.g. `Module1.bas`).
fn document_leaf_title(document_id: &str) -> String {
    document_id
        .rsplit(|c| c == '/' || c == '\\')
        .next()
        .map(String::from)
        .unwrap_or_else(|| document_id.to_string())
}

fn cursor_offset(lines: &[String], cursor: CursorPosition) -> Position {
    let target_line = usize::from(cursor.line.saturating_sub(1));
    let mut offset = 0usize;

    for (index, line) in lines.iter().enumerate() {
        if index == target_line {
            let column = usize::from(cursor.column.saturating_sub(1));
            let line_offset = line.chars().take(column).map(char::len_utf8).sum::<usize>();
            return (offset + line_offset.min(line.len())) as Position;
        }
        offset += line.len() + 1;
    }

    offset as Position
}

fn hover_lines_from_info(hover: &oxvba_languageservice::HoverInfo) -> Vec<String> {
    let mut lines = vec![hover.label.clone()];
    if let Some(detail) = &hover.detail {
        lines.push(detail.clone());
    }
    if let Some(provenance) = &hover.provenance {
        lines.push(format!("Defined in {}", provenance.document_id));
    }
    lines
}

fn format_reference(
    document_id: &DocumentId,
    offset: Position,
    source_by_document: &HashMap<DocumentId, String>,
) -> String {
    if let Some(source) = source_by_document.get(document_id) {
        let (line_number, source_line) = source_line_for_offset(source, offset);
        return format!("{}:{} {}", document_id, line_number, source_line.trim());
    }

    format!("{}:{}", document_id, offset)
}

fn source_line_for_offset(source: &str, offset: Position) -> (usize, String) {
    let target = offset as usize;
    let mut line_number = 1usize;
    let mut line_start = 0usize;

    for (index, byte) in source.bytes().enumerate() {
        if index >= target {
            break;
        }
        if byte == b'\n' {
            line_number += 1;
            line_start = index + 1;
        }
    }

    let line_end = source[line_start..]
        .find('\n')
        .map(|relative| line_start + relative)
        .unwrap_or(source.len());

    (line_number, source[line_start..line_end].to_string())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{
        format_diagnostic_line, is_generated_document_id, is_vba_intrinsic_type_name,
        load_execution_state, load_semantic_state, run_project_state, sanitize_output_text,
    };
    use crate::shell::state::CursorPosition;

    // J3-c / P1 — "user slots" is internal allocator jargon emitted by
    // `oxvba-web-shell` (`project run completed with {N} user slots`).
    // `sanitize_output_text` strips the specific " with <digits> user
    // slots" suffix and leaves everything else untouched.
    #[test]
    fn sanitize_output_text_strips_user_slots_suffix_from_run_completion() {
        assert_eq!(
            sanitize_output_text("project run completed with 1 user slots"),
            "project run completed"
        );
        assert_eq!(
            sanitize_output_text("project run completed with 42 user slots"),
            "project run completed"
        );
    }

    #[test]
    fn sanitize_output_text_preserves_unrelated_with_clauses() {
        // "with" appearing in a non-user-slots context must not be
        // truncated — only the specific jargon tail is scrubbed.
        assert_eq!(
            sanitize_output_text("opened file with unicode bom"),
            "opened file with unicode bom"
        );
        assert_eq!(
            sanitize_output_text("project run completed"),
            "project run completed"
        );
        assert_eq!(
            sanitize_output_text("user slots reserved"),
            "user slots reserved"
        );
    }

    // J3-b / P1 — tool-inserted helpers (any `__OxVba…` id) are not
    // user code. Their diagnostic rows carry a `[generated]` prefix so
    // the user can see the message is not about their own module.
    #[test]
    fn is_generated_document_id_matches_oxvba_tool_helpers() {
        assert!(is_generated_document_id("__OxVbaStartupEntryShim"));
        assert!(is_generated_document_id("__OxVbaAnything"));
        assert!(!is_generated_document_id("Module1.bas"));
        assert!(!is_generated_document_id("_private_but_not_oxvba"));
        assert!(!is_generated_document_id(""));
    }

    #[test]
    fn vba_intrinsic_type_names_cover_the_builtin_scalar_types() {
        for name in [
            "Boolean", "Byte", "Currency", "Date", "Decimal", "Double", "Integer", "Long",
            "LongLong", "LongPtr", "Object", "Single", "String", "Variant",
        ] {
            assert!(
                is_vba_intrinsic_type_name(name),
                "{name} must be treated as a VBA intrinsic type name"
            );
        }
        // User-author identifiers must still pass through; the filter
        // is strictly an intrinsic-keyword denylist.
        for name in ["answer", "Main", "MyModule", "string_buffer", "intResult"] {
            assert!(
                !is_vba_intrinsic_type_name(name),
                "{name} is a user identifier and must not be filtered"
            );
        }
    }

    #[test]
    fn format_diagnostic_line_tags_generated_documents_and_preserves_user_ids() {
        assert_eq!(
            format_diagnostic_line("__OxVbaStartupEntryShim", "error", "bad thing"),
            "[generated] error bad thing"
        );
        assert_eq!(
            format_diagnostic_line("Module1.bas", "warning", "unused binding"),
            "Module1.bas warning unused binding"
        );
    }

    #[test]
    fn loads_execution_state_from_real_oxvba_runtime_contract() {
        let execution = load_execution_state(
            Path::new("examples/thin-slice/ThinSliceHello.basproj"),
            String::from("win-console"),
            String::from("Module1.Main"),
        );

        assert_eq!(execution.build_status, "passing");
        assert_eq!(execution.runtime_status, "prepared");
        assert_eq!(execution.last_exit_code, Some(0));
        assert!(
            execution
                .output_lines
                .iter()
                .any(|line| line.contains("runtime reset to a fresh prepared session"))
        );
        assert!(
            execution
                .log_lines
                .iter()
                .any(|line| line.contains("workspace loaded"))
        );
    }

    #[test]
    fn failures_project_into_blocked_execution_state() {
        let execution = load_execution_state(
            Path::new("examples/does-not-exist/Missing.basproj"),
            String::from("win-console"),
            String::from("Missing.Main"),
        );

        assert_eq!(execution.build_status, "failed");
        assert_eq!(execution.runtime_status, "blocked");
        assert_eq!(execution.last_exit_code, Some(1));
        assert!(
            execution
                .output_lines
                .iter()
                .any(|line| line.contains("[error] load workspace"))
        );
    }

    #[test]
    fn runs_project_through_real_oxvba_runtime_contract() {
        let execution = run_project_state(
            Path::new("examples/thin-slice/ThinSliceHello.basproj"),
            String::from("win-console"),
            String::from("Module1.Main"),
        );

        assert_eq!(execution.build_status, "passing");
        assert_eq!(execution.runtime_status, "completed");
        assert!(
            execution
                .output_lines
                .iter()
                .any(|line| line.contains("project run completed"))
        );
        assert!(
            execution
                .log_lines
                .iter()
                .any(|line| line.contains("run-state completed"))
        );
        // J3-c / P1 — the OxVba runtime phrase "user slots" must not
        // reach any user-facing surface. Sanitizer runs over both the
        // Output pane and the build/run log.
        assert!(
            execution
                .output_lines
                .iter()
                .all(|line| !line.contains("user slots")),
            "output_lines leaked 'user slots': {:?}",
            execution.output_lines
        );
        assert!(
            execution
                .log_lines
                .iter()
                .all(|line| !line.contains("user slots")),
            "log_lines leaked 'user slots': {:?}",
            execution.log_lines
        );
    }

    #[test]
    fn loads_semantic_state_from_real_oxvba_workspace_session() {
        let semantic = load_semantic_state(
            Path::new("examples/thin-slice/ThinSliceHello.basproj"),
            Some("Module1.bas"),
            &[
                String::from("Attribute VB_Name = \"Module1\""),
                String::from(""),
                String::from("Option Explicit"),
                String::from(""),
                String::from("Public Sub Main()"),
                String::from("    Dim answer As Integer"),
                String::from("    answer = 40 + 2"),
                String::from("End Sub"),
            ],
            CursorPosition::new(1, 1),
        )
        .expect("semantic state");

        assert!(semantic.symbols.iter().any(|symbol| symbol == "Main"));
        assert!(!semantic.hover_lines.is_empty());
        assert!(!semantic.references.is_empty());

        // J2-c / P1 — only `SourceModule` symbols belong in the user-
        // visible Symbols pane. Intrinsic scalar projections (e.g.
        // `Integer`, `Long`, `Variant`) and imported type-library
        // declarations are internal taxonomy and must be filtered out
        // in `all_symbols`.
        for intrinsic in ["Integer", "Long", "Variant", "Double", "Boolean", "String"] {
            assert!(
                !semantic.symbols.iter().any(|symbol| symbol == intrinsic),
                "intrinsic type `{intrinsic}` leaked into Symbols pane: {:?}",
                semantic.symbols
            );
        }
    }
}
