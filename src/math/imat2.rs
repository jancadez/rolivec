use crate::IVec2;
use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct IMat2 {
    pub x: IVec2,
    pub y: IVec2,
}

impl IMat2 {
    pub fn new(x: IVec2, y: IVec2) -> IMat2 {
        IMat2 { x, y }
    }

    pub fn determinant(self) -> i32 {
        self.x.x * self.y.y - self.x.y * self.y.x
    }
}

impl Mul for IMat2 {
    type Output = IMat2;
    fn mul(self, rhs: IMat2) -> IMat2 {
        IMat2::new(
            self * IVec2::new(rhs.x.x, rhs.x.y),
            self * IVec2::new(rhs.y.x, rhs.y.y),
        )
    }
}

impl Mul<IVec2> for IMat2 {
    type Output = IVec2;
    fn mul(self, rhs: IVec2) -> IVec2 {
        IVec2::new(
            self.x.x * rhs.x + self.y.x * rhs.y,
            self.x.y * rhs.x + self.y.y * rhs.y,
        )
    }
}
