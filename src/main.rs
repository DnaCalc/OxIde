mod shell;

use std::{env, io, path::PathBuf};

use ftui::prelude::{App, ScreenMode};

use shell::ShellModel;

fn main() -> io::Result<()> {
    let project_path = env::args().nth(1).map(PathBuf::from);

    App::new(ShellModel::new(project_path))
        .screen_mode(ScreenMode::AltScreen)
        .run()
}
