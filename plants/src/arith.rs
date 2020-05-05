use std::borrow::Borrow;

#[derive(Clone, Debug)]
pub enum Arith {
    Var(Var),
    Op(ArithOp)
}


impl Arith {
    pub fn eval(&self) -> f32 {
        match self {
            Arith::Var(v) => v.eval(),
            Arith::Op(v) => v.eval()
        }
    }

    pub fn vars(&self) -> Vec<&str> {
        match self {
            Arith::Var(v) => v.vars(),
            Arith::Op(v) => v.vars()
        }
    }

    pub fn set(&mut self, var: &str, val: f32) -> Result<(), ()> {
        match self {
            Arith::Var(v) => v.set(var, val),
            Arith::Op(v) => v.set(var, val)
        }
    }
}


#[derive(Clone, Debug)]
pub struct Var {
    name: Option<String>,
    value: f32
}

impl Var {
    fn eval(&self) -> f32 {
        self.value
    }

    fn vars(&self) -> Vec<&str> {
        match &self.name {
            Some(n) => vec![n.borrow()],
            None => Vec::new()
        }
    }

    fn set(&mut self, var: &str, val: f32) -> Result<(), ()> {
        match &self.name {
            Some(v) if v == var => {
                self.value = val;
                Ok(())
            },
            _ => { Err(()) }
        }
    }

    fn get(&self) -> f32 {
        self.value
    }

    pub fn new_name(name: String) -> Box<Arith> {
        Box::new(Arith::Var(Var{name: Some(name), value: 0.0}))
    }

    pub fn new_value(value: f32) -> Box<Arith> {
        Box::new(Arith::Var(Var{name: None, value}))
    }
}

#[derive(Clone, Debug)]
pub struct ArithOp {
    left: Box<Arith>,
    right: Box<Arith>,
    operator: fn(f32, f32) -> f32
}

pub enum OpType {
    Add,
    Sub,
    Mul,
    Div
}

impl OpType {
    pub fn from(s: &str) -> OpType {
        match s {
            "+" => OpType::Add,
            "-" => OpType::Sub,
            "*" => OpType::Mul,
            "/" => OpType::Div,
            _ => OpType::Add//error case, default to add
        }
    }
}

impl ArithOp {
    fn eval(&self) -> f32 {
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
            //both sides failed, var does not exist
            (Err(_), Err(_)) => Err(()),
            //at least one side succeeded, var exists
            _ => Ok(())
        }
    }

    fn add(x: f32, y: f32) -> f32 {
        x + y
    }

    fn sub(x: f32, y: f32) -> f32 {
        x - y
    }

    fn mul(x: f32, y: f32) -> f32 {
        x * y
    }

    fn div(x: f32, y: f32) -> f32 {
        x / y
    }

    pub fn new(op: &OpType, left: Box<Arith>, right: Box<Arith>) -> Box<Arith> {
        let fun = match op {
            OpType::Add => ArithOp::add,
            OpType::Sub => ArithOp::sub,
            OpType::Mul => ArithOp::mul,
            OpType::Div => ArithOp::div
        };
        Box::new(Arith::Op(ArithOp{left, right, operator: fun}))
    }
}

pub trait ArithFactory {
    type Exp;

    fn create_from(exp: &Self::Exp) -> Result<Box<Arith>, &'static str>;
}