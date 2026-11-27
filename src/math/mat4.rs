use super::Vec4;
use crate::macros::impl_mat;

impl_mat!(Mat4, Vec4, {x, y, z, w});

impl Mat4 {
    pub fn projection(fov: f32, aspect: f32, n: f32, f: f32) -> Self {
        let tan_half_fov = (fov / 2.0).tan();
        let g = 1.0 / tan_half_fov;

        let inv = 1.0 / (n - f);
        let scale = (f + n) * inv;
        let offset = (2.0 * f * n) * inv;

        Self::new(
            Vec4::new(g / aspect, 0.0, 0.0, 0.0),
            Vec4::new(0.0, g, 0.0, 0.0),
            Vec4::new(0.0, 0.0, scale, -1.0),
            Vec4::new(0.0, 0.0, offset, 0.0),
        )
    }

    pub fn rot_x(rad: f32) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();

        Self::new(
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, cos, sin, 0.0),
            Vec4::new(0.0, -sin, cos, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        )
    }

    pub fn rot_y(rad: f32) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();

        Self::new(
            Vec4::new(cos, 0.0, -sin, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(sin, 0.0, cos, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        )
    }

    pub fn rot_z(rad: f32) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();

        Self::new(
            Vec4::new(cos, sin, 0.0, 0.0),
            Vec4::new(-sin, cos, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        )
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Self {
        Self::new(
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(x, y, z, 1.0),
        )
    }
}
