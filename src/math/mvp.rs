use crate::{Mat4, Vec3, Vec4, VertexShader};

pub struct Mvp {
    model: Mat4,
    view_proj: Mat4,
    mvp: Mat4,
}

impl Mvp {
    pub fn new(model: Mat4, view_proj: Mat4) -> Mvp {
        Mvp {
            model,
            view_proj: view_proj,
            mvp: view_proj * model,
        }
    }

    pub fn model(&self) -> Mat4 {
        self.model
    }

    pub fn view_proj(&self) -> Mat4 {
        self.view_proj
    }

    pub fn set_model(&mut self, model: Mat4) {
        self.model = model;
        self.compute_mvp();
    }

    pub fn set_view_proj(&mut self, view_proj: Mat4) {
        self.view_proj = view_proj;
        self.compute_mvp();
    }

    fn compute_mvp(&mut self) {
        self.mvp = self.view_proj * self.model;
    }
}

impl VertexShader for Mvp {
    fn transform(&self, vert: Vec3) -> Vec4 {
        self.mvp * Vec4::new(vert.x, vert.y, vert.z, 1.0)
    }
}
