use crate::data::Vertex;

pub struct Config {
    pub vertex_buffer_size: u64,
    pub index_buffer_size: u64,
    pub texture_buffer_size: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            vertex_buffer_size: std::mem::size_of::<Vertex>() as u64 * 1000, // one Vertex is currently 28 bytes
            index_buffer_size: std::mem::size_of::<u16>() as u64 * 10000,    // indicies are u16
            texture_buffer_size: 4 * 256 * 256,                              // only fits example
        }
    }
}
