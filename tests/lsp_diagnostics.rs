use serde_json::json;
use tempfile::tempdir;
use todos_lsp::lsp::server;
use todos_lsp::lsp::session::SessionState;

#[test]
fn publish_diagnostics_from_shared_core_findings() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("sample.rs");
    std::fs::write(&path, "// TODO: publish diagnostics\n").expect("fixture written");

    let mut session = SessionState::default();
    let response = server::handle_message(
        &mut session,
        &json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": { "rootUri": todos_lsp::core::model::path_to_uri(dir.path()) }
        }),
    )
    .expect("initialize works")
    .expect("response exists");

    assert!(
        response["result"]["capabilities"]["workspaceSymbolProvider"]
            .as_bool()
            .unwrap_or(false)
    );

    let opened = server::handle_message(
        &mut session,
        &json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": {
                "textDocument": {
                    "uri": todos_lsp::core::model::path_to_uri(&path),
                    "version": 1,
                    "text": "// TODO: updated diagnostics\n"
                }
            }
        }),
    )
    .expect("didOpen works")
    .expect("notification exists");

    assert_eq!(opened["method"], "textDocument/publishDiagnostics");
    assert_eq!(
        opened["params"]["diagnostics"].as_array().map(Vec::len),
        Some(1)
    );
}
