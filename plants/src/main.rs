use std::env;
use std::fs;

mod pattern;
mod iterate;
mod parse_rules;


fn main() {
    let args: Vec<String> = env::args().collect();
    let axiom = args[1].clone();                        // Base word to iterate upon
    let in_file = args[2].clone();                      // File containing rules
    let iterations = args[3].parse::<i32>().unwrap();   // Wanted depth

    let rule_str = fs::read_to_string(in_file)
        .expect("Failed reading file.");
    let rules = parse_rules::parse_rules(&rule_str);

    let mut res = axiom;

    for _i in 0..iterations {
        res = iterate::iterate(&res, &rules);
    }

    println!("{}", res);
    // TODO : Write a file with the result string
}
