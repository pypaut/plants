use std::f64::consts::PI;

use crate::turtle;
use crate::mesh;
use crate::vector3;

use turtle::Turtle;
use mesh::Mesh;
use vector3::Vector3;
use std::collections::HashMap;
use crate::object::Object;


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
        let dir3 = s.b.pos() - self.a.pos();
        let dot_align = dir3.dot(self_dir);

        let prod = self_dir.dot(s_dir);
        prod >= 1.0 - epsilon && prod <= 1.0 + epsilon
        && dot_align >= 1.0 - epsilon && dot_align <= 1.0 + epsilon
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

pub fn read_header(s: &str, i: usize) -> (usize, HashMap<String, mesh::Mesh>) {
    let mut i = i;
    if s.as_bytes()[i] as char == '#' {
        let mut lines = s.lines();
        //get only the first line and split it
        let line_pos = if i > 0 {1} else {0};
        let mut split = match lines.nth(line_pos) {
            Some(l) => l.trim().trim_start_matches('#').split(" "),
            None => return (i, HashMap::new())
        };

        let mut map = HashMap::new();
        while let Some(object_name) = split.next() {
            let object_name = object_name.chars().collect();

            let object_mesh = match split.next() {
                Some(s) => Mesh::load(&s.to_string()),
                _ => return (i, HashMap::new())
            };

            //println!("{}", object_mesh.clone().get_str());
            map.insert(object_name, object_mesh);
        }

        while s.as_bytes()[i] as char != '\n' {
            i += 1;
        }
        (i, map)
    } else {
        (i, HashMap::new())
    }
}

pub fn read_tropism(s: &str) -> (usize, Vector3, f64) {
    let mut i = 0;
    if s.as_bytes()[0] as char == '@' {
        let mut lines = s.lines();
        //get only the first line and split it
        let mut split = match lines.nth(0) {
            Some(l) => l.trim().trim_start_matches('@').split(" "),
            None => return (0, Vector3::new(0.0,0.0,0.0), 0.0)
        };

        let mut data = Vec::new();
        for _ in 0..4 {
            let f_str = match split.next() {
                Some(f) => {f},
                _ => "0.0"
            };
            let f = f_str.parse::<f64>().expect("Invalid tropism data");

            data.push(f);
        }

        while s.as_bytes()[i] as char != '\n' {
            i += 1;
        }
        let v = Vector3::new(data[0], data[1], data[2]);
        let a = data[3];
        (i, v, a)
    } else {
        (0, Vector3::new(0.0,0.0,0.0), 0.0)
    }
}

pub fn read_str(s : &str,
                dist : f64,
                angle : f64,
                d_limits : (f64, f64),
                d_reason : f64,
                nb_colors : i64) -> (Vec<Segment>, Vec<Leaf>, Vec<Object>) {
    if d_reason > 1.0 {
        panic!("Invalid reason.");
    }

    let mut current_color_i = 0;
    let mut color_stack = Vec::new();

    let mut t = Turtle::new();
    let mut stack : Vec<Turtle> = Vec::with_capacity(10);
    let mut leaf_mode = 0;  // If true, we are creating a leaf

    let mut segments : Vec<Segment> = Vec::new();
    let mut leaves : Vec<Leaf> = Vec::new();
    let mut objects : Vec<Object> = Vec::new();

    let max_d_delta = d_limits.1 - d_limits.0;//max - min

    let mut tmp_leaf = Leaf{pts: Vec::new(), color_i: current_color_i};
    let mut leaf_stack: Vec<Leaf> = Vec::with_capacity(5);

    let len = s.len();

    //read header
    let (mut i, tropism_vec, tropism_a) = read_tropism(s);
    let (mut i, mesh_map) = read_header(s, i);

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

                //tropism. We do it before because it's more logical
                if tropism_a < -0.0001 || tropism_a > 0.0001 {
                    let r_axis = t.heading().cross(tropism_vec);
                    let alpha = tropism_a * (r_axis).norm();

                    if r_axis.norm() > 0.0001 {
                        //we use alpha for simplicity
                        t.rot_axis(alpha, r_axis.normalized());
                    }
                }

                let a = t.clone();
                t.forward(new_dist);
                let b = t.clone();

                segments.push(
                    Segment{a, b, width : d_limits.0 + t.size() * max_d_delta, color_i: current_color_i}
                );
            },  // Place two points
            'f' => {
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
                    new_angle = (new_angle * PI) / 180.0;
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
                    new_angle = (new_angle * PI) / 180.0;
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
                    new_angle = (new_angle * PI) / 180.0;
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
                    new_angle = (new_angle * PI) / 180.0;
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
                    new_angle = (new_angle * PI) / 180.0;
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
                    new_angle = (new_angle * PI) / 180.0;
                    i = e;
                }

                t.rot_roll(-new_angle);
            },
            '|' => {t.rot_yaw(PI);},
            '[' => {
                stack.push(t.clone());
                color_stack.push(current_color_i);
            },
            ']' => {
                t = stack.pop().unwrap_or(t);
                current_color_i = color_stack.pop().unwrap_or(0);
            },
            '{' => {
                leaf_stack.push(tmp_leaf.clone());
                leaf_mode += 1;
                tmp_leaf = Leaf{pts: Vec::new(), color_i: current_color_i};
            },  // :smirk:
            '}' => {
                leaf_mode -= 1;
                //tmp_leaf.pts.push(t.pos().clone());
                leaves.push(tmp_leaf.clone());
                tmp_leaf = leaf_stack.pop().unwrap_or(Leaf{pts: Vec::new(), color_i: current_color_i});
            },
            '!' => {
                let mut has_parameter = false;

                // Check for ( parameter
                if i + 1 < len && (s.as_bytes()[i+1] as char) == '(' {
                    has_parameter = true;
                    let (parameter, e) = get_parameter(s, i + 2, len);
                    t.set_size(parameter.parse().unwrap());
                    i = e;
                }

                if !has_parameter {
                    t.decrease(d_reason);
                }
            },
            '\'' => {
                current_color_i += 1;
                current_color_i %= nb_colors;
            },
            '$' => {
                let minus_g = Vector3::new(0f64, 0f64, 1f64);
                let new_left = minus_g.cross(t.heading()).normalized();
                let new_up = t.heading().cross(new_left);
                t = Turtle::new_param(t.pos(), t.heading(), new_left,
                new_up, t.size());
            },
            '~' => {
                if i + 1 < len && (s.as_bytes()[i+1] as char) == '(' {
                    let (parameter, e) = get_parameter(s, i + 2, len);
                    match mesh_map.get(parameter) {
                        Some(mesh) => {
                            objects.push(Object::new(mesh.clone(), t));
                        },
                        _ => {}
                    }
                    i = e;
                }
            },
            '.' => {
                if leaf_mode == 0 {
                    println!("ERROR : dot found out of leaf.");
                }
                tmp_leaf.pts.push(t.pos().clone());
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

    (process_segments(segments), leaves, objects)
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

pub fn gen_geometry(segments : Vec<Segment>, leaves : Vec<Leaf>,
                    objects : Vec<Object>, nb_colors: i64) -> Vec<Mesh> {

    let mut meshes : Vec<Mesh> = Vec::new();

    for _ in 0..nb_colors {
        meshes.push(Mesh::new());
    }

    for s in segments {
        let mut top : Vec<usize> = Vec::new();  // Top vertices
        let mut bot : Vec<usize> = Vec::new();  // Bottom vertices

        let current_color_i : usize = s.color_i as usize;

        //println!("{:?}", s.a);
        let nb_face = 6;
        for i in 0..nb_face {  // Generate hexagons
            let mut rot = s.a().clone();
            rot.rot_roll((2.0 * PI / (nb_face as f64)) * (i as f64));
            //println!("{:?}", rot);
            let p = rot.pos() + rot.up() * (s.width() / 2.0);  // Place point
            top.push(meshes[current_color_i].add_vert(&p));

            let mut rot = s.b.clone();
            rot.rot_roll((2.0 * PI / (nb_face as f64)) * (i as f64));
            let p = rot.pos() + rot.up() * (s.width() / 2.0);
            bot.push(meshes[current_color_i].add_vert(&p));
        }

        let e1 = s.a().pos() - s.a().heading() * (s.width() / 2.0);
        let e2 = s.b().pos() + s.b().heading() * (s.width() / 2.0);
        let e1 = meshes[current_color_i].add_vert(&e1);
        let e2 = meshes[current_color_i].add_vert(&e2);

        // We now have all points placed, we need to set faces
        for i in 0..nb_face {
            let a_t = i;
            let b_t = (i + 1) % nb_face;
            let a_b = i;
            let b_b = (i + 1) % nb_face;

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

    for obj in objects {
        //add all objects to result mesh
        meshes[0].merge(&obj.get_transformed_mesh());
    }

    meshes
}
