//! The Heightmap struct used in the shader
//!


#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Heightmap {
    pub height: f32,
}

impl Heightmap {
    pub fn zero() -> Self {
        Self { height: 0.0 }
    }
}