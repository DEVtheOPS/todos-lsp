use std::io::Write;
use std::path::PathBuf;

use crate::cli::args::{OutputArg, ScanArgs};
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
    let config = runtime_config_from_scan_args(&args)?;
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

fn runtime_config_from_scan_args(args: &ScanArgs) -> Result<RuntimeConfig, TodoError> {
    let todo_types = match &args.todo_types {
        Some(value) => value
            .split(',')
            .map(str::trim)
            .filter(|item| !item.is_empty())
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>(),
        None => crate::core::config::default_todo_types(),
    };

    RuntimeConfig {
        todo_types,
        output: match args.output.unwrap_or(OutputArg::Default) {
            OutputArg::Default => OutputFormat::Default,
            OutputArg::Github => OutputFormat::Github,
            OutputArg::Json => OutputFormat::Json,
        },
        blame: args.blame,
        excludes: args.excludes.clone(),
        exclude_dirs: args.exclude_dirs.clone(),
        exclude_hidden: args.exclude_hidden,
        follow_symlinks: args.follow,
        ignore_file_names: args.ignore_file_names.clone(),
        include_vcs: args.include_vcs,
        include_generated: args.include_generated,
        include_vendored: args.include_vendored,
        labels: args.labels.clone(),
        no_error_on_unsupported: args.no_error_on_unsupported,
        charset: args.charset.clone(),
    }
    .validate()
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

    use super::{run, run_for_paths, runtime_config_from_scan_args};
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

    #[test]
    fn translates_cli_scan_args_to_runtime_config() {
        let config = runtime_config_from_scan_args(&ScanArgs {
            blame: true,
            charset: String::from("detect"),
            excludes: vec![String::from("target")],
            exclude_dirs: vec![String::from("vendor")],
            exclude_hidden: true,
            follow: true,
            ignore_file_names: vec![String::from(".ignore")],
            include_vcs: true,
            include_generated: true,
            include_vendored: true,
            labels: vec![String::from("owner")],
            no_error_on_unsupported: true,
            output: Some(OutputArg::Json),
            todo_types: Some(String::from("TODO,FIXME")),
            paths: vec![std::path::PathBuf::from(".")],
        })
        .expect("config translates");

        assert_eq!(config.todo_types, ["TODO", "FIXME"]);
        assert_eq!(config.output, crate::core::config::OutputFormat::Json);
        assert!(config.blame);
        assert!(config.follow_symlinks);
        assert_eq!(config.labels, ["owner"]);
    }
}
