use crate::arith;
use crate::arith::Arith;
use crate::ast::AstNode;
use crate::lexer::{lexer, TokenType};
use crate::ast_to_arith;


pub struct Symbol {
    pub sym: char,
    pub var_names : Vec<String>,
    pub ariths : Vec<Box<dyn arith::Arith>>
}

impl Symbol {
    pub fn from_ast(exp: &Box<AstNode>) -> Result<Symbol, &str> {
        if exp.node_type != TokenType::ParamWord && exp.node_type != TokenType::Pred {
            Err("Invalid node type, expected ParamWord|Pred.")
        } else {
            if exp.children.is_empty() {
                Err("No data for ast conversion to Symbol")
            } else {
                let sym = exp.children[0].data;
                let params = exp.children[1..].iter()
                    .filter_map(|x| {
                        if x.children.is_empty() {
                            None
                        } else {
                            Some(Arith::create_from(&x.children[0]))
                        }
                    }).collect();
                Ok(Symbol{sym: sym.chars().nth(0).unwrap(),
                    var_names: Vec::new(),
                    ariths: params.copy()})
            }
        }
    }

    pub fn compute_var_names(&self) {
        for arith in self.ariths {
            let vec = arith.vars();
            for var in vec {
                self.var_names.push(var);
            }
        }
        self.var_names.sort();
        self.var_names.dedup();
    }

    pub fn to_string(&self) -> String {
        let mut res = c.to_string();
        if var_names.len() == 0 {
            res
        } else {
            res.push_back(&("(".to_string()));
            // add values...
            res.push_back(&(")".to_string()));
        }

        res
    }

    pub fn change_var(&self) {

    }
}
