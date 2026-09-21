#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r, g, b, 255)
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::new(r, g, b, a)
    }

    pub fn blend_over(self, dst: Color) -> Color {
        let src_a = self.a as u32;
        let dst_a = dst.a as u32;

        let out_a = src_a + dst_a * (255 - src_a) / 255;
        let dst_factor = dst_a * (255 - src_a) / 255;

        let out_r = (self.r as u32 * src_a + dst.r as u32 * dst_factor) / out_a;
        let out_g = (self.g as u32 * src_a + dst.g as u32 * dst_factor) / out_a;
        let out_b = (self.b as u32 * src_a + dst.b as u32 * dst_factor) / out_a;

        Color::new(out_r as u8, out_g as u8, out_b as u8, out_a as u8)
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

    pub const BLACK: Color = Color::rgb(18, 20, 24);
    pub const DARK_GRAY: Color = Color::rgb(45, 48, 55);
    pub const GRAY: Color = Color::rgb(100, 105, 115);
    pub const LIGHT_GRAY: Color = Color::rgb(170, 175, 185);
    pub const WHITE: Color = Color::rgb(238, 240, 242);
    pub const OFF_WHITE: Color = Color::rgb(220, 218, 205);
}
