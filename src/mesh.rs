use crate::Vec3;
use std::fmt;

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub indices: Vec<(usize, usize, usize)>,
}

#[derive(Debug)]
pub enum MeshError {
    InvalidIndex {
        position: usize,
        value: (usize, usize, usize),
        vertex_count: usize,
    },
}

impl fmt::Display for MeshError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MeshError::InvalidIndex {
                position,
                value,
                vertex_count,
            } => {
                write!(
                    f,
                    "Index at position {} ({:?}) is out of bounds for a vertex array of size {}",
                    position, value, vertex_count
                )
            }
        }
    }
}

impl std::error::Error for MeshError {}

impl Mesh {
    pub fn new(
        vertices: Vec<Vec3>,
        indices: Vec<(usize, usize, usize)>,
    ) -> Result<Self, MeshError> {
        let max_vertex = vertices.len();

        for (i, &idx) in indices.iter().enumerate() {
            let out_of_bounds = idx.0 >= max_vertex || idx.1 >= max_vertex || idx.2 >= max_vertex;
            if out_of_bounds {
                return Err(MeshError::InvalidIndex {
                    position: i,
                    value: idx,
                    vertex_count: vertices.len(),
                });
            }
        }

        Ok(Mesh { vertices, indices })
    }
}
