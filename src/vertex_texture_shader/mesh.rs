//! Contains the device buffers to render an object with this shader
//!

use super::Vertex;
use super::Texture;
use super::Instance;
use wgpu::util::{DeviceExt, self};


/// A general purpose shader using vertices, colors and an instance matrix
pub struct Mesh
{
    vertex_buffer: wgpu::Buffer,
    texture_index: usize,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    instance_buffer: wgpu::Buffer,
    num_instances: u32,
}

impl Mesh
{
    pub fn new(device: &mut wgpu::Device, 
        vertices: &[Vertex],
        texture_index: usize,
        indices: &[u32],
        instances: &[Instance]) -> Self
    {
        let vertex_buffer = device.create_buffer_init(
            &util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        let num_indices = indices.len() as u32;

        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data.as_slice()),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        let num_instances = instance_data.len() as u32;

        Self {
            vertex_buffer,
            texture_index,
            index_buffer,
            num_indices,
            instance_buffer,
            num_instances,
        }
    }

    pub fn update_vertex_buffer(&mut self, queue: &mut wgpu::Queue, vertices: &[Vertex])
    {   
        let data = bytemuck::cast_slice(&vertices);

        if self.vertex_buffer.size() == data.len() as u64 {
            queue.write_buffer(&self.vertex_buffer, 0, data);
        }
    }

    pub fn _set_texture_index(&mut self, texture_index: usize)
    {
        self.texture_index = texture_index;
    }

    pub fn update_instance_buffer(&mut self, queue: &mut wgpu::Queue, instances: &[Instance])
    {
        let instance_data = &instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let data = bytemuck::cast_slice(&instance_data);

        if self.instance_buffer.size() == data.len() as u64 {
            
            queue.write_buffer(&self.instance_buffer, 0, data);
        }
    }

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, textures: &'a [Texture])
    {
        {
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            textures[self.texture_index].bind(render_pass);
            
            render_pass.set_index_buffer(
                self.index_buffer.slice(..), 
                wgpu::IndexFormat::Uint32);

            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));

            render_pass.draw_indexed(0..self.num_indices, 0, 0..self.num_instances);
            
        }

    }
}