use std::collections::BTreeMap;

#[derive(Debug, Default, Clone)]
pub struct OverlayStore {
    entries: BTreeMap<String, OverlayEntry>,
}

#[derive(Debug, Clone)]
pub struct OverlayEntry {
    pub text: String,
    pub version: i32,
}

impl OverlayStore {
    pub fn open(&mut self, uri: String, text: String, version: i32) {
        self.entries.insert(uri, OverlayEntry { text, version });
    }

    pub fn get(&self, uri: &str) -> Option<&OverlayEntry> {
        self.entries.get(uri)
    }

    pub fn close(&mut self, uri: &str) {
        self.entries.remove(uri);
    }
}

#[cfg(test)]
mod tests {
    use super::OverlayStore;

    #[test]
    fn stores_and_removes_entries() {
        let mut overlays = OverlayStore::default();
        overlays.open(
            String::from("file:///tmp/sample.rs"),
            String::from("todo"),
            7,
        );
        let entry = overlays.get("file:///tmp/sample.rs").expect("entry exists");
        assert_eq!(entry.text, "todo");
        assert_eq!(entry.version, 7);

        overlays.close("file:///tmp/sample.rs");
        assert!(overlays.get("file:///tmp/sample.rs").is_none());
    }
}
