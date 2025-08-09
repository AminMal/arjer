use crate::json::{JsValue, Num};

#[allow(dead_code)]
pub struct Cursor<'a> {
    underlying: Option<&'a JsValue>,
    error: Option<String>,
}

impl<'a> Cursor<'a> {
    fn if_matches<P, EF>(
        opt: Option<&JsValue>,
        predicate: P,
        if_doesnt_match: EF,
        if_empty: String,
    ) -> (Option<&JsValue>, Option<String>)
    where
        EF: FnOnce(&JsValue) -> String,
        P: FnOnce(&JsValue) -> bool,
    {
        match &opt {
            Some(t) if predicate(t) => (opt, None),
            Some(t) => (None, Some(if_doesnt_match(t))),
            _ => (None, Some(if_empty)),
        }
    }

    fn if_exists(opt: Option<&JsValue>, if_empty: String) -> (Option<&JsValue>, Option<String>) {
        match &opt {
            Some(_) => (opt, None),
            _ => (None, Some(if_empty)),
        }
    }

    fn map<F>(self, f: F) -> Self
    where
        F: FnOnce(&'a JsValue) -> (Option<&'a JsValue>, Option<String>),
    {
        match self {
            Self { error: Some(_), .. } => self,
            Self {
                underlying: Some(j),
                ..
            } => {
                let (new_j, e) = f(j);
                Self {
                    underlying: new_j,
                    error: e,
                }
            }
            _ => Self {
                underlying: None,
                error: Some("illegal state".into()),
            },
        }
    }

    pub fn obj<S: Into<String>>(self, path: S) -> Self {
        let p = path.into();
        self.map(|j| match j {
            JsValue::JsObject(key_values) => Self::if_matches(
                key_values.get(&p),
                JsValue::is_obj,
                |inner| format!("{:?} is not an object", inner),
                format!("no such element: {}", p),
            ),
            other => (None, Some(format!("{:?} is not an object", other))),
        })
    }

    pub fn arr(self, path: String) -> Self {
        self.map(|j| match j {
            JsValue::JsObject(key_values) => Self::if_matches(
                key_values.get(&path),
                JsValue::is_array,
                |inner| format!("{:?} is not an array", inner),
                format!("no such element: {}", path),
            ),
            other => (None, Some(format!("{:?} is not an object", other))),
        })
    }

    pub fn nth(self, n: usize) -> Self {
        self.map(|j| match j {
            JsValue::JsArray(elems) => {
                Self::if_exists(elems.get(n), format!("index {} out of bounds", n))
            }
            other => (None, Some(format!("{:?} is not an object", other))),
        })
    }

    pub fn string<S: Into<String>>(self, path: S) -> Result<String, String> {
        let p = path.into();
        let result = self
            .map(|j| match j {
                JsValue::JsObject(key_values) => Self::if_matches(
                    key_values.get(&p),
                    JsValue::is_str,
                    |inner| format!("{:?} is not a string", inner),
                    format!("no such element: {}", p),
                ),
                other => (None, Some(format!("{:?} is not an object", other))),
            })
            .get();
        match result {
            Ok(JsValue::JsString(s)) => Ok(s.clone()),
            Ok(other) => Err(format!("{:?} is not a string", other)),
            Err(e) => Err(e),
        }
    }

    pub fn boolean<S: Into<String>>(self, path: S) -> Result<bool, String> {
        let p = path.into();
        let result = self
            .map(|j| match j {
                JsValue::JsObject(key_values) => Self::if_matches(
                    key_values.get(&p),
                    JsValue::is_bool,
                    |inner| format!("{:?} is not a bool", inner),
                    format!("no such element: {}", p),
                ),
                other => (None, Some(format!("{:?} is not an object", other))),
            })
            .get();
        match result {
            Ok(JsValue::JsBool(b)) => Ok(b.clone()),
            Ok(other) => Err(format!("{:?} is not a bool", other)),
            Err(e) => Err(e),
        }
    }

    pub fn num_u32<S: Into<String>>(self, path: S) -> Result<u32, String> {
        let p = path.into();
        let result = self
            .map(|j| match j {
                JsValue::JsObject(key_values) => Self::if_matches(
                    key_values.get(&p),
                    JsValue::is_num_u32,
                    |inner| format!("{:?} is not a u32", inner),
                    format!("no such element: {}", p),
                ),
                other => (None, Some(format!("{:?} is not an object", other))),
            })
            .get();
        match result {
            Ok(JsValue::JsNumber(Num::U32(n))) => Ok(n.clone()),
            Ok(other) => Err(format!("{:?} is not a u32", other)),
            Err(e) => Err(e),
        }
    }

    pub fn num_u64<S: Into<String>>(self, path: S) -> Result<u64, String> {
        let p = path.into();
        let result = self
            .map(|j| match j {
                JsValue::JsObject(key_values) => Self::if_matches(
                    key_values.get(&p),
                    JsValue::is_num_u64,
                    |inner| format!("{:?} is not a u64", inner),
                    format!("no such element: {}", p),
                ),
                other => (None, Some(format!("{:?} is not an object", other))),
            })
            .get();
        match result {
            Ok(JsValue::JsNumber(Num::U64(n))) => Ok(n.clone()),
            Ok(other) => Err(format!("{:?} is not a u64", other)),
            Err(e) => Err(e),
        }
    }

    pub fn num_f64<S: Into<String>>(self, path: S) -> Result<f64, String> {
        let p = path.into();
        let result = self
            .map(|j| match j {
                JsValue::JsObject(key_values) => Self::if_matches(
                    key_values.get(&p),
                    JsValue::is_num_f64,
                    |inner| format!("{:?} is not a f64", inner),
                    format!("no such element: {}", p),
                ),
                other => (None, Some(format!("{:?} is not an object", other))),
            })
            .get();
        match result {
            Ok(JsValue::JsNumber(Num::F(n))) => Ok(n.clone()),
            Ok(other) => Err(format!("{:?} is not a f64", other)),
            Err(e) => Err(e),
        }
    }

    pub fn get(self) -> Result<&'a JsValue, String> {
        match self {
            Self {
                underlying: Some(j),
                ..
            } => Ok(j),
            Self { error: Some(e), .. } => Err(e),
            _ => Err("illegal state of cursor".into()),
        }
    }
}

impl JsValue {
    pub fn cursor(&self) -> Cursor {
        Cursor {
            underlying: Some(self),
            error: None,
        }
    }

    pub fn is_obj(&self) -> bool {
        matches!(self, JsValue::JsObject(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, JsValue::JsArray(_))
    }

    pub fn is_str(&self) -> bool {
        matches!(self, JsValue::JsString(_))
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, JsValue::JsBool(_))
    }

    pub fn is_num_u32(&self) -> bool {
        matches!(self, JsValue::JsNumber(Num::U32(_)))
    }

    pub fn is_num_u64(&self) -> bool {
        matches!(self, JsValue::JsNumber(Num::U64(_)))
    }

    pub fn is_num_f64(&self) -> bool {
        matches!(self, JsValue::JsNumber(Num::F(_)))
    }

    fn indent_impl(&self, s: &str, level: usize) -> String {
        let space_inner = s.repeat(level);
        let space_outer = s.repeat(level - 1);
        match self {
            JsValue::JsNull => String::from("null"),
            JsValue::JsString(s) => format!("\"{}\"", s),
            JsValue::JsNumber(Num::U32(u)) => u.to_string(),
            JsValue::JsNumber(Num::U64(u)) => u.to_string(),
            JsValue::JsNumber(Num::F(f)) => f.to_string(),
            JsValue::JsBool(b) => b.to_string(),
            JsValue::JsObject(obj) => {
                if obj.is_empty() {
                    String::from("{}")
                } else {
                    let inner = obj
                        .iter()
                        .map(|(k, v)| {
                            format!(
                                "\n{}\"{}\": {}",
                                space_inner,
                                k,
                                v.indent_impl(s, level + 1)
                            )
                        })
                        .collect::<Vec<_>>()
                        .join(",");
                    format!("{}{}\n{}{}", "{", inner, space_outer, "}")
                }
            }
            JsValue::JsArray(arr) => {
                if arr.is_empty() {
                    String::from("[]")
                } else {
                    let inner = arr
                        .iter()
                        .map(|v| format!("\n{}{}", space_inner, v.indent_impl(s, level + 1)))
                        .collect::<Vec<_>>()
                        .join(",");
                    format!("[{}\n{}]", inner, space_outer)
                }
            }
        }
    }

    pub fn pretty_print(&self) -> String {
        self.indent_impl("  ", 1)
    }

    pub fn indent(&self, space: &str) -> String {
        self.indent_impl(space, 1)
    }
}
