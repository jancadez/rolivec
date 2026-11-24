use rolivec::{
    Color, Fragment, FragmentShader, Framebuffer, Mat4, Mesh, MeshError, Vec3, Mvp
};

use minifb::{Window, WindowOptions};
use std::time::{Duration, Instant};

const WIDTH: usize = 1600;
const HEIGHT: usize = 900;

fn main() {
    let mut window = match Window::new("Test", WIDTH, HEIGHT, WindowOptions::default()) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return;
        }
    };

    window.set_target_fps(10000);

    let mut fb = Framebuffer::new(WIDTH, HEIGHT);
    let mesh = cube().unwrap();

    let mut vert = Mvp::new(
        Mat4::identity(),
        Mat4::translate(0.0, 0.0, -4.0),
        Mat4::projection(
            90_f32.to_radians(),
            WIDTH as f32 / HEIGHT as f32,
            0.1,
            100.0,
        ),
    );
    let frag = Shade;

    const D_RAD: f32 = std::f32::consts::PI / 2.0;
    let mut dt = 0.0;
    let mut deg = 0.0;

    let mut buffer = vec![0_u32; WIDTH * HEIGHT];

    let mut last_time = Instant::now();
    let mut frame_count: usize = 0;

    while window.is_open()
        && !(window.is_key_down(minifb::Key::Q) || window.is_key_down(minifb::Key::Escape))
    {
        let start = Instant::now();

        deg += dt * D_RAD;

        vert.set_model(Mat4::rot_y(deg) * Mat4::rot_z(deg));
        fb.clear_all(Color::BLACK);
        fb.render_mesh(&mesh, &vert, &frag);

        for (dst, src) in buffer.iter_mut().zip(fb.pixels().iter()) {
            *dst = u32::from_be_bytes([0, src.r, src.g, src.b]);
        }

        let Ok(_) = window.update_with_buffer(&buffer, WIDTH, HEIGHT) else {
            return;
        };

        let elapsed = last_time.elapsed();
        if elapsed >= Duration::from_millis(500) {
            let fps = frame_count as f32 / elapsed.as_secs_f32();
            println!("{:.2}", fps);

            frame_count = 0;
            last_time = Instant::now();
        }

        frame_count += 1;
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

struct Shade;
impl FragmentShader for Shade {
    fn shade(&self, frag: Fragment) -> Option<Color> {
        let r = (frag.bary.x * 255.0).round() as u8;
        let g = (frag.bary.y * 255.0).round() as u8;
        let b = (frag.bary.z * 255.0).round() as u8;
        Some(Color::rgb(r, g, b))
    }
}
