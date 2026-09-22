use crate::Vec3;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Mat3<T> {
    pub x: Vec3<T>,
    pub y: Vec3<T>,
    pub z: Vec3<T>,
}

impl<T> Mat3<T>
where
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
    T: Copy,
{
    pub fn new(x: Vec3<T>, y: Vec3<T>, z: Vec3<T>) -> Self {
        Self { x, y, z }
    }

    pub fn determinant(&self) -> T {
        self.x.x * self.y.y * self.z.z
            + self.y.x * self.z.y * self.x.z
            + self.z.x * self.x.y * self.y.z
            - self.z.x * self.y.y * self.x.z
            - self.y.x * self.x.y * self.z.z
            - self.x.x * self.z.y * self.y.z
    }
}
