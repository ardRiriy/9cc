use crate::tokenizer::Token;

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
        Node {
            kind,
            lhs,
            rhs,
            val,
        }
    }
}

pub struct NodeTree {
    tokens: Vec<Token>,
    read: usize, // 読んだトークン数
    node_tree: Vec<Node>,
}

impl NodeTree {
    pub fn new(tokens: Vec<Token>) -> NodeTree {
        NodeTree {
            tokens,
            node_tree: vec![],
            read: 0,
        }
    }

    pub fn parse(&mut self) {
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

    fn expr(&mut self) -> usize {
        self.equality()
    }

    fn equality(&mut self) -> usize {
        // ==, !=
        let mut node = self.relational();

        while self.read < self.tokens.len() {
            if self.tokens[self.read].consume(&mut self.read, "==") {
                let new_node = Node::new(NodeKind::Eq, node, self.relational(), None);
                self.node_tree.push(new_node);
                node = self.node_tree.len() - 1;
            } else if self.tokens[self.read].consume(&mut self.read, "!=") {
                let new_node = Node::new(NodeKind::Nq, node, self.relational(), None);
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
                let new_node = Node::new(NodeKind::Lt, node, self.add(), None);
                self.node_tree.push(new_node);
                node = self.node_tree.len() - 1;
            } else if self.tokens[self.read].consume(&mut self.read, "<=") {
                let new_node = Node::new(NodeKind::Le, node, self.add(), None);
                self.node_tree.push(new_node);
                node = self.node_tree.len() - 1;
            } else if self.tokens[self.read].consume(&mut self.read, ">") {
                // 左辺と右辺を入れ替えて評価することでgenの実装量が減って嬉しい(>= も同様)
                let new_node = Node::new(NodeKind::Lt, self.add(), node, None);
                self.node_tree.push(new_node);
                node = self.node_tree.len() - 1;
            } else if self.tokens[self.read].consume(&mut self.read, ">=") {
                let new_node = Node::new(NodeKind::Le, self.add(), node, None);
                self.node_tree.push(new_node);
                node = self.node_tree.len() - 1;
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
                let new_node = Node::new(NodeKind::Add, node, self.mul(), None);
                self.node_tree.push(new_node);
                node = self.node_tree.len() - 1;
            } else if self.tokens[self.read].consume(&mut self.read, "-") {
                let new_node = Node::new(NodeKind::Sub, node, self.mul(), None);
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
                let new_node = Node::new(NodeKind::Mul, node, self.unary(), None);
                self.node_tree.push(new_node);
                node = self.node_tree.len() - 1;
            } else if self.tokens[self.read].consume(&mut self.read, "/") {
                let new_node = Node::new(NodeKind::Div, node, self.unary(), None);
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
                self.node_tree.len() - 1,
                self.primary(),
                None,
            );
            self.node_tree.push(new_node);
            return self.node_tree.len() - 1;
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
        let new_node = Node::new(
            NodeKind::Num,
            inf,
            inf,
            Some(self.tokens[self.read].expect_number(&mut self.read)),
        );
        self.node_tree.push(new_node);
        self.node_tree.len() - 1
    }
}
