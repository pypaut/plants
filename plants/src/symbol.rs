use crate::arith;
use crate::arith::Arith;


pub struct Symbol {
    pub c : char,
    pub var_names : Vec<String>,
    pub ariths : Vec<Box<dyn arith::Arith>>
}

impl Symbol {
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
