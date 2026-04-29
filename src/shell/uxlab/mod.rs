//! Lab-only UX scenario registry.
//!
//! W038 Phase 1 keeps this deliberately small: named scenarios,
//! fixed viewport classes, and provider composition. Rendering and
//! interactive browsing land in later beads.

pub mod firehorse;

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io::Write;

use ftui::{Cell, Frame, GraphemePool};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ViewportSize {
    pub width: u16,
    pub height: u16,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ViewportClass {
    Standard,
    Compact,
    Wide,
    FirstClass,
    Studio,
}

impl ViewportClass {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Compact => "compact",
            Self::Wide => "wide",
            Self::FirstClass => "first-class",
            Self::Studio => "studio",
        }
    }

    pub const fn wtd_size(self) -> ViewportSize {
        match self {
            Self::Standard => ViewportSize {
                width: 120,
                height: 34,
            },
            Self::Compact => ViewportSize {
                width: 92,
                height: 30,
            },
            Self::Wide => ViewportSize {
                width: 160,
                height: 40,
            },
            Self::FirstClass => ViewportSize {
                width: 160,
                height: 42,
            },
            Self::Studio => ViewportSize {
                width: 190,
                height: 48,
            },
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "standard" => Some(Self::Standard),
            "compact" => Some(Self::Compact),
            "wide" => Some(Self::Wide),
            "first-class" => Some(Self::FirstClass),
            "studio" => Some(Self::Studio),
            _ => None,
        }
    }
}

impl fmt::Display for ViewportClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LabScenarioDescriptor {
    pub id: &'static str,
    pub suite: &'static str,
    pub title: &'static str,
    pub purpose: &'static str,
    pub default_viewport: ViewportClass,
    pub tags: &'static [&'static str],
}

impl LabScenarioDescriptor {
    pub const fn default_size(self) -> ViewportSize {
        self.default_viewport.wtd_size()
    }
}

pub trait LabScenarioProvider {
    fn suite(&self) -> &'static str;
    fn scenarios(&self) -> &'static [LabScenarioDescriptor];
    fn render(
        &self,
        scenario_id: &str,
        viewport: ViewportClass,
    ) -> Result<LabRenderedFrame, LabRenderError>;

    fn render_mockup(
        &self,
        scenario_id: &str,
        viewport: ViewportClass,
    ) -> Result<LabRenderedFrame, LabRenderError> {
        self.render(scenario_id, viewport)
    }

    fn render_mockup_terminal_stream(
        &self,
        scenario_id: &str,
        viewport: ViewportClass,
    ) -> Result<Vec<u8>, LabRenderError> {
        Ok(self.render_mockup(scenario_id, viewport)?.text.into_bytes())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum LabRegistryError {
    DuplicateScenarioId {
        id: &'static str,
        first_suite: &'static str,
        duplicate_suite: &'static str,
    },
    ProviderSuiteMismatch {
        provider_suite: &'static str,
        scenario_id: &'static str,
        scenario_suite: &'static str,
    },
}

impl fmt::Display for LabRegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateScenarioId {
                id,
                first_suite,
                duplicate_suite,
            } => write!(
                f,
                "duplicate lab scenario id '{id}' registered by suites '{first_suite}' and '{duplicate_suite}'"
            ),
            Self::ProviderSuiteMismatch {
                provider_suite,
                scenario_id,
                scenario_suite,
            } => write!(
                f,
                "lab provider suite '{provider_suite}' returned scenario '{scenario_id}' with suite '{scenario_suite}'"
            ),
        }
    }
}

impl Error for LabRegistryError {}

pub struct LabScenarioRegistry<'a> {
    scenarios: Vec<LabScenarioDescriptor>,
    providers_by_id: HashMap<&'static str, &'a dyn LabScenarioProvider>,
}

impl<'registry> LabScenarioRegistry<'registry> {
    pub fn from_providers(
        providers: impl IntoIterator<Item = &'registry dyn LabScenarioProvider>,
    ) -> Result<LabScenarioRegistry<'registry>, LabRegistryError> {
        let mut scenarios = Vec::new();
        let mut ids: HashMap<&'static str, &'static str> = HashMap::new();
        let mut providers_by_id = HashMap::new();

        for provider in providers {
            let provider_suite = provider.suite();
            for scenario in provider.scenarios() {
                if scenario.suite != provider_suite {
                    return Err(LabRegistryError::ProviderSuiteMismatch {
                        provider_suite,
                        scenario_id: scenario.id,
                        scenario_suite: scenario.suite,
                    });
                }

                if let Some(first_suite) = ids.insert(scenario.id, scenario.suite) {
                    return Err(LabRegistryError::DuplicateScenarioId {
                        id: scenario.id,
                        first_suite,
                        duplicate_suite: scenario.suite,
                    });
                }

                scenarios.push(*scenario);
                providers_by_id.insert(scenario.id, provider);
            }
        }

        scenarios.sort_by(|left, right| {
            left.suite
                .cmp(right.suite)
                .then_with(|| left.id.cmp(right.id))
        });

        Ok(LabScenarioRegistry {
            scenarios,
            providers_by_id,
        })
    }

    pub fn scenarios(&self) -> &[LabScenarioDescriptor] {
        &self.scenarios
    }

    pub fn find(&self, suite: &str, id: &str) -> Option<&LabScenarioDescriptor> {
        self.scenarios
            .iter()
            .find(|scenario| scenario.suite == suite && scenario.id == id)
    }

    pub fn render(
        &self,
        suite: &str,
        id: &str,
        viewport: Option<ViewportClass>,
    ) -> Result<LabRenderedFrame, LabRunError> {
        let descriptor = self
            .find(suite, id)
            .ok_or_else(|| self.unknown_scenario(suite, id))?;
        let provider = self
            .providers_by_id
            .get(descriptor.id)
            .expect("registered descriptor must have a provider");
        let viewport = viewport.unwrap_or(descriptor.default_viewport);
        provider
            .render(descriptor.id, viewport)
            .map_err(LabRunError::Render)
    }

    pub fn render_mockup(
        &self,
        suite: &str,
        id: &str,
        viewport: Option<ViewportClass>,
    ) -> Result<LabRenderedFrame, LabRunError> {
        let descriptor = self
            .find(suite, id)
            .ok_or_else(|| self.unknown_scenario(suite, id))?;
        let provider = self
            .providers_by_id
            .get(descriptor.id)
            .expect("registered descriptor must have a provider");
        let viewport = viewport.unwrap_or(descriptor.default_viewport);
        provider
            .render_mockup(descriptor.id, viewport)
            .map_err(LabRunError::Render)
    }

    pub fn render_mockup_terminal_stream(
        &self,
        suite: &str,
        id: &str,
        viewport: Option<ViewportClass>,
    ) -> Result<Vec<u8>, LabRunError> {
        let descriptor = self
            .find(suite, id)
            .ok_or_else(|| self.unknown_scenario(suite, id))?;
        let provider = self
            .providers_by_id
            .get(descriptor.id)
            .expect("registered descriptor must have a provider");
        let viewport = viewport.unwrap_or(descriptor.default_viewport);
        provider
            .render_mockup_terminal_stream(descriptor.id, viewport)
            .map_err(LabRunError::Render)
    }

    pub fn available_rows(&self) -> Vec<String> {
        self.scenarios
            .iter()
            .map(|scenario| Self::scenario_row(scenario))
            .collect()
    }

    pub fn available_rows_for_suite(&self, suite: &str) -> Vec<String> {
        self.scenarios
            .iter()
            .filter(|scenario| scenario.suite == suite)
            .map(|scenario| Self::scenario_row(scenario))
            .collect()
    }

    fn unknown_scenario(&self, suite: &str, id: &str) -> LabRunError {
        let requested_suite_exists = self
            .scenarios
            .iter()
            .any(|scenario| scenario.suite == suite);
        if requested_suite_exists {
            LabRunError::UnknownScenario {
                suite: suite.to_string(),
                id: id.to_string(),
                available: self.available_rows(),
            }
        } else {
            LabRunError::UnknownSuite {
                suite: suite.to_string(),
                available: self.available_rows(),
            }
        }
    }

    fn scenario_row(scenario: &LabScenarioDescriptor) -> String {
        let size = scenario.default_size();
        format!(
            "{}/{} | {} | {} {}x{} | tags: {}",
            scenario.suite,
            scenario.id,
            scenario.title,
            scenario.default_viewport.name(),
            size.width,
            size.height,
            scenario.tags.join(",")
        )
    }
}

impl LabScenarioRegistry<'static> {
    pub fn built_in() -> Self {
        LabScenarioRegistry::from_providers([
            &SMOKE_PROVIDER as &dyn LabScenarioProvider,
            &firehorse::FIRE_HORSE_PROVIDER as &dyn LabScenarioProvider,
        ])
        .expect("built-in UX lab providers must be valid")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LabRenderedFrame {
    pub scenario: LabScenarioDescriptor,
    pub viewport: ViewportClass,
    pub size: ViewportSize,
    pub text: String,
}

#[derive(Debug, Eq, PartialEq)]
pub enum LabRenderError {
    UnknownScenario { suite: &'static str, id: String },
}

impl fmt::Display for LabRenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownScenario { suite, id } => {
                write!(f, "lab suite '{suite}' cannot render scenario '{id}'")
            }
        }
    }
}

impl Error for LabRenderError {}

#[derive(Debug, Eq, PartialEq)]
pub enum LabRunError {
    MissingValue {
        flag: &'static str,
    },
    MissingOnceSelection,
    MissingScenario,
    MissingSuite,
    UnknownArgument {
        value: String,
    },
    UnknownViewport {
        value: String,
    },
    UnknownScenario {
        suite: String,
        id: String,
        available: Vec<String>,
    },
    UnknownSuite {
        suite: String,
        available: Vec<String>,
    },
    Render(LabRenderError),
    Io(String),
}

impl fmt::Display for LabRunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingValue { flag } => write!(f, "missing value for {flag}"),
            Self::MissingOnceSelection => {
                write!(
                    f,
                    "interactive UX lab is not implemented yet; use --once or --list"
                )
            }
            Self::MissingScenario => write!(f, "--once requires --scenario <id>"),
            Self::MissingSuite => write!(f, "--once requires --suite <suite>"),
            Self::UnknownArgument { value } => write!(f, "unknown oxide-uxlab argument: {value}"),
            Self::UnknownViewport { value } => write!(
                f,
                "unknown viewport '{value}'; valid viewports: standard, compact, wide, first-class, studio"
            ),
            Self::UnknownScenario {
                suite,
                id,
                available,
            } => write!(
                f,
                "unknown scenario '{suite}/{id}'. Available scenarios:\n{}",
                available.join("\n")
            ),
            Self::UnknownSuite { suite, available } => write!(
                f,
                "unknown suite '{suite}'. Available scenarios:\n{}",
                available.join("\n")
            ),
            Self::Render(error) => write!(f, "{error}"),
            Self::Io(error) => write!(f, "{error}"),
        }
    }
}

impl Error for LabRunError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LabCliMode {
    Once,
    List,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LabCliSelection {
    pub suite: Option<String>,
    pub scenario: Option<String>,
    pub viewport: Option<ViewportClass>,
    pub mode: Option<LabCliMode>,
    pub mockup: bool,
    pub ansi: bool,
}

impl LabCliSelection {
    pub fn parse(args: impl IntoIterator<Item = String>) -> Result<Self, LabRunError> {
        let mut suite = None;
        let mut scenario = None;
        let mut viewport = None;
        let mut mode = None;
        let mut mockup = false;
        let mut ansi = false;
        let mut args = args.into_iter();

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--suite" => {
                    suite = Some(next_value(&mut args, "--suite")?);
                }
                "--scenario" => {
                    scenario = Some(next_value(&mut args, "--scenario")?);
                }
                "--viewport" => {
                    let value = next_value(&mut args, "--viewport")?;
                    viewport = Some(
                        ViewportClass::parse(&value)
                            .ok_or(LabRunError::UnknownViewport { value })?,
                    );
                }
                "--once" => {
                    mode = Some(LabCliMode::Once);
                }
                "--mockup" => {
                    mockup = true;
                }
                "--ansi" => {
                    ansi = true;
                }
                "--list" => {
                    mode = Some(LabCliMode::List);
                }
                "--help" | "-h" => {
                    mode = Some(LabCliMode::List);
                }
                _ => return Err(LabRunError::UnknownArgument { value: arg }),
            }
        }

        Ok(Self {
            suite,
            scenario,
            viewport,
            mode,
            mockup,
            ansi,
        })
    }
}

fn next_value(
    args: &mut impl Iterator<Item = String>,
    flag: &'static str,
) -> Result<String, LabRunError> {
    args.next().ok_or(LabRunError::MissingValue { flag })
}

pub fn run_cli<I, W>(
    args: I,
    registry: &LabScenarioRegistry<'_>,
    mut out: W,
) -> Result<(), LabRunError>
where
    I: IntoIterator<Item = String>,
    W: Write,
{
    let selection = LabCliSelection::parse(args)?;
    match selection.mode {
        Some(LabCliMode::List) => {
            writeln!(out, "OxIde UX lab scenarios")
                .map_err(|error| LabRunError::Io(error.to_string()))?;
            let rows = if let Some(suite) = selection.suite.as_deref() {
                let rows = registry.available_rows_for_suite(suite);
                if rows.is_empty() {
                    return Err(LabRunError::UnknownSuite {
                        suite: suite.to_string(),
                        available: registry.available_rows(),
                    });
                }
                rows
            } else {
                registry.available_rows()
            };
            for row in rows {
                writeln!(out, "{row}").map_err(|error| LabRunError::Io(error.to_string()))?;
            }
            Ok(())
        }
        Some(LabCliMode::Once) => {
            let suite = selection.suite.ok_or(LabRunError::MissingSuite)?;
            let scenario = selection.scenario.ok_or(LabRunError::MissingScenario)?;
            if selection.mockup && selection.ansi {
                let stream = registry.render_mockup_terminal_stream(
                    &suite,
                    &scenario,
                    selection.viewport,
                )?;
                out.write_all(&stream)
                    .map_err(|error| LabRunError::Io(error.to_string()))?;
            } else {
                let rendered = if selection.mockup {
                    registry.render_mockup(&suite, &scenario, selection.viewport)?
                } else {
                    registry.render(&suite, &scenario, selection.viewport)?
                };
                write!(out, "{}", rendered.text)
                    .map_err(|error| LabRunError::Io(error.to_string()))?;
            }
            Ok(())
        }
        None => Err(LabRunError::MissingOnceSelection),
    }
}

pub struct SmokeScenarioProvider;

impl LabScenarioProvider for SmokeScenarioProvider {
    fn suite(&self) -> &'static str {
        "lab-smoke"
    }

    fn scenarios(&self) -> &'static [LabScenarioDescriptor] {
        &SMOKE_SCENARIOS
    }

    fn render(
        &self,
        scenario_id: &str,
        viewport: ViewportClass,
    ) -> Result<LabRenderedFrame, LabRenderError> {
        let scenario = SMOKE_SCENARIOS
            .iter()
            .find(|scenario| scenario.id == scenario_id)
            .copied()
            .ok_or_else(|| LabRenderError::UnknownScenario {
                suite: self.suite(),
                id: scenario_id.to_string(),
            })?;
        Ok(render_smoke_frame(scenario, viewport))
    }
}

pub static SMOKE_PROVIDER: SmokeScenarioProvider = SmokeScenarioProvider;

pub static SMOKE_SCENARIOS: [LabScenarioDescriptor; 1] = [LabScenarioDescriptor {
    id: "lab-smoke-editing",
    suite: "lab-smoke",
    title: "Lab Smoke Editing",
    purpose: "Proves the W038 scenario registry and viewport contract without Fire Horse fixtures.",
    default_viewport: ViewportClass::Standard,
    tags: &["smoke", "editing", "w038-phase-1"],
}];

fn render_smoke_frame(
    scenario: LabScenarioDescriptor,
    viewport: ViewportClass,
) -> LabRenderedFrame {
    let size = viewport.wtd_size();
    let mut pool = GraphemePool::new();
    let mut frame = Frame::new(size.width, size.height, &mut pool);
    frame.set_cursor(None);
    frame.set_cursor_visible(false);

    write_text(&mut frame, 0, 0, "OxIde UX Lab");
    write_text(
        &mut frame,
        0,
        1,
        &format!("suite: {}  scenario: {}", scenario.suite, scenario.id),
    );
    write_text(&mut frame, 0, 2, &format!("title: {}", scenario.title));
    write_text(
        &mut frame,
        0,
        3,
        &format!(
            "viewport: {} {}x{}",
            viewport.name(),
            size.width,
            size.height
        ),
    );
    write_text(
        &mut frame,
        0,
        5,
        "Project Spine | Code Canvas | Context Dock",
    );
    write_text(&mut frame, 0, 7, "Public Sub LabSmoke()");
    write_text(&mut frame, 0, 8, "    Debug.Print \"W038 lab smoke\"");
    write_text(&mut frame, 0, 9, "End Sub");
    write_text(
        &mut frame,
        0,
        size.height.saturating_sub(2),
        "Activity: Problems 0 | Output ready",
    );
    write_text(
        &mut frame,
        0,
        size.height.saturating_sub(1),
        "F6 Command Lens  Enter run  Esc close",
    );

    let text = frame_to_text(&frame);
    LabRenderedFrame {
        scenario,
        viewport,
        size,
        text,
    }
}

fn write_text(frame: &mut Frame<'_>, x: u16, y: u16, text: &str) {
    if y >= frame.height() || x >= frame.width() {
        return;
    }

    let mut column = x;
    for ch in text.chars() {
        if column >= frame.width() {
            break;
        }
        frame.buffer.set(column, y, Cell::from_char(ch));
        column += 1;
    }
}

pub(crate) fn frame_to_text(frame: &Frame<'_>) -> String {
    let mut output = String::new();
    for y in 0..frame.height() {
        if y > 0 {
            output.push('\n');
        }
        for x in 0..frame.width() {
            let cell = frame
                .buffer
                .get(x, y)
                .expect("frame coordinates should be in bounds");
            if let Some(ch) = cell.content.as_char() {
                output.push(ch);
            } else if let Some(id) = cell.content.grapheme_id() {
                output.push_str(frame.pool.get(id).unwrap_or(" "));
            } else {
                output.push(' ');
            }
        }
    }
    output.push('\n');
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    struct StaticProvider {
        suite: &'static str,
        scenarios: &'static [LabScenarioDescriptor],
    }

    impl LabScenarioProvider for StaticProvider {
        fn suite(&self) -> &'static str {
            self.suite
        }

        fn scenarios(&self) -> &'static [LabScenarioDescriptor] {
            self.scenarios
        }

        fn render(
            &self,
            scenario_id: &str,
            _viewport: ViewportClass,
        ) -> Result<LabRenderedFrame, LabRenderError> {
            Err(LabRenderError::UnknownScenario {
                suite: self.suite,
                id: scenario_id.to_string(),
            })
        }
    }

    static DUPLICATE_A: [LabScenarioDescriptor; 1] = [LabScenarioDescriptor {
        id: "duplicate-id",
        suite: "first-suite",
        title: "First",
        purpose: "First duplicate owner.",
        default_viewport: ViewportClass::Standard,
        tags: &["first"],
    }];

    static DUPLICATE_B: [LabScenarioDescriptor; 1] = [LabScenarioDescriptor {
        id: "duplicate-id",
        suite: "second-suite",
        title: "Second",
        purpose: "Second duplicate owner.",
        default_viewport: ViewportClass::Compact,
        tags: &["second"],
    }];

    #[test]
    fn smoke_provider_appears_in_registry_by_id() {
        let registry = LabScenarioRegistry::built_in();

        let scenario = registry
            .find("lab-smoke", "lab-smoke-editing")
            .expect("smoke scenario should be registered");

        assert_eq!(scenario.title, "Lab Smoke Editing");
        assert_eq!(scenario.default_viewport, ViewportClass::Standard);
        assert_eq!(scenario.tags, &["smoke", "editing", "w038-phase-1"]);
    }

    #[test]
    fn duplicate_scenario_ids_fail_with_named_error() {
        let first = StaticProvider {
            suite: "first-suite",
            scenarios: &DUPLICATE_A,
        };
        let second = StaticProvider {
            suite: "second-suite",
            scenarios: &DUPLICATE_B,
        };

        let error = match LabScenarioRegistry::from_providers([
            &first as &dyn LabScenarioProvider,
            &second as &dyn LabScenarioProvider,
        ]) {
            Ok(_) => panic!("duplicate scenario ids must fail"),
            Err(error) => error,
        };

        assert_eq!(
            error,
            LabRegistryError::DuplicateScenarioId {
                id: "duplicate-id",
                first_suite: "first-suite",
                duplicate_suite: "second-suite",
            }
        );
        assert!(error.to_string().contains("duplicate-id"));
    }

    #[test]
    fn standard_and_compact_viewports_resolve_to_fixed_wtd_sizes() {
        assert_eq!(
            ViewportClass::Standard.wtd_size(),
            ViewportSize {
                width: 120,
                height: 34,
            }
        );
        assert_eq!(
            ViewportClass::Compact.wtd_size(),
            ViewportSize {
                width: 92,
                height: 30,
            }
        );
        assert_eq!(ViewportClass::Standard.name(), "standard");
        assert_eq!(
            ViewportClass::parse("compact"),
            Some(ViewportClass::Compact)
        );
        assert_eq!(
            ViewportClass::FirstClass.wtd_size(),
            ViewportSize {
                width: 160,
                height: 42,
            }
        );
        assert_eq!(
            ViewportClass::Studio.wtd_size(),
            ViewportSize {
                width: 190,
                height: 48,
            }
        );
        assert_eq!(
            ViewportClass::parse("first-class"),
            Some(ViewportClass::FirstClass)
        );
        assert_eq!(ViewportClass::parse("studio"), Some(ViewportClass::Studio));
    }

    #[test]
    fn cli_selection_resolves_smoke_scenario_by_suite_and_id() {
        let registry = LabScenarioRegistry::built_in();
        let mut output = Vec::new();

        run_cli(
            [
                "--suite",
                "lab-smoke",
                "--scenario",
                "lab-smoke-editing",
                "--viewport",
                "standard",
                "--once",
            ]
            .into_iter()
            .map(String::from),
            &registry,
            &mut output,
        )
        .expect("smoke --once should render");

        let text = String::from_utf8(output).expect("utf8 smoke output");
        assert!(text.contains("Lab Smoke Editing"));
        assert!(text.contains("viewport: standard 120x34"));
        assert!(text.contains("F6 Command Lens"));
    }

    #[test]
    fn unknown_scenario_errors_include_valid_alternatives() {
        let registry = LabScenarioRegistry::built_in();
        let mut output = Vec::new();

        let error = run_cli(
            ["--suite", "lab-smoke", "--scenario", "missing", "--once"]
                .into_iter()
                .map(String::from),
            &registry,
            &mut output,
        )
        .expect_err("unknown scenario should fail");

        let message = error.to_string();
        assert!(message.contains("lab-smoke/missing"));
        assert!(message.contains("lab-smoke/lab-smoke-editing"));
    }

    #[test]
    fn list_command_prints_smoke_descriptor_with_tags_and_viewport() {
        let registry = LabScenarioRegistry::built_in();
        let mut output = Vec::new();

        run_cli(
            ["--list"].into_iter().map(String::from),
            &registry,
            &mut output,
        )
        .expect("--list should succeed");

        let text = String::from_utf8(output).expect("utf8 list output");
        assert!(text.contains("lab-smoke/lab-smoke-editing"));
        assert!(text.contains("standard 120x34"));
        assert!(text.contains("w038-phase-1"));
    }

    #[test]
    fn mockup_flag_uses_frankentui_firehorse_renderer_without_replacing_contract_output() {
        let registry = LabScenarioRegistry::built_in();
        let mut contract_output = Vec::new();
        let mut mockup_output = Vec::new();

        run_cli(
            [
                "--suite",
                "firehorse",
                "--scenario",
                "firehorse-editing-lens-standard",
                "--viewport",
                "standard",
                "--once",
            ]
            .into_iter()
            .map(String::from),
            &registry,
            &mut contract_output,
        )
        .expect("contract Fire Horse render should succeed");

        run_cli(
            [
                "--suite",
                "firehorse",
                "--scenario",
                "firehorse-editing-lens-standard",
                "--viewport",
                "standard",
                "--once",
                "--mockup",
            ]
            .into_iter()
            .map(String::from),
            &registry,
            &mut mockup_output,
        )
        .expect("mockup Fire Horse render should succeed");

        let contract = String::from_utf8(contract_output).expect("utf8 contract output");
        let mockup = String::from_utf8(mockup_output).expect("utf8 mockup output");
        assert!(contract.contains("Fire Horse Editing Lens"));
        assert!(contract.contains("Identity Rail"));
        assert!(mockup.contains("OXIDE FIRE HORSE UX LAB"));
        assert!(mockup.contains("┌"));
        assert_ne!(contract, mockup);
    }

    #[test]
    fn mockup_ansi_flag_emits_terminal_stream() {
        let registry = LabScenarioRegistry::built_in();
        let mut output = Vec::new();

        run_cli(
            [
                "--suite",
                "firehorse",
                "--scenario",
                "firehorse-editing-lens-standard",
                "--viewport",
                "studio",
                "--once",
                "--mockup",
                "--ansi",
            ]
            .into_iter()
            .map(String::from),
            &registry,
            &mut output,
        )
        .expect("mockup ansi render should succeed");

        let text = String::from_utf8(output).expect("utf8 ansi stream");
        assert!(text.contains("\u{1b}["));
        assert!(text.contains("OXIDE FIRE HORSE UX LAB"));
    }
}
