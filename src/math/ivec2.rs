use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}

impl IVec2 {
    pub fn new(x: i32, y: i32) -> IVec2 {
        IVec2 { x, y }
    }

    pub fn splat(n: i32) -> IVec2 {
        IVec2::new(n, n)
    }

    pub fn min(self, rhs: IVec2) -> IVec2 {
        IVec2::new(self.x.min(rhs.x), self.y.min(rhs.y))
    }

    pub fn max(self, rhs: IVec2) -> IVec2 {
        IVec2::new(self.x.max(rhs.x), self.y.max(rhs.y))
    }
}

impl Add for IVec2 {
    type Output = IVec2;
    fn add(self, rhs: IVec2) -> IVec2 {
        IVec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for IVec2 {
    type Output = IVec2;
    fn sub(self, rhs: IVec2) -> IVec2 {
        IVec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<i32> for IVec2 {
    type Output = IVec2;
    fn mul(self, rhs: i32) -> IVec2 {
        IVec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<i32> for IVec2 {
    type Output = IVec2;
    fn div(self, rhs: i32) -> IVec2 {
        IVec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
