use crate::{Color, Framebuffer, Mat3, Vec2, Vec3};

#[derive(Clone, Copy)]
pub struct Fragment {
    pos: Vec3<f32>,
    bary: Vec3<f32>,
}

pub trait VertexShader {
    fn transform(&self , verts: Vec3<f32>) -> Vec<f32>;
}

pub trait FragmentShader {
    fn transform(&self , frag: Fragment) -> Color;
}

impl Framebuffer {
    pub fn draw_triangle(
        &mut self,
        vertices: [Vec3<f32>; 3],
        vert: VertexShader,
        frag: FragmentShader,
        color: Color,
    ) {
        self.triangle(vertices, vert, frag, color, Framebuffer::set_pixel);
    }

    pub fn blend_triangle<F, G>(
        &mut self,
        vertices: [Vec3<f32>; 3],
        vert: &impl VertexShader,
        frag: &impl FragmentShader,
        color: Color,
    ) {
        if color.a == 0 {
            return;
        }

        let range_func = match color.a {
            255 => Framebuffer::set_pixel,
            _ => Framebuffer::blend_pixel,
        };

        self.triangle(vertices, vert, frag, color, range_func);
    }

    pub(crate) fn triangle<F>(
        &mut self,
        vertices: [Vec3<f32>; 3],
        vert: VertexShader,
        frag: FragmentShader,
        color: Color,
        range_func: F,
    ) where
        F: Fn(&mut Framebuffer, Vec2<i32>, Color),
    {
        let a = vertices[0];
        let b = vertices[1];
        let c = vertices[2];

        let mat = Mat3::new(
            Vec3::new(a.x, a.y, 1.0),
            Vec3::new(b.x, b.y, 1.0),
            Vec3::new(c.x, c.y, 1.0),
        );

        let bounding_box = self.bounding_box(
            Vec2::new(a.x as i32, a.y as i32),
            Vec2::new(b.x as i32, b.y as i32),
            Vec2::new(c.x as i32, c.y as i32),
        );

        let Some(start) = self.map_idx(bounding_box.0) else {
            return;
        };
        let Some(end) = self.map_idx(bounding_box.1) else {
            return;
        };
    }
}
