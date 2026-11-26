use crate::{Mat4, Vec3};

pub struct Camera {
    pub pos: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new(
        pos: Vec3,
        yaw: f32,
        pitch: f32,
        fov: f32,
        scr_w: usize,
        scr_h: usize,
        near: f32,
        far: f32,
    ) -> Camera {
        Camera {
            pos,
            yaw,
            pitch,
            fov,
            aspect: scr_w as f32 / scr_h as f32,
            near,
            far,
        }
    }

    pub fn proj(&mut self) -> Mat4 {
        Mat4::projection(self.fov, self.aspect, self.near, self.far)
    }

    pub fn view(&mut self) -> Mat4 {
        let np = -self.pos;

        Mat4::rot_x(self.pitch) * Mat4::rot_y(self.yaw) * Mat4::translate(np.x, np.y, np.z)
    }

    pub fn view_proj(&mut self) -> Mat4 {
        self.proj() * self.view()
    }

    pub fn forward(&self) -> Vec3 {
        Vec3::new(
            self.pitch.cos() * self.yaw.sin(),
            -self.pitch.sin(),
            -self.pitch.cos() * self.yaw.cos(),
        )
    }

    pub fn right(&self) -> Vec3 {
        self.forward().cross(Vec3::y()).normalize()
    }

    pub fn move_forward(&mut self, d: f32) {
        self.pos += self.forward() * d;
    }

    pub fn move_right(&mut self, d: f32) {
        self.pos += self.right() * d;
    }

    pub fn move_up(&mut self, d: f32) {
        self.pos += Vec3::y() * d;
    }
}
