use std::default::Default;
use std::ops::{Add, Mul, Sub};

#[derive(Default, Clone, Copy)]
pub struct Empty;
impl Add for Empty {
    type Output = Self;
    fn add(self, _: Self) -> Self {
        Self
    }
}
impl Sub for Empty {
    type Output = Self;
    fn sub(self, _: Self) -> Self {
        Self
    }
}
impl Mul<f32> for Empty {
    type Output = Self;
    fn mul(self, _: f32) -> Self {
        Self
    }
}
