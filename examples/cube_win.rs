use rolivec::{
    Color, Fragment, FragmentShader, Framebuffer, Mat4, Mesh, MeshError, Vec3, Vec4, VertexShader,
};

use minifb::{Window, WindowOptions};
use std::f32::{self, consts::PI};
use std::time::{Duration, Instant};

static WIDTH: usize = 1600;
static HEIGHT: usize = 900;

fn main() {
    let mut window = match Window::new("Test", WIDTH, HEIGHT, WindowOptions::default()) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return;
        }
    };

    let mut fb = Framebuffer::new(WIDTH, HEIGHT);
    let mesh = cube().unwrap();

    let mut vert = Transform {
        model: Mat4::empty(),
        view: Mat4::identity(),
        projection: Mat4::projection(PI / 2.0, WIDTH as f32 / HEIGHT as f32, 0.1, 100.0),
    };
    let frag = Shade;

    const D_RAD: f32 = f32::consts::PI / 2.0;
    let mut dt = 0.0;
    let mut deg = 0.0;
    let trans_z = Mat4::translate(0.0, 0.0, -4.0);

    let mut buffer = vec![0_u32; fb.pixels().len()];

    let mut last_time = Instant::now();
    let mut frame_count: usize = 0;

    while window.is_open() {
        let start = Instant::now();

        deg += dt * D_RAD;

        vert.model = trans_z * Mat4::rot_xz(deg) * Mat4::rot_xy(deg);
        fb.clear(Color::BLACK);
        fb.clear_z();
        fb.render_mesh(&mesh, &vert, &frag);

        for (dst, src) in buffer.iter_mut().zip(fb.pixels().iter()) {
            *dst = u32::from_be_bytes([0, src.r, src.g, src.b]);
        }

        let _ = window.update_with_buffer(&buffer, WIDTH, HEIGHT);

        let elapsed = last_time.elapsed();
        if elapsed >= Duration::from_millis(500) {
            let fps = frame_count as f32 / elapsed.as_secs_f32();
            println!("{:.2}", fps);

            frame_count = 0;
            last_time = Instant::now();
        } else {
            frame_count += 1;
        }

        dt = start.elapsed().as_secs_f32();
    }
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
    fn shade(&self, frag: Fragment) -> Option<Color> {
        let r = (frag.bary.x * 255.0).round() as u8;
        let g = (frag.bary.y * 255.0).round() as u8;
        let b = (frag.bary.z * 255.0).round() as u8;
        Some(Color::rgb(r, g, b))
    }
}
