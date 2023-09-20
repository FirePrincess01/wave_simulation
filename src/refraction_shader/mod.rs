//! Contains the refraction shader to render refraction on a water surface
//! Also contains the color shader for colored waves

use super::vertex_heightmap_shader;

pub fn create_refraction_pipeline(device: 
    &wgpu::Device, 
    camera_bind_group_layout: &vertex_heightmap_shader::CameraBindGroupLayout, 
    texture_bind_group_layout: &vertex_heightmap_shader::TextureBindGroupLayout, 
    heightmap_bind_group_layout: &vertex_heightmap_shader::HeightmapBindGroupLayout, 
    surface_format: wgpu::TextureFormat) -> vertex_heightmap_shader::Pipeline
    {
        vertex_heightmap_shader::Pipeline::new(
            device,
            camera_bind_group_layout,
            texture_bind_group_layout,
            heightmap_bind_group_layout,
            surface_format,
            Some(include_str!("shader_refraction.wgsl")),
        )
    }


