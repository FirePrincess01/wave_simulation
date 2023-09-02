#![feature(new_uninit)]

//! The main file of the application
//!

mod vertex_color_shader;
mod wgpu_renderer;
mod geometry;
mod wave_equation;
mod mouse_selector;
use cgmath::{Point3, Angle};

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window},
};

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

const M: usize = 80*4;
const N: usize = 70*4;
const MN: usize = M * N;
const WAVE_INDEX: usize = 0;    //The index of the wave instance

struct WaveSimulation
{   
    // wgpu_renderer
    wgpu_renderer : wgpu_renderer::WgpuRenderer,
    pipeline: vertex_color_shader::Pipeline,

    // camera
    camera: wgpu_renderer::camera::Camera,
    camera_controller: wgpu_renderer::camera::CameraController,
    projection: wgpu_renderer::camera::Projection,

    camera_uniform: vertex_color_shader::CameraUniform,
    camera_uniform_buffer: vertex_color_shader::CameraUniformBuffer,

    // render data
    grid_host: geometry::Grid<M, N, MN>,
    grid_device: vertex_color_shader::Mesh,
    grid_instances: Vec<vertex_color_shader::Instance>,

    // input
    mouse_pressed_camera: bool,
    mouse_pressed_forces: bool,
    mouse_selector: mouse_selector::MouseSelector,

    // simulation
    wave_equation: wave_equation::WaveEquation<M, N>,
}

impl WaveSimulation
{
    async fn new(window: &Window) -> Self
    {
        let mut wgpu_renderer = wgpu_renderer::WgpuRenderer::new(&window).await;
        let surface_format = wgpu_renderer.config().format;
        let pipeline = vertex_color_shader::Pipeline::new(&mut wgpu_renderer.device(), surface_format);

        let position = Point3::new(0.0, 0.0, 67.0*4.0);
        let yaw = cgmath::Deg(-90.0);
        let pitch = cgmath::Deg(0.0);
        let camera = wgpu_renderer::camera::Camera::new(position, yaw, pitch);

        let speed = 1.0;
        let sensitivity = 1.0;
        let camera_controller = wgpu_renderer::camera::CameraController::new(speed, sensitivity);

        let width = wgpu_renderer.config().width;
        let height = wgpu_renderer.config().height;
        let fovy = cgmath::Deg(45.0);
        let znear = 0.1;
        let zfar = 100.0;
        let projection = wgpu_renderer::camera::Projection::new(width, height, fovy, znear, zfar);

        let camera_uniform = vertex_color_shader::CameraUniform::new();

        let camera_uniform_buffer = vertex_color_shader::CameraUniformBuffer::new(
            &mut wgpu_renderer.device(), 
            pipeline.camera_bind_group_layout());


        // const VERTICES: &[vertex_color_shader::Vertex] = &[
        //     vertex_color_shader::vertex::Vertex { position: [-0.0868241, 0.49240386, 0.0] }, // A
        //     vertex_color_shader::vertex::Vertex { position: [-0.49513406, 0.06958647, 0.0] }, // B
        //     vertex_color_shader::vertex::Vertex { position: [-0.21918549, -0.44939706, 0.0] }, // C
        //     vertex_color_shader::vertex::Vertex { position: [0.35966998, -0.3473291, 0.0] }, // D
        //     vertex_color_shader::vertex::Vertex { position: [0.44147372, 0.2347359, 0.0] }, // E
        // ];

        // const COLORS: &[vertex_color_shader::Color] = &[
        //     vertex_color_shader::color::Color { color: [0.5, 0.0, 0.5] }, // A
        //     vertex_color_shader::color::Color { color: [0.5, 0.0, 0.5] }, // B
        //     vertex_color_shader::color::Color { color: [0.5, 0.0, 0.5] }, // C
        //     vertex_color_shader::color::Color { color: [0.5, 0.0, 0.5] }, // D
        //     vertex_color_shader::color::Color { color: [0.5, 0.0, 0.5] }, // E
        // ];

        // const INDICES: &[u16] = &[
        //     0, 1, 4,
        //     1, 2, 4,
        //     2, 3, 4,
        // ];

        let grid_host: geometry::Grid<M, N, MN> = geometry::Grid::new();

        const INSTANCES: &[vertex_color_shader::Instance] = &[ 
            vertex_color_shader::Instance{
                position: glam::Vec3::new(-((N/2) as f32), -((M/2) as f32), 0.0),
                rotation: glam::Quat::IDENTITY,
            },
        ];

        // let grid_vertices = Vec::from(VERTICES);
        // let grid_colors = Vec::from(COLORS);
        // let grid_indices = Vec::from(INDICES);
        let grid_instances = Vec::from(INSTANCES);

        //hacked FoV, application appears to use a multiplicator of exactly 1.5
        let mouse_selector = mouse_selector::MouseSelector::new(width, height, (fovy / 2.).tan() * 1.5, grid_instances[WAVE_INDEX]);

        let grid_device = vertex_color_shader::Mesh::new(
            &mut wgpu_renderer.device(),
            &grid_host.vertices_slice(),
            &grid_host.colors_slice(),
            &grid_host.indices_slice(),
            &grid_instances,
        );

        let wave_equation: wave_equation::WaveEquation<M, N> = wave_equation::WaveEquation::new();

        Self {
            wgpu_renderer,
            pipeline,

            camera,
            camera_controller,
            projection,

            camera_uniform,
            camera_uniform_buffer,

            grid_host,
            grid_device,
    
            // grid_vertices,
            // grid_colors,
            // _grid_indices: grid_indices,
            grid_instances,

            mouse_pressed_camera: false,
            mouse_pressed_forces: false,
            mouse_selector,

            wave_equation,
        }
    }

    fn mouse_pressed(&self) -> bool {
        self.mouse_pressed_camera
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.projection.resize(new_size.width, new_size.height);
        self.wgpu_renderer.resize(new_size);
        self.mouse_selector.resize(new_size.width, new_size.height);
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(key),
                        state,
                        ..
                    },
                ..
            } => self.camera_controller.process_keyboard(*key, *state),
            WindowEvent::MouseWheel { delta, .. } => {
                self.camera_controller.process_scroll(delta);
                true
            }
            WindowEvent::MouseInput {
                button: MouseButton::Right,
                state,
                ..
            } => {
                self.mouse_pressed_camera = *state == ElementState::Pressed;
                true
            }
            WindowEvent::MouseInput {
                button: MouseButton::Left,
                state,//ElementState::Pressed,
                ..
            } => {
                self.mouse_pressed_forces = *state == ElementState::Pressed;
                true
            } 
            WindowEvent::Touch(touch) => {
                match touch.phase {
                    TouchPhase::Started => {
                        self.mouse_pressed_forces = true;
                        self.mouse_selector.calc_mouse_position_on_screen(touch.location.x as f32, touch.location.y as f32);
                    },
                    TouchPhase::Ended => {self.mouse_pressed_forces = false;},
                    TouchPhase::Cancelled => {self.mouse_pressed_forces = false;},
                    TouchPhase::Moved => {
                        self.mouse_selector.calc_mouse_position_on_screen(touch.location.x as f32, touch.location.y as f32);
                    },
                }
                true
            } 
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_selector.calc_mouse_position_on_screen(position.x as f32, position.y as f32);
                true
            }
            _ => false,
        }
    }

    fn wave_equation_to_grid_host(&mut self) {
        for y in 0..M {
            for x in 0..N {

                let val = self.wave_equation.get_current()[y][x];
                let val_colour = ((val + 1.0) * 0.5) as f64;

                let gradient = colorous::COOL;
                let color = gradient.eval_continuous(val_colour);

                let r =   color.r as f32 / 255.0;
                let g = color.g as f32 / 255.0;
                let b = color.b as f32 / 255.0;

                self.grid_host.colors[y][x].color = [r, g, b];
                self.grid_host.vertices[y][x].position[2] = val * 1.0;
            }
        }
    }

    fn update(&mut self, dt: instant::Duration) {
        // camera
        self.camera_controller.update_camera(&mut self.camera, dt);
        self.camera_uniform.update_view_proj(&self.camera, &self.projection);
        self.camera_uniform_buffer.update(self.wgpu_renderer.queue(), self.camera_uniform);

       // simulation
        // Apply forces
        if self.mouse_pressed_forces {
            let (y,x) = self.mouse_selector.mouse_position_on_grid(&self.camera);
            self.wave_equation.add_impulse(y, x);
        }

        // calculate simulation step
        self.wave_equation.step();
        
        // convert to colours
        self.wave_equation_to_grid_host();

        // mesh
        self.grid_device.update_vetex_buffer(&mut self.wgpu_renderer.queue(), &self.grid_host.vertices_slice());
        self.grid_device.update_color_buffer(&mut self.wgpu_renderer.queue(), &self.grid_host.colors_slice());
        self.grid_device.update_instance_buffer(&mut self.wgpu_renderer.queue(), &self.grid_instances);
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.wgpu_renderer.get_current_texture()?;

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.wgpu_renderer.device().create_command_encoder(&wgpu::CommandEncoderDescriptor{
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { 
                label: Some("Render Pass"), 
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.01,
                            g: 0.02,
                            b: 0.03,
                            a: 1.0,
                        }),
                        store: true,
                    }
                })], 
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.wgpu_renderer.get_depth_texture_view(),
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }) 
            });

            self.pipeline.bind(&mut render_pass);

            self.camera_uniform_buffer.bind(&mut render_pass);

            self.grid_device.draw(&mut render_pass)
        }

        self.wgpu_renderer.queue().submit(std::iter::once(encoder.finish()));
        output.present();
        
        Ok(())
    }
}



// runs the event loop
#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub async fn run()
{
    // We need to toggle what logger we are using based on if we are in WASM land or not. 
    cfg_if::cfg_if! {
        
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    // create our event loop and window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    use winit::dpi::PhysicalSize;
        window.set_inner_size(PhysicalSize::new(700, 800));

    // we need to add a canvas to the HTML document that we will host our application
    #[cfg(target_arch = "wasm32")]
    {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        // use winit::dpi::PhysicalSize;
        // window.set_inner_size(PhysicalSize::new(600, 800));
        
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-demo")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }



    let mut state = WaveSimulation::new(&window).await;

    let mut last_render_time = instant::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion{ delta, },
                .. // We're not using device_id currently
            } => if state.mouse_pressed() {
                state.camera_controller.process_mouse(delta.0, delta.1)
            },

            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => if !state.input(event) {
                match event {
                    #[cfg(not(target_arch="wasm32"))]
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size , ..} => {
                        state.resize(**new_inner_size);
                    }
                    _ => {}
            } 
            },
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let now = instant::Instant::now();
                let dt = now - last_render_time;
                last_render_time = now;
                
                state.update(dt);
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => *control_flow = ControlFlow::Exit,
                    // Err(wgpu::SurfaceError::Lost) => self.resize(state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually request it
                window.request_redraw();
            }
            _ => {}
        }
    });
}