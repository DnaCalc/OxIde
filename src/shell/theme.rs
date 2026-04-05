use ftui::{PackedRgba, Style};

pub const BACKGROUND: &str = "#0A0E14";
pub const PANEL: &str = "#0D1117";
pub const PANEL_ALT: &str = "#111827";
pub const BORDER: &str = "#1F2937";
pub const TEXT: &str = "#E6E6E8";
pub const MUTED: &str = "#6C7680";
pub const PRIMARY: &str = "#39BAE6";
pub const WARN: &str = "#FFB454";
pub const ERROR_HOT: &str = "#F97E72";
pub const SUCCESS: &str = "#50FA7B";
pub const SELECTION: &str = "#214D66";

pub fn token_summary() -> String {
    format!(
        "bg {BACKGROUND} panel {PANEL} panel-alt {PANEL_ALT} border {BORDER} text {TEXT} muted {MUTED} primary {PRIMARY} warn {WARN} hot {ERROR_HOT} success {SUCCESS} select {SELECTION}"
    )
}

pub fn palette_name() -> &'static str {
    "Mockup-derived instrument palette"
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelTone {
    TopBar,
    Navigation,
    Editor,
    Context,
    Utility,
    Overlay,
}

pub fn background() -> PackedRgba {
    PackedRgba::rgb(0x0A, 0x0E, 0x14)
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

pub fn panel_style(tone: PanelTone, active: bool) -> Style {
    let bg = match (tone, active) {
        (PanelTone::TopBar, true) => panel_alt(),
        (PanelTone::TopBar, false) => panel(),
        (PanelTone::Overlay, _) => panel_alt(),
        (PanelTone::Editor, _) => panel(),
        (_, true) => panel_alt(),
        (_, false) => background(),
    };

    Style::new().bg(bg).fg(text())
}

pub fn content_style(tone: PanelTone, active: bool) -> Style {
    let style = Style::new().bg(match (tone, active) {
        (PanelTone::TopBar, true) => panel_alt(),
        (PanelTone::TopBar, false) => panel(),
        (PanelTone::Overlay, _) => panel_alt(),
        (PanelTone::Editor, _) => panel(),
        (_, true) => panel_alt(),
        (_, false) => background(),
    });

    if active {
        style.fg(text())
    } else {
        style.fg(muted()).dim()
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
        Style::new().fg(border()).bg(background())
    }
}
