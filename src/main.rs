use std::env;
use std::sync::Mutex;
use itertools::Itertools;

lazy_static::lazy_static! {
    static ref USER_INPUT: Mutex<String> = Mutex::new(String::new());
}

#[derive(Debug, PartialEq)]
enum TokenKind {
    TkReserved,
    TkNum,
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    val: Option<i32>,
    str: String,
}

impl Token {
    fn new(kind: TokenKind, str: String) -> Token {
        Token { kind, val: None, str}
    }

    fn expect_number(&self, idx: &mut usize) -> i32 {
        if self.kind != TokenKind::TkNum {
            let user_input = USER_INPUT.lock().unwrap().clone();
            error_at(*idx, user_input, "数ではありません");
        }

        *idx += 1;
        self.val.unwrap()
    }

    fn expect(&self, idx: &mut usize, op: String) {
        if self.kind != TokenKind::TkReserved || self.str != op {
            let user_input = USER_INPUT.lock().unwrap().clone();
            error_at(*idx, user_input, &format!("{op}ではありません"));
        }

        *idx += 1;
    }

    fn consume(&self, idx: &mut usize, op: String) -> bool {
        if self.kind != TokenKind::TkReserved || self.str != op {
            return false;
        }
        *idx += 1;
        true
    }
}

fn tokenize(p: &Vec<char>) -> Vec<Token> {
    let mut tokens = vec![];
    let mut idx = 0;
    while idx < p.len() {
        match p[idx] {
            '+' | '-' => {
                let new_token = Token::new(
                    TokenKind::TkReserved,
                    format!("{}", p[idx]),
                );
                tokens.push(new_token);
                idx += 1;
            },
            ' ' => {
                // 空白はskipする
                idx += 1;
            }
            '0'..='9' => {
                let num = strtol(p, &mut idx);
                let mut new_token = Token::new(
                    TokenKind::TkNum,
                    num.to_string()
                );
                new_token.val = Some(num);
                tokens.push(new_token);
            }
            _ => {
                let user_input = USER_INPUT.lock().unwrap().clone();
                error_at(idx, user_input, &format!("予期しない文字です: '{}'", p[idx]));
                panic!();
            }
        }
    }
    tokens
}

// p[idx]から違う記号が出てくるまでを数字として返す
fn strtol(p: &[char], idx: &mut usize) -> i32 {
    let mut num = 0;
    while *idx < p.len() {
        match p[*idx] {
            '0'..='9' => {
                num = num * 10 + (p[*idx] as i32 - '0' as i32);
            },
            _ => {
                break;
            }
        }
        *idx += 1;
    }
    num
}

fn get_args() -> Result<Vec<String>, String> {
    let args :Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(String::from("引数の個数が正しくありません"));
    }
    Ok(args)
}

fn main() {
    let p = match get_args() {
        Ok(v) => {
            let mut user_input = USER_INPUT.lock().unwrap();
            *user_input = v[1].clone();
            v[1].chars().collect::<Vec<char>>()
        },
        Err(msg) => {panic!("{}", msg);}
    };

    println!(".intel_syntax noprefix");
    println!(".globl _main");
    println!("_main:");

    let tokens = tokenize(&p);

    // 最初の項を処理
    let mut idx = 0;
    println!("  mov rax, {}", tokens[0].expect_number(&mut idx));

    while idx < tokens.len() {
        if tokens[idx].consume(&mut idx, "+".to_string()) {
            println!("  add rax, {}", tokens[idx].expect_number(&mut idx));
            continue;
        }

        tokens[idx].expect(&mut idx, "-".to_string());
        println!("  sub rax, {}", tokens[idx].expect_number(&mut idx));
    }

    println!("  ret");
}

fn error_at(idx: usize, user_input: String, reason: &str) {
    eprintln!("{}", user_input);
    eprintln!("{}^ {}",
        (0..idx).map(|_| " ").join(""),
        reason
    );
    panic!();
}
