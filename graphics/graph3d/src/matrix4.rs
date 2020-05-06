use crate::turtle;
use crate::vector3;

#[derive(Debug)]
#[derive(Clone)]
pub struct Matrix4 {
    // Columns
    j0: Vec<f64>,
    j1: Vec<f64>,
    j2: Vec<f64>,
    j3: Vec<f64>
}

impl Matrix4 {
    pub fn new() -> Matrix4 {
        Matrix4{
            j0: vec![0.0; 4],
            j1: vec![0.0; 4],
            j2: vec![0.0; 4],
            j3: vec![0.0; 4],
        }
    }

    pub fn transform(t: turtle::Turtle) -> Matrix4 {
        let mut res = Matrix4::new();
        res.j0[0] = *t.heading().x();
        res.j0[1] = *t.heading().y();
        res.j0[2] = *t.heading().z();

        res.j1[0] = *t.left().x();
        res.j1[1] = *t.left().y();
        res.j1[2] = *t.left().z();

        res.j2[0] = *t.up().x();
        res.j2[1] = *t.up().y();
        res.j2[2] = *t.up().z();

        res.j3[0] = *t.pos().x();
        res.j3[1] = *t.pos().y();
        res.j3[2] = *t.pos().z();
        res.j3[3] = 1.0;

        res
    }

    pub fn mult(&self, v: vector3::Vector3) -> vector3::Vector3 {
        let x =   self.j0[0] * v.x()
                + self.j1[0] * v.y()
                + self.j2[0] * v.z()
                + self.j3[0];  // * 1

        let y =   self.j0[1] * v.x()
                + self.j1[1] * v.y()
                + self.j2[1] * v.z()
                + self.j3[1];  // * 1

        let z =   self.j0[2] * v.x()
                + self.j1[2] * v.y()
                + self.j2[2] * v.z()
                + self.j3[2];  // * 1

        vector3::Vector3::new(x, y, z)
    }
}
