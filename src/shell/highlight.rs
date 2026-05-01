//! VBA syntax highlighter used to render the Editor panel.
//!
//! Produces a stream of classified tokens per source line, plus a
//! helper that builds a styled `ftui::text::Text` (line-numbers gutter
//! + colorized source) suitable for direct rendering via `Paragraph`.
//!
//! This is a first-pass lexer — good enough to make source code
//! recognizable without needing `tree-sitter-vba` or a real parser
//! path. Later work (W060) can replace the token stream with a
//! semantic-aware highlighter driven by the OxVba language service;
//! the render side only depends on `build_editor_text` producing
//! styled `Text`, not on how the tokens were classified.
//!
//! Grounded in:
//! - VBA reference (case-insensitive keywords, `'` line comments,
//!   `"` strings with no escape sequences, `_` line continuation).
//! - What the thin-slice fixture (`examples/thin-slice/Module1.bas`)
//!   uses; regression tests pin the classification of its tokens.

use ftui::Style;
use ftui::text::{Line, Span, Text};

use super::theme;

/// Classification of a single lexeme in the source.
///
/// `Whitespace`, `Identifier`, and `Punctuation` render with the
/// default editor-panel foreground (no `Span` style override); the
/// other kinds paint the distinguishing colour from `theme`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    /// Reserved VBA control / declaration words (`Sub`, `If`, `Dim`, ...).
    Keyword,
    /// Built-in VBA scalar type names (`Integer`, `String`, ...).
    /// Tagged distinctly so the editor can match the convention some
    /// IDEs use of colouring type names differently from control words.
    TypeKeyword,
    Identifier,
    Number,
    /// Quoted string literal, including its enclosing `"`.
    String,
    /// `'` to end-of-line or an explicit `Rem` comment.
    Comment,
    /// Single non-identifier, non-whitespace punctuation char (`=`, `(`,
    /// `+`, etc.). Individual tokens so a future highlighter can colour
    /// operators separately without re-lexing.
    Punctuation,
    Whitespace,
}

impl TokenKind {
    /// Style override for this token kind. `None` means "paint with the
    /// panel's default foreground" — used for identifiers, punctuation,
    /// and whitespace so they pick up the active/inactive foreground
    /// rule that the panel is already configured with.
    pub fn style(self) -> Option<Style> {
        match self {
            Self::Keyword => Some(theme::style_keyword()),
            Self::TypeKeyword => Some(theme::style_type_keyword()),
            Self::Number => Some(theme::style_number()),
            Self::String => Some(theme::style_string()),
            Self::Comment => Some(theme::style_comment()),
            Self::Identifier | Self::Punctuation | Self::Whitespace => None,
        }
    }
}

/// A classified byte range inside a single source line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub text: &'a str,
}

/// Reserved VBA words we classify as `Keyword`. Case-insensitive match.
/// `Rem` is present but we also detect it specially below (the rest of
/// the line is a comment when a line begins with `Rem` as a full word).
const KEYWORDS: &[&str] = &[
    "Sub",
    "End",
    "Function",
    "Dim",
    "ReDim",
    "As",
    "Public",
    "Private",
    "Friend",
    "Global",
    "Option",
    "Explicit",
    "Compare",
    "Base",
    "Module",
    "If",
    "Then",
    "Else",
    "ElseIf",
    "EndIf",
    "For",
    "Next",
    "Do",
    "Loop",
    "While",
    "Wend",
    "Until",
    "Select",
    "Case",
    "With",
    "New",
    "Set",
    "Let",
    "Const",
    "Declare",
    "Type",
    "Property",
    "Get",
    "True",
    "False",
    "Nothing",
    "Null",
    "Empty",
    "On",
    "Error",
    "GoTo",
    "Resume",
    "Return",
    "Exit",
    "Each",
    "In",
    "To",
    "Step",
    "ByVal",
    "ByRef",
    "Optional",
    "ParamArray",
    "Attribute",
    "And",
    "Or",
    "Not",
    "Xor",
    "Eqv",
    "Imp",
    "Mod",
    "Like",
    "Is",
    "Me",
    "Implements",
    "Call",
    "Rem",
    "Stop",
];

/// Reserved VBA scalar type names.
const TYPE_KEYWORDS: &[&str] = &[
    "Integer", "Long", "LongLong", "LongPtr", "Single", "Double", "Currency", "Decimal", "String",
    "Boolean", "Byte", "Date", "Object", "Variant", "Any",
];

fn match_ci(word: &str, set: &[&str]) -> bool {
    set.iter().any(|entry| entry.eq_ignore_ascii_case(word))
}

/// Tokenize a single source line.
///
/// Does not handle multi-line constructs (line continuation with a
/// trailing ` _`, multi-line string literals — which VBA does not
/// have anyway). Comments run to end-of-line; string literals without
/// a closing `"` are accepted as running to end-of-line (matching most
/// editors' "in-progress" colouring).
pub fn tokenize(line: &str) -> Vec<Token<'_>> {
    let bytes = line.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'\'' {
            // `'` comment: everything to end-of-line.
            out.push(Token {
                kind: TokenKind::Comment,
                text: &line[i..],
            });
            i = bytes.len();
        } else if b == b'"' {
            // String literal. VBA escapes `""` for a literal quote;
            // advance over paired quotes as a single logical run.
            let start = i;
            i += 1;
            while i < bytes.len() {
                if bytes[i] == b'"' {
                    if i + 1 < bytes.len() && bytes[i + 1] == b'"' {
                        i += 2;
                        continue;
                    }
                    i += 1;
                    break;
                }
                i += 1;
            }
            out.push(Token {
                kind: TokenKind::String,
                text: &line[start..i],
            });
        } else if b.is_ascii_digit() {
            let start = i;
            while i < bytes.len()
                && (bytes[i].is_ascii_digit()
                    || bytes[i] == b'.'
                    || bytes[i] == b'E'
                    || bytes[i] == b'e')
            {
                i += 1;
            }
            out.push(Token {
                kind: TokenKind::Number,
                text: &line[start..i],
            });
        } else if b.is_ascii_alphabetic() || b == b'_' {
            let start = i;
            while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                i += 1;
            }
            let text = &line[start..i];
            // VBA's `Rem` keyword makes the rest of the line a comment.
            // Detect it here so the highlighter classifies the tail as
            // a comment rather than a keyword followed by identifiers.
            if text.eq_ignore_ascii_case("Rem") {
                out.push(Token {
                    kind: TokenKind::Keyword,
                    text,
                });
                if i < bytes.len() {
                    out.push(Token {
                        kind: TokenKind::Comment,
                        text: &line[i..],
                    });
                    i = bytes.len();
                }
            } else {
                let kind = if match_ci(text, KEYWORDS) {
                    TokenKind::Keyword
                } else if match_ci(text, TYPE_KEYWORDS) {
                    TokenKind::TypeKeyword
                } else {
                    TokenKind::Identifier
                };
                out.push(Token { kind, text });
            }
        } else if b == b' ' || b == b'\t' {
            let start = i;
            while i < bytes.len() && (bytes[i] == b' ' || bytes[i] == b'\t') {
                i += 1;
            }
            out.push(Token {
                kind: TokenKind::Whitespace,
                text: &line[start..i],
            });
        } else {
            // Punctuation — emit one char at a time. Keeps the token
            // boundaries predictable for tests; consolidating runs is
            // a rendering-only concern.
            let start = i;
            i += 1;
            out.push(Token {
                kind: TokenKind::Punctuation,
                text: &line[start..i],
            });
        }
    }
    out
}

/// Width of the line-numbers gutter (in terminal columns) for a buffer
/// with `total_lines` logical rows. Always >= 3 so single-line buffers
/// still carry a visible gutter (`"  1 "`).
pub fn gutter_width(total_lines: usize) -> usize {
    total_lines.max(1).to_string().len().max(3)
}

/// Number of columns the gutter occupies including its trailing
/// `" │ "` separator. Useful to the editor cursor-positioning path,
/// which must shift the visible cursor by this amount.
pub fn gutter_total_width(total_lines: usize) -> usize {
    // Format: `{number:>width$} │ ` — width + 1 space + 1 separator
    // char + 1 trailing space.
    gutter_width(total_lines) + 3
}

/// Build the styled editor `Text` for a buffer.
///
/// Each source line becomes one `Line` consisting of:
/// - a muted gutter span (`" 42 │ "`),
/// - a series of token spans styled by `TokenKind::style`.
///
/// `total_lines` lets callers size the gutter against the *whole*
/// buffer even when only a visible slice of `lines` is passed in —
/// relevant once virtual scrolling lands (today we pass the whole
/// buffer, so `total_lines == lines.len()`).
pub fn build_editor_text(lines: &[String], total_lines: usize) -> Text<'static> {
    let gutter = gutter_width(total_lines);
    let gutter_style = theme::style_gutter();
    let mut out_lines = Vec::with_capacity(lines.len());
    for (idx, source_line) in lines.iter().enumerate() {
        let gutter_text = format!("{:>width$} │ ", idx + 1, width = gutter);
        let mut spans: Vec<Span<'static>> = Vec::new();
        spans.push(Span::styled(gutter_text, gutter_style));
        for token in tokenize(source_line) {
            let owned = token.text.to_string();
            match token.kind.style() {
                Some(style) => spans.push(Span::styled(owned, style)),
                None => spans.push(Span::raw(owned)),
            }
        }
        out_lines.push(Line::from_spans(spans));
    }
    Text::from_lines(out_lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds(line: &str) -> Vec<TokenKind> {
        tokenize(line).into_iter().map(|t| t.kind).collect()
    }

    fn classify(line: &str, text: &str) -> TokenKind {
        tokenize(line)
            .into_iter()
            .find(|t| t.text == text)
            .unwrap_or_else(|| panic!("token {text:?} not found in {line:?}"))
            .kind
    }

    #[test]
    fn keywords_are_case_insensitive() {
        assert_eq!(classify("Sub Main()", "Sub"), TokenKind::Keyword);
        assert_eq!(classify("sub main()", "sub"), TokenKind::Keyword);
        assert_eq!(classify("SUB MAIN()", "SUB"), TokenKind::Keyword);
    }

    #[test]
    fn type_names_are_classified_separately_from_keywords() {
        let line = "    Dim answer As Integer";
        assert_eq!(classify(line, "Dim"), TokenKind::Keyword);
        assert_eq!(classify(line, "As"), TokenKind::Keyword);
        assert_eq!(classify(line, "Integer"), TokenKind::TypeKeyword);
        assert_eq!(classify(line, "answer"), TokenKind::Identifier);
    }

    #[test]
    fn numeric_literals_with_decimal_or_exponent() {
        assert_eq!(classify("answer = 42", "42"), TokenKind::Number);
        assert_eq!(classify("pi = 3.14", "3.14"), TokenKind::Number);
        assert_eq!(classify("k = 6.02E23", "6.02E23"), TokenKind::Number);
    }

    #[test]
    fn string_literals_run_to_closing_quote() {
        let line = "name = \"Hello\"";
        assert_eq!(classify(line, "\"Hello\""), TokenKind::String);
    }

    #[test]
    fn string_literals_handle_doubled_quote_escape() {
        let line = "msg = \"Say \"\"hi\"\" please\"";
        assert_eq!(
            classify(line, "\"Say \"\"hi\"\" please\""),
            TokenKind::String,
            "VBA escapes a literal quote as `\"\"`; it must not close the string"
        );
    }

    #[test]
    fn comment_runs_to_end_of_line() {
        let line = "x = 1  ' initial value";
        let tokens = tokenize(line);
        let last = tokens.last().unwrap();
        assert_eq!(last.kind, TokenKind::Comment);
        assert_eq!(last.text, "' initial value");
    }

    #[test]
    fn rem_keyword_turns_rest_of_line_into_a_comment() {
        let line = "Rem this is also a comment";
        let tokens = tokenize(line);
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::Keyword);
        assert_eq!(tokens[0].text, "Rem");
        assert_eq!(tokens[1].kind, TokenKind::Comment);
        assert_eq!(tokens[1].text, " this is also a comment");
    }

    #[test]
    fn identifiers_and_punctuation_stay_unstyled() {
        let line = "answer = 40 + 2";
        assert_eq!(classify(line, "answer"), TokenKind::Identifier);
        assert_eq!(classify(line, "="), TokenKind::Punctuation);
        assert_eq!(classify(line, "+"), TokenKind::Punctuation);
        // Neither identifier nor punctuation gets a style override.
        assert!(TokenKind::Identifier.style().is_none());
        assert!(TokenKind::Punctuation.style().is_none());
    }

    #[test]
    fn thin_slice_main_line_tokenizes_with_expected_kinds() {
        let line = "Public Sub Main()";
        assert_eq!(
            kinds(line),
            vec![
                TokenKind::Keyword, // Public
                TokenKind::Whitespace,
                TokenKind::Keyword, // Sub
                TokenKind::Whitespace,
                TokenKind::Identifier,  // Main
                TokenKind::Punctuation, // (
                TokenKind::Punctuation, // )
            ]
        );
    }

    #[test]
    fn gutter_width_adapts_to_total_line_count() {
        assert_eq!(
            gutter_width(1),
            3,
            "single-line buffer still gets a 3-col gutter"
        );
        assert_eq!(gutter_width(9), 3);
        assert_eq!(gutter_width(10), 3);
        assert_eq!(gutter_width(99), 3);
        assert_eq!(gutter_width(100), 3);
        assert_eq!(gutter_width(999), 3);
        assert_eq!(gutter_width(1000), 4);
        assert_eq!(gutter_width(10_000), 5);
    }

    #[test]
    fn gutter_total_width_accounts_for_separator() {
        assert_eq!(gutter_total_width(1), 6, "3 digits + \" │ \" (3 chars) = 6");
        assert_eq!(gutter_total_width(1000), 7);
    }

    #[test]
    fn build_editor_text_prefixes_each_line_with_right_aligned_number() {
        let lines = vec![
            String::from("Option Explicit"),
            String::from("Public Sub Main()"),
        ];
        let text = build_editor_text(&lines, lines.len());
        let rendered = text.to_plain_text();
        // Exactly the first gutter form: "  1 │ " then the source.
        // Right-aligned in a 3-col numeric field.
        assert!(
            rendered.starts_with("  1 │ Option Explicit"),
            "first line must carry gutter + source, got {rendered:?}"
        );
        assert!(
            rendered.contains("  2 │ Public Sub Main()"),
            "second line must carry its own gutter, got {rendered:?}"
        );
    }
}
