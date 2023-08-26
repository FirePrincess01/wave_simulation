#![feature(new_uninit)]


mod wave_equation;
mod wgpu_renderer;
mod vertex_color_shader;
pub mod wave_simulation;

fn main() {

    const N: usize = 100;
    const M: usize = 200;

    let mut grid1 = unsafe {Box::<[[f32; N]; M]>::new_zeroed().assume_init()};
    let mut grid2 = unsafe {Box::<[[f32; N]; M]>::new_zeroed().assume_init()};
    let mut grid3 = unsafe {Box::<[[f32; N]; M]>::new_zeroed().assume_init()};

    const H: f32 = 1.0;
    const DELTA_T: f32 = 0.0001;



    let mut previous =  & mut *grid1;
    let mut current = & mut *grid2;
    let mut next = & mut *grid3;

    for _i in 0..1000 
    {
        wave_equation::wave_equation_step(&previous, &current, & mut next, DELTA_T, H);

        let tmp = previous;
        previous = current;
        current = next;
        next = tmp;
    }


    pollster::block_on(wave_simulation::run());


    println!("Hello, world!"); 
}
