use lsp_types::{
    Diagnostic, DiagnosticSeverity, Location, OneOf, Position, Range, SymbolInformation, SymbolKind,
};

use crate::core::model::Finding;

pub fn to_diagnostic(finding: &Finding) -> Diagnostic {
    Diagnostic {
        range: Range {
            start: Position::new(finding.range.start.line, finding.range.start.character),
            end: Position::new(finding.range.end.line, finding.range.end.character),
        },
        severity: Some(DiagnosticSeverity::WARNING),
        source: Some(String::from("todos-lsp")),
        message: finding
            .message
            .clone()
            .unwrap_or_else(|| finding.todo_type.clone()),
        ..Diagnostic::default()
    }
}

pub fn to_workspace_symbol(finding: &Finding) -> SymbolInformation {
    SymbolInformation {
        name: finding
            .message
            .clone()
            .unwrap_or_else(|| finding.raw_text.clone()),
        kind: SymbolKind::EVENT,
        tags: None,
        location: Location {
            uri: finding.uri.parse().expect("valid URI"),
            range: Range {
                start: Position::new(finding.range.start.line, finding.range.start.character),
                end: Position::new(finding.range.end.line, finding.range.end.character),
            },
        },
        container_name: Some(finding.todo_type.clone()),
        #[allow(deprecated)]
        deprecated: None,
    }
}

pub fn to_workspace_symbol_response(
    symbol: &SymbolInformation,
) -> OneOf<SymbolInformation, lsp_types::WorkspaceSymbol> {
    OneOf::Left(symbol.clone())
}

#[cfg(test)]
mod tests {
    use super::{to_workspace_symbol, to_workspace_symbol_response};
    use crate::core::model::{Finding, SourceKind};

    #[test]
    fn wraps_symbol_information_for_workspace_response() {
        let finding = Finding::new(
            std::path::PathBuf::from("src/lib.rs"),
            String::from("file:///tmp/src/lib.rs"),
            0,
            0,
            String::from("// TODO: response"),
            String::from("TODO"),
            None,
            Some(String::from("response")),
            SourceKind::Disk,
        );

        let symbol = to_workspace_symbol(&finding);
        let wrapped = to_workspace_symbol_response(&symbol);
        assert!(matches!(wrapped, lsp_types::OneOf::Left(_)));
    }
}
