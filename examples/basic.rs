use rolivec::{Color, Fragment, FragmentShader, Framebuffer, Vec2, Vec3, Vec4, VertexShader};
use std::io::Result;

mod ppm;

struct Transform;
impl VertexShader for Transform {
    fn transform(&self, vert: Vec3) -> Vec4 {
        Vec4::new(vert.x, vert.y, vert.z, 1.0)
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

    let mut fb = Framebuffer::new(800, 600);

    fb.clear(Color::BLUE);
    write(&fb)?;

    fb.set_line(Vec2::new(-1.0, -1.0), Vec2::new(1.0, 1.0), Color::ORANGE);
    write(&fb)?;

    fb.set_rectangle(Vec2::new(-1.0, -1.0), Vec2::new(-0.5, -0.5), Color::LIME);
    write(&fb)?;

    let lavender = Color {
        a: 100,
        ..Color::LAVENDER
    };

    fb.draw_rectangle(Vec2::new(-0.25, -0.25), Vec2::new(-0.75, -0.75), lavender);
    write(&fb)?;

    let vert = Transform;
    let frag = Shade;

    fb.triangle(
        [
            Vec3::new(-1.0, -1.0, 1.0),
            Vec3::new(1.0, -1.0, 1.0),
            Vec3::new(0.0, 1.0, 1.0),
        ],
        &vert,
        &frag,
    );
    write(&fb)?;

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
