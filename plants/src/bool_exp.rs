use crate::arith;

trait BoolExp {
    fn eval(&self) -> bool;
    fn vars(&self) -> Vec<&str>;
    fn set(&mut self, var: &str, val: f32) -> Result<(), ()>;
}