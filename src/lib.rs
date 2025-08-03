pub mod json;
mod core;

pub mod parser {
    use crate::core::{parse_tokens, tokenize};
    use crate::json::JsValue;

    pub fn parse<T: Into<String>>(t: T) -> Result<JsValue, String> {
        parse_tokens(tokenize(t.into())?)
    }
}