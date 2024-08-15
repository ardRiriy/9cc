use std::env;

fn main() {
    let args = match get_args() {
        Ok(v) => v,
        Err(msg) => {panic!("{}", msg);}
    };

    println!(".intel_syntax noprefix");
    println!(".globl _main");
    println!("_main:");
    println!("  mov rax, {}", args[1]);
    println!("  ret");
}

fn get_args() -> Result<Vec<String>, String> {
    let args :Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(String::from("引数の個数が正しくありません"));
    }
    Ok(args)
}
