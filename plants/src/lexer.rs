use std::collections::VecDeque;


/*pub enum TokenType {
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
}*/

pub enum TokenType {
    General(GeneralToken),
    Preproc(PreprocToken),
    Rule(RuleToken)
}

pub enum GeneralToken {
    Ws,//whitespace
    Err,//error value
    Char,//any character that is not a separator or keyword
    Letter//any letter (upper and lower case)
}

pub enum PreprocToken {
    PreprocStart,
    Param,
    Word
}

pub enum RuleToken {
    Rule,
    Pat,
    Pword,
    Lctx,
    Rctx,
    Cond,
    Prob,
    PatSep,
    LpSep,
    RpSep,
    Lsep,
    Rsep,
}

pub struct Token {
    pub toktype : TokenType,
    pub val : String,
}

pub struct Lexer {
    data : String,//remaining data to read
    last : Token,//last extracted token
    consume : bool//indicate if the next peek has to read a new token
}

impl Lexer {
    pub fn New(data : String) -> Lexer {
        Lexer{data, last: Token{toktype: TokenType::General(GeneralToken::Err), val: String::new()},
            consume: true}
    }

    //peek next token, can be called many times, does not consume token
    pub fn peek(&mut self) -> &Token {
        if self.consume {
            //consume a new token and return it
        }
        else {
            //return the last token
        }

        self.consume = false;
    }

    //consume
    pub fn consume(&mut self) {
        self.consume = true;
    }
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
