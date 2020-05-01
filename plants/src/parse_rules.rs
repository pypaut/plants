use crate::pattern::{Pattern};
use crate::ast::AstNode;
use crate::lexer::{self, TokenType};
use crate::symbolstring::SymbolString;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt;
use crate::symbol::Symbol;
use crate::iter_ctx::IterCtx;
use crate::bool_exp::{BoolExp, BoolExpFactory};
use crate::bool_exp::BoolExp::Bool;
use crate::{ast_to_boolexp, bool_exp};

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

fn err(tok: &str, rule: &str, got: &lexer::Token) {
    println!("{}: Expected token {}, got {:?}.", rule, tok, got.toktype);
}

type AstRet = (Option<Box<AstNode>>, usize);

//lctx: pat '<'
fn lctx(tokens : &VecDeque<lexer::Token>, index : usize) -> AstRet {
    let mut i = index;
    let result = match pat(tokens, i) {
        (Some(mut p), j) => {
            i = j;
            p.node_type = TokenType::Lctx;
            p
        },

        (None, _) => {
            //err("pat", "lctx", &tokens[i]);
            return (None, index);}
    };

    if  i >= tokens.len() || tokens[i].toktype != TokenType::Lsep {
        //err("Lsep", "lctx", &tokens[i]);
        return (None, index);
    }

    i += 1;
    (Some(result), i)
}

//rctx: '>' pat
fn rctx(tokens : &VecDeque<lexer::Token>, index : usize) -> AstRet {
    let mut i = index;
    if i >= tokens.len() || tokens[i].toktype != TokenType::Rsep {
        //err("Rsep", "rctx", &tokens[i]);
        return (None, index);
    }

    i += 1;
    let result = match pat(tokens, i) {
        (Some(mut p), j) => {i = j; p.node_type = TokenType::Rctx;
        p},
        (None, _) => {
            //err("pat", "rctx", &tokens[i]);
            return (None, index);}
    };

    (Some(result), i)
}

//p_word: char ['(' word ')']
fn p_word(tokens : &VecDeque<lexer::Token>, index : usize) -> AstRet {
    if index >= tokens.len()
        || tokens[index].toktype != TokenType::Char && tokens[index].toktype != TokenType::Letter {
        //err("char|letter", "p_word", &tokens[index]);
        return (None, index);
    }

    let mut i = index;
    let mut result = AstNode{data: String::new(),
        children: vec!(Box::new(AstNode{data: tokens[index].val.clone(), children: Vec::new(),
        node_type: TokenType::Word})),
    node_type: TokenType::ParamWord};
    //create an ast node with first child being the word and second child the parameter

    i += 1;
    //check if there is a parameter
    if i < tokens.len() && tokens[i].toktype == TokenType::Lpara {
        match a_exp(tokens, i + 1) {
            (Some(mut w), j) => {
                i = j;
                let tmp = AstNode{
                    data: String::new(),
                    children: vec![w],
                    node_type: TokenType::Param
                };
                result.children.push(Box::new(tmp));
            },
            _ => {
                //err("word", "p_word", &tokens[i]);
                return (None, index);}
        };

        loop {
            if i >= tokens.len() || tokens[i].val != ','.to_string() {
                break;
            }
            i += 1;

            match a_exp(tokens, i) {
                (Some(mut w), j) => {
                    i = j;
                    let tmp = AstNode{
                        data: String::new(),
                        children: vec![w],
                        node_type: TokenType::Param
                    };
                    result.children.push(Box::new(tmp));
                },
                _ => {
                    //err("word", "p_word", &tokens[i]);
                    return (None, index);}
            };
        }

        //check closing separator
        if i >= tokens.len() || tokens[i].toktype != TokenType::Rpara {
            //err("Rpara", "p_word", &tokens[i]);
            return (None, index);
        }
        i += 1;
    }

    (Some(Box::new(result)), i)
}

//prob: '@' number
fn prob(tokens : &VecDeque<lexer::Token>, index : usize) -> AstRet {
    let mut i = index;
    if i >= tokens.len() || tokens[i].toktype != TokenType::Psep {
        //err("Psep", "prob", &tokens[i]);
        return (None, index);
    }

    i += 1;
    if i >= tokens.len() || tokens[i].toktype != TokenType::Number {
        //err("number", "prob", &tokens[i]);
        return (None, index);
    }

    let ret = AstNode{data: tokens[i].val.clone(), children: Vec::new(), node_type: TokenType::Prob};

    i += 1;

    (Some(Box::new(ret)), i)
}

//word: letter+
fn word(tokens : &VecDeque<lexer::Token>, index : usize) -> AstRet {
    let mut res_str = String::new();

    let mut i = index;
    while i < tokens.len() && tokens[i].toktype == TokenType::Letter {
        res_str.push_str(&tokens[i].val);
        i += 1;
    }

    if res_str.len() > 0 {
        (Some(Box::new(AstNode{data: res_str, children: Vec::new(),
            node_type: TokenType::Word})), i)
    }
    else {
        //err("Letter", "word", &tokens[i]);
        (None, index)
    }
}

//pattern: (p_word|Char)+
pub fn pat(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    let mut i = index;
    let mut result = match p_word(tokens, i) {
        //create the root and add the child
        (Some(tree), j) => {
            i = j;
            AstNode{
            data: String::new(),
            children: vec![tree],
            node_type: TokenType::Pat
        }},
        //check if we have a char(probably redundant)
        (None, _) => {
            if i < tokens.len() {
                match tokens[i].toktype {
                    TokenType::Char => {
                        i += 1;
                        AstNode {
                            data: tokens[i].val.clone(),
                            children: Vec::new(),
                            node_type: TokenType::Char
                        }
                    },
                    //invalid token, just return
                    _ => {
                        //err("p_word", "pat", &tokens[i]);
                        return (None, index);
                    }
                }
            } else {
                return (None, index);
            }
        }
    };

    //loop until the next token is not a p_word
    loop {
        if i >= tokens.len() {
            break;
        }
        match p_word(tokens, i) {
            (Some(tree), j) => {i = j; result.children.push(tree);}
            (None, _) => {
                match tokens[i].toktype {
                    TokenType::Char => {
                        i += 1;
                        let tmp = AstNode{
                            data: tokens[i].val.clone(),
                            children: Vec::new(),
                            node_type: TokenType::Char
                        };
                        result.children.push(Box::new(tmp));
                    },
                    //invalid token, just return
                    _ => {
                        break;
                    }
                };
            }
        };
    }

    (Some(Box::new(result)), i)
}

//param: any+ ws
fn param(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    let mut res_str = String::new();

    let mut i = index;
    while i < tokens.len() && tokens[i].toktype != TokenType::Ws {
        res_str.push_str(&tokens[i].val);
        i += 1;
    }

    if res_str.len() > 0 {
        (Some(Box::new(AstNode{data: res_str, children: Vec::new(),
            node_type: TokenType::Param})), i)
    }
    else {
        //err("!Ws", "param", &tokens[i]);
        (None, index)
    }
}

//preproc: preproc_start word ws param+
fn preproc(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    let mut i = index;
    if i >= tokens.len() || tokens[i].toktype != TokenType::PreprocStart {
        //err("PreprocStart", "preproc", &tokens[i]);
        return (None, index);
    }
    i += 1;

    let mut result = match word(tokens, i) {
        (Some(mut w), j) => {w.node_type = TokenType::Preproc;
            i = j;
            w
        }
        (None, _) => {
            //err("word", "preproc", &tokens[i]);
            return (None, index);}
    };

    loop {
        if i < tokens.len() && tokens[i].toktype == TokenType::Ws {
            i += 1;
        }
        else {
            break;
        }

        match param(tokens, i) {
            (Some(p), j) => {result.children.push(p); i = j}
            (None, _) => {break;}
        }
    }

    (Some(result), i)
}

fn not_tok(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    //println!("Entering not_tok");
    if index < tokens.len() && tokens[index].toktype == TokenType::Char {
        if tokens[index].val == '!'.to_string() {
            //return the token with the correct type
            (Some(Box::new(AstNode{data: tokens[index].val.clone(),
                children: Vec::new(), node_type: TokenType::Not})), index + 1)
        }
        else {
            //err("!", "not_tok", &tokens[index]);
            (None, index)
        }
    }
    else {
        //err("Char", "not_tok", &tokens[index]);
        (None, index)
    }
}

fn or_tok(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    //println!("Entering or_tok");
    if index < tokens.len() && tokens[index].toktype == TokenType::Char {
        if tokens[index].val == '|'.to_string() {
            //return the token with the correct type
            (Some(Box::new(AstNode{data: tokens[index].val.clone(),
                children: Vec::new(), node_type: TokenType::Or})), index + 1)
        }
        else {
            //err("|", "or_tok", &tokens[index]);
            (None, index)
        }
    }
    else {
        //err("Char", "or_tok", &tokens[index]);
        (None, index)
    }
}

//B_exp := '!' B_exp | B_exp '|' B_exp_and | B_exp_and
fn cond(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    //println!("Entering cond");
    let mut i = index;
    let not = match not_tok(tokens, i) {
        (Some(_), j) => {
            i = j;
            true
        },
        (None, _) => false
    };

    let mut ret = AstNode{data: String::new(), children: Vec::new(), node_type: TokenType::Cond};
    match cond_and(tokens, i) {
        (Some(and), j) => {
            i = j;
            ret.children.push(and);
        },
        (None, _) => {
            //err("cond_and", "cond", &tokens[i]);
            return (None, index);}
    };

    loop {
        match or_tok(tokens, i) {
            (Some(_), j) => {i = j;},
            (None, _) => {break;}
        };

        match cond_and(tokens, i) {
            (Some(and), j) => {
                i = j;
                ret.children.push(and);
            },
            (None, _) => {
                //err("cond_and", "cond", &tokens[i]);
                return (None, index);}
        };
    }

    if not {
        let tmp = AstNode{data: '!'.to_string(), children: vec![Box::new(ret)],
            node_type: TokenType::Not};
        (Some(Box::new(tmp)), i)
    } else {
        (Some(Box::new(ret)), i)
    }
}

fn and_tok(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    //println!("Entering and_tok");
    if index < tokens.len() && tokens[index].toktype == TokenType::Char {
        if tokens[index].val == '&'.to_string() {
            //return the token with the correct type
            (Some(Box::new(AstNode{data: tokens[index].val.clone(),
                children: Vec::new(), node_type: TokenType::And})), index + 1)
        }
        else {
            //err("&", "and_tok", &tokens[index]);
            (None, index)
        }
    }
    else {
        //err("Char", "and_tok", &tokens[index]);
        (None, index)
    }
}

//B_exp_and := B_exp_and '&' B_para | B_para
fn cond_and(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    //println!("Entering cond_and");
    let mut i = index;
    let mut ret = AstNode{data: String::new(), children: Vec::new(), node_type: TokenType::CondAnd};

    match cond_para(tokens, i) {
        (Some(para), j) => {
            i = j;
            ret.children.push(para);
        },
        (None, _) => {
            //err("cond_para", "cond_and", &tokens[i]);
            return (None, index);}
    };

    loop {
        match and_tok(tokens, i) {
            (Some(_), j) => {i = j;},
            (None, _) => {break;}
        };

        match cond_para(tokens, i) {
            (Some(para), j) => {
                i = j;
                ret.children.push(para);
            },
            (None, _) => {
                //err("cond_para", "cond_and", &tokens[i]);
                return (None, index);}
        };
    }

    (Some(Box::new(ret)), i)
}

//B_para := '(' B_exp ')' | Bool
fn cond_para(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    //println!("Entering cond_para");
    let mut i = index;
    if i < tokens.len() && tokens[i].toktype == TokenType::Lpara {
        let ret = match cond(tokens, i + 1) {
            (Some(cond), j) => {
                i = j;
                cond
            },
            (None, _) => {
                //err("cond", "cond_para", &tokens[i]);
                //potentially a_para
                let (b, j) = cond_bool(tokens, index);
                return (b, j);
            }
        };

        if i >= tokens.len() || tokens[i].toktype != TokenType::Rpara {
            //err("Rpara", "cond_para", &tokens[i]);
            (None, index)
        } else {
            (Some(ret), i + 1)
        }

    } else {
        match cond_bool(tokens, i) {
            (Some(b), j) => {
                (Some(b), j)
            },
            (None, _) => {
                //err("cond_bool", "cond_para", &tokens[i]);
                (None, index)
            }
        }
    }
}

//Bool := 'true' | 'false' | Comp_exp
fn cond_bool(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    //println!("Entering cond_bool");
    let mut i = index;
    let ret = match word(tokens, i) {
        (Some(mut w), j)
        if w.data.as_str() == "true" || w.data.as_str() == "false" => {
            i = j;
            w.node_type = TokenType::CondBool;
            w
        },
        (Some(_), _) | (None, _) => {
            match comp_exp(tokens, i) {
                (Some(exp), j) => {
                    i = j;
                    exp
                },
                (None, _) => {
                    //err("comp_exp", "cond_bool", &tokens[i]);
                    return (None, index);}
            }
        }
    };

    (Some(ret), i)
}

//Comp_exp := A_exp Comp_op A_exp
fn comp_exp(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    //println!("Entering comp_exp");
    let mut i = index;
    let mut res = AstNode{data: String::new(), children: Vec::new(), node_type: TokenType::CompExp};
    match a_exp(tokens, i) {
        (Some(exp), j) => {i = j; res.children.push(exp);},
        (None, _) => {
            //err("a_exp", "comp_exp", &tokens[i]);
            return (None, index);}
    }

    if i < tokens.len() && tokens[i].toktype == TokenType::CompOp
        || tokens[i].toktype == TokenType::Rsep || tokens[i].toktype == TokenType::Lsep {
        res.data = tokens[i].val.clone();
    }
    else {
        //err("CompOp", "comp_exp", &tokens[i]);
        return (None, index)
    }

    i += 1;

    match a_exp(tokens, i) {
        (Some(exp), j) => {i = j; res.children.push(exp);},
        (None, _) => {
            //err("a_exp", "comp_exp", &tokens[i]);
            return (None, index);
        }
    }

    (Some(Box::new(res)), i)
}

//Comp_op := '=' | '!=' | '>' | '<' | '>=' | '<='
//in lexer

fn add_tok(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    //println!("Entering add_tok");
    if index < tokens.len() && tokens[index].toktype == TokenType::Char {
        if tokens[index].val == '+'.to_string() || tokens[index].val == '-'.to_string() {
            (Some(Box::new(AstNode{data: tokens[index].val.clone(),
            children: Vec::new(), node_type: TokenType::Add})), index + 1)
        }
        else {
            //err("+|-", "add_tok", &tokens[index]);
            (None, index)
        }
    }
    else {
        //err("Char", "add_tok", &tokens[index]);
        (None, index)
    }
}

//A_exp := A_exp '+' A_exp_mul | A_exp_mul
fn a_exp(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    //println!("Entering a_exp");
    let mut i = index;
    let mut ret = AstNode{data: String::new(), children: Vec::new(), node_type: TokenType::Aexp};

    match a_exp_mul(tokens, i) {
        (Some(mul), j) => {i = j; ret.children.push(mul);},
        (None, _) => {
            //err("a_exp_mul", "a_exp", &tokens[i]);
            return (None, index);
        }
    };

    loop {
        match add_tok(tokens, i) {
            (Some(a), j) => {
                i = j;
                ret.children.push(a);
            },
            (None, _) => {break;}
        };

        match a_exp_mul(tokens, i) {
            (Some(mul), j) => {i = j; ret.children.push(mul);},
            (None, _) => {
                //err("a_exp_mul", "a_exp", &tokens[i]);
                return (None, index);
            }
        };
    }

    (Some(Box::new(ret)), i)
}

//test if token i  is a multiplication or division token
//differenciate between the two operators when creating the ast for evaluating the condition
fn mul_tok(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    //println!("Entering mul_tok");
    if index < tokens.len() && tokens[index].toktype == TokenType::Char {
        if tokens[index].val == '/'.to_string() || tokens[index].val == '*'.to_string() {
            //return the token with the correct type
            (Some(Box::new(AstNode{data: tokens[index].val.clone(),
            children: Vec::new(), node_type: TokenType::Mul})), index + 1)
        }
        else {
            //err("/|*", "mul_tok", &tokens[index]);
            (None, index)
        }
    }
    else {
        //err("Char", "mul_tok", &tokens[index]);
        (None, index)
    }
}

//A_exp_mul := A_exp_mul '*' A_para | A_para
fn a_exp_mul(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    //println!("Entering a_exp_mul");
    let mut i = index;
    let mut ret = AstNode{data: String::new(), children: Vec::new(), node_type: TokenType::AexpMul};

    match a_para(tokens, i) {
        (Some(para), j) => {i = j; ret.children.push(para);},
        (None, _) => {
            //err("a_para", "a_exp_mul", &tokens[i]);
            return (None, index);
        }
    };

    loop {
        match mul_tok(tokens, i) {
            (Some(m), j) => {
                i = j;
                ret.children.push(m);
            },
            (None, _) => {break;}
        };

        match a_para(tokens, i) {
            (Some(para), j) => {i = j; ret.children.push(para);},
            (None, _) => {
                //err("a_para", "a_exp_mul", &tokens[i]);
                return (None, index);
            }
        };
    }

    (Some(Box::new(ret)), i)
}

//A_para := '(' A_exp ')' | Num
fn a_para(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    //println!("Entering a_para");
    let mut i = index;
    if i < tokens.len() && tokens[i].toktype != TokenType::Lpara {
        match a_num(tokens, i) {
            (Some(mut n), j) => {n.node_type = TokenType::Apara; (Some(n), j)},
            (None, _) => {
                //err("a_num", "a_para", &tokens[i]);
                (None, index)
            }
        }
    }
    else {
        i += 1;
        let tmp = match a_exp(tokens, i) {
            (Some(mut e), j) => {
                i = j;
                e.node_type = TokenType::Apara;
                Some(e)
            },
            (None, _) => {
                //err("a_exp", "a_para", &tokens[i]);
                None
            }
        };

        if i >= tokens.len() || tokens[i].toktype != TokenType::Rpara {
            //err("Rpara", "a_para", &tokens[i]);
            (None, index)
        }
        else {
            (tmp, i + 1)
        }
    }
}

//Num := 0|...|9 | 0 Num | ... | 9 Num | Var
fn a_num(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    let i = index;
    if i < tokens.len() && tokens[i].toktype == TokenType::Number {
        (Some(Box::new(AstNode{data: tokens[i].val.clone(), children: Vec::new(),
        node_type: TokenType::Anum})), i + 1)
    }
    else {
        match word(tokens, i) {
            (Some(mut w), j) => {w.node_type = TokenType::Anum; (Some(w), j)},
            (None, _) => {
                //err("word|Number", "a_num", &tokens[i]);
                (None, index)
            }
        }
    }
}

//rule: [lctx] p_word [rctx] [cond] [prob] patsep pat
fn rule(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    let mut i = index;
    let mut result = AstNode{data: String::new(), children: Vec::new(), node_type: TokenType::Rule};

    match lctx(tokens, i) {
        (Some(ctx), j) => {
            i = j;
            result.children.push(ctx);
        },
        _ => {}
    };

    //can only replace one symbol, thus use p_word
    match p_word(tokens, i) {
        (Some(mut p), j) => {
            i = j;
            p.node_type = TokenType::Pred;
            result.children.push(p);
        },
        _ => {
            //err("pat", "rule", &tokens[i]);
            return (None, index);
        }
    };

    match rctx(tokens, i) {
        (Some(ctx), j) => {
            i = j;
            result.children.push(ctx);
        },
        _ => {}
    };

    if tokens[i].toktype == TokenType::Condsep {
        match cond(tokens, i + 1) {
            (Some(c), j) => {
                i = j;
                result.children.push(c);
            },
            _ => {}
        };
    }

    match prob(tokens, i) {
        (Some(p), j) => {
            i = j;
            result.children.push(p);
        },
        _ => {}
    };

    if i >= tokens.len() || tokens[i].toktype != TokenType::Patsep {
        //err("PatSep", "rule", &tokens[i]);
        return (None, index);
    }
     i += 1;

    match pat(tokens, i) {
        (Some(mut p), j) => {
            i = j;
            p.node_type = TokenType::Replacement;
            result.children.push(p);
        },
        _ => {
            //err("pat", "rule", &tokens[i]);
            return (None, index);
        }
    };

    (Some(Box::new(result)), i)
}

//preproc | rule
fn parse(s : &str) -> Option<Box<AstNode>> {
    let tokens = lexer::lexer(&s);
    let mut result = Box::new(AstNode{data: String::new(), children: Vec::new(),
        node_type: TokenType::Rule});
    match preproc(&tokens, 0) {
        (Some(p), _) => Some(p),
        _ => {
            match rule(&tokens, 0) {
                (Some(r), _) => Some(r),
                _ => {
                    //err("preproc|rule", "parse", &tokens[0]);
                    None
                }
            }
        }
    }
}

fn create_rule(ast: Box<AstNode>) -> Result<Pattern, &'static str> {
    let mut left : Option<SymbolString> = None;
    let mut right : Option<SymbolString> = None;
    let mut p : f32 = 1.0;
    let mut pattern : Symbol = Symbol{
        sym: 'a',
        var_names: Vec::new(),
        params: Vec::new(),
        rule_set: String::new()
    };
    let mut replacement : SymbolString = SymbolString{ symbols: Vec::new() };
    let mut has_pattern : bool = false;
    let mut has_replacement : bool = false;
    let mut cond : Option<Box<BoolExp>> = None;

    for tok in ast.children.iter() {
        //tok.print(0);
        match tok.node_type {
            TokenType::Rctx => {
                //println!("Rctx");
                right = Some(SymbolString::from_ast(tok, String::new())?);
            },
            TokenType::Lctx => {
                //println!("Lctx");
                left = Some(SymbolString::from_ast(tok, String::new())?);
            },
            TokenType::Prob => {
                //println!("Prob");
                p = tok.data.parse::<f32>().unwrap()
            },
            TokenType::Pred => {
                //println!("Pred");
                pattern = Symbol::from_ast(tok)?;
                has_pattern = true;
            },
            TokenType::Replacement => {
                //println!("Replacement");
                replacement = SymbolString::from_ast(tok, String::new())?;
                has_replacement = true;
            },
            TokenType::Cond => {
                cond = Some(BoolExp::create_from(tok)
                                .unwrap_or(bool_exp::Bool::new(false)));
            },
            _ => {
                //println!("Unknown: {:?}", tok.node_type);
            }
        };
    }

    if !has_pattern || !has_replacement {
       Err("Rule is missing a pattern or replacement.")
    } else {
        //println!("{}", pattern.to_string());
        //println!("{}", replacement.to_string());
        Ok(Pattern::new(pattern, replacement, p, left, right, cond))
    }
}

fn get_param_value(ast: Box<AstNode>, i: usize) -> String {
    if i >= ast.children.len() {
        "ERROR".to_string()
    } else {
        ast.children[i].data.clone()
    }
}

fn get_define_value(ast: Box<AstNode>, i: usize) -> Vec<String> {
    let mut res = Vec::new();
    if i + 1 >= ast.children.len() {
        res.push("ERROR".to_string());
        res.push("ERROR".to_string());
    } else {
        res.push(ast.children[i].data.clone());
        res.push(ast.children[i+1].data.clone());
    }
    res
}

fn read_preproc(ast: Box<AstNode>, ctx: &mut IterCtx) {
    if ast.node_type != TokenType::Preproc {
        return;//invalid node type
    }

    match ast.data.as_str() {
        "ignore" => {ctx.ignored = get_param_value(ast, 0);},
        "axiom" => {ctx.axiom = get_param_value(ast, 0);},
        "niter" => {ctx.n_iter = get_param_value(ast, 0).parse::<usize>()
            .expect("Invalid parameter formating for niter command.");},
        "define" => {
            let def = get_define_value(ast, 0);
            ctx.define.insert(def[0].clone(), def[1].clone().parse().unwrap());
        },
        "include" => {
            let alias_file = get_define_value(ast, 0);
            let alias = alias_file[0].clone();
            let file = alias_file[1].clone();
            ctx.include.insert(alias, file);
        },
        _ => {}
    };
}

// Instantiate Pattern objects from a string.
pub fn parse_rules(data : &str) -> IterCtx {
    let mut result = Vec::new();
    let mut ctx : IterCtx = IterCtx{
                                ignored  : String::new(),
                                axiom: String::new(),
                                n_iter   : 0,
                                define   : HashMap::new(),
                                include  : HashMap::new(),
                                patterns : Vec::new()
    };

    for l in data.lines() {
        //println!("{}", l);
        //println!("{:?}", tokenize(l));
        let rule_ast = parse(l);
        let rule_ast = match rule_ast {
            Some(ast) => {
                //ast.print(0);
                //ast
                match ast.node_type {
                    TokenType::Rule => {
                        match create_rule(ast) {
                            Ok(r) => {result.push(r);},
                            Err(e) => {
                                println!("Error while creating rule: {}", e);
                            }
                    }},
                    TokenType::Preproc => {read_preproc(ast, &mut ctx);},
                    _ => {}
                };
            },
            None => {
                println!("Invalid rule: {}", l);
                //return (Vec::new(), "".to_string());
            }
        };
    }

    //println!("{:?}", ctx);
    result.sort_by(|a, b| a.cmp_pat(b));
    ctx.patterns = result;
    ctx
}
