use crate::arith;
use crate::arith::Arith;

use crate::symbol;
use crate::symbol::Symbol;
use crate::ast::AstNode;
use crate::lexer::TokenType;


pub struct SymbolString {
    pub symbols : Vec<Symbol>
}


impl SymbolString {
    pub fn from_ast(exp: &Box<AstNode>) -> Result<SymbolString, &str> {
        if exp.node_type != TokenType::Lctx && exp.node_type != TokenType::Rctx
            && exp.node_type != TokenType::Replacement {
            Err("SymbolString creation failed: invalid node type.")
        } else {
            let symbols = exp.children.iter()
                .map(|x| {
                   Symbol::from_ast(x)
                }).collect();
            Ok(SymbolString{symbols})
        }
    }

    pub fn compute_var_names(&self) {
        for sym in self.symbols {
            sym.compute_var_names();
        }
    }

    pub fn to_string(&self) -> String {
        let mut res = "".to_string();
        
        for sym in self.symbols {
            res.push_back(&sym.to_string());
        }

        res;
    }

    pub fn change_var(&self) {
        for sym in self.symbols {
            sym.change_var();
        }
    }
}
