use image::{DynamicImage, GenericImageView, RgbaImage};
use kurbo::{Rect, Size, Vec2};

use crate::error::Result;

#[derive(Clone)]
pub struct WgpuImage {
    dynamic: DynamicImage,
}

impl WgpuImage {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let image = image::load_from_memory(bytes).unwrap();
        Self { dynamic: image }
    }
}

impl piet::Image for WgpuImage {
    fn size(&self) -> Size {
        let (width, height) = self.dynamic.dimensions();

        Size {
            width: width.into(),
            height: height.into(),
        }
    }
}

struct WgpuImageAtlas {
    size: Vec2,
    images: Vec<Rect>,
    buffer: RgbaImage,
}

impl WgpuImageAtlas {
    fn new(size: Vec2) -> Self {
        Self {
            size,
            buffer: RgbaImage::new(size.x as u32, size.y as u32),
        }
    }

    fn push(&mut self, image: WgpuImage) -> Result<Rect> {
        self.images
    }
}
