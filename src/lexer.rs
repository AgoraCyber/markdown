/// Transformer for markdown token stream.
pub struct Lexer {
    /// Markdown source stream.
    _source: String,
}

impl Lexer {
    /// Create new [`Lexer`] from source `S`
    pub fn new<S: ToOwned<Owned = String>>(source: S) -> Lexer {
        Lexer {
            _source: source.to_owned(),
        }
    }

    /// Move lexer cursor to next token.
    pub fn next() -> Token {
        unimplemented!()
    }
}

/// Markdown token variant.
pub enum Token {
    /// Token signs(#)
    Pounds(Range),
    /// End of the input markdown text stream.
    Eof(Range),
}

/// Token range in source markdown stream.
#[derive(Debug, Clone, Default)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}
