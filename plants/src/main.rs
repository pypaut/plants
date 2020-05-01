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
    //let axiom = args[1].clone();                        // Base word to iterate upon
    let in_file = args[1].clone();                      // File containing rules
    let out_file = args[2].clone(); //output file name
    let save_iter = if args.len() > 3 {
        args[3]
            .parse::<usize>().expect("Invalid value for save_iter.")
    } else {0};
    let save_iter = if save_iter == 1 {true} else {false};
    //let iterations = args[3].parse::<i32>().unwrap();   // Wanted depth

    // Parse shapes
    let shapes_str = fs::read_dir("../grammars").unwrap();
    let mut shapesIterCtx : HashMap<String, iter_ctx::IterCtx> = HashMap::new();
    let mut shapesPatterns : HashMap<String, Vec<pattern::Pattern>> = HashMap::new();

    for shape in shapes_str {
        let shape_str = match shape.unwrap().file_name().into_string() {
            Ok(string) => string,
            Err(e) => {
                println!("Error parsing shapes.");
                String::from("")
            }
        };
        let shape_rules_str = fs::read_to_string(shape_str.clone())
            .expect("Failed reading file");
        let (rules, ctx) = parse_rules::parse_rules(&shape_rules_str, HashMap::new());

        shapesIterCtx.insert(shape_str.clone(), ctx);
        shapesPatterns.insert(shape_str.clone(), rules);
    }


    // Parse rules
    let rule_str = fs::read_to_string(in_file)
        .expect("Failed reading file.");
    let (mut rules, ctx) = parse_rules::parse_rules(&rule_str, shapesIterCtx);

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
