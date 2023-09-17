#![feature(new_uninit)]

//! The main file of the application
//!

mod vertex_color_shader;
mod vertex_texture_shader;
mod wgpu_renderer;
mod geometry;
mod wave_equation;
mod performance_monitor;
mod mouse_selector;

use cgmath::Point3;
use cgmath::prelude::*;

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
    size: winit::dpi::PhysicalSize<u32>,
    scale_factor: f32,

    // wgpu_renderer
    wgpu_renderer : wgpu_renderer::WgpuRenderer,
    _camera_bind_group_layout: vertex_color_shader::CameraBindGroupLayout,
    pipeline: vertex_color_shader::Pipeline,
    pipeline_lines: vertex_color_shader::Pipeline,
    _texture_bind_group_layout: vertex_texture_shader::TextureBindGroupLayout,
    pipeline_texture: vertex_texture_shader::Pipeline,

    // camera
    camera: wgpu_renderer::camera::Camera,
    camera_controller: wgpu_renderer::camera::CameraController,
    projection: wgpu_renderer::camera::Projection,

    camera_uniform: vertex_color_shader::CameraUniform,
    camera_uniform_buffer: vertex_color_shader::CameraUniformBuffer,

    camera_uniform_orthographic: vertex_color_shader::CameraUniform,
    camera_uniform_orthographic_buffer: vertex_color_shader::CameraUniformBuffer,

    // textures
    textures: Vec<vertex_texture_shader::Texture>,

    // grid
    grid_host: geometry::Grid<M, N, MN>,
    grid_device: vertex_color_shader::Mesh,
    grid_instances: Vec<vertex_color_shader::Instance>,

    // grid textured
    grid_textured_device: vertex_texture_shader::Mesh,

    // input
    mouse_pressed_camera: bool,
    mouse_pressed_forces: bool,
    show_performance_graph: bool,
    show_textured_grid: bool,
    mouse_selector: mouse_selector::MouseSelector,

    // simulation
    wave_equation: wave_equation::WaveEquation<M, N>,

    // performance monitor
    watch: performance_monitor::Watch<4>,
    graph_host: performance_monitor::Graph,
    graph_device: vertex_color_shader::Mesh,
}

impl WaveSimulation
{
    async fn new(window: &Window) -> Self
    {
        let size = window.inner_size();
        let scale_factor = window.scale_factor() as f32;
        let mut wgpu_renderer = wgpu_renderer::WgpuRenderer::new(&window).await;
        let surface_format = wgpu_renderer.config().format;
        let camera_bind_group_layout = vertex_color_shader::CameraBindGroupLayout::new(&mut wgpu_renderer.device());
        let pipeline = vertex_color_shader::Pipeline::new(
            &mut wgpu_renderer.device(), 
            &camera_bind_group_layout, 
            surface_format,
        );
        let pipeline_lines = vertex_color_shader::Pipeline::new_lines(
            &mut wgpu_renderer.device(), 
            &camera_bind_group_layout, 
            surface_format,
        );
        let texture_bind_group_layout = vertex_texture_shader::TextureBindGroupLayout::new(&mut wgpu_renderer.device());
        let pipeline_texture = vertex_texture_shader::Pipeline::new(
            &mut wgpu_renderer.device(), 
            &camera_bind_group_layout, 
            &texture_bind_group_layout, 
            surface_format
        );

        let position = Point3::new(0.0, 0.0, 0.0);
        let yaw = cgmath::Deg(0.0);
        let pitch = cgmath::Deg(0.0);
        let mut camera = wgpu_renderer::camera::Camera::new(position, yaw, pitch);
        Self::top_view_point(&mut camera);

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
            &camera_bind_group_layout);

        let camera_uniform_orthographic: vertex_color_shader::CameraUniform = vertex_color_shader::CameraUniform::new_orthographic(width, height);
        let mut camera_uniform_orthographic_buffer = vertex_color_shader::CameraUniformBuffer::new(
                &mut wgpu_renderer.device(), 
                &camera_bind_group_layout);

        camera_uniform_orthographic_buffer.update(wgpu_renderer.queue(), camera_uniform_orthographic);   // add uniform identity matrix



        let grid_host: geometry::Grid<M, N, MN> = geometry::Grid::new();

        const INSTANCES: &[vertex_color_shader::Instance] = &[ 
            vertex_color_shader::Instance{
                position: glam::Vec3::new(-((N/2) as f32), -((M/2) as f32), 0.0),
                rotation: glam::Quat::IDENTITY,
            },
        ];

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

        let grid_textured_device = vertex_texture_shader::Mesh::new(
            &mut wgpu_renderer.device(),
            &grid_host.vertices_textured_slice(),
            0, 
            &grid_host.indices_slice(),
            &grid_instances,
        );

        let wave_equation: wave_equation::WaveEquation<M, N> = wave_equation::WaveEquation::new();

        // performance monitor
        const WATCHPOINTS_SIZE: usize  = 4;
        let watch: performance_monitor::Watch<WATCHPOINTS_SIZE> = performance_monitor::Watch::new(); 
        let graph_host = performance_monitor::Graph::new(WATCHPOINTS_SIZE);
        let graph_instance = vertex_color_shader::Instance{
            position: glam::Vec3::ZERO,
            rotation: glam::Quat::IDENTITY,
        };
        let graph_instances = [graph_instance];
        let graph_device = vertex_color_shader::Mesh::new(
            &mut wgpu_renderer.device(),
            &graph_host.vertices.as_slice(),
            &graph_host.colors.as_slice(),
            &graph_host.indices.as_slice(),
            &graph_instances,
        );

        // image
        let diffuse_bytes = include_bytes!("pony2.png");
        let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();
        let diffuse_rgba = diffuse_image.to_rgba8();

        let diffuse_texture = vertex_texture_shader::Texture::new(
            &mut wgpu_renderer.device(), 
            &texture_bind_group_layout, 
            &diffuse_rgba, 
            Some("pony2.png")).unwrap(); 
        diffuse_texture.write(&mut wgpu_renderer.queue(), &diffuse_rgba);

        let textures = vec![diffuse_texture];

        // Test data

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

        // const VERTICES: &[vertex_texture_shader::Vertex]  = &[
        //     vertex_texture_shader::Vertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 0.99240386], }, // A
        //     vertex_texture_shader::Vertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 0.56958647], }, // B
        //     vertex_texture_shader::Vertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 0.05060294], }, // C
        //     vertex_texture_shader::Vertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 0.1526709], }, // D
        //     vertex_texture_shader::Vertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.7347359], }, // E
        // ];
        
        // const INDICES: &[u32] = &[
        //     0, 1, 4,
        //     1, 2, 4,
        //     2, 3, 4,
        // ];

        // const INSTANCES2: &[vertex_color_shader::Instance] = &[ 
        //     vertex_color_shader::Instance{
        //         position: glam::Vec3::new(0.0, 0.0, 0.0),
        //         rotation: glam::Quat::IDENTITY,
        //     },
        // ];

        // let textured_mesh_instances = Vec::from(INSTANCES2);


        Self {
            size,
            scale_factor,

            wgpu_renderer,
            _camera_bind_group_layout: camera_bind_group_layout,
            pipeline,
            pipeline_lines,
            _texture_bind_group_layout: texture_bind_group_layout,
            pipeline_texture,

            camera,
            camera_controller,
            projection,

            camera_uniform,
            camera_uniform_buffer,

            camera_uniform_orthographic,
            camera_uniform_orthographic_buffer,

            grid_host,
            grid_device,
    
            grid_instances,

            grid_textured_device,

            textures,

            mouse_pressed_camera: false,
            mouse_pressed_forces: false,
            show_performance_graph: false,
            show_textured_grid: false,
            mouse_selector,

            wave_equation,

            watch,
            graph_host,
            graph_device,
        }
    }

    fn mouse_pressed(&self) -> bool {
        self.mouse_pressed_camera
    }

    fn update_scale_factor(&mut self, scale_factor: f32) {
        self.scale_factor = scale_factor;
    }

    fn top_view_point(camera: &mut wgpu_renderer::camera::Camera) {
        let position = Point3::new(0.0, 0.0, 67.0*4.0);
        let yaw = cgmath::Deg(-90.0).into();
        let pitch = cgmath::Deg(0.0).into();

        camera.position = position;
        camera.yaw = yaw;
        camera.pitch = pitch;
    }

    fn side_view_point(camera: &mut wgpu_renderer::camera::Camera) {
        let position = Point3::new(0.0, -(50.0 * 4.0), 55.0);
        let yaw = cgmath::Deg(-90.0).into();
        let pitch = cgmath::Deg(60.0).into();

        camera.position = position;
        camera.yaw = yaw;
        camera.pitch = pitch;
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        
        self.projection.resize(new_size.width, new_size.height);
        self.wgpu_renderer.resize(new_size);
        
        self.camera_uniform_orthographic.resize_orthographic(new_size.width, new_size.height);
        self.camera_uniform_orthographic_buffer.update(self.wgpu_renderer.queue(), self.camera_uniform_orthographic);
        
        self.mouse_selector.resize(new_size.width, new_size.height);
    }

    fn apply_scale_factor(&self, position: winit::dpi::PhysicalPosition<f64>) -> winit::dpi::PhysicalPosition<f64> {
        
        cfg_if::cfg_if! {
            // apply scale factor for the web
            if #[cfg(target_arch = "wasm32")] {
                let mut res = position;
                res.x = res.x / self.scale_factor as f64;
                res.y = res.y / self.scale_factor as f64;
                res
            }
            else {
                position
            }
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::F2),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => { 
                self.show_performance_graph = !self.show_performance_graph;
                true
            },
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Key1),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => { 
                Self::top_view_point(&mut self.camera);
                true
            },
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Key2),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => { 
                Self::side_view_point(&mut self.camera);
                true
            },
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Key3),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => { 
                self.show_textured_grid = !self.show_textured_grid;
                true
            },
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
                self.wave_equation.interupt_mouse();
                true
            } 
            WindowEvent::Touch(touch) => {
                let pos = self.apply_scale_factor(touch.location);

                match touch.phase {
                    TouchPhase::Started => {
                        self.mouse_pressed_forces = true;
                        self.mouse_selector.calc_mouse_position_on_screen(pos.x as f32, pos.y as f32);
                    }
                    TouchPhase::Ended => {
                        self.mouse_pressed_forces = false;
                        self.wave_equation.interupt_mouse();
                    }
                    TouchPhase::Cancelled => {
                        self.mouse_pressed_forces = false;
                        self.wave_equation.interupt_mouse();
                    }
                    TouchPhase::Moved => {
                        self.mouse_selector.calc_mouse_position_on_screen(pos.x as f32, pos.y as f32);
                    }
                }
                true
            } 
            WindowEvent::CursorMoved { position, .. } => {
                let pos = self.apply_scale_factor(*position);

                self.mouse_selector.calc_mouse_position_on_screen(pos.x as f32, pos.y as f32);
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

                self.grid_host.vertices_textured[y][x].position[2] = val * 1.0;
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
        self.watch.start(1);
            self.wave_equation.step();
        self.watch.stop(1);
        
        // convert to colours
        self.watch.start(2);
            self.wave_equation_to_grid_host();
        self.watch.stop(2);

        // mesh
        self.watch.start(3);
            self.grid_device.update_vetex_buffer(&mut self.wgpu_renderer.queue(), &self.grid_host.vertices_slice());
            self.grid_device.update_color_buffer(&mut self.wgpu_renderer.queue(), &self.grid_host.colors_slice());
            self.grid_device.update_instance_buffer(&mut self.wgpu_renderer.queue(), &self.grid_instances);

            self.grid_textured_device.update_vetex_buffer(&mut self.wgpu_renderer.queue(), &self.grid_host.vertices_textured_slice());
            self.grid_textured_device.update_instance_buffer(&mut self.wgpu_renderer.queue(), &self.grid_instances);
        self.watch.stop(3);

        // performance monitor
        self.watch.update();
        self.watch.update_viewer(&mut self.graph_host);
        self.graph_device.update_vetex_buffer(&mut self.wgpu_renderer.queue(), self.graph_host.vertices.as_slice());
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

            // grid
            if self.show_textured_grid {
                self.pipeline_texture.bind(&mut render_pass);
                self.camera_uniform_buffer.bind(&mut render_pass);
                self.grid_textured_device.draw(&mut render_pass, &self.textures);
            }
            else {
                self.pipeline.bind(&mut render_pass);
                self.camera_uniform_buffer.bind(&mut render_pass);
                self.grid_device.draw(&mut render_pass);
            }

            // performance monitor
            if self.show_performance_graph {
                self.pipeline_lines.bind(&mut render_pass);
                self.camera_uniform_orthographic_buffer.bind(&mut render_pass);
                self.graph_device.draw(&mut render_pass);
            }
        }

        self.watch.start(0);
            self.wgpu_renderer.queue().submit(std::iter::once(encoder.finish()));
            output.present();
        self.watch.stop(0);
        
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

        // on the web, the resize event does not fire, so we check the value manually
        #[cfg(target_arch = "wasm32")] 
        {
            if window.inner_size() != state.size
            {
                let scale = window.scale_factor() as f32;
                let size = window.inner_size();
    
                state.update_scale_factor(scale);
                state.resize(size);
            }
        }

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
                    WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size} => {
                        state.update_scale_factor(*scale_factor as f32);
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
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
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