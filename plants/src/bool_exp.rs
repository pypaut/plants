use crate::arith;

/*pub trait BoolExp {
    fn eval(&self) -> bool;
    fn vars(&self) -> Vec<&str>;
    fn set(&mut self, var: &str, val: f32) -> Result<(), ()>;
}*/

#[derive(Clone, Debug)]
pub enum BoolExp {
    BinOp(BinOp),
    Unop(UnOp),
    CompOp(CompOp),
    Bool(Bool)
}

impl BoolExp {
    pub fn eval(&self) -> bool {
        match self {
            BoolExp::BinOp(v) => v.eval(),
            BoolExp::Unop(v) => v.eval(),
            BoolExp::CompOp(v) => v.eval(),
            BoolExp::Bool(v) => v.eval(),
        }
    }

    pub fn vars(&self) -> Vec<&str> {
        match self {
            BoolExp::BinOp(v) => v.vars(),
            BoolExp::Unop(v) => v.vars(),
            BoolExp::CompOp(v) => v.vars(),
            BoolExp::Bool(v) => v.vars(),
        }
    }

    pub fn set(&mut self, var: &str, val: f32) -> Result<(), ()> {
        match self {
            BoolExp::BinOp(v) => v.set(var, val),
            BoolExp::Unop(v) => v.set(var, val),
            BoolExp::CompOp(v) => v.set(var, val),
            BoolExp::Bool(v) => v.set(var, val),
        }
    }
}

pub enum BinOpType {
    And,
    Or
}

#[derive(Clone, Debug)]
pub struct BinOp {
    left: Box<BoolExp>,
    right: Box<BoolExp>,
    operator: fn(bool, bool) -> bool
}

impl BinOp {
    fn eval(&self) -> bool {
        (self.operator)(self.left.eval(), self.right.eval())
    }

    fn vars(&self) -> Vec<&str> {
        let mut v1 = self.left.vars();
        let mut v2 = self.right.vars();
        v1.append(&mut v2);
        v1.sort();
        v1.dedup();
        v1
    }

    fn set(&mut self, var: &str, val: f32) -> Result<(), ()> {
        let res1 = self.left.set(var, val);
        let res2 = self.right.set(var, val);

        match (res1, res2) {
            (Err(_), Err(_)) => Err(()),
            _ => Ok(())
        }
    }

    fn and(x: bool, y: bool) -> bool {
        x && y
    }

    fn or(x: bool, y: bool) -> bool {
        x || y
    }

    pub fn new(op: &BinOpType, left: Box<BoolExp>, right: Box<BoolExp>) -> Box<BoolExp> {
        let fun = match op {
            BinOpType::And => BinOp::and,
            BinOpType::Or => BinOp::or
        };

        Box::new(BoolExp::BinOp(BinOp{left, right, operator: fun}))
    }
}

pub enum UnOpType {
    Not
}

#[derive(Clone, Debug)]
pub struct UnOp {
    exp: Box<BoolExp>,
    operator: fn(bool) -> bool
}

impl UnOp {
    fn eval(&self) -> bool {
        (self.operator)(self.exp.eval())
    }

    fn vars(&self) -> Vec<&str> {
        self.exp.vars()
    }

    fn set(&mut self, var: &str, val: f32) -> Result<(), ()> {
        self.exp.set(var, val)
    }

    fn not(x: bool) -> bool {
        !x
    }

    pub fn new(op: &UnOpType, exp: Box<BoolExp>) -> Box<BoolExp> {
        let fun = match op {
            UnOpType::Not => UnOp::not
        };

        Box::new(BoolExp::Unop(UnOp{exp, operator: fun}))
    }
}

pub enum CompType {
    Less,
    Greater,
    LessEq,
    GreaterEq,
    Equal,
    NotEqual
}

impl CompType {
    pub fn from(s: &str) -> CompType {
        match s {
            "<" => CompType::Less,
            ">" => CompType::Greater,
            "<=" => CompType::LessEq,
            ">=" => CompType::GreaterEq,
            "=" => CompType::Equal,
            "!=" => CompType::NotEqual,
            _ => CompType::Less//error, use a default value
        }
    }
}

#[derive(Clone, Debug)]
pub struct CompOp {
    left: Box<arith::Arith>,
    right: Box<arith::Arith>,
    operator: fn(f32, f32) -> bool
}

impl CompOp {
    fn eval(&self) -> bool {
        (self.operator)(self.left.eval(), self.right.eval())
    }

    fn vars(&self) -> Vec<&str> {
        let mut v1 = self.left.vars();
        let mut v2 = self.right.vars();
        v1.append(&mut v2);
        v1.sort();
        v1.dedup();
        v1
    }

    fn set(&mut self, var: &str, val: f32) -> Result<(), ()> {
        let res1 = self.left.set(var, val);
        let res2 = self.right.set(var, val);

        match (res1, res2) {
            (Err(_), Err(_)) => Err(()),
            _ => Ok(())
        }
    }

    fn less(x: f32, y: f32) -> bool {
        x < y
    }

    fn greater(x: f32, y: f32) -> bool {
        x > y
    }

    fn less_eq(x: f32, y: f32) -> bool {
        x <= y
    }

    fn greater_eq(x: f32, y: f32) -> bool {
        x >= y
    }

    fn equal(x: f32, y: f32) -> bool {
        x == y
    }

    fn not_equal(x: f32, y: f32) -> bool {
        x != y
    }

    pub fn new(op: &CompType, left: Box<arith::Arith>, right: Box<arith::Arith>) -> Box<BoolExp> {
        let fun = match op {
            CompType::Less => CompOp::less,
            CompType::Greater => CompOp::greater,
            CompType::LessEq => CompOp::less_eq,
            CompType::GreaterEq => CompOp::greater_eq,
            CompType::Equal => CompOp::equal,
            CompType::NotEqual => CompOp::not_equal
        };

        Box::new(BoolExp::CompOp(CompOp{left, right, operator: fun}))
    }
}

#[derive(Clone, Debug)]
pub struct Bool {
    pub value: bool
}

impl Bool {
    pub fn new(val: bool) -> Box<BoolExp> {
        Box::new(BoolExp::Bool(Bool{value: val}))
    }

    fn eval(&self) -> bool {
        self.value
    }

    fn vars(&self) -> Vec<&str> {
        Vec::new()
    }

    fn set(&mut self, var: &str, val: f32) -> Result<(), ()> {
        Err(())//no variable will be found ever, return an error
    }
}

pub trait BoolExpFactory {
    type Exp;

    fn create_from(exp: &Self::Exp) -> Result<Box<BoolExp>, &'static str>;
}