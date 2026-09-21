use crate::{Color, Framebuffer, Vec2};

impl Framebuffer {
    pub fn set_pixel(&mut self, p: Vec2<i32>, color: Color) {
        if let Some(pixel) = self.pixel_mut(p) {
            *pixel = color
        }
    }

    pub fn blend_pixel(&mut self, p: Vec2<i32>, color: Color) {
        if let Some(dst) = self.pixel_mut(p) {
            *dst = color.blend_over(*dst);
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

    pub fn blend_range(&mut self, a: Vec2<i32>, b: Vec2<i32>, color: Color) {
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

    pub fn draw_line(&mut self, a: Vec2<f32>, b: Vec2<f32>, color: Color) {
        self.line(a, b, color, Framebuffer::set_pixel);
    }

    pub fn blend_line(&mut self, a: Vec2<f32>, b: Vec2<f32>, color: Color) {
        if color.a == 0 {
            return;
        }

        let pixel_func = match color.a {
            255 => Framebuffer::set_pixel,
            _ => Framebuffer::blend_pixel,
        };

        self.line(a, b, color, pixel_func);
    }

    pub(crate) fn line<F>(&mut self, a: Vec2<f32>, b: Vec2<f32>, color: Color, pixel_func: F)
    where
        F: Fn(&mut Framebuffer, Vec2<i32>, Color),
    {
        let a_ndc = self.screen(a);
        let b_ndc = self.screen(b);
        let dx = (b_ndc.x - a_ndc.x) as f32;
        let dy = (b_ndc.y - a_ndc.y) as f32;

        let step_x: f32;
        let step_y: f32;

        if dx.abs() > dy.abs() {
            step_x = dx / dx;
            step_y = dy / dx;
        } else {
            step_x = dx / dy;
            step_y = dy / dy;
        }

        let mut x = a_ndc.x as f32;
        let mut y = a_ndc.y as f32;

        for _ in 0..dx.abs().max(dy.abs()) as usize {
            let p = Vec2::new(x.round() as i32, y.round() as i32);
            pixel_func(self, p, color);

            x += step_x;
            y += step_y;
        }

        pixel_func(self, b_ndc, color);
    }

    pub fn draw_rectangle(&mut self, a: Vec2<f32>, b: Vec2<f32>, color: Color) {
        self.rectangle(a, b, color, Framebuffer::set_range);
    }

    pub fn blend_rectangle(&mut self, a: Vec2<f32>, b: Vec2<f32>, color: Color) {
        if color.a == 0 {
            return;
        }

        let range_func = match color.a {
            255 => Framebuffer::set_range,
            _ => Framebuffer::blend_range,
        };

        self.rectangle(a, b, color, range_func);
    }

    pub(crate) fn rectangle<F>(&mut self, a: Vec2<f32>, b: Vec2<f32>, color: Color, range_func: F)
    where
        F: Fn(&mut Framebuffer, Vec2<i32>, Vec2<i32>, Color),
    {
        let a_ndc = self.screen(a);
        let b_ndc = self.screen(b);

        let min_x = a_ndc.x.min(b_ndc.x);
        let max_x = a_ndc.x.max(b_ndc.x);
        let min_y = a_ndc.y.min(b_ndc.y);
        let max_y = a_ndc.y.max(b_ndc.y);

        let x_limit = (self.width - 1) as i32;
        let y_limit = (self.height - 1) as i32;

        if max_x < 0 || max_y < 0 || min_x > x_limit || min_y > y_limit {
            return;
        }

        let left = min_x.clamp(0, x_limit);
        let right = max_x.clamp(0, x_limit);
        let top = min_y.clamp(0, y_limit);
        let bottom = max_y.clamp(0, y_limit);

        if left == 0 && right == x_limit {
            range_func(self, Vec2::new(left, top), Vec2::new(right, bottom), color);
            return;
        }

        for y in top..=bottom {
            range_func(self, Vec2::new(left, y), Vec2::new(right, y), color);
        }
    }
}
