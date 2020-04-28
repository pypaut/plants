use std::env;
use std::fs;
use crate::symbolstring::SymbolString;

mod pattern;
mod iterate;
mod parse_rules;
mod parse_shapes;
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

    let shapes_str = fs::read_to_string("../shapes")
        .expect("Failed reading shapes file.");
    let shapes = parse_shapes::parse_shapes(&shapes_str);

    let rule_str = fs::read_to_string(in_file)
        .expect("Failed reading file.");
    let (mut rules, ctx) = parse_rules::parse_rules(&rule_str);

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
