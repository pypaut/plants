use crate::lexer::{self, TokenType};

pub struct AstNode {
    pub data : String,
    pub children : Vec<Box<AstNode>>,
    pub node_type : TokenType
}

impl AstNode {
    pub fn print(&self, depth: usize) {
        for _ in 0..depth {
            print!("    ");
        }

        println!("{:?}: {}", self.node_type, self.data);

        for c in &self.children {
            c.print(depth + 1);
        }
    }
}