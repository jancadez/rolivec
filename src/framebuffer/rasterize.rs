use crate::{
    BackfaceCulling, Fragment, FragmentShader, Framebuffer, I64Mat2, I64Vec2, IVec2, Mesh, Vec2,
    Vec3, Vec4, VertexShader,
};

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
        for verts in mesh.indices.iter().copied() {
            self.rasterize_triangle(verts.map(|i| clip_vertices[i]), frag);
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

        let a_w = clip_vertices[0].w;
        let b_w = clip_vertices[1].w;
        let c_w = clip_vertices[2].w;

        let a_w_inv = 1.0 / a_w;
        let b_w_inv = 1.0 / b_w;
        let c_w_inv = 1.0 / c_w;

        let a_scr = self.screen(Vec2::new(a.x, a.y));
        let b_scr = self.screen(Vec2::new(b.x, b.y));
        let c_scr = self.screen(Vec2::new(c.x, c.y));

        let a_scrf = self.screen_fine(Vec2::new(a.x, a.y));
        let b_scrf = self.screen_fine(Vec2::new(b.x, b.y));
        let c_scrf = self.screen_fine(Vec2::new(c.x, c.y));

        let (start, end) = self.bounding_box(a_scr, b_scr, c_scr);

        let triangle_det = I64Mat2::new(b_scrf - a_scrf, c_scrf - a_scrf).determinant();
        let det_sign;

        match triangle_det.signum() {
            -1 => {
                if self.culling == BackfaceCulling::CcwCulling {
                    return;
                }

                det_sign = -1;
            }
            1 => {
                if self.culling == BackfaceCulling::CwCulling {
                    return;
                }

                det_sign = 1;
            }
            _ => return,
        }

        let inv_det = 1.0 / (triangle_det * det_sign) as f32;

        let startf = self.to_fine(start);

        let mut base_e_ab = I64Mat2::new(b_scrf - a_scrf, startf - a_scrf).determinant() * det_sign;
        let mut base_e_bc = I64Mat2::new(c_scrf - b_scrf, startf - b_scrf).determinant() * det_sign;
        let mut base_e_ca = I64Mat2::new(a_scrf - c_scrf, startf - c_scrf).determinant() * det_sign;

        let ab_step = I64Vec2::new(a_scrf.y - b_scrf.y, b_scrf.x - a_scrf.x)
            * Framebuffer::FINE_GRID_SCALE as i64
            * det_sign;
        let bc_step = I64Vec2::new(b_scrf.y - c_scrf.y, c_scrf.x - b_scrf.x)
            * Framebuffer::FINE_GRID_SCALE as i64
            * det_sign;
        let ca_step = I64Vec2::new(c_scrf.y - a_scrf.y, a_scrf.x - c_scrf.x)
            * Framebuffer::FINE_GRID_SCALE as i64
            * det_sign;

        for y in start.y..=end.y {
            let mut e_ab = base_e_ab - ab_step.x;
            let mut e_bc = base_e_bc - bc_step.x;
            let mut e_ca = base_e_ca - ca_step.x;

            for x in start.x..=end.x {
                e_ab += ab_step.x;
                e_bc += bc_step.x;
                e_ca += ca_step.x;

                if e_ab < 0 || e_bc < 0 || e_ca < 0 {
                    continue;
                }

                let wa_scr = e_bc as f32 * inv_det;
                let wb_scr = e_ca as f32 * inv_det;
                let wc_scr = e_ab as f32 * inv_det;

                let wa = wa_scr * a_w_inv;
                let wb = wb_scr * b_w_inv;
                let wc = wc_scr * c_w_inv;

                let w_sum = wa + wb + wc;
                let w_sum_inv = 1.0 / w_sum;

                let p = IVec2::new(x, y);
                let p_ndc = self.screen_to_ndc(p);

                let z = a.z * wa_scr + b.z * wb_scr + c.z * wc_scr;
                let Some(z_buf) = self.z_mut(p) else {
                    continue;
                };

                if z > *z_buf {
                    continue;
                }

                let fragment = Fragment {
                    pos: Vec3::new(p_ndc.x, p_ndc.y, z),
                    bary: Vec3::new(wa * w_sum_inv, wb * w_sum_inv, wc * w_sum_inv),
                };

                if let Some(color) = frag.shade(fragment) {
                    self.set_pixel(p, color);
                    *z_buf = z;
                };
            }

            base_e_ab += ab_step.y;
            base_e_bc += bc_step.y;
            base_e_ca += ca_step.y;
        }
    }
}
