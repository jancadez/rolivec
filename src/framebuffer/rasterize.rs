use crate::{Color, Framebuffer, IMat2, IVec2, Mesh, Vec2, Vec3, Vec4};
#[derive(Clone, Copy)]
pub struct Fragment {
    pub pos: Vec3,
    pub bary: Vec3,
}

pub trait VertexShader {
    fn transform(&self, vert: Vec3) -> Vec4;
}

pub trait FragmentShader {
    fn shade(&self, frag: Fragment) -> Color;
}

impl Framebuffer {
    pub fn render_mesh(
        &mut self,
        mesh: &Mesh,
        vert: &impl VertexShader,
        frag: &impl FragmentShader,
    ) {
        let clip_vertices = mesh
            .vertices
            .iter()
            .map(|&v| vert.transform(v))
            .collect::<Vec<_>>();
        for (a, b, c) in mesh.indices.iter().copied() {
            self.rasterize_triangle([clip_vertices[a], clip_vertices[b], clip_vertices[c]], frag);
        }
    }

    pub fn triangle(
        &mut self,
        vertices: [Vec3; 3],
        vert: &impl VertexShader,
        frag: &impl FragmentShader,
    ) {
        self.rasterize_triangle(
            [
                vert.transform(vertices[0]),
                vert.transform(vertices[1]),
                vert.transform(vertices[2]),
            ],
            frag,
        )
    }

    pub(crate) fn rasterize_triangle(
        &mut self,
        clip_vertices: [Vec4; 3],
        frag: &impl FragmentShader,
    ) {
        for v in clip_vertices {
            if v.x < -v.w || v.x > v.w {
                return;
            }
            if v.y < -v.w || v.y > v.w {
                return;
            }
            if v.z < -v.w || v.z > v.w {
                return;
            }
        }

        let a = clip_vertices[0].project();
        let b = clip_vertices[1].project();
        let c = clip_vertices[2].project();

        let a_scr = self.screen(Vec2::new(a.x, a.y));
        let b_scr = self.screen(Vec2::new(b.x, b.y));
        let c_scr = self.screen(Vec2::new(c.x, c.y));

        let (start, end) = self.bounding_box(a_scr, b_scr, c_scr);

        let triangle_det = IMat2::new(b_scr - a_scr, c_scr - a_scr).determinant();
        if triangle_det == 0 {
            return;
        }

        let inv_det = 1.0 / triangle_det as f32;
        if triangle_det.signum() == 1 {
            return;
        }

        for x in start.x..=end.x {
            for y in start.y..=end.y {
                let p = IVec2::new(x, y);

                let e_ab = IMat2::new(b_scr - a_scr, p - a_scr).determinant();
                let e_bc = IMat2::new(c_scr - b_scr, p - b_scr).determinant();
                let e_ca = IMat2::new(a_scr - c_scr, p - c_scr).determinant();

                if -e_ab < 0 || -e_bc < 0 || -e_ca < 0 {
                    continue;
                }

                let wa = e_bc as f32 * inv_det;
                let wb = e_ca as f32 * inv_det;
                let wc = e_ab as f32 * inv_det;

                let p_ndc = self.screen_to_ndc(p);
                let z = a.z * wa + b.z * wb + c.z * wc;
                let Some(z_buf) = self.z_mut(p) else {
                    continue;
                };

                if z < *z_buf {
                    continue;
                } else {
                    *z_buf = z;
                }

                let fragment = Fragment {
                    pos: Vec3::new(p_ndc.x, p_ndc.y, z),
                    bary: Vec3::new(wa, wb, wc),
                };

                let color = frag.shade(fragment);

                self.set_pixel(p, color);
            }
        }
    }
}
