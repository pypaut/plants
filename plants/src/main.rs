use std::env;
use std::fs;

mod pattern;
mod iterate;
mod parse_rules;
mod lexer;
mod ast;
mod arith;
mod ast_to_arith;
mod bool_exp;
mod ast_to_boolexp;


fn main() {
    let args: Vec<String> = env::args().collect();
    let axiom = args[1].clone();                        // Base word to iterate upon
    let in_file = args[2].clone();                      // File containing rules
    let iterations = args[3].parse::<i32>().unwrap();   // Wanted depth

    let rule_str = fs::read_to_string(in_file)
        .expect("Failed reading file.");
    let (rules, ignored) = parse_rules::parse_rules(&rule_str);

    //println!("{:?}", rules);
    //println!("{:?}", ignored);
    let mut res = axiom;

    for _i in 0..iterations {
        res = iterate::iterate(&res, &rules, &ignored);
    }

    fs::write("result.txt", res).expect("Unable to write to 'result.txt' file");
}
