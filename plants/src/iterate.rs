use crate::pattern;
use crate::symbolstring::SymbolString;


// Apply rules once from left to right on the given word.
pub fn iterate<'a>(s : &SymbolString, patterns : &Vec<pattern::Pattern>,
               ignored : &str) -> SymbolString {
    let mut result = SymbolString::from_string("").unwrap();

    for i in 0..s.len() {
        let mut found = false;
        for p in patterns.iter() {
            if p.test(i, s, ignored) {
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
