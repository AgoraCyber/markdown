use std::{ops::Range, str::Chars};

use crate::ast::AlignType;

const KEYCHARS: &[char] = [
    '\\', '`', '*', '_', '{', '}', '[', ']', '(', ')', '#', '+', '-', '.', '!', '|', '>', '<',
]
.as_slice();

const WHITESPACECHARS: &[char] = [' ', '\t'].as_slice();

const LINEBREAKCHARS: &[char] = ['\r', '\n'].as_slice();

/// Transformer for markdown token stream.
#[derive(Debug)]
pub struct Lexer<'a> {
    /// Markdown source stream.
    _source: &'a str,
    /// Source chars iterator
    _iter: Chars<'a>,
    /// Lookahead cached next token instance.
    _lookahead: Option<Token>,
}

impl<'a> Lexer<'a> {
    /// Create new [`Lexer`] from source `S`
    pub fn new(source: &'a str) -> Self {
        Lexer {
            _source: source,
            _lookahead: None,
            _iter: source.chars(),
        }
    }

    /// Rollback lexer cursor to `token` start offset
    pub fn rollback_to<T: AsRef<Token> + ToOwned<Owned = Token>>(&mut self, token: T) {
        let range = token.as_ref().to_range();

        assert!(range.start < self._source.len());
        assert!(range.end < self._source.len());

        self._iter = self._source[range.start..].chars();
        self._lookahead = Some(token.to_owned());
    }

    /// Move lexer cursor to next token.
    pub fn next_token(&mut self) -> Token {
        if let Some(token) = self._lookahead.take() {
            return token;
        }

        let start = self.offset();

        if let Some(c) = self._iter.next() {
            match c {
                '#' => return self.read_pounds(start),
                '*' => return self.read_asterisks(start),
                '+' => return self.read_pluses(start),
                '-' => return self.read_align_type_or_plaintext(start, false),
                ':' => {
                    let token = self.read_align_type_or_plaintext(start, true);

                    if let Token::PlainText(range) = &token {
                        let next_token = self.lookahead();

                        if let Token::PlainText(next_range) = next_token {
                            self.next_token();

                            return Token::PlainText(range.start..next_range.end);
                        } else {
                            return token;
                        }
                    } else {
                        return token;
                    }
                }
                '_' => return self.read_underscores(start),
                '`' => return self.read_backticks(start),
                ' ' | '\t' => return self.read_whitespaces(start),
                '\r' | '\n' => return self.read_linebreaks(start),
                _ => {
                    // if this is a key char
                    if KEYCHARS.contains(&c) {
                        return Token::KeyChar(start..start + 1);
                    }

                    return self.read_plaintext(start);
                }
            }
        } else {
            let offset = self._source.len();
            return Token::Eof(offset..offset);
        }
    }

    /// read pounds token
    fn read_pounds(&mut self, start: usize) -> Token {
        let range = self.read_until(|c| c == '#');

        return Token::Pounds(start..range.end);
    }

    fn read_align_type_or_plaintext(&mut self, start: usize, has_prefix: bool) -> Token {
        let mut suffix = false;
        let range = self.read_until(|c| {
            if c == '-' {
                return true;
            }

            if !suffix && c == ':' {
                suffix = true;
                return true;
            }

            return false;
        });

        if has_prefix && range.len() > 1 {
            if suffix {
                return Token::Align(start..range.end, AlignType::Center);
            } else {
                return Token::Align(start..range.end, AlignType::Left);
            }
        }

        if !has_prefix && range.len() > 1 {
            if suffix {
                return Token::Align(start..range.end, AlignType::Right);
            } else {
                return Token::Dashes(start..range.end);
            }
        }

        return Token::PlainText(start..range.end);
    }

    fn read_asterisks(&mut self, start: usize) -> Token {
        let range = self.read_until(|c| c == '*');

        return Token::Asterisks(start..range.end);
    }

    fn read_underscores(&mut self, start: usize) -> Token {
        let range = self.read_until(|c| c == '_');

        return Token::Underscores(start..range.end);
    }

    fn read_pluses(&mut self, start: usize) -> Token {
        let range = self.read_until(|c| c == '+');

        return Token::Pluses(start..range.end);
    }

    fn read_backticks(&mut self, start: usize) -> Token {
        let range = self.read_until(|c| c == '`');

        return Token::Backticks(start..range.end);
    }

    fn read_whitespaces(&mut self, start: usize) -> Token {
        let range = self.read_until(|c| WHITESPACECHARS.contains(&c));

        return Token::WhiteSpaces(start..range.end);
    }

    fn read_linebreaks(&mut self, start: usize) -> Token {
        let range = self.read_until(|c| LINEBREAKCHARS.contains(&c));

        return Token::LineBreaks(start..range.end);
    }

    fn read_plaintext(&mut self, start: usize) -> Token {
        let mut escaping = false;

        let range = self.read_until(|c| {
            if KEYCHARS.contains(&c) {
                if c == '\\' && !escaping {
                    escaping = true;
                    return true;
                }

                return escaping;
            }

            if WHITESPACECHARS.contains(&c) {
                return false;
            }

            if LINEBREAKCHARS.contains(&c) {
                return false;
            }

            return true;
        });

        return Token::PlainText(start..range.end);
    }

    /// Parse next token but not moving lexer cursor.
    pub fn lookahead(&mut self) -> Token {
        let token = self.next_token();

        self._lookahead = Some(token.clone());

        token
    }

    /// Convert token to [`&str`](AsRef<str>)
    pub fn token_as_str<T: AsRef<Token>>(&mut self, token: T) -> &str {
        let range = token.as_ref().to_range();

        &self._source[range]
    }

    /// consume chars until `F` returns false
    fn read_until<F>(&mut self, mut f: F) -> Range<usize>
    where
        F: FnMut(char) -> bool,
    {
        let begin = self.offset();

        while let Some(c) = self._iter.next() {
            if !f(c) {
                self._iter = self._source[(self.offset() - 1)..].chars();
                return begin..self.offset();
            }
        }

        return begin..self.offset();
    }

    pub fn offset(&self) -> usize {
        self._source.len() - self._iter.as_str().len()
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();

        if let Token::Eof(_) = token {
            return None;
        } else {
            return Some(token);
        }
    }
}

/// Markdown token variant.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// Token signs(#)
    Pounds(Range<usize>),
    /// End of the input markdown text stream.
    Eof(Range<usize>),
    /// Table align types
    Align(Range<usize>, AlignType),
    /// Blockquote prefix number signs (>)
    GreaterThans(Range<usize>),
    /// number signs(*)
    Asterisks(Range<usize>),
    /// number signs(_)
    Underscores(Range<usize>),
    /// number signs(---)
    Dashes(Range<usize>),
    /// number signs(+)
    Pluses(Range<usize>),
    /// signs(`)
    Backticks(Range<usize>),
    /// [\r,\n]+
    LineBreaks(Range<usize>),
    /// \s+
    WhiteSpaces(Range<usize>),
    /// Escapable key char
    KeyChar(Range<usize>),
    /// Plain text range
    PlainText(Range<usize>),
}

impl Token {
    /// Convert [`Token`] to the [`Range`] object of the source stream.
    pub fn to_range(&self) -> Range<usize> {
        let r = match self {
            Token::Eof(r) => r,
            Token::Pounds(r) => r,
            Token::Align(r, _) => r,
            Token::GreaterThans(r) => r,
            Token::Asterisks(r) => r,
            Token::Underscores(r) => r,
            Token::Dashes(r) => r,
            Token::Backticks(r) => r,
            Token::LineBreaks(r) => r,
            Token::WhiteSpaces(r) => r,
            Token::Pluses(r) => r,
            Token::KeyChar(r) => r,
            Token::PlainText(r) => r,
        };

        return r.clone();
    }
}

impl AsRef<Token> for Token {
    fn as_ref(&self) -> &Token {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::AlignType, lexer::Token};

    use super::Lexer;

    #[test]
    fn test_heading() {
        let md = r#"# Heading
        "#;

        let mut lexer = Lexer::new(md);

        assert_eq!(lexer.next(), Some(Token::Pounds(0..1)));
        assert_eq!(lexer.next(), Some(Token::WhiteSpaces(1..2)));
        assert_eq!(lexer.next(), Some(Token::PlainText(2..9)));
        assert_eq!(lexer.next(), Some(Token::LineBreaks(9..10)));
    }

    #[test]
    fn test_thematic_break() {
        let md = "\n__________\n----------\n***";

        let mut lexer = Lexer::new(md);

        assert_eq!(lexer.next(), Some(Token::LineBreaks(0..1)));
        assert_eq!(lexer.next(), Some(Token::Underscores(1..11)));
        assert_eq!(lexer.next(), Some(Token::LineBreaks(11..12)));
        assert_eq!(lexer.next(), Some(Token::Dashes(12..22)));
        assert_eq!(lexer.next(), Some(Token::LineBreaks(22..23)));
        assert_eq!(lexer.next(), Some(Token::Asterisks(23..26)));

        assert_eq!(md.len(), 26);
    }

    #[test]
    fn test_list() {
        let md = "1. hello";

        let mut lexer = Lexer::new(md);

        assert_eq!(lexer.next(), Some(Token::PlainText(0..1)));
        assert_eq!(lexer.lookahead(), Token::KeyChar(1..2));

        let token = lexer.next().unwrap();

        assert_eq!(lexer.token_as_str(token), ".");

        assert_eq!(lexer.next(), Some(Token::WhiteSpaces(2..3)));
        assert_eq!(lexer.next(), Some(Token::PlainText(3..8)));
    }

    #[test]
    fn test_asterisks() {
        let md = "* hello";

        let mut lexer = Lexer::new(md);

        assert_eq!(lexer.next(), Some(Token::Asterisks(0..1)));

        assert_eq!(lexer.next(), Some(Token::WhiteSpaces(1..2)));
        assert_eq!(lexer.next(), Some(Token::PlainText(2..7)));
    }

    #[test]
    fn test_align_type() {
        let md = ":-- :---: --: ----";

        let mut lexer = Lexer::new(md);

        assert_eq!(lexer.next(), Some(Token::Align(0..3, AlignType::Left)));
        assert_eq!(lexer.next(), Some(Token::WhiteSpaces(3..4)));
        assert_eq!(lexer.next(), Some(Token::Align(4..9, AlignType::Center)));
        assert_eq!(lexer.next(), Some(Token::WhiteSpaces(9..10)));
        assert_eq!(lexer.next(), Some(Token::Align(10..13, AlignType::Right)));
        assert_eq!(lexer.next(), Some(Token::WhiteSpaces(13..14)));
        assert_eq!(lexer.next(), Some(Token::Dashes(14..18)));
    }

    #[test]
    fn test_keychars() {
        let md = "| foo |";
        let mut lexer = Lexer::new(md);
        assert_eq!(lexer.next(), Some(Token::KeyChar(0..1)));
        assert_eq!(lexer.next(), Some(Token::WhiteSpaces(1..2)));
        assert_eq!(lexer.next(), Some(Token::PlainText(2..5)));
        assert_eq!(lexer.next(), Some(Token::WhiteSpaces(5..6)));
        assert_eq!(lexer.next(), Some(Token::KeyChar(6..7)));
    }
}
