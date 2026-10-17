pub mod draw2d;
pub mod pixel;
pub mod rasterize;

use crate::{Color, IVec2, Vec2};

#[derive(Debug)]
pub struct Framebuffer {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) inv_width: f32,
    pub(crate) inv_height: f32,
    pub(crate) pixels: Box<[Color]>,
    pub(crate) z_buffer: Box<[f32]>,
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

        let denom_x = if width > 1 { (width - 1) as f32 } else { 1.0 };
        let denom_y = if height > 1 { (height - 1) as f32 } else { 1.0 };

        Self {
            width,
            height,
            inv_width: 1.0 / denom_x,
            inv_height: 1.0 / denom_y,
            pixels: vec![Color::rgb(0, 0, 0); pixels].into_boxed_slice(),
            z_buffer: vec![f32::MIN; pixels].into_boxed_slice(),
        }
    }

    pub(crate) fn map_idx(&self, p: IVec2) -> Option<usize> {
        let inside_x = (0..self.width as i32).contains(&p.x);
        let inside_y = (0..self.height as i32).contains(&p.y);
        (inside_x && inside_y).then(|| p.y as usize * self.width + p.x as usize)
    }

    pub(crate) fn screen(&self, p: Vec2) -> IVec2 {
        let x = ((p.x + 1.0) / 2.0 * (self.width - 1) as f32).round() as i32;
        let y = ((1.0 - (p.y + 1.0) / 2.0) * (self.height - 1) as f32).round() as i32;
        IVec2::new(x, y)
    }

    pub(crate) fn screen_to_ndc(&self, p: IVec2) -> Vec2 {
        let x = (p.x as f32 * self.inv_width * 2.0) - 1.0;
        let y = 1.0 - (p.y as f32 * self.inv_height * 2.0);
        Vec2::new(x, y)
    }

    pub(crate) fn bounding_box(&self, a: IVec2, b: IVec2, c: IVec2) -> (IVec2, IVec2) {
        let max_w = self.width as i32 - 1;
        let max_h = self.height as i32 - 1;

        let min_x = a.x.min(b.x.min(c.x)).clamp(0, max_w);
        let min_y = a.y.min(b.y.min(c.y)).clamp(0, max_h);
        let max_x = a.x.max(b.x.max(c.x)).clamp(0, max_w);
        let max_y = a.y.max(b.y.max(c.y)).clamp(0, max_h);

        (IVec2::new(min_x, min_y), IVec2::new(max_x, max_y))
    }

    pub(crate) fn pixel_mut(&mut self, p: IVec2) -> Option<&mut Color> {
        self.map_idx(p).map(|idx| &mut self.pixels[idx])
    }

    pub(crate) fn get_z(&self, p: IVec2) -> Option<f32> {
        self.map_idx(p).map(|idx| self.z_buffer[idx])
    }

    pub(crate) fn z_mut(&mut self, p: IVec2) -> Option<&mut f32> {
        self.map_idx(p).map(|idx| &mut self.z_buffer[idx])
    }

    pub fn clear(&mut self, color: Color) {
        (&mut *self.pixels).fill(color);
    }

    pub fn clear_z(&mut self) {
        (&mut self.z_buffer).fill(f32::MIN);
    }
}
