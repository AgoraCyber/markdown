/// [mdast](https://github.com/syntax-tree/mdast#list) implementation
pub mod ast;

/// Transform markdown document stream to token stream.
pub mod lexer;

/// Consume [`lexer`](lexer::Lexer) output and generate [mdast](https://github.com/syntax-tree/mdast#list)
pub mod parser;
