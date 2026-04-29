use std::env;
use std::io;
use std::process::ExitCode;

use ftui::prelude::{App, ScreenMode};
use ox_ide::shell::uxlab::audit::controller::AuditLabModel;
use ox_ide::shell::uxlab::{
    LabCliSelection, LabRunError, LabScenarioRegistry, run_cli_with_outcome,
};

fn main() -> ExitCode {
    let args = env::args().skip(1).collect::<Vec<_>>();
    match LabCliSelection::parse(args.clone()) {
        Ok(selection) if selection.audit && selection.mode.is_none() && !selection.json => {
            return match AuditLabModel::new(&selection) {
                Ok(model) => match App::new(model).screen_mode(ScreenMode::AltScreen).run() {
                    Ok(()) => ExitCode::SUCCESS,
                    Err(error) => {
                        eprintln!("{error}");
                        ExitCode::from(3)
                    }
                },
                Err(error) => {
                    eprintln!("{error}");
                    ExitCode::from(error_exit_code(&error))
                }
            };
        }
        Ok(_) => {}
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::from(error_exit_code(&error));
        }
    }

    let registry = LabScenarioRegistry::built_in();
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    match run_cli_with_outcome(args, &registry, &mut handle) {
        Ok(outcome) => ExitCode::from(outcome.exit_code()),
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(error_exit_code(&error))
        }
    }
}

fn error_exit_code(error: &LabRunError) -> u8 {
    match error {
        LabRunError::Render(_) => 3,
        _ => 2,
    }
}
