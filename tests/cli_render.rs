use todos_lsp::cli::render;
use todos_lsp::core::config::OutputFormat;
use todos_lsp::core::model::{Finding, SourceKind};

#[test]
fn renders_plain_and_json_outputs_from_shared_findings() {
    let finding = Finding::new(
        std::path::PathBuf::from("src/lib.rs"),
        String::from("file:///tmp/src/lib.rs"),
        1,
        0,
        String::from("// TODO(owner): write renderer tests"),
        String::from("TODO"),
        Some(String::from("owner")),
        Some(String::from("write renderer tests")),
        SourceKind::Disk,
    );

    let plain = render::render(std::slice::from_ref(&finding), OutputFormat::Default)
        .expect("plain output");
    assert!(plain.contains("src/lib.rs:2:// TODO(owner): write renderer tests"));

    let json = render::render(&[finding], OutputFormat::Json).expect("json output");
    assert!(json.contains("\"type\":\"TODO\""));
    assert!(json.contains("\"label\":\"owner\""));
}
