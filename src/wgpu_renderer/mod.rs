//! Instantiates the device to render with wgpu
//!

pub mod camera;
pub mod texture;

use winit::window::Window;

pub struct WgpuRenderer
{
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    depth_texture: texture::Texture,
}

impl WgpuRenderer
{
    pub async fn new(window: &Window) -> Self 
    {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        // # Safety
        //
        // The surface needs to live as long as the window that created it
        // State owns the window so this should be safe
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                limits: if cfg!(target_arch = "wasm32") {
                    let mut defaults = wgpu::Limits::downlevel_webgl2_defaults();
                    defaults.max_texture_dimension_2d = 4096;
                    defaults
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None, 
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account fo that when drawing to the frame.
        let surface_format = surface_caps.formats.iter()
            .copied()
            .filter(|f| f.describe().srgb)
            .next()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![]
        };

        surface.configure(&device, &config);

        let depth_texture = texture::Texture::create_depth_texture(&device, &config, "depth_texture");

        Self {
            surface,
            device,
            queue,
            config,
            size,
            depth_texture,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.depth_texture = texture::Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
            self.surface.configure(&self.device, &self.config)
        }
    }

    pub fn device(&mut self) -> &mut wgpu::Device {
        &mut self.device
    }

    pub fn queue(&mut self) -> &mut wgpu::Queue {
        &mut self.queue
    }

    pub fn config(&self) -> &wgpu::SurfaceConfiguration {
        &self.config
    }

    pub fn get_current_texture(&self) -> Result<wgpu::SurfaceTexture, wgpu::SurfaceError> {
        
        self.surface.get_current_texture()
    }

    pub fn get_depth_texture_view(&self) -> &wgpu::TextureView {
        &self.depth_texture.view
    }

    // pub fn render(&mut self) -> WgpuRenderPass
    // {
    //     let output = self.surface.get_current_texture()?;

    //     let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

    //     let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor{
    //         label: Some("Render Encoder"),
    //     });

        
    //     let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { 
    //         label: Some("Render Pass"), 
    //         color_attachments: &[Some(wgpu::RenderPassColorAttachment {
    //             view: &view,
    //             resolve_target: None,
    //             ops: wgpu::Operations {
    //                 load: wgpu::LoadOp::Clear(wgpu::Color {
    //                     r: 0.1,
    //                     g: 0.2,
    //                     b: 0.3,
    //                     a: 1.0,
    //                 }),
    //                 store: true,
    //             }
    //         })], 
    //         depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
    //             view: &self.depth_texture.view,
    //             depth_ops: Some(wgpu::Operations {
    //                 load: wgpu::LoadOp::Clear(1.0),
    //                 store: true,
    //             }),
    //             stencil_ops: None,
    //         }) 
    //     });

    //     WgpuRenderPass {
    //         render_pass,
    //     }

    //     //     render_pass.set_pipeline(&self.render_pipeline);
    //     //     render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

    //     //     render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
    //     //     render_pass.set_vertex_buffer(1, self.color_buffer_sun.slice(..));
            
    //     //     render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            
    //     //     // sun
    //     //     render_pass.set_vertex_buffer(2, self.instance_buffer.slice(..));
    //     //     render_pass.draw_indexed(0..self.num_indices, 0, 0..1 as u32);

    //     //     // planet
    //     //     render_pass.set_vertex_buffer(1, self.color_buffer.slice(..));
    //     //     render_pass.draw_indexed(0..self.num_indices, 0, 1..self.instances.len() as u32);

    //     // }

    //     // self.queue.submit(std::iter::once(encoder.finish()));
    //     output.present();
        
    //     // Ok(())
    // }

}

