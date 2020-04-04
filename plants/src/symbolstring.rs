use crate::arith;
use crate::arith::Arith;

use crate::symbol;
use crate::symbol::Symbol;


pub struct SymbolString {
    pub symbols : Vec<Symbol>
}


impl SymbolString {
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
