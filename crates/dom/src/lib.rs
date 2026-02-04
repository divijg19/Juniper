//! Juniper DOM (RDOM) minimal implementation.
//!
//! This crate demonstrates how the parser AST is expanded into a small
//! Reactive Document Object Model. Each node records lightweight dependencies
//! (by index) discovered from `@reference` AST nodes.
use juniper_parser::{AST, AstBlock, AstInline};

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
    for (i, block) in ast.blocks.iter().enumerate() {
        match block {
            AstBlock::Paragraph(inlines) => {
                let mut words: Vec<String> = Vec::new();
                let mut deps: Vec<usize> = Vec::new();
                for inline in inlines {
                    match inline {
                        AstInline::Word(w) => words.push(w.clone()),
                        AstInline::Reference(r) => {
                            // attempt to resolve reference to a prior paragraph containing the token
                            for pn in &nodes {
                                if pn.text.split_whitespace().any(|tok| tok == r) {
                                    if !deps.contains(&pn.id) {
                                        deps.push(pn.id);
                                    }
                                    break;
                                }
                            }
                            words.push(format!("@{}", r));
                        }
                    }
                }
                let text = words.join(" ");
                nodes.push(RdomNode { id: i, text, deps });
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
        // use double-newline paragraph separation to produce three paragraphs
        let ast = parse_source("a\n\n@a\n\nb").unwrap();
        let rdom = build_rdom(&ast);
        assert_eq!(rdom.nodes.len(), 3);
        assert_eq!(rdom.nodes[1].deps.len(), 1);
        assert_eq!(rdom.nodes[1].deps[0], 0);
    }

    #[test]
    fn unresolved_forward_reference() {
        // reference to a later paragraph should NOT resolve
        let ast = parse_source("@later\n\nlater").unwrap();
        let rdom = build_rdom(&ast);
        assert_eq!(rdom.nodes.len(), 2);
        assert_eq!(rdom.nodes[0].deps.len(), 0);
    }

    #[test]
    fn multiple_references_and_deps() {
        let ast = parse_source("one two\n\n@one @two").unwrap();
        let rdom = build_rdom(&ast);
        assert_eq!(rdom.nodes.len(), 2);
        // second node should have deps pointing to the first paragraph
        assert_eq!(rdom.nodes[1].deps.len(), 1);
        assert_eq!(rdom.nodes[1].deps[0], 0);
        // and its text should contain both references literalized
        assert!(rdom.nodes[1].text.contains("@one"));
        assert!(rdom.nodes[1].text.contains("@two"));
    }
}
