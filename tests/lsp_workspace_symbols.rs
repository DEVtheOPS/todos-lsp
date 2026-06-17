use serde_json::json;
use tempfile::tempdir;
use todos_lsp::lsp::commands::LIST_TODOS_COMMAND;
use todos_lsp::lsp::server;
use todos_lsp::lsp::session::SessionState;

#[test]
fn workspace_symbol_uses_overlay_aware_index() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("sample.rs");
    std::fs::write(&path, "// TODO: disk symbol\n").expect("fixture written");
    let uri = todos_lsp::core::model::path_to_uri(&path);

    let mut session = SessionState::default();
    session
        .initialize(vec![dir.path().to_path_buf()])
        .expect("session initializes");
    session
        .open_document(uri.clone(), String::from("// TODO: overlay symbol\n"), 1)
        .expect("overlay opens");

    let symbols = session.workspace_symbols("overlay");
    assert_eq!(symbols.len(), 1);
    assert!(symbols[0].name.contains("overlay symbol"));

    let command = server::handle_message(
        &mut session,
        &json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "workspace/executeCommand",
            "params": { "command": LIST_TODOS_COMMAND }
        }),
    )
    .expect("command works")
    .expect("response exists");

    assert_eq!(command["result"].as_array().map(Vec::len), Some(1));
}
