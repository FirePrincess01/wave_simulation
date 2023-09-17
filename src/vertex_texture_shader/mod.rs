//! A general purpose pipeline using vertices, textures and instances
//!

mod mesh;
mod pipeline;
mod texture_bind_group_layout;
mod texture;
mod vertex;

pub use pipeline::Pipeline;

pub use vertex::Vertex;
pub use texture::Texture;
pub use texture_bind_group_layout::TextureBindGroupLayout;

pub use mesh::Mesh;

pub use super::vertex_color_shader::instance::Instance;
pub use super::vertex_color_shader::instance::InstanceRaw;

pub use super::vertex_color_shader::camera_bind_group_layout::CameraBindGroupLayout;
pub use super::vertex_color_shader::camera_uniform::CameraUniform;
pub use super::vertex_color_shader::camera_uniform_buffer::CameraUniformBuffer;