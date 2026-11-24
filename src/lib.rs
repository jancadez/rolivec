mod color;
pub use color::Color;

mod framebuffer;
pub use framebuffer::Framebuffer;

use framebuffer::{draw2d, pixel, rasterize, shade};
pub use shade::{Fragment, FragmentShader, VertexShader, Mvp};

mod mesh;
pub use mesh::{Mesh, MeshError};

mod math;
use math::{i64mat2, imat2, mat2, mat3, mat4};
use math::{i64vec2, ivec2, vec2, vec3, vec4};

pub use i64mat2::I64Mat2;
pub use imat2::IMat2;
pub use mat2::Mat2;
pub use mat3::Mat3;
pub use mat4::Mat4;

pub use ivec2::IVec2;
pub use i64vec2::I64Vec2;
pub use vec2::Vec2;
pub use vec3::Vec3;
pub use vec4::Vec4;
