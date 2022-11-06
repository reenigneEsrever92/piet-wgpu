use kurbo::Vec2;

use crate::data::{Primitive, Vertex};

#[derive(Clone, Debug)]
pub struct Config {
    pub vertex_buffer_size: u64,
    pub index_buffer_size: u64,
    pub texture_buffer_dimensions: Vec2,
    pub primitve_buffer_size: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            vertex_buffer_size: std::mem::size_of::<Vertex>() as u64 * 1024, // one Vertex is currently 28 bytes
            index_buffer_size: std::mem::size_of::<u16>() as u64 * 4096,     // indicies are u16
            texture_buffer_dimensions: Vec2 {
                x: 2048.0,
                y: 512.0,
            },
            primitve_buffer_size: std::mem::size_of::<Primitive>() as u64 * 512,
        }
    }
}
