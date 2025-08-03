mod core;

pub mod error;
pub mod json;

use crate::core::{parse_tokens, tokenize};
use crate::error::ParseError;
use crate::json::JsValue;

pub fn parse<T: Into<String>>(t: T) -> Result<JsValue, ParseError> {
    parse_tokens(tokenize(t.into())?)
}
