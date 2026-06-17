use std::io::{BufRead, BufReader, Read, Write};
use std::path::PathBuf;

use serde_json::{json, Value};

use crate::core::errors::TodoError;
use crate::lsp::session::SessionState;
use crate::lsp::translate;

const MAX_FRAME_SIZE: usize = 1024 * 1024;

pub fn serve_stdio() -> Result<(), TodoError> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    serve_transport(stdin.lock(), stdout.lock())
}

pub fn serve_transport<R: Read, W: Write>(reader: R, mut writer: W) -> Result<(), TodoError> {
    let mut reader = BufReader::new(reader);
    let mut session = SessionState::default();

    while let Some(message) = read_message(&mut reader)? {
        if let Some(response) = handle_message(&mut session, &message)? {
            write_message(&mut writer, &response)?;
        }
    }

    Ok(())
}

pub fn handle_message(
    session: &mut SessionState,
    message: &Value,
) -> Result<Option<Value>, TodoError> {
    let method = message
        .get("method")
        .and_then(Value::as_str)
        .unwrap_or_default();

    match method {
        "initialize" => {
            let root_uri = message
                .pointer("/params/rootUri")
                .and_then(Value::as_str)
                .or_else(|| {
                    message
                        .pointer("/params/workspaceFolders/0/uri")
                        .and_then(Value::as_str)
                });
            let roots = root_uri
                .and_then(|uri| url::Url::parse(uri).ok())
                .and_then(|uri| uri.to_file_path().ok())
                .map(|path| vec![path])
                .unwrap_or_else(|| {
                    vec![std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))]
                });
            session.initialize(roots)?;
            Ok(Some(json!({
                "jsonrpc": "2.0",
                "id": message.get("id").cloned().unwrap_or(Value::Null),
                "result": {
                    "capabilities": {
                        "textDocumentSync": 1,
                        "workspaceSymbolProvider": true,
                        "executeCommandProvider": {
                            "commands": [crate::lsp::commands::LIST_TODOS_COMMAND]
                        }
                    }
                }
            })))
        }
        "textDocument/didOpen" => {
            let uri = message
                .pointer("/params/textDocument/uri")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let text = message
                .pointer("/params/textDocument/text")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let version = message
                .pointer("/params/textDocument/version")
                .and_then(Value::as_i64)
                .unwrap_or(0) as i32;
            let diagnostics = session.open_document(uri.to_string(), text.to_string(), version)?;
            Ok(Some(diagnostics_notification(uri, diagnostics)))
        }
        "textDocument/didChange" => {
            let uri = message
                .pointer("/params/textDocument/uri")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let version = message
                .pointer("/params/textDocument/version")
                .and_then(Value::as_i64)
                .unwrap_or(0) as i32;
            let text = message
                .pointer("/params/contentChanges/0/text")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let diagnostics =
                session.change_document(uri.to_string(), text.to_string(), version)?;
            Ok(Some(diagnostics_notification(uri, diagnostics)))
        }
        "textDocument/didClose" => {
            let uri = message
                .pointer("/params/textDocument/uri")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let diagnostics = session.close_document(uri);
            Ok(Some(diagnostics_notification(uri, diagnostics)))
        }
        "workspace/symbol" => {
            let query = message
                .pointer("/params/query")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let result = session
                .workspace_symbols(query)
                .iter()
                .map(translate::to_workspace_symbol_response)
                .collect::<Vec<_>>();
            Ok(Some(json!({
                "jsonrpc": "2.0",
                "id": message.get("id").cloned().unwrap_or(Value::Null),
                "result": result,
            })))
        }
        "workspace/executeCommand" => {
            let command = message
                .pointer("/params/command")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let result = session.execute_command(command)?;
            Ok(Some(json!({
                "jsonrpc": "2.0",
                "id": message.get("id").cloned().unwrap_or(Value::Null),
                "result": result,
            })))
        }
        "shutdown" => Ok(Some(json!({
            "jsonrpc": "2.0",
            "id": message.get("id").cloned().unwrap_or(Value::Null),
            "result": null,
        }))),
        "exit" => Ok(None),
        _ => Ok(None),
    }
}

fn diagnostics_notification(uri: &str, diagnostics: Vec<lsp_types::Diagnostic>) -> Value {
    json!({
        "jsonrpc": "2.0",
        "method": "textDocument/publishDiagnostics",
        "params": {
            "uri": uri,
            "diagnostics": diagnostics,
        }
    })
}

fn read_message(reader: &mut dyn BufRead) -> Result<Option<Value>, TodoError> {
    let mut content_length = None;
    loop {
        let mut line = String::new();
        let read = reader.read_line(&mut line)?;
        if read == 0 {
            return Ok(None);
        }
        let line = line.trim_end();
        if line.is_empty() {
            break;
        }
        if let Some(value) = line.strip_prefix("Content-Length: ") {
            content_length = value.parse::<usize>().ok();
        }
    }

    let Some(content_length) = content_length else {
        return Ok(None);
    };

    if content_length > MAX_FRAME_SIZE {
        return Err(TodoError::InvalidConfig(format!(
            "content length exceeds maximum LSP frame size: {content_length} > {MAX_FRAME_SIZE}"
        )));
    }

    let mut body = vec![0; content_length];
    reader.read_exact(&mut body)?;
    Ok(Some(serde_json::from_slice(&body)?))
}

fn write_message(writer: &mut dyn Write, message: &Value) -> Result<(), TodoError> {
    let body = serde_json::to_vec(message)?;
    writer.write_all(format!("Content-Length: {}\r\n\r\n", body.len()).as_bytes())?;
    writer.write_all(&body)?;
    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use serde_json::json;
    use tempfile::tempdir;

    use super::{read_message, serve_transport, MAX_FRAME_SIZE};

    fn frame(message: &serde_json::Value) -> Vec<u8> {
        let body = serde_json::to_vec(message).expect("json body");
        let mut framed = format!("Content-Length: {}\r\n\r\n", body.len()).into_bytes();
        framed.extend(body);
        framed
    }

    fn read_all_messages(bytes: &[u8]) -> Vec<serde_json::Value> {
        let mut cursor = Cursor::new(bytes.to_vec());
        let mut values = Vec::new();
        while let Some(value) = read_message(&mut cursor).expect("message read") {
            values.push(value);
        }
        values
    }

    #[test]
    fn serve_transport_handles_lsp_round_trip() {
        let dir = tempdir().expect("tempdir");
        let path = dir.path().join("sample.rs");
        std::fs::write(&path, "// TODO: disk item\n").expect("fixture written");
        let uri = crate::core::model::path_to_uri(&path);
        let root_uri = crate::core::model::path_to_uri(dir.path());

        let mut input = Vec::new();
        input.extend(frame(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": { "rootUri": root_uri }
        })));
        input.extend(frame(&json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": {
                "textDocument": {
                    "uri": uri,
                    "version": 1,
                    "text": "// TODO: overlay item\n"
                }
            }
        })));
        input.extend(frame(&json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "workspace/symbol",
            "params": { "query": "overlay" }
        })));
        input.extend(frame(&json!({
            "jsonrpc": "2.0",
            "id": 3,
            "method": "workspace/executeCommand",
            "params": { "command": crate::lsp::commands::LIST_TODOS_COMMAND }
        })));
        input.extend(frame(&json!({
            "jsonrpc": "2.0",
            "id": 4,
            "method": "shutdown",
            "params": null
        })));

        let mut output = Vec::new();
        serve_transport(Cursor::new(input), &mut output).expect("transport succeeds");

        let messages = read_all_messages(&output);
        assert_eq!(messages.len(), 5);
        assert_eq!(messages[1]["method"], "textDocument/publishDiagnostics");
        assert_eq!(messages[2]["result"].as_array().map(Vec::len), Some(1));
        assert_eq!(messages[3]["result"].as_array().map(Vec::len), Some(1));
        assert!(messages[4]["result"].is_null());
    }

    #[test]
    fn rejects_oversized_content_length() {
        let input = format!("Content-Length: {}\r\n\r\n", MAX_FRAME_SIZE + 1);
        let mut cursor = Cursor::new(input.into_bytes());
        let error = read_message(&mut cursor).expect_err("oversized frame should fail closed");
        assert!(error.to_string().contains("content length exceeds"));
    }
}
