use crate::error::ParseError;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum Num {
    U32(u32),
    U64(u64),
    F(f64),
}

impl TryFrom<String> for Num {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.contains(".") {
            value
                .parse::<f64>()
                .map(|f| Self::F(f))
                .map_err(|_| ParseError::InvalidNumber {
                    tpe: "f64".into(),
                    value: value.clone(),
                })
        } else {
            value
                .parse::<u32>()
                .map(|u| Self::U32(u))
                .map_err(|_| ParseError::InvalidNumber {
                    tpe: "u32".into(),
                    value: value.clone(),
                })
                .or(value.parse::<u64>().map(|u| Self::U64(u)).map_err(|_| {
                    ParseError::InvalidNumber {
                        tpe: "u64".into(),
                        value: value.clone(),
                    }
                }))
        }
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum JsValue {
    JsNull,
    JsString(String),
    JsNumber(Num),
    JsBool(bool),
    JsObject(HashMap<String, JsValue>),
    JsArray(Vec<JsValue>),
}
