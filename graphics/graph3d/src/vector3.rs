use std::ops::{Add, Sub, Mul};
use vecmath;


#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3{x, y, z}
    }

    pub fn from(v : vecmath::Vector3<f64>) -> Vector3 {
        Vector3{x: v[0], y: v[1], z: v[2]}
    }

    pub fn to_arr(&self) -> vecmath::Vector3<f64> {
        [self.x, self.y, self.z]
    }

    pub fn x(&self) -> &f64 {  // x getter
        &self.x
    }

    pub fn y(&self) -> &f64 {  // y getter
        &self.y
    }

    pub fn z(&self) -> &f64 {  // z getter
        &self.z
    }

    pub fn norm(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn times(&self, a : f64) -> Vector3 {
        Vector3::new(self.x * a, self.y * a, self.z * a)
    }

    pub fn normalized(&self) -> Vector3 {
        self.times(1.0 / self.norm())
    }

    pub fn dot(&self, v : Vector3) -> f64 {
        let dot = self.x * v.x + self.y * v.y + self.z * v.z;
        dot
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3{x: self.x + rhs.x,
        y: self.y + rhs.y,
        z: self.z + rhs.z}
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3{x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z}
    }
}
