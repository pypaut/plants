use crate::arith::{self, Arith, ArithFactory};
use crate::bool_exp::{self, BoolExp, BoolExpFactory};
use crate::lexer::TokenType;
use crate::ast::AstNode;

impl BoolExpFactory for BoolExp {
    type Exp = AstNode;

    fn create_from(exp: &Self::Exp) -> Result<Box<dyn BoolExp>, &'static str> {
        match exp.node_type {
            TokenType::Cond | TokenType::CondAnd => {
                if exp.children.len() == 0 {
                    Err("Could not convert Cond: No children.")
                } else {
                    let mut left = BoolExp::create_from(
                        &exp.children[0]
                    )?;
                    let op = match exp.node_type {
                        TokenType::Cond => bool_exp::BinOpType::Or,
                        TokenType::CondAnd => bool_exp::BinOpType::And,
                        _ => {return Err("Invalid operator.")}
                    };
                    for i in 1..exp.children.len() {
                        let right = BoolExp::create_from(
                            &exp.children[i]
                        )?;
                        left = bool_exp::BinOp::new(
                                    &op,
                                    left,
                                    right
                                );
                    }

                    Ok(left)
                }
            },
            TokenType::Not => {
                if exp.children.len() == 0 {
                    Err("Could not convert Not: No children.")
                } else {
                    let exp = BoolExp::create_from(
                        &exp.children[0]
                    )?;
                    Ok(bool_exp::UnOp::new(&bool_exp::UnOpType::Not, exp))
                }
            },
            TokenType::CondBool => {
                if exp.data.len() > 0 {
                    match exp.data.as_str() {
                        "true" => Ok(Box::new(bool_exp::Bool{value: true})),
                        "false" => Ok(Box::new(bool_exp::Bool{value: false})),
                        _ => Err("Could not convert CondBool: invalid value.")
                    }
                } else {
                    Err("Could not convert CondBool: No data.")
                }
            },
            TokenType::CompExp => {
                if exp.children.len() != 2 {
                    Err("Could not convert CompExp: invalid number of children.")
                } else {
                    let left = Arith::create_from(&exp.children[0])?;
                    let right = Arith::create_from(&exp.children[1])?;
                    Ok(bool_exp::CompOp::new(&bool_exp::CompType::from(&exp.data),
                        left, right))
                }
            }
            _ => Err("Invalid expression type.")
        }
    }
}