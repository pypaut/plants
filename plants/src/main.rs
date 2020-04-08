use std::env;
use std::fs;
use crate::symbolstring::SymbolString;

mod pattern;
mod iterate;
mod parse_rules;
mod lexer;
mod ast;
mod arith;
mod ast_to_arith;
mod bool_exp;
mod ast_to_boolexp;
mod symbol;
mod symbolstring;
mod iter_ctx;


fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    //let axiom = args[1].clone();                        // Base word to iterate upon
    let in_file = args[1].clone();                      // File containing rules
    let out_file = args[2].clone(); //output file name
    //let iterations = args[3].parse::<i32>().unwrap();   // Wanted depth

    let rule_str = fs::read_to_string(in_file)
        .expect("Failed reading file.");
    let (rules, ctx) = parse_rules::parse_rules(&rule_str);

    //println!("{:?}", rules);
    //println!("{:?}", ignored);
    let mut res = SymbolString::from_string(ctx.axion.as_str())?;

    for _i in 0..ctx.n_iter {
        res = iterate::iterate(&res,
                               &rules, &ctx);
    }

    fs::write(out_file, res.to_string())
        .expect("Unable to write to 'result.txt' file");

    Ok(())
}
