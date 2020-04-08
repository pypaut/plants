use core::borrow::BorrowMut;
use rand::{thread_rng, Rng};
use std::cmp::Ordering::{Less, Equal, Greater};
use std::cmp::Ordering;
use std::iter::{Iterator, Rev};
use std::str::Chars;
use crate::symbolstring::{SymbolString};
use crate::symbol::Symbol;
use crate::bool_exp::BoolExp;
use std::collections::HashMap;

pub struct Pattern {
    pub pattern : Symbol,       // Initial character
    pub replacement : SymbolString, // Replacement string
    pub p : f32,               // Replacement probability
    pub left : Option<SymbolString>,          // Left context
    pub right : Option<SymbolString>,         // Right context
    pub cond : Option<Box<BoolExp>> //condition
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::BorrowMut;

    fn test_rctx(s : &str, pat : &str) -> bool {
        let s = match SymbolString::from_string(s) {
            Ok(s) => s,
            _ => {return false;}
        };
        let ctx = match SymbolString::from_string(pat) {
            Ok(s) => s,
            _ => {return false;}
        };
        let (b, _) = Pattern::rctx(s.iter(), ctx.iter().borrow_mut(),
                      "");
        b
    }

    fn test_lctx(s : &str, pat : &str) -> bool {
        let s = match SymbolString::from_string(s) {
            Ok(s) => s,
            _ => {return false;}
        };
        let ctx = match SymbolString::from_string(pat) {
            Ok(s) => s,
            _ => {return false;}
        };
        let (b, _) = Pattern::lctx(s.iter().rev(), ctx.iter().rev().borrow_mut(),
                      "");
        b
    }

    #[test]
    fn rctx_true() {
        let res = test_rctx("bc", "bc");

        assert!(res);
    }

    #[test]
    fn rctx_false() {
        let res = test_rctx("bc", "d");

        assert!(!res);
    }

    #[test]
    fn rctx_true_short() {
        let res = test_rctx("bc", "b");

        assert!(res);
    }

    #[test]
    fn rctx_false_start() {
        let res = test_rctx("aabc", "bc");

        assert!(!res);
    }

    #[test]
    fn lctx_true() {
        let res = test_lctx("bc", "bc");

        assert!(res);
    }

    #[test]
    fn lctx_false() {
        let res = test_lctx("bc", "d");

        assert!(!res);
    }

    #[test]
    fn lctx_true_short() {
        let res = test_lctx("abc", "bc");

        assert!(res);
    }

    #[test]
    fn lctx_false_start() {
        let res = test_lctx("bca", "c");

        assert!(!res);
    }

    #[test]
    fn lctx_bracket_true() {
        let res = test_lctx("bc[abdhj[gfh]][", "bc");

        assert!(res);
    }

    #[test]
    fn lctx_bracket_false() {
        let res = test_lctx("bc[abdhj[gfh]][", "d");

        assert!(!res);
    }

    #[test]
    fn rctx_bracket_neg_lvl_false() {
        let res = test_rctx("b]c", "bc");

        assert!(!res);
    }

    #[test]
    fn rctx_bracket_true_simple() {
        let res = test_rctx("b[]c", "bc");

        assert!(res);
    }

    #[test]
    fn rctx_bracket_true_with_branch() {
        let res = test_rctx("b[ae]c", "bc");

        assert!(res);
    }

    #[test]
    fn rctx_bracket_cmp_branch() {
        let res = test_rctx("b[c]d", "b[c]d");

        assert!(res);
    }

    #[test]
    fn rctx_bracket_cmp_branch_multiple_levels() {
        let res = test_rctx("b[c[ae]]d", "b[c]d");

        assert!(res);
    }

    #[test]
    fn rctx_bracket_cmp_branch_complex() {
        let res = test_rctx("b[c[ae]kl]d", "b[c]d");

        assert!(res);
    }

    #[test]
    fn rctx_bracket_cmp_branch_complex_2() {
        let res = test_rctx("b[c[ae]kl][vb]d", "b[c[a]][v]d");

        assert!(res);
    }

    #[test]
    fn rctx_bracket_cmp_branch_false() {
        let res = test_rctx("b[c[ae]kl][vb]d", "b[c[aj]k][v]d");

        assert!(!res);
    }
}

impl Pattern {
    pub fn new<'a>(pat : Symbol, r : SymbolString, p : f32,
               left : Option<SymbolString>, right : Option<SymbolString>,
                cond : Option<Box<BoolExp>>) -> Pattern {
        Pattern{pattern: pat, replacement: r, p, left, right, cond}
    }

    fn rctx(it : std::slice::Iter<'_, Symbol>,
                   ctx : &mut std::slice::Iter<'_, Symbol>,
                   ignore : &str) -> (bool, Vec<f32>) {
        let mut cur = match ctx.next() {
            Some(c) => c,
            None => return (true, Vec::new())
        };

        let mut values = Vec::new();
        let mut lvl = 0;
        let mut pat_lvl = if cur == &'[' {1} else {0};

        for c in it {
            if c == &'[' {
                lvl += 1;
            }
            else if c == &']' {
                lvl -= 1;
            }
            if ignore.contains(&c.to_string())
            {
                continue
            }
            else if c == cur && lvl >= 0 && lvl == pat_lvl {
                let mut params = c.get_vec();
                values.append(&mut params);
                cur = match ctx.next() {
                    Some(c) => c,
                    None => return (true, values)
                };
                if cur == &'[' {
                    pat_lvl += 1;
                }
                else if cur == &']' {
                    pat_lvl -= 1;
                }
            }
            else {
                if lvl >= 0 && (c == &'[' || c == &']' || lvl != pat_lvl) {
                    continue;
                }
                else {
                    return (false, Vec::new());
                }
            }
        }

        (false, Vec::new())
    }

    fn lctx(it : Rev<std::slice::Iter<Symbol>>,
            ctx : &mut Rev<std::slice::Iter<'_, Symbol>>,
            ignore : &str) -> (bool, Vec<f32>) {
        let mut cur = match ctx.next() {
            Some(c) => c,
            None => return (true, Vec::new())
        };

        let mut values = Vec::new();
        let mut min_lvl = 0;//minimum level explored
        let mut lvl = 0;//level of current context
        //it should never go up

        for c in it {
            if c == &'[' {
                lvl -= 1;
                if lvl < min_lvl {
                    min_lvl = lvl;
                }
            }
            else if c == &']' {
                lvl += 1;
            }

            //ignore branches that came before the current char
            //because they are not part of the left context
            else if lvl <= min_lvl  && !ignore.contains(&c.to_string()) {
                if c == cur {
                    let mut params = c.get_vec();
                    params.append(&mut values);
                    values = params;
                    cur = match ctx.next() {
                        Some(c) => c,
                        None => return (true, values)
                    };
                } else {
                    return (false, Vec::new());
                }
            }
            else {
                continue;
            }
        }

        (false, Vec::new())
    }

    pub fn test(&mut self, i : usize, s : &SymbolString, ignored : &str) -> bool {

        let mut rng = thread_rng();
        if rng.gen_bool(self.p.into()) {
            //if (self.left == ' ') && (self.right == ' ') {  // No context
            let mut valid = s.symbols[i] == self.pattern;
            let pat_values = if !valid {
                return false;
            } else {
                s.symbols[i].get_vec()
            };
            //partition string in left and right part
            let left_str : SymbolString = s.iter().take(i).collect();
            let right_str : SymbolString = s.iter().skip(i + 1).collect();
            //if we have a left context, check the left context
            let (valid_tmp, lctx_values) = match &self.left {
                Some(ctx) => Pattern::lctx(left_str.iter().rev(),
                                                     ctx.symbols.iter().rev().borrow_mut(),
                                           ignored),
                None => (true, Vec::new())
            };
            valid &= valid_tmp;
            if !valid {
                return false;
            }
            //if we have a right context, check the right context
            let (valid_tmp, rctx_values) = match &self.right {
                Some(ctx) => Pattern::rctx(right_str.iter(),
                                                     ctx.symbols.iter().borrow_mut(),
                                           ignored),
                None => (true, Vec::new())
            };
            valid &= valid_tmp;
            if !valid {
                return false;
            }

            //set values in lctx, rctx, pred
            let mut lvars = match &mut self.left {
                Some(ctx) => {
                    ctx.vars()
                },
                None => {
                    Vec::new()
                }
            };
            let mut rvars = match &mut self.right {
                Some(ctx) => {
                    ctx.vars()
                },
                None => {
                    Vec::new()
                }
            };
            self.pattern.compute_var_names();

            let mut pat_vars = &self.pattern.var_names.clone();

            //bind variable names to values in a dictionary
            let mut bindings = HashMap::new();
            //println!("{:?}->{:?}", lvars, lctx_values);
            //println!("{:?}->{:?}", rvars, rctx_values);
            for (i, var) in lvars.iter().enumerate() {
                bindings.insert(*var, lctx_values[i].clone());
            }
            for (i, var) in rvars.iter().enumerate() {
                bindings.insert(*var, rctx_values[i].clone());
            }
            for (i, var) in pat_vars.iter().enumerate() {
                bindings.insert(var, pat_values[i].clone());
            }

            println!("Bindings: {:?}", bindings);
            //set values in condition
            if let Some(cond) = &self.cond {
                let cond_vars = cond.vars();
                for v in cond_vars {
                    if !bindings.contains_key(v) {
                        panic!("Could not read cond variable in binding table.");
                    } else {
                        let mut cond_tmp = cond.clone();
                        cond_tmp.set(v, bindings[v]);
                        if !cond_tmp.eval() {
                            return false;
                        }
                    }
                }
            }

            //set values in replacement
            //not good for performance but I don't know how to do
            let mut replace_vars = self.replacement.clone();
            let replace_vars = replace_vars.vars();
            for v in replace_vars {
                if !bindings.contains_key(v) {
                    panic!("Could not read replacement variable in binding table.");
                } else {
                    self.replacement.set(v, bindings[v]);
                }
            }

            valid
        }
        else {
            false
        }
    }

    // Sort list from contexted to context free.
    pub fn cmp_pat(&self, pat : &Pattern) -> Ordering {
        if self.left.is_none() && self.right.is_none() {  // 2 None
            if !pat.left.is_none() || !pat.right.is_none() {  // 0-1 None
                Greater
            }
            else {  // pat.left == None && pat.right == None  // 2 None
                Equal
            }
        }
        else if self.left.is_none() || self.right.is_none() {  // 1 None
            if pat.left.is_none() && pat.right.is_none() {  // 2 None
                Less
            }
            else if pat.left.is_none() || pat.right.is_none() {  // 1 None
                Equal
            }
            else {  // pat.left != None && pat.right != None  // 0 None
                Greater
            }
        }
        else {  // self.left != None && self.right != None  // 0 None
            if pat.left.is_some() && pat.right.is_some() {  // 0 None
                Equal
            }
            else {  // 1-2 None
                Less
            }
        }
    }
}
