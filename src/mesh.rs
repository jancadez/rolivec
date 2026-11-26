use crate::Vec3;

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub indices: Vec<[usize; 3]>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vec3>, indices: Vec<[usize; 3]>) -> Option<Self> {
        let max_vertex = vertices.len();

        for (_, &idx) in indices.iter().enumerate() {
            let out_of_bounds =
                idx[0] >= max_vertex || idx[1] >= max_vertex || idx[2] >= max_vertex;
            if out_of_bounds {
                return None;
            }
        }

        Some(Mesh { vertices, indices })
    }
}
