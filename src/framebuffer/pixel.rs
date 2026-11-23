use crate::{Color, Framebuffer, IVec2};

impl Framebuffer {
    pub fn draw_pixel(&mut self, p: IVec2, color: Color, alpha: u8) {
        match alpha {
            0 => return,
            255 => self.set_pixel(p, color),
            _ => self.blend_pixel(p, color, alpha),
        };
    }

    pub fn set_pixel(&mut self, p: IVec2, color: Color) {
        if let Some(pixel) = self.pixel_mut(p) {
            *pixel = color
        }
    }

    pub fn blend_pixel(&mut self, p: IVec2, color: Color, alpha: u8) {
        if let Some(pixel) = self.pixel_mut(p) {
            *pixel = pixel.blend(color, alpha);
        }
    }

    pub fn draw_range(&mut self, a: IVec2, b: IVec2, color: Color, alpha: u8) {
        let Some(start) = self.map_idx(a) else {
            return;
        };

        let Some(end) = self.map_idx(b) else {
            return;
        };

        let pixels = self.pixels_mut();
        for i in start..=end {
            pixels[i] = pixels[i].blend(color, alpha);
        }
    }

    pub fn set_range(&mut self, a: IVec2, b: IVec2, color: Color) {
        let Some(start) = self.map_idx(a) else {
            return;
        };

        let Some(end) = self.map_idx(b) else {
            return;
        };

        let pixels = self.pixels_mut();
        pixels[start..=end].fill(color);
    }
}
