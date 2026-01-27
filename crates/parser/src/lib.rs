/// Juniper parser: very small stubs for CST and AST and a parse function.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CST {
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AST {
    pub nodes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ParseError(pub String);

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error: {}", self.0)
    }
}

impl std::error::Error for ParseError {}

/// Parse source text into an AST (via a CST). This is a stubbed parser.
pub fn parse_source(source: &str) -> Result<AST, ParseError> {
    let cst = CST {
        source: source.to_string(),
    };
    // trivial lowering: split on whitespace
    let nodes = cst
        .source
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    Ok(AST { nodes })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple() {
        let ast = parse_source("hello world").unwrap();
        assert_eq!(ast.nodes, vec!["hello".to_string(), "world".to_string()]);
    }
}
