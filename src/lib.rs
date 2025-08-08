mod core;

pub mod error;
pub mod json;

use crate::core::{parse_tokens, tokenize};
use crate::error::ParseError;
use crate::json::JsValue;

pub fn parse(t: &str) -> Result<JsValue, ParseError> {
    parse_tokens(tokenize(t)?)
}
