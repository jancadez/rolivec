use rolivec::camera::Camera;
use rolivec::color::Color;
use rolivec::framebuffer::{Framebuffer, RenderingMode};
use rolivec::math::{Mat4, Vec3, Vec4};
use rolivec::mesh::Mesh;
use rolivec::shade::{Fragment, Mvp, Shader};

use sdl3::keyboard::Keycode;
use sdl3::pixels::PixelFormat;
use sdl3::{event::Event, keyboard::Scancode};
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

    let mut canvas = window.into_canvas();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormat::RGBX32, WIDTH as u32, HEIGHT as u32)
        .unwrap();

    let mut fb = Framebuffer::new(WIDTH, HEIGHT);
    let cube = cube();
    let floor = floor();

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

    let mut cube_shader = CubeShader::new(Mat4::identity(), camera.view_proj());
    let mut floor_shader = FloorShader::new(Mat4::identity(), camera.view_proj());

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
                    keycode: Some(Keycode::Q),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => sdl_context
                    .mouse()
                    .set_relative_mouse_mode(canvas.window(), false),
                Event::KeyDown {
                    keycode: Some(Keycode::M),
                    ..
                } => fb.set_mode(if fb.mode() == RenderingMode::Normal {
                    RenderingMode::Wireframe
                } else {
                    RenderingMode::Normal
                }),
                Event::MouseButtonDown { .. } => sdl_context
                    .mouse()
                    .set_relative_mouse_mode(canvas.window(), true),
                Event::MouseMotion { xrel, yrel, .. }
                    if sdl_context.mouse().relative_mouse_mode(canvas.window()) =>
                {
                    camera.set_yaw(camera.yaw() + xrel * SENS);
                    camera.set_pitch(
                        (camera.pitch() + yrel * SENS)
                            .clamp(-88_f32.to_radians(), 88_f32.to_radians()),
                    );
                }
                _ => {}
            }
        }

        let keys = event_pump.keyboard_state();
        let mov_speed = 5.0 * dt;

        if keys.is_scancode_pressed(Scancode::W) {
            camera.move_forward_fps(mov_speed);
        }
        if keys.is_scancode_pressed(Scancode::S) {
            camera.move_forward_fps(-mov_speed);
        }
        if keys.is_scancode_pressed(Scancode::D) {
            camera.move_right(mov_speed);
        }
        if keys.is_scancode_pressed(Scancode::A) {
            camera.move_right(-mov_speed);
        }
        if keys.is_scancode_pressed(Scancode::Space) {
            camera.move_up(mov_speed);
        }
        if keys.is_scancode_pressed(Scancode::LShift) {
            camera.move_up(-mov_speed);
        }

        deg += dt * D_RAD;

        fb.clear_all(Color::BLACK);

        cube_shader.mvp.set_view_proj(camera.view_proj());
        cube_shader
            .mvp
            .set_model(Mat4::rot_y(deg) * Mat4::rot_z(deg));
        fb.render_mesh(&cube, &cube_shader);

        floor_shader.mvp.set_view_proj(camera.view_proj());
        fb.render_mesh(&floor, &floor_shader);

        texture
            .with_lock(None, |buf: &mut [u8], pitch: usize| {
                fb.rgbx32_into(buf, Some(pitch));
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

fn floor() -> Mesh<(Vec3, Vec3)> {
    Mesh::new(
        vec![
            (Vec3::new(10.0, -2.0, 10.0), Vec3::new(1.0, 1.0, 0.0)),
            (Vec3::new(10.0, -2.0, -10.0), Vec3::new(0.0, 1.0, 0.0)),
            (Vec3::new(-10.0, -2.0, 10.0), Vec3::new(0.0, 1.0, 1.0)),
            (Vec3::new(-10.0, -2.0, -10.0), Vec3::new(1.0, 0.0, 1.0)),
        ],
        vec![[0, 1, 2], [3, 2, 1]],
    )
    .unwrap()
}

pub struct FloorShader {
    pub mvp: Mvp,
}

impl FloorShader {
    pub fn new(model: Mat4, view_proj: Mat4) -> Self {
        Self {
            mvp: Mvp::new(model, view_proj),
        }
    }
}

impl Shader for FloorShader {
    type Vertex = (Vec3, Vec3);
    type Varying = Vec3;

    fn vertex(&self, v: &Self::Vertex) -> (Vec4, Self::Varying) {
        (self.mvp.clip(&v.0), v.1)
    }

    fn fragment(&self, v: Self::Varying, _frag: Fragment) -> Option<Color> {
        Some(Color::vec3(v))
    }
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

pub struct CubeShader {
    pub mvp: Mvp,
}

impl CubeShader {
    pub fn new(model: Mat4, view_proj: Mat4) -> Self {
        Self {
            mvp: Mvp::new(model, view_proj),
        }
    }
}

impl Shader for CubeShader {
    type Vertex = Vec3;
    type Varying = Vec3;

    fn vertex(&self, v: &Self::Vertex) -> (Vec4, Self::Varying) {
        let x = (v.x + 1.0) / 2.0;
        let y = (v.y + 1.0) / 2.0;
        let z = (v.z + 1.0) / 2.0;
        (self.mvp.clip(v), Vec3::new(x, y, z))
    }

    fn fragment(&self, v: Self::Varying, _frag: Fragment) -> Option<Color> {
        Some(Color::vec3(v))
    }
}
