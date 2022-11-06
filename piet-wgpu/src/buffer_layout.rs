use image::GenericImageView;
use kurbo::{Rect, Vec2};

use crate::config::Config;

pub struct BufferLayout2D {
    size: Vec2,
    textures: Vec<Rect>,
}

impl BufferLayout2D {
    pub fn new(config: &Config) -> Self {
        Self {
            size: config.texture_buffer_dimensions,
            textures: Vec::new(),
        }
    }

    pub fn search(&self, size: Vec2) -> Option<Rect> {
        Some(
            self.textures
                .last()
                .map(|last| Rect {
                    x0: last.x1,
                    y0: 0.0,
                    x1: last.x1 + size.x,
                    y1: size.y,
                })
                .unwrap_or_else(|| Rect {
                    x0: 0.0,
                    y0: 0.0,
                    x1: size.x,
                    y1: size.y,
                }),
        )
    }

    pub fn search_and_allocate(&mut self, size: Vec2) -> Option<Rect> {
        match self.search(size) {
            Some(rect) => {
                self.textures.push(rect);
                Some(rect)
            }
            None => None,
        }
    }
}
