//! A bind group to create a heightmap for this shader
//!

pub struct HeightmapBindGroupLayout {
    heightmap_bind_group_layout: wgpu::BindGroupLayout,
}

impl HeightmapBindGroupLayout {

    pub fn new(device: &mut wgpu::Device) -> Self {

            // Camera
        let heightmap_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset:false, 
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("heightmap_bind_group_layout"),
        });

        Self {
            heightmap_bind_group_layout,
        }
    }

    pub fn get(&self) -> &wgpu::BindGroupLayout {
        &self.heightmap_bind_group_layout
    }

}