use ftui::{
    KeyEventKind,
    prelude::{Cmd, Event, Frame, KeyCode, KeyEvent, Model},
};

use super::uxlab::firehorse::{mockup, projection_for_scenario_id};
use super::uxlab::{LabRenderError, ViewportClass};

pub const FIRE_HORSE_DESIGN_SCREEN_IDS: [&str; 7] = [
    "firehorse-launchpad-standard",
    "firehorse-editing-lens-standard",
    "firehorse-command-lens-standard",
    "firehorse-run-lane-standard",
    "firehorse-debug-cockpit-standard",
    "firehorse-console-fit-light",
    "firehorse-focus-compact",
];

const DEFAULT_SCREEN_ID: &str = "firehorse-editing-lens-standard";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FireHorseDesignMsg {
    Quit,
    NextScreen,
    PreviousScreen,
    Noop,
}

impl From<Event> for FireHorseDesignMsg {
    fn from(event: Event) -> Self {
        match event {
            Event::Key(key) if !is_actionable_key(key) => Self::Noop,
            Event::Key(key) if is_quit_key(key) => Self::Quit,
            Event::Key(key) if is_next_screen_key(key) => Self::NextScreen,
            Event::Key(key) if is_previous_screen_key(key) => Self::PreviousScreen,
            _ => Self::Noop,
        }
    }
}

pub struct FireHorseDesignModel {
    selected: usize,
}

impl FireHorseDesignModel {
    pub fn new(screen_id: Option<&str>) -> Result<Self, LabRenderError> {
        let selected = match screen_id.unwrap_or(DEFAULT_SCREEN_ID) {
            id if FIRE_HORSE_DESIGN_SCREEN_IDS.contains(&id) => FIRE_HORSE_DESIGN_SCREEN_IDS
                .iter()
                .position(|candidate| *candidate == id)
                .expect("screen id was checked"),
            id => {
                return Err(LabRenderError::UnknownScenario {
                    suite: "firehorse",
                    id: id.to_string(),
                });
            }
        };

        Ok(Self { selected })
    }

    pub fn selected_screen_id(&self) -> &'static str {
        FIRE_HORSE_DESIGN_SCREEN_IDS[self.selected]
    }

    fn cycle(&mut self, delta: isize) {
        let len = FIRE_HORSE_DESIGN_SCREEN_IDS.len() as isize;
        self.selected = (self.selected as isize + delta).rem_euclid(len) as usize;
    }
}

impl Model for FireHorseDesignModel {
    type Message = FireHorseDesignMsg;

    fn update(&mut self, msg: Self::Message) -> Cmd<Self::Message> {
        match msg {
            FireHorseDesignMsg::Quit => Cmd::quit(),
            FireHorseDesignMsg::NextScreen => {
                self.cycle(1);
                Cmd::none()
            }
            FireHorseDesignMsg::PreviousScreen => {
                self.cycle(-1);
                Cmd::none()
            }
            FireHorseDesignMsg::Noop => Cmd::none(),
        }
    }

    fn view(&self, frame: &mut Frame) {
        let viewport = viewport_for_width(frame.width());
        let (_, projection) = projection_for_scenario_id(self.selected_screen_id())
            .expect("selected design screen must have a projection");
        mockup::render_mockup_into_frame(frame, viewport, &projection);
    }
}

fn viewport_for_width(width: u16) -> ViewportClass {
    if width >= ViewportClass::Studio.wtd_size().width {
        ViewportClass::Studio
    } else if width >= ViewportClass::FirstClass.wtd_size().width {
        ViewportClass::FirstClass
    } else if width >= ViewportClass::Standard.wtd_size().width {
        ViewportClass::Standard
    } else {
        ViewportClass::Compact
    }
}

fn is_actionable_key(key: KeyEvent) -> bool {
    matches!(key.kind, KeyEventKind::Press)
}

fn is_quit_key(key: KeyEvent) -> bool {
    key.is_char('q')
}

fn is_next_screen_key(key: KeyEvent) -> bool {
    key.is_char('j')
        || matches!(
            key.code,
            KeyCode::Tab | KeyCode::Right | KeyCode::Down | KeyCode::PageDown
        )
}

fn is_previous_screen_key(key: KeyEvent) -> bool {
    key.is_char('k') || matches!(key.code, KeyCode::Left | KeyCode::Up | KeyCode::PageUp)
}

#[cfg(test)]
mod tests {
    use ftui::{GraphemePool, prelude::Model};

    use super::*;
    use crate::shell::uxlab::frame_to_text;

    fn render_text(screen_id: &str, width: u16, height: u16) -> String {
        let model = FireHorseDesignModel::new(Some(screen_id)).expect("design screen");
        let mut pool = GraphemePool::new();
        let mut frame = Frame::new(width, height, &mut pool);
        model.view(&mut frame);
        frame_to_text(&frame)
    }

    #[test]
    fn default_screen_is_editing_lens() {
        let model = FireHorseDesignModel::new(None).expect("default model");

        assert_eq!(
            model.selected_screen_id(),
            "firehorse-editing-lens-standard"
        );
    }

    #[test]
    fn all_designed_fire_horse_screens_render_in_the_skeleton() {
        for screen_id in FIRE_HORSE_DESIGN_SCREEN_IDS {
            let text = render_text(screen_id, 190, 48);

            assert!(
                text.contains("OxIde") || text.contains("OXIDE"),
                "screen {screen_id} should render OxIde identity, got {text:?}"
            );
            assert!(
                text.contains("FIRE HORSE")
                    || text.contains("Fire Horse")
                    || text.contains("Command Lens")
                    || text.contains("Console Fit"),
                "screen {screen_id} should render Fire Horse design surface, got {text:?}"
            );
        }
    }

    #[test]
    fn screen_selection_cycles_without_leaving_the_design_set() {
        let mut model =
            FireHorseDesignModel::new(Some("firehorse-focus-compact")).expect("compact focus");

        model.update(FireHorseDesignMsg::NextScreen);

        assert_eq!(model.selected_screen_id(), "firehorse-launchpad-standard");
    }
}
