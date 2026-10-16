use crate::{Color, Framebuffer, IVec2};

impl Framebuffer {
    pub fn set_pixel(&mut self, p: IVec2, color: Color) {
        if let Some(pixel) = self.pixel_mut(p) {
            *pixel = color
        }
    }

    pub fn blend_pixel(&mut self, p: IVec2, color: Color) {
        if let Some(dst) = self.pixel_mut(p) {
            *dst = color.blend_over(*dst);
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

    pub fn blend_range(&mut self, a: IVec2, b: IVec2, color: Color) {
        let Some(start) = self.map_idx(a) else {
            return;
        };

        let Some(end) = self.map_idx(b) else {
            return;
        };

        let pixels = self.pixels_mut();
        for pixel in &mut pixels[start..=end] {
            *pixel = color.blend_over(*pixel);
        }
    }
}
