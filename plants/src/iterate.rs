use crate::pattern;
use crate::symbolstring::SymbolString;
use crate::iter_ctx::IterCtx;


// Apply rules once from left to right on the given word.
pub fn iterate(s : &SymbolString, patterns : &mut Vec<pattern::Pattern>,
               ctx : &IterCtx) -> SymbolString {
    let mut result = SymbolString::empty();

    for i in 0..s.len() {
        let mut found = false;
        for p in patterns.iter_mut() {
            if p.test(i, s, ctx.ignored.as_str()) {
                result.push_str(&p.replacement);
                found = true;
                break;
            }
        }
        if !found {
            result.push(s.symbols[i].clone())
        }
    }

    result
}
