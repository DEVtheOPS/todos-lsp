use tempfile::tempdir;
use todos_lsp::lsp::session::SessionState;

#[test]
fn open_buffer_overrides_disk_and_reverts_on_close() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("sample.rs");
    std::fs::write(&path, "// TODO: disk copy\n").expect("disk fixture written");

    let mut session = SessionState::default();
    session
        .initialize(vec![dir.path().to_path_buf()])
        .expect("session initializes");
    let uri = todos_lsp::core::model::path_to_uri(&path);
    assert_eq!(session.diagnostics(&uri).len(), 1);

    let diagnostics = session
        .open_document(uri.clone(), String::from("// FIXME: overlay copy\n"), 1)
        .expect("overlay opens");
    assert_eq!(diagnostics.len(), 1);
    assert!(diagnostics[0].message.contains("overlay copy"));

    let reverted = session.close_document(&uri);
    assert_eq!(reverted.len(), 1);
    assert!(reverted[0].message.contains("disk copy"));
}

#[test]
fn overlay_paths_remain_relative_to_initialized_workspace_root() {
    let dir = tempdir().expect("tempdir");
    let nested = dir.path().join("src");
    std::fs::create_dir(&nested).expect("nested dir");
    let path = nested.join("sample.rs");
    std::fs::write(&path, "// TODO: disk copy\n").expect("disk fixture written");

    let mut session = SessionState::default();
    session
        .initialize(vec![dir.path().to_path_buf()])
        .expect("session initializes");
    let uri = todos_lsp::core::model::path_to_uri(&path);

    session
        .open_document(uri, String::from("// TODO: overlay copy\n"), 1)
        .expect("overlay opens");
    let result = session
        .execute_command(todos_lsp::lsp::commands::LIST_TODOS_COMMAND)
        .expect("command succeeds");
    let path = result[0]["path"].as_str().expect("path string");
    assert_eq!(path, "src/sample.rs");
}
