//! Implementation of a 2D wave equation
//!
//! It uses the Verlet Method on a 2D grid 
//!



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
    previous: &[[f32;M];N], 
    current: &[[f32;M];N], 
    next: &mut [[f32;M];N],
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

