#[test]
fn core_config_does_not_import_cli_adapter_types() {
    let config = std::fs::read_to_string("src/core/config.rs").expect("config source");
    assert!(!config.contains("crate::cli::args"));
    assert!(!config.contains("ScanArgs"));
    assert!(!config.contains("OutputArg"));
}

#[test]
fn ci_and_release_validation_require_pinned_upstream_binary_for_parity() {
    let ci = std::fs::read_to_string(".github/workflows/ci.yml").expect("ci workflow");
    let release = std::fs::read_to_string(".github/workflows/release-validate.yml")
        .expect("release validation workflow");
    for workflow in [ci, release] {
        assert!(workflow.contains(".tmp/todos-upstream"));
        assert!(workflow.contains("UPSTREAM_TODOS_BIN"));
        assert!(workflow.contains("cargo test --test parity_cli"));
    }
}

#[test]
fn publish_workflow_validates_before_publishing_and_uses_env_token() {
    let publish =
        std::fs::read_to_string(".github/workflows/publish.yml").expect("publish workflow");
    assert!(publish.contains("cargo fmt --check"));
    assert!(publish.contains("cargo clippy --all-targets --all-features -- -D warnings"));
    assert!(publish.contains("cargo test --all-targets"));
    assert!(publish.contains("cargo package"));
    assert!(publish.contains("cargo publish --dry-run"));
    assert!(publish.contains("CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}"));
    assert!(publish.contains("cargo publish"));
    assert!(!publish.contains("cargo publish --token"));
}

#[test]
fn readme_documents_operator_surface_and_clean_room_scope() {
    let readme = std::fs::read_to_string("README.md").expect("readme");
    for required in [
        "Installation",
        "Usage",
        "CLI parity scope",
        "todos-lsp serve",
        "Release and publishing",
        "Clean-room",
        "MVP-only parity",
    ] {
        assert!(
            readme.contains(required),
            "missing README section: {required}"
        );
    }
}
