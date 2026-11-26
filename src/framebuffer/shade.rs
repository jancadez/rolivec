use crate::{Color, Vec3, Vec4};

#[derive(Clone, Copy)]
pub struct Fragment {
    pub pos: Vec3,
    pub bary: Vec3,
}

pub trait VertexShader {
    fn transform(&self, vert: Vec3) -> Vec4;
}

pub trait FragmentShader {
    fn shade(&self, frag: Fragment) -> Option<Color>;
}

