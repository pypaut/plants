use std::f64::consts::PI;

use crate::turtle;
use crate::mesh;
use crate::vector3;

use turtle::Turtle;
use mesh::Mesh;
use vector3::Vector3;


pub struct Segment {
    pub a : Turtle,
    pub b : Turtle,
    pub width : f64
}

#[derive(Clone)]
pub struct Leaf {
    pub pts : Vec<Vector3>
}

impl Segment {
    pub fn a(&self) -> Turtle {
        self.a
    }

    pub fn b(&self) -> Turtle {
        self.b
    }

    pub fn width(&self) -> f64 {
        self.width
    }
}


pub fn read_str(s : &str, dist : f64, angle : f64) -> (Vec<Segment>, Vec<Leaf>) {
    let mut t = Turtle::new();
    let mut stack : Vec<Turtle> = Vec::with_capacity(10);
    let mut leaf_mode = 0;  // If true, we are creating a leaf

    let mut segments : Vec<Segment> = Vec::new();
    let mut leaves : Vec<Leaf> = Vec::new();

    let mut tmp_leaf = Leaf{pts: Vec::new()};
    let it = s.chars();
    for c in it {
        // Read characters and add data to the output file
        // Characters:
        // - Basic movements in space: +-&^\/|fF
        // - Branches: [(push state)](pop state)
        // - Leaves: {(start polygon)}(end polygon)
        match c {
            'F' => {
                let a = t.clone();
                t.forward(dist);
                let b = t.clone();
                segments.push(Segment{a, b, width : dist / 2.0});
            },  // Place two points
            'f' => {
                if leaf_mode > 0 {
                    tmp_leaf.pts.push(t.pos().clone());
                }
                t.forward(dist);
            },  // Only move except if we are creating a leaf
            '+' => {t.rot_yaw(angle);},
            '-' => {t.rot_yaw(-angle);},
            '&' => {t.rot_pitch(angle);},
            '^' => {t.rot_pitch(-angle);},
            '\\' => {t.rot_roll(angle);},
            '/' => {t.rot_roll(-angle);},
            '|' => {t.rot_yaw(PI);},
            '[' => {stack.push(t.clone());},
            ']' => {t = stack.pop().unwrap_or(t);},
            '{' => {leaf_mode += 1;},  // TODO: How can we manage leaves?
            '}' => {
                leaf_mode -= 1;
                if leaf_mode == 0 {  // Ending a leaf
                    tmp_leaf.pts.push(t.pos().clone());
                    leaves.push(tmp_leaf.clone());
                    tmp_leaf = Leaf{pts: Vec::new()};
                }
                },
            _ => {}  // Do nothing on unknown char
        }
    }

    (segments, leaves)
}

pub fn gen_geometry(segments : Vec<Segment>, leaves : Vec<Leaf>) -> Mesh {
    let mut m = Mesh::new();

    for s in segments {
        let mut top : Vec<usize> = Vec::new();  // Top vertices
        let mut bot : Vec<usize> = Vec::new();  // Bottom vertices

        //println!("{:?}", s.a);
        for i in 0..6 {  // Generate hexagons
            let mut rot = s.a().clone();
            rot.rot_roll((2.0 * PI / 6.0) * (i as f64));
            //println!("{:?}", rot);
            let p = rot.pos() + rot.up() * (s.width() / 2.0);  // Place point
            top.push(m.add_vert(&p));

            let mut rot = s.b.clone();
            rot.rot_roll((2.0 * PI / 6.0) * (i as f64));
            let p = rot.pos() + rot.up() * (s.width() / 2.0);
            bot.push(m.add_vert(&p));
        }

        let e1 = s.a().pos() - s.a().heading() * (s.width() / 2.0);
        let e2 = s.b().pos() + s.b().heading() * (s.width() / 2.0);
        let e1 = m.add_vert(&e1);
        let e2 = m.add_vert(&e2);

        // We now have all points placed, we need to set faces
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