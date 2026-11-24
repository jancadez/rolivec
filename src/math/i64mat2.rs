use crate::I64Vec2;
use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct I64Mat2 {
    pub x: I64Vec2,
    pub y: I64Vec2,
}

impl I64Mat2 {
    pub fn new(x: I64Vec2, y: I64Vec2) -> I64Mat2 {
        I64Mat2 { x, y }
    }

    pub fn determinant(self) -> i64 {
        self.x.x * self.y.y - self.x.y * self.y.x
    }
}

impl Mul for I64Mat2 {
    type Output = I64Mat2;
    fn mul(self, rhs: I64Mat2) -> I64Mat2 {
        I64Mat2::new(
            self * rhs.x,
            self * rhs.y,
        )
    }
}

impl Mul<I64Vec2> for I64Mat2 {
    type Output = I64Vec2;
    fn mul(self, rhs: I64Vec2) -> I64Vec2 {
        I64Vec2::new(
            self.x.x * rhs.x + self.y.x * rhs.y,
            self.x.y * rhs.x + self.y.y * rhs.y,
        )
    }
}
