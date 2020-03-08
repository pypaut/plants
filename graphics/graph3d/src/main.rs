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
#[derive(Copy, Clone)]
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

    fn rot_roll(&mut self, a: f64) {
        let mat = Mat3{data: vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, a.cos(), -a.sin()],
            vec![0.0, a.sin(), a.cos()]]};

        self.heading = self.heading.apply(&mat);
        self.left = self.left.apply(&mat);
        self.up = self.up.apply(&mat);
    }

    fn rot_yaw(&mut self, a: f64) {
        let mat = Mat3{data: vec![
            vec![a.cos(), a.sin(), 0.0],
            vec![-a.sin(), a.cos(), 0.0],
            vec![0.0, 0.0, 1.0]]};

        self.heading = self.heading.apply(&mat);
        self.left = self.left.apply(&mat);
        self.up = self.up.apply(&mat);
    }
}

struct Segment {
    a : Turtle,
    b : Turtle,
    width : f64
}

fn read_str(s : &str, dist : f64, angle : f64) -> Vec<Segment> {
    let mut t = Turtle::new();
    let mut stack : Vec<Turtle> = Vec::with_capacity(10);
    let mut leaf_mode = 0;//if true, we are creating a leaf

    let mut segments : Vec<Segment> = Vec::new();

    let it = s.chars();
    for c in it {
        //read characters and add data to the output file
        //characters:
        //basic movements in space: +-&^\/|fF
        //branches: [(push state) ](pop state)
        //leaves: {(start polygon) }(end polygon)
        match c {
            'F' => {
                let a = t.clone();
                t.forward(dist);
                let b = t.clone();
                segments.push(Segment{a, b, width : dist / 4.0});
                },//place two points
            'f' => {t.forward(dist);},//only move
            '+' => {t.rot_yaw(angle);},
            '-' => {t.rot_yaw(-angle);},
            '&' => {t.rot_pitch(angle);},
            '^' => {t.rot_pitch(-angle);},
            '\\' => {t.rot_roll(angle);},
            '/' => {t.rot_roll(-angle);},
            '|' => {t.rot_yaw(PI);},
            '[' => {stack.push(t.clone());},
            ']' => {t = stack.pop().unwrap_or(t);},
            '{' => {leaf_mode += 1;},//TODO: How can we manage leaves?
            '}' => {leaf_mode -= 1;},
            _ => {}//do nothing on unknown char
        }
    }

    segments
}

struct Mesh {
    verts : Vec<Vector3>,
    triangles : Vec<usize>
}

impl Mesh {
    fn new() -> Mesh {
        Mesh{verts: Vec::new(), triangles: Vec::new()}
    }

    fn add_vert(&mut self, p : &Vector3) -> usize {
        let len = self.verts.len();
        self.verts.push(p.clone());

        len
    }

    fn add_face(&mut self, a : usize, b : usize, c : usize) {
        self.triangles.push(a);
        self.triangles.push(b);
        self.triangles.push(c);
    }

    fn get_str(self) -> String {
        let mut res = String::new();
        for v in self.verts {
            res.push_str(&String::from(format!("v {} {} {}\n", v.x, v.y, v.z)));
        }

        res.push_str("\n");

        for i in (0..self.triangles.len()).step_by(3) {
            res.push_str(&String::from(format!("f {}// {}// {}//\n", self.triangles[i],
                self.triangles[i + 1], self.triangles[i + 2])));
        }

        res
    }
}

fn gen_geometry(segments : Vec<Segment>) -> Mesh {
    let mut m = Mesh::new();

    for s in segments {
        let mut top : Vec<usize> = Vec::new();//top vertices
        let mut bot : Vec<usize> = Vec::new();//bottom vertices

        for i in 0..6 { //generate hexagons
            let mut rot = s.a.clone();
            rot.rot_roll((2.0 * PI / 6.0) * i as f64);
            let p = rot.pos + rot.up * (s.width / 2.0);//place a point
            top.push(m.add_vert(&p));

            let mut rot = s.b.clone();
            rot.rot_roll((2.0 * PI / 6.0) * i as f64);
            let p = rot.pos + rot.up * (s.width / 2.0);
            bot.push(m.add_vert(&p));
        }

        let e1 = s.a.pos + s.a.heading * (s.width / 2.0);
        let e2 = s.b.pos + s.b.heading * (s.width / 2.0);
        let e1 = m.add_vert(&e1);
        let e2 = m.add_vert(&e2);

        //we now have all points placed, we need to set faces
        for i in 0..6 {
            let a_t = i;
            let b_t = i + 1;
            let a_b = i;
            let b_b = i;

            m.add_face(top[a_t], top[b_t], bot[a_b]);
            m.add_face(top[b_t], bot[b_b], bot[a_b]);

            m.add_face(top[a_t], e1, top[b_t]);
            m.add_face(bot[b_b], e2, bot[a_b]);
        }
    }

    m
}

fn main() {
    println!("Hello, world!");
    let mut t = Turtle::new();
    println!("{:?}", &t);
    t.rot_pitch(PI/2.0);
    println!("{:?}", &t);
}
