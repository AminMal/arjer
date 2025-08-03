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
    U32(u32),
    Bool(bool),
    Null,
}
