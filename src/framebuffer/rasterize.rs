use crate::{Color, Framebuffer, IVec2, Mat3, Vec3};

#[derive(Clone, Copy)]
pub struct Fragment {
    pos: Vec3,
    bary: Vec3,
}

pub trait VertexShader {
    fn transform(&self, verts: Vec3) -> Vec3;
}

pub trait FragmentShader {
    fn transform(&self, frag: Fragment) -> Color;
}

impl Framebuffer {
    pub(crate) fn triangle(
        &mut self,
        vertices: [Vec3; 3],
        vert: &impl VertexShader,
        frag: &impl FragmentShader,
    ) {
        let a = vertices[0];
        let b = vertices[1];
        let c = vertices[2];

        let bounding_box = self.bounding_box(
            IVec2::new(a.x as i32, a.y as i32),
            IVec2::new(b.x as i32, b.y as i32),
            IVec2::new(c.x as i32, c.y as i32),
        );

        let Some(start) = self.map_idx(bounding_box.0) else {
            return;
        };
        let Some(end) = self.map_idx(bounding_box.1) else {
            return;
        };
    }
}
