use std::path::{Path, PathBuf};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FixtureCase {
    pub id: &'static str,
    pub root: &'static str,
    pub description: &'static str,
}

pub fn basic_fixture() -> FixtureCase {
    FixtureCase {
        id: "basic-default-scan",
        root: "tests/fixtures/basic",
        description: "Default recursive scan with ignore and inline comments",
    }
}

#[allow(dead_code)]
pub fn unsupported_fixture() -> FixtureCase {
    FixtureCase {
        id: "direct-unsupported",
        root: "tests/fixtures/unsupported",
        description: "Direct unsupported file input",
    }
}

pub fn fixture_root(case: &FixtureCase) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(case.root)
}
