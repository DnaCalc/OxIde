use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

use oxide_guilab::{GuiLabError, GuiScenarioRegistry, run_cli};

fn main() -> ExitCode {
    let args = env::args().skip(1).collect::<Vec<_>>();
    match run_cli(args, repo_root()) {
        Ok(output) => {
            print!("{output}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(error_exit_code(&error))
        }
    }
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

fn error_exit_code(error: &GuiLabError) -> u8 {
    match error {
        GuiLabError::Usage { .. } => 2,
        GuiLabError::UnknownScenario { .. } => 2,
        GuiLabError::DuplicateScenarioId { .. } => 3,
        GuiLabError::ProjectOpen(_) => 4,
    }
}

#[allow(dead_code)]
fn _registry_type_anchor(_: &GuiScenarioRegistry) {}
