

use super::gui::*;
use super::vertex_color_shader;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
enum ButtonOptionsId{
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
    pub color_buffer: vertex_color_shader::ColorBuffer,
    pub instance_buffer: vertex_color_shader::InstanceBuffer,
}

impl BtnMesh {
    pub fn new(device: &wgpu::Device, 
        colors: &[vertex_color_shader::Color],
        instance: &vertex_color_shader::Instance) -> Self
    {
        let color_buffer = vertex_color_shader::ColorBuffer::new(device, colors);
        let instance_raw = instance.to_raw();
        let instance_buffer = vertex_color_shader::InstanceBuffer::new(device, &[instance_raw]);

        Self {
            color_buffer,
            instance_buffer,
        }
    }

    pub fn update_instance_buffer(&mut self, queue: &wgpu::Queue, instance: &vertex_color_shader::Instance)
    {
        let instance_raw = instance.to_raw();
        self.instance_buffer.update(queue, &[instance_raw]);
    }

    pub fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>,) 
    {
        self.color_buffer.bind(render_pass);
        self.instance_buffer.bind(render_pass);
    }
}

pub struct WaveSimGui {
    btn_vertex_buffer: vertex_color_shader::VertexBuffer,
    btn_index_buffer: vertex_color_shader::IndexBuffer,

    // Menu
    gui_menu: Gui<ButtonMenuId, LabelId>,
    // gui_menu_change_position_events: Vec<ChangePositionEvent<ButtonMenuId, LabelId>>,
    btn_menu_mesh: BtnMesh,
    
    // Options
    gui_options: Gui<ButtonOptionsId, LabelId>,
    // gui_options_change_position_events: Vec<ChangePositionEvent<ButtonMenuId, LabelId>>,
    btn_switch_view_point_mesh: BtnMesh,
    btn_switch_texture_mesh: BtnMesh,
    btn_performance_graph_mesh: BtnMesh,

    show_submenu: bool,
}

impl WaveSimGui {
    pub fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {
        let btn_width = 25;
        let btn_height = 25;
        let btn_boarder = 5;

        let btn_vertex_buffer = vertex_color_shader::VertexBuffer::new(device, &Self::vertices(btn_width, btn_height));
        let btn_index_buffer = vertex_color_shader::IndexBuffer::new(device, &Self::indices());

        // let default_instance_position = vertex_color_shader::Instance::zero();
        
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
        let mut btn_menu_instance = vertex_color_shader::Instance::zero();
        let events = gui_menu.resize(width, height);
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

        let btn_menu_mesh = BtnMesh::new(device, &Self::colors([0.0, 0.5, 0.5]), &btn_menu_instance);

        // Options
        let vertical_layout =  VerticalLayout::<ButtonOptionsId, LabelId>::new(vec![
            GuiElement::Button(Button::new(
                btn_width, 
                btn_height, 
                btn_boarder,
                ButtonOptionsId::SwitchViewPoint)),
            GuiElement::Button(Button::new(
                btn_width, 
                btn_height, 
                btn_boarder,
                ButtonOptionsId::SwitchTexture)),
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
                AlignedElement::new(Alignment::BottomRight, 10, 50, GuiElement::VerticalLayout(vertical_layout)), 
            ]
        );
        let mut btn_switch_view_point_instance = vertex_color_shader::Instance::zero();
        let mut btn_switch_texture_instance = vertex_color_shader::Instance::zero();
        let mut btn_performance_graph_instance = vertex_color_shader::Instance::zero();
        let events = gui_options.resize(width, height);
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
        let btn_switch_view_point_mesh = BtnMesh::new(device, &Self::colors([0.5, 0.0, 0.0]), &btn_switch_view_point_instance);
        let btn_switch_texture_mesh = BtnMesh::new(device, &Self::colors([0.0, 0.5, 0.0]), &btn_switch_texture_instance);
        let btn_performance_graph_mesh = BtnMesh::new(device, &Self::colors([0.0, 0.0, 0.5]), &btn_performance_graph_instance);


        Self {
            btn_vertex_buffer,
            btn_index_buffer,

            gui_menu,
            btn_menu_mesh,

            gui_options,
            btn_switch_view_point_mesh,
            btn_switch_texture_mesh,
            btn_performance_graph_mesh,

            show_submenu: true,
        }
    }

    fn colors(color: [f32; 3]) -> [vertex_color_shader::Color; 4]
    {
        let colors: [vertex_color_shader::Color; 4] = [
            vertex_color_shader::Color { color }, // A
            vertex_color_shader::Color { color }, // B
            vertex_color_shader::Color { color }, // C
            vertex_color_shader::Color { color }, // D
        ];

        colors
    }

    fn vertices(width: u32, height: u32) -> [vertex_color_shader::Vertex; 4]
    {
        let width = width as f32;
        let height = height as f32;

        let vertices: [vertex_color_shader::Vertex; 4] = [
            vertex_color_shader::Vertex { position: [0.0, 0.0, 0.0] }, // A
            vertex_color_shader::Vertex { position: [width, 0.0, 0.0] }, // B
            vertex_color_shader::Vertex { position: [width, height, 0.0] }, // C
            vertex_color_shader::Vertex { position: [0.0, height, 0.0] }, // D
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
        let mut btn_menu_instance = vertex_color_shader::Instance::zero();
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

        let mut btn_switch_view_point_instance = vertex_color_shader::Instance::zero();
        let mut btn_switch_texture_instance = vertex_color_shader::Instance::zero();
        let mut btn_performance_graph_instance = vertex_color_shader::Instance::zero();
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