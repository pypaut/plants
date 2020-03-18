use std::collections::VecDeque;


pub enum TokenType {
    Rule,
    Lctx,
    Rctx,
    Replace,
    Patsep,
    Lsep,
    Rsep,
    LsepParam,
    RsepParam,
    Condsep,
    Cond,
    Lpsep,
    Rpsep,
    Prob,
    LsepProb,
    RsepProb,
    Left,
    Right,
    Pat,
    ParamWord,
    Word,
    Preproc,
    PreprocStart,
    Param,
}

pub struct Token {
    toktype : TokenType,
    val : String,
}





pub fn lexer(rules : &str) -> VecDeque<Token> {
    let mut tokens : VecDeque<Token> = VecDeque::new();

    for line in rules.lines() {
        let mut word = String::from("");
        let mut added_to_word = false;
        let mut i = 0;

        while i < line.len() {
            added_to_word = false;
            if line.chars().nth(i)  == Some('<') {
                tokens.push_back(Token{toktype : TokenType::Lsep, val : String::from("")});
            } else if line.chars().nth(i) == Some('>') {
                tokens.push_back(Token{toktype : TokenType::Rsep, val : String::from("")});
            } else if line.chars().nth(i) == Some('(') {
                tokens.push_back(Token{toktype : TokenType::LsepParam, val : String::from("")});
            } else if line.chars().nth(i) == Some(')') {
                tokens.push_back(Token{toktype : TokenType::RsepParam, val : String::from("")});
            } else if line.chars().nth(i) == Some('[') {
                tokens.push_back(Token{toktype : TokenType::LsepProb, val : String::from("")});
            } else if line.chars().nth(i) == Some(']') {
                tokens.push_back(Token{toktype : TokenType::RsepProb, val : String::from("")});
            } else if line.chars().nth(i) == Some(':') {
                tokens.push_back(Token{toktype : TokenType::Condsep, val : String::from("")});
            } else if line.chars().nth(i) == Some('#') {
                tokens.push_back(Token{toktype : TokenType::PreprocStart, val : String::from("")});
            } else if line.chars().nth(i) == Some('-') && line.chars().nth(i+1) == Some('>') {
                tokens.push_back(Token{toktype : TokenType::Patsep, val : String::from("")});
                i += 1;
            } else {
                if line.chars().nth(i) != Some(' ') {
                    word.push(line.chars().nth(i).unwrap());
                    added_to_word = true;
                }
            }

            if !added_to_word {  // Last word is done
                tokens.push_back(Token{toktype : TokenType::Word, val : word});
                word = String::from("");
            }

            i += 1;
        }
    }

    tokens
}
