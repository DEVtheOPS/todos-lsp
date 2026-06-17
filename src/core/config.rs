use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::cli::args::{OutputArg, ScanArgs};
use crate::core::errors::TodoError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutputFormat {
    Default,
    Github,
    Json,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub todo_types: Vec<String>,
    pub output: OutputFormat,
    pub blame: bool,
    pub excludes: Vec<String>,
    pub exclude_dirs: Vec<String>,
    pub exclude_hidden: bool,
    pub follow_symlinks: bool,
    pub ignore_file_names: Vec<String>,
    pub include_vcs: bool,
    pub include_generated: bool,
    pub include_vendored: bool,
    pub labels: Vec<String>,
    pub no_error_on_unsupported: bool,
    pub charset: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FileConfig {
    pub todo_types: Option<Vec<String>>,
    pub output: Option<OutputFormat>,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            todo_types: default_todo_types(),
            output: OutputFormat::Default,
            blame: false,
            excludes: Vec::new(),
            exclude_dirs: Vec::new(),
            exclude_hidden: false,
            follow_symlinks: false,
            ignore_file_names: vec![String::from(".gitignore"), String::from(".todosignore")],
            include_vcs: false,
            include_generated: false,
            include_vendored: false,
            labels: Vec::new(),
            no_error_on_unsupported: false,
            charset: String::from("UTF-8"),
        }
    }
}

impl RuntimeConfig {
    pub fn from_scan_args(args: &ScanArgs) -> Result<Self, TodoError> {
        let todo_types = match &args.todo_types {
            Some(value) => value
                .split(',')
                .map(str::trim)
                .filter(|item| !item.is_empty())
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>(),
            None => default_todo_types(),
        };

        let config = Self {
            todo_types,
            output: match args.output.unwrap_or(OutputArg::Default) {
                OutputArg::Default => OutputFormat::Default,
                OutputArg::Github => OutputFormat::Github,
                OutputArg::Json => OutputFormat::Json,
            },
            blame: args.blame,
            excludes: args.excludes.clone(),
            exclude_dirs: args.exclude_dirs.clone(),
            exclude_hidden: args.exclude_hidden,
            follow_symlinks: args.follow,
            ignore_file_names: args.ignore_file_names.clone(),
            include_vcs: args.include_vcs,
            include_generated: args.include_generated,
            include_vendored: args.include_vendored,
            labels: args.labels.clone(),
            no_error_on_unsupported: args.no_error_on_unsupported,
            charset: args.charset.clone(),
        };

        config.validate()
    }

    pub fn validate(self) -> Result<Self, TodoError> {
        if self.todo_types.is_empty() {
            return Err(TodoError::InvalidConfig(String::from(
                "at least one todo type is required",
            )));
        }

        if self.charset != "UTF-8" && self.charset != "detect" {
            return Err(TodoError::InvalidConfig(format!(
                "unsupported charset: {}",
                self.charset
            )));
        }

        Ok(self)
    }

    pub fn from_file(path: &Path) -> Result<Self, TodoError> {
        let contents = std::fs::read_to_string(path)?;
        let file_config: FileConfig = serde_json::from_str(&contents)
            .map_err(|error| TodoError::InvalidConfig(error.to_string()))?;

        let mut config = Self::default();
        if let Some(todo_types) = file_config.todo_types {
            config.todo_types = todo_types;
        }
        if let Some(output) = file_config.output {
            config.output = output;
        }
        config.validate()
    }
}

pub fn default_todo_types() -> Vec<String> {
    [
        "TODO", "Todo", "todo", "FIXME", "Fixme", "fixme", "FIXIT", "Fixit", "fixit", "BUG", "Bug",
        "bug", "HACK", "Hack", "hack", "ISSUE", "Issue", "issue", "WARN", "Warn", "warn",
        "WARNING", "Warning", "warning", "XXX", "COMBAK",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}

pub fn resolve_workspace_root(path: &Path) -> PathBuf {
    if path.is_dir() {
        path.to_path_buf()
    } else {
        path.parent().unwrap_or(path).to_path_buf()
    }
}
