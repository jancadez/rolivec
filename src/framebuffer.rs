pub mod clip;
pub mod draw2d;
pub mod draw3d;
pub mod export;
pub mod pixel;
pub mod rasterize;
pub mod shade;

use crate::{Color, I64Vec2, IVec2, Vec2};

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum BackfaceCulling {
    NoCulling,
    CwCulling,
    CcwCulling,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum RenderingMode {
    Normal,
    Wireframe,
}

#[derive(Debug)]
pub struct Framebuffer {
    width: usize,
    height: usize,
    inv_width: f32,
    inv_height: f32,
    pixels: Box<[Color]>,
    z_buffer: Box<[f32]>,
    culling: BackfaceCulling,
    mode: RenderingMode,
}

impl Framebuffer {
    pub const FINE_GRID_SHIFT: u32 = 4;
    pub const FINE_GRID_SCALE: i64 = 1 << Framebuffer::FINE_GRID_SHIFT;

    pub const TILE_SHIFT: u32 = 5;
    pub const TILE_SIZE: usize = 1 << Framebuffer::TILE_SHIFT;
    pub const TILE_MASK: usize = Framebuffer::TILE_SIZE - 1;

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn pixels(&self) -> &[Color] {
        &self.pixels
    }

    pub(crate) fn pixels_mut(&mut self) -> &mut [Color] {
        &mut self.pixels
    }

    pub fn new(width: usize, height: usize) -> Self {
        let pixels = width.checked_mul(height).expect("Framebuffer too large");

        if pixels == 0 {
            panic!("Framebuffer width and height can't be 0");
        }

        let denom_x = if width > 1 { (width - 1) as f32 } else { 1.0 };
        let denom_y = if height > 1 { (height - 1) as f32 } else { 1.0 };

        Self {
            width,
            height,
            inv_width: 1.0 / denom_x,
            inv_height: 1.0 / denom_y,
            pixels: vec![Color::rgb(0, 0, 0); pixels].into_boxed_slice(),
            z_buffer: vec![f32::MAX; pixels].into_boxed_slice(),
            culling: BackfaceCulling::CwCulling,
            mode: RenderingMode::Normal,
        }
    }

    pub fn set_culling(&mut self, culling: BackfaceCulling) {
        self.culling = culling;
    }

    pub fn mode(&self) -> RenderingMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: RenderingMode) {
        self.mode = mode;
    }

    pub(crate) fn map_idx(&self, p: IVec2) -> Option<usize> {
        let inside_x = (0..self.width as i32).contains(&p.x);
        let inside_y = (0..self.height as i32).contains(&p.y);
        (inside_x && inside_y).then(|| p.y as usize * self.width + p.x as usize)
    }

    pub(crate) fn screen(&self, p: Vec2) -> IVec2 {
        let x = ((p.x + 1.0) * 0.5 * (self.width - 1) as f32).round() as i32;
        let y = ((1.0 - (p.y + 1.0) * 0.5) * (self.height - 1) as f32).round() as i32;
        IVec2::new(x, y)
    }

    pub(crate) fn screen_fine(&self, p: Vec2) -> I64Vec2 {
        let x =
            ((p.x + 1.0) * 0.5 * ((self.width - 1) * Framebuffer::FINE_GRID_SCALE as usize) as f32)
                .round() as i64;
        let y = ((1.0 - (p.y + 1.0) * 0.5)
            * ((self.height - 1) * Framebuffer::FINE_GRID_SCALE as usize) as f32)
            .round() as i64;
        I64Vec2::new(x, y)
    }

    pub(crate) fn to_fine(&self, p: IVec2) -> I64Vec2 {
        I64Vec2::new(
            p.x as i64 * Framebuffer::FINE_GRID_SCALE,
            p.y as i64 * Framebuffer::FINE_GRID_SCALE,
        )
    }

    pub(crate) fn screen_to_ndc(&self, p: IVec2) -> Vec2 {
        let x = (p.x as f32 * self.inv_width * 2.0) - 1.0;
        let y = 1.0 - (p.y as f32 * self.inv_height * 2.0);
        Vec2::new(x, y)
    }

    pub(crate) fn bounding_box(&self, vs: [Vec2; 3]) -> (IVec2, IVec2) {
        let scr_vs = vs.map(|v| self.screen(Vec2::new(v.x, v.y)));

        let scr_min = IVec2::new(0, 0);
        let scr_max = IVec2::new(self.width as i32 - 1, self.height as i32 - 1);

        let (min, max) = scr_vs.into_iter().fold(
            (IVec2::splat(i32::MAX), IVec2::splat(i32::MIN)),
            |acc, v| (acc.0.min(v), acc.1.max(v)),
        );

        (min.max(scr_min), max.min(scr_max))
    }

    pub(crate) fn pixel_mut(&mut self, p: IVec2) -> Option<&mut Color> {
        self.map_idx(p).map(|idx| &mut self.pixels[idx])
    }

    pub fn clear(&mut self, color: Color) {
        (&mut *self.pixels).fill(color);
    }

    pub fn clear_z(&mut self) {
        (&mut self.z_buffer).fill(f32::MAX);
    }

    pub fn clear_all(&mut self, color: Color) {
        self.clear(color);
        self.clear_z();
    }
}
