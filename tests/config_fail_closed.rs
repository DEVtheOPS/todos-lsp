use tempfile::tempdir;
use todos_lsp::core::blame;
use todos_lsp::core::config::RuntimeConfig;

#[test]
fn invalid_config_fails_closed() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("todos.json");
    std::fs::write(&path, "{not-json}").expect("config written");

    let error = RuntimeConfig::from_file(&path).expect_err("config should fail");
    assert!(error.to_string().contains("invalid config"));
}

#[test]
fn missing_required_runtime_dependency_fails_closed() {
    let dir = tempdir().expect("tempdir");
    let file = dir.path().join("sample.rs");
    std::fs::write(&file, "// TODO: blame this\n").expect("fixture written");
    let finding = todos_lsp::core::model::Finding::new(
        file.clone(),
        todos_lsp::core::model::path_to_uri(&file),
        0,
        0,
        String::from("// TODO: blame this"),
        String::from("TODO"),
        None,
        Some(String::from("blame this")),
        todos_lsp::core::model::SourceKind::Disk,
    );

    let error = blame::enrich_with_blame(vec![finding], std::path::Path::new("missing-git"))
        .expect_err("missing command should fail");
    assert!(error.to_string().contains("missing runtime dependency"));
}
