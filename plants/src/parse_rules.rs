use crate::pattern::Pattern;
use std::iter::FromIterator;
use crate::lexer::{lexer, TokenType};
use std::collections::VecDeque;
use std::borrow::Borrow;
use crate::lexer::TokenType::ParamWord;

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

type AstRet = ((Option<Box<AstNode>>, usize))

fn lctx(tokens : &VecDeque<lexer::Token>, index : usize) -> AstRet {

}

fn p_word(tokens : &VecDeque<lexer::Token>, index : usize) -> AstRet {
    if tokens[index].toktype != TokenType::Char && tokens[index].toktype != TokenType::Letter {
        return (None, index)
    }

    let mut i = index;
    let mut result = AstNode{data: String::new(),
        children: vec!(Box::new(AstNode{data: tokens[index].val, children: Vec::new(),
        node_type: TokenType::Word})),
    node_type: TokenType::ParamWord};
    //create an ast node with first child being the word and second child the parameter

    i += 1;
    //check if there is a parameter
    if tokens[i].toktype == TokenType::LsepParam {
        let (w, i) = word(tokens, i);
        match w {
            Some(w) => {
                w.node_type = TokenType::Param;
                result.children.push(w);
            },
            None => {}
        };

        //check closing separator
        if tokens[i].toktype != TokenType::RsepParam {
            return (None, index);
        }
    }

    (Some(Box::new(result)), i)
}

fn prob() -> AstRet {
    let mut i = index;
    if tokens[i].toktype != TokenType::Lpsep {
        return (None, index);
    }

    i += 1;
    if tokens[i].toktype != TokenType::Number {
        return (None, index);
    }

    let ret = AstNode{data: tokens[i].val, children: Vec::new(), node_type: TokenType::Prob};

    i += 1;

    if tokens[i].toktype != TokenType::Rpsep {
        return (None, index);
    }

    (Some(Box::new(ret)), i)
}

//word rule: letter+
fn word(tokens : &VecDeque<lexer::Token>, index : usize) -> AstRet {
    let mut res_str = String::new();

    let mut i = index;
    while tokens[i].toktype == TokenType::Letter {
        res_str.push_str(tokens[i].val);
    }

    if res_str.len() > 0 {
        (Some(Box::new(AstNode{data: res_str, children: Vec::new(),
            node_type: TokenType::Word})), i)
    }
    else {
        (None, index)
    }
}

//pattern rule: char+
fn pat(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
}

fn param(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    let mut res_str = String::new();

    let mut i = index;
    while tokens[i].toktype != TokenType::Ws {
        res_str.push_str(tokens[i].val);
    }

    if res_str.len() > 0 {
        (Some(Box::new(AstNode{data: res_str, children: Vec::new(),
            node_type: TokenType::Param})), i)
    }
    else {
        (None, index)
    }
}

fn preproc(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    let mut i = index;
    if tokens[index].toktype != TokenType::PreprocStart {
        return (None, index);
    }
    i += 1;

    let mut result = match word(tokens, index + 1) {
        (Some(w), j) => {w.node_type = TokenType::Preproc;
            i = j;
            w
        }
        (None, _) => {return (None, index);}
    };

    let mut valid = true;
    while valid {
        if tokens[i].toktype == TokenType::Ws {
            i += 1;
        }
        else {
            valid = false;
            break;
        }

        match param(tokens, i) {
            (Some(p), j) => {result.children.push(p); i = j}
            (None, _) => {break;}
        }
    }

    if valid {
        (Result, i)
    }
    else {
        (None, index)
    }
}

fn parse(s : String) -> Option<Box<AstNode>> {
    let tokens = lexer(&s);
    let mut result = Box::new(AstNode{data: String::new(), children: Vec::new(),
        node_type: TokenType::Rule});
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
