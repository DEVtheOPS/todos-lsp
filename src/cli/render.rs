use crate::core::config::OutputFormat;
use crate::core::model::Finding;

pub fn render(findings: &[Finding], output: OutputFormat) -> Result<String, serde_json::Error> {
    match output {
        OutputFormat::Default => Ok(render_plain(findings)),
        OutputFormat::Github => Ok(render_github(findings)),
        OutputFormat::Json => render_json(findings),
    }
}

pub fn render_plain(findings: &[Finding]) -> String {
    if findings.is_empty() {
        return String::new();
    }

    let lines = findings.iter().map(render_plain_line).collect::<Vec<_>>();
    format!("{}\n", lines.join("\n"))
}

pub fn render_github(findings: &[Finding]) -> String {
    if findings.is_empty() {
        return String::new();
    }

    let lines = findings
        .iter()
        .map(|finding| {
            format!(
                "::warning file={},line={}::{}",
                escape_github_annotation(&finding.display_path()),
                finding.range.start.line + 1,
                escape_github_annotation(&finding.raw_text)
            )
        })
        .collect::<Vec<_>>();
    format!("{}\n", lines.join("\n"))
}

fn escape_github_annotation(value: &str) -> String {
    value
        .replace('%', "%25")
        .replace('\r', "%0D")
        .replace('\n', "%0A")
        .replace(':', "%3A")
        .replace(',', "%2C")
}

pub fn render_json(findings: &[Finding]) -> Result<String, serde_json::Error> {
    if findings.is_empty() {
        return Ok(String::new());
    }

    let mut output = String::new();
    for finding in findings {
        let line = escape_html_json_chars(serde_json::to_string(&finding.as_cli_json())?);
        output.push_str(&line);
        output.push('\n');
    }
    Ok(output)
}

fn escape_html_json_chars(input: String) -> String {
    input
        .replace('<', "\\u003c")
        .replace('>', "\\u003e")
        .replace('&', "\\u0026")
}

fn render_plain_line(finding: &Finding) -> String {
    match &finding.blame {
        Some(blame) => format!(
            "{}:{}:{} <{}>:{}",
            finding.display_path(),
            finding.range.start.line + 1,
            blame.author,
            blame.email,
            finding.raw_text
        ),
        None => format!(
            "{}:{}:{}",
            finding.display_path(),
            finding.range.start.line + 1,
            finding.raw_text
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::{render, render_github, render_plain};
    use crate::core::config::OutputFormat;
    use crate::core::model::{Finding, SourceKind};

    fn sample_finding() -> Finding {
        Finding::new(
            std::path::PathBuf::from("src/lib.rs"),
            String::from("file:///tmp/src/lib.rs"),
            0,
            0,
            String::from("// TODO: sample"),
            String::from("TODO"),
            None,
            Some(String::from("sample")),
            SourceKind::Disk,
        )
    }

    #[test]
    fn renders_empty_outputs() {
        assert_eq!(render_plain(&[]), "");
        assert_eq!(render_github(&[]), "");
        assert_eq!(render(&[], OutputFormat::Json).expect("json"), "");
    }

    #[test]
    fn renders_github_output() {
        let mut finding = sample_finding();
        finding.path = std::path::PathBuf::from("src/a%b:c,d\r\nlib.rs");
        finding.raw_text = String::from("// TODO: 100% done\r\nnext: item, ok");
        let output = render_github(&[finding]);
        assert!(output.contains("file=src/a%25b%3Ac%2Cd%0D%0Alib.rs"));
        assert!(output.contains("::// TODO%3A 100%25 done%0D%0Anext%3A item%2C ok"));
    }
}
