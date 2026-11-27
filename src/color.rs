use crate::Vec3;
use std::ops::Mul;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    _padding: u8,
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, scalar: f32) -> Color {
        Color::rgb(
            (self.r as f32 * scalar) as u8,
            (self.g as f32 * scalar) as u8,
            (self.b as f32 * scalar) as u8,
        )
    }
}

#[inline]
fn div255(x: u32) -> u8 {
    let x = x + 128;
    ((x + (x >> 8)) >> 8) as u8
}

#[inline]
fn mix(dst: u8, src: u8, a: u8) -> u8 {
    div255(src as u32 * a as u32 + dst as u32 * (255 - a as u32))
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color {
            r,
            g,
            b,
            _padding: 0,
        }
    }

    pub fn blend(self, src: Color, alpha: u8) -> Color {
        Color {
            r: mix(self.r, src.r, alpha),
            g: mix(self.g, src.g, alpha),
            b: mix(self.b, src.b, alpha),
            _padding: 0,
        }
    }

    pub fn vec3(v: Vec3) -> Color {
        let q = |c: f32| (c.clamp(0.0, 1.0) * 255.0).round() as u8;
        Color::rgb(q(v.x), q(v.y), q(v.z))
    }

    pub const RED: Color = Color::rgb(220, 70, 70);
    pub const GREEN: Color = Color::rgb(75, 190, 110);
    pub const BLUE: Color = Color::rgb(70, 125, 220);

    pub const YELLOW: Color = Color::rgb(235, 195, 75);
    pub const ORANGE: Color = Color::rgb(225, 125, 55);
    pub const PURPLE: Color = Color::rgb(155, 95, 195);
    pub const PINK: Color = Color::rgb(220, 100, 155);

    pub const CYAN: Color = Color::rgb(65, 190, 195);
    pub const TEAL: Color = Color::rgb(55, 155, 145);
    pub const LIME: Color = Color::rgb(145, 195, 70);

    pub const BROWN: Color = Color::rgb(145, 95, 65);
    pub const GOLD: Color = Color::rgb(205, 165, 55);

    pub const NAVY: Color = Color::rgb(45, 65, 115);
    pub const SKY: Color = Color::rgb(100, 175, 220);
    pub const MINT: Color = Color::rgb(105, 195, 160);

    pub const CORAL: Color = Color::rgb(225, 110, 90);
    pub const CRIMSON: Color = Color::rgb(175, 50, 65);
    pub const INDIGO: Color = Color::rgb(85, 75, 165);
    pub const VIOLET: Color = Color::rgb(125, 70, 175);

    pub const OLIVE: Color = Color::rgb(125, 130, 65);
    pub const SALMON: Color = Color::rgb(235, 135, 115);
    pub const PEACH: Color = Color::rgb(240, 175, 125);
    pub const LAVENDER: Color = Color::rgb(165, 145, 205);
    pub const PERIWINKLE: Color = Color::rgb(120, 135, 205);

    pub const FOREST: Color = Color::rgb(45, 120, 75);
    pub const SEAFOAM: Color = Color::rgb(85, 165, 145);

    pub const BLACK: Color = Color::rgb(0, 0, 0);
    pub const DARK_GRAY: Color = Color::rgb(18, 20, 24);
    pub const GRAY: Color = Color::rgb(100, 105, 115);
    pub const LIGHT_GRAY: Color = Color::rgb(170, 175, 185);
    pub const WHITE: Color = Color::rgb(255, 255, 255);
    pub const OFF_WHITE: Color = Color::rgb(238, 240, 242);
}
