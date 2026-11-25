use crate::clip::{ClipVertex, clip_triangle};
use crate::{FragmentShader, Framebuffer, Mesh, Vec3, Vec4, VertexShader};

impl Framebuffer {
    fn draw_clip_triangle(&mut self, clip: [Vec4; 3], frag: &impl FragmentShader) {
        let mut vs = [ClipVertex::default(); 9];
        let n = clip_triangle(&clip, &mut vs);

        if n < 3 {
            return;
        }

        for i in 1..n.saturating_sub(1) {
            self.triangle([vs[0], vs[i], vs[i + 1]], frag);
        }
    }

    pub fn render_mesh(
        &mut self,
        mesh: &Mesh,
        vert: &impl VertexShader,
        frag: &impl FragmentShader,
    ) {
        let clipped = mesh
            .vertices
            .iter()
            .map(|&v| vert.transform(v))
            .collect::<Vec<_>>();

        for indices in mesh.indices.iter().copied() {
            self.draw_clip_triangle(indices.map(|i| clipped[i]), frag);
        }
    }

    pub fn draw_triangle(
        &mut self,
        vertices: [Vec3; 3],
        vert: &impl VertexShader,
        frag: &impl FragmentShader,
    ) {
        self.draw_clip_triangle(vertices.map(|v| vert.transform(v)), frag);
    }
}
