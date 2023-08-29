#![feature(new_uninit)]


mod wave_equation;
mod wgpu_renderer;
mod vertex_color_shader;
mod wave_simulation;
mod geometry;

fn main() {

    pollster::block_on(wave_simulation::run());

    println!("Hello, world!"); 
}
