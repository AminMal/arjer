use crate::core::Token;
use crate::error::ParseError;
use crate::json::ast::Num;
use std::collections::VecDeque;

fn extract_head_string(l: &mut VecDeque<char>) -> Result<String, ParseError> {
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

fn extract_head_value(l: &mut VecDeque<char>) -> Result<Token, ParseError> {
    while let Some(&c) = l.front() {
        match c {
            't' => {
                let value_chars = l.drain(0..=3).collect::<Vec<_>>();
                match value_chars[..] {
                    ['t', 'r', 'u', 'e'] => {
                        return Ok(Token::Bool(true));
                    }
                    _ => {
                        return Err(ParseError::UnexpectedToken {
                            expected: vec!["true".into()],
                            got: value_chars.iter().collect(),
                        });
                    }
                }
            }
            'f' => {
                let value_chars = l.drain(0..=4).collect::<Vec<_>>();
                match value_chars[..] {
                    ['f', 'a', 'l', 's', 'e'] => {
                        return Ok(Token::Bool(false));
                    }
                    _ => {
                        return Err(ParseError::UnexpectedToken {
                            expected: vec!["false".into()],
                            got: value_chars.iter().collect(),
                        });
                    }
                }
            }
            'n' => {
                let value_chars = l.drain(0..=3).collect::<Vec<_>>();
                match value_chars[..] {
                    ['n', 'u', 'l', 'l'] => {
                        return Ok(Token::Null);
                    }
                    _ => {
                        return Err(ParseError::UnexpectedToken {
                            expected: vec!["null".into()],
                            got: value_chars.iter().collect(),
                        });
                    }
                }
            }
            n if n.is_numeric() => {
                l.pop_front();
                let mut num_str = String::new();
                num_str.push(n);
                while let Some(&next_n) = l.front() {
                    if next_n.is_numeric() || next_n == '.' {
                        l.pop_front();
                        num_str.push(next_n);
                    } else {
                        break;
                    }
                }
                let num = Num::try_from(num_str)?;
                return Ok(Token::N(num));
            }
            ch => {
                return Err(ParseError::UnexpectedToken {
                    expected: vec![],
                    got: String::from(ch),
                });
            }
        }
    }
    Err(ParseError::EOF)
}

pub fn tokenize(s: String) -> Result<Vec<Token>, ParseError> {
    let mut chars = s.chars().collect::<VecDeque<_>>();
    let mut result: Vec<Token> = vec![];

    while let Some(&next) = chars.front() {
        match next {
            '{' => {
                chars.pop_front();
                result.push(Token::OCurlyBrace);
            }
            '}' => {
                chars.pop_front();
                result.push(Token::CCurlyBrace);
            }
            '[' => {
                chars.pop_front();
                result.push(Token::OBracket);
            }
            ']' => {
                chars.pop_front();
                result.push(Token::CBracket);
            }
            '"' => {
                chars.pop_front(); // " is popped
                result.push(Token::DQuote);
                // core string
                let string = extract_head_string(&mut chars)?;
                result.push(Token::Str(string));
                result.push(Token::DQuote);
            }
            ':' => {
                chars.pop_front();
                result.push(Token::Colon);
            }
            ',' => {
                chars.pop_front();
                result.push(Token::Comma);
            }
            ' ' | '\n' | '\t' => {
                // skip spaces which are not part of a string
                chars.pop_front();
            }
            _ => {
                let token = extract_head_value(&mut chars)?;
                result.push(token);
            }
        }
    }
    Ok(result)
}
