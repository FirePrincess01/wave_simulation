//! A general purpose pipeline using vertices, colors and instances
//!
//! Vertices and Colors are independently updateable
//! The implementation uses wgpu for rendering
//!


pub mod vertex;
pub mod color;
pub mod instance;
pub mod mesh;
pub mod pipeline;
pub mod camera_bind_group_layout;
pub mod camera_uniform;
pub mod camera_uniform_buffer;

pub use vertex::Vertex;
pub use color::Color;
pub use instance::Instance;
pub use instance::InstanceRaw;
pub use mesh::Mesh;
pub use pipeline::Pipeline;
pub use camera_bind_group_layout::CameraBindGroupLayout;
pub use camera_uniform::CameraUniform;
pub use camera_uniform_buffer::CameraUniformBuffer;

