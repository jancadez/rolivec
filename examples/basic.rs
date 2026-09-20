use rolivec::{Color, Framebuffer, Vec2};
use std::io::Result;

mod ppm;

fn main() -> Result<()> {
    ppm::reset()?;

    let mut fb = Framebuffer::new(800, 600);

    fb.clear(Color::BLUE);
    write(&fb)?;

    fb.draw_line(Vec2::new(0, 0), Vec2::new(800, 600), Color::ORANGE);
    write(&fb)?;

    fb.draw_rectangle(Vec2::new(0, 0), Vec2::new(120, 90), Color::LIME);
    write(&fb)?;

    let mut lavander = Color::LAVENDER;
    lavander.a = 100;
    for x in 100..=200 {
        for y in 60..=120 {
            fb.blend_pixel(Vec2::new(x, y), lavander);
        }
    }
    write(&fb)?;

    Ok(())
}

fn write(fb: &Framebuffer) -> Result<()> {
    ppm::write(fb, &format!("frame{}.ppm", increase_counter()))
}

fn increase_counter() -> usize {
    static mut COUNTER: usize = 0;
    unsafe {
        COUNTER += 1;
        COUNTER
    }
}
