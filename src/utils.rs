use itertools::Itertools;
use std::sync::Mutex;
use std::env;

lazy_static::lazy_static! {
    pub static ref USER_INPUT: Mutex<String> = Mutex::new(String::new());
}

pub fn error_at(idx: usize, user_input: String, reason: &str) {
    eprintln!("{}", user_input);
    eprintln!("{}^ {}", (0..idx).map(|_| " ").join(""), reason);
    panic!();
}

// p[idx]から違う記号が出てくるまでを数字として返す
pub fn strtol(p: &[char], idx: &mut usize) -> i32 {
    let mut num = 0;
    while *idx < p.len() {
        match p[*idx] {
            '0'..='9' => {
                num = num * 10 + (p[*idx] as i32 - '0' as i32);
            }
            _ => {
                break;
            }
        }
        *idx += 1;
    }
    num
}

pub fn get_args() -> Result<Vec<String>, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(String::from("引数の個数が正しくありません"));
    }
    Ok(args)
}
