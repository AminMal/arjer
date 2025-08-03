use super::token::Token;
use crate::json::JsValue;
use std::collections::HashMap;

#[derive(Clone, Debug)]
enum SubExpr {
    J(JsValue),
    KV(String, JsValue),
    KvSet(Vec<SubExpr>),
    JArr(Vec<JsValue>),
    S(String),
    T(Token),
}

fn string_between_dquotes(it: &Vec<SubExpr>) -> Option<(usize, SubExpr)> {
    // windows of size 3 because we're looking for [", Str, "]
    it.windows(3).enumerate().find_map(|(index, slice)| {
        if let [SubExpr::T(Token::DQuote), SubExpr::T(Token::Str(s)), SubExpr::T(Token::DQuote)] =
            slice
        {
            Some((index, SubExpr::S(s.clone())))
        } else {
            None
        }
    })
}

fn string_value(it: &Vec<SubExpr>) -> Option<(usize, SubExpr)> {
    // windows of size 2 because we're looking for [:, Str]
    it.windows(2).enumerate().find_map(|(index, slice)| {
        if let [SubExpr::T(Token::Colon), SubExpr::S(s)] = slice {
            // skip colon, hence index + 1
            Some((index + 1, SubExpr::J(JsValue::JsString(s.clone()))))
        } else if let [SubExpr::S(s), SubExpr::T(Token::Comma | Token::CCurlyBrace | Token::CBracket)] = slice {
            Some((index, SubExpr::J(JsValue::JsString(s.clone()))))
        } else {
            None
        }
    })
}

fn key_value(it: &Vec<SubExpr>) -> Option<(usize, SubExpr)> {
    // windows of size 3 because we're looking for [S, :, J]
    it.windows(3).enumerate().find_map(|(index, slice)| {
        if let [SubExpr::S(s), SubExpr::T(Token::Colon), SubExpr::J(j)] = slice {
            Some((index, SubExpr::KV(s.clone(), j.clone())))
        } else {
            None
        }
    })
}

fn first_kv_in_obj(it: &Vec<SubExpr>) -> Option<(usize, SubExpr)> {
    it.windows(2).enumerate().find_map(|(index, slice)| {
        if let [SubExpr::T(Token::OCurlyBrace), kv @ SubExpr::KV(_, _)] = slice {
            // skip OCurlyBrace, hence index + 1
            Some((index + 1, SubExpr::KvSet(vec![kv.clone()])))
        } else {
            None
        }
    })
}

fn kv_after_kvset(it: &Vec<SubExpr>) -> Option<(usize, SubExpr)> {
    it.windows(3).enumerate().find_map(|(index, slice)| {
        if let [SubExpr::KvSet(key_values), SubExpr::T(Token::Comma), kv @ SubExpr::KV(_, _)] =
            slice
        {
            let mut new_key_values = key_values.clone();
            new_key_values.push(kv.clone());
            Some((index, SubExpr::KvSet(new_key_values)))
        } else {
            None
        }
    })
}

fn obj(it: &Vec<SubExpr>) -> Option<(usize, SubExpr)> {
    it.windows(3).enumerate().find_map(|(index, slice)| {
        if let [SubExpr::T(Token::OCurlyBrace), SubExpr::KvSet(kvs), SubExpr::T(Token::CCurlyBrace)] = slice {
            let mut obj_map = HashMap::new();
            kvs.iter().filter_map(|se| {
                match se {
                    SubExpr::KV(k, v) => Some((k, v)),
                    _ => None
                }
            }).for_each(|(k, v)| {obj_map.insert(k.clone(), v.clone()); });
            Some((index, SubExpr::J(JsValue::JsObject(obj_map))))
        } else {
            None
        }
    })
}

fn first_elem_jarr(it: &Vec<SubExpr>) -> Option<(usize, SubExpr)> {
    it.windows(2).enumerate().find_map(|(index, slice)| {
        if let [SubExpr::T(Token::OBracket), SubExpr::J(js)] = slice {
            // skip OBracket, hence index + 1
            Some((index + 1, SubExpr::JArr(vec![js.clone()])))
        } else {
            None
        }
    })
}

fn jsvalue_after_jarr(it: &Vec<SubExpr>) -> Option<(usize, SubExpr)> {
    it.windows(3).enumerate().find_map(|(index, slice)| {
        if let [SubExpr::JArr(jarr), SubExpr::T(Token::Comma), SubExpr::J(j)] = slice {
            let mut new_jarr = jarr.clone();
            new_jarr.push(j.clone());
            Some((index, SubExpr::JArr(new_jarr)))
        } else {
            None
        }
    })
}

fn arr(it: &Vec<SubExpr>) -> Option<(usize, SubExpr)> {
    it.windows(3).enumerate().find_map(|(index, slice)| {
        if let [SubExpr::T(Token::OBracket), SubExpr::JArr(jarr), SubExpr::T(Token::CBracket)] =
            slice
        {
            Some((index, SubExpr::J(JsValue::JsArray(jarr.clone()))))
        } else {
            None
        }
    })
}

pub fn parse(tokens: Vec<Token>) -> Result<JsValue, String> {
    let mut subexprs = tokens
        .iter()
        .map(|t| match t {
            Token::Null => SubExpr::J(JsValue::JsNull),
            Token::Bool(b) => SubExpr::J(JsValue::JsBool(b.clone())),
            Token::U32(u) => SubExpr::J(JsValue::JsNumber(u.clone())),
            _ => SubExpr::T(t.clone()),
        })
        .collect::<Vec<_>>();

    while !matches!(&subexprs[..], &[SubExpr::J(_)]) {
        if let Some((i, subexpr)) = string_between_dquotes(&subexprs) {
            _ = subexprs.splice(i..i + 3, [subexpr]);
        } else if let Some((i, subexpr)) = string_value(&subexprs) {
            _ = subexprs.splice(i..i + 1, [subexpr]);
        } else if let Some((i, subexpr)) = key_value(&subexprs) {
            _ = subexprs.splice(i..i + 3, [subexpr]);
        } else if let Some((i, subexpr)) = first_kv_in_obj(&subexprs) {
            _ = subexprs.splice(i..i + 1, [subexpr]);
        } else if let Some((i, subexpr)) = kv_after_kvset(&subexprs) {
            _ = subexprs.splice(i..i + 3, [subexpr]);
        } else if let Some((i, subexpr)) = obj(&subexprs) {
            _ = subexprs.splice(i..i + 3, [subexpr]);
        } else if let Some((i, subexpr)) = first_elem_jarr(&subexprs) {
            _ = subexprs.splice(i..i + 1, [subexpr]);
        } else if let Some((i, subexpr)) = jsvalue_after_jarr(&subexprs) {
            _ = subexprs.splice(i..i + 3, [subexpr]);
        } else if let Some((i, subexpr)) = arr(&subexprs) {
            _ = subexprs.splice(i..i + 3, [subexpr]);
        } else {
            dbg!(&subexprs);
            break;
        }
    }
    if let [SubExpr::J(js)] = &subexprs[..] {
        Ok(js.clone())
    } else {
        Err("failed".into())
    }
}
