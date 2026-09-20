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
