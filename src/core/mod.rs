pub mod parser;
pub mod tokenizer;

pub use parser::parse_tokens;
pub use tokenizer::tokenize;
use crate::json::ast::Num;

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

