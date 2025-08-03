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
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        dbg!(&value);
        if value.contains(".") {
            value
                .parse::<f64>()
                .map(|f| Self::F(f))
                .map_err(|e| format!("{}", e))
        } else {
            value
                .parse::<u32>()
                .map(|u| Self::U32(u))
                .map_err(|e| format!("{}", e))
                .or(value
                    .parse::<u64>()
                    .map(|u| Self::U64(u))
                    .map_err(|e| format!("{}", e)))
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
