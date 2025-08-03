use crate::core::Token;
use crate::json::ast::Num;
use std::collections::VecDeque;

fn extract_head_string(l: &mut VecDeque<char>) -> Result<String, String> {
    let mut result = String::new();
    while let Some(c) = l.pop_front() {
        match c {
            '"' => break,
            '\\' => {
                let x = l
                    .pop_front()
                    .ok_or(String::from("expected escaped character"))?;
                result.push(x);
            }
            other => {
                result.push(other);
            }
        }
    }
    Ok(result)
}

fn extract_head_value(l: &mut VecDeque<char>) -> Result<Token, String> {
    let mut result = None;
    while let Some(&c) = l.front() {
        match c {
            't' => {
                let value_chars = l.drain(0..=3).collect::<Vec<_>>();
                match value_chars[..] {
                    ['t', 'r', 'u', 'e'] => {
                        result = Some(Token::Bool(true));
                    }
                    _ => return Err(format!("expected `true`, got {:?}", value_chars)),
                }
            }
            'f' => {
                let value_chars = l.drain(0..=4).collect::<Vec<_>>();
                match value_chars[..] {
                    ['f', 'a', 'l', 's', 'e'] => {
                        result = Some(Token::Bool(false));
                    }
                    _ => {
                        return Err(format!("expected `false`, got {:?}", value_chars));
                    }
                }
            }
            'n' => {
                let value_chars = l.drain(0..=3).collect::<Vec<_>>();
                match value_chars[..] {
                    ['n', 'u', 'l', 'l'] => {
                        result = Some(Token::Null);
                    }
                    _ => {
                        return Err(format!("expected `null`, got {:?}", value_chars));
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
                result = Some(Token::N(num));
            }
            ',' | '}' | ']' => break,
            ' ' | '\n' | '\t' => {
                l.pop_front();
                // Skip spaces
            }
            ch => {
                return Err(format!("invalid char {}", ch));
            }
        }
    }
    result.ok_or("failed extracting head value".into())
}

pub fn tokenize(s: String) -> Result<Vec<Token>, String> {
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
