use crate::data::{Primitive, Vertex};

pub struct Config {
    pub vertex_buffer_size: u64,
    pub index_buffer_size: u64,
    pub texture_buffer_dimensions: Dimensions,
    pub primitve_buffer_size: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            vertex_buffer_size: std::mem::size_of::<Vertex>() as u64 * 1024, // one Vertex is currently 28 bytes
            index_buffer_size: std::mem::size_of::<u16>() as u64 * 4096,     // indicies are u16
            texture_buffer_dimensions: Dimensions(2048, 2048),
            primitve_buffer_size: std::mem::size_of::<Primitive>() as u64 * 512,
        }
    }
}

pub struct Dimensions(pub u32, pub u32);
