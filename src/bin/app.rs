
fn main() {
    println!("Hello, world!"); 

    pollster::block_on(wave_simulation::run());
}