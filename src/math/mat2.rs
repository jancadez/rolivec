use crate::Vec2;
use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Mat2 {
    pub x: Vec2,
    pub y: Vec2,
}

impl Mat2 {
    pub fn new(x: Vec2, y: Vec2) -> Mat2 {
        Mat2 { x, y }
    }

    pub fn determinant(self) -> f32 {
        self.x.x * self.y.y - self.x.y * self.y.x
    }
}

impl Mul for Mat2 {
    type Output = Mat2;
    fn mul(self, rhs: Mat2) -> Mat2 {
        Mat2::new(
            self * Vec2::new(rhs.x.x, rhs.x.y),
            self * Vec2::new(rhs.y.x, rhs.y.y),
        )
    }
}

impl Mul<Vec2> for Mat2 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2::new(
            self.x.x * rhs.x + self.y.x * rhs.y,
            self.x.y * rhs.x + self.y.y * rhs.y,
        )
    }
}
