//! Rendering-independent editor core for the OxIde GUI pivot.
//!
//! This crate must not import the parked TUI editor state. It owns the
//! small GUI-native editor data model that later UI crates can render.

use serde::{Deserialize, Serialize};

/// Immutable source snapshot used by deterministic lab/editor tests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceSnapshot {
    text: String,
}

impl SourceSnapshot {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn apply(&self, operation: &EditOperation) -> EditOutcome {
        match operation {
            EditOperation::Noop => EditOutcome {
                snapshot: self.clone(),
                applied: false,
            },
            EditOperation::CommentOutFirstLineContaining { needle } => {
                let mut applied = false;
                let mut lines = self.text.lines().map(String::from).collect::<Vec<_>>();
                for line in &mut lines {
                    if !applied && line.contains(needle) {
                        line.insert_str(0, "'");
                        applied = true;
                    }
                }
                let mut text = lines.join("\n");
                if self.text.ends_with('\n') {
                    text.push('\n');
                }
                EditOutcome {
                    snapshot: SourceSnapshot::new(text),
                    applied,
                }
            }
        }
    }
}

/// Minimal named edit operation for W220 deterministic scenarios.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EditOperation {
    Noop,
    CommentOutFirstLineContaining { needle: String },
}

impl EditOperation {
    pub fn comment_out_option_explicit() -> Self {
        Self::CommentOutFirstLineContaining {
            needle: String::from("Option Explicit"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditOutcome {
    pub snapshot: SourceSnapshot,
    pub applied: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    const THIN_SLICE_SOURCE: &str =
        "Attribute VB_Name = \"Module1\"\n\nOption Explicit\n\nPublic Sub Main()\nEnd Sub\n";

    #[test]
    fn source_snapshot_preserves_text() {
        let snapshot = SourceSnapshot::new(THIN_SLICE_SOURCE);

        assert_eq!(snapshot.text(), THIN_SLICE_SOURCE);
    }

    #[test]
    fn noop_edit_preserves_source_and_reports_not_applied() {
        let snapshot = SourceSnapshot::new(THIN_SLICE_SOURCE);

        let outcome = snapshot.apply(&EditOperation::Noop);

        assert!(!outcome.applied);
        assert_eq!(outcome.snapshot.text(), THIN_SLICE_SOURCE);
    }

    #[test]
    fn comment_out_option_explicit_is_deterministic() {
        let snapshot = SourceSnapshot::new(THIN_SLICE_SOURCE);

        let outcome = snapshot.apply(&EditOperation::comment_out_option_explicit());

        assert!(outcome.applied);
        assert!(outcome.snapshot.text().contains("'Option Explicit"));
        assert!(!outcome.snapshot.text().contains("\nOption Explicit\n"));
        assert!(outcome.snapshot.text().contains("Public Sub Main()"));
    }

    #[test]
    fn comment_out_missing_line_preserves_source_and_reports_not_applied() {
        let snapshot = SourceSnapshot::new(THIN_SLICE_SOURCE);

        let outcome = snapshot.apply(&EditOperation::CommentOutFirstLineContaining {
            needle: String::from("MissingToken"),
        });

        assert!(!outcome.applied);
        assert_eq!(outcome.snapshot.text(), THIN_SLICE_SOURCE);
    }
}
