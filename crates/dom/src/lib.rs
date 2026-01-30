//! Juniper DOM (RDOM) minimal implementation.
//!
//! This crate demonstrates how the parser AST is expanded into a small
//! Reactive Document Object Model. Each node records lightweight dependencies
//! (by index) discovered from `@reference` AST nodes.
use juniper_parser::{AST, AstNode};

/// A node in the RDOM. `deps` contains indices of other RDOM nodes this node
/// depends on (empty when none).
#[derive(Debug, Clone)]
pub struct RdomNode {
    pub id: usize,
    pub text: String,
    pub deps: Vec<usize>,
}

/// The reactive document model: an ordered list of nodes.
#[derive(Debug, Clone)]
pub struct Rdom {
    pub nodes: Vec<RdomNode>,
}

/// Build a small RDOM from an AST.
///
/// Strategy: create one `RdomNode` per AST word. When an `AstNode::Reference`
/// is seen, resolve it to a prior node with matching text (if any) and add
/// a dependency edge. This is intentionally simplified for bootstrapping.
pub fn build_rdom(ast: &AST) -> Rdom {
    let mut nodes: Vec<RdomNode> = Vec::new();
    for (i, n) in ast.nodes.iter().enumerate() {
        match n {
            AstNode::Word(w) => {
                nodes.push(RdomNode { id: i, text: w.clone(), deps: Vec::new() });
            }
            AstNode::Reference(r) => {
                // find prior node with matching text
                let mut dep = None;
                for pn in &nodes {
                    if pn.text == *r {
                        dep = Some(pn.id);
                        break;
                    }
                }
                let mut deps = Vec::new();
                if let Some(d) = dep { deps.push(d); }
                nodes.push(RdomNode { id: i, text: format!("@{}", r), deps });
            }
        }
    }
    Rdom { nodes }
}

#[cfg(test)]
mod tests {
    use super::*;
    use juniper_parser::parse_source;

    #[test]
    fn build_from_ast() {
        let ast = parse_source("a @a b").unwrap();
        let rdom = build_rdom(&ast);
        assert_eq!(rdom.nodes.len(), 3);
        assert_eq!(rdom.nodes[1].deps.len(), 1);
        assert_eq!(rdom.nodes[1].deps[0], 0);
    }
}
