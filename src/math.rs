use crate::macros::{impl_mat, impl_vec};

mod mat4;
pub use mat4::Mat4;

impl_vec!(Vec2, f32, {x, y});
impl_vec!(Vec3, f32, {x, y, z});
impl_vec!(Vec4, f32, {x, y, z, w});

impl Vec3 {
    pub fn cross(self, rhs: Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
    pub fn length(self) -> f32 {
        f32::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2))
    }
    pub fn normalize(self) -> Self {
        self / self.length()
    }
}
impl Vec4 {
    pub fn project(self) -> Vec3 {
        Vec3::new(self.x / self.w, self.y / self.w, self.z / self.w)
    }
}

impl_mat!(Mat2, Vec2, {x, y});
impl_mat!(Mat3, Vec3, {x, y, z});

impl_vec!(IVec2, i32, {x, y});
impl_vec!(I64Vec2, i64, {x, y});

impl I64Vec2 {
    pub fn det(self, rhs: Self) -> i64 {
        self.x * rhs.y - self.y * rhs.x
    }
}
