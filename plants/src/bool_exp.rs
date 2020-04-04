use crate::arith;

pub trait BoolExp {
    fn eval(&self) -> bool;
    fn vars(&self) -> Vec<&str>;
    fn set(&mut self, var: &str, val: f32) -> Result<(), ()>;
}

pub enum BinOpType {
    And,
    Or
}

pub struct BinOp {
    left: Box<dyn BoolExp>,
    right: Box<dyn BoolExp>,
    operator: fn(bool, bool) -> bool
}

impl BoolExp for BinOp {
    fn eval(&self) -> bool {
        self.operator(self.left.eval(), self.right.eval())
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
}

impl BinOp {
    pub fn new(op: BinOpType, left: Box<dyn BoolExp>, right: Box<dyn BoolExp>) -> Box<BinOp> {
        let fun = match op {
            BinOpType::And => |x, y| {x && y},
            BinOpType::Or => |x, y| {x || y}
        };

        Box::new(BinOp{left, right, operator: fun})
    }
}

pub enum UnOpType {
    Not
}

pub struct UnOp {
    exp: Box<dyn BoolExp>,
    operator: fn(bool) -> bool
}

impl BoolExp for UnOp {
    fn eval(&self) -> bool {
        self.operator(self.exp.eval())
    }

    fn vars(&self) -> Vec<&str> {
        self.exp.vars()
    }

    fn set(&mut self, var: &str, val: f32) -> Result<(), ()> {
        self.exp.set(var, val)
    }
}

impl UnOp {
    pub fn new(op: UnOpType, exp: Box<dyn BoolExp>) -> Box<UnOp> {
        let fun = match op {
            UnOpType::Not => |x| {!x}
        };

        Box::new(UnOp{exp, operator: fun})
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
    fn from(s: &str) -> CompType {
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

pub struct CompOp {
    left: Box<dyn arith::Arith>,
    right: Box<dyn arith::Arith>,
    operator: fn(f32, f32) -> bool
}

impl BoolExp for CompOp {
    fn eval(&self) -> bool {
        self.operator(self.left.eval(), self.right.eval())
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
}

impl CompOp {
    pub fn new(op: CompType, left: Box<dyn arith::Arith>, right: Box<dyn arith::Arith>) -> Box<CompOp> {
        let fun = match op {
            CompType::Less => |x, y| {x < y},
            CompType::Greater => |x, y| {x > y},
            CompType::LessEq => |x, y| {x <= y},
            CompType::GreaterEq => |x, y| {x >= y},
            CompType::Equal => |x, y| {x == y},
            CompType::NotEqual => |x, y| {x != y}
        };

        Box::new(CompOp{left, right, operator: fun})
    }
}

pub struct Bool {
    value: bool
}

impl BoolExp for Bool {
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

    fn create_from(exp: &self::Exp) -> Result<Box<dyn BoolExp>, &'static str>;
}