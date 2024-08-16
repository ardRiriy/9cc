use std::collections::BTreeMap;

use itertools::Itertools;

use crate::localvar::LVar;
use crate::tokenizer::{Token, TokenKind};

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
    Assign,
    LocalVar,
}

#[derive(Debug)]
struct Node {
    kind: NodeKind,
    lhs: usize, // 配列上のindex
    rhs: usize, // 配列上のindex
    val: Option<i32>,
    offset: usize, // スタック上のオフセット。LocalVarの時だけ使う
}

impl Node {
    fn new(kind: NodeKind, lhs: usize, rhs: usize, val: Option<i32>, offset: usize) -> Node {
        Node {
            kind,
            lhs,
            rhs,
            val,
            offset,
        }
    }
}

#[derive(Debug)]
pub struct NodeTree {
    tokens: Vec<Token>,
    read: usize, // 読んだトークン数
    nodes: Vec<Node>,
    start_nodes: Vec<usize>, // 式の始まりのトークンのindexを保存
    vars: BTreeMap<String, LVar>,
    pub offset: usize,
}

impl NodeTree {
    pub fn new(tokens: Vec<Token>) -> NodeTree {
        NodeTree {
            tokens,
            nodes: Vec::new(),
            read: 0,
            start_nodes: Vec::new(),
            vars: BTreeMap::new(),
            offset: 8,
        }
    }

    pub fn parse(&mut self) {
        self.program();

        // スタックフレーム宣言
        println!("  push rbp");
        println!("  mov rbp, rsp");
        println!("  sub rsp, {}", self.offset);
        // eprintln!("[{}]\n",
        //     self.nodes.iter().enumerate().map(|(idx, node)| format!("{idx}: {:?}", *node)).join("\n")
        // );

        for &loc in &self.start_nodes.clone() {
            // eprintln!("start at: {loc}");
            self.generate(loc);
            println!("  pop rax"); // 最後に評価した値がスタックに残るので、popしておく
        }
    }

    fn gen_lval(&self, loc: usize) {
        if self.nodes[loc].kind != NodeKind::LocalVar {
            panic!("代入の左辺値が変数ではありません");
        }
        println!("  mov rax, rbp");
        println!("  sub rax, {}", self.nodes[loc].offset);
        println!("  push rax");
    }

    fn generate(&mut self, loc: usize) {
        match self.nodes[loc].kind {
            NodeKind::Num => {
                println!("  push {}", self.nodes[loc].val.unwrap());
                return;
            }
            NodeKind::LocalVar => {
                // 変数の読み出し
                // 「変数に値を代入する」操作はNodeKind::Assignの時点で処理済みなので、
                // ここに流れ着くのは読み出しの場合のみ
                self.gen_lval(loc);
                println!("  pop rax");
                println!("  mov rax, [rax]");
                println!("  push rax");
                return;
            }
            NodeKind::Assign => {
                self.gen_lval(self.nodes[loc].lhs);
                self.generate(self.nodes[loc].rhs);
                println!("  pop rdi");
                println!("  pop rax");
                println!("  mov [rax], rdi");
                println!("  push rdi");
                return;
            }
            _ => {}
        }

        self.generate(self.nodes[loc].lhs);
        self.generate(self.nodes[loc].rhs);
        println!("  pop rdi");
        println!("  pop rax");

        match self.nodes[loc].kind {
            NodeKind::Add => {
                println!("  add rax, rdi");
            }
            NodeKind::Sub => {
                println!("  sub rax, rdi");
            }
            NodeKind::Mul => {
                println!("  imul rax, rdi");
            }
            NodeKind::Div => {
                println!("  cqo");
                println!("  idiv rdi");
            }
            NodeKind::Eq => {
                println!("  cmp rax, rdi");
                println!("  sete al");
                println!("  movzx rax, al");
            }
            NodeKind::Nq => {
                println!("  cmp rax, rdi");
                println!("  setne al");
                println!("  movzx rax, al");
            }
            NodeKind::Lt => {
                println!("  cmp rax, rdi");
                println!("  setl al");
                println!("  movzx rax, al");
            }
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

    fn program(&mut self) {
        while self.read < self.tokens.len() {
            let node = self.stmt();
            self.start_nodes.push(node);
        }
    }

    fn stmt(&mut self) -> usize {
        let node = self.expr();
        self.tokens[self.read].expect(&mut self.read, ";");
        node
    }

    fn expr(&mut self) -> usize {
        self.assign()
    }

    fn assign(&mut self) -> usize {
        let mut node = self.equality();
        if self.tokens[self.read].consume(&mut self.read, "=") {
            let new_node = Node::new(NodeKind::Assign, node, self.assign(), None, 0);
            self.nodes.push(new_node);
            node = self.nodes.len() - 1;
        }
        node
    }

    fn equality(&mut self) -> usize {
        // ==, !=
        let mut node = self.relational();

        while self.read < self.tokens.len() {
            if self.tokens[self.read].consume(&mut self.read, "==") {
                let new_node = Node::new(NodeKind::Eq, node, self.relational(), None, 0);
                self.nodes.push(new_node);
                node = self.nodes.len() - 1;
            } else if self.tokens[self.read].consume(&mut self.read, "!=") {
                let new_node = Node::new(NodeKind::Nq, node, self.relational(), None, 0);
                self.nodes.push(new_node);
                node = self.nodes.len() - 1;
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
                let new_node = Node::new(NodeKind::Lt, node, self.add(), None, 0);
                self.nodes.push(new_node);
                node = self.nodes.len() - 1;
            } else if self.tokens[self.read].consume(&mut self.read, "<=") {
                let new_node = Node::new(NodeKind::Le, node, self.add(), None, 0);
                self.nodes.push(new_node);
                node = self.nodes.len() - 1;
            } else if self.tokens[self.read].consume(&mut self.read, ">") {
                // 左辺と右辺を入れ替えて評価することでgenの実装量が減って嬉しい(>= も同様)
                let new_node = Node::new(NodeKind::Lt, self.add(), node, None, 0);
                self.nodes.push(new_node);
                node = self.nodes.len() - 1;
            } else if self.tokens[self.read].consume(&mut self.read, ">=") {
                let new_node = Node::new(NodeKind::Le, self.add(), node, None, 0);
                self.nodes.push(new_node);
                node = self.nodes.len() - 1;
            } else {
                return node;
            }
        }

        node
    }

    fn add(&mut self) -> usize {
        let mut node = self.mul();
        while self.read < self.tokens.len() {
            if self.tokens[self.read].consume(&mut self.read, "+") {
                let new_node = Node::new(NodeKind::Add, node, self.mul(), None, 0);
                self.nodes.push(new_node);
                node = self.nodes.len() - 1;
            } else if self.tokens[self.read].consume(&mut self.read, "-") {
                let new_node = Node::new(NodeKind::Sub, node, self.mul(), None, 0);
                self.nodes.push(new_node);
                node = self.nodes.len() - 1;
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
                let new_node = Node::new(NodeKind::Mul, node, self.unary(), None, 0);
                self.nodes.push(new_node);
                node = self.nodes.len() - 1;
            } else if self.tokens[self.read].consume(&mut self.read, "/") {
                let new_node = Node::new(NodeKind::Div, node, self.unary(), None, 0);
                self.nodes.push(new_node);
                node = self.nodes.len() - 1;
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
            let zero_node = Node::new(NodeKind::Num, inf, inf, Some(0), 0);
            self.nodes.push(zero_node);
            let new_node = Node::new(NodeKind::Sub, self.nodes.len() - 1, self.primary(), None, 0);
            self.nodes.push(new_node);
            return self.nodes.len() - 1;
        }

        self.primary()
    }

    fn primary(&mut self) -> usize {
        if self.tokens[self.read].consume(&mut self.read, "(") {
            let idx = self.expr();
            self.tokens[self.read].expect(&mut self.read, ")");
            return idx;
        }

        // 実装ミスっていたら配列外参照で落ちるようにしておく
        let inf = 1e10 as usize;

        if self.tokens[self.read].kind == TokenKind::Ident {
            // 変数の場合
            let var_name = self.tokens[self.read].str.clone();
            let offset = match self.vars.get(&var_name) {
                Some(val) => val.offset,
                None => {
                    let new_var = LVar::new(var_name.clone(), self.offset);
                    self.vars.insert(var_name, new_var.clone());
                    self.offset += 8;
                    new_var.offset
                }
            };
            let new_node = Node::new(NodeKind::LocalVar, inf, inf, None, offset);
            self.read += 1; // TODO: ここもっといい実装ある気がするけど一旦放置で
            self.nodes.push(new_node);
        } else {
            // 数字の場合
            let new_node = Node::new(
                NodeKind::Num,
                inf,
                inf,
                Some(self.tokens[self.read].expect_number(&mut self.read)),
                0,
            );
            self.nodes.push(new_node);
        }
        self.nodes.len() - 1
    }
}
