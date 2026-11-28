use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::math::{IVec2, Vec2};

impl Framebuffer {
    pub fn line(&mut self, a: Vec2, b: Vec2, color: Color, alpha: u8) {
        let a = self.screen(a);
        let b = self.screen(b);

        let dx = (b.x - a.x) as f32;
        let dy = (b.y - a.y) as f32;

        let steps = dx.abs().max(dy.abs()) as usize;

        if steps == 0 {
            self.draw_pixel(a, color, alpha);
            return;
        }

        let step_x = dx / steps as f32;
        let step_y = dy / steps as f32;

        let mut x = a.x as f32;
        let mut y = a.y as f32;

        for _ in 0..=steps {
            let p = IVec2::new(x.round() as i32, y.round() as i32);
            self.draw_pixel(p, color, alpha);

            x += step_x;
            y += step_y;
        }
    }

    pub fn rectangle(&mut self, a: Vec2, b: Vec2, color: Color, alpha: u8) {
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
            self.draw_range(
                IVec2::new(left, top),
                IVec2::new(right, bottom),
                color,
                alpha,
            );
            return;
        }

        for y in top..=bottom {
            self.draw_range(IVec2::new(left, y), IVec2::new(right, y), color, alpha);
        }
    }
}
