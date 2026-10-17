use crate::Vec4;
use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Mat4 {
    pub x: Vec4,
    pub y: Vec4,
    pub z: Vec4,
    pub w: Vec4,
}

impl Mat4 {
    pub fn new(x: Vec4, y: Vec4, z: Vec4, w: Vec4) -> Mat4 {
        Mat4 { x, y, z, w }
    }

    pub fn projection(fov: f32, aspect: f32, n: f32, f: f32) -> Mat4 {
        let tan_half_fov = (fov / 2.0).tan();
        let g = 1.0 / tan_half_fov;

        let inv = 1.0 / (n - f);
        let scale = (f + n) * inv;
        let offset = (2.0 * f * n) * inv;

        Mat4::new(
            Vec4::new(g / aspect, 0.0, 0.0, 0.0),
            Vec4::new(0.0, g, 0.0, 0.0),
            Vec4::new(0.0, 0.0, scale, -1.0),
            Vec4::new(0.0, 0.0, offset, 0.0),
        )
    }
}

impl Mul for Mat4 {
    type Output = Mat4;
    fn mul(self, rhs: Mat4) -> Mat4 {
        Mat4::new(
            self * Vec4::new(rhs.x.x, rhs.x.y, rhs.x.z, rhs.x.w),
            self * Vec4::new(rhs.y.x, rhs.y.y, rhs.y.z, rhs.y.w),
            self * Vec4::new(rhs.z.x, rhs.z.y, rhs.z.z, rhs.z.w),
            self * Vec4::new(rhs.w.x, rhs.w.y, rhs.w.z, rhs.w.w),
        )
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;
    fn mul(self, rhs: Vec4) -> Vec4 {
        Vec4::new(
            self.x.x * rhs.x + self.y.x * rhs.y + self.z.x * rhs.z + self.w.x * rhs.w,
            self.x.y * rhs.x + self.y.y * rhs.y + self.z.y * rhs.z + self.w.y * rhs.w,
            self.x.z * rhs.x + self.y.z * rhs.y + self.z.z * rhs.z + self.w.z * rhs.w,
            self.x.w * rhs.x + self.y.w * rhs.y + self.z.w * rhs.z + self.w.w * rhs.w,
        )
    }
}
