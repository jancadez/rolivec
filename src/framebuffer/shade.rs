use crate::{Color, Mat4, Vec3, Vec4};

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

pub struct Mvp {
    model: Mat4,
    view: Mat4,
    projection: Mat4,
    view_proj: Mat4,
    mvp: Mat4,
}

impl Mvp {
    pub fn new(model: Mat4, view: Mat4, projection: Mat4) -> Mvp {
        Mvp {
            model,
            view,
            projection,
            view_proj: projection * view,
            mvp: projection * view * model,
        }
    }

    pub fn model(&self) -> Mat4 {
        self.model
    }

    pub fn view(&self) -> Mat4 {
        self.view
    }

    pub fn projection(&self) -> Mat4 {
        self.projection
    }

    pub fn set_model(&mut self, model: Mat4) {
        self.model = model;
        self.mvp = self.view_proj * model;
    }

    pub fn set_view(&mut self, view: Mat4) {
        self.view = view;
        self.view_proj = self.projection * view;
        self.mvp = self.view_proj * self.model;
    }

    pub fn set_projection(&mut self, projection: Mat4) {
        self.projection = projection;
        self.view_proj = projection * self.view;
        self.mvp = self.view_proj * self.model;
    }
}

impl VertexShader for Mvp {
    fn transform(&self, vert: Vec3) -> Vec4 {
        self.mvp * Vec4::new(vert.x, vert.y, vert.z, 1.0)
    }
}
