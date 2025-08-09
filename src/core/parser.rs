use crate::error::ParseError;
use crate::json::{JsValue, Num};
use std::collections::{HashMap, VecDeque};

fn parse_str(l: &mut VecDeque<char>) -> Result<String, ParseError> {
    let mut result = String::new();
    while let Some(c) = l.pop_front() {
        match c {
            '"' => break,
            '\\' => {
                let x = l.pop_front().ok_or(ParseError::EOF)?;
                result.push(x);
            }
            other => {
                result.push(other);
            }
        }
    }
    Ok(result)
}

fn parse_value_v2(s: &mut VecDeque<char>) -> Result<JsValue, ParseError> {
    let head = s.front().ok_or(ParseError::EOF)?;
    match head {
        't' => {
            let value_chars = s.drain(0..=3).collect::<Vec<_>>();
            match value_chars[..] {
                ['t', 'r', 'u', 'e'] => Ok(JsValue::JsBool(true)),
                _ => Err(ParseError::UnexpectedToken {
                    expected: vec!["true".into()],
                    got: value_chars.iter().collect(),
                }),
            }
        }
        'f' => {
            let value_chars = s.drain(0..=4).collect::<Vec<_>>();
            match value_chars[..] {
                ['f', 'a', 'l', 's', 'e'] => Ok(JsValue::JsBool(false)),
                _ => Err(ParseError::UnexpectedToken {
                    expected: vec!["false".into()],
                    got: value_chars.iter().collect(),
                }),
            }
        }
        'n' => {
            let value_chars = s.drain(0..=3).collect::<Vec<_>>();
            match value_chars[..] {
                ['n', 'u', 'l', 'l'] => Ok(JsValue::JsNull),
                _ => Err(ParseError::UnexpectedToken {
                    expected: vec!["null".into()],
                    got: value_chars.iter().collect(),
                }),
            }
        }
        n if n.is_numeric() => {
            let head = s.pop_front().unwrap();
            let mut num_str = String::new();
            num_str.push(head);
            while let Some(&next_n) = s.front() {
                if next_n.is_numeric() || next_n == '.' {
                    s.pop_front();
                    num_str.push(next_n);
                } else {
                    break;
                }
            }
            let num = Num::try_from(num_str)?;
            Ok(JsValue::JsNumber(num))
        }
        '"' => {
            _ = s.pop_front();
            Ok(JsValue::JsString(parse_str(s)?))
        }
        '{' => parse_obj_v2(s),
        '[' => parse_arr_v2(s),
        ' ' | '\t' | '\n' => {
            _ = s.pop_front();
            parse_value_v2(s)
        }
        _ => Err(ParseError::UnexpectedToken {
            expected: vec![],
            got: String::from(head.clone()),
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

fn parse_obj_v2(s: &mut VecDeque<char>) -> Result<JsValue, ParseError> {
    _ = s.pop_front(); // pop open curly brace
    let mut state: ObjectParseState = ObjectParseState::ExpectingKeyOrEndOfObject;
    let mut key_values: HashMap<String, JsValue> = HashMap::new();
    let mut latest_key: Option<String> = None;

    loop {
        let next = s.front().ok_or(ParseError::EOF)?;
        match state {
            ObjectParseState::ExpectingKey => {
                match next {
                    '"' => {
                        _ = s.pop_front(); // pop "
                        latest_key = Some(parse_str(s)?);
                        state = ObjectParseState::ExpectingColon;
                    }
                    ' ' | '\t' | '\n' => {
                        _ = s.pop_front(); // ignore whitespaces here
                    }
                    _ => {
                        return Err(ParseError::UnexpectedToken {
                            expected: vec![String::from("\"")],
                            got: String::from(next.clone()),
                        });
                    }
                }
            }
            ObjectParseState::ExpectingKeyOrEndOfObject => {
                match next {
                    '}' => {
                        _ = s.pop_front();
                        break;
                    }
                    '"' => {
                        _ = s.pop_front(); // pop "
                        latest_key = Some(parse_str(s)?);
                        state = ObjectParseState::ExpectingColon;
                    }
                    ' ' | '\t' | '\n' => {
                        _ = s.pop_front(); // ignore whitespaces here
                    }
                    _ => {
                        return Err(ParseError::UnexpectedToken {
                            expected: vec![String::from("\""), String::from("}")],
                            got: String::from(next.clone()),
                        });
                    }
                }
            }
            ObjectParseState::ExpectingCommaOrEndOfObject => {
                match next {
                    '}' => {
                        _ = s.pop_front();
                        break;
                    }
                    ',' => {
                        _ = s.pop_front();
                        state = ObjectParseState::ExpectingKey;
                    }
                    ' ' | '\t' | '\n' => {
                        _ = s.pop_front(); // ignore whitespaces here
                    }
                    _ => {
                        return Err(ParseError::UnexpectedToken {
                            expected: vec![String::from(","), String::from("}")],
                            got: String::from(next.clone()),
                        });
                    }
                }
            }
            ObjectParseState::ExpectingColon => {
                match next {
                    ':' => {
                        _ = s.pop_front();
                        state = ObjectParseState::ExpectingValue;
                    }
                    ' ' | '\t' | '\n' => {
                        _ = s.pop_front(); // ignore whitespaces here
                    }
                    _ => {
                        return Err(ParseError::UnexpectedToken {
                            expected: vec![String::from(":")],
                            got: String::from(next.clone()),
                        });
                    }
                }
            }
            ObjectParseState::ExpectingValue => {
                match next {
                    ' ' | '\t' | '\n' => {
                        _ = s.pop_front(); // ignore whitespaces here
                    }
                    _ => {
                        let value = parse_value_v2(s)?;
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
fn parse_arr_v2(s: &mut VecDeque<char>) -> Result<JsValue, ParseError> {
    let mut values: Vec<JsValue> = vec![];
    let mut state: ArrParseState = ArrParseState::ExpectingValueOrEndOfArray;
    _ = s.pop_front(); // pop [
    loop {
        let head = s.front().map(char::clone).ok_or(ParseError::EOF)?;
        match state {
            ArrParseState::ExpectingValueOrEndOfArray => match head {
                ']' => {
                    _ = s.pop_front();
                    break;
                }
                ' ' | '\t' | '\n' => {
                    _ = s.pop_front();
                }
                _ => {
                    values.push(parse_value_v2(s)?);
                    state = ArrParseState::ExpectingCommaOrEndOfArray;
                }
            },
            ArrParseState::ExpectingCommaOrEndOfArray => match head {
                ']' => {
                    _ = s.pop_front();
                    break;
                }
                ' ' | '\t' | '\n' => {
                    _ = s.pop_front();
                }
                ',' => {
                    _ = s.pop_front();
                    state = ArrParseState::ExpectingValue;
                }
                _ => {
                    return Err(ParseError::UnexpectedToken {
                        expected: vec![String::from(","), String::from("]")],
                        got: String::from(head.clone()),
                    });
                }
            },
            ArrParseState::ExpectingValue => match head {
                ' ' | '\t' | '\n' => {
                    _ = s.pop_front();
                }
                _ => {
                    values.push(parse_value_v2(s)?);
                    state = ArrParseState::ExpectingCommaOrEndOfArray;
                }
            },
        }
    }
    Ok(JsValue::JsArray(values))
}

pub fn parse_raw(s: &str) -> Result<JsValue, ParseError> {
    let mut chars = s.chars().collect::<VecDeque<char>>();
    parse_value_v2(&mut chars)
}
