//! Contains a buffer for the Heightmap array
//!

use super::heightmap::Heightmap;
use super::heightmap_bind_group_layout::HeightmapBindGroupLayout;
use wgpu::util::DeviceExt;

pub struct HeightmapBuffer{
    heightmap_buffer: wgpu::Buffer,
    hightmap_bind_group: wgpu::BindGroup,
}

impl HeightmapBuffer {
    pub fn new(
        device: &mut wgpu::Device, 
        heightmap_bind_group_layout: &HeightmapBindGroupLayout, 
        heightmap: &[Heightmap]) -> Self {

        let heightmap_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Heightmap Buffer"),
                contents: bytemuck::cast_slice(&heightmap),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            }
        );

        let hightmap_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &heightmap_bind_group_layout.get(),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: heightmap_buffer.as_entire_binding(),
                }
            ],
            label: Some("hightmap_bind_group"),
        });

        Self {
            heightmap_buffer,
            hightmap_bind_group,
        }
    }

    pub fn update(&mut self, queue: &mut wgpu::Queue, heightmap: &[Heightmap])
    {   
        let data = bytemuck::cast_slice(&heightmap);

        if self.heightmap_buffer.size() == data.len() as u64 {
            queue.write_buffer(&self.heightmap_buffer, 0, data);
        }
    }

    pub fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>){
        render_pass.set_bind_group(2, &self.hightmap_bind_group, &[]);
    }

}