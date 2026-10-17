mod ppm;
use rolivec::{
    Color, Fragment, FragmentShader, Framebuffer, Mat3, Mat4, Mesh, MeshError, Vec3, Vec4, VertexShader
};

use std::io::Result;
use std::f32::{self, consts::PI};

static WIDTH: usize = 1600;
static HEIGHT: usize = 900;

fn main() -> Result<()> {
    ppm::reset();

    let mut fb = Framebuffer::new(WIDTH, HEIGHT);
    let mesh = cube().unwrap();

    let mut vert = Transform {
        model: Mat4::empty(),
        view: Mat4::new(
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(0.0, 0.0, -3.0, 1.0),
        ),
        projection: Mat4::projection(PI / 2.0, WIDTH as f32 / HEIGHT as f32, 0.1, 100.0),
    };
    let frag = Shade;

    const fps: f32 = 60.0;
    const d_rad: f32 = f32::consts::PI / 2.0;
    const dt: f32 = 1.0 / fps;
    let mut deg = 0.0;

    while deg < 4.0 * f32::consts::PI {
        let rot_xz = Mat4::new(
            Vec4::new(deg.cos(), 0.0, -deg.sin(), 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(deg.sin(), 0.0, deg.cos(), 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        );
        let rot_xy = Mat4::new(
            Vec4::new(deg.cos(), deg.sin(), 0.0, 0.0),
            Vec4::new(-deg.sin(), deg.cos(), 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        );

        vert.model = rot_xz * rot_xy;
        fb.clear(Color::BLACK);
        fb.clear_z();
        fb.render_mesh(&mesh, &vert, &frag);
        deg += dt * d_rad;

        write(&fb);
    }

    Ok(())
}

fn cube() -> std::result::Result<Mesh, MeshError> {
    Mesh::new(
        vec![
            Vec3::new(-1.0, 1.0, 1.0),
            Vec3::new(1.0, 1.0, 1.0),
            Vec3::new(-1.0, -1.0, 1.0),
            Vec3::new(1.0, -1.0, 1.0),
            Vec3::new(-1.0, 1.0, -1.0),
            Vec3::new(1.0, 1.0, -1.0),
            Vec3::new(-1.0, -1.0, -1.0),
            Vec3::new(1.0, -1.0, -1.0),
        ],
        vec![
            (0, 2, 1),
            (1, 2, 3),
            (5, 6, 4),
            (7, 6, 5),
            (0, 1, 5),
            (0, 5, 4),
            (1, 3, 7),
            (1, 7, 5),
            (3, 2, 6),
            (7, 3, 6),
            (4, 6, 2),
            (0, 4, 2),
        ],
    )
}

struct Transform {
    pub model: Mat4,
    pub view: Mat4,
    pub projection: Mat4,
}

impl VertexShader for Transform {
    fn transform(&self, vert: Vec3) -> Vec4 {
        self.projection * (self.view * (self.model * Vec4::new(vert.x, vert.y, vert.z, 1.0)))
    }
}

struct Shade;
impl FragmentShader for Shade {
    fn shade(&self, frag: Fragment) -> Color {
        let r = (frag.bary.x * 255.0).round() as u8;
        let g = (frag.bary.y * 255.0).round() as u8;
        let b = (frag.bary.z * 255.0).round() as u8;
        Color::rgb(r, g, b)
    }
}

fn write(fb: &Framebuffer) -> Result<()> {
    ppm::write(fb, &format!("frame{:03}.ppm", increase_counter()))
}

fn increase_counter() -> usize {
    static mut COUNTER: usize = 0;
    unsafe {
        COUNTER += 1;
        COUNTER
    }
}
