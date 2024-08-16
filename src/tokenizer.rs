use crate::utils::{error_at, strtol, USER_INPUT};
use itertools::Itertools;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Reserved,
    Ident,
    Num,
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
        if self.kind != TokenKind::Num {
            let user_input = USER_INPUT.lock().unwrap().clone();
            error_at(*idx, user_input, "数ではありません");
        }

        *idx += 1;
        self.val.unwrap()
    }

    pub fn expect(&self, idx: &mut usize, op: &str) {
        if self.kind != TokenKind::Reserved || self.str != op {
            let user_input = USER_INPUT.lock().unwrap().clone();
            error_at(*idx, user_input, &format!("{op}ではありません"));
        }

        *idx += 1;
    }

    pub fn consume(&self, idx: &mut usize, op: &str) -> bool {
        if self.kind != TokenKind::Reserved || self.str != op {
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
                let new_token = Token::new(TokenKind::Reserved, format!("{}", p[idx]));
                tokens.push(new_token);
                idx += 1;
            }
            ' ' => {
                // 空白はskipする
                idx += 1;
            }
            '0'..='9' => {
                let num = strtol(p, &mut idx);
                let mut new_token = Token::new(TokenKind::Num, num.to_string());
                new_token.val = Some(num);
                tokens.push(new_token);
            }
            '<' | '=' | '>' | '!' => {
                if idx + 1 < p.len() {
                    if p[idx + 1] == '=' {
                        let op = p[idx..=idx + 1].iter().join("");
                        let new_token = Token::new(TokenKind::Reserved, op);
                        tokens.push(new_token);
                        idx += 2;
                        continue;
                    }
                    let new_token = Token::new(TokenKind::Reserved, format!("{}", p[idx]));
                    tokens.push(new_token);
                    idx += 1;
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut last = idx + 1;
                while last < p.len()
                    && (p[last].is_ascii_lowercase()
                        || p[last].is_ascii_uppercase()
                        || p[last] == '_')
                {
                    last += 1;
                }
                let var = p[idx..last].iter().collect::<String>();
                let new_token = Token::new(TokenKind::Ident, var.to_string());
                tokens.push(new_token);
                idx = last;
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

    eprintln!(
        "[{}]\n",
        tokens.iter().map(|tkn| format!("{:?}", *tkn)).join(",\n")
    );
    tokens
}
