use std::ops::{Add, Sub, Mul};
use std::f64::consts::PI;

struct Mat3 {
    data : Vec<Vec<f64>>
}

impl Mat3 {
    fn new(data : Vec<Vec<f64>>) -> Mat3 {
        Mat3{data}
    }

    fn get(&self, i : usize, j : usize) -> f64 {
        self.data[i][j]
    }
}

#[derive(Debug)]
#[derive(Clone, Copy)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vector3 {
    fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3{x, y, z}
    }

    fn cross(&self, other : Vector3) -> Vector3 {
        Vector3{x: self.y * other.z - self.z * other.y,
        y: self.z * other.x - self.x * other.z,
        z: self.x * other.y - self.y * other.x}
    }

    fn get(&self, i : usize) -> f64 {
        if i == 0 {
            self.x
        }
        else if i == 1 {
            self.y
        }
        else {
            self.z
        }
    }

    fn set(&mut self, value : f64, i : usize) {
        if i == 0 {
            self.x = value;
        }
        else if i == 1 {
            self.y = value;
        }
        else {
            self.z = value;
        }
    }

    fn apply(&self, mat : &Mat3) -> Vector3 {
        let mut res = Vector3::new(0f64, 0f64, 0f64);

        for i in 0..3 {
            let mut tmp = 0f64;
            for j in 0..3 {
                tmp += self.get(j) * mat.get(i, j);
            }
            res.set(tmp, i);
        }

        res
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

#[derive(Debug)]
struct Turtle {
    pos: Vector3,
    heading: Vector3,
    left: Vector3,
    up: Vector3
}

impl Turtle {
    //create a new default turtle pointing upward
    fn new() -> Turtle {
        Turtle{pos: Vector3::new(0f64, 0f64, 0f64),
        heading: Vector3::new(0f64, 0f64, 1f64),
        left: Vector3::new(0f64, -1f64, 0f64),
        up: Vector3::new(1f64, 0f64, 0f64)}
    }

    fn forward(&mut self, dist: f64) {
        self.pos = self.pos + self.heading * dist;
    }
    
    fn rot_pitch(&mut self, a: f64) {
        let mat = Mat3{data: vec![
        vec![a.cos(), 0.0, -a.sin()],
        vec![0.0, 1.0, 0.0],
        vec![a.sin(), 0.0, a.cos()]]};

        self.heading = self.heading.apply(&mat);
        self.left = self.left.apply(&mat);
        self.up = self.up.apply(&mat);
    }

    fn rot_roll(&mut self, angle: f64) {
        let mat = Mat3{data: vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, a.cos(), -a.sin()],
            vec![0.0, a.sin(), a.cos()]]};

        self.heading = self.heading.apply(&mat);
        self.left = self.left.apply(&mat);
        self.up = self.up.apply(&mat);
    }

    fn rot_yaw(&mut self, angle: f64) {
        let mat = Mat3{data: vec![
            vec![a.cos(), a.sin(), 0.0],
            vec![-a.sin(), a.cos(), 0.0],
            vec![0.0, 0.0, 1.0]]};

        self.heading = self.heading.apply(&mat);
        self.left = self.left.apply(&mat);
        self.up = self.up.apply(&mat);
    }
}

fn read_str(s: &str) {
    let it = s.chars();
    for c in it {
        //read characters and add data to the output file
        //characters:
        //basic movements in space: +-&^\/|fF
        //branches: [(push state) ](pop state)
        //leaves: {(start polygon) }(end polygon)
    }
}

fn main() {
    println!("Hello, world!");
    let mut t = Turtle::new();
    println!("{:?}", &t);
    t.rot_pitch(PI/2.0);
    println!("{:?}", &t);
}
