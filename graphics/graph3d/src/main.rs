use std::f64::consts::PI;
use std::{env, fs};
use std::fs::File;
use std::error::Error;
use std::io::Write;

mod leaf;
mod mesh;
mod segment;
mod turtle;
mod vector3;

use leaf::Leaf;
use mesh::Mesh;
use segment::Segment;
use turtle::Turtle;


fn read_str(s : &str, dist : f64, angle : f64) -> (Vec<Segment>, Vec<Leaf>) {
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

fn gen_geometry(segments : Vec<Segment>, leaves : Vec<Leaf>) -> Mesh {
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

fn main() {
    // Parse arguments
    let args : Vec<String> = env::args().collect();
    let input = args[1].clone();
    let output = args[2].clone();
    let angle = args[3].parse::<f64>().expect("Failed while parsing angle.");
    let dist = args[4].parse::<f64>().expect("Failed while parsing distance.");

    // Read file and get string
    let in_str = fs::read_to_string(input)
        .expect("Failed reading file.");

    // Generate segments
    let (segments, leaves) = read_str(&in_str,
                                      dist, angle * (PI / 180.0));

    // Generate & print geometry
    let mesh = gen_geometry(segments, leaves);
    let out_str = mesh.get_str();

    let mut file = match File::create(&output) {
        Err(why) => panic!("Couldn't create {}: {}", output, why.description()),
        Ok(file) => file,
    };

    match file.write_all(out_str.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", output, why.description()),
        Ok(_) => println!("Successfully wrote to {}", output),
    }
}
