use crate::core::strit::StrIt;
use crate::error::ParseError;
use crate::json::{JsValue, Num};
use std::collections::HashMap;

fn parse_str(i: &mut StrIt) -> Result<String, ParseError> {
    let mut result = String::new();
    while let Some(c) = i.pop() {
        match c {
            b'"' => break,
            b'\\' => {
                let x = i.pop().ok_or(ParseError::EOF)?;
                result.push(x as char);
            }
            other => {
                result.push(other as char);
            }
        }
    }
    Ok(result)
}

fn parse_value(i: &mut StrIt) -> Result<JsValue, ParseError> {
    let head = i.peek().ok_or(ParseError::EOF)?;
    match head {
        b't' => {
            if i.starts_with(&[b't', b'r', b'u', b'e']) {
                i.shift(4);
                Ok(JsValue::JsBool(true))
            } else {
                Err(ParseError::UnexpectedToken {
                    expected: vec!["true".into()],
                    got: i.peek_n(4),
                })
            }
        }
        b'f' => {
            if i.starts_with(&[b'f', b'a', b'l', b's', b'e']) {
                i.shift(5);
                Ok(JsValue::JsBool(false))
            } else {
                Err(ParseError::UnexpectedToken {
                    expected: vec!["false".into()],
                    got: i.peek_n(5),
                })
            }
        }
        b'n' => {
            if i.starts_with(&[b'n', b'u', b'l', b'l']) {
                i.shift(4);
                Ok(JsValue::JsNull)
            } else {
                Err(ParseError::UnexpectedToken {
                    expected: vec!["null".into()],
                    got: i.peek_n(4),
                })
            }
        }
        n if (*n as char).is_numeric() => {
            let head = i.pop().unwrap();
            let mut num_str = String::from(head as char);
            while let Some(&next_n) = i.peek() {
                if (next_n as char).is_numeric() || (next_n as char) == '.' {
                    i.pop();
                    num_str.push(next_n as char);
                } else {
                    break;
                }
            }
            let num = Num::try_from(num_str)?;
            Ok(JsValue::JsNumber(num))
        }
        b'"' => {
            _ = i.pop();
            Ok(JsValue::JsString(parse_str(i)?))
        }
        b'{' => parse_obj(i),
        b'[' => parse_arr(i),
        b' ' | b'\t' | b'\n' => {
            _ = i.pop();
            parse_value(i)
        }
        _ => Err(ParseError::UnexpectedToken {
            expected: vec![],
            got: String::from(*head as char),
        }),
    }
}

enum ObjectParseState {
    ExpectingKey,
    ExpectingKeyOrEndOfObject,
    ExpectingCommaOrEndOfObject,
    ExpectingColon,
    ExpectingValue,
}

fn parse_obj(i: &mut StrIt) -> Result<JsValue, ParseError> {
    _ = i.pop(); // pop open curly brace
    let mut state: ObjectParseState = ObjectParseState::ExpectingKeyOrEndOfObject;
    let mut key_values: HashMap<String, JsValue> = HashMap::new();
    let mut latest_key: Option<String> = None;

    loop {
        let next = i.peek().ok_or(ParseError::EOF)?;
        match state {
            ObjectParseState::ExpectingKey => {
                match next {
                    b'"' => {
                        _ = i.pop(); // pop "
                        latest_key = Some(parse_str(i)?);
                        state = ObjectParseState::ExpectingColon;
                    }
                    b' ' | b'\t' | b'\n' => {
                        _ = i.pop(); // ignore whitespaces here
                    }
                    _ => {
                        return Err(ParseError::UnexpectedToken {
                            expected: vec![String::from("\"")],
                            got: String::from(*next as char),
                        });
                    }
                }
            }
            ObjectParseState::ExpectingKeyOrEndOfObject => {
                match next {
                    b'}' => {
                        _ = i.pop();
                        break;
                    }
                    b'"' => {
                        _ = i.pop(); // pop "
                        latest_key = Some(parse_str(i)?);
                        state = ObjectParseState::ExpectingColon;
                    }
                    b' ' | b'\t' | b'\n' => {
                        _ = i.pop(); // ignore whitespaces here
                    }
                    _ => {
                        return Err(ParseError::UnexpectedToken {
                            expected: vec![String::from("\""), String::from("}")],
                            got: String::from(*next as char),
                        });
                    }
                }
            }
            ObjectParseState::ExpectingCommaOrEndOfObject => {
                match next {
                    b'}' => {
                        _ = i.pop();
                        break;
                    }
                    b',' => {
                        _ = i.pop();
                        state = ObjectParseState::ExpectingKey;
                    }
                    b' ' | b'\t' | b'\n' => {
                        _ = i.pop(); // ignore whitespaces here
                    }
                    _ => {
                        return Err(ParseError::UnexpectedToken {
                            expected: vec![String::from(","), String::from("}")],
                            got: String::from(*next as char),
                        });
                    }
                }
            }
            ObjectParseState::ExpectingColon => {
                match next {
                    b':' => {
                        _ = i.pop();
                        state = ObjectParseState::ExpectingValue;
                    }
                    b' ' | b'\t' | b'\n' => {
                        _ = i.pop(); // ignore whitespaces here
                    }
                    _ => {
                        return Err(ParseError::UnexpectedToken {
                            expected: vec![String::from(":")],
                            got: String::from(*next as char),
                        });
                    }
                }
            }
            ObjectParseState::ExpectingValue => {
                match next {
                    b' ' | b'\t' | b'\n' => {
                        _ = i.pop(); // ignore whitespaces here
                    }
                    _ => {
                        let value = parse_value(i)?;
                        match &latest_key {
                            Some(key) => {
                                key_values.insert(key.clone(), value);
                            }
                            _ => {
                                return Err(ParseError::InvalidJsonStructure);
                            }
                        }
                        latest_key = None;
                        state = ObjectParseState::ExpectingCommaOrEndOfObject;
                    }
                }
            }
        }
    }
    Ok(JsValue::JsObject(key_values))
}

enum ArrParseState {
    ExpectingValue,
    ExpectingValueOrEndOfArray,
    ExpectingCommaOrEndOfArray,
}

fn parse_arr(i: &mut StrIt) -> Result<JsValue, ParseError> {
    let mut values: Vec<JsValue> = vec![];
    let mut state: ArrParseState = ArrParseState::ExpectingValueOrEndOfArray;
    _ = i.pop(); // pop [
    loop {
        let head = i.peek().copied().ok_or(ParseError::EOF)?;
        match state {
            ArrParseState::ExpectingValueOrEndOfArray => match head {
                b']' => {
                    _ = i.pop();
                    break;
                }
                b' ' | b'\t' | b'\n' => {
                    _ = i.pop();
                }
                _ => {
                    values.push(parse_value(i)?);
                    state = ArrParseState::ExpectingCommaOrEndOfArray;
                }
            },
            ArrParseState::ExpectingCommaOrEndOfArray => match head {
                b']' => {
                    _ = i.pop();
                    break;
                }
                b' ' | b'\t' | b'\n' => {
                    _ = i.pop();
                }
                b',' => {
                    _ = i.pop();
                    state = ArrParseState::ExpectingValue;
                }
                _ => {
                    return Err(ParseError::UnexpectedToken {
                        expected: vec![String::from(","), String::from("]")],
                        got: String::from(head as char),
                    });
                }
            },
            ArrParseState::ExpectingValue => match head {
                b' ' | b'\t' | b'\n' => {
                    _ = i.pop();
                }
                _ => {
                    values.push(parse_value(i)?);
                    state = ArrParseState::ExpectingCommaOrEndOfArray;
                }
            },
        }
    }
    Ok(JsValue::JsArray(values))
}

pub fn parse_raw(s: &str) -> Result<JsValue, ParseError> {
    let mut it = StrIt {
        s: s.as_bytes(),
        pos: 0,
    };
    parse_value(&mut it)
}
