use std::fmt::format;

use crate::utils::{error_at, strtol, USER_INPUT};
use itertools::Itertools;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    TkReserved,
    TkIdent,
    TkNum,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    val: Option<i32>,
    pub str: String,
}

impl Token {
    fn new(kind: TokenKind, str: String) -> Token {
        Token {
            kind,
            val: None,
            str,
        }
    }

    pub fn expect_number(&self, idx: &mut usize) -> i32 {
        if self.kind != TokenKind::TkNum {
            let user_input = USER_INPUT.lock().unwrap().clone();
            error_at(*idx, user_input, "数ではありません");
        }

        *idx += 1;
        self.val.unwrap()
    }

    pub fn expect(&self, idx: &mut usize, op: &str) {
        if self.kind != TokenKind::TkReserved || self.str != op {
            let user_input = USER_INPUT.lock().unwrap().clone();
            error_at(*idx, user_input, &format!("{op}ではありません"));
        }

        *idx += 1;
    }

    pub fn consume(&self, idx: &mut usize, op: &str) -> bool {
        if self.kind != TokenKind::TkReserved || self.str != op {
            return false;
        }
        *idx += 1;
        true
    }
}

pub fn tokenize(p: &[char]) -> Vec<Token> {
    let mut tokens = vec![];
    let mut idx = 0;
    while idx < p.len() {
        match p[idx] {
            '+' | '-' | '*' | '/' | '(' | ')' | ';' => {
                let new_token = Token::new(TokenKind::TkReserved, format!("{}", p[idx]));
                tokens.push(new_token);
                idx += 1;
            }
            ' ' => {
                // 空白はskipする
                idx += 1;
            }
            '0'..='9' => {
                let num = strtol(p, &mut idx);
                let mut new_token = Token::new(TokenKind::TkNum, num.to_string());
                new_token.val = Some(num);
                tokens.push(new_token);
            }
            '<' | '=' | '>' | '!' => {
                if idx + 1 < p.len() {
                    if p[idx + 1] == '=' {
                        let op = p[idx..=idx + 1].iter().join("");
                        let new_token = Token::new(TokenKind::TkReserved, op);
                        tokens.push(new_token);
                        idx += 2;
                        continue;
                    }
                    let new_token = Token::new(TokenKind::TkReserved, format!("{}", p[idx]));
                    tokens.push(new_token);
                    idx += 1;
                }
            }
            'a'..='z' => {
                let new_token = Token::new(TokenKind::TkIdent, format!("{}", p[idx]));
                tokens.push(new_token);
                idx += 1;
            }
            _ => {
                let user_input = USER_INPUT.lock().unwrap().clone();
                error_at(
                    idx,
                    user_input,
                    &format!("予期しない文字です: '{}'", p[idx]),
                );
                panic!();
            }
        }
    }

    eprintln!("[{}]\n", tokens.iter().map(|tkn| format!("{:?}", *tkn)).join(",\n"));
    tokens
}
