use std::env;

fn main() {
    let p = match get_args() {
        Ok(v) => v[1].chars().collect::<Vec<char>>(),
        Err(msg) => {panic!("{}", msg);}
    };

    println!(".intel_syntax noprefix");
    println!(".globl _main");
    println!("_main:");

    // 最初の項を処理
    let mut idx = 0;
    println!("  mov rax, {}", strtol(&p, &mut idx));

    while idx < p.len() {
        match p[idx] {
            '+' => {
                idx += 1;
                println!("  add rax, {}", strtol(&p, &mut idx));
            },
            '-' => {
                idx += 1;
                println!("  sub rax, {}", strtol(&p, &mut idx));
            },
            _ => {
                panic!("予期しない文字です: '{}'", p[idx]);
            }
        }
    }

    println!("  ret");
}

// p[idx]から違う記号が出てくるまでを数字として返す
fn strtol(p: &Vec<char>, idx: &mut usize) -> i32 {
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
