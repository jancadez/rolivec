mod ppm;
use rolivec::{
    Color, Fragment, FragmentShader, Framebuffer, Mat3, Mat4, Mesh, MeshError, Mvp, Vec3,
    Vec4, VertexShader,
};

use std::f32::{self, consts::PI};
use std::io::Result;

static WIDTH: usize = 1600;
static HEIGHT: usize = 900;

fn main() -> Result<()> {
    ppm::reset();

    let mut fb = Framebuffer::new(WIDTH, HEIGHT);
    let mesh = cube().unwrap();

    let mut vert = Mvp::new(
        Mat4::identity(),
        Mat4::translate(0.0, 0.0, -2.0),
        Mat4::projection(90_f32.to_radians(), WIDTH as f32 / HEIGHT as f32, 0.1, 100.0),
    );
    let frag = Shade;

    const fps: f32 = 60.0;
    const D_RAD: f32 = f32::consts::PI / 2.0;
    const dt: f32 = 1.0 / fps;
    let mut deg = 0.0;

    while deg < 4.0 * f32::consts::PI {
        vert.set_model(Mat4::rot_y(deg) * Mat4::rot_z(deg));
        fb.clear_all(Color::BLACK);
        fb.render_mesh(&mesh, &vert, &frag);
        deg += dt * D_RAD;

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
            [0, 2, 1],
            [1, 2, 3],
            [5, 6, 4],
            [7, 6, 5],
            [0, 1, 5],
            [0, 5, 4],
            [1, 3, 7],
            [1, 7, 5],
            [3, 2, 6],
            [7, 3, 6],
            [4, 6, 2],
            [0, 4, 2],
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
    fn shade(&self, frag: Fragment) -> Option<Color> {
        let r = (frag.bary.x * 255.0).round() as u8;
        let g = (frag.bary.y * 255.0).round() as u8;
        let b = (frag.bary.z * 255.0).round() as u8;
        Some(Color::rgb(r, g, b))
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
