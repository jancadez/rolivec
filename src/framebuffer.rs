use crate::{Color, Vec2};

#[derive(Debug)]
pub struct Framebuffer {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) pixels: Box<[Color]>,
}

impl Framebuffer {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn pixels(&self) -> &[Color] {
        &self.pixels
    }

    pub(crate) fn pixels_mut(&mut self) -> &mut [Color] {
        &mut self.pixels
    }

    pub fn new(width: usize, height: usize) -> Self {
        let pixels = width.checked_mul(height).expect("Framebuffer too large");

        if pixels == 0 {
            panic!("Framebuffer width and height can't be 0");
        }

        Self {
            width: width,
            height: height,
            pixels: vec![Color::rgb(0, 0, 0); width * height].into_boxed_slice(),
        }
    }

    pub(crate) fn map_idx(&self, p: Vec2<i32>) -> Option<usize> {
        let inside_x = (0..self.width as i32).contains(&p.x);
        let inside_y = (0..self.height as i32).contains(&p.y);
        (inside_x && inside_y).then(|| p.y as usize * self.width + p.x as usize)
    }

    pub(crate) fn screen(&self, p: Vec2<f32>) -> Vec2<i32> {
        let x = ((p.x + 1.0) / 2.0 * (self.width - 1) as f32).round();
        let y = ((1.0 - (p.y + 1.0) / 2.0) * (self.height - 1) as f32).round();
        Vec2::new(x as i32, y as i32)
    }

    pub(crate) fn pixel_mut(&mut self, p: Vec2<i32>) -> Option<&mut Color> {
        self.map_idx(p).map(|idx| &mut self.pixels[idx])
    }

    pub fn clear(&mut self, color: Color) {
        (&mut *self.pixels).fill(color);
    }
}
