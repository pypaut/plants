use std::collections::VecDeque;

#[derive(PartialEq)]
pub enum TokenType {
    Rule,
    Lctx,
    Rctx,
    Patsep,
    Lsep,
    Rsep,
    Lpara,
    Rpara,
    Condsep,
    Cond,
    Lpsep,
    Rpsep,
    Prob,
    Pat,
    ParamWord,
    Word,
    Preproc,
    PreprocStart,
    Param,
    Ws,
    Char,
    Letter,
    Number,
    Anum,
    Apara,
    AexpMul,
    Aexp,
    CompExp,
    CondBool,
    CondPara,
    CondAnd,
    Mul,
    Add,
    CompOp,
    And,
    Or,
    Not,
    Err
}

pub struct Token {
    pub toktype : TokenType,
    pub val : String,
}

pub fn lexer(rules : &str) -> VecDeque<Token> {
    let mut tokens : VecDeque<Token> = VecDeque::new();

    for line in rules.lines() {
        let mut i = 0;

        while i < line.len() {
            if line.chars().nth(i)  == Some('<') {
                if line.chars().nth(i + 1) == Some('=') {
                    tokens.push_back(Token{toktype: TokenType::CompOp,
                        val: String::from("<=")});
                    i += 1;
                } else {
                    tokens.push_back(Token { toktype: TokenType::Lsep,
                        val: String::from("<") });
                }
            } else if line.chars().nth(i) == Some('>') {
                if line.chars().nth(i + 1) == Some('=') {
                    tokens.push_back(Token{toktype: TokenType::CompOp,
                        val: String::from(">=")});
                } else {
                    tokens.push_back(Token { toktype: TokenType::Rsep,
                        val: String::from(">") });
                }
            } else if line.chars().nth(i) == Some('='){
                tokens.push_back(Token{toktype: TokenType::CompOp,
                    val: String::from("=")});
            } else if line.chars().nth(i) == Some('!') && line.chars().nth(i + 1) == Some('=') {
                tokens.push_back(Token{toktype: TokenType::CompOp, val: String::from("!=")});
                i += 1;
            } else if line.chars().nth(i) == Some('(') {
                tokens.push_back(Token{toktype : TokenType::Lpara, val : String::from("(")});
            } else if line.chars().nth(i) == Some(')') {
                tokens.push_back(Token{toktype : TokenType::Rpara, val : String::from(")")});
            } else if line.chars().nth(i) == Some('[') {
                tokens.push_back(Token{toktype : TokenType::Lpsep, val : String::from("[")});
            } else if line.chars().nth(i) == Some(']') {
                tokens.push_back(Token{toktype : TokenType::Rpsep, val : String::from("]")});
            } else if line.chars().nth(i) == Some(':') {
                tokens.push_back(Token{toktype : TokenType::Condsep, val : String::from(":")});
            } else if line.chars().nth(i) == Some('#') {
                tokens.push_back(Token{toktype : TokenType::PreprocStart, val : String::from("#")});
            } else if line.chars().nth(i) == Some('-') && line.chars().nth(i+1) == Some('>') {
                tokens.push_back(Token{toktype : TokenType::Patsep, val : String::from("->")});
                i += 1;
            } else if line.chars().nth(i) == Some(' ') || line.chars().nth(i) == Some('\n') {
                tokens.push_back(Token{toktype : TokenType::Ws,
                    val : line.chars().nth(i).unwrap().to_string()});
            } else if line.chars().nth(i).unwrap().is_digit(10) {
                let mut dot = false;
                //while we have something to read and we are reading digits or a dot
                let mut c = line.chars().nth(i).unwrap();
                let mut s = String::new();
                while i < line.len() && (c.is_digit(10)
                    || (c == '.' && !dot)) {
                    s.push(c);
                    if c == '.' {
                        dot = true;
                    }
                    i += 1;
                    if i < line.len() {
                        c = line.chars().nth(i).unwrap();
                    }
                }

                tokens.push_back(Token{toktype: TokenType::Number,
                    val : s});
            } else {
                let val = line.chars().nth(i).unwrap();
                if val.is_alphabetic() {
                    tokens.push_back(Token{toktype : TokenType::Letter,
                        val : val.to_string()});
                }
                else {
                    tokens.push_back(Token{toktype : TokenType::Char,
                        val : val.to_string()});
                }
            }

            i += 1;
        }
    }

    tokens
}
