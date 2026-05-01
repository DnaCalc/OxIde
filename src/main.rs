use std::{env, io, path::PathBuf};

use ftui::prelude::{App, ScreenMode};

use ox_ide::shell::{FireHorseDesignModel, ShellModel};

/// CLI shape is intentionally tiny for W030-W035: an optional project path
/// and an optional `--dev-scenes` flag that re-enables the F2/F3/F4
/// scene-flip affordances (uxpass D6). The flag can appear in any position.
fn main() -> io::Result<()> {
    let mut dev_scenes = false;
    let mut firehorse_design = false;
    let mut firehorse_screen: Option<String> = None;
    let mut project_path: Option<PathBuf> = None;
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--dev-scenes" => dev_scenes = true,
            "--firehorse-design" | "--firehorse-skeleton" => firehorse_design = true,
            "--firehorse-screen" => {
                firehorse_design = true;
                firehorse_screen = args.next();
            }
            other if other.starts_with("--firehorse-screen=") => {
                firehorse_design = true;
                firehorse_screen = Some(other["--firehorse-screen=".len()..].to_string());
            }
            other if project_path.is_none() => project_path = Some(PathBuf::from(other)),
            _ => {
                // Silently ignore extra positional args for now; a real
                // argument parser lands with the command model (W090).
            }
        }
    }

    if firehorse_design {
        let model = FireHorseDesignModel::new(firehorse_screen.as_deref())
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error.to_string()))?;
        App::new(model).screen_mode(ScreenMode::AltScreen).run()
    } else {
        App::new(ShellModel::with_dev_scenes(project_path, dev_scenes))
            .screen_mode(ScreenMode::AltScreen)
            .run()
    }
}
