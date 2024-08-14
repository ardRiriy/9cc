use std::{env, process::exit};

fn main() {
    let args = match get_args() {
        Ok(v) => v,
        Err(msg) => {panic!("{}", msg);}
    };

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");


}

fn get_args() -> Result<Vec<String>, String> {
    let args :Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(String::from("引数の個数が正しくありません"));
    }
    return Ok(args);
}
