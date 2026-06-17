use std::path::{Path, PathBuf};

use glob::Pattern;
use ignore::WalkBuilder;

use crate::core::config::RuntimeConfig;
use crate::core::errors::TodoError;

pub fn discover_inputs(
    root: &Path,
    explicit_paths: &[PathBuf],
    config: &RuntimeConfig,
) -> Result<Vec<PathBuf>, TodoError> {
    let mut discovered = Vec::new();

    for path in explicit_paths {
        let candidate = if path.is_absolute() {
            path.clone()
        } else {
            root.join(path)
        };
        let explicit_symlink = candidate
            .symlink_metadata()
            .map(|metadata| metadata.file_type().is_symlink())
            .unwrap_or(false);

        if candidate.is_file() || candidate.symlink_metadata()?.file_type().is_symlink() {
            if is_supported_path(&candidate) {
                discovered.push(candidate.canonicalize().unwrap_or(candidate));
            } else if !config.no_error_on_unsupported {
                return Err(TodoError::UnsupportedInput(candidate.display().to_string()));
            }
            continue;
        }

        let dir = if candidate.exists() {
            candidate
        } else {
            root.to_path_buf()
        };
        let mut builder = WalkBuilder::new(&dir);
        builder.hidden(false);
        builder.follow_links(config.follow_symlinks || explicit_symlink);
        for name in &config.ignore_file_names {
            builder.add_custom_ignore_filename(name);
        }

        for entry in builder.build() {
            let entry = entry.map_err(|error| TodoError::Io(error.to_string()))?;
            let path = entry.path();
            if entry.file_type().is_some_and(|kind| kind.is_dir()) && should_skip_dir(path, config)
            {
                continue;
            }
            if !entry.file_type().is_some_and(|kind| kind.is_file()) {
                continue;
            }
            if should_skip_path(path, config) {
                continue;
            }
            if is_supported_path(path) {
                discovered.push(path.to_path_buf());
            }
        }
    }

    discovered.sort();
    discovered.dedup();
    Ok(discovered)
}

pub fn is_supported_path(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };

    if matches!(
        name,
        "Makefile" | ".editorconfig" | "Dockerfile" | "Containerfile"
    ) {
        return true;
    }

    if matches!(name, ".gitignore" | ".gitconfig") {
        return true;
    }

    matches!(
        path.extension().and_then(|extension| extension.to_str()),
        Some(
            "rs" | "js"
                | "ts"
                | "tsx"
                | "jsx"
                | "c"
                | "dart"
                | "erb"
                | "ex"
                | "exs"
                | "h"
                | "go"
                | "graphql"
                | "gql"
                | "graphqls"
                | "java"
                | "kt"
                | "lua"
                | "py"
                | "rb"
                | "sh"
                | "bash"
                | "html"
                | "htm"
                | "tf"
                | "hcl"
                | "yaml"
                | "yml"
                | "toml"
                | "ini"
                | "cfg"
                | "conf"
                | "json5"
                | "svelte"
                | "md"
        )
    )
}

pub fn is_generated_path(path: &Path) -> bool {
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    name.contains(".gen.") || name.contains("generated")
}

fn should_skip_dir(path: &Path, config: &RuntimeConfig) -> bool {
    let name = path
        .file_name()
        .and_then(|part| part.to_str())
        .unwrap_or_default();
    if !config.include_vcs && matches!(name, ".git" | ".hg" | ".svn") {
        return true;
    }
    if !config.include_vendored && matches!(name, "vendor" | "node_modules" | "third_party") {
        return true;
    }
    if config.exclude_hidden && name.starts_with('.') {
        return true;
    }

    config
        .exclude_dirs
        .iter()
        .filter_map(|glob| Pattern::new(glob).ok())
        .any(|pattern| pattern.matches(name))
}

fn should_skip_path(path: &Path, config: &RuntimeConfig) -> bool {
    let name = path
        .file_name()
        .and_then(|part| part.to_str())
        .unwrap_or_default();
    if config.exclude_hidden && name.starts_with('.') {
        return true;
    }
    if !config.include_generated && is_generated_path(path) {
        return true;
    }
    config
        .excludes
        .iter()
        .filter_map(|glob| Pattern::new(glob).ok())
        .any(|pattern| pattern.matches(name) || pattern.matches_path(path))
}
