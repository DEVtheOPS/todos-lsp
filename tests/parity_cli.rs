#[cfg(unix)]
use std::os::unix::fs as unix_fs;
use std::path::Path;
use tempfile::tempdir;

use assert_cmd::Command;

mod support;

#[test]
fn compares_todos_lsp_to_pinned_upstream_for_mvp_rows() {
    let Some(upstream) = support::upstream::upstream_binary() else {
        eprintln!("skipping parity check: upstream binary unavailable");
        return;
    };

    let fixture = support::fixtures::basic_fixture();
    let root = support::fixtures::fixture_root(&fixture);

    let upstream_observation = support::upstream::observe(&upstream, &["-o", "json", "."], &root);

    let local_output = Command::cargo_bin("todos-lsp")
        .expect("binary exists")
        .args(["-o", "json", "."])
        .current_dir(&root)
        .output()
        .expect("local run succeeds");

    assert_eq!(
        String::from_utf8_lossy(&local_output.stdout),
        upstream_observation.stdout
    );
    assert_eq!(
        local_output.status.code().unwrap_or(1),
        upstream_observation.exit_code
    );
}

#[test]
fn compares_unsupported_file_behavior_to_pinned_upstream() {
    let Some(upstream) = support::upstream::upstream_binary() else {
        eprintln!("skipping parity check: upstream binary unavailable");
        return;
    };

    let fixture = support::fixtures::unsupported_fixture();
    let root = support::fixtures::fixture_root(&fixture);
    let file = root.join("plain.txt");

    let upstream_observation = support::upstream::observe(
        &upstream,
        &[file.to_string_lossy().as_ref()],
        Path::new(env!("CARGO_MANIFEST_DIR")),
    );

    let local_output = Command::cargo_bin("todos-lsp")
        .expect("binary exists")
        .arg(file.to_string_lossy().as_ref())
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("local run succeeds");

    assert_eq!(
        local_output.status.code().unwrap_or(1),
        upstream_observation.exit_code
    );
    let local_stderr = String::from_utf8_lossy(&local_output.stderr);
    assert!(local_stderr.contains("unsupported"));
    assert!(local_stderr.contains(file.to_string_lossy().as_ref()));
}

#[test]
fn matches_dockerfile_support_behavior() {
    let Some(upstream) = support::upstream::upstream_binary() else {
        eprintln!("skipping parity check: upstream binary unavailable");
        return;
    };

    let dir = tempdir().expect("tempdir");
    std::fs::write(dir.path().join("Dockerfile"), "# TODO: docker support\n")
        .expect("fixture written");

    let upstream_output = std::process::Command::new(&upstream)
        .arg(".")
        .current_dir(dir.path())
        .output()
        .expect("upstream run succeeds");

    let local_output = Command::cargo_bin("todos-lsp")
        .expect("binary exists")
        .arg(".")
        .current_dir(dir.path())
        .output()
        .expect("local run succeeds");

    assert_eq!(
        String::from_utf8_lossy(&local_output.stdout),
        String::from_utf8_lossy(&upstream_output.stdout)
    );
    assert_eq!(
        local_output.status.code().unwrap_or(1),
        upstream_output.status.code().unwrap_or(1)
    );
}

#[test]
fn matches_charset_detect_behavior() {
    let Some(upstream) = support::upstream::upstream_binary() else {
        eprintln!("skipping parity check: upstream binary unavailable");
        return;
    };

    let dir = tempdir().expect("tempdir");
    std::fs::write(
        dir.path().join("sample.py"),
        b"# TODO: caf\xe9\n".as_slice(),
    )
    .expect("fixture written");

    let upstream_output = std::process::Command::new(&upstream)
        .args(["--charset", "detect", "."])
        .current_dir(dir.path())
        .output()
        .expect("upstream run succeeds");

    let local_output = Command::cargo_bin("todos-lsp")
        .expect("binary exists")
        .args(["--charset", "detect", "."])
        .current_dir(dir.path())
        .output()
        .expect("local run succeeds");

    assert_eq!(
        String::from_utf8_lossy(&local_output.stdout),
        String::from_utf8_lossy(&upstream_output.stdout)
    );
    assert_eq!(
        String::from_utf8_lossy(&local_output.stderr),
        String::from_utf8_lossy(&upstream_output.stderr)
    );
    assert_eq!(
        local_output.status.code().unwrap_or(1),
        upstream_output.status.code().unwrap_or(1)
    );
}

#[test]
fn matches_lua_support_behavior() {
    let Some(upstream) = support::upstream::upstream_binary() else {
        eprintln!("skipping parity check: upstream binary unavailable");
        return;
    };

    let dir = tempdir().expect("tempdir");
    std::fs::write(dir.path().join("sample.lua"), "-- TODO: lua support\n")
        .expect("fixture written");

    let upstream_output = std::process::Command::new(&upstream)
        .arg(".")
        .current_dir(dir.path())
        .output()
        .expect("upstream run succeeds");

    let local_output = Command::cargo_bin("todos-lsp")
        .expect("binary exists")
        .arg(".")
        .current_dir(dir.path())
        .output()
        .expect("local run succeeds");

    assert_eq!(
        String::from_utf8_lossy(&local_output.stdout),
        String::from_utf8_lossy(&upstream_output.stdout)
    );
    assert_eq!(
        local_output.status.code().unwrap_or(1),
        upstream_output.status.code().unwrap_or(1)
    );
}

#[cfg(unix)]
#[test]
fn matches_symlink_traversal_behavior() {
    let Some(upstream) = support::upstream::upstream_binary() else {
        eprintln!("skipping parity check: upstream binary unavailable");
        return;
    };

    let dir = tempdir().expect("tempdir");
    let real_dir = dir.path().join("real");
    std::fs::create_dir(&real_dir).expect("real dir created");
    std::fs::write(
        real_dir.join("nested.rs"),
        "pub fn sample() {\n    // TODO: nested symlink target\n}\n",
    )
    .expect("fixture written");
    unix_fs::symlink(&real_dir, dir.path().join("linked-real")).expect("symlink created");

    let upstream_default = std::process::Command::new(&upstream)
        .arg(".")
        .current_dir(dir.path())
        .output()
        .expect("upstream default succeeds");
    let local_default = Command::cargo_bin("todos-lsp")
        .expect("binary exists")
        .arg(".")
        .current_dir(dir.path())
        .output()
        .expect("local default succeeds");

    assert_eq!(
        String::from_utf8_lossy(&local_default.stdout),
        String::from_utf8_lossy(&upstream_default.stdout)
    );
    assert_eq!(
        local_default.status.code().unwrap_or(1),
        upstream_default.status.code().unwrap_or(1)
    );

    let upstream_follow = std::process::Command::new(&upstream)
        .args(["--follow", "."])
        .current_dir(dir.path())
        .output()
        .expect("upstream follow succeeds");
    let local_follow = Command::cargo_bin("todos-lsp")
        .expect("binary exists")
        .args(["--follow", "."])
        .current_dir(dir.path())
        .output()
        .expect("local follow succeeds");

    assert_eq!(
        String::from_utf8_lossy(&local_follow.stdout),
        String::from_utf8_lossy(&upstream_follow.stdout)
    );
    assert_eq!(
        local_follow.status.code().unwrap_or(1),
        upstream_follow.status.code().unwrap_or(1)
    );
}

#[test]
fn matches_label_filter_behavior() {
    let Some(upstream) = support::upstream::upstream_binary() else {
        eprintln!("skipping parity check: upstream binary unavailable");
        return;
    };

    let fixture = support::fixtures::basic_fixture();
    let root = support::fixtures::fixture_root(&fixture);

    let upstream_output = std::process::Command::new(&upstream)
        .args(["-o", "json", "-l", "owner", "."])
        .current_dir(&root)
        .output()
        .expect("upstream run succeeds");

    let local_output = Command::cargo_bin("todos-lsp")
        .expect("binary exists")
        .args(["-o", "json", "-l", "owner", "."])
        .current_dir(&root)
        .output()
        .expect("local run succeeds");

    assert_eq!(
        String::from_utf8_lossy(&local_output.stdout),
        String::from_utf8_lossy(&upstream_output.stdout)
    );
    assert_eq!(
        local_output.status.code().unwrap_or(1),
        upstream_output.status.code().unwrap_or(1)
    );
}

#[test]
fn matches_no_error_on_unsupported_behavior() {
    let Some(upstream) = support::upstream::upstream_binary() else {
        eprintln!("skipping parity check: upstream binary unavailable");
        return;
    };

    let fixture = support::fixtures::unsupported_fixture();
    let root = support::fixtures::fixture_root(&fixture);
    let file = root.join("plain.txt");

    let upstream_output = std::process::Command::new(&upstream)
        .args(["--no-error-on-unsupported", file.to_string_lossy().as_ref()])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("upstream run succeeds");

    let local_output = Command::cargo_bin("todos-lsp")
        .expect("binary exists")
        .args(["--no-error-on-unsupported", file.to_string_lossy().as_ref()])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("local run succeeds");

    assert_eq!(
        String::from_utf8_lossy(&local_output.stdout),
        String::from_utf8_lossy(&upstream_output.stdout)
    );
    assert_eq!(
        String::from_utf8_lossy(&local_output.stderr),
        String::from_utf8_lossy(&upstream_output.stderr)
    );
    assert_eq!(
        local_output.status.code().unwrap_or(1),
        upstream_output.status.code().unwrap_or(1)
    );
}

#[test]
fn matches_blame_success_behavior() {
    let Some(upstream) = support::upstream::upstream_binary() else {
        eprintln!("skipping parity check: upstream binary unavailable");
        return;
    };

    let dir = tempdir().expect("tempdir");
    std::process::Command::new("git")
        .arg("init")
        .current_dir(dir.path())
        .output()
        .expect("git init succeeds");
    std::process::Command::new("git")
        .args(["config", "user.name", "Example Dev"])
        .current_dir(dir.path())
        .output()
        .expect("git config name succeeds");
    std::process::Command::new("git")
        .args(["config", "user.email", "dev@example.com"])
        .current_dir(dir.path())
        .output()
        .expect("git config email succeeds");

    std::fs::write(
        dir.path().join("sample.rs"),
        "pub fn sample() {\n    // TODO: blame support\n}\n",
    )
    .expect("fixture written");
    std::process::Command::new("git")
        .args(["add", "sample.rs"])
        .current_dir(dir.path())
        .output()
        .expect("git add succeeds");
    std::process::Command::new("git")
        .args(["commit", "-m", "add sample"])
        .current_dir(dir.path())
        .output()
        .expect("git commit succeeds");

    let upstream_output = std::process::Command::new(&upstream)
        .args(["--blame", "."])
        .current_dir(dir.path())
        .output()
        .expect("upstream run succeeds");

    let local_output = Command::cargo_bin("todos-lsp")
        .expect("binary exists")
        .args(["--blame", "."])
        .current_dir(dir.path())
        .output()
        .expect("local run succeeds");

    assert_eq!(
        String::from_utf8_lossy(&local_output.stdout),
        String::from_utf8_lossy(&upstream_output.stdout)
    );
    assert_eq!(
        String::from_utf8_lossy(&local_output.stderr),
        String::from_utf8_lossy(&upstream_output.stderr)
    );
    assert_eq!(
        local_output.status.code().unwrap_or(1),
        upstream_output.status.code().unwrap_or(1)
    );
}

#[test]
fn matches_mixed_language_support_behavior() {
    let Some(upstream) = support::upstream::upstream_binary() else {
        eprintln!("skipping parity check: upstream binary unavailable");
        return;
    };

    let dir = tempdir().expect("tempdir");
    std::fs::write(
        dir.path().join("main.go"),
        "package main\nfunc main() {\n    // TODO: go support\n}\n",
    )
    .expect("go fixture written");
    std::fs::write(
        dir.path().join("Main.java"),
        "class Main { void run() { // TODO: java support\n} }\n",
    )
    .expect("java fixture written");
    std::fs::write(
        dir.path().join("infra.tf"),
        "# TODO: terraform support\nresource \"null_resource\" \"x\" {}\n",
    )
    .expect("terraform fixture written");
    std::fs::write(
        dir.path().join("index.html"),
        "<html><!-- TODO: html support --></html>\n",
    )
    .expect("html fixture written");
    std::fs::write(
        dir.path().join("sample.rb"),
        "def run\n  # TODO: ruby support\nend\n",
    )
    .expect("ruby fixture written");
    std::fs::write(
        dir.path().join("schema.graphql"),
        "# TODO: graphql support\ntype Query { x: String }\n",
    )
    .expect("graphql fixture written");
    std::fs::write(
        dir.path().join(".gitignore"),
        "# TODO: ignore list support\nnode_modules\n",
    )
    .expect("gitignore fixture written");
    std::fs::write(
        dir.path().join(".gitconfig"),
        "; TODO: gitconfig support\n[user]\nname = x\n",
    )
    .expect("gitconfig fixture written");
    std::fs::write(
        dir.path().join("Main.kt"),
        "fun main() { // TODO: kotlin support\n}\n",
    )
    .expect("kotlin fixture written");
    std::fs::write(
        dir.path().join("sample.c"),
        "int main() { /* TODO: c block support */ return 0; }\n",
    )
    .expect("c fixture written");
    std::fs::write(
        dir.path().join("main.dart"),
        "void main() { // TODO: dart support\n}\n",
    )
    .expect("dart fixture written");
    std::fs::write(
        dir.path().join("page.erb"),
        "<!-- TODO: erb support -->\n<div>Hello</div>\n",
    )
    .expect("erb fixture written");

    let upstream_output = std::process::Command::new(&upstream)
        .args(["-o", "json", "."])
        .current_dir(dir.path())
        .output()
        .expect("upstream run succeeds");

    let local_output = Command::cargo_bin("todos-lsp")
        .expect("binary exists")
        .args(["-o", "json", "."])
        .current_dir(dir.path())
        .output()
        .expect("local run succeeds");

    assert_eq!(
        String::from_utf8_lossy(&local_output.stdout),
        String::from_utf8_lossy(&upstream_output.stdout)
    );
    assert_eq!(
        local_output.status.code().unwrap_or(1),
        upstream_output.status.code().unwrap_or(1)
    );
}

#[test]
fn matches_multiline_block_comment_behavior() {
    let Some(upstream) = support::upstream::upstream_binary() else {
        eprintln!("skipping parity check: upstream binary unavailable");
        return;
    };

    let dir = tempdir().expect("tempdir");
    std::fs::write(
        dir.path().join("sample.c"),
        "/*\n * TODO: multiline block support\n */\nint main() { return 0; }\n",
    )
    .expect("fixture written");

    let upstream_output = std::process::Command::new(&upstream)
        .args(["-o", "json", "."])
        .current_dir(dir.path())
        .output()
        .expect("upstream run succeeds");

    let local_output = Command::cargo_bin("todos-lsp")
        .expect("binary exists")
        .args(["-o", "json", "."])
        .current_dir(dir.path())
        .output()
        .expect("local run succeeds");

    assert_eq!(
        String::from_utf8_lossy(&local_output.stdout),
        String::from_utf8_lossy(&upstream_output.stdout)
    );
    assert_eq!(
        local_output.status.code().unwrap_or(1),
        upstream_output.status.code().unwrap_or(1)
    );
}

#[test]
fn matches_elixir_hash_comment_behavior() {
    let Some(upstream) = support::upstream::upstream_binary() else {
        eprintln!("skipping parity check: upstream binary unavailable");
        return;
    };

    let dir = tempdir().expect("tempdir");
    std::fs::write(
        dir.path().join("sample.ex"),
        "defmodule Sample do\n  # TODO: elixir support\nend\n",
    )
    .expect("fixture written");

    let upstream_output = std::process::Command::new(&upstream)
        .args(["-o", "json", "."])
        .current_dir(dir.path())
        .output()
        .expect("upstream run succeeds");

    let local_output = Command::cargo_bin("todos-lsp")
        .expect("binary exists")
        .args(["-o", "json", "."])
        .current_dir(dir.path())
        .output()
        .expect("local run succeeds");

    assert_eq!(
        String::from_utf8_lossy(&local_output.stdout),
        String::from_utf8_lossy(&upstream_output.stdout)
    );
    assert_eq!(
        local_output.status.code().unwrap_or(1),
        upstream_output.status.code().unwrap_or(1)
    );
}
