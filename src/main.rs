use std::{env, io, path::PathBuf};

use ftui::prelude::{App, ScreenMode};

use ox_ide::shell::ShellModel;

/// CLI shape is intentionally tiny for W030-W035: an optional project path
/// and an optional `--dev-scenes` flag that re-enables the F2/F3/F4
/// scene-flip affordances (uxpass D6). The flag can appear in any position.
fn main() -> io::Result<()> {
    let mut dev_scenes = false;
    let mut project_path: Option<PathBuf> = None;

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "--dev-scenes" => dev_scenes = true,
            other if project_path.is_none() => project_path = Some(PathBuf::from(other)),
            _ => {
                // Silently ignore extra positional args for now; a real
                // argument parser lands with the command model (W090).
            }
        }
    }

    App::new(ShellModel::with_dev_scenes(project_path, dev_scenes))
        .screen_mode(ScreenMode::AltScreen)
        .run()
}
