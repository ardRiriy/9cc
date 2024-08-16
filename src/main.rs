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

    fn expect(&self, idx: &mut usize, op: &str) {
        if self.kind != TokenKind::TkReserved || self.str != op {
            let user_input = USER_INPUT.lock().unwrap().clone();
            error_at(*idx, user_input, &format!("{op}ではありません"));
        }

        *idx += 1;
    }

    fn consume(&self, idx: &mut usize, op: &str) -> bool {
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
            '+' | '-' | '*' | '/' | '(' | ')' => {
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
            '<' | '=' | '>' | '!'  => {
                if idx + 1 < p.len() {
                    if p[idx+1] == '=' {
                        let op = p[idx..=idx+1].iter().join("");
                        let new_token = Token::new(
                            TokenKind::TkReserved,
                            op
                        );
                        tokens.push(new_token);
                        idx += 2;
                        continue;
                    }
                    let new_token = Token::new(
                        TokenKind::TkReserved,
                        format!("{}", p[idx])
                    );
                    tokens.push(new_token);
                    idx += 1;
                }
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

#[derive(Debug, PartialEq)]
enum NodeKind {
    Add,
    Sub,
    Mul,
    Div,
    Num,
    Eq,
    Nq,
    Le, // <=
    Lt, // <
}

#[derive(Debug)]
struct Node {
    kind: NodeKind,
    lhs: usize, // 配列上のindex
    rhs: usize, // 配列上のindex
    val: Option<i32>,
}

impl Node {
    fn new(kind: NodeKind, lhs: usize, rhs: usize, val: Option<i32>) -> Node {
        Node { kind, lhs, rhs, val }
    }
}

struct NodeTree {
    tokens: Vec<Token>,
    read: usize, // 読んだトークン数
    node_tree: Vec<Node>
}

impl NodeTree {
    fn new(tokens: Vec<Token>) -> NodeTree {
        NodeTree { tokens, node_tree: vec![], read: 0 }
    }

    fn parse(&mut self) {
        let loc = self.expr();
        self.generate(loc);
    }

    fn generate(&mut self, loc: usize) {
        if self.node_tree[loc].kind == NodeKind::Num {
            println!("  push {}", self.node_tree[loc].val.unwrap());
            return;
        }

        self.generate(self.node_tree[loc].lhs);
        self.generate(self.node_tree[loc].rhs);
        println!("  pop rdi");
        println!("  pop rax");

        match self.node_tree[loc].kind {
            NodeKind::Add => {
                println!("  add rax, rdi");
            },
            NodeKind::Sub => {
                println!("  sub rax, rdi");
            },
            NodeKind::Mul => {
                println!("  imul rax, rdi");
            },
            NodeKind::Div => {
                println!("  cqo");
                println!("  idiv rdi");
            },
            NodeKind::Eq => {
                println!("  cmp rax, rdi");
                println!("  sete al");
                println!("  movzx rax, al");
            },
            NodeKind::Nq => {
                println!("  cmp rax, rdi");
                println!("  setne al");
                println!("  movzx rax, al");
            },
            NodeKind::Lt => {
                println!("  cmp rax, rdi");
                println!("  setl al");
                println!("  movzx rax, al");
            },
            NodeKind::Le => {
                println!("  cmp rax, rdi");
                println!("  setle al");
                println!("  movzx rax, al");
            }
            _ => {
                unreachable!();
            }
        }

        println!("  push rax");
    }

    fn expr(&mut self) -> usize {
        self.equality()
    }

    fn equality(&mut self) -> usize {
        // ==, !=
        let mut node = self.relational();

        while self.read < self.tokens.len() {
            if self.tokens[self.read].consume(&mut self.read, "==") {
                let new_node = Node::new(
                    NodeKind::Eq,
                    node,
                    self.relational(),
                    None
                );
                self.node_tree.push(new_node);
                node = self.node_tree.len() - 1;
            } else if self.tokens[self.read].consume(&mut self.read, "!=") {
                let new_node = Node::new(
                    NodeKind::Nq,
                    node,
                    self.relational(),
                    None
                );
                self.node_tree.push(new_node);
                node = self.node_tree.len() - 1;
            } else {
                return node;
            }
        }
        node
    }

    fn relational(&mut self) -> usize {
        // <, <=, >, >=
        let mut node = self.add();

        while self.read < self.tokens.len() {
            if self.tokens[self.read].consume(&mut self.read, "<") {
                let new_node = Node::new(
                    NodeKind::Lt,
                    node,
                    self.add(),
                    None
                );
                self.node_tree.push(new_node);
                node = self.node_tree.len() - 1;
            } else if self.tokens[self.read].consume(&mut self.read, "<=") {
                let new_node = Node::new(
                    NodeKind::Le,
                    node,
                    self.add(),
                    None
                );
                self.node_tree.push(new_node);
                node = self.node_tree.len() - 1;
            } else if self.tokens[self.read].consume(&mut self.read, ">") {
                // 左辺と右辺を入れ替えて評価することでgenの実装量が減って嬉しい(>= も同様)
                let new_node = Node::new(
                    NodeKind::Lt,
                    self.add(),
                    node,
                    None
                );
                self.node_tree.push(new_node);
                node = self.node_tree.len() - 1;
            } else if self.tokens[self.read].consume(&mut self.read, ">=") {
                let new_node = Node::new(
                    NodeKind::Le,
                    self.add(),
                    node,
                    None
                );
                self.node_tree.push(new_node);
                node = self.node_tree.len() - 1;
            } else {
                return node;
            }
        }

        return node;
    }

    fn add(&mut self) -> usize {
        let mut node = self.mul();
        while self.read < self.tokens.len() {
            if self.tokens[self.read].consume(&mut self.read, "+") {
                let new_node = Node::new(
                    NodeKind::Add,
                    node,
                    self.mul(),
                    None
                );
                self.node_tree.push(new_node);
                eprintln!("{}\n", self.node_tree.iter().map(|node| format!("{:?}", *node)).join(",\n"));
                node = self.node_tree.len() - 1;
            } else if self.tokens[self.read].consume(&mut self.read, "-") {
                let new_node = Node::new(
                    NodeKind::Sub,
                    node,
                    self.mul(),
                    None
                );
                self.node_tree.push(new_node);
                node = self.node_tree.len() - 1;
            } else {
                return node;
            }
        }
        node
    }

    fn mul(&mut self) -> usize {
        let mut node = self.unary();
        while self.read < self.tokens.len() {
            if self.tokens[self.read].consume(&mut self.read, "*") {
                let new_node = Node::new(
                    NodeKind::Mul,
                    node,
                    self.unary(),
                    None,
                );
                self.node_tree.push(new_node);
                node = self.node_tree.len() - 1;
            } else if self.tokens[self.read].consume(&mut self.read, "/") {
                let new_node = Node::new(
                    NodeKind::Div,
                    node,
                    self.unary(),
                    None,
                );
                self.node_tree.push(new_node);
                node = self.node_tree.len() - 1;
            } else {
                return node;
            }
        }
        node
    }

    fn unary(&mut self) -> usize {
        if self.tokens[self.read].consume(&mut self.read, "+") {
            return self.primary();
        } else if self.tokens[self.read].consume(&mut self.read, "-") {
            // -x は 0 - xとして取り扱う
            let inf = 1e10 as usize;
            let zero_node = Node::new(NodeKind::Num, inf, inf, Some(0));
            self.node_tree.push(zero_node);
            let new_node = Node::new(
                NodeKind::Sub,
                self.node_tree.len()-1,
                self.primary(),
                None
            );
            self.node_tree.push(new_node);
            return self.node_tree.len() - 1;
        }

        return self.primary();
    }

    fn primary(&mut self) -> usize {
        if self.tokens[self.read].consume(&mut self.read, "(") {
            let idx = self.expr();
            self.tokens[self.read].expect(&mut self.read, ")");
            return idx;
        }

        // 実装ミスっていたら配列外参照で落ちるようにしておく
        let inf = 1e10 as usize;
        let new_node = Node::new(
            NodeKind::Num,
            inf,
            inf,
            Some(self.tokens[self.read].expect_number(&mut self.read))
        );
        self.node_tree.push(new_node);
        eprintln!("{}\n", self.node_tree.iter().map(|node| format!("{:?}", *node)).join(",\n"));
        return self.node_tree.len() - 1;
    }
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
    let mut node_tree = NodeTree::new(tokens);
    node_tree.parse();

    // スタックトップの値を関数からの戻り値とする
    println!("  pop rax");
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
