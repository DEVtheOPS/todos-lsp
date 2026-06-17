use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::core::config::RuntimeConfig;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum SourceKind {
    Disk,
    Overlay,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BlameInfo {
    pub author: String,
    pub email: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Finding {
    pub id: String,
    pub path: PathBuf,
    pub uri: String,
    pub range: Range,
    pub raw_text: String,
    pub todo_type: String,
    pub label: Option<String>,
    pub message: Option<String>,
    pub comment_line: u32,
    pub source: SourceKind,
    pub blame: Option<BlameInfo>,
}

#[derive(Debug, Clone)]
pub struct DiskScanRequest {
    pub root: PathBuf,
    pub paths: Vec<PathBuf>,
    pub config: RuntimeConfig,
}

#[derive(Debug, Clone)]
pub struct TextScanRequest {
    pub uri: String,
    pub text: String,
    pub language_hint: Option<String>,
    pub config: RuntimeConfig,
    pub workspace_root: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CliJsonFinding {
    pub path: String,
    pub r#type: String,
    pub text: String,
    pub label: String,
    pub message: String,
    pub line: u32,
    pub comment_line: u32,
}

impl Finding {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        path: PathBuf,
        uri: String,
        line: u32,
        column: u32,
        raw_text: String,
        todo_type: String,
        label: Option<String>,
        message: Option<String>,
        source: SourceKind,
    ) -> Self {
        let id = format!(
            "{}:{}:{}:{}:{}",
            uri,
            line,
            column,
            todo_type,
            message.clone().unwrap_or_default()
        );
        let end_character = column + raw_text.chars().count() as u32;

        Self {
            id,
            path,
            uri,
            range: Range {
                start: Position {
                    line,
                    character: column,
                },
                end: Position {
                    line,
                    character: end_character,
                },
            },
            raw_text,
            todo_type,
            label,
            message,
            comment_line: line,
            source,
            blame: None,
        }
    }

    pub fn display_path(&self) -> String {
        self.path.to_string_lossy().to_string()
    }

    pub fn as_cli_json(&self) -> CliJsonFinding {
        CliJsonFinding {
            path: self.display_path(),
            r#type: self.todo_type.clone(),
            text: self.raw_text.clone(),
            label: self.label.clone().unwrap_or_default(),
            message: self.message.clone().unwrap_or_default(),
            line: self.range.start.line + 1,
            comment_line: self.comment_line + 1,
        }
    }

    pub fn with_comment_line(mut self, comment_line: u32) -> Self {
        self.comment_line = comment_line;
        self
    }

    pub fn with_blame(mut self, blame: BlameInfo) -> Self {
        self.blame = Some(blame);
        self
    }
}

pub fn path_to_uri(path: &Path) -> String {
    url::Url::from_file_path(path)
        .expect("valid path URI")
        .to_string()
}
