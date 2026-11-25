use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct I64Vec2 {
    pub x: i64,
    pub y: i64,
}

impl I64Vec2 {
    pub fn new(x: i64, y: i64) -> I64Vec2 {
        I64Vec2 { x, y }
    }
}

impl Add for I64Vec2 {
    type Output = I64Vec2;
    fn add(self, rhs: I64Vec2) -> I64Vec2 {
        I64Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for I64Vec2 {
    type Output = I64Vec2;
    fn sub(self, rhs: I64Vec2) -> I64Vec2 {
        I64Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<i64> for I64Vec2 {
    type Output = I64Vec2;
    fn mul(self, rhs: i64) -> I64Vec2 {
        I64Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<i64> for I64Vec2 {
    type Output = I64Vec2;
    fn div(self, rhs: i64) -> I64Vec2 {
        I64Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
