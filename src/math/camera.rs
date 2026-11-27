use crate::{Mat4, Vec3};

pub struct Camera {
    pos: Vec3,
    yaw: f32,
    pitch: f32,
    fov: f32,
    aspect: f32,
    near: f32,
    far: f32,
    view_proj: Mat4,
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
        let mut camera = Camera {
            pos,
            yaw,
            pitch,
            fov,
            aspect: scr_w as f32 / scr_h as f32,
            near,
            far,
            view_proj: Mat4::identity(),
        };

        camera.set_view_proj();
        camera
    }

    pub fn proj(&self) -> Mat4 {
        Mat4::projection(self.fov, self.aspect, self.near, self.far)
    }

    pub fn view(&self) -> Mat4 {
        let np = -self.pos;

        Mat4::rot_x(self.pitch) * Mat4::rot_y(self.yaw) * Mat4::translate(np.x, np.y, np.z)
    }

    pub fn view_proj(&self) -> Mat4 {
        self.view_proj
    }

    pub fn forward(&self) -> Vec3 {
        Vec3::new(
            self.pitch.cos() * self.yaw.sin(),
            -self.pitch.sin(),
            -self.pitch.cos() * self.yaw.cos(),
        )
    }

    pub fn forward_fps(&self) -> Vec3 {
        Vec3::new(self.yaw.sin(), 0.0, -self.yaw.cos())
    }

    pub fn right(&self) -> Vec3 {
        self.forward().cross(Vec3::y()).normalize()
    }

    pub fn pos(&self) -> Vec3 {
        self.pos
    }

    pub fn yaw(&self) -> f32 {
        self.yaw
    }

    pub fn pitch(&self) -> f32 {
        self.pitch
    }

    pub fn fov(&self) -> f32 {
        self.fov
    }

    pub fn aspect(&self) -> f32 {
        self.aspect
    }

    pub fn near(&self) -> f32 {
        self.near
    }

    pub fn far(&self) -> f32 {
        self.far
    }

    pub fn move_forward(&mut self, d: f32) {
        self.pos += self.forward() * d;
        self.set_view_proj();
    }

    pub fn move_forward_fps(&mut self, d: f32) {
        self.pos += self.forward_fps() * d;
        self.set_view_proj();
    }

    pub fn move_right(&mut self, d: f32) {
        self.pos += self.right() * d;
        self.set_view_proj();
    }

    pub fn move_up(&mut self, d: f32) {
        self.pos += Vec3::y() * d;
        self.set_view_proj();
    }

    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
        self.set_view_proj();
    }

    pub fn set_yaw(&mut self, yaw: f32) {
        self.yaw = yaw;
        self.set_view_proj();
    }

    pub fn set_pitch(&mut self, pitch: f32) {
        self.pitch = pitch;
        self.set_view_proj();
    }

    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
        self.set_view_proj();
    }

    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
        self.set_view_proj();
    }

    pub fn set_near(&mut self, near: f32) {
        self.near = near;
        self.set_view_proj();
    }

    pub fn set_far(&mut self, far: f32) {
        self.far = far;
        self.set_view_proj();
    }

    fn set_view_proj(&mut self) {
        self.view_proj = self.proj() * self.view();
    }
}
