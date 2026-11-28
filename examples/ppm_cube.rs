mod ppm;
use rolivec::color::Color;
use rolivec::framebuffer::{BarycentricShader, Framebuffer};
use rolivec::math::{Mat4, Vec3};
use rolivec::mesh::Mesh;

use std::f32::{self};
use std::io::Result;

static WIDTH: usize = 1600;
static HEIGHT: usize = 900;

fn main() -> Result<()> {
    ppm::reset()?;

    let mut fb = Framebuffer::new(WIDTH, HEIGHT);
    let mesh = cube();

    let view_proj = Mat4::projection(
        60_f32.to_radians(),
        WIDTH as f32 / HEIGHT as f32,
        0.1,
        100.0,
    ) * Mat4::translate(0.0, 0.0, -4.0);

    let mut shader = BarycentricShader::new(Mat4::identity(), view_proj);

    const FPS: f32 = 60.0;
    const D_RAD: f32 = f32::consts::PI / 2.0;
    const DT: f32 = 1.0 / FPS;
    let mut deg = 0.0;

    while deg < 4.0 * f32::consts::PI {
        shader.mvp.set_model(Mat4::rot_y(deg) * Mat4::rot_z(deg));
        fb.clear_all(Color::BLACK);
        fb.render_mesh(&mesh, &shader);
        deg += DT * D_RAD;

        write(&fb)?;
    }

    Ok(())
}

fn cube() -> Mesh<Vec3> {
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
    .unwrap()
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
