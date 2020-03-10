use crate::vector3;


pub struct Mesh {
    verts : Vec<vector3::Vector3>,
    triangles : Vec<usize>,
    leaf_faces : Vec<Vec<usize>>
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh{verts: Vec::new(), triangles: Vec::new(),
            leaf_faces: Vec::new()}
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
}
