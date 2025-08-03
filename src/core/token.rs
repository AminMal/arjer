use crate::json::ast::Num;

#[derive(Debug, Clone)]
pub enum Token {
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
