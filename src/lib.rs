mod core;

pub mod error;
pub mod json;

use crate::core::parser::parse_raw;
use crate::error::ParseError;
use crate::json::JsValue;

pub fn parse(t: &str) -> Result<JsValue, ParseError> {
    parse_raw(t)
}
