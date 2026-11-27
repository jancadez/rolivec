use crate::clip::ClipVertex;
use crate::{BackfaceCulling, Fragment, Framebuffer, I64Vec2, IVec2, Shader, Vary, Vec2, Vec3};

struct Edge {
    base: i64,
    step: I64Vec2,
}

impl Edge {
    #[inline]
    fn new(p: I64Vec2, q: I64Vec2, start: I64Vec2, sign: i64, scale: i64) -> Edge {
        Edge {
            base: (q - p).det(start - p) * sign,
            step: I64Vec2::new(p.y - q.y, q.x - p.x) * scale * sign,
        }
    }
}

struct TriangleVertex<T> {
    pos: Vec3,
    bary: Vec3,
    vary: T,
    w_inv: f32,
}

impl<T> TriangleVertex<T> {
    fn new(v: ClipVertex<T>) -> TriangleVertex<T> {
        TriangleVertex {
            pos: v.pos.project(),
            bary: v.bary,
            vary: v.vary,
            w_inv: 1.0 / v.pos.w as f32,
        }
    }
}

struct TriangleSetup<T> {
    a: TriangleVertex<T>,
    b: TriangleVertex<T>,
    c: TriangleVertex<T>,
    eab: Edge,
    ebc: Edge,
    eca: Edge,
    inv_det: f32,
    start: IVec2,
    end: IVec2,
}

impl Framebuffer {
    pub(crate) fn triangle<S: Shader>(&mut self, vs: [ClipVertex<S::Varying>; 3], shader: &S) {
        let Some(setup) = self.setup_triangle(vs) else {
            return;
        };

        self.rasterize_triangle(setup, shader);
    }

    fn setup_triangle<T: Vary>(&self, vs: [ClipVertex<T>; 3]) -> Option<TriangleSetup<T>> {
        let a = TriangleVertex::new(vs[0]);
        let b = TriangleVertex::new(vs[1]);
        let c = TriangleVertex::new(vs[2]);

        let a_scr = self.screen_fine(Vec2::new(a.pos.x, a.pos.y));
        let b_scr = self.screen_fine(Vec2::new(b.pos.x, b.pos.y));
        let c_scr = self.screen_fine(Vec2::new(c.pos.x, c.pos.y));

        let det = (b_scr - a_scr).det(c_scr - a_scr);
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

        let (start, end) = self.bounding_box([
            Vec2::new(a.pos.x, a.pos.y),
            Vec2::new(b.pos.x, b.pos.y),
            Vec2::new(c.pos.x, c.pos.y),
        ]);
        let startf = self.to_fine(start);

        let eab = Edge::new(a_scr, b_scr, startf, sign, Framebuffer::FINE_GRID_SCALE);
        let ebc = Edge::new(b_scr, c_scr, startf, sign, Framebuffer::FINE_GRID_SCALE);
        let eca = Edge::new(c_scr, a_scr, startf, sign, Framebuffer::FINE_GRID_SCALE);

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

    fn rasterize_triangle<S: Shader>(&mut self, tri: TriangleSetup<S::Varying>, shader: &S) {
        let mut base_ab = tri.eab.base;
        let mut base_bc = tri.ebc.base;
        let mut base_ca = tri.eca.base;

        for y in tri.start.y..=tri.end.y {
            let mut e_ab = base_ab - tri.eab.step.x;
            let mut e_bc = base_bc - tri.ebc.step.x;
            let mut e_ca = base_ca - tri.eca.step.x;

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
                let vary = tri.a.vary * la + tri.b.vary * lb + tri.c.vary * lc;

                let fragment = Fragment {
                    pos: Vec3::new(p_ndc.x, p_ndc.y, z),
                    bary: bary,
                };

                if let Some(color) = shader.fragment(vary, fragment) {
                    self.z_buffer[idx] = z;
                    self.pixels[idx] = color;
                };
            }

            base_ab += tri.eab.step.y;
            base_bc += tri.ebc.step.y;
            base_ca += tri.eca.step.y;
        }
    }
}
