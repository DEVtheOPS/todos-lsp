use std::ffi::OsString;
use std::path::PathBuf;

use clap::{ArgAction, Parser, Subcommand, ValueEnum};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Scan(ScanArgs),
    Serve,
}

#[derive(Debug, Clone, Parser)]
#[command(
    name = "todos-lsp",
    about = "Search for TODO comments in code.",
    version
)]
struct Cli {
    #[command(subcommand)]
    subcommand: Option<Subcommands>,

    #[command(flatten)]
    scan: ScanArgs,
}

#[derive(Debug, Clone, Subcommand)]
enum Subcommands {
    /// Run the LSP server over stdio.
    Serve,
}

#[derive(Debug, Clone, Parser, PartialEq, Eq)]
pub struct ScanArgs {
    #[arg(long, default_value = "false", action = ArgAction::SetTrue)]
    pub blame: bool,

    #[arg(long, short = 'c', default_value = "UTF-8")]
    pub charset: String,

    #[arg(long = "exclude")]
    pub excludes: Vec<String>,

    #[arg(long = "exclude-dir")]
    pub exclude_dirs: Vec<String>,

    #[arg(long, default_value = "false", action = ArgAction::SetTrue)]
    pub exclude_hidden: bool,

    #[arg(long, default_value = "false", action = ArgAction::SetTrue)]
    pub follow: bool,

    #[arg(long = "ignore-file-name", default_values_t = vec![String::from(".gitignore"), String::from(".todosignore")])]
    pub ignore_file_names: Vec<String>,

    #[arg(long, default_value = "false", action = ArgAction::SetTrue)]
    pub include_vcs: bool,

    #[arg(long, default_value = "false", action = ArgAction::SetTrue)]
    pub include_generated: bool,

    #[arg(long, default_value = "false", action = ArgAction::SetTrue)]
    pub include_vendored: bool,

    #[arg(long, short = 'l')]
    pub labels: Vec<String>,

    #[arg(long, default_value = "false", action = ArgAction::SetTrue)]
    pub no_error_on_unsupported: bool,

    #[arg(long, short = 'o', value_enum)]
    pub output: Option<OutputArg>,

    #[arg(long = "todo-types")]
    pub todo_types: Option<String>,

    #[arg(value_name = "PATH")]
    pub paths: Vec<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputArg {
    Default,
    Github,
    Json,
}

pub fn parse_from<I>(args: I) -> Result<Command, i32>
where
    I: IntoIterator,
    I::Item: Into<OsString> + Clone,
{
    let cli = match Cli::try_parse_from(args) {
        Ok(cli) => cli,
        Err(error) => {
            let exit_code = error.exit_code();
            let _ = error.print();
            return Err(exit_code);
        }
    };

    match cli.subcommand {
        Some(Subcommands::Serve) => Ok(Command::Serve),
        None => Ok(Command::Scan(cli.scan.with_default_path())),
    }
}

impl ScanArgs {
    pub fn with_default_path(mut self) -> Self {
        if self.paths.is_empty() {
            self.paths.push(PathBuf::from("."));
        }

        if self.output.is_none() {
            self.output = Some(OutputArg::Default);
        }

        self
    }
}
