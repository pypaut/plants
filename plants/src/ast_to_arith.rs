use crate::arith::{Arith, ArithFactory};
use crate::lexer::TokenType;
use crate::ast::AstNode;

impl ArithFactory for AstNode {
    type Exp = AstNode;

    fn create_from(exp: &self::Exp) -> Box<dyn Arith> {
        unimplemented!()
    }
}