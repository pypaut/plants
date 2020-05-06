use crate::matrix4;
use crate::mesh;
use crate::turtle;

pub struct Object
{
    turtle: turtle::Turtle,
    mesh: mesh::Mesh
}

impl Object {
    pub fn new(mesh: mesh::Mesh, turtle: turtle::Turtle) -> Object {
        Object{mesh, turtle}
    }

    pub fn get_transformed_mesh(&self) -> mesh::Mesh {
        let mut res = mesh::Mesh::new();
        res.set_triangles(self.mesh.get_triangles().to_vec());
        res.set_leaf_faces(self.mesh.get_leaf_faces().to_vec());

        // Create transform matrix
        let transform = matrix4::Matrix4::transform(self.turtle);

        // Get the transformed vertices into the new mesh
        let mut verts = Vec::new();
        for vert in self.mesh.get_verts().to_vec().iter() {
            verts.push(
                transform.mult(*vert)
            );
        }

        res.set_verts(verts);

        res
    }
}
