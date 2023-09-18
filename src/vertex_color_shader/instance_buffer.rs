
use super::InstanceRaw;
use wgpu::util::DeviceExt;

pub struct InstanceBuffer {
    buffer: wgpu::Buffer,
    size: u32,
}

impl InstanceBuffer {
    pub fn new(device: &mut wgpu::Device, instances: &[InstanceRaw]) -> Self
    {
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instances),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        let size = instances.len() as u32;

        Self {
            buffer,
            size,
        }
    }

    pub fn update(&mut self, queue: &mut wgpu::Queue, instances: &[InstanceRaw])
    {   
        let data = bytemuck::cast_slice(&instances);

        if self.buffer.size() == data.len() as u64 {
            queue.write_buffer(&self.buffer, 0, data);
        }
    }

    pub fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>,) 
    {
        render_pass.set_vertex_buffer(2, self.buffer.slice(..));
    }

    pub fn size(&self) -> u32 {
        self.size
    }

}