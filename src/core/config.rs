use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

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
