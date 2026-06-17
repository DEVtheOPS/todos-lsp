use crate::core::blame;
use crate::core::discovery;
use crate::core::errors::TodoError;
use crate::core::model::{DiskScanRequest, Finding, TextScanRequest};
use crate::core::parser;

pub fn scan_disk(request: &DiskScanRequest) -> Result<Vec<Finding>, TodoError> {
    let paths = discovery::discover_inputs(&request.root, &request.paths, &request.config)?;
    let mut findings = Vec::new();

    for path in paths {
        let contents = read_path_text(&path, &request.config)?;
        let relative_path = path
            .strip_prefix(&request.root)
            .map(|value| value.to_path_buf())
            .unwrap_or_else(|_| path.clone());
        let parsed = parser::parse_disk_text(relative_path, path, &contents, &request.config);
        findings.extend(parsed);
    }

    if !request.config.labels.is_empty() {
        findings.retain(|finding| {
            request.config.labels.iter().any(|label| {
                finding.label.as_ref().is_some_and(|value| {
                    glob::Pattern::new(label)
                        .map(|pattern| pattern.matches(value))
                        .unwrap_or(false)
                })
            })
        });
    }

    if request.config.blame {
        blame::enrich_with_blame(findings, &blame::git_executable())
    } else {
        Ok(findings)
    }
}

pub fn scan_text(request: &TextScanRequest) -> Result<Vec<Finding>, TodoError> {
    crate::core::parser::parse_text(request)
}

fn read_path_text(
    path: &std::path::Path,
    config: &crate::core::config::RuntimeConfig,
) -> Result<String, TodoError> {
    let bytes = std::fs::read(path)?;
    match config.charset.as_str() {
        "detect" => Ok(String::from_utf8_lossy(&bytes).into_owned()),
        "UTF-8" | "ISO-8859-1" => {
            String::from_utf8(bytes).map_err(|error| TodoError::Io(error.to_string()))
        }
        other => Err(TodoError::InvalidConfig(format!(
            "unsupported charset: {other}"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::{scan_disk, scan_text};
    use crate::core::config::RuntimeConfig;
    use crate::core::model::{path_to_uri, DiskScanRequest, TextScanRequest};

    #[test]
    fn filters_findings_by_label() {
        let dir = tempdir().expect("tempdir");
        let file = dir.path().join("sample.rs");
        std::fs::write(
            &file,
            "// TODO(owner): keep this\n// TODO(other): drop this\n",
        )
        .expect("fixture written");

        let config = RuntimeConfig {
            labels: vec![String::from("owner")],
            ..RuntimeConfig::default()
        };
        let findings = scan_disk(&DiskScanRequest {
            root: dir.path().to_path_buf(),
            paths: vec![std::path::PathBuf::from(".")],
            config,
        })
        .expect("scan succeeds");

        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].label.as_deref(), Some("owner"));
    }

    #[test]
    fn scans_text_requests_with_language_hint() {
        let dir = tempdir().expect("tempdir");
        let file = dir.path().join("sample.rs");
        let findings = scan_text(&TextScanRequest {
            uri: path_to_uri(&file),
            text: String::from("// TODO: from text\n"),
            language_hint: Some(String::from("sample.rs")),
            config: RuntimeConfig::default(),
            workspace_root: Some(dir.path().to_path_buf()),
        })
        .expect("scan succeeds");

        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].message.as_deref(), Some("from text"));
    }
}
