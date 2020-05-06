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

fn get_output_string(header: &String, contents: &SymbolString) -> String {
    if header.len() > 0 {
        format!("{}\n{}", header, contents.to_string())
    } else {
        contents.to_string()
    }
}

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    let in_file = args[1].clone();                      // File containing rules
    let mut path_split = in_file.rsplitn(2, "/");
    path_split.next();
    let file_folder = match path_split.next() {
        Some(s) => s.to_string(),
        _ => ".".to_string()
    };
    let out_file = args[2].clone(); //output file name
    let save_iter = if args.len() > 3 {
        args[3]
            .parse::<usize>().expect("Invalid value for save_iter.")
    } else {0};
    let save_iter = if save_iter == 1 {true} else {false};

    // Parse rules
    let rule_str = fs::read_to_string(in_file)
        .expect("Failed reading file.");
    let mut ctx = parse_rules::parse_rules(&rule_str);

    for r in &mut ctx.patterns {
        r.rule_set(&"root".to_string());
    }

    // Parse included rules
    let mut shapesCtx : HashMap<String, iter_ctx::IterCtx> = HashMap::new();
    let mut shapesRes : HashMap<String, SymbolString> = HashMap::new();

    for (alias, file) in ctx.include.iter() {
        println!("Importing file: {}/{}", file_folder, file);
        let shape_rule_str = fs::read_to_string(format!("{}/{}", file_folder, file))
            .expect("Failed reading file");
        let mut shape_ctx = parse_rules::parse_rules(&shape_rule_str);
        for pat in &mut shape_ctx.patterns {
            pat.rule_set(alias);
        }

        shapesCtx.insert(alias.to_string(), shape_ctx);

        let shape_res = match SymbolString::from_string(shapesCtx
            .get(alias).unwrap().axiom.as_str()) {
            Ok(mut sym) => {sym.rule_set(alias); sym},
            Err(e) => {
                println!("Error parsing included rules : {}", e);
                SymbolString{symbols : Vec::new()}
            }
        };
        shapesRes.insert(alias.to_string(), shape_res);
    }

    //replace alias in "root" to the correct value
    for pat in &mut ctx.patterns {
        for (alias, value) in &shapesRes {
            pat.replace(&alias, &value);
        }
    }

    //println!("{:?}", rules);
    //println!("{:?}", ignored);
    let mut res = SymbolString::from_string(ctx.axiom.as_str())?;
    res.rule_set(&"root".to_string());

    //add root ctx to IterCtx map
    shapesCtx.insert("root".to_string(), ctx);

    //create the output header string
    let mut output_header = String::from("#");
    for (rule_set, ctx) in &shapesCtx {
        output_header.push_str(&ctx.get_object_header(rule_set, &file_folder));
    }
    let output_header = if output_header.len() > 1 {
        output_header
    } else {
        String::new()
    };

    //iterate
    let n_iter = shapesCtx["root"].n_iter;
    for i in 0..n_iter {
        // Iterate once on final res
        res = iterate::iterate(
            &res,
            &mut shapesCtx
        );
        //println!("-----------------------------");
        if save_iter {
            let out_tmp = format!("{}{}", out_file, i.to_string());
            println!("Saving {}", out_tmp);
            fs::write(out_tmp, get_output_string(&output_header, &res))
                .expect("Unable to write to temporary output file.");
        }
    }

    fs::write(out_file, get_output_string(&output_header, &res))
        .expect("Unable to write to output file");

    Ok(())
}
