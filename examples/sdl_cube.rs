use rolivec::{
    Camera, Color, Fragment, FragmentShader, Framebuffer, Mat4, Mesh, MeshError, Mvp, Vec3,
};

use sdl3::{event::Event, keyboard::Scancode};
use sdl3::keyboard::Keycode;
use sdl3::pixels::PixelFormat;
use std::time::{Duration, Instant};

const WIDTH: usize = 1600;
const HEIGHT: usize = 900;

fn main() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rolivec demo", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    sdl_context.mouse().set_relative_mouse_mode(&window, true);

    let mut canvas = window.into_canvas();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormat::RGB24, WIDTH as u32, HEIGHT as u32)
        .unwrap();

    let mut fb = Framebuffer::new(WIDTH, HEIGHT);
    let mesh = cube().unwrap();

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 4.0),
        0.0,
        0.0,
        60_f32.to_radians(),
        WIDTH,
        HEIGHT,
        0.1,
        100.0,
    );

    let mut vert = Mvp::new(
        Mat4::identity(),
        camera.view_proj(),
    );
    let frag = Shade;

    const D_RAD: f32 = std::f32::consts::PI / 2.0;
    let mut dt = 0.0;
    let mut deg = 0.0;

    let mut last_time = Instant::now();
    let mut frame_count: usize = 0;

    const SENS: f32 = 0.0015;

    'running: loop {
        let start = Instant::now();

        i = (i + 1) % 255;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape | Keycode::Q),
                    ..
                } => break 'running,
                Event::MouseMotion { xrel, yrel, .. } => {
                    camera.yaw   += xrel * SENS;
                    camera.pitch  = (camera.pitch + yrel * SENS).clamp(-90_f32.to_radians(), 90_f32.to_radians());
                }
                _ => {}
            }
        }

        let keys = event_pump.keyboard_state();
        let mov_speed = 5.0 * dt;

        if keys.is_scancode_pressed(Scancode::W) {camera.move_forward(mov_speed);}
        if keys.is_scancode_pressed(Scancode::S) {camera.move_forward(-mov_speed);}
        if keys.is_scancode_pressed(Scancode::D) {camera.move_right(mov_speed);}
        if keys.is_scancode_pressed(Scancode::A) {camera.move_right(-mov_speed);}
        if keys.is_scancode_pressed(Scancode::Space) {camera.move_up(mov_speed);}
        if keys.is_scancode_pressed(Scancode::LShift) {camera.move_up(-mov_speed);}

        deg += dt * D_RAD;

        vert.set_view_proj(camera.view_proj());
        vert.set_model(Mat4::rot_y(deg) * Mat4::rot_z(deg));
        fb.clear_all(Color::BLACK);
        fb.render_mesh(&mesh, &vert, &frag);

        texture
            .with_lock(None, |buf: &mut [u8], pitch: usize| {
                fb.rgb24_into(buf, Some(pitch));
            })
            .unwrap();

        canvas.clear();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

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

struct Shade;
impl FragmentShader for Shade {
    fn shade(&self, frag: Fragment) -> Option<Color> {
        let r = (frag.bary.x * 255.0).round() as u8;
        let g = (frag.bary.y * 255.0).round() as u8;
        let b = (frag.bary.z * 255.0).round() as u8;
        Some(Color::rgb(r, g, b))
    }
}
