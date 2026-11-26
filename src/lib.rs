mod macros;

mod color;
pub use color::Color;

mod framebuffer;
pub use framebuffer::{BackfaceCulling, Framebuffer, RenderingMode};

use framebuffer::{clip, shade};
pub use shade::{Fragment, FragmentShader, VertexShader};

mod mesh;
pub use mesh::{Mesh, MeshError};

mod math;
pub use math::{I64Vec2, IVec2, Mat2, Mat3, Mat4, Vec2, Vec3, Vec4};

use math::{camera, mvp};
pub use mvp::Mvp;
pub use camera::Camera;
