use crate::ast::*;
use crate::lexer::*;

use thiserror::Error;

/// `mdast` associated error type.
#[derive(Error, Debug)]
pub enum ParserError {
    #[error("mdast error {0}")]
    AstError(#[from] AstError),
}

/// Markdown text stream parser.
pub struct Parser<'a> {
    _lexer: Lexer<'a>,
}

impl<'a, L> From<L> for Parser<'a>
where
    L: Into<Lexer<'a>>,
{
    fn from(value: L) -> Self {
        Self::new(value)
    }
}

impl<'a> Parser<'a> {
    /// Create new parser from lexer implementation
    pub fn new<L: Into<Lexer<'a>>>(l: L) -> Self {
        Parser { _lexer: l.into() }
    }

    /// Parse input markdown text stream.
    pub fn parse(&mut self) -> Result<Document<'a>, ParserError> {
        let mut document = Document::default();

        loop {
            if let Some(node) = self.parse_flow_content()? {
                document.add_child(node)?;
            } else {
                return Ok(document);
            }
        }
    }

    fn parse_flow_content(&mut self) -> Result<Option<Node<'a>>, ParserError> {
        let token = self._lexer.next_token();

        match token {
            Token::Eof(_) => return Ok(None),
            _ => {
                unimplemented!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;

    #[test]
    fn test_heading() {
        let md = "# heading";

        let mut parser: Parser = md.into();

        parser.parse().unwrap();
    }
}
