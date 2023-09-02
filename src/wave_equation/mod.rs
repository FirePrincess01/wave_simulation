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

    x_old: f32,
    y_old: f32,
    mouse_interupted: bool,
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
            x_old: 0.,
            y_old: 0.,
            mouse_interupted: true,
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
            self.mouse_interupted = true;
            return;
        }
        let force_strength = 4.0;
        let same_quad = x.floor() == self.x_old.floor() && y.floor() == self.y_old.floor();

        if self.mouse_interupted || same_quad {
            let w_x = x - x.floor();
            let w_y = y - y.floor();
            let x_i = x.floor() as usize;
            let y_i = y.floor() as usize;

            //Add the force to position
            self.forces[y_i][x_i] += force_strength * (1.-w_y) * (1.-w_x);
            self.forces[y_i+1][x_i] += force_strength * (w_y) * (1.-w_x);
            self.forces[y_i][x_i+1] += force_strength * (1.-w_y) * (w_x);
            self.forces[y_i+1][x_i+1] += force_strength * (w_y) * (w_x);
        } else {
            self.add_line_force(x, y, force_strength);
        }
        self.x_old = x;
        self.y_old = y;
        self.mouse_interupted = false;
    }

    // Integral along the line on the unit square for f=x*y
    fn unit_square_integral(&self, x_in: f32, y_in: f32, x_out: f32, y_out: f32) -> f32 {
        (2. * x_in * y_in + x_in * y_out + y_in * x_out + 2. * x_out * y_out) / 6.
    }

    fn add_line_force_to_square(&mut self, x_i: usize, y_i: usize, total_lenght: f32, force: f32, x_in: f32, y_in: f32, x_out: f32, y_out: f32) {
        let length = f32::sqrt((x_out-x_in) * (x_out-x_in) + (y_out-y_in) * (y_out-y_in));
        let factor = length * force / (total_lenght);

        self.forces[y_i][x_i]       += factor * self.unit_square_integral(1.-x_in, 1.-y_in, 1.-x_out, 1.-y_out);
        self.forces[y_i+1][x_i]     += factor * self.unit_square_integral(1.-x_in, y_in, 1.-x_out, y_out);
        self.forces[y_i][x_i+1]     += factor * self.unit_square_integral(x_in, 1.-y_in, x_out, 1.-y_out);
        self.forces[y_i+1][x_i+1]   += factor * self.unit_square_integral(x_in, y_in, x_out, y_out);
    }

    fn add_line_force(&mut self, x_new: f32, y_new: f32, force: f32) {
        let reverse = x_new < self.x_old;
        let mut x_0= self.x_old;
        let mut y_0= self.y_old;
        let mut x_1= x_new;
        let mut y_1= y_new;
        // Start from lower x end
        if reverse{
            std::mem::swap(&mut x_0, &mut x_1);
            std::mem::swap(&mut y_0, &mut y_1);
        }

        let total_lenght = ((x_1 - x_0) * (x_1 - x_0) + (y_1 - y_0) * (y_1 - y_0)).sqrt();
        if x_1 == x_0 {return;} //currently not able to handle vertical lines
        let inclination = (y_1 - y_0) / (x_1 - x_0);

        // index of the current and final square
        let mut x_i = x_0.floor() as usize;
        let mut y_i = y_0.floor() as usize;
        let x_final = x_1.floor() as usize;
        let y_final = y_1.floor() as usize;
        let steps = x_final - x_i + y_final.abs_diff(y_i);

        // current and next line intersection in local coordinates
        let mut x_curr = x_0 - x_0.floor();
        let mut y_curr = y_0 - y_0.floor();
        let mut x_cut;
        let mut y_cut;

        for _j in 0..steps {
            x_cut = 1.;
            y_cut = y_curr + inclination * (x_cut - x_curr);
            //successful cut on right border
            if y_cut < 1. && y_cut > 0. {
                self.add_line_force_to_square(x_i, y_i, total_lenght, force, x_curr, y_curr, x_cut, y_cut);
                x_i += 1;
                x_cut = 0.;
            }
            else  {
                y_cut = if inclination > 0. {1.} else {0.};
                x_cut = x_curr + (y_cut - y_curr) / inclination;
                self.add_line_force_to_square(x_i, y_i, total_lenght, force, x_curr, y_curr, x_cut, y_cut);
                if inclination > 0. {
                    y_i += 1;
                    y_cut = 0.;
                }
                else {
                    y_i -= 1;
                    y_cut = 1.;
                }
            }
            x_curr = x_cut;
            y_curr = y_cut;
        }
        //final square
        self.add_line_force_to_square(x_i, y_i, total_lenght, force, x_curr, y_curr, x_1 - x_1.floor(), y_1 - y_1.floor());
    }

    pub fn interupt_mouse(&mut self) {self.mouse_interupted = true}

    pub fn get_current(&self) -> &[[f32; N]; M] {
        &self.current
    }


}

