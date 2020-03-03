use std::env;
use std::fs;

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

fn iterate(s : &str, patterns : &Vec<Pattern>) -> String{
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

fn parse_rules(data : &str) -> Vec<Pattern> {
    let mut result = Vec::new();

    for l in data.lines() {
        let split : Vec<&str> = l.split(':').collect();
        if split.len() != 2 {
            panic!("Invalid rule");
        }
        result.push(Pattern::new(split[0].chars().next().unwrap(),
                                 String::from(split[1])));
    }

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let axiom = args[1].clone();
    let in_file = args[2].clone();
    let iterations = args[3].parse::<i32>().unwrap();

    let rule_str = fs::read_to_string(in_file)
        .expect("Failed reading file.");
    let rules = parse_rules(&rule_str);

    let mut res = axiom;

    for _i in 0..iterations {
        res = iterate(&res, &rules);
    }

    println!("{}", res);
    //write a file with the result string
}
