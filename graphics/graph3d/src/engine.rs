use std::f64::consts::PI;

use crate::turtle;
use crate::mesh;
use crate::vector3;

use turtle::Turtle;
use mesh::Mesh;
use vector3::Vector3;


#[derive(Clone, Copy)]
pub struct Segment {
    pub a : Turtle,
    pub b : Turtle,
    pub width : f64,
    pub color_i : i64
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

    pub fn build_dir(&self) -> Vector3 {
        Vector3::new(
            self.b.pos().x() - self.a.pos().x(),
            self.b.pos().y() - self.a.pos().y(),
            self.b.pos().z() - self.a.pos().z(),
        ).normalized()
    }

    pub fn collinear(&self, s : Segment, epsilon : f64) -> bool {
        let self_dir : Vector3 = self.build_dir();
        let s_dir : Vector3 = s.build_dir();

        let prod = self_dir.dot(s_dir);
        prod >= 1.0 - epsilon && prod <= 1.0 + epsilon
    }

    pub fn size_eq(&self, s : &Segment, epsilon : f64) -> bool {
        let s1 = self.width;
        let s2 = s.width;

        s1 >= s2 - epsilon && s1 <= s2 + epsilon
    }
}

#[derive(Clone)]
pub struct Leaf {
    pub pts : Vec<Vector3>,
    pub color_i : i64
}

fn get_parameter(s : &str, i : usize, len : usize) -> (&str, usize) {
    let mut e = i;
    while e < len && (s.as_bytes()[e] as char) != ')' {
        e += 1;
    }

    let parameter = &s[i..e];

    (parameter, e)
}


pub fn read_str(s : &str, dist : f64, angle : f64, d_limits : (f64, f64), d_reason : f64, nb_colors : i64) -> (Vec<Segment>, Vec<Leaf>) {
    if d_reason > 1.0 {
        panic!("Invalid reason.");
    }

    let mut current_color_i = 0;

    let mut t = Turtle::new();
    let mut stack : Vec<Turtle> = Vec::with_capacity(10);
    let mut leaf_mode = 0;  // If true, we are creating a leaf

    let mut segments : Vec<Segment> = Vec::new();
    let mut leaves : Vec<Leaf> = Vec::new();

    let max_d_delta = d_limits.1 - d_limits.0;//max - min

    let mut tmp_leaf = Leaf{pts: Vec::new(), color_i: current_color_i};

    let len = s.len();
    let mut i = 0;

    while i < len {
        // Read characters and add data to the output file
        // Characters:
        // - Basic movements in space: +-&^\/|fF
        // - Branches: [(push state)](pop state)
        // - Leaves: {(start polygon)}(end polygon)
        // let mut b: u8 = s.as_bytes()[i];
        // let c : char = b as char;
        match s.as_bytes()[i] as char {
            'F' => {
                let mut new_dist = dist as f64;

                // Check for ( parameter
                if i + 1 < len && (s.as_bytes()[i+1] as char) == '(' {
                    let (parameter, e) = get_parameter(s, i + 2, len);
                    new_dist = parameter.parse().unwrap();
                    i = e;
                }

                let a = t.clone();
                t.forward(new_dist);
                let b = t.clone();
                segments.push(
                    Segment{a, b, width : d_limits.0 + t.size() * max_d_delta, color_i: current_color_i}
                );
            },  // Place two points
            'f' => {
                if leaf_mode > 0 {
                    tmp_leaf.pts.push(t.pos().clone());
                }

                let mut new_dist = dist as f64;

                // Check for ( parameter
                if i + 1 < len && (s.as_bytes()[i+1] as char) == '(' {
                    let (parameter, e) = get_parameter(s, i + 2, len);
                    new_dist = parameter.parse().unwrap();
                    i = e;
                }

                t.forward(new_dist);
            },  // Only move except if we are creating a leaf
            '+' => {
                let mut new_angle = angle as f64;

                // Check for ( parameter
                if i + 1 < len && (s.as_bytes()[i+1] as char) == '(' {
                    let (parameter, e) = get_parameter(s, i + 2, len);
                    new_angle = parameter.parse().unwrap();
                    i = e;
                }

                t.rot_yaw(new_angle);
            },
            '-' => {
                let mut new_angle = angle as f64;

                // Check for ( parameter
                if i + 1 < len && (s.as_bytes()[i+1] as char) == '(' {
                    let (parameter, e) = get_parameter(s, i + 2, len);
                    new_angle = parameter.parse().unwrap();
                    i = e;
                }

                t.rot_yaw(-new_angle);
            },
            '&' => {
                let mut new_angle = angle as f64;

                // Check for ( parameter
                if i + 1 < len && (s.as_bytes()[i+1] as char) == '(' {
                    let (parameter, e) = get_parameter(s, i + 2, len);
                    new_angle = parameter.parse().unwrap();
                    i = e;
                }

                t.rot_pitch(new_angle);
            },
            '^' => {
                let mut new_angle = angle as f64;

                // Check for ( parameter
                if i + 1 < len && (s.as_bytes()[i+1] as char) == '(' {
                    let (parameter, e) = get_parameter(s, i + 2, len);
                    new_angle = parameter.parse().unwrap();
                    i = e;
                }

                t.rot_pitch(-new_angle);
            },
            '\\' => {
                let mut new_angle = angle as f64;

                // Check for ( parameter
                if i + 1 < len && (s.as_bytes()[i+1] as char) == '(' {
                    let (parameter, e) = get_parameter(s, i + 2, len);
                    new_angle = parameter.parse().unwrap();
                    i = e;
                }

                t.rot_roll(new_angle);
            },
            '/' => {
                let mut new_angle = angle as f64;

                // Check for ( parameter
                if i + 1 < len && (s.as_bytes()[i+1] as char) == '(' {
                    let (parameter, e) = get_parameter(s, i + 2, len);
                    new_angle = parameter.parse().unwrap();
                    i = e;
                }

                t.rot_roll(-new_angle);
            },
            '|' => {t.rot_yaw(PI);},
            '[' => {stack.push(t.clone());},
            ']' => {t = stack.pop().unwrap_or(t);},
            '{' => {leaf_mode = 1;},  // TODO: How can we manage leaves?
            '}' => {
                leaf_mode = 0;
                if leaf_mode == 0 {  // Ending a leaf
                    tmp_leaf.pts.push(t.pos().clone());
                    leaves.push(tmp_leaf.clone());
                    tmp_leaf = Leaf{pts: Vec::new(), color_i: current_color_i};
                }
            },
            '!' => {
                t.decrease(d_reason);
            },
            '\'' => {
                current_color_i += 1;
                current_color_i %= nb_colors;
            },
            _ => {  // Unknown char : do nothing & ignore parameters, if any
                if i + 1 < len && (s.as_bytes()[i+1] as char) == '(' {
                    let mut e = i + 1;
                    while e < len && (s.as_bytes()[e] as char) != ')' {
                        e += 1;
                    }
                    i = e;
                }
            }
        }

        i += 1;
    }

    (process_segments(segments), leaves)
}


fn process_segments(segments : Vec<Segment>) -> Vec<Segment> {
    let mut new_segments : Vec<Segment> = Vec::new();

    let mut i = 0;
    while i < segments.len() {
        let mut j = 1;
        while i + j < segments.len() && segments[i].collinear(segments[i + j], 0.001)
            && segments[i].size_eq(&segments[i + j], 0.001) {
            j += 1;
        }

        let end_turtle = Turtle::new_param( segments[i + j - 1].b().pos(),
                                            segments[i].a().heading(),
                                            segments[i].a().left(),
                                            segments[i].a().up(),
                                            segments[i].a().size());

        new_segments.push(
            Segment{
                            a : segments[i].a(),
                            b : end_turtle,
                            width : segments[i].width(),
                            color_i : segments[i].color_i
            }
        );
        i += j;
    }

    new_segments
}

pub fn gen_geometry(segments : Vec<Segment>, leaves : Vec<Leaf>, nb_colors: i64) -> Vec<Mesh> {

    let mut meshes : Vec<Mesh> = Vec::new();

    for _ in 0..nb_colors {
        meshes.push(Mesh::new());
    }

    for s in segments {
        let mut top : Vec<usize> = Vec::new();  // Top vertices
        let mut bot : Vec<usize> = Vec::new();  // Bottom vertices

        let current_color_i : usize = s.color_i as usize;

        //println!("{:?}", s.a);
        for i in 0..6 {  // Generate hexagons
            let mut rot = s.a().clone();
            rot.rot_roll((2.0 * PI / 6.0) * (i as f64));
            //println!("{:?}", rot);
            let p = rot.pos() + rot.up() * (s.width() / 2.0);  // Place point
            top.push(meshes[current_color_i].add_vert(&p));

            let mut rot = s.b.clone();
            rot.rot_roll((2.0 * PI / 6.0) * (i as f64));
            let p = rot.pos() + rot.up() * (s.width() / 2.0);
            bot.push(meshes[current_color_i].add_vert(&p));
        }

        let e1 = s.a().pos() - s.a().heading() * (s.width() / 2.0);
        let e2 = s.b().pos() + s.b().heading() * (s.width() / 2.0);
        let e1 = meshes[current_color_i].add_vert(&e1);
        let e2 = meshes[current_color_i].add_vert(&e2);

        // We now have all points placed, we need to set faces
        for i in 0..6 {
            let a_t = i;
            let b_t = (i + 1) % 6;
            let a_b = i;
            let b_b = (i + 1) % 6;

            meshes[current_color_i].add_face(top[a_t], top[b_t], bot[a_b]);
            meshes[current_color_i].add_face(top[b_t], bot[b_b], bot[a_b]);

            meshes[current_color_i].add_face(top[a_t], e1, top[b_t]);
            meshes[current_color_i].add_face(bot[b_b], e2, bot[a_b]);
        }
    }

    for l in leaves {
        let current_color_i : usize = l.color_i as usize;
        let mut verts : Vec<usize> = Vec::new();

        for v in l.pts {
            verts.push(meshes[current_color_i].add_vert(&v));
        }
        meshes[current_color_i].add_poly(verts);
    }

    meshes
}
