//! Runs the application on a desktop PC
//! 
//! In the browser, the 'run' method is called from the native event loop
//!

fn main() {
    println!("Hello, world!"); 

    pollster::block_on(wave_simulation::run());
}