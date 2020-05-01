use std::collections::HashMap;
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
    let in_file = args[1].clone();                      // File containing rules
    let out_file = args[2].clone(); //output file name
    let save_iter = if args.len() > 3 {
        args[3]
            .parse::<usize>().expect("Invalid value for save_iter.")
    } else {0};
    let save_iter = if save_iter == 1 {true} else {false};

    // Parse rules
    let rule_str = fs::read_to_string(in_file)
        .expect("Failed reading file.");
    let (mut rules, ctx) = parse_rules::parse_rules(&rule_str);

    // Parse included rules
    let mut shapesRules : HashMap<String, Vec<pattern::Pattern>> = HashMap::new();
    let mut shapesCtx : HashMap<String, iter_ctx::IterCtx> = HashMap::new();
    let mut shapesRes : HashMap<String, SymbolString> = HashMap::new();

    for (alias, file) in ctx.include.iter() {
        let shape_rule_str = fs::read_to_string(file)
            .expect("Failed reading file");
        let (shape_rules, shape_ctx) = parse_rules::parse_rules(&shape_rule_str);
        shapesRules.insert(alias.to_string(), shape_rules);
        shapesCtx.insert(alias.to_string(), shape_ctx);

        let shape_res = match SymbolString::from_string(shapesCtx.get(alias).unwrap().axion.as_str()) {
            Ok(sym) => sym,
            Err(e) => {
                println!("Error parsing included rules : {}", e);
                SymbolString{symbols : Vec::new()}
            }
        };
        shapesRes.insert(alias.to_string(), shape_res);
    }

    //println!("{:?}", rules);
    //println!("{:?}", ignored);
    let mut res = SymbolString::from_string(ctx.axion.as_str())?;

    for i in 0..ctx.n_iter {
        res = iterate::iterate(&res,
                               &mut rules, &ctx);
        //println!("-----------------------------");
        if save_iter {
            let out_tmp = format!("{}{}", out_file, i.to_string());
            println!("Saving {}", out_tmp);
            fs::write(out_tmp, res.to_string())
                .expect("Unable to write to temporary output file.");
        }
    }

    fs::write(out_file, res.to_string())
        .expect("Unable to write to output file");

    Ok(())
}
