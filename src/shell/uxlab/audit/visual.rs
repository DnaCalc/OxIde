use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use ab_glyph::{Font, FontArc, PxScale, ScaleFont, point};
use image::imageops::FilterType;
use image::{DynamicImage, ImageBuffer, ImageFormat, Rgba, RgbaImage};
use serde::Serialize;
use unicode_width::UnicodeWidthChar;

use super::export::is_audit_export_root;
use super::model::{AuditArtifactRef, UxAuditScenario, UxAuditSuite};
use crate::shell::uxlab::{LabCliSelection, LabRunError, LabScenarioRegistry, ViewportClass};

const DEFAULT_FONT_SIZE: f32 = 18.0;
const LABEL_FONT_SIZE: f32 = 24.0;
const PADDING_X: u32 = 24;
const PADDING_Y: u32 = 20;
const DEFAULT_FG: Color = Color(236, 242, 244);
const DEFAULT_BG: Color = Color(7, 11, 16);
const PANEL_BG: Color = Color(13, 17, 23);

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VisualReviewResult {
    pub root: String,
    pub files_written: Vec<String>,
    pub scenarios: Vec<VisualReviewScenarioResult>,
    pub status: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VisualReviewScenarioResult {
    pub audit_scenario_id: String,
    pub firehorse_scenario_id: String,
    pub viewport: String,
    pub terminal_png: String,
    pub ansi_stream: String,
    pub comparison_png: Option<String>,
    pub reference_image: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Color(u8, u8, u8);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Cell {
    ch: char,
    fg: Color,
    bg: Color,
    bold: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct RenderState {
    fg: Color,
    bg: Color,
    bold: bool,
}

impl Default for RenderState {
    fn default() -> Self {
        Self {
            fg: DEFAULT_FG,
            bg: DEFAULT_BG,
            bold: false,
        }
    }
}

pub fn export_visual_review(
    selection: &LabCliSelection,
    suite: &UxAuditSuite,
    lab_registry: &LabScenarioRegistry<'_>,
    root: &Path,
) -> Result<VisualReviewResult, LabRunError> {
    if !is_audit_export_root(root) {
        return Err(LabRunError::Io(format!(
            "refusing visual review outside docs/firehorse_mockups/ux_audit_lab or target/ux_audit_lab: {}",
            root.display()
        )));
    }

    let run_root = root.join(visual_run_directory_name(suite.id));
    fs::create_dir_all(&run_root).map_err(|error| LabRunError::Io(error.to_string()))?;
    let font = ReviewFonts::load()?;
    let scenarios = selected_scenarios(selection, suite)?;

    let mut files_written = Vec::new();
    let mut scenario_results = Vec::new();
    let mut terminal_images = Vec::new();

    for scenario in scenarios {
        let viewport = viewport_for(selection, scenario)?;
        let stem = review_stem(scenario.firehorse_scenario_id, viewport);
        let ansi = lab_registry.render_mockup_terminal_stream(
            suite.id,
            scenario.firehorse_scenario_id,
            Some(viewport),
        )?;
        let ansi_path = run_root.join(format!("{stem}.ansi"));
        write_new(&ansi_path, &ansi)?;

        let terminal_png = run_root.join(format!("{stem}.png"));
        let terminal = render_ansi_png(
            &ansi,
            viewport,
            &font,
            &format!("{} / {}", scenario.firehorse_scenario_id, viewport.name()),
        )?;
        write_png_new(&terminal_png, &terminal)?;

        let reference = reference_image(scenario);
        let comparison_png = if let Some(reference) = reference {
            let comparison_path = run_root.join(format!("{stem}_comparison.png"));
            let comparison = comparison_image(
                Path::new(reference.path),
                &terminal,
                &font,
                "Refined mockup",
                "Current terminal realization",
            )?;
            write_png_new(&comparison_path, &comparison)?;
            Some(comparison_path)
        } else {
            None
        };

        files_written.push(ansi_path.to_string_lossy().to_string());
        files_written.push(terminal_png.to_string_lossy().to_string());
        if let Some(path) = &comparison_png {
            files_written.push(path.to_string_lossy().to_string());
        }

        terminal_images.push((terminal_png.clone(), terminal));
        scenario_results.push(VisualReviewScenarioResult {
            audit_scenario_id: scenario.id.to_string(),
            firehorse_scenario_id: scenario.firehorse_scenario_id.to_string(),
            viewport: viewport.name().to_string(),
            terminal_png: terminal_png.to_string_lossy().to_string(),
            ansi_stream: ansi_path.to_string_lossy().to_string(),
            comparison_png: comparison_png.map(|path| path.to_string_lossy().to_string()),
            reference_image: reference.map(|artifact| artifact.path.to_string()),
        });
    }

    let contact_sheet = contact_sheet_image(&terminal_images, &font)?;
    let contact_sheet_path = run_root.join("firehorse_terminal_contact_sheet.png");
    write_png_new(&contact_sheet_path, &contact_sheet)?;
    files_written.push(contact_sheet_path.to_string_lossy().to_string());

    let readme_path = run_root.join("README.md");
    write_new(
        &readme_path,
        visual_review_readme(suite, &scenario_results, &contact_sheet_path).as_bytes(),
    )?;
    files_written.push(readme_path.to_string_lossy().to_string());

    Ok(VisualReviewResult {
        root: run_root.to_string_lossy().to_string(),
        files_written,
        scenarios: scenario_results,
        status: "exported",
    })
}

fn selected_scenarios<'a>(
    selection: &LabCliSelection,
    suite: &'a UxAuditSuite,
) -> Result<Vec<&'a UxAuditScenario>, LabRunError> {
    if let Some(id) = selection.scenario.as_deref() {
        Ok(vec![suite.find_scenario(id).ok_or_else(|| {
            LabRunError::UnknownScenario {
                suite: suite.id.to_string(),
                id: id.to_string(),
                available: suite
                    .scenarios
                    .iter()
                    .map(|scenario| {
                        format!(
                            "{}/{} -> {}",
                            suite.id, scenario.id, scenario.firehorse_scenario_id
                        )
                    })
                    .collect(),
            }
        })?])
    } else {
        Ok(suite.scenarios.iter().collect())
    }
}

fn viewport_for(
    selection: &LabCliSelection,
    scenario: &UxAuditScenario,
) -> Result<ViewportClass, LabRunError> {
    if let Some(viewport) = selection.viewport {
        Ok(viewport)
    } else {
        ViewportClass::parse(scenario.default_viewport).ok_or_else(|| {
            LabRunError::UnknownViewport {
                value: scenario.default_viewport.to_string(),
            }
        })
    }
}

fn render_ansi_png(
    ansi: &[u8],
    viewport: ViewportClass,
    fonts: &ReviewFonts,
    title: &str,
) -> Result<RgbaImage, LabRunError> {
    let terminal = TerminalCells::from_ansi(ansi, viewport)?;
    let metrics = FontMetrics::new(&fonts.regular, DEFAULT_FONT_SIZE);
    let title_metrics = FontMetrics::new(&fonts.bold, DEFAULT_FONT_SIZE);
    let title_height = title_metrics.cell_h + 8;
    let width = PADDING_X * 2 + terminal.width as u32 * metrics.cell_w;
    let height = PADDING_Y * 2 + title_height + terminal.height as u32 * metrics.cell_h;
    let mut image = ImageBuffer::from_pixel(width, height, rgba(DEFAULT_BG));
    draw_text(
        &mut image,
        &fonts.bold,
        DEFAULT_FONT_SIZE,
        PADDING_X,
        PADDING_Y,
        title,
        DEFAULT_FG,
    );

    let y0 = PADDING_Y + title_height;
    for row in 0..terminal.height {
        for col in 0..terminal.width {
            let cell = terminal.cell(row, col);
            if cell.bg != DEFAULT_BG {
                fill_rect(
                    &mut image,
                    PADDING_X + col as u32 * metrics.cell_w,
                    y0 + row as u32 * metrics.cell_h,
                    metrics.cell_w,
                    metrics.cell_h,
                    cell.bg,
                );
            }
        }
    }

    for row in 0..terminal.height {
        for col in 0..terminal.width {
            let cell = terminal.cell(row, col);
            if cell.ch == ' ' {
                continue;
            }
            draw_char(
                &mut image,
                if cell.bold {
                    &fonts.bold
                } else {
                    &fonts.regular
                },
                DEFAULT_FONT_SIZE,
                PADDING_X + col as u32 * metrics.cell_w,
                y0 + row as u32 * metrics.cell_h,
                cell.ch,
                cell.fg,
            );
        }
    }

    Ok(image)
}

fn comparison_image(
    reference_path: &Path,
    terminal: &RgbaImage,
    fonts: &ReviewFonts,
    left_label: &str,
    right_label: &str,
) -> Result<RgbaImage, LabRunError> {
    let reference = image::open(reference_path).map_err(|error| {
        LabRunError::Io(format!(
            "failed to open reference image {}: {error}",
            reference_path.display()
        ))
    })?;
    let reference = reference.to_rgba8();
    let max_height = 900u32;
    let reference = resize_to_height(&reference, max_height);
    let terminal = resize_to_height(terminal, max_height);
    let gap = 28;
    let label_height = 44;
    let width = 20 + reference.width() + gap + terminal.width() + 20;
    let height = label_height + max_height + 32;
    let mut image = ImageBuffer::from_pixel(width, height, rgba(DEFAULT_BG));

    draw_text(
        &mut image,
        &fonts.bold,
        LABEL_FONT_SIZE,
        20,
        12,
        left_label,
        DEFAULT_FG,
    );
    draw_text(
        &mut image,
        &fonts.bold,
        LABEL_FONT_SIZE,
        20 + reference.width() + gap,
        12,
        right_label,
        DEFAULT_FG,
    );
    overlay(&mut image, &reference, 20, label_height);
    overlay(
        &mut image,
        &terminal,
        20 + reference.width() + gap,
        label_height,
    );
    Ok(image)
}

fn contact_sheet_image(
    images: &[(PathBuf, RgbaImage)],
    fonts: &ReviewFonts,
) -> Result<RgbaImage, LabRunError> {
    let tile_w = 580;
    let tile_h = 380;
    let cols = 2;
    let rows = images.len().max(1).div_ceil(cols);
    let mut sheet =
        ImageBuffer::from_pixel(tile_w * cols as u32, tile_h * rows as u32, rgba(DEFAULT_BG));

    for (index, (path, image)) in images.iter().enumerate() {
        let col = index % cols;
        let row = index / cols;
        let x = col as u32 * tile_w;
        let y = row as u32 * tile_h;
        fill_rect(&mut sheet, x, y, tile_w, tile_h, PANEL_BG);
        draw_text(
            &mut sheet,
            &fonts.regular,
            14.0,
            x + 12,
            y + 10,
            &path
                .file_stem()
                .and_then(|name| name.to_str())
                .unwrap_or("terminal render")
                .chars()
                .take(64)
                .collect::<String>(),
            DEFAULT_FG,
        );
        let thumb = resize_to_box(image, 560, 330);
        overlay(&mut sheet, &thumb, x + 10, y + 40);
    }

    Ok(sheet)
}

struct TerminalCells {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl TerminalCells {
    fn from_ansi(ansi: &[u8], viewport: ViewportClass) -> Result<Self, LabRunError> {
        let size = viewport.wtd_size();
        let width = usize::from(size.width);
        let height = usize::from(size.height);
        let mut terminal = Self {
            width,
            height,
            cells: vec![default_cell(); width * height],
        };
        let text = std::str::from_utf8(ansi).map_err(|error| LabRunError::Io(error.to_string()))?;
        let mut state = RenderState::default();
        let mut row = 0usize;
        let mut col = 0usize;
        let mut chars = text.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\u{1b}' && chars.peek() == Some(&'[') {
                chars.next();
                let mut sequence = String::new();
                for seq_ch in chars.by_ref() {
                    sequence.push(seq_ch);
                    if ('@'..='~').contains(&seq_ch) {
                        break;
                    }
                }
                handle_csi(&sequence, &mut state, &mut row, &mut col);
                continue;
            }

            match ch {
                '\r' => col = 0,
                '\n' => {
                    row = row.saturating_add(1);
                    col = 0;
                }
                '\t' => col = ((col / 4) + 1) * 4,
                _ if ch.is_control() => {}
                _ => {
                    terminal.put(row, col, ch, state);
                    col = col.saturating_add(UnicodeWidthChar::width(ch).unwrap_or(1).max(1));
                }
            }
        }

        Ok(terminal)
    }

    fn put(&mut self, row: usize, col: usize, ch: char, state: RenderState) {
        if row >= self.height || col >= self.width {
            return;
        }
        self.cells[row * self.width + col] = Cell {
            ch,
            fg: state.fg,
            bg: state.bg,
            bold: state.bold,
        };
    }

    fn cell(&self, row: usize, col: usize) -> Cell {
        self.cells[row * self.width + col]
    }
}

fn handle_csi(sequence: &str, state: &mut RenderState, row: &mut usize, col: &mut usize) {
    let Some(final_byte) = sequence.chars().last() else {
        return;
    };
    let params = &sequence[..sequence.len().saturating_sub(final_byte.len_utf8())];
    match final_byte {
        'm' => apply_sgr(params, state),
        'H' | 'f' => {
            let mut parts = params.split(';').filter_map(|part| {
                if part.starts_with('?') {
                    None
                } else if part.is_empty() {
                    Some(1usize)
                } else {
                    part.parse::<usize>().ok()
                }
            });
            *row = parts.next().unwrap_or(1).saturating_sub(1);
            *col = parts.next().unwrap_or(1).saturating_sub(1);
        }
        _ => {}
    }
}

fn apply_sgr(params: &str, state: &mut RenderState) {
    if params.is_empty() {
        *state = RenderState::default();
        return;
    }

    let values = params
        .split(';')
        .map(|part| part.parse::<u16>().unwrap_or(0))
        .collect::<Vec<_>>();
    let mut index = 0;
    while index < values.len() {
        match values[index] {
            0 => *state = RenderState::default(),
            1 => state.bold = true,
            22 => state.bold = false,
            30..=37 => state.fg = ansi_16((values[index] - 30) as usize),
            39 => state.fg = DEFAULT_FG,
            40..=47 => state.bg = ansi_16((values[index] - 40) as usize),
            49 => state.bg = DEFAULT_BG,
            90..=97 => state.fg = ansi_16((8 + values[index] - 90) as usize),
            100..=107 => state.bg = ansi_16((8 + values[index] - 100) as usize),
            38 | 48 => {
                let is_fg = values[index] == 38;
                if values.get(index + 1) == Some(&2) && index + 4 < values.len() {
                    let color = Color(
                        clamp_color(values[index + 2]),
                        clamp_color(values[index + 3]),
                        clamp_color(values[index + 4]),
                    );
                    if is_fg {
                        state.fg = color;
                    } else {
                        state.bg = color;
                    }
                    index += 4;
                } else if values.get(index + 1) == Some(&5) && index + 2 < values.len() {
                    let color = ansi_256(values[index + 2] as usize);
                    if is_fg {
                        state.fg = color;
                    } else {
                        state.bg = color;
                    }
                    index += 2;
                }
            }
            _ => {}
        }
        index += 1;
    }
}

struct ReviewFonts {
    regular: FontArc,
    bold: FontArc,
}

impl ReviewFonts {
    fn load() -> Result<Self, LabRunError> {
        let regular = load_font(&[
            "C:/Windows/Fonts/CascadiaMono.ttf",
            "C:/Windows/Fonts/consola.ttf",
            "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf",
        ])?;
        let bold = load_font(&[
            "C:/Windows/Fonts/CascadiaCode.ttf",
            "C:/Windows/Fonts/consolab.ttf",
            "/usr/share/fonts/truetype/dejavu/DejaVuSansMono-Bold.ttf",
        ])
        .unwrap_or_else(|_| regular.clone());
        Ok(Self { regular, bold })
    }
}

struct FontMetrics {
    cell_w: u32,
    cell_h: u32,
}

impl FontMetrics {
    fn new(font: &FontArc, size: f32) -> Self {
        let scale = PxScale::from(size);
        let scaled = font.as_scaled(scale);
        let glyph = font.glyph_id('M');
        let cell_w = scaled.h_advance(glyph).ceil() as u32 + 1;
        let cell_h = (scaled.ascent() - scaled.descent()).ceil() as u32 + 4;
        Self { cell_w, cell_h }
    }
}

fn load_font(candidates: &[&str]) -> Result<FontArc, LabRunError> {
    for candidate in candidates {
        let path = Path::new(candidate);
        if !path.exists() {
            continue;
        }
        let bytes = fs::read(path).map_err(|error| LabRunError::Io(error.to_string()))?;
        return FontArc::try_from_vec(bytes).map_err(|error| {
            LabRunError::Io(format!("failed to load font {}: {error}", path.display()))
        });
    }
    Err(LabRunError::Io(
        "failed to find a monospace font for visual review PNG rendering".to_string(),
    ))
}

fn draw_text(
    image: &mut RgbaImage,
    font: &FontArc,
    size: f32,
    mut x: u32,
    y: u32,
    text: &str,
    color: Color,
) {
    let scale = PxScale::from(size);
    let scaled = font.as_scaled(scale);
    for ch in text.chars() {
        draw_char(image, font, size, x, y, ch, color);
        x = x.saturating_add(scaled.h_advance(font.glyph_id(ch)).ceil() as u32);
    }
}

fn draw_char(
    image: &mut RgbaImage,
    font: &FontArc,
    size: f32,
    x: u32,
    y: u32,
    ch: char,
    color: Color,
) {
    let scale = PxScale::from(size);
    let scaled = font.as_scaled(scale);
    let glyph = font
        .glyph_id(ch)
        .with_scale_and_position(scale, point(x as f32, y as f32 + scaled.ascent() + 2.0));
    let Some(outlined) = font.outline_glyph(glyph) else {
        return;
    };
    let bounds = outlined.px_bounds();
    outlined.draw(|glyph_x, glyph_y, coverage| {
        let px = bounds.min.x as i32 + glyph_x as i32;
        let py = bounds.min.y as i32 + glyph_y as i32;
        if px < 0 || py < 0 {
            return;
        }
        let px = px as u32;
        let py = py as u32;
        if px >= image.width() || py >= image.height() {
            return;
        }
        blend_pixel(image.get_pixel_mut(px, py), color, coverage);
    });
}

fn fill_rect(image: &mut RgbaImage, x: u32, y: u32, width: u32, height: u32, color: Color) {
    let max_x = x.saturating_add(width).min(image.width());
    let max_y = y.saturating_add(height).min(image.height());
    for py in y..max_y {
        for px in x..max_x {
            image.put_pixel(px, py, rgba(color));
        }
    }
}

fn overlay(base: &mut RgbaImage, overlay: &RgbaImage, x: u32, y: u32) {
    for py in 0..overlay.height() {
        for px in 0..overlay.width() {
            let target_x = x + px;
            let target_y = y + py;
            if target_x < base.width() && target_y < base.height() {
                base.put_pixel(target_x, target_y, *overlay.get_pixel(px, py));
            }
        }
    }
}

fn resize_to_height(image: &RgbaImage, height: u32) -> RgbaImage {
    let width = ((image.width() as f32 * height as f32) / image.height().max(1) as f32)
        .round()
        .max(1.0) as u32;
    image::imageops::resize(image, width, height, FilterType::Lanczos3)
}

fn resize_to_box(image: &RgbaImage, max_width: u32, max_height: u32) -> RgbaImage {
    let scale = (max_width as f32 / image.width().max(1) as f32)
        .min(max_height as f32 / image.height().max(1) as f32)
        .min(1.0);
    let width = (image.width() as f32 * scale).round().max(1.0) as u32;
    let height = (image.height() as f32 * scale).round().max(1.0) as u32;
    image::imageops::resize(image, width, height, FilterType::Lanczos3)
}

fn blend_pixel(pixel: &mut Rgba<u8>, color: Color, alpha: f32) {
    let alpha = alpha.clamp(0.0, 1.0);
    let inv = 1.0 - alpha;
    pixel.0[0] = (color.0 as f32 * alpha + pixel.0[0] as f32 * inv).round() as u8;
    pixel.0[1] = (color.1 as f32 * alpha + pixel.0[1] as f32 * inv).round() as u8;
    pixel.0[2] = (color.2 as f32 * alpha + pixel.0[2] as f32 * inv).round() as u8;
    pixel.0[3] = 255;
}

fn write_png_new(path: &Path, image: &RgbaImage) -> Result<(), LabRunError> {
    let file = create_new(path)?;
    let mut writer = std::io::BufWriter::new(file);
    DynamicImage::ImageRgba8(image.clone())
        .write_to(&mut writer, ImageFormat::Png)
        .map_err(|error| LabRunError::Io(error.to_string()))?;
    writer
        .flush()
        .map_err(|error| LabRunError::Io(error.to_string()))?;
    Ok(())
}

fn write_new(path: &Path, bytes: &[u8]) -> Result<(), LabRunError> {
    let mut file = create_new(path)?;
    file.write_all(bytes)
        .map_err(|error| LabRunError::Io(error.to_string()))
}

fn create_new(path: &Path) -> Result<std::fs::File, LabRunError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| LabRunError::Io(error.to_string()))?;
    }
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| LabRunError::Io(format!("failed to create {}: {error}", path.display())))
}

fn reference_image(scenario: &UxAuditScenario) -> Option<&AuditArtifactRef> {
    scenario
        .reference_artifacts
        .iter()
        .find(|artifact| artifact.kind == "image" && Path::new(artifact.path).exists())
}

fn visual_review_readme(
    suite: &UxAuditSuite,
    scenarios: &[VisualReviewScenarioResult],
    contact_sheet: &Path,
) -> String {
    let mut out = String::new();
    out.push_str("# Fire Horse Visual Review\n\n");
    out.push_str(&format!("Suite: `{}`  \n", suite.id));
    out.push_str(&format!("Scenarios: `{}`  \n", scenarios.len()));
    out.push_str(&format!(
        "Contact sheet: `{}`\n\n",
        contact_sheet.to_string_lossy()
    ));
    out.push_str("These PNGs are fixed-size renders of the ANSI terminal stream, so terminal window resizing does not affect visual review.\n\n");
    for scenario in scenarios {
        out.push_str(&format!(
            "- `{}` `{}` -> `{}`",
            scenario.firehorse_scenario_id, scenario.viewport, scenario.terminal_png
        ));
        if let Some(comparison) = &scenario.comparison_png {
            out.push_str(&format!(" | comparison `{comparison}`"));
        }
        out.push('\n');
    }
    out
}

fn visual_run_directory_name(suite_id: &str) -> String {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!(
        "visual_{}_{}_{:09}_{}",
        sanitize_file_stem(suite_id),
        duration.as_secs(),
        duration.subsec_nanos(),
        std::process::id()
    )
}

fn review_stem(scenario_id: &str, viewport: ViewportClass) -> String {
    sanitize_file_stem(&format!("{}_{}", scenario_id, viewport.name()))
}

fn sanitize_file_stem(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

fn default_cell() -> Cell {
    Cell {
        ch: ' ',
        fg: DEFAULT_FG,
        bg: DEFAULT_BG,
        bold: false,
    }
}

fn rgba(color: Color) -> Rgba<u8> {
    Rgba([color.0, color.1, color.2, 255])
}

fn clamp_color(value: u16) -> u8 {
    value.min(255) as u8
}

fn ansi_16(index: usize) -> Color {
    const COLORS: [Color; 16] = [
        Color(0, 0, 0),
        Color(128, 0, 0),
        Color(0, 128, 0),
        Color(128, 128, 0),
        Color(0, 0, 128),
        Color(128, 0, 128),
        Color(0, 128, 128),
        Color(192, 192, 192),
        Color(128, 128, 128),
        Color(255, 0, 0),
        Color(0, 255, 0),
        Color(255, 255, 0),
        Color(0, 0, 255),
        Color(255, 0, 255),
        Color(0, 255, 255),
        Color(255, 255, 255),
    ];
    COLORS[index.min(COLORS.len() - 1)]
}

fn ansi_256(index: usize) -> Color {
    if index < 16 {
        return ansi_16(index);
    }
    if index < 232 {
        let value = index - 16;
        let levels = [0, 95, 135, 175, 215, 255];
        return Color(
            levels[value / 36],
            levels[(value / 6) % 6],
            levels[value % 6],
        );
    }
    let gray = 8 + (index.saturating_sub(232).min(23) as u8 * 10);
    Color(gray, gray, gray)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ansi_parser_preserves_truecolor_cells() {
        let cells = TerminalCells::from_ansi(
            b"\x1b[2;3H\x1b[38;2;1;2;3m\x1b[48;2;4;5;6mA",
            ViewportClass::Standard,
        )
        .expect("parse ansi");

        let cell = cells.cell(1, 2);
        assert_eq!(cell.ch, 'A');
        assert_eq!(cell.fg, Color(1, 2, 3));
        assert_eq!(cell.bg, Color(4, 5, 6));
    }

    #[test]
    fn review_stem_is_filesystem_safe() {
        assert_eq!(
            review_stem("firehorse/editing:lens", ViewportClass::Studio),
            "firehorse_editing_lens_studio"
        );
    }
}
