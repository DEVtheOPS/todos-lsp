use std::ffi::OsString;

use crate::cli::{args, commands};
use crate::core::errors::TodoError;

pub fn run<I>(args: I) -> std::process::ExitCode
where
    I: IntoIterator,
    I::Item: Into<OsString> + Clone,
{
    match args::parse_from(args) {
        Ok(args::Command::Serve) => match crate::lsp::server::serve_stdio() {
            Ok(()) => std::process::ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("{error}");
                std::process::ExitCode::from(exit_code_for_error(&error))
            }
        },
        Ok(args::Command::Scan(scan_args)) => {
            match commands::run(scan_args, &mut std::io::stdout(), &mut std::io::stderr()) {
                Ok(code) => std::process::ExitCode::from(code as u8),
                Err(error) => {
                    eprintln!("{error}");
                    std::process::ExitCode::from(exit_code_for_error(&error))
                }
            }
        }
        Err(exit_code) => std::process::ExitCode::from(exit_code as u8),
    }
}

fn exit_code_for_error(error: &TodoError) -> u8 {
    match error {
        TodoError::UnsupportedInput(_) => 3,
        TodoError::InvalidConfig(_) => 2,
        TodoError::MissingRuntimeDependency(_) | TodoError::Io(_) | TodoError::Serialization(_) => {
            1
        }
    }
}
