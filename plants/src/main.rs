struct Pattern {
    pattern : char,
    replacement : String
}

impl Pattern {
    fn new(p : char, r : String) -> Pattern {
        Pattern{pattern: p, replacement: r}
    }

    fn test(&self, c : char) -> bool {
        c == self.pattern
    }
}

fn iterate(s : &str, patterns : Vec<Pattern>) -> String{
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

fn main() {
    println!("Hello, world!");

    let res = iterate("b", vec![Pattern::new('b', String::from("a"))]);

    println!("{}", res);
    //write a file with the result string
}
