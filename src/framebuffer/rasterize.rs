use crate::clip::ClipVertex;
use crate::{
    BackfaceCulling, Fragment, FragmentShader, Framebuffer, I64Mat2, I64Vec2, IVec2, Vec2, Vec3,
};

impl Framebuffer {
    pub(crate) fn rasterize_triangle(&mut self, vs: [ClipVertex; 3], frag: &impl FragmentShader) {
        let a = vs[0].pos.project();
        let b = vs[1].pos.project();
        let c = vs[2].pos.project();

        let aw = vs[0].pos.w;
        let bw = vs[1].pos.w;
        let cw = vs[2].pos.w;

        let aw_inv = 1.0 / aw;
        let bw_inv = 1.0 / bw;
        let cw_inv = 1.0 / cw;

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

            let mut entered = false;

            for x in start.x..=end.x {
                e_ab += ab_step.x;
                e_bc += bc_step.x;
                e_ca += ca_step.x;

                if e_ab < 0 || e_bc < 0 || e_ca < 0 {
                    if entered {
                        break;
                    } else {
                        continue;
                    }
                }

                let wa_scr = e_bc as f32 * inv_det;
                let wb_scr = e_ca as f32 * inv_det;
                let wc_scr = e_ab as f32 * inv_det;

                let p = IVec2::new(x, y);
                let Some(idx) = self.map_idx(p) else { return };

                let z = a.z * wa_scr + b.z * wb_scr + c.z * wc_scr;

                if z > self.z_buffer[idx] {
                    continue;
                }

                let p_ndc = self.screen_to_ndc(p);

                let wa = wa_scr * aw_inv;
                let wb = wb_scr * bw_inv;
                let wc = wc_scr * cw_inv;

                let w_sum = wa + wb + wc;
                let w_sum_inv = 1.0 / w_sum;

                let la = wa * w_sum_inv;
                let lb = wb * w_sum_inv;
                let lc = wc * w_sum_inv;

                let bary = vs[0].bary * la + vs[1].bary * lb + vs[2].bary * lc;

                let fragment = Fragment {
                    pos: Vec3::new(p_ndc.x, p_ndc.y, z),
                    bary: bary,
                };

                if let Some(color) = frag.shade(fragment) {
                    self.z_buffer[idx] = z;
                    self.pixels[idx] = color;
                };

                entered = true;
            }

            base_e_ab += ab_step.y;
            base_e_bc += bc_step.y;
            base_e_ca += ca_step.y;
        }
    }
}
