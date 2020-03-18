use crate::pattern::Pattern;
use std::iter::FromIterator;
use crate::lexer::{lexer, TokenType};
use std::collections::VecDeque;
use std::borrow::Borrow;
use std::ptr::Unique;

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

struct AstNode {
    data : String,
    children : Vec<Box<AstNode>>,
    node_type : TokenType
}

fn lctx(tokens : &VecDeque<lexer::Token>, index : usize) -> Option<Box<AstNode>> {

}

fn p_word(tokens : &VecDeque<lexer::Token>, index : usize) -> Option<Box<AstNode>> {

}

fn parse(s : String) -> Option<Box<AstNode>> {
    let tokens = lexer(&s);
    let mut result = Box(Unique(AstNode{data: String::new(), children: Vec::new(),
        node_type: TokenType::Rule}));

    let tok = &tokens[0];
    if tok.toktype != TokenType::Word {
        panic!("Expected Word.");
    }

    match  lctx(&tokens, 0) {
        //create a lctx token and add it to children of the main mode
        Some(ctx) => {},
        //we don't have a lctx node, the first word is the pattern
        None => ()
    };

    None
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

    result.sort_by(|a, b| a.cmp_pat(b));
    (result, ignored)
}
