use std::collections::HashMap;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum JsValue {
    JsNull,
    JsString(String),
    JsNumber(u32),
    JsBool(bool),
    JsObject(HashMap<String, JsValue>),
    JsArray(Vec<JsValue>),
}
