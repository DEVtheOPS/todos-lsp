use assert_cmd::Command;
use std::path::Path;

#[test]
fn packaged_binary_exposes_help_and_serve_help() {
    Command::cargo_bin("todos-lsp")
        .expect("binary exists")
        .arg("--help")
        .assert()
        .success();

    Command::cargo_bin("todos-lsp")
        .expect("binary exists")
        .args(["serve", "--help"])
        .assert()
        .success();

    for path in [
        "release-please-config.json",
        ".release-please-manifest.json",
        ".github/workflows/ci.yml",
        ".github/workflows/release-please.yml",
        ".github/workflows/release-validate.yml",
        ".github/workflows/publish.yml",
        "docs/parity/matrix.md",
        "docs/parity/provenance.md",
        "CONTRIBUTING.md",
    ] {
        assert!(Path::new(path).exists(), "expected {path} to exist");
    }
}
