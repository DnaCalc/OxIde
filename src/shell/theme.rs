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
