use std::ops::{Add, Sub};

struct Vector3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vector3 {
    fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3{x, y, z}
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
        left: Vector3::new(-1f64, 0f64, -0f64),
        up: Vector3::new(0f64, -1f64, 0f64)}
    }

    fn forward(mut self, dist: f64) {
        self.pos = self.pos + self.heading;
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
}
