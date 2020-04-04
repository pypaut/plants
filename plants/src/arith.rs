use std::borrow::Borrow;

pub trait Arith {
    fn eval(&self) -> f32;
    fn vars(&self) -> Vec<&str>;
    fn set(&mut self, var: &str, val: f32) -> Result<(), ()>;
}

pub struct Var {
    name: Option<String>,
    value: f32
}

impl Arith for Var {
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
        if self.name.borrow() == var {
            self.value = val;
            Ok(())
        } else {
            Err(())
        }
    }
}

impl Var {
    pub fn new_name(name: String) -> Box<Var> {
        Box::new(Var{name: Some(name), value: 0.0})
    }

    pub fn new_value(value: f32) -> Box<Var> {
        Box::new(Var{name: None, value})
    }
}

pub struct ArithOp {
    left: Box<dyn Arith>,
    right: Box<dyn Arith>,
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

impl Arith for ArithOp {
    fn eval(&self) -> f32 {
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
            //both sides failed, var does not exist
            (Err(_), Err(_)) => Err(()),
            //at least one side succeeded, var exists
            _ => Ok(())
        }
    }
}

impl ArithOp {
    pub fn new(op: OpType, left: Box<dyn Arith>, right: Box<dyn Arith>) -> Box<ArithOp> {
        let fun = match op {
            OpType::Add => |x, y| {x + y},
            OpType::Sub => |x, y| {x - y},
            OpType::Mul => |x, y| {x * y},
            OpType::Div => |x, y| {x / y}
        };
        Box::new(ArithOp{left, right, operator: fun})
    }
}

pub trait ArithFactory {
    type Exp;

    fn create_from(exp: &self::Exp) -> Box<dyn Arith>;
}