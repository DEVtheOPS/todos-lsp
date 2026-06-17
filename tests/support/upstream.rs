use std::path::PathBuf;
use std::process::Command;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandObservation {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

#[allow(dead_code)]
pub fn upstream_binary() -> Option<PathBuf> {
    let binary = std::env::var_os("UPSTREAM_TODOS_BIN")
        .map(PathBuf::from)
        .map(|path| path.canonicalize().unwrap_or(path))
        .or_else(|| {
            let candidate = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".tmp/todos-upstream");
            candidate.exists().then_some(candidate)
        });

    if binary.is_none() && std::env::var_os("CI").is_some() {
        panic!(
            "pinned upstream todos binary is required in CI; set UPSTREAM_TODOS_BIN or provision .tmp/todos-upstream"
        );
    }

    binary
}

#[allow(dead_code)]
pub fn observe(
    program: &std::path::Path,
    args: &[&str],
    current_dir: &std::path::Path,
) -> CommandObservation {
    let output = Command::new(program)
        .args(args)
        .current_dir(current_dir)
        .output()
        .expect("command executes");

    CommandObservation {
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        exit_code: output.status.code().unwrap_or(1),
    }
}
