pub mod parser;
pub mod tokenizer;

use crate::json::ast::Num;
pub use parser::parse_tokens;
pub use tokenizer::tokenize;

#[derive(Debug, Clone)]
pub(super) enum Token {
    OBracket,
    CBracket,
    DQuote,
    OCurlyBrace,
    CCurlyBrace,
    Comma,
    Colon,
    Str(String),
    N(Num),
    Bool(bool),
    Null,
}
