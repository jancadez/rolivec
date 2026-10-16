use crate::{Color, Framebuffer, IVec2, Vec2};

impl Framebuffer {
    pub fn set_line(&mut self, a: Vec2, b: Vec2, color: Color) {
        self.line(a, b, color, Framebuffer::set_pixel);
    }

    pub fn draw_line(&mut self, a: Vec2, b: Vec2, color: Color) {
        if color.a == 0 {
            return;
        }

        let pixel_func = match color.a {
            255 => Framebuffer::set_pixel,
            _ => Framebuffer::blend_pixel,
        };

        self.line(a, b, color, pixel_func);
    }

    pub(crate) fn line<F>(&mut self, a: Vec2, b: Vec2, color: Color, pixel_func: F)
    where
        F: Fn(&mut Framebuffer, IVec2, Color),
    {
        let a_scr = self.screen(a);
        let b_scr = self.screen(b);
        let dx = (b_scr.x - a_scr.x) as f32;
        let dy = (b_scr.y - a_scr.y) as f32;

        let step_x: f32;
        let step_y: f32;

        if dx.abs() > dy.abs() {
            step_x = dx / dx;
            step_y = dy / dx;
        } else {
            step_x = dx / dy;
            step_y = dy / dy;
        }

        let mut x = a_scr.x as f32;
        let mut y = a_scr.y as f32;

        for _ in 0..dx.abs().max(dy.abs()) as usize {
            let p = IVec2::new(x.round() as i32, y.round() as i32);
            pixel_func(self, p, color);

            x += step_x;
            y += step_y;
        }

        pixel_func(self, a_scr, color);
    }

    pub fn set_rectangle(&mut self, a: Vec2, b: Vec2, color: Color) {
        self.rectangle(a, b, color, Framebuffer::set_range);
    }

    pub fn draw_rectangle(&mut self, a: Vec2, b: Vec2, color: Color) {
        if color.a == 0 {
            return;
        }

        let range_func = match color.a {
            255 => Framebuffer::set_range,
            _ => Framebuffer::blend_range,
        };

        self.rectangle(a, b, color, range_func);
    }

    pub(crate) fn rectangle<F>(&mut self, a: Vec2, b: Vec2, color: Color, range_func: F)
    where
        F: Fn(&mut Framebuffer, IVec2, IVec2, Color),
    {
        let a_scr = self.screen(a);
        let b_scr = self.screen(b);

        let min_x = a_scr.x.min(b_scr.x);
        let max_x = a_scr.x.max(b_scr.x);
        let min_y = a_scr.y.min(b_scr.y);
        let max_y = a_scr.y.max(b_scr.y);

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
            range_func(
                self,
                IVec2::new(left, top),
                IVec2::new(right, bottom),
                color,
            );
            return;
        }

        for y in top..=bottom {
            range_func(self, IVec2::new(left, y), IVec2::new(right, y), color);
        }
    }
}
