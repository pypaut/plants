use crate::mesh;
use crate::turtle;

struct Object
{
    turtle: turtle::Turtle,
    mesh: mesh::Mesh
}

impl Object {
    pub fn new(mesh: mesh::Mesh, turtle: turtle::Turtle) -> Object {
        Object{mesh, turtle}
    }

    pub fn get_transformed_mesh(&self) -> mesh::Mesh {
        mesh::Mesh::new()//FIXME
    }
}