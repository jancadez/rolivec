use crate::color::Color;
use crate::math::{Mat4, Vec3, Vec4};
use crate::shade::{Empty, Fragment, Mvp, Shader};

pub struct BarycentricShader {
    pub mvp: Mvp,
}

impl BarycentricShader {
    pub fn new(model: Mat4, view_proj: Mat4) -> Self {
        Self {
            mvp: Mvp::new(model, view_proj),
        }
    }
}

impl Shader for BarycentricShader {
    type Vertex = Vec3;
    type Varying = Empty;

    fn vertex(&self, v: &Self::Vertex) -> (Vec4, Self::Varying) {
        (self.mvp.clip(v), Empty)
    }

    fn fragment(&self, _v: Self::Varying, frag: Fragment) -> Option<Color> {
        let r = (255_f32 * frag.bary.x) as u8;
        let g = (255_f32 * frag.bary.y) as u8;
        let b = (255_f32 * frag.bary.z) as u8;
        Some(Color::rgb(r, g, b))
    }
}
