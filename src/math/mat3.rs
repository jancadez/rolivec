use crate::Vec3;
use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Mat3 {
    pub x: Vec3,
    pub y: Vec3,
    pub z: Vec3,
}

impl Mat3 {
    pub fn new(x: Vec3, y: Vec3, z: Vec3) -> Mat3 {
        Mat3 { x, y, z }
    }
}

impl Mul for Mat3 {
    type Output = Mat3;
    fn mul(self, rhs: Mat3) -> Mat3 {
        Mat3::new(
            self * Vec3::new(rhs.x.x, rhs.x.y, rhs.x.z),
            self * Vec3::new(rhs.y.x, rhs.y.y, rhs.y.z),
            self * Vec3::new(rhs.z.x, rhs.z.y, rhs.z.z),
        )
    }
}

impl Mul<Vec3> for Mat3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.x.x * rhs.x + self.y.x * rhs.y + self.z.x * rhs.z,
            self.x.y * rhs.x + self.y.y * rhs.y + self.z.y * rhs.z,
            self.x.z * rhs.x + self.y.z * rhs.y + self.z.z * rhs.z,
        )
    }
}
