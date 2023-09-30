//! Gui for the wave simulation app

use crate::wgpu_renderer;

use super::gui;
use super::vertex_texture_shader;
use super::label;
use rusttype;
use wgpu::Label;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum ButtonOptionsId{
    SwitchViewPoint,
    SwitchTexture,
    PerformanceGraph,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
enum ButtonMenuId{
    Menu,
}

#[derive(Copy, Clone)]
enum LabelId{
    Fps
}

struct BtnMesh {
    pub texture: vertex_texture_shader::Texture,
    pub instance_buffer: vertex_texture_shader::InstanceBuffer,
}

impl BtnMesh {
    pub fn new(wgpu_renderer: &mut impl wgpu_renderer::WgpuRendererInterface, 
        texture_bytes: &[u8],
        texture_bind_group_layout: &vertex_texture_shader::TextureBindGroupLayout,
        instance: &vertex_texture_shader::Instance) -> Self
    {
        let texture_image = image::load_from_memory(texture_bytes).unwrap();
        let texture_rgba = texture_image.to_rgba8();

        let texture = vertex_texture_shader::Texture::new(
            wgpu_renderer, 
            &texture_bind_group_layout, 
            &texture_rgba, 
            Some("gui texture")).unwrap(); 

        let instance_raw = instance.to_raw();
        let instance_buffer = vertex_texture_shader::InstanceBuffer::new(wgpu_renderer.device(), &[instance_raw]);

        Self {
            texture,
            instance_buffer,
        }
    }

    pub fn update_instance_buffer(&mut self, queue: &wgpu::Queue, instance: &vertex_texture_shader::Instance)
    {
        let instance_raw = instance.to_raw();
        self.instance_buffer.update(queue, &[instance_raw]);
    }

    pub fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>,) 
    {
        self.texture.bind(render_pass);
        self.instance_buffer.bind_slot(render_pass, 1);
    }
}

pub struct WaveSimGui {
    width: u32,
    height: u32,

    btn_vertex_buffer: vertex_texture_shader::VertexBuffer,
    btn_index_buffer: vertex_texture_shader::IndexBuffer,

    // Menu
    gui_menu: gui::Gui<ButtonMenuId, LabelId>,
    btn_menu_mesh: BtnMesh,
    
    // Options
    gui_options: gui::Gui<ButtonOptionsId, LabelId>,
    btn_switch_view_point_mesh: BtnMesh,
    btn_switch_texture_mesh: BtnMesh,
    btn_performance_graph_mesh: BtnMesh,

    lbl_fps_host: label::Label,
    lbl_fps_mesh: label::LabelMesh, 

    show_submenu: bool,
}

impl WaveSimGui {
    pub fn new(wgpu_renderer: &mut impl wgpu_renderer::WgpuRendererInterface, 
        texture_bind_group_layout: &vertex_texture_shader::TextureBindGroupLayout,
        width: u32, 
        height: u32,
        font: &rusttype::Font) -> Self {

        let z = 10.1;
        let btn_width = 40;
        let btn_height = 40;
        let btn_boarder = 5;

        let btn_vertex_buffer = vertex_texture_shader::VertexBuffer::new(wgpu_renderer.device(), &Self::vertices(btn_width, btn_height));
        let btn_index_buffer = vertex_texture_shader::IndexBuffer::new(wgpu_renderer.device(), &Self::indices());

        // Menu
        let btn_menu = gui::Button::new(
            btn_width, 
            btn_height, 
            btn_boarder,
            ButtonMenuId::Menu);
        let mut gui_menu = gui::Gui::<ButtonMenuId, LabelId>::new(
            width,
            height,
            vec![
                gui::AlignedElement::new(gui::Alignment::BottomRight, 10, 10, gui::GuiElement::Button(btn_menu)), 
            ]
        );
        let mut btn_menu_instance = vertex_texture_shader::Instance::zero();
        let events = gui_menu.resize(width, height);
        for event in &events {
            match event.element_id {
                gui::ElementId::Button(button_id) => {
                    match button_id {
                        ButtonMenuId::Menu => {
                            btn_menu_instance.position.x = event.x as f32;
                            btn_menu_instance.position.y = event.y as f32;
                            btn_menu_instance.position.z = z;
                        },
                    }
                },
                gui::ElementId::Label(_) =>  { }
            }
        }

        let btn_menu_mesh = BtnMesh::new(wgpu_renderer, 
            include_bytes!("menu.png"), 
            texture_bind_group_layout, 
            &btn_menu_instance);


        let lbl_fps_host = label::Label::new(
            &font, 20.0, "60 fps  "
        );

        // Options
        let vertical_layout =  gui::VerticalLayout::<ButtonOptionsId, LabelId>::new(vec![
            gui::GuiElement::Button(gui::Button::new(
                btn_width, 
                btn_height, 
                btn_boarder,
                ButtonOptionsId::SwitchTexture)),
            gui::GuiElement::Button(gui::Button::new(
                btn_width, 
                btn_height, 
                btn_boarder,
                ButtonOptionsId::SwitchViewPoint)),
            gui::GuiElement::Button(gui::Button::new(
                btn_width, 
                btn_height, 
                btn_boarder,
                ButtonOptionsId::PerformanceGraph)), 
        ]);
        let mut gui_options = gui::Gui::<ButtonOptionsId, LabelId>::new(
            width,
            height,
            vec![
                gui::AlignedElement::new(
                    gui::Alignment::BottomRight, 
                    10, 
                    10 + btn_height + 2*btn_boarder, 
                    gui::GuiElement::VerticalLayout(vertical_layout)), 
                gui::AlignedElement::new(
                    gui::Alignment::TopLeft, 
                    5, 
                    5, 
                    gui::GuiElement::Label(gui::Label::new(
                        lbl_fps_host.width(), 
                        lbl_fps_host.height(), 
                        btn_boarder,
                        LabelId::Fps))),                    
        ]
        );
        let mut lbl_fps_instance = vertex_texture_shader::Instance::zero();
        let mut btn_switch_view_point_instance = vertex_texture_shader::Instance::zero();
        let mut btn_switch_texture_instance = vertex_texture_shader::Instance::zero();
        let mut btn_performance_graph_instance = vertex_texture_shader::Instance::zero();
        let events = gui_options.resize(width, height);
        for event in &events {
            match event.element_id {
                gui::ElementId::Button(button_id) => {
                    match button_id {
                        ButtonOptionsId::SwitchViewPoint => {
                            btn_switch_view_point_instance.position.x = event.x as f32;
                            btn_switch_view_point_instance.position.y = event.y as f32;
                            btn_switch_view_point_instance.position.z = z;
                        },
                        ButtonOptionsId::SwitchTexture => {
                            btn_switch_texture_instance.position.x = event.x as f32;
                            btn_switch_texture_instance.position.y = event.y as f32;
                            btn_switch_texture_instance.position.z = z;
                        },
                        ButtonOptionsId::PerformanceGraph => {
                            btn_performance_graph_instance.position.x = event.x as f32;
                            btn_performance_graph_instance.position.y = event.y as f32;
                            btn_performance_graph_instance.position.z = z;
                        },
                    }
                },
                gui::ElementId::Label(label_id) =>  {
                    match label_id {
                        LabelId::Fps => {
                            lbl_fps_instance.position.x = event.x as f32;
                            lbl_fps_instance.position.y = event.y as f32;
                            lbl_fps_instance.position.z = z;
                        },
                    }
                 }
            }
        }
        let btn_switch_view_point_mesh = BtnMesh::new(
            wgpu_renderer, 
            include_bytes!("view.png"), 
            texture_bind_group_layout,
            &btn_switch_view_point_instance);
        let btn_switch_texture_mesh = BtnMesh::new(
            wgpu_renderer, include_bytes!("mode.png"), 
            texture_bind_group_layout,
            &btn_switch_texture_instance);
        let btn_performance_graph_mesh = BtnMesh::new(wgpu_renderer, 
            include_bytes!("performance.png"), 
            texture_bind_group_layout,
            &btn_performance_graph_instance);

        let lbl_fps_mesh = label::LabelMesh::new(wgpu_renderer, 
            lbl_fps_host.get_image(), 
            texture_bind_group_layout,
            &btn_performance_graph_instance);

        // lbl_fps_host.get_image().save("fps_image.png").unwrap();


        Self {
            width,
            height,

            btn_vertex_buffer,
            btn_index_buffer,

            gui_menu,
            btn_menu_mesh,

            gui_options,
            btn_switch_view_point_mesh,
            btn_switch_texture_mesh,
            btn_performance_graph_mesh,

            lbl_fps_host,
            lbl_fps_mesh,

            show_submenu: false,

        }
    }


    fn vertices(width: u32, height: u32) -> [vertex_texture_shader::Vertex; 4]
    {
        let width = width as f32;
        let height = height as f32;

        let vertices: [vertex_texture_shader::Vertex; 4] = [
            vertex_texture_shader::Vertex { position: [0.0, 0.0, 0.0], tex_coords: [0.0, 1.0] }, // A
            vertex_texture_shader::Vertex { position: [width, 0.0, 0.0], tex_coords: [1.0, 1.0] }, // B
            vertex_texture_shader::Vertex { position: [width, height, 0.0], tex_coords: [1.0, 0.0] }, // C
            vertex_texture_shader::Vertex { position: [0.0, height, 0.0], tex_coords: [0.0, 0.0] }, // D
        ];

        vertices
    }

    fn indices() -> [u32; 6]
    {
        const INDICES: [u32;6] = [
            0, 1, 2,
            2, 3, 0,
        ];

        INDICES
    }

    pub fn mouse_moved(&mut self, x: u32, y: u32) -> bool
    {
        // change from mouse coordinate system to the gui coordinate system
        let y = self.height - y.min(self.height);

        let mouse_event = gui::MouseEvent::Moved{ x, y };

        let (consumed, _events) = self.gui_menu.mouse_event(mouse_event);
        if consumed {
            return true;
        }

        if self.show_submenu {
            let (consumed, _events) = self.gui_options.mouse_event(mouse_event);
            if consumed {
                return true;
            }
        }

        return false;
    }

    pub fn mouse_pressed(&mut self, pressed: bool) -> (bool, Option<gui::ButtonPressedEvent<ButtonOptionsId>>)
    {
        let mouse_event = if pressed {
                gui::MouseEvent::Pressed
            } else {
                gui::MouseEvent::Released
            };

        let (consumed, event) = self.gui_menu.mouse_event(mouse_event);
        if consumed {
            match event {
                Some(event) => { 
                    self.handle_gui_menu_event(event);
                },
                None => {},
            }
            
            return (true, None);
        }
    
        if self.show_submenu {
            let (consumed, event) = self.gui_options.mouse_event(mouse_event);
            if consumed {
                return (true, event);
            }
        }
    

        (false, None)
    }

    fn handle_gui_menu_event(&mut self, event: gui::ButtonPressedEvent<ButtonMenuId>) {
        match event.button_id {
            ButtonMenuId::Menu => {
                self.show_submenu = !self.show_submenu;
            },
        }
    }

    pub fn set_fps<'a>(&mut self, queue: &wgpu::Queue, font: &'a rusttype::Font, fps: u32) {

        let text = fps.to_string() + " fps";
        self.lbl_fps_host.update(font, &text);
        self.lbl_fps_mesh.update_texture(queue, self.lbl_fps_host.get_image());
    }

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>)
    {
        self.btn_vertex_buffer.bind(render_pass);
        self.btn_index_buffer.bind(render_pass);

        self.btn_menu_mesh.bind(render_pass);
        render_pass.draw_indexed(0..self.btn_index_buffer.size(), 0, 0..1);

        if self.show_submenu {
            
            render_pass.draw_indexed(0..self.btn_index_buffer.size(), 0, 0..1);

            self.btn_switch_view_point_mesh.bind(render_pass);
            render_pass.draw_indexed(0..self.btn_index_buffer.size(), 0, 0..1);

            self.btn_switch_texture_mesh.bind(render_pass);
            render_pass.draw_indexed(0..self.btn_index_buffer.size(), 0, 0..1);

            self.btn_performance_graph_mesh.bind(render_pass);
            render_pass.draw_indexed(0..self.btn_index_buffer.size(), 0, 0..1);

            self.lbl_fps_mesh.draw(render_pass);
        }
    }

    pub fn resize(&mut self, queue: &wgpu::Queue, width: u32, height: u32)
    {
        self.width = width;
        self.height = height;

        let mut btn_menu_instance = vertex_texture_shader::Instance::zero();
        let events = self.gui_menu.resize(width, height);
        for event in &events {
            match event.element_id {
                gui::ElementId::Button(button_id) => {
                    match button_id {
                        ButtonMenuId::Menu => {
                            btn_menu_instance.position.x = event.x as f32;
                            btn_menu_instance.position.y = event.y as f32;
                        },
                    }
                },
                gui::ElementId::Label(_) =>  { }
            }
        }

        let mut lbl_fps_instance = vertex_texture_shader::Instance::zero();
        let mut btn_switch_view_point_instance = vertex_texture_shader::Instance::zero();
        let mut btn_switch_texture_instance = vertex_texture_shader::Instance::zero();
        let mut btn_performance_graph_instance = vertex_texture_shader::Instance::zero();
        let events = self.gui_options.resize(width, height);
        for event in &events {
            match event.element_id {
                gui::ElementId::Button(button_id) => {
                    match button_id {
                        ButtonOptionsId::SwitchViewPoint => {
                            btn_switch_view_point_instance.position.x = event.x as f32;
                            btn_switch_view_point_instance.position.y = event.y as f32;
                        },
                        ButtonOptionsId::SwitchTexture => {
                            btn_switch_texture_instance.position.x = event.x as f32;
                            btn_switch_texture_instance.position.y = event.y as f32;
                        },
                        ButtonOptionsId::PerformanceGraph => {
                            btn_performance_graph_instance.position.x = event.x as f32;
                            btn_performance_graph_instance.position.y = event.y as f32;
                        },
                    }
                },
                gui::ElementId::Label(label_id) =>  { 
                    match label_id {
                        LabelId::Fps => {
                            lbl_fps_instance.position.x = event.x as f32;
                            lbl_fps_instance.position.y = event.y as f32;
                        },
                    }
                }
            }
        }

        self.btn_menu_mesh.update_instance_buffer(queue, &btn_menu_instance);
        self.lbl_fps_mesh.update_instance_buffer(queue, &lbl_fps_instance);
        self.btn_switch_view_point_mesh.update_instance_buffer(queue, &btn_switch_view_point_instance);
        self.btn_switch_texture_mesh.update_instance_buffer(queue, &btn_switch_texture_instance);
        self.btn_performance_graph_mesh.update_instance_buffer(queue, &btn_performance_graph_instance);
    }
}