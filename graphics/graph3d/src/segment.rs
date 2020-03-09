use crate::turtle;


pub struct Segment {
    pub a : turtle::Turtle,
    pub b : turtle::Turtle,
    pub width : f64
}

impl Segment {
    pub fn a(&self) -> turtle::Turtle {
        self.a
    }

    pub fn b(&self) -> turtle::Turtle {
        self.b
    }

    pub fn width(&self) -> f64 {
        self.width
    }
}
