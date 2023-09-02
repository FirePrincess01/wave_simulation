//! Creates a triangulated 2D grid
//!

use super::super::vertex_color_shader::Vertex as Vertex;
use super::super::vertex_color_shader::Color as Color;

pub struct Grid<const M:usize, const N:usize, const MN: usize> {
    pub vertices: Box<[[Vertex; N]; M]>,
    pub colors: Box<[[Color; N]; M]>,
    pub indices: Vec<u32>,
}

impl<const M:usize, const N:usize, const MN: usize> Grid<M, N, MN> {
    
    /// Creates a triangulated 2D grid
    pub fn new() -> Self {

        // create vertices
        let mut vertices = unsafe {Box::<[[Vertex; N]; M]>::new_zeroed().assume_init()};
        for y in 0..M {
            for x in 0..N {
                vertices[y][x] = Vertex{position: [x as f32, y as f32, 0.0]};
            }
        }

        // create colors
        let color: Color = Color{color: [0.5, 0.5, 0.5]};
        let mut colors = unsafe {Box::<[[Color; N]; M]>::new_zeroed().assume_init()};
        for y in 0..M {
            for x in 0..N {
                colors[y][x] = color;
            }
        }

        // Triangulate the grid
        let indices_size: usize = (M-1) * (N-1) * 6;
        let mut indices = vec![0; indices_size];

        for y in 0..M-1 {
            for x in 0..N-1 {
                let i = (y * (N-1) + x) * 6;

                // A, B, C,
                indices[i+0] = ((y + 0) * N + (x + 0)) as u32;
                indices[i+1] = ((y + 0) * N + (x + 1)) as u32;
                indices[i+2] = ((y + 1) * N + (x + 1)) as u32;

                // C, D, A,
                indices[i+3] = ((y + 1) * N + (x + 1)) as u32;
                indices[i+4] = ((y + 1) * N + (x + 0)) as u32;
                indices[i+5] = ((y + 0) * N + (x + 0)) as u32;
            }
        }

        Self { 
            vertices, 
            colors,
            indices,
        }
    }

    pub fn vertices_slice(&self) -> &[Vertex] {
        let data = unsafe { std::mem::transmute::<&[[Vertex; N]; M],  &[Vertex; MN]>  (&*self.vertices) };
        
        data
    }

    pub fn colors_slice(&self) -> &[Color] {
        let data = unsafe { std::mem::transmute::<&[[Color; N]; M], &[Color; MN]>  (&*self.colors) };

        data
    }

    pub fn indices_slice(&self) -> &[u32] {
        self.indices.as_slice()
    }
}