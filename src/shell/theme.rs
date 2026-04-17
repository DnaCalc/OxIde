use ftui::{PackedRgba, Style};

// The RGB values below are the authoritative palette. Previously they were
// mirrored as `pub const *_HEX: &str` strings and rendered into the Inspector
// as a `Tokens` sub-pane — dev telemetry on a user surface (P1 / D4). Those
// constants and `token_summary()` were removed together with the Inspector's
// `Tokens` dump. If a palette viewer is ever needed it belongs behind a dev
// flag, not on the default shell.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelTone {
    TopBar,
    Navigation,
    Editor,
    Context,
    Utility,
    Overlay,
}

pub fn panel() -> PackedRgba {
    PackedRgba::rgb(0x0D, 0x11, 0x17)
}

pub fn panel_alt() -> PackedRgba {
    PackedRgba::rgb(0x11, 0x18, 0x27)
}

pub fn border() -> PackedRgba {
    PackedRgba::rgb(0x1F, 0x29, 0x37)
}

pub fn text() -> PackedRgba {
    PackedRgba::rgb(0xE6, 0xE6, 0xE8)
}

pub fn muted() -> PackedRgba {
    PackedRgba::rgb(0x6C, 0x76, 0x80)
}

pub fn primary() -> PackedRgba {
    PackedRgba::rgb(0x39, 0xBA, 0xE6)
}

pub fn warn() -> PackedRgba {
    PackedRgba::rgb(0xFF, 0xB4, 0x54)
}

pub fn selection() -> PackedRgba {
    PackedRgba::rgb(0x21, 0x4D, 0x66)
}

// ---------------------------------------------------------------------
// Syntax-highlighting palette.
//
// The Editor panel uses these colours to paint VBA tokens (see
// `src/shell/highlight.rs`). Kept here next to the rest of the palette
// so any future theme toggle (W100) changes one file. Identifiers and
// punctuation deliberately have no override — they pick up the panel's
// active/inactive foreground from `content_style` and so remain
// legible when the panel loses focus.

/// Cyan/blue for control-flow and declaration keywords
/// (`Sub`, `Dim`, `If`, `End`, ...).
pub fn keyword_color() -> PackedRgba {
    PackedRgba::rgb(0x7C, 0xB3, 0xE8)
}

/// Softer cyan for built-in type names (`Integer`, `String`, ...),
/// distinct from control keywords so declarations read `Dim x As Integer`
/// with two visibly different accents.
pub fn type_keyword_color() -> PackedRgba {
    PackedRgba::rgb(0x5F, 0x9E, 0xA0)
}

/// Green for string literals.
pub fn string_color() -> PackedRgba {
    PackedRgba::rgb(0x8F, 0xC1, 0x75)
}

/// Soft yellow for numeric literals.
pub fn number_color() -> PackedRgba {
    PackedRgba::rgb(0xE6, 0xB4, 0x50)
}

pub fn style_keyword() -> Style {
    Style::new().fg(keyword_color()).bold()
}

pub fn style_type_keyword() -> Style {
    Style::new().fg(type_keyword_color())
}

pub fn style_string() -> Style {
    Style::new().fg(string_color())
}

pub fn style_number() -> Style {
    Style::new().fg(number_color())
}

pub fn style_comment() -> Style {
    Style::new().fg(muted())
}

/// Line-number gutter rendered in the muted foreground so it reads as
/// secondary information next to the source.
pub fn style_gutter() -> Style {
    Style::new().fg(muted())
}

fn panel_background(tone: PanelTone, active: bool) -> PackedRgba {
    match (tone, active) {
        (PanelTone::TopBar, true) => panel_alt(),
        (PanelTone::TopBar, false) => panel(),
        (PanelTone::Overlay, _) => panel_alt(),
        (PanelTone::Editor, _) => panel(),
        (_, true) => panel_alt(),
        (_, false) => panel(),
    }
}

pub fn panel_style(tone: PanelTone, active: bool) -> Style {
    Style::new().bg(panel_background(tone, active)).fg(text())
}

pub fn content_style(tone: PanelTone, active: bool) -> Style {
    let style = Style::new().bg(panel_background(tone, active));

    if active {
        style.fg(text())
    } else {
        style.fg(muted())
    }
}

pub fn border_style(tone: PanelTone, active: bool) -> Style {
    if active {
        let accent = match tone {
            PanelTone::TopBar | PanelTone::Editor | PanelTone::Overlay => primary(),
            PanelTone::Utility => warn(),
            PanelTone::Navigation | PanelTone::Context => selection(),
        };
        Style::new().fg(accent).bg(panel_alt()).bold()
    } else {
        Style::new().fg(border()).bg(panel_background(tone, active))
    }
}
