mod macros;

mod color;
pub use color::Color;

mod framebuffer;
pub use framebuffer::{BackfaceCulling, Framebuffer, RenderingMode};

use framebuffer::{clip, shade};
pub use shade::{BarycentricShader, Empty, Fragment, Shader, Vary};

mod mesh;
pub use mesh::Mesh;

mod math;
pub use math::{I64Vec2, IVec2, Mat2, Mat3, Mat4, Vec2, Vec3, Vec4};

pub use camera::Camera;
use math::{camera, mvp};
pub use mvp::Mvp;
