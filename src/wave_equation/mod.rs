//! Implementation of a 2D wave equation
//!
//! It uses the Verlet Method on a 2D grid 
//!

pub struct WaveEquation<const M:usize, const N:usize> {

    grid0: Box<[[f32; N]; M]>,
    grid1: Box<[[f32; N]; M]>,
    grid2: Box<[[f32; N]; M]>,
    h: f32,
    delta_t: f32,

    index: usize,
}

impl<const M:usize, const N:usize>  WaveEquation<M, N>{

    pub fn new() -> Self {
        let grid0: Box<[[f32; N]; M]> = unsafe {Box::<[[f32; N]; M]>::new_zeroed().assume_init()};
        let grid1: Box<[[f32; N]; M]> = unsafe {Box::<[[f32; N]; M]>::new_zeroed().assume_init()};
        let grid2: Box<[[f32; N]; M]> = unsafe {Box::<[[f32; N]; M]>::new_zeroed().assume_init()};   
        const H: f32 = 1.0;
        const DELTA_T: f32 = 0.1;

        Self {
            grid0,
            grid1,
            grid2,
            h: H,
            delta_t: DELTA_T,

            index: 0,
        }
    }

    pub fn step(&mut self) {

        // index == 0
        let mut previous: &[[f32; N]; M] = &*self.grid0;
        let mut current =  &*self.grid1;
        let mut next = &mut *self.grid2;

        if self.index == 1 {
            previous = &*self.grid1;
            current = &*self.grid2;
            next = &mut *self.grid0;
        }
        else if self.index == 2 {
            previous = &*self.grid2;
            current = &*self.grid0;
            next = &mut *self.grid1;
        }

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
                4.0*current[y][x]));
            }
        }

        self.index = (self.index + 1) % 3;

    }

    pub fn get_current(&self) -> &[[f32; N]; M]  {
        let mut current =  &*self.grid1;

        if self.index == 1 {
            current = &*self.grid2;
        }
        else if self.index == 2 {
            current = &*self.grid0;
        }

        current
    }

    pub fn get_current_mut(&mut self) -> &mut [[f32; N]; M]  {
        let mut current =  &mut *self.grid1;

        if self.index == 1 {
            current = &mut *self.grid2;
        }
        else if self.index == 2 {
            current = &mut *self.grid0;
        }

        current
    }

    pub fn get_previous_mut(&mut self) -> &mut [[f32; N]; M]  {
        let mut previous = &mut *self.grid0;

        if self.index == 1 {
            previous = &mut *self.grid1;
        }
        else if self.index == 2 {
            previous = &mut *self.grid2;
        }

        previous
    }
}

/// Performs a single step of the wave equation
/// 
/// ### Arguments
///
/// * `previous` - A 2D grid calculated in the previous step
/// * `current` - A 2D grid calculated in the latest step
/// * `next` - A 2D grid and the resutl of this calculation
/// * `delta_t` - Time between this and the last calculation
/// * `delta_h` - Distance between the points in the grid
/// * 
/// 
pub fn wave_equation_step<const M:usize, const N:usize>(
    previous: &[[f32;N];M], 
    current:  &[[f32;N];M], 
    next: &mut [[f32;N];M],
    delta_t: f32,
    delta_h: f32)
{
    for y in 0..M {
        for x in 0..N {
            next[y][x] = 2.0 * current[y][x] - previous[y][x] + 
            (delta_t*delta_t) / (delta_h*delta_h) * 
            (current[y][x.saturating_add_signed(-1)] + 
            current[y][(x+1).min(N-1)] + 
            current[y.saturating_add_signed(-1)][x] + 
            current[(y+1).min(M-1)][x] - 
            4.0*current[y][x]);
        }
    }
}

