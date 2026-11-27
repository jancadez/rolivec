use crate::clip::{ClipVertex, clip_triangle};
use crate::{Color, Framebuffer, Mesh, RenderingMode, Shader, Vec2, Vec4};

impl Framebuffer {
    fn draw_clip_triangle<S: Shader>(
        &mut self,
        clip: [Vec4; 3],
        vary: [S::Varying; 3],
        shader: &S,
    ) {
        let mut vs = [ClipVertex::default(); 9];
        let n = clip_triangle(&clip, &vary, &mut vs);

        if n < 3 {
            return;
        }

        if self.mode == RenderingMode::Wireframe {
            let vs = vs.map(|v| v.pos.project());

            for i in 0..n {
                self.line(
                    Vec2::new(vs[i].x, vs[i].y),
                    Vec2::new(vs[(i + 1) % n].x, vs[(i + 1) % n].y),
                    Color::WHITE,
                    255,
                );
            }

            return;
        }

        for i in 1..n - 1 {
            self.triangle([vs[0], vs[i], vs[i + 1]], shader);
        }
    }

    pub fn render_mesh<S: Shader>(&mut self, mesh: &Mesh<S::Vertex>, shader: &S) {
        let clipped = mesh
            .vertices
            .iter()
            .map(|v| shader.vertex(v))
            .collect::<Vec<_>>();

        for indices in mesh.indices.iter() {
            let tri = indices.map(|i| clipped[i]);
            self.draw_clip_triangle(tri.map(|v| v.0), tri.map(|v| v.1), shader);
        }
    }

    pub fn draw_triangle<S: Shader>(&mut self, vertices: [S::Vertex; 3], shader: &S) {
        let vs = vertices.map(|v| shader.vertex(&v));
        self.draw_clip_triangle(vs.map(|v| v.0), vs.map(|v| v.1), shader);
    }
}
