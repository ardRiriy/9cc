use itertools::Itertools;
use std::env;
use std::sync::Mutex;

lazy_static::lazy_static! {
    pub static ref USER_INPUT: Mutex<String> = Mutex::new(String::new());
}

pub fn error_at(idx: usize, user_input: String, reason: &str) {
    eprintln!("{}", user_input);
    eprintln!("{}^ {}", (0..idx).map(|_| " ").join(""), reason);
    panic!();
}

pub fn get_keyword(p: &[char], idx: &mut usize) -> String {
    let mut last = *idx + 1;
    while *idx < p.len()
        && (p[last].is_lowercase()
            || p[last].is_uppercase()
            || p[last].is_numeric()
            || p[last] == '_')
    {
        last += 1;
    }

    let var = p[*idx..last].iter().collect::<String>();
    *idx = last;
    var
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
