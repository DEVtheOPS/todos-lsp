use std::path::PathBuf;

use lsp_types::{Diagnostic, SymbolInformation};
use serde_json::json;

use crate::core::config::{resolve_workspace_root, RuntimeConfig};
use crate::core::errors::TodoError;
use crate::core::index::FindingIndex;
use crate::core::model::{DiskScanRequest, TextScanRequest};
use crate::core::scan;
use crate::lsp::commands::LIST_TODOS_COMMAND;
use crate::lsp::overlay::OverlayStore;
use crate::lsp::translate;

#[derive(Debug, Clone, Default)]
pub struct SessionState {
    pub config: RuntimeConfig,
    pub workspace_roots: Vec<PathBuf>,
    pub index: FindingIndex,
    pub overlays: OverlayStore,
}

impl SessionState {
    pub fn initialize(&mut self, roots: Vec<PathBuf>) -> Result<(), TodoError> {
        self.workspace_roots = roots;
        self.refresh_workspace()
    }

    pub fn refresh_workspace(&mut self) -> Result<(), TodoError> {
        let mut findings = Vec::new();
        for root in &self.workspace_roots {
            let request = DiskScanRequest {
                root: root.clone(),
                paths: vec![PathBuf::from(".")],
                config: self.config.clone(),
            };
            findings.extend(scan::scan_disk(&request)?);
        }
        self.index.replace_disk(findings);
        Ok(())
    }

    pub fn open_document(
        &mut self,
        uri: String,
        text: String,
        version: i32,
    ) -> Result<Vec<Diagnostic>, TodoError> {
        self.overlays.open(uri.clone(), text.clone(), version);
        self.reindex_overlay(uri, text)
    }

    pub fn change_document(
        &mut self,
        uri: String,
        text: String,
        version: i32,
    ) -> Result<Vec<Diagnostic>, TodoError> {
        self.overlays.open(uri.clone(), text.clone(), version);
        self.reindex_overlay(uri, text)
    }

    pub fn close_document(&mut self, uri: &str) -> Vec<Diagnostic> {
        self.overlays.close(uri);
        self.index.clear_overlay(uri);
        self.diagnostics(uri)
    }

    pub fn diagnostics(&self, uri: &str) -> Vec<Diagnostic> {
        self.index
            .findings_for_uri(uri)
            .iter()
            .map(translate::to_diagnostic)
            .collect()
    }

    pub fn workspace_symbols(&self, query: &str) -> Vec<SymbolInformation> {
        self.index
            .all_findings()
            .into_iter()
            .filter(|finding| {
                query.is_empty()
                    || finding.raw_text.contains(query)
                    || finding
                        .message
                        .as_ref()
                        .is_some_and(|message| message.contains(query))
            })
            .map(|finding| translate::to_workspace_symbol(&finding))
            .collect()
    }

    pub fn execute_command(&self, command: &str) -> Result<serde_json::Value, TodoError> {
        match command {
            LIST_TODOS_COMMAND => Ok(json!(self.index.all_findings())),
            _ => Err(TodoError::InvalidConfig(format!(
                "unknown command: {command}"
            ))),
        }
    }

    fn reindex_overlay(&mut self, uri: String, text: String) -> Result<Vec<Diagnostic>, TodoError> {
        let path = url::Url::parse(&uri)
            .ok()
            .and_then(|uri| uri.to_file_path().ok())
            .unwrap_or_default();
        let request = TextScanRequest {
            uri: uri.clone(),
            text,
            language_hint: path
                .file_name()
                .and_then(|name| name.to_str())
                .map(String::from),
            config: self.config.clone(),
            workspace_root: Some(self.workspace_root_for_path(&path)),
        };
        let findings = scan::scan_text(&request)?;
        self.index.upsert_overlay(uri.clone(), findings);
        Ok(self.diagnostics(&uri))
    }

    fn workspace_root_for_path(&self, path: &std::path::Path) -> PathBuf {
        self.workspace_roots
            .iter()
            .find(|root| path.starts_with(root))
            .cloned()
            .unwrap_or_else(|| resolve_workspace_root(path))
    }
}
