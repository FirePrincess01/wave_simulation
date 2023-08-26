//! A general purpose shader using vertices, colors and instance matrices
//!
//! Vertices and Colors are independently updateable
//! The implementation uses wgpu for rendering
//!

pub mod vertex;
pub mod color;
pub mod instance;
pub mod mesh;
pub mod pipeline;
pub mod camera_uniform;
pub mod camer_unform_buffer;

pub use vertex::Vertex;
pub use color::Color;
pub use instance::Instance;
pub use instance::InstanceRaw;
pub use mesh::Mesh;
pub use pipeline::Pipeline;
pub use camera_uniform::CameraUniform;
pub use camer_unform_buffer::CameraUniformBuffer;

