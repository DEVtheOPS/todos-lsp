use assert_cmd::Command;

#[test]
fn parses_root_scan_and_serve_paths() {
    Command::cargo_bin("todos-lsp")
        .expect("binary exists")
        .arg("serve")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn supports_version_flag() {
    Command::cargo_bin("todos-lsp")
        .expect("binary exists")
        .arg("--version")
        .assert()
        .success();
}
