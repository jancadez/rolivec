use crate::{Color, Mat4, Mvp, Vec3, Vec4};
use std::default::Default;
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy)]
pub struct Fragment {
    pub pos: Vec3,
    pub bary: Vec3,
}

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

pub struct BarycentricShader {
    pub mvp: Mvp,
}

impl BarycentricShader {
    pub fn new(model: Mat4, view_proj: Mat4) -> Self {
        Self {
            mvp: Mvp::new(model, view_proj),
        }
    }
}

impl Shader for BarycentricShader {
    type Vertex = Vec3;
    type Varying = Empty;

    fn vertex(&self, v: &Self::Vertex) -> (Vec4, Self::Varying) {
        (self.mvp.clip(v), Empty)
    }

    fn fragment(&self, _v: Self::Varying, frag: Fragment) -> Option<Color> {
        let r = (255_f32 * frag.bary.x) as u8;
        let g = (255_f32 * frag.bary.y) as u8;
        let b = (255_f32 * frag.bary.z) as u8;
        Some(Color::rgb(r, g, b))
    }
}
