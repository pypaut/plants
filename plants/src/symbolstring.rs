use crate::arith::Arith;

use crate::symbol::Symbol;
use crate::ast::AstNode;
use crate::lexer::TokenType;

pub struct SymbolString {
    pub symbols : Vec<Symbol>
}


impl SymbolString {
    pub fn from_ast(exp: &Box<AstNode>) -> Result<SymbolString, &'static str> {
        if exp.node_type != TokenType::Lctx && exp.node_type != TokenType::Rctx
            && exp.node_type != TokenType::Replacement {
            Err("SymbolString creation failed: invalid node type.")
        } else {
            let symbols = exp.children.iter()
                .map(|x| {
                   Symbol::from_ast(x).unwrap()
                }).collect();
            Ok(SymbolString{symbols})
        }
    }

    pub fn vars(&mut self) -> Vec<&str> {
        let mut result = Vec::new();
        for mut sym in &mut self.symbols {
            sym.compute_var_names();
            //should maybe save this in a buffer
            for v in &sym.var_names {
                result.push(v.as_str());
            }
        }

        result
    }

    pub fn to_string(&self) -> String {
        let mut res = "".to_string();
        
        for sym in &self.symbols {
            res.push_str(&sym.to_string());
        }

        res
    }

    pub fn set(&mut self, var: &str, val: f32) -> Result<(), ()> {
        for mut sym in &mut self.symbols {
            sym.set(var, val)?;
        }

        Ok(())
    }
}
