use crate::pattern;


// Apply rules once from left to right on the given word.
pub fn iterate(s : &str, patterns : &Vec<pattern::Pattern>,
               ignored : &Vec<char>) -> String {
    let mut result = String::new();

    for i in 0..s.len() {
        let mut found = false;
        for p in patterns.iter() {
            if p.test(i, s.to_string()) {
                result.push_str(&p.replacement);
                found = true;
                break;
            }
        }
        if !found {
            result.push(s.chars().nth(i).unwrap())
        }
    }

    result
}
