use std::env;
use std::io;
use std::process::ExitCode;

use ox_ide::shell::uxlab::{LabScenarioRegistry, run_cli};

fn main() -> ExitCode {
    let registry = LabScenarioRegistry::built_in();
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    match run_cli(env::args().skip(1), &registry, &mut handle) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(2)
        }
    }
}
