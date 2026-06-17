use std::path::{Path, PathBuf};
use std::process::Command;

use crate::core::errors::TodoError;
use crate::core::model::{BlameInfo, Finding};

pub fn enrich_with_blame(
    findings: Vec<Finding>,
    executable: &Path,
) -> Result<Vec<Finding>, TodoError> {
    findings
        .into_iter()
        .map(|finding| {
            let path = url::Url::parse(&finding.uri)
                .ok()
                .and_then(|uri| uri.to_file_path().ok())
                .unwrap_or_else(|| finding.path.clone());
            let blame = run_blame(&path, finding.range.start.line + 1, executable)?;
            Ok(finding.with_blame(blame))
        })
        .collect()
}

fn run_blame(path: &Path, line: u32, executable: &Path) -> Result<BlameInfo, TodoError> {
    let Some(parent) = path.parent() else {
        return Err(TodoError::MissingRuntimeDependency(String::from(
            "git blame requires a repository path",
        )));
    };

    let line_range = format!("{line},{line}");
    let filename = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    let output = Command::new(executable)
        .args(blame_args(&line_range, filename))
        .current_dir(parent)
        .output()
        .map_err(|_| {
            TodoError::MissingRuntimeDependency(format!(
                "{} is required for blame",
                executable.display()
            ))
        })?;

    if !output.status.success() {
        return Err(TodoError::MissingRuntimeDependency(String::from(
            "git blame requires a Git repository",
        )));
    }

    parse_porcelain(&String::from_utf8_lossy(&output.stdout))
}

fn blame_args(line_range: &str, filename: &str) -> Vec<String> {
    vec![
        String::from("blame"),
        String::from("--porcelain"),
        String::from("-L"),
        line_range.to_string(),
        String::from("--"),
        filename.to_string(),
    ]
}

fn parse_porcelain(output: &str) -> Result<BlameInfo, TodoError> {
    let mut author = None;
    let mut email = None;

    for line in output.lines() {
        if let Some(value) = line.strip_prefix("author ") {
            author = Some(value.to_string());
        }
        if let Some(value) = line.strip_prefix("author-mail <") {
            email = Some(value.trim_end_matches('>').to_string());
        }
    }

    match (author, email) {
        (Some(author), Some(email)) => Ok(BlameInfo { author, email }),
        _ => Err(TodoError::Io(String::from(
            "unable to parse git blame output",
        ))),
    }
}

pub fn git_executable() -> PathBuf {
    PathBuf::from("git")
}

#[cfg(test)]
mod tests {
    use super::{blame_args, parse_porcelain};

    #[test]
    fn parses_porcelain_output() {
        let blame = parse_porcelain(
            "deadbeef 1 1 1\nauthor Example Dev\nauthor-mail <dev@example.com>\nsummary add todo\n",
        )
        .expect("porcelain parsed");

        assert_eq!(blame.author, "Example Dev");
        assert_eq!(blame.email, "dev@example.com");
    }

    #[test]
    fn rejects_incomplete_porcelain_output() {
        let error = parse_porcelain("deadbeef 1 1 1\nauthor Example Dev\n")
            .expect_err("porcelain should fail");
        assert!(error.to_string().contains("unable to parse"));
    }

    #[test]
    fn blame_args_delimit_revisions_from_paths() {
        let args = blame_args("1,1", "--looks-like-revision.rs");
        assert!(args.iter().any(|arg| arg == "--"));
        assert_eq!(
            args.last().map(String::as_str),
            Some("--looks-like-revision.rs")
        );
    }
}
