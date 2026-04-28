use std::env;
use std::io;
use std::process::ExitCode;

use ox_ide::vt::run_cli;

fn main() -> ExitCode {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    match run_cli(env::args().skip(1), &mut handle) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(2)
        }
    }
}
