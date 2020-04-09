use crate::arith::Arith;

use crate::symbol::Symbol;
use crate::ast::AstNode;
use crate::lexer::TokenType;
use crate::parse_rules;
use crate::lexer::lexer;
use std::iter::FromIterator;

#[derive(Clone)]
pub struct SymbolString {
    pub symbols : Vec<Symbol>
}


impl SymbolString {
    pub fn from_ast(exp: &Box<AstNode>) -> Result<SymbolString, &'static str> {
        if exp.node_type != TokenType::Lctx && exp.node_type != TokenType::Rctx
            && exp.node_type != TokenType::Replacement && exp.node_type != TokenType::Pred
        && exp.node_type != TokenType::Pat {
            Err("SymbolString creation failed: invalid node type.")
        } else {
            let symbols = exp.children.iter()
                .map(|x| {
                   match Symbol::from_ast(x) {
                       Ok(a) => a,
                       Err(e) => {
                           println!("Symbol [{:?}] creation failed: {}", x.node_type, e);
                            Symbol{
                                sym: 'a',
                                var_names: Vec::new(),
                                params: Vec::new()
                       }}
                   }
                }).collect();
            Ok(SymbolString{symbols})
        }
    }

    pub fn from_string(exp: &str) -> Result<SymbolString, &'static str> {
        let tokens = lexer(exp);
        let ast = match parse_rules::pat(&tokens, 0) {
            (Some(a), _) => a,
            _ => {return Err("Could not parse expression to SymbolString");}
        };

        SymbolString::from_ast(&ast)
    }

    pub fn empty() -> SymbolString {
        SymbolString{symbols: Vec::new()}
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
        let mut fail = true;
        for mut sym in &mut self.symbols {
            if let Ok(()) = sym.set(var, val) {
                fail = false;
            }
        }

        if fail {
            Err(())
        } else {
            Ok(())
        }
    }

    pub fn n_params(&self) -> usize {
        let mut res = 0;
        for s in &self.symbols {
            res += s.n_param();
        }

        res
    }

    pub fn set_vec(&mut self, vec: &Vec<f32>) {
        let mut i = 0;
        for sym in self.symbols.iter_mut() {
            for j in 0..sym.n_param() {
                sym.set_i(j, vec[i + j]);
            }
            i += sym.n_param();
        }
    }

    pub fn get_vec(&self) -> Vec<f32> {
        let mut res = Vec::new();
        for sym in self.symbols.iter() {
            for j in 0..sym.n_param() {
                res.push(sym.get_i(j).expect("Error while getting parameter vec."));
            }
        }

        res
    }

    pub fn push(&mut self, sym: Symbol) {
        self.symbols.push(sym);
    }

    pub fn push_str(&mut self, s: &SymbolString) {
        for i in &s.symbols {
            self.push(i.clone());
        }
    }

    pub fn len(&self) -> usize {
        self.symbols.len()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Symbol> {
        self.symbols.iter()
    }
}

impl<'a> FromIterator<&'a Symbol> for SymbolString {
    fn from_iter<T: IntoIterator<Item=&'a Symbol>>(iter: T) -> Self {
        let mut s = SymbolString{symbols: Vec::new()};

        for i in iter {
            s.symbols.push(i.clone());
        }

        s
    }
}
