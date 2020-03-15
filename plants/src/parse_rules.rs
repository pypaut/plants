use crate::pattern::Pattern;
use std::iter::FromIterator;

#[derive(Debug)]
enum LineType {
    Rule,//rules
    Context//lines begining with '#'
}

#[derive(Debug)]
enum TokType {
    Rctx,
    Lctx,
    Prob,
    Pattern,
    Replacement,
    Ctx,
    CtxArg
}

#[derive(Debug)]
struct SepLine {
    line_type : LineType,
    tokens : Vec<String>,
    tok_types : Vec<TokType>
}

fn separate(line : &str, sep : char) -> (Option<String>, &str) {
    let split : Vec<&str> = line.splitn(2, sep).collect();

    //we need to consider the left context
    if split.len() == 2 {
        (Some(String::from(split[0].trim())), split[1].trim())
    }
    else {
        (None, line.trim())
    }
}

fn tokenize(line : &str) -> Option<SepLine> {
    let mut chars = line.chars();

    //split at space, strip and return
    let first = match chars.next() {
        Some(c) => c,
        None => return None
    };
    if first == '#' {
        let line = String::from_iter(chars);
        let split : Vec<&str> = line.split(' ').collect();
        if split.len() < 2 {
            None
        }
        else {
            let line_type = LineType::Context;
            let tokens : Vec<String> = split.iter()
                .map(|x| String::from(x.trim()))
                .collect();

            let mut tok_types = vec![TokType::Ctx];
            for _i in 1..tokens.len() {
                tok_types.push(TokType::CtxArg);
            }
            Some(SepLine{line_type, tokens, tok_types})
        }
    }
    else {//reset iterator, split at '<', '>' and ':', strip and return
        let mut tokens : Vec<String> = Vec::new();
        let mut tok_types : Vec<TokType> = Vec::new();

        //separate pattern from replacement and probability
        let (tok, line) = separate(line, ':');
        let pattern = match tok {
            Some(tok) => {
                tok
            }
            None => {
                panic!("Error while parsing rule.");
            }
        };

        //separate replacement from probability
        let (tok, line) = separate(line, ':');
        match tok {
            Some(tok) => {
                tokens.push(tok);
                tok_types.push(TokType::Prob);
                tokens.push(String::from(line));
                tok_types.push(TokType::Replacement);
            },
            None => {
                tokens.push(String::from(line));
                tok_types.push(TokType::Replacement);
            }
        };

        //separate left context
        let (tok, line) = separate(&pattern, '<');
        match tok {
            Some(tok) => {
                tokens.push(tok);
                tok_types.push(TokType::Lctx);
            },
            None => ()
        };

        //separate right context and pattern
        let (tok, line) = separate(line, '>');
        match tok {
            Some(tok) => {
                tokens.push(tok);
                tok_types.push(TokType::Pattern);
                tokens.push(String::from(line));
                tok_types.push(TokType::Rctx);
            },
            None => {
                tokens.push(String::from(line));
                tok_types.push(TokType::Pattern);
            }
        };

        Some(SepLine{line_type: LineType::Rule, tokens, tok_types})
    }
}

fn create_rule(line : &SepLine) -> Pattern {
    let mut left : Option<String> = None;
    let mut right : Option<String> = None;
    let mut p : f32 = 1.0;
    let mut pattern : char = ' ';
    let mut replacement : String = String::new();

    for (i, tok) in line.tokens.iter().enumerate() {
        match line.tok_types[i] {
            TokType::Rctx => {right = Some(tok.to_string());},
            TokType::Lctx => {left = Some(tok.to_string());},
            TokType::Prob => {p = tok.parse::<f32>().unwrap()},
            TokType::Pattern => {pattern = tok.chars().next().unwrap()},
            TokType::Replacement => {replacement = tok.to_string()},
            _ => {}
        }
    }

    Pattern::new(pattern, replacement, p, left, right)
}

// Instantiate Pattern objects from a string.
pub fn parse_rules(data : &str) -> (Vec<Pattern>, String) {
    let mut result = Vec::new();
    let mut ignored : String = String::new();

    for l in data.lines() {
        //println!("{}", l);
        //println!("{:?}", tokenize(l));
        let line = tokenize(l);
        match line {
            Some(line) => {
                match line.line_type {
                    LineType::Context => {
                        if line.tokens[0] == String::from("ignore") {
                            ignored = line.tokens[1].clone();
                        }
                    },
                    LineType::Rule => {
                        result.push(create_rule(&line));
                    }
                };
            },
            None => continue
        };
    }

    (result, ignored)
}
