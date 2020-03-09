use std::ops::{Add, Sub, Mul};
use std::f64::consts::PI;
use std::{env, fs};
use std::fs::File;
use std::error::Error;
use std::io::Write;
use quaternion;
use vecmath;

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

    fn from(v : vecmath::Vector3<f64>) -> Vector3 {
        Vector3{x: v[0], y: v[1], z: v[2]}
    }

    fn to_arr(&self) -> vecmath::Vector3<f64> {
        [self.x, self.y, self.z]
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
        let quat = quaternion::axis_angle(self.left.to_arr(), a);

        self.heading = Vector3::from(quaternion::rotate_vector(quat,
                                                               self.heading.to_arr()));
        self.left = Vector3::from(quaternion::rotate_vector(quat,
                                                            self.left.to_arr()));
        self.up = Vector3::from(quaternion::rotate_vector(quat,
                                                            self.up.to_arr()));
    }

    fn rot_roll(&mut self, a: f64) {
        let quat = quaternion::axis_angle(self.heading.to_arr(), a);

        self.heading = Vector3::from(quaternion::rotate_vector(quat,
                                                               self.heading.to_arr()));
        self.left = Vector3::from(quaternion::rotate_vector(quat,
                                                            self.left.to_arr()));
        self.up = Vector3::from(quaternion::rotate_vector(quat,
                                                          self.up.to_arr()));
    }

    fn rot_yaw(&mut self, a: f64) {
        let quat = quaternion::axis_angle(self.up.to_arr(), a);

        self.heading = Vector3::from(quaternion::rotate_vector(quat,
                                                               self.heading.to_arr()));
        self.left = Vector3::from(quaternion::rotate_vector(quat,
                                                            self.left.to_arr()));
        self.up = Vector3::from(quaternion::rotate_vector(quat,
                                                          self.up.to_arr()));
    }
}

struct Segment {
    a : Turtle,
    b : Turtle,
    width : f64
}

#[derive(Clone)]
struct Leaf {
    pts : Vec<Vector3>
}

fn read_str(s : &str, dist : f64, angle : f64) -> (Vec<Segment>, Vec<Leaf>) {
    let mut t = Turtle::new();
    let mut stack : Vec<Turtle> = Vec::with_capacity(10);
    let mut leaf_mode = 0;//if true, we are creating a leaf

    let mut segments : Vec<Segment> = Vec::new();
    let mut leaves : Vec<Leaf> = Vec::new();

    let mut tmp_leaf = Leaf{pts: Vec::new()};
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
                segments.push(Segment{a, b, width : dist / 2.0});
                },//place two points
            'f' => {
                if leaf_mode > 0 {
                    tmp_leaf.pts.push(t.pos.clone());
                }
                t.forward(dist);
                },//only move except if we are creating a leaf
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
            '}' => {
                leaf_mode -= 1;
                if leaf_mode == 0 {//ending a leaf
                    tmp_leaf.pts.push(t.pos.clone());
                    leaves.push(tmp_leaf.clone());
                    tmp_leaf = Leaf{pts: Vec::new()};
                }
                },
            _ => {}//do nothing on unknown char
        }
    }

    (segments, leaves)
}

struct Mesh {
    verts : Vec<Vector3>,
    triangles : Vec<usize>,
    leaf_faces : Vec<Vec<usize>>
}

impl Mesh {
    fn new() -> Mesh {
        Mesh{verts: Vec::new(), triangles: Vec::new(),
            leaf_faces: Vec::new()}
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

    fn add_poly(&mut self, f : Vec<usize>) {
            self.leaf_faces.push(f.clone());
    }

    fn get_str(self) -> String {
        let mut res = String::new();
        for v in self.verts {
            res.push_str(&String::from(format!("v {} {} {}\n", v.x, v.y, v.z)));
        }

        res.push_str("\ng branches\n");

        for i in (0..self.triangles.len()).step_by(3) {
            res.push_str(&String::from(format!("f {}// {}// {}//\n",
                                               self.triangles[i] + 1,
                                               self.triangles[i + 1] + 1,
                                               self.triangles[i + 2] + 1)));
        }

        res.push_str("\ng leaves\n");
        for f in self.leaf_faces {
            res.push_str("f");
            for v in f {
                res.push_str(&String::from(format!(" {}//", v + 1)));
            }
            res.push_str("\n");
        }

        res
    }
}

fn gen_geometry(segments : Vec<Segment>, leaves : Vec<Leaf>) -> Mesh {
    let mut m = Mesh::new();

    for s in segments {
        let mut top : Vec<usize> = Vec::new();//top vertices
        let mut bot : Vec<usize> = Vec::new();//bottom vertices

        //println!("{:?}", s.a);
        for i in 0..6 { //generate hexagons
            let mut rot = s.a.clone();
            rot.rot_roll((2.0 * PI / 6.0) * (i as f64));
            //println!("{:?}", rot);
            let p = rot.pos + rot.up * (s.width / 2.0);//place a point
            top.push(m.add_vert(&p));

            let mut rot = s.b.clone();
            rot.rot_roll((2.0 * PI / 6.0) * (i as f64));
            let p = rot.pos + rot.up * (s.width / 2.0);
            bot.push(m.add_vert(&p));
        }

        let e1 = s.a.pos - s.a.heading * (s.width / 2.0);
        let e2 = s.b.pos + s.b.heading * (s.width / 2.0);
        let e1 = m.add_vert(&e1);
        let e2 = m.add_vert(&e2);

        //we now have all points placed, we need to set faces
        for i in 0..6 {
            let a_t = i;
            let b_t = (i + 1) % 6;
            let a_b = i;
            let b_b = (i + 1) % 6;

            m.add_face(top[a_t], top[b_t], bot[a_b]);
            m.add_face(top[b_t], bot[b_b], bot[a_b]);

            m.add_face(top[a_t], e1, top[b_t]);
            m.add_face(bot[b_b], e2, bot[a_b]);
        }
    }

    for l in leaves {
        let mut verts : Vec<usize> = Vec::new();

        for v in l.pts {
            verts.push(m.add_vert(&v));
        }
        m.add_poly(verts);
    }

    m
}

fn main() {
    let args : Vec<String> = env::args().collect();
    let input = args[1].clone();
    let output = args[2].clone();
    let angle = args[3].parse::<f64>().expect("Failed while parsing angle.");
    let dist = args[4].parse::<f64>().expect("Failed while parsing distance.");

    //read the file, get the string, generate the segments, generate the geometry, print the geometry
    let in_str = fs::read_to_string(input)
        .expect("Failed reading file.");

    let (segments, leaves) = read_str(&in_str,
                                      dist, angle * (PI / 180.0));
    let mesh = gen_geometry(segments, leaves);
    let out_str = mesh.get_str();

    let mut file = match File::create(&output) {
        Err(why) => panic!("couldn't create {}: {}", output, why.description()),
        Ok(file) => file,
    };

    match file.write_all(out_str.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", output, why.description()),
        Ok(_) => println!("successfully wrote to {}", output),
    }
}
