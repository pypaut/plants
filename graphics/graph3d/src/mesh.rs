use crate::vector3;
use std::fs;


#[derive(Clone)]
pub struct Mesh {
    verts : Vec<vector3::Vector3>,
    triangles : Vec<usize>,
    leaf_faces : Vec<Vec<usize>>,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh{verts: Vec::new(), triangles: Vec::new(),
            leaf_faces: Vec::new()}
    }

    pub fn load(path: &String) -> Mesh {
        let in_str = fs::read_to_string(path)
            .expect("Failed reading file.");

        let lines = in_str.lines();

        let mut result = Mesh::new();
        for l in lines {
            let mut split = l.split(" ");
            let line_type = match split.next() {
                Some(s) => s,
                _ => {continue;}
            };

            match line_type {
                "v" => {
                    let x = split.next().unwrap_or("0.0")
                        .parse::<f64>()
                        .expect("Invalid vertex data");
                    let y = split.next().unwrap_or("0.0")
                        .parse::<f64>()
                        .expect("Invalid vertex data");
                    let z = split.next().unwrap_or("0.0")
                        .parse::<f64>()
                        .expect("Invalid vertex data");
                    result.add_vert(&vector3::Vector3::new(x, y, z));
                },
                "f" => {
                    let mut face = Vec::new();
                    for v_id in split {
                        let mut id_split = v_id.split("/");
                        let v_id = id_split.nth(0).expect("No face data.");
                        let v_id = v_id.parse::<usize>().expect("Invalid face data.") - 1;
                        face.push(v_id);
                    }
                    result.leaf_faces.push(face);
                },
                _ => {continue;}
            }
        }
        result
    }

    pub fn add_vert(&mut self, p : &vector3::Vector3) -> usize {
        let len = self.verts.len();
        self.verts.push(p.clone());

        len
    }

    pub fn add_face(&mut self, a : usize, b : usize, c : usize) {
        self.triangles.push(a);
        self.triangles.push(b);
        self.triangles.push(c);
    }

    pub fn add_poly(&mut self, f : Vec<usize>) {
            self.leaf_faces.push(f.clone());
    }

    pub fn merge(&mut self, other: &Mesh) {
        //compute offset
        let offset = self.verts.len();

        //add verts to mesh
        for v in &other.verts {
            self.verts.push(v.clone());
        }

        //add faces with index offset (use leaf_faces to support n-gons)
        for f in &other.leaf_faces {
            self.leaf_faces.push(f.iter().map(|x| x + offset).collect());
        }
    }

    pub fn get_str(self) -> String {
        let mut res = String::new();
        for v in self.verts {
            res.push_str(&String::from(format!("v {} {} {}\n", v.x(), v.y(), v.z())));
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

    pub fn get_verts(&self) -> &Vec<vector3::Vector3> {
        &self.verts
    }

    pub fn get_triangles(&self) -> &Vec<usize> {
        &self.triangles
    }

    pub fn get_leaf_faces(&self) -> &Vec<Vec<usize>> {
        &self.leaf_faces
    }

    pub fn set_verts(&mut self, verts: Vec<vector3::Vector3>) {
        self.verts = verts;
    }

    pub fn set_triangles(&mut self, triangles: Vec<usize>) {
        self.triangles = triangles;
    }

    pub fn set_leaf_faces(&mut self, leaf_faces: Vec<Vec<usize>>) {
        self.leaf_faces = leaf_faces;
    }
}
