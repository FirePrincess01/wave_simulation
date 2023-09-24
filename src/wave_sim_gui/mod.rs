//! Gui for the wave simulation app

use crate::wgpu_renderer;

use super::gui::*;
use super::vertex_texture_shader;

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
    gui_menu: Gui<ButtonMenuId, LabelId>,
    btn_menu_mesh: BtnMesh,
    
    // Options
    gui_options: Gui<ButtonOptionsId, LabelId>,
    btn_switch_view_point_mesh: BtnMesh,
    btn_switch_texture_mesh: BtnMesh,
    btn_performance_graph_mesh: BtnMesh,

    show_submenu: bool,
}

impl WaveSimGui {
    pub fn new(wgpu_renderer: &mut impl wgpu_renderer::WgpuRendererInterface, 
        texture_bind_group_layout: &vertex_texture_shader::TextureBindGroupLayout,
        width: u32, 
        height: u32) -> Self {

        let z = 10.1;
        let btn_width = 40;
        let btn_height = 40;
        let btn_boarder = 5;

        let btn_vertex_buffer = vertex_texture_shader::VertexBuffer::new(wgpu_renderer.device(), &Self::vertices(btn_width, btn_height));
        let btn_index_buffer = vertex_texture_shader::IndexBuffer::new(wgpu_renderer.device(), &Self::indices());

        // Menu
        let btn_menu = Button::new(
            btn_width, 
            btn_height, 
            btn_boarder,
            ButtonMenuId::Menu);
        let mut gui_menu = Gui::<ButtonMenuId, LabelId>::new(
            width,
            height,
            vec![
                AlignedElement::new(Alignment::BottomRight, 10, 10, GuiElement::Button(btn_menu)), 
            ]
        );
        let mut btn_menu_instance = vertex_texture_shader::Instance::zero();
        let events = gui_menu.resize(width, height);
        for event in &events {
            match event.element_id {
                ElementId::Button(button_id) => {
                    match button_id {
                        ButtonMenuId::Menu => {
                            btn_menu_instance.position.x = event.x as f32;
                            btn_menu_instance.position.y = event.y as f32;
                            btn_menu_instance.position.z = z;
                        },
                    }
                },
                ElementId::Label(_) =>  { }
            }
        }

        let btn_menu_mesh = BtnMesh::new(wgpu_renderer, 
            include_bytes!("menu.png"), 
            texture_bind_group_layout, 
            &btn_menu_instance);

        // Options
        let vertical_layout =  VerticalLayout::<ButtonOptionsId, LabelId>::new(vec![
            GuiElement::Button(Button::new(
                btn_width, 
                btn_height, 
                btn_boarder,
                ButtonOptionsId::SwitchTexture)),
            GuiElement::Button(Button::new(
                btn_width, 
                btn_height, 
                btn_boarder,
                ButtonOptionsId::SwitchViewPoint)),
            GuiElement::Button(Button::new(
                btn_width, 
                btn_height, 
                btn_boarder,
                ButtonOptionsId::PerformanceGraph)), 
        ]);
        let mut gui_options = Gui::<ButtonOptionsId, LabelId>::new(
            width,
            height,
            vec![
                AlignedElement::new(Alignment::BottomRight, 10, 10 + btn_height + 2*btn_boarder, GuiElement::VerticalLayout(vertical_layout)), 
            ]
        );
        let mut btn_switch_view_point_instance = vertex_texture_shader::Instance::zero();
        let mut btn_switch_texture_instance = vertex_texture_shader::Instance::zero();
        let mut btn_performance_graph_instance = vertex_texture_shader::Instance::zero();
        let events = gui_options.resize(width, height);
        for event in &events {
            match event.element_id {
                ElementId::Button(button_id) => {
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
                ElementId::Label(_) =>  { }
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

        let mouse_event = MouseEvent::Moved{ x, y };

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

    pub fn mouse_pressed(&mut self, pressed: bool) -> (bool, Option<ButtonPressedEvent<ButtonOptionsId>>)
    {
        let mouse_event = if pressed {
                MouseEvent::Pressed
            } else {
                MouseEvent::Released
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

    fn handle_gui_menu_event(&mut self, event: ButtonPressedEvent<ButtonMenuId>) {
        match event.button_id {
            ButtonMenuId::Menu => {
                self.show_submenu = !self.show_submenu;
            },
        }
    }

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>)
    {
        self.btn_vertex_buffer.bind(render_pass);
        self.btn_index_buffer.bind(render_pass);

        self.btn_menu_mesh.bind(render_pass);
        render_pass.draw_indexed(0..self.btn_index_buffer.size(), 0, 0..1);

        if self.show_submenu {
            self.btn_switch_view_point_mesh.bind(render_pass);
            render_pass.draw_indexed(0..self.btn_index_buffer.size(), 0, 0..1);

            self.btn_switch_texture_mesh.bind(render_pass);
            render_pass.draw_indexed(0..self.btn_index_buffer.size(), 0, 0..1);

            self.btn_performance_graph_mesh.bind(render_pass);
            render_pass.draw_indexed(0..self.btn_index_buffer.size(), 0, 0..1);
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
                ElementId::Button(button_id) => {
                    match button_id {
                        ButtonMenuId::Menu => {
                            btn_menu_instance.position.x = event.x as f32;
                            btn_menu_instance.position.y = event.y as f32;
                        },
                    }
                },
                ElementId::Label(_) =>  { }
            }
        }

        let mut btn_switch_view_point_instance = vertex_texture_shader::Instance::zero();
        let mut btn_switch_texture_instance = vertex_texture_shader::Instance::zero();
        let mut btn_performance_graph_instance = vertex_texture_shader::Instance::zero();
        let events = self.gui_options.resize(width, height);
        for event in &events {
            match event.element_id {
                ElementId::Button(button_id) => {
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
                ElementId::Label(_) =>  { }
            }
        }

        self.btn_menu_mesh.update_instance_buffer(queue, &btn_menu_instance);
        self.btn_switch_view_point_mesh.update_instance_buffer(queue, &btn_switch_view_point_instance);
        self.btn_switch_texture_mesh.update_instance_buffer(queue, &btn_switch_texture_instance);
        self.btn_performance_graph_mesh.update_instance_buffer(queue, &btn_performance_graph_instance);
    }
}