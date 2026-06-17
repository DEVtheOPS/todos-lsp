use std::io::Write;
use std::path::PathBuf;

use crate::cli::args::ScanArgs;
use crate::cli::render;
use crate::core::config::{OutputFormat, RuntimeConfig};
use crate::core::errors::TodoError;
use crate::core::model::DiskScanRequest;
use crate::core::scan;

pub fn run(
    args: ScanArgs,
    stdout: &mut dyn Write,
    stderr: &mut dyn Write,
) -> Result<i32, TodoError> {
    let config = RuntimeConfig::from_scan_args(&args)?;
    let request = DiskScanRequest {
        root: std::env::current_dir().map_err(TodoError::from)?,
        paths: args.paths,
        config: config.clone(),
    };

    let findings = scan::scan_disk(&request)?;
    let rendered = render::render(&findings, config.output)?;

    if !rendered.is_empty() {
        stdout.write_all(rendered.as_bytes())?;
    }

    if findings.is_empty() {
        let _ = stderr.flush();
        Ok(0)
    } else {
        Ok(1)
    }
}

pub fn run_for_paths(paths: Vec<PathBuf>, config: RuntimeConfig) -> Result<String, TodoError> {
    let request = DiskScanRequest {
        root: std::env::current_dir().map_err(TodoError::from)?,
        paths,
        config: config.clone(),
    };
    let findings = scan::scan_disk(&request)?;
    Ok(render::render(&findings, OutputFormat::Default)?)
}

#[cfg(test)]
mod tests {
    use std::sync::{Mutex, OnceLock};

    use tempfile::tempdir;

    use super::{run, run_for_paths};
    use crate::cli::args::{OutputArg, ScanArgs};
    use crate::core::config::RuntimeConfig;

    fn cwd_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    #[test]
    fn returns_lint_exit_code_when_findings_exist() {
        let _guard = cwd_lock().lock().expect("cwd lock");
        let dir = tempdir().expect("tempdir");
        std::fs::write(dir.path().join("sample.rs"), "// TODO: command test\n")
            .expect("fixture written");
        let old_dir = std::env::current_dir().expect("cwd");
        std::env::set_current_dir(dir.path()).expect("change dir");

        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        let exit_code = run(
            ScanArgs {
                blame: false,
                charset: String::from("UTF-8"),
                excludes: Vec::new(),
                exclude_dirs: Vec::new(),
                exclude_hidden: false,
                follow: false,
                ignore_file_names: vec![String::from(".gitignore"), String::from(".todosignore")],
                include_vcs: false,
                include_generated: false,
                include_vendored: false,
                labels: Vec::new(),
                no_error_on_unsupported: false,
                output: Some(OutputArg::Default),
                todo_types: None,
                paths: vec![std::path::PathBuf::from(".")],
            },
            &mut stdout,
            &mut stderr,
        )
        .expect("command succeeds");

        std::env::set_current_dir(old_dir).expect("restore dir");
        assert_eq!(exit_code, 1);
        assert!(String::from_utf8_lossy(&stdout).contains("sample.rs:1:// TODO: command test"));
    }

    #[test]
    fn renders_paths_through_helper() {
        let _guard = cwd_lock().lock().expect("cwd lock");
        let dir = tempdir().expect("tempdir");
        std::fs::write(dir.path().join("sample.rs"), "// TODO: helper output\n")
            .expect("fixture written");
        let old_dir = std::env::current_dir().expect("cwd");
        std::env::set_current_dir(dir.path()).expect("change dir");

        let output = run_for_paths(
            vec![std::path::PathBuf::from(".")],
            RuntimeConfig::default(),
        )
        .expect("helper succeeds");

        std::env::set_current_dir(old_dir).expect("restore dir");
        assert!(output.contains("sample.rs:1:// TODO: helper output"));
    }
}
