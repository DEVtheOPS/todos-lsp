use thiserror::Error;

#[derive(Debug, Error)]
pub enum TodoError {
    #[error("invalid config: {0}")]
    InvalidConfig(String),

    #[error("missing runtime dependency: {0}")]
    MissingRuntimeDependency(String),

    #[error("unsupported input: {0}")]
    UnsupportedInput(String),

    #[error("i/o error: {0}")]
    Io(String),

    #[error("serialization error: {0}")]
    Serialization(String),
}

impl From<std::io::Error> for TodoError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value.to_string())
    }
}

impl From<serde_json::Error> for TodoError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serialization(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::TodoError;

    #[test]
    fn converts_io_and_json_errors() {
        let io_error = std::io::Error::other("boom");
        let todo_error = TodoError::from(io_error);
        assert!(matches!(todo_error, TodoError::Io(_)));

        let json_error = serde_json::from_str::<serde_json::Value>("{").expect_err("invalid json");
        let todo_error = TodoError::from(json_error);
        assert!(matches!(todo_error, TodoError::Serialization(_)));
    }
}
