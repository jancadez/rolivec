use rolivec::{
    Color, Fragment, FragmentShader, Framebuffer, Mat3, Mat4, Mesh, Vec3, Vec4, VertexShader,
};
use std::{f32, io::Result};

mod ppm;

static WIDTH: usize = 1600;
static HEIGHT: usize = 900;

struct Transform {
    pub dxz: f32,
    pub dxy: f32,
}

impl VertexShader for Transform {
    fn transform(&self, vert: Vec3) -> Vec4 {
        let cos_xz = self.dxz.cos();
        let sin_xz = self.dxz.sin();
        let rot_xz = Mat3::new(
            Vec3::new(cos_xz, 0.0, -sin_xz),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(sin_xz, 0.0, cos_xz),
        );

        let cos_xy = self.dxy.cos();
        let sin_xy = self.dxy.sin();
        let rot_xy = Mat3::new(
            Vec3::new(cos_xy, sin_xy, 0.0),
            Vec3::new(-sin_xy, cos_xy, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        );

        let mut v = rot_xz * (rot_xy * vert);
        v.z -= 3.0;

        let proj_matrix = Mat4::projection(
            90_f32.to_radians(),
            WIDTH as f32 / HEIGHT as f32,
            0.1,
            100.0,
        );

        proj_matrix * Vec4::new(v.x, v.y, v.z, 1.0)
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

fn main() -> Result<()> {
    ppm::reset()?;

    let mut fb = Framebuffer::new(WIDTH, HEIGHT);

    let mesh = Mesh::new(
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
    .unwrap();

    let mut vert = Transform { dxz: 0.0, dxy: 0.0 };
    let frag = Shade;

    const fps: f32 = 60.0;
    const d_rad: f32 = f32::consts::PI / 2.0;
    const dt: f32 = 1.0 / fps;

    while vert.dxz < 1.25 * f32::consts::PI {
        fb.clear(Color::BLACK);
        fb.clear_z();
        fb.render_mesh(&mesh, &vert, &frag);
        write(&fb)?;
        vert.dxz += dt * d_rad;
    }

    while vert.dxy < 2.0 * f32::consts::PI {
        fb.clear(Color::BLACK);
        fb.clear_z();
        fb.render_mesh(&mesh, &vert, &frag);
        write(&fb)?;
        vert.dxy += dt * d_rad;
    }

    Ok(())
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
