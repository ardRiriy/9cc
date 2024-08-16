use tokenizer::tokenize;
use utils::{get_args, USER_INPUT};
use codegen::NodeTree;

mod tokenizer;
mod codegen;
mod utils;

fn main() {
    let p = match get_args() {
        Ok(v) => {
            let mut user_input = USER_INPUT.lock().unwrap();
            *user_input = v[1].clone();
            v[1].chars().collect::<Vec<char>>()
        }
        Err(msg) => {
            panic!("{}", msg);
        }
    };

    println!(".intel_syntax noprefix");
    println!(".globl _main");
    println!("_main:");

    let tokens = tokenize(&p);
    let mut node_tree = NodeTree::new(tokens);
    node_tree.parse();

    // スタックトップの値を関数からの戻り値とする
    println!("  pop rax");
    println!("  ret");
}
