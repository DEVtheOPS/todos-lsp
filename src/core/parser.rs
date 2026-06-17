use std::path::PathBuf;

use crate::core::config::RuntimeConfig;
use crate::core::errors::TodoError;
use crate::core::model::{path_to_uri, Finding, SourceKind, TextScanRequest};

pub fn parse_text(request: &TextScanRequest) -> Result<Vec<Finding>, TodoError> {
    let path = request
        .workspace_root
        .as_ref()
        .and_then(|root| {
            url::Url::parse(&request.uri)
                .ok()
                .and_then(|uri| uri.to_file_path().ok())
                .map(|path| path.strip_prefix(root).unwrap_or(&path).to_path_buf())
        })
        .unwrap_or_else(|| {
            request
                .language_hint
                .clone()
                .map(PathBuf::from)
                .unwrap_or_default()
        });

    Ok(parse_contents(
        &request.text,
        request.config.clone(),
        path,
        request.uri.clone(),
        SourceKind::Overlay,
    ))
}

pub fn parse_disk_text(
    display_path: PathBuf,
    absolute_path: PathBuf,
    text: &str,
    config: &RuntimeConfig,
) -> Vec<Finding> {
    parse_contents(
        text,
        config.clone(),
        display_path,
        path_to_uri(&absolute_path),
        SourceKind::Disk,
    )
}

fn parse_contents(
    text: &str,
    config: RuntimeConfig,
    path: PathBuf,
    uri: String,
    source: SourceKind,
) -> Vec<Finding> {
    let style = comment_style_for_path(&path);
    let todo_types = config.todo_types;

    let mut findings = Vec::new();
    let mut block_start_line: Option<u32> = None;

    for (index, line) in text.lines().enumerate() {
        if let Some(start_line) = block_start_line {
            let comment = line
                .split_once("*/")
                .map(|(comment, _)| comment)
                .unwrap_or(line);
            if let Some(parsed) = parse_comment(comment, &todo_types) {
                let column = line.find(|ch: char| !ch.is_whitespace()).unwrap_or(0);
                findings.push(
                    Finding::new(
                        path.clone(),
                        uri.clone(),
                        index as u32,
                        column as u32,
                        comment.trim().to_string(),
                        parsed.todo_type,
                        parsed.label,
                        parsed.message,
                        source.clone(),
                    )
                    .with_comment_line(start_line),
                );
            }
            if line.contains("*/") {
                block_start_line = None;
            }
            continue;
        }

        if matches!(style, CommentStyle::SlashSlash) && starts_multiline_block(line) {
            block_start_line = Some(index as u32);
        }

        let Some((column, comment)) = extract_comment(line, style) else {
            continue;
        };
        let Some(parsed) = parse_comment(comment, &todo_types) else {
            continue;
        };
        findings.push(Finding::new(
            path.clone(),
            uri.clone(),
            index as u32,
            column as u32,
            raw_text_for(style, line, column, comment),
            parsed.todo_type,
            parsed.label,
            parsed.message,
            source.clone(),
        ));
    }

    findings
}

fn starts_multiline_block(line: &str) -> bool {
    line.contains("/*") && !line.contains("*/")
}

fn raw_text_for(style: CommentStyle, line: &str, column: usize, comment: &str) -> String {
    match style {
        CommentStyle::Html => format!("<!--{}-->", comment),
        CommentStyle::SlashSlash if line[column..].starts_with("/*") => {
            let end = line[column + 2..]
                .find("*/")
                .map(|value| column + 4 + value)
                .unwrap_or(line.len());
            line[column..end].trim().to_string()
        }
        _ => line[column..].trim().to_string(),
    }
}

#[derive(Clone, Copy)]
enum CommentStyle {
    SlashSlash,
    DashDash,
    Hash,
    Semicolon,
    Html,
}

struct ParsedComment {
    todo_type: String,
    label: Option<String>,
    message: Option<String>,
}

fn comment_style_for_path(path: &std::path::Path) -> CommentStyle {
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    match path.extension().and_then(|ext| ext.to_str()) {
        Some(
            "rs" | "js" | "ts" | "tsx" | "jsx" | "json5" | "svelte" | "go" | "java" | "kt" | "c"
            | "dart" | "h",
        ) => CommentStyle::SlashSlash,
        Some("lua") => CommentStyle::DashDash,
        Some(
            "py" | "sh" | "bash" | "yaml" | "yml" | "toml" | "md" | "tf" | "hcl" | "ex" | "exs"
            | "rb" | "graphql" | "gql" | "graphqls",
        ) => CommentStyle::Hash,
        Some("ini" | "cfg" | "conf") => CommentStyle::Semicolon,
        _ if matches!(
            name,
            "Makefile" | ".editorconfig" | "Dockerfile" | "Containerfile" | ".gitignore"
        ) =>
        {
            CommentStyle::Hash
        }
        _ if matches!(name, ".gitconfig") => CommentStyle::Semicolon,
        Some("erb") => CommentStyle::Html,
        _ => CommentStyle::Html,
    }
}

fn extract_comment(line: &str, style: CommentStyle) -> Option<(usize, &str)> {
    match style {
        CommentStyle::SlashSlash => extract_slash_comment(line),
        CommentStyle::DashDash => find_marker_outside_quotes(line, "--"),
        CommentStyle::Hash => find_marker_outside_quotes(line, "#"),
        CommentStyle::Semicolon => find_marker_outside_quotes(line, ";"),
        CommentStyle::Html => line.find("<!--").map(|index| {
            let end = line[index + 4..]
                .find("-->")
                .map(|value| index + 4 + value)
                .unwrap_or(line.len());
            (index, &line[index + 4..end])
        }),
    }
}

fn extract_slash_comment(line: &str) -> Option<(usize, &str)> {
    if let Some(comment) = find_marker_outside_quotes(line, "//") {
        return Some(comment);
    }

    line.find("/*").map(|index| {
        let end = line[index + 2..]
            .find("*/")
            .map(|value| index + 2 + value)
            .unwrap_or(line.len());
        (index, &line[index + 2..end])
    })
}

fn find_marker_outside_quotes<'a>(line: &'a str, marker: &str) -> Option<(usize, &'a str)> {
    let mut in_single = false;
    let mut in_double = false;
    let chars = line.char_indices();
    for (index, ch) in chars {
        match ch {
            '\'' if !in_double => in_single = !in_single,
            '"' if !in_single => in_double = !in_double,
            _ => {}
        }
        if !in_single && !in_double && line[index..].starts_with(marker) {
            return Some((index, &line[index + marker.len()..]));
        }
    }
    None
}

fn parse_comment(comment: &str, todo_types: &[String]) -> Option<ParsedComment> {
    let trimmed = comment
        .trim_start_matches(|ch: char| {
            ch.is_whitespace() || matches!(ch, '/' | '*' | '#' | ';' | '!')
        })
        .trim_start();
    let trimmed = trimmed.strip_prefix('@').unwrap_or(trimmed);

    let mut sorted = todo_types.to_vec();
    sorted.sort_by_key(|value| std::cmp::Reverse(value.len()));

    for todo_type in sorted {
        if !trimmed.starts_with(&todo_type) {
            continue;
        }
        let rest = &trimmed[todo_type.len()..];
        let next = rest.chars().next();
        if matches!(next, Some(ch) if !(ch.is_whitespace() || ch == ':' || ch == '(')) {
            continue;
        }

        let mut remaining = rest.trim_start();
        let label = if remaining.starts_with('(') {
            let close = remaining.find(')')?;
            let label = remaining[1..close].trim().to_string();
            remaining = remaining[close + 1..].trim_start();
            if label.is_empty() {
                None
            } else {
                Some(label)
            }
        } else {
            None
        };

        let message = if remaining.is_empty() {
            None
        } else if let Some(message) = remaining.strip_prefix(':') {
            let trimmed = message.trim().to_string();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        } else {
            return None;
        };

        return Some(ParsedComment {
            todo_type,
            label,
            message,
        });
    }

    None
}
