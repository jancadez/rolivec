mod color;
pub use color::Color;

mod framebuffer;
pub use framebuffer::Framebuffer;

use framebuffer::{pixel, draw2d, rasterize};
pub use rasterize::{VertexShader, Fragment, FragmentShader};

mod math;
use math::{vec2, vec3, mat3};
pub use vec2::Vec2;
pub use vec3::Vec3;
pub use mat3::Mat3;
