use crate::{Color, Framebuffer, Vec2};

impl Framebuffer {
    pub fn set_pixel(&mut self, p: Vec2<i32>, color: Color) {
        if let Some(pixel) = self.pixel_mut(p) {
            *pixel = color
        }
    }

    pub fn set_range(&mut self, a: Vec2<i32>, b: Vec2<i32>, color: Color) {
        let Some(start) = self.map_idx(a) else {
            return;
        };

        let Some(end) = self.map_idx(b) else {
            return;
        };

        let pixels = self.pixels_mut();
        pixels[start..=end].fill(color);
    }

    pub fn blend_pixel(&mut self, p: Vec2<i32>, color: Color) {
        if let Some(dst) = self.pixel_mut(p) {
            let src_a = color.a as u32;
            let dst_a = dst.a as u32;

            let out_a = src_a + dst_a * (255 - src_a) / 255;
            let dst_factor = dst_a * (255 - src_a) / 255;

            let out_r = (color.r as u32 * src_a + dst.r as u32 * dst_factor) / out_a;
            let out_g = (color.g as u32 * src_a + dst.g as u32 * dst_factor) / out_a;
            let out_b = (color.b as u32 * src_a + dst.b as u32 * dst_factor) / out_a;

            *dst = Color::new(out_r as u8, out_g as u8, out_b as u8, out_a as u8);
        }
    }

    pub fn draw_line(&mut self, a: Vec2<i32>, b: Vec2<i32>, color: Color) {
        let dx = (b.x - a.x) as f32;
        let dy = (b.y - a.y) as f32;

        let step_x: f32;
        let step_y: f32;

        if dx.abs() > dy.abs() {
            step_x = dx / dx;
            step_y = dy / dx;
        } else {
            step_x = dx / dy;
            step_y = dy / dy;
        }

        let mut x = a.x as f32;
        let mut y = a.y as f32;

        for _ in 0..dx.abs().max(dy.abs()) as usize {
            let p = Vec2::new(x.round() as i32, y.round() as i32);
            self.set_pixel(p, color);

            x += step_x;
            y += step_y;
        }

        self.set_pixel(b, color);
    }

    pub fn draw_rectangle(&mut self, a: Vec2<i32>, b: Vec2<i32>, color: Color) {
        let min_x = a.x.min(b.x);
        let max_x = a.x.max(b.x);
        let min_y = a.y.min(b.y);
        let max_y = a.y.max(b.y);

        let x_limit = (self.width - 1) as i32;
        let y_limit = (self.height - 1) as i32;

        if max_x < 0 || max_y < 0 || min_x > x_limit || min_y > y_limit {
            return;
        }

        let left = min_x.clamp(0, x_limit);
        let right = max_x.clamp(0, x_limit);
        let top = min_y.clamp(0, y_limit);
        let bottom = max_y.clamp(0, y_limit);

        if left == 0 && right == self.width - 1 {
            self.set_range(Vec2::new(left, top), Vec2::new(right, bottom), color);
            return;
        }

        for y in top..=bottom {
            self.set_range(Vec2::new(left, y), Vec2::new(right, y), color);
        }
    }
}
