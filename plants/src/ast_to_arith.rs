use crate::arith::{self, Arith, ArithFactory};
use crate::lexer::TokenType;
use crate::ast::AstNode;

impl ArithFactory for Arith {
    type Exp = AstNode;

    fn create_from(exp: &self::Exp) -> Result<Box<dyn Arith>, &'static str> {
        match exp.node_type {
            TokenType::Aexp | TokenType::AexpMul => {
                if exp.children.len() == 0 {
                    Err("Could not convert Aexp: No children.")
                } else {
                    let mut left = ArithFactory::create_from(exp.children[0])?;
                    if exp.children.len() % 2 == 0 && exp.children.len() > 1 {
                        Err("Could not convert Aexp: Invalid number of children.")
                    } else {
                        for i in (1..exp.children.len()).step_by(2) {
                            //extract right operand
                            let right = match exp.children[i + 1].node_type {
                                TokenType::Apara | TokenType::AexpMul => {
                                    ArithFactory::create_from(exp.children[i+1])
                                },
                                _ => {
                                    Err("Could not convert Aexp: Expected Apara|AexpMul expression.");
                                }
                            }?;
                            //extract operator
                            left = match exp.children[i].node_type {
                                TokenType::Add | TokenType::Mul => {
                                    arith::ArithOp::new(
                                        arith::OpType::from(exp.children[i].data),
                                        left,
                                        right
                                    )
                                },
                                _ => {
                                    Err("Could not convert Aexp: Expected Add or Mul operator.");
                                }
                            }?;
                        }
                    }

                    Ok(left)
                }
            },
            TokenType::Apara => {
                if exp.data.len() > 0 {
                    //create a var Arith
                    match exp.data.parse::<f32>() {
                        Ok(k) => Ok(arith::Var::new_value(k)),
                        Err(_) => Ok(arith::Var::new_name(exp.data))
                    }
                } else {
                    //recursively call create_from to have the full expression
                    if exp.children.len() == 0 {
                        Err("Could not convert Apara expression: No children to convert.")
                    } else {
                        ArithFactory::create_from(exp.children[0])
                    }
                }
            }
        }
    }
}