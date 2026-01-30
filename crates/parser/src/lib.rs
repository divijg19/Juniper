//! Minimal parser crate
//!
//! This crate provides a small, well-documented lexer -> CST -> AST pipeline.
//! The goal is to preserve source fidelity in the CST and produce a typed
//! AST suitable for DOM expansion.

/// A byte-range span in the original source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

/// Tokens produced by the lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Word(String, Span),
    Punct(char, Span),
    Ref(String, Span), // e.g. @ref
    Eof(Span),
}

/// Concrete Syntax Tree preserves token sequence and original source.
#[derive(Debug, Clone)]
pub struct CST {
    pub source: String,
    pub tokens: Vec<Token>,
}

/// Inline AST nodes (words, references).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstInline {
    Word(String),
    Reference(String),
}

/// Block-level AST: paragraphs containing inline nodes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstBlock {
    Paragraph(Vec<AstInline>),
}

/// AST is a sequence of block-level elements.
#[derive(Debug, Clone)]
pub struct AST {
    pub blocks: Vec<AstBlock>,
}

/// Error returned by the parser or lexer.
#[derive(Debug, Clone)]
pub struct ParseError(pub String);

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error: {}", self.0)
    }
}
impl std::error::Error for ParseError {}

/// Very small lexer: splits into words, punctuation and @references.
pub fn lex(source: &str) -> CST {
    let mut tokens = Vec::new();
    let s = source;
    let mut i = 0usize;
    while i < s.len() {
        let ch = s.as_bytes()[i] as char;
        if ch.is_whitespace() {
            i += 1;
            continue;
        }
        if ch == '@' {
            // reference: collect following word characters
            let start = i;
            i += 1;
            let mut j = i;
            while j < s.len() {
                let c = s.as_bytes()[j] as char;
                if c.is_alphanumeric() || c == '_' || c == '-' {
                    j += 1;
                } else {
                    break;
                }
            }
            let val = s[start + 1..j].to_string();
            tokens.push(Token::Ref(val, Span { start, end: j }));
            i = j;
            continue;
        }
        if ch.is_ascii_punctuation() {
            let start = i;
            tokens.push(Token::Punct(ch, Span { start, end: i + 1 }));
            i += 1;
            continue;
        }
        // word
        let start = i;
        let mut j = i;
        while j < s.len() {
            let c = s.as_bytes()[j] as char;
            if c.is_whitespace() || c.is_ascii_punctuation() {
                break;
            }
            j += 1;
        }
        let val = s[start..j].to_string();
        tokens.push(Token::Word(val, Span { start, end: j }));
        i = j;
    }
    tokens.push(Token::Eof(Span {
        start: s.len(),
        end: s.len(),
    }));
    CST {
        source: s.to_string(),
        tokens,
    }
}

/// Lower CST into a typed AST. This step performs simple classification only.
pub fn lower(cst: &CST) -> AST {
    // Group tokens into paragraphs separated by empty tokens (Eof or punctuation
    // that is a newline). Since lexer doesn't produce explicit newlines, we use
    // a simple heuristic: treat consecutive Word/Ref tokens and Punct('\n')
    // are not present; instead split by double newlines in source.
    let mut blocks = Vec::new();
    for para in cst.source.split("\n\n") {
        let mut inlines = Vec::new();
        let mut i = 0usize;
        while i < para.len() {
            // reuse a tiny inner lexer on the paragraph string
            let ch = para.as_bytes()[i] as char;
            if ch.is_whitespace() {
                i += 1;
                continue;
            }
            if ch == '@' {
                let start = i;
                i += 1;
                let mut j = i;
                while j < para.len() {
                    let c = para.as_bytes()[j] as char;
                    if c.is_alphanumeric() || c == '_' || c == '-' {
                        j += 1;
                    } else {
                        break;
                    }
                }
                let val = para[start + 1..j].to_string();
                inlines.push(AstInline::Reference(val));
                i = j;
                continue;
            }
            if ch.is_ascii_punctuation() {
                i += 1;
                continue;
            }
            let start = i;
            let mut j = i;
            while j < para.len() {
                let c = para.as_bytes()[j] as char;
                if c.is_whitespace() || c.is_ascii_punctuation() {
                    break;
                }
                j += 1;
            }
            let val = para[start..j].to_string();
            inlines.push(AstInline::Word(val));
            i = j;
        }
        blocks.push(AstBlock::Paragraph(inlines));
    }
    AST { blocks }
}

/// Convenience: parse source text to AST (lexer + lowering).
pub fn parse_source(source: &str) -> Result<AST, ParseError> {
    let cst = lex(source);
    Ok(lower(&cst))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_and_lower() {
        let text = "Hello, @ref world!";
        let cst = lex(text);
        assert!(matches!(cst.tokens.first().unwrap(), Token::Word(_, _)));
        let ast = lower(&cst);
        match &ast.blocks[0] {
            AstBlock::Paragraph(inlines) => {
                assert!(matches!(inlines[0], AstInline::Word(ref w) if w == "Hello"));
                assert!(matches!(inlines[1], AstInline::Reference(ref r) if r == "ref"));
            }
        }
    }
}
