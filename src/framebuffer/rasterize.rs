use crate::clip::ClipVertex;
use crate::{
    BackfaceCulling, Fragment, FragmentShader, Framebuffer, I64Mat2, I64Vec2, IVec2, Vec2, Vec3,
};

struct Edge {
    base: i64,
    step: I64Vec2,
}

impl Edge {
    #[inline]
    fn new(p: I64Vec2, q: I64Vec2, start: I64Vec2, sign: i64, scale: i64) -> Edge {
        Edge {
            base: I64Mat2::new(q - p, start - p).determinant() * sign,
            step: I64Vec2::new(p.y - q.y, q.x - p.x) * scale * sign,
        }
    }
}

struct TriangleVertex {
    pos: Vec3,
    bary: Vec3,
    w_inv: f32,
}

impl TriangleVertex {
    #[inline]
    fn new(v: ClipVertex) -> TriangleVertex {
        TriangleVertex {
            pos: v.pos.project(),
            bary: v.bary,
            w_inv: 1.0 / v.pos.w as f32,
        }
    }
}

struct TriangleSetup {
    a: TriangleVertex,
    b: TriangleVertex,
    c: TriangleVertex,
    eab: Edge,
    ebc: Edge,
    eca: Edge,
    inv_det: f32,
    start: IVec2,
    end: IVec2,
}

impl Framebuffer {
    pub(crate) fn triangle(&mut self, vs: [ClipVertex; 3], frag: &impl FragmentShader) {
        let Some(setup) = self.setup_triangle(vs) else {
            return;
        };

        self.rasterize_triangle(setup, frag);
    }

    fn setup_triangle(&mut self, vs: [ClipVertex; 3]) -> Option<TriangleSetup> {
        let a = TriangleVertex::new(vs[0]);
        let b = TriangleVertex::new(vs[1]);
        let c = TriangleVertex::new(vs[2]);

        let a_scr = self.screen_fine(Vec2::new(a.pos.x, a.pos.y));
        let b_scr = self.screen_fine(Vec2::new(b.pos.x, b.pos.y));
        let c_scr = self.screen_fine(Vec2::new(c.pos.x, c.pos.y));

        let (start, end) = self.bounding_box([
            Vec2::new(a.pos.x, a.pos.y),
            Vec2::new(b.pos.x, b.pos.y),
            Vec2::new(c.pos.x, c.pos.y),
        ]);
        let startf = self.to_fine(start);

        let det = I64Mat2::new(b_scr - a_scr, c_scr - a_scr).determinant();
        let sign;

        match det.signum() {
            -1 => {
                if self.culling == BackfaceCulling::CcwCulling {
                    return None;
                }

                sign = -1;
            }
            1 => {
                if self.culling == BackfaceCulling::CwCulling {
                    return None;
                }

                sign = 1;
            }
            _ => return None,
        }

        let eab = Edge::new(a_scr, b_scr, startf, sign, Framebuffer::FINE_GRID_SCALE as i64);
        let ebc = Edge::new(b_scr, c_scr, startf, sign, Framebuffer::FINE_GRID_SCALE as i64);
        let eca = Edge::new(c_scr, a_scr, startf, sign, Framebuffer::FINE_GRID_SCALE as i64);

        Some(TriangleSetup {
            a,
            b,
            c,
            eab,
            ebc,
            eca,
            inv_det: 1.0 / (det * sign) as f32,
            start,
            end,
        })
    }

    fn rasterize_triangle(&mut self, mut tri: TriangleSetup, frag: &impl FragmentShader) {
        for y in tri.start.y..=tri.end.y {
            let mut e_ab = tri.eab.base - tri.eab.step.x;
            let mut e_bc = tri.ebc.base - tri.ebc.step.x;
            let mut e_ca = tri.eca.base - tri.eca.step.x;

            let mut entered = false;

            for x in tri.start.x..=tri.end.x {
                e_ab += tri.eab.step.x;
                e_bc += tri.ebc.step.x;
                e_ca += tri.eca.step.x;

                if e_ab < 0 || e_bc < 0 || e_ca < 0 {
                    if entered {
                        break;
                    } else {
                        continue;
                    }
                } else {
                    entered = true;
                }

                let wa_scr = e_bc as f32 * tri.inv_det;
                let wb_scr = e_ca as f32 * tri.inv_det;
                let wc_scr = e_ab as f32 * tri.inv_det;

                let p = IVec2::new(x, y);
                let Some(idx) = self.map_idx(p) else { return };

                let z = tri.a.pos.z * wa_scr + tri.b.pos.z * wb_scr + tri.c.pos.z * wc_scr;

                if z > self.z_buffer[idx] {
                    continue;
                }

                let p_ndc = self.screen_to_ndc(p);

                let wa = wa_scr * tri.a.w_inv;
                let wb = wb_scr * tri.b.w_inv;
                let wc = wc_scr * tri.c.w_inv;

                let w_sum = wa + wb + wc;
                let w_sum_inv = 1.0 / w_sum;

                let la = wa * w_sum_inv;
                let lb = wb * w_sum_inv;
                let lc = wc * w_sum_inv;

                let bary = tri.a.bary * la + tri.b.bary * lb + tri.c.bary * lc;

                let fragment = Fragment {
                    pos: Vec3::new(p_ndc.x, p_ndc.y, z),
                    bary: bary,
                };

                if let Some(color) = frag.shade(fragment) {
                    self.z_buffer[idx] = z;
                    self.pixels[idx] = color;
                };
            }

            tri.eab.base += tri.eab.step.y;
            tri.ebc.base += tri.ebc.step.y;
            tri.eca.base += tri.eca.step.y;
        }
    }
}
