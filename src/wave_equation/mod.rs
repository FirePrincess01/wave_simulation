//! Implementation of a 2D wave equation
//!
//! It uses the Verlet Method on a 2D grid 
//!

pub struct WaveEquation<const M:usize, const N:usize> {

    previous: Box<[[f32; N]; M]>,
    current: Box<[[f32; N]; M]>,
    next: Box<[[f32; N]; M]>,
    forces: Box<[[f32; N]; M]>,
    h: f32,
    delta_t: f32,
}

impl<const M:usize, const N:usize>  WaveEquation<M, N>{

    pub fn new() -> Self {
        let previous: Box<[[f32; N]; M]> = unsafe {Box::<[[f32; N]; M]>::new_zeroed().assume_init()};
        let current: Box<[[f32; N]; M]> = unsafe {Box::<[[f32; N]; M]>::new_zeroed().assume_init()};
        let next: Box<[[f32; N]; M]> = unsafe {Box::<[[f32; N]; M]>::new_zeroed().assume_init()};   
        let forces: Box<[[f32; N]; M]> = unsafe {Box::<[[f32; N]; M]>::new_zeroed().assume_init()};   
        const H: f32 = 0.125;
        const DELTA_T: f32 = 0.05;

        Self {
            previous,
            current,
            next,
            forces,
            h: H,
            delta_t: DELTA_T,
        }
    }

    pub fn step(&mut self) {

        let previous: &[[f32; N]; M] = &*self.previous;
        let current =  &*self.current;
        let next = &mut *self.next;

        let delta_t = self.delta_t;
        let h = self.h;

        let d = 0.999;

        for y in 0..M {
            for x in 0..N {
                next[y][x] = d *(2.0 * current[y][x] - previous[y][x] + 
                (delta_t*delta_t) / (h*h) * 
                (current[y][x.saturating_add_signed(-1)] + 
                current[y][(x+1).min(N-1)] + 
                current[y.saturating_add_signed(-1)][x] + 
                current[(y+1).min(M-1)][x] - 
                4.0*current[y][x]
                + self.forces[y][x]));
                
                self.forces[y][x] = 0.0;
            }
        }
        
        std::mem::swap(&mut self.previous,&mut self.current);
        std::mem::swap(&mut self.current,&mut self.next);

    }

    // Adds forces to the position of y and x to the grid
    pub fn add_impulse(&mut self, y:f32, x:f32) {
        // check bounds
        if y <= 0.0 || y >= (M-1) as f32 ||
        x <= 0.0 || x >= (N-1) as f32 {
            return;
        }
        let force_strength = 4.0;

        let w_x = x - x.floor();
        let w_y = y - y.floor();
        let x_i = x.floor() as usize;
        let y_i = y.floor() as usize;

        //Add the force to position
        self.forces[y_i][x_i] += force_strength * (1.-w_y) * (1.-w_x);
        self.forces[y_i+1][x_i] += force_strength * (w_y) * (1.-w_x);
        self.forces[y_i][x_i+1] += force_strength * (1.-w_y) * (w_x);
        self.forces[y_i+1][x_i+1] += force_strength * (w_y) * (w_x);
    }

    pub fn get_current(&self) -> &[[f32; N]; M] {
        &self.current
    }


}

