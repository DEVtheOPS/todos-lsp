use std::collections::BTreeMap;

use crate::core::model::Finding;

#[derive(Debug, Default, Clone)]
pub struct FindingIndex {
    disk: BTreeMap<String, Vec<Finding>>,
    overlay: BTreeMap<String, Vec<Finding>>,
}

impl FindingIndex {
    pub fn upsert_disk(&mut self, uri: String, findings: Vec<Finding>) {
        self.disk.insert(uri, findings);
    }

    pub fn replace_disk(&mut self, findings: Vec<Finding>) {
        self.disk.clear();
        for finding in findings {
            self.disk
                .entry(finding.uri.clone())
                .or_default()
                .push(finding);
        }
    }

    pub fn upsert_overlay(&mut self, uri: String, findings: Vec<Finding>) {
        self.overlay.insert(uri, findings);
    }

    pub fn clear_overlay(&mut self, uri: &str) {
        self.overlay.remove(uri);
    }

    pub fn findings_for_uri(&self, uri: &str) -> Vec<Finding> {
        self.overlay
            .get(uri)
            .cloned()
            .or_else(|| self.disk.get(uri).cloned())
            .unwrap_or_default()
    }

    pub fn all_findings(&self) -> Vec<Finding> {
        let mut merged = self.disk.clone();
        for (uri, findings) in &self.overlay {
            merged.insert(uri.clone(), findings.clone());
        }
        merged.into_values().flatten().collect()
    }
}
