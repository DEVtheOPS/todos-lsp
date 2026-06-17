use todos_lsp::core::config::RuntimeConfig;
use todos_lsp::core::model::{DiskScanRequest, TextScanRequest};
use todos_lsp::core::scan;

mod support;

#[test]
fn scans_disk_and_text_through_same_model() {
    let fixture = support::fixtures::basic_fixture();
    let root = support::fixtures::fixture_root(&fixture);
    let request = DiskScanRequest {
        root: root.clone(),
        paths: vec![std::path::PathBuf::from(".")],
        config: RuntimeConfig::default(),
    };

    let disk_findings = scan::scan_disk(&request).expect("disk scan succeeds");
    assert_eq!(disk_findings.len(), 3);
    assert!(disk_findings
        .iter()
        .all(|finding| !finding.display_path().contains("ignored.rs")));
    assert!(disk_findings
        .iter()
        .all(|finding| !finding.display_path().contains("ignored.gen.rs")));

    let text = std::fs::read_to_string(root.join("src/lib.rs")).expect("fixture readable");
    let text_findings = scan::scan_text(&TextScanRequest {
        uri: todos_lsp::core::model::path_to_uri(&root.join("src/lib.rs")),
        text,
        language_hint: Some(String::from("lib.rs")),
        config: RuntimeConfig::default(),
        workspace_root: Some(root),
    })
    .expect("text scan succeeds");

    assert_eq!(text_findings.len(), 2);
    assert_eq!(text_findings[0].todo_type, "TODO");
    assert_eq!(text_findings[1].todo_type, "FIXME");
}
