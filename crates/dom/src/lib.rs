/// Juniper DOM (RDOM) minimal stubs.
use juniper_parser::AST;

#[derive(Debug, Clone)]
pub struct RdomNode {
    pub id: usize,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct Rdom {
    pub nodes: Vec<RdomNode>,
}

/// Build a very small RDOM from an AST.
pub fn build_rdom(ast: &AST) -> Rdom {
    let nodes = ast
        .nodes
        .iter()
        .enumerate()
        .map(|(i, s)| RdomNode {
            id: i,
            text: s.clone(),
        })
        .collect();
    Rdom { nodes }
}

#[cfg(test)]
mod tests {
    use super::*;
    use juniper_parser::parse_source;

    #[test]
    fn build_from_ast() {
        let ast = parse_source("a b c").unwrap();
        let rdom = build_rdom(&ast);
        assert_eq!(rdom.nodes.len(), 3);
        assert_eq!(rdom.nodes[0].text, "a");
    }
}
