use std::collections::HashMap;
use std::path::Path;

use oxvba_languageservice::service::Position;
use oxvba_languageservice::{DiagnosticSeverity, DocumentId, HostWorkspaceSession};
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

    let diagnostics = documents
        .iter()
        .flat_map(|document| {
            session
                .diagnostics(&document.id)
                .unwrap_or_default()
                .into_iter()
                .map(|diagnostic| {
                    format!(
                        "{} {} {}",
                        document.id,
                        host_diagnostic_severity_label(diagnostic.severity),
                        diagnostic.message
                    )
                })
        })
        .collect::<Vec<_>>();

    let all_symbols = documents
        .iter()
        .flat_map(|document| session.document_symbols(&document.id).unwrap_or_default())
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
                lines.push(format!("[{}] {text}", output_stream_label(*stream)));
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
                lines.push(format!("output {} {text}", output_stream_label(*stream)));
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

    use super::{load_execution_state, load_semantic_state, run_project_state};
    use crate::shell::state::CursorPosition;

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
    }
}
