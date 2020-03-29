use crate::pattern::Pattern;
use crate::lexer::{self, TokenType};
use std::collections::VecDeque;

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

        (None, _) => {return (None, index);}
    };

    if tokens[i].toktype != TokenType::Lsep {
        return (None, index);
    }

    i += 1;
    (Some(result), i)
}

//rctx: '>' pat
fn rctx(tokens : &VecDeque<lexer::Token>, index : usize) -> AstRet {
    let mut i = index;
    if tokens[i].toktype != TokenType::Rsep {
        return (None, index);
    }

    i += 1;
    let result = match pat(tokens, i) {
        (Some(mut p), j) => {i = j; p.node_type = TokenType::Rctx;
        p},
        (None, _) => {return (None, index);}
    };

    (Some(result), i)
}

//p_word: char ['(' word ')']
fn p_word(tokens : &VecDeque<lexer::Token>, index : usize) -> AstRet {
    if tokens[index].toktype != TokenType::Char && tokens[index].toktype != TokenType::Letter {
        return (None, index)
    }

    let mut i = index;
    let mut result = AstNode{data: String::new(),
        children: vec!(Box::new(AstNode{data: tokens[index].val.clone(), children: Vec::new(),
        node_type: TokenType::Word})),
    node_type: TokenType::ParamWord};
    //create an ast node with first child being the word and second child the parameter

    i += 1;
    //check if there is a parameter
    if tokens[i].toktype == TokenType::Lpara {
        let (w, i) = word(tokens, i);
        match w {
            Some(mut w) => {
                w.node_type = TokenType::Param;
                result.children.push(w);
            },
            None => {}
        };

        //check closing separator
        if tokens[i].toktype != TokenType::Rpara {
            return (None, index);
        }
    }

    (Some(Box::new(result)), i)
}

//prob: '[' number ']'
fn prob(tokens : &VecDeque<lexer::Token>, index : usize) -> AstRet {
    let mut i = index;
    if tokens[i].toktype != TokenType::Lpsep {
        return (None, index);
    }

    i += 1;
    if tokens[i].toktype != TokenType::Number {
        return (None, index);
    }

    let ret = AstNode{data: tokens[i].val.clone(), children: Vec::new(), node_type: TokenType::Prob};

    i += 1;

    if tokens[i].toktype != TokenType::Rpsep {
        return (None, index);
    }

    (Some(Box::new(ret)), i)
}

//word: letter+
fn word(tokens : &VecDeque<lexer::Token>, index : usize) -> AstRet {
    let mut res_str = String::new();

    let mut i = index;
    while tokens[i].toktype == TokenType::Letter {
        res_str.push_str(&tokens[i].val);
        i += 1;
    }

    if res_str.len() > 0 {
        (Some(Box::new(AstNode{data: res_str, children: Vec::new(),
            node_type: TokenType::Word})), i)
    }
    else {
        (None, index)
    }
}

//pattern: p_word+
fn pat(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    let mut i = index;
    let mut result = match p_word(tokens, i) {
        //create the root and add the child
        (Some(tree), j) => {i = j; AstNode{
            data: String::new(),
            children: vec![tree],
            node_type: TokenType::Pat
        }},
        //just return
        (None, _) => {return (None, index);}
    };

    //loop until the next token is not a p_word
    loop {
        match p_word(tokens, i) {
            (Some(tree), j) => {i = j; result.children.push(tree);}
            (None, _) => {break;}
        };
    }

    (Some(Box::new(result)), index)
}

//param: any+ ws
fn param(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    let mut res_str = String::new();

    let mut i = index;
    while tokens[i].toktype != TokenType::Ws {
        res_str.push_str(&tokens[i].val);
        i += 1;
    }

    if res_str.len() > 0 {
        (Some(Box::new(AstNode{data: res_str, children: Vec::new(),
            node_type: TokenType::Param})), i)
    }
    else {
        (None, index)
    }
}

//preproc: preproc_start word ws param+
fn preproc(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    let mut i = index;
    if tokens[index].toktype != TokenType::PreprocStart {
        return (None, index);
    }
    i += 1;

    let mut result = match word(tokens, i) {
        (Some(mut w), j) => {w.node_type = TokenType::Preproc;
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
        (Some(result), i)
    }
    else {
        (None, index)
    }
}

fn not_tok(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    if tokens[index].toktype == TokenType::Char {
        if tokens[index].val == '!'.to_string() {
            //return the token with the correct type
            (Some(Box::new(AstNode{data: tokens[index].val.clone(),
                children: Vec::new(), node_type: TokenType::Not})), index + 1)
        }
        else {
            (None, index)
        }
    }
    else {
        (None, index)
    }
}

fn or_tok(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    if tokens[index].toktype == TokenType::Char {
        if tokens[index].val == '|'.to_string() {
            //return the token with the correct type
            (Some(Box::new(AstNode{data: tokens[index].val.clone(),
                children: Vec::new(), node_type: TokenType::Or})), index + 1)
        }
        else {
            (None, index)
        }
    }
    else {
        (None, index)
    }
}

//B_exp := '!' B_exp | B_exp '|' B_exp_and | B_exp_and
fn cond(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
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
        (None, _) => {return (None, index);}
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
            (None, _) => {return (None, index);}
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
    if tokens[index].toktype == TokenType::Char {
        if tokens[index].val == '&'.to_string() {
            //return the token with the correct type
            (Some(Box::new(AstNode{data: tokens[index].val.clone(),
                children: Vec::new(), node_type: TokenType::And})), index + 1)
        }
        else {
            (None, index)
        }
    }
    else {
        (None, index)
    }
}

//B_exp_and := B_exp_and '&' B_para | B_para
fn cond_and(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    let mut i = index;
    let mut ret = AstNode{data: String::new(), children: Vec::new(), node_type: TokenType::CondAnd};

    match cond_para(tokens, i) {
        (Some(para), j) => {
            i = j;
            ret.children.push(para);
        },
        (None, _) => {return (None, index);}
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
            (None, _) => {return (None, index);}
        };
    }

    (Some(Box::new(ret)), i)
}

//B_para := '(' B_exp ')' | Bool
fn cond_para(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    let mut i = index;
    if tokens[i].toktype == TokenType::Lpara {
        let ret = match cond(tokens, i) {
            (Some(cond), j) => {
                i = j;
                cond
            },
            (None, _) => {
                return (None, index);
            }
        };

        if tokens[i].toktype != TokenType::Rpara {
            (None, index)
        } else {
            (Some(ret), i + 1)
        }

    } else {
        match cond_bool(tokens, i) {
            (Some(b), j) => {
                (Some(b), j)
            },
            (None, _) => (None, index)
        }
    }
}

//Bool := 'true' | 'false' | Comp_exp
fn cond_bool(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
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
                (None, _) => {return (None, index);}
            }
        }
    };

    (Some(ret), i)
}

//Comp_exp := A_exp Comp_op A_exp
fn comp_exp(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    let mut i = index;
    let mut res = AstNode{data: String::new(), children: Vec::new(), node_type: TokenType::CompExp};
    match a_exp(tokens, i) {
        (Some(exp), j) => {i = j; res.children.push(exp);},
        (None, _) => {return (None, index);}
    }

    if tokens[i].toktype == TokenType::CompOp {
        res.data = tokens[i].val.clone();
    }
    else {
        return (None, index)
    }

    i += 1;

    match a_exp(tokens, i) {
        (Some(exp), j) => {i = j; res.children.push(exp);},
        (None, _) => {return (None, index);}
    }

    (Some(Box::new(res)), i)
}

//Comp_op := '=' | '!=' | '>' | '<' | '>=' | '<='
//in lexer

fn add_tok(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    if tokens[index].toktype == TokenType::Char {
        if tokens[index].val == '+'.to_string() || tokens[index].val == '-'.to_string() {
            (Some(Box::new(AstNode{data: tokens[index].val.clone(),
            children: Vec::new(), node_type: TokenType::Add})), index + 1)
        }
        else {
            (None, index)
        }
    }
    else {
        (None, index)
    }
}

//A_exp := A_exp '+' A_exp_mul | A_exp_mul
fn a_exp(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    let mut i = index;
    let mut ret = AstNode{data: String::new(), children: Vec::new(), node_type: TokenType::Aexp};

    match a_exp_mul(tokens, i) {
        (Some(mul), j) => {i = j; ret.children.push(mul);},
        (None, _) => {return (None, index)}
    };

    loop {
        match add_tok(tokens, i) {
            (Some(_), j) => {i = j;},
            (None, _) => {break;}
        };

        match a_exp_mul(tokens, i) {
            (Some(mul), j) => {i = j; ret.children.push(mul);},
            (None, _) => {return (None, index);}
        };
    }

    (Some(Box::new(ret)), i)
}

//test if token i  is a multiplication or division token
//differenciate between the two operators when creating the ast for evaluating the condition
fn mul_tok(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    if tokens[index].toktype == TokenType::Char {
        if tokens[index].val == '/'.to_string() || tokens[index].val == '*'.to_string() {
            //return the token with the correct type
            (Some(Box::new(AstNode{data: tokens[index].val.clone(),
            children: Vec::new(), node_type: TokenType::Mul})), index + 1)
        }
        else {
            (None, index)
        }
    }
    else {
        (None, index)
    }
}

//A_exp_mul := A_exp_mul '*' A_para | A_para
fn a_exp_mul(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    let mut i = index;
    let mut ret = AstNode{data: String::new(), children: Vec::new(), node_type: TokenType::AexpMul};

    match a_para(tokens, i) {
        (Some(para), j) => {i = j; ret.children.push(para);},
        (None, _) => {return (None, index);}
    };

    loop {
        match mul_tok(tokens, i) {
            (Some(_), j) => {i = j;},
            (None, _) => {break;}
        };

        match a_para(tokens, i) {
            (Some(para), j) => {i = j; ret.children.push(para);},
            (None, _) => {return (None, index);}
        };
    }

    (Some(Box::new(ret)), i)
}

//A_para := '(' A_exp ')' | Num
fn a_para(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    let mut i = index;
    if tokens[i].toktype != TokenType::Lpara {
        match a_num(tokens, i) {
            (Some(mut n), j) => {n.node_type = TokenType::Apara; (Some(n), j)},
            (None, _) => (None, index)
        }
    }
    else {
        i += 1;
        let tmp = match a_exp(tokens, i) {
            (Some(mut e), j) => {i = j; e.node_type = TokenType::Apara;
                (Some(e), j)},
            (None, _) => (None, index)
        };

        if tokens[i].toktype != TokenType::Rpara {
            (None, index)
        }
        else {
            tmp
        }
    }
}

//Num := 0|...|9 | 0 Num | ... | 9 Num | Var
fn a_num(tokens: &VecDeque<lexer::Token>, index: usize) -> AstRet {
    let i = index;
    if tokens[i].toktype == TokenType::Number {
        (Some(Box::new(AstNode{data: tokens[i].val.clone(), children: Vec::new(),
        node_type: TokenType::Anum})), i + 1)
    }
    else {
        match word(tokens, i) {
            (Some(mut w), j) => {w.node_type = TokenType::Anum; (Some(w), j)},
            (None, _) => (None, index)
        }
    }
}


/*fn parse(s : String) -> Option<Box<AstNode>> {
    let tokens = lexer::lexer(&s);
    let mut result = Box::new(AstNode{data: String::new(), children: Vec::new(),
        node_type: TokenType::Rule});
}*/

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
    let result = Vec::new();
    let ignored : String = String::new();

    for l in data.lines() {
        //println!("{}", l);
        //println!("{:?}", tokenize(l));
        let tokens = lexer::lexer(l);
        let rule_ast = cond(&tokens, 0);
        let _rule_ast = match rule_ast {
            (Some(ast), _) => ast,
            (None, _) => {
                println!("Invalid rule: {}", l);
                return (Vec::new(), "".to_string());
            }
        };
    }

    //result.sort_by(|a, b| a.cmp_pat(b));
    (result, ignored)
}
