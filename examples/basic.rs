use rolivec::{Color, Framebuffer, Vec2};
use std::io::Result;

mod ppm;

fn main() -> Result<()> {
    ppm::reset()?;

    let mut fb = Framebuffer::new(800, 600);

    fb.clear(Color::BLUE);
    write(&fb)?;

    fb.draw_line(Vec2::new(-1.0, -1.0), Vec2::new(1.0, 1.0), Color::ORANGE);
    write(&fb)?;

    fb.draw_rectangle(Vec2::new(-1.0, -1.0), Vec2::new(-0.5, -0.5), Color::LIME);
    write(&fb)?;

    let lavander = Color {
        a: 100,
        ..Color::LAVENDER
    };

    fb.blend_rectangle(Vec2::new(-0.25, -0.25), Vec2::new(-0.75, -0.75), lavander);
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
