mod shell;

use std::io;

use ftui::prelude::{App, ScreenMode};

use shell::ShellModel;

fn main() -> io::Result<()> {
    App::new(ShellModel::new())
        .screen_mode(ScreenMode::AltScreen)
        .run()
}
