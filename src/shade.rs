use crate::color::Color;
use crate::math::{Vec3, Vec4};
use std::default::Default;
use std::ops::{Add, Mul, Sub};

mod bary;
pub use bary::BarycentricShader;
mod empty;
pub use empty::Empty;
mod mvp;
pub use mvp::Mvp;

#[derive(Clone, Copy)]
pub struct Fragment {
    pub pos: Vec3,
    pub bary: Vec3,
}

pub trait Vary:
    Copy + Default + Add<Output = Self> + Sub<Output = Self> + Mul<f32, Output = Self>
{
}

impl<T> Vary for T where T: Copy + Default + Add<Output = T> + Sub<Output = T> + Mul<f32, Output = T>
{}

pub trait Shader {
    type Vertex;
    type Varying: Vary;

    fn vertex(&self, v: &Self::Vertex) -> (Vec4, Self::Varying);
    fn fragment(&self, v: Self::Varying, frag: Fragment) -> Option<Color>;
}
