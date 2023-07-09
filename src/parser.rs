use std::ops::Range;

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
    /// Parse flow content:
    /// Blockquote | Code | Heading | Html | List | ThematicBreak | Content
    fn parse_flow_content(&mut self) -> Result<Option<Node<'a>>, ParserError> {
        let token = self._lexer.next_token();

        match token {
            Token::GreaterThans(range) => return self.parse_block_quote(range).map(Some),
            Token::Backticks(range) => return self.parse_code(range).map(Some),
            Token::Pounds(range) => return self.parse_heading(range).map(Some),
            Token::Eof(_) => return Ok(None),
            _ => {
                unimplemented!()
            }
        }
    }

    fn parse_block_quote(&mut self, _range: Range<usize>) -> Result<Node<'a>, ParserError> {
        unimplemented!()
    }

    fn parse_code(&mut self, _range: Range<usize>) -> Result<Node<'a>, ParserError> {
        unimplemented!()
    }
    /// expect: #* ws plaintext
    fn parse_heading(&mut self, pounds: Range<usize>) -> Result<Node<'a>, ParserError> {
        let expect = self._lexer.lookahead();

        match expect {
            Token::WhiteSpaces(range) => {
                let mut heading = Heading::new(pounds.len());

                if range.len() > 1 {
                    let value = self._lexer.range_as_str(range.start + 1..range.end);
                    heading.add_child(Text {
                        value: value.into(),
                    })?;
                }
                while let Some(content) = self.parse_phrasing_content()? {
                    heading.add_child_node(content)?;
                }
            }
            _ => {}
        }

        unimplemented!()
    }

    fn parse_phrasing_content(&mut self) -> Result<Option<Node<'a>>, ParserError> {
        unimplemented!()
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
