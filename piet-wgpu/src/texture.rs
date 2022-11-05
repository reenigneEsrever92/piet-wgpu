use kurbo::Vec2;

use crate::config::Config;

pub struct TextureLayout {
    size: Vec2,
    texture_positions: Vec<kurbo::Rect>,
}

impl TextureLayout {
    pub fn new(config: &Config, device: &wgpu::Device) -> Self {
        Self {
            size: config.texture_buffer_dimensions.clone(),
            texture_positions: Vec::new(),
        }
    }

    pub fn find_spot(&self, rect: Vec2) -> kurbo::Rect {
        kurbo::Rect {
            x0: 0.0,
            y0: 0.0,
            x1: 1.0,
            y1: 1.0,
        }
    }
}
