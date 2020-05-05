use crate::pattern;
use crate::symbolstring::SymbolString;
use crate::iter_ctx::{IterCtx, LightCtx};
use std::collections::HashMap;


// Apply rules once from left to right on the given word.
pub fn iterate(s : &SymbolString, ctx_list : &mut HashMap<String, IterCtx>) -> SymbolString {
    let mut result = SymbolString::empty();

    let light_ctx : HashMap<String, LightCtx> = ctx_list.iter()
        .map(|(s, ctx)| -> (String, LightCtx) {
            (s.clone(), ctx.to_light_ctx())
    }).collect();

    for i in 0..s.len() {
        let mut found = false;
        match ctx_list.get_mut(&s.symbols[i].rule_set) {
            Some(ctx) => {
                //println!("{:?}", ctx);
                for p in ctx.patterns.iter_mut() {
                    //println!("{:?}", p);
                    if p.test(i, s, &light_ctx[&s.symbols[i].rule_set]) {
                        result.push_str(&p.replacement);
                        found = true;
                        break;
                    }
                }},
            _ => {
                //println!("Could not find IterCtx: {}", &s.symbols[i].rule_set);
            }
        };
        if !found {
            result.push(s.symbols[i].clone())
        }
    }

    result
}
