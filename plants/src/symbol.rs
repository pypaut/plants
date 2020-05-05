use crate::arith;
use crate::arith::{Arith, ArithFactory};
use crate::ast_to_arith;
use crate::ast::AstNode;
use crate::lexer::{lexer, TokenType};
use crate::symbolstring::SymbolString;

#[derive(Clone, Debug)]
pub struct Symbol {
    pub sym: char,
    pub var_names : Vec<String>,
    pub params: Vec<Box<Arith>>,
    pub rule_set : String//rule set name for replacing the symbol
}

impl Symbol {
    pub fn new(sym: char, params: Vec<Box<Arith>>, rule_set: String) -> Symbol {
        Symbol{sym, params, var_names: Vec::new(), rule_set}
    }

    pub fn new_with_values(sym: char, params: Vec<f32>, rule_set: String) -> Symbol {
        let params = params.iter()
            .map(|x| {arith::Var::new_value(*x)}).collect();

        Symbol::new(sym, params, rule_set)
    }

    pub fn from_ast(exp: &Box<AstNode>) -> Result<Symbol, &'static str> {
        if exp.node_type != TokenType::ParamWord && exp.node_type != TokenType::Pred {
            Err("Invalid node type, expected ParamWord|Pred.")
        } else {
            if exp.children.is_empty() {
                Err("No data for ast conversion to Symbol")
            } else {
                let sym = &exp.children[0].data;
                let params: Vec<Box<Arith>> = exp.children[1..].iter()
                    .filter_map(|x| {
                        if x.children.is_empty() {
                            None
                        } else {
                            Some(Arith::create_from(&x.children[0]))
                        }
                    }).map(|x| {x.unwrap()}).collect();
                Ok(Symbol{sym: sym.chars().nth(0).unwrap(),
                    var_names: Vec::new(),
                    params,
                    rule_set: String::new()
                })
            }
        }
    }

    pub fn eq_alias(&self, alias: &String) -> bool {
        if !self.params.is_empty() || self.rule_set != "root".to_string() {
            false//if we have a parameter or we are not from the root file we return false
        } else {
            self.sym.to_string() == alias.to_string()//test if we have the correct symbol
        }
    }

    pub fn compute_var_names(&mut self) {
        //var names will be computed only once
        if self.var_names.len() > 0 {
            return;
        }

        for arith in &self.params {
            let vec = arith.vars();
            for var in vec {
                self.var_names.push(var.to_string());
            }
        }
        //self.var_names.sort();
        //self.var_names.dedup();
    }

    pub fn to_string(&self) -> String {
        let mut res = self.sym.to_string();
        if self.params.len() == 0 {
            res
        } else {
            res.push('(');
            // add values...
            for p in &self.params {
                res.push_str(&p.eval().to_string());
                res.push(',');
            }
            res.pop();
            res.push(')');
            res
        }
    }

    pub fn set(&mut self, var: &str, val: f32) -> Result<(), ()> {
        let mut fail = true;
        for mut p in &mut self.params {
            match p.set(var, val) {
                Ok(()) => {fail = false;},
                _ => {}
            }
        }

        if fail {
            Err(())
        } else {
            Ok(())
        }
    }

    //set variable at index i
    pub fn set_i(&mut self, i: usize, val: f32) -> Result<(), ()> {
        if i >= self.params.len() {
            Err(())
        } else {
            let var = String::from({
                let vars = self.params[i].vars();

                if vars.len() == 1 {
                    vars[0].clone()
                } else {
                    return Err(())
                }
            });
            self.params[i].set(var.as_str(), val)
        }
    }

    pub fn set_vec(&mut self, vec: &Vec<f32>) {
        if (vec.len() >= self.params.len()) {
            return;
        }
        for (i, x) in vec.iter().enumerate() {
            self.set_i(i, *x);
        }
    }

    pub fn get(&self, var: &str) -> Result<f32, ()> {
        for v in &self.params {
            let vars = v.vars();
            if vars.len() == 1 && vars[0] == var {
                return Ok(v.eval());
            }
        }

        Err(())
    }

    pub fn get_i(&self, i: usize) -> Result<f32, ()> {
        if i >= self.params.len() {
            Err(())
        } else {
            Ok(self.params[i].eval())
        }
    }

    pub fn get_vec(&self) -> Vec<f32> {
        self.params.iter().map(|x| x.eval()).collect()
    }

    pub fn n_param(&self) -> usize {
        self.params.len()
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.sym == other.sym && self.params.len() == other.params.len()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<char> for Symbol {
    fn eq(&self, other: &char) -> bool {
        self.sym == *other && self.params.is_empty()
    }

    fn ne(&self, other: &char) -> bool {
        !self.eq(other)
    }
}

impl Eq for Symbol {}
