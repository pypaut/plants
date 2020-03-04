use crate::pattern;


// Apply rules once from left to right on the given word.
pub fn iterate(s : &str, patterns : &Vec<pattern::Pattern>) -> String {
    let mut result = String::new();

    for c in s.chars() {
        let mut found = false;
        for p in patterns.iter() {
            if p.test(c) {
                result.push_str(&p.replacement);
                found = true;
                break;
            }
        }
        if !found {
            result.push(c)
        }
    }

    result
}
