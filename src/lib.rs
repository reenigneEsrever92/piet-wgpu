mod data;
mod error;
pub mod immediate;
mod renderer;
mod settings;
mod text;

use std::{borrow::Cow, ops::Deref};

use image::{DynamicImage, GenericImageView};
pub use piet::kurbo::*;
pub use piet::*;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use renderer::WgpuRenderer;
use text::{WgpuText, WgpuTextLayout};

pub struct PietWgpu<T>
where
    T: WgpuRenderer + Sized,
{
    pub renderer: T,
    pub window: WgpuWindow,
}

impl<T> PietWgpu<T>
where
    T: WgpuRenderer,
{
    pub fn new<W: HasRawWindowHandle + HasRawDisplayHandle>(
        window: &W,
        renderer: T,
        width: u32,
        height: u32,
        scale: f64,
    ) -> Self {
        let window = WgpuWindow::new(width, height, scale);

        let mut piet_wgpu = Self { renderer, window };
        piet_wgpu.set_size(width, height);
        piet_wgpu
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.window.width = width;
        self.window.height = height;

        self.renderer.set_size(width, height);
    }

    pub fn set_scale(&mut self, scale: f64) {
        self.window.scale = scale;
    }
}

pub struct WgpuWindow {
    width: u32,
    height: u32,
    scale: f64,
}

impl WgpuWindow {
    fn new(width: u32, height: u32, scale: f64) -> Self {
        Self {
            width,
            height,
            scale,
        }
    }
}

#[derive(Clone)]
pub enum WgpuBrush {
    Solid(Color),
    Gradient(FixedGradient),
}

impl<T: WgpuRenderer> IntoBrush<PietWgpu<T>> for WgpuBrush {
    fn make_brush<'a>(
        &'a self,
        piet: &mut PietWgpu<T>,
        bbox: impl FnOnce() -> kurbo::Rect,
    ) -> std::borrow::Cow<'a, <PietWgpu<T> as RenderContext>::Brush> {
        Cow::Owned(WgpuBrush::Solid(Color::grey(0.5))) // TODO
    }
}

#[derive(Clone)]
pub struct WgpuImage {
    image: DynamicImage,
}

impl WgpuImage {
    fn from_bytes(bytes: &[u8]) -> Self {
        let image = image::load_from_memory(bytes).unwrap();
        Self { image }
    }
}

impl piet::Image for WgpuImage {
    fn size(&self) -> Size {
        let (width, height) = self.image.dimensions();

        Size {
            width: width.into(),
            height: height.into(),
        }
    }
}

impl<T: WgpuRenderer> piet::RenderContext for PietWgpu<T> {
    type Brush = WgpuBrush;

    type Text = WgpuText;

    type TextLayout = WgpuTextLayout;

    type Image = WgpuImage;

    fn status(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn solid_brush(&mut self, color: Color) -> Self::Brush {
        WgpuBrush::Solid(color)
    }

    fn gradient(&mut self, gradient: impl Into<FixedGradient>) -> Result<Self::Brush, Error> {
        Ok(WgpuBrush::Gradient(gradient.into()))
    }

    fn clear(&mut self, region: impl Into<Option<kurbo::Rect>>, color: Color) {
        let (r, g, b, a) = color.as_rgba();
        let region: Option<kurbo::Rect> = region.into();
        match region {
            Some(rect) => todo!(),
            None => self.renderer.clear_all(wgpu::Color { r, g, b, a }),
        }
    }

    fn stroke(&mut self, shape: impl kurbo::Shape, brush: &impl IntoBrush<Self>, width: f64) {
        todo!()
    }

    fn stroke_styled(
        &mut self,
        shape: impl kurbo::Shape,
        brush: &impl IntoBrush<Self>,
        width: f64,
        style: &StrokeStyle,
    ) {
        todo!()
    }

    fn fill(&mut self, shape: impl Shape, brush: &impl IntoBrush<Self>) {
        let brush: std::borrow::Cow<'_, WgpuBrush> =
            brush.make_brush(self, || Rect::new(0.0, 0.0, 0.0, 0.0)); // TODO implement bounding box

        if let Some(rect) = shape.as_rect() {
            self.renderer.fill_rect(rect, brush.deref());
        }
    }

    fn fill_even_odd(&mut self, shape: impl kurbo::Shape, brush: &impl IntoBrush<Self>) {
        todo!()
    }

    fn clip(&mut self, shape: impl kurbo::Shape) {
        todo!()
    }

    fn text(&mut self) -> &mut Self::Text {
        todo!()
    }

    fn draw_text(&mut self, layout: &Self::TextLayout, pos: impl Into<kurbo::Point>) {
        todo!()
    }

    fn save(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn restore(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn finish(&mut self) -> Result<(), Error> {
        self.renderer
            .finish()
            .map_err(|e| piet::Error::BackendError(Box::new(e)))
    }

    fn transform(&mut self, transform: kurbo::Affine) {
        todo!()
    }

    fn make_image(
        &mut self,
        width: usize,
        height: usize,
        buf: &[u8],
        format: ImageFormat,
    ) -> Result<Self::Image, Error> {
        todo!()
    }

    fn draw_image(
        &mut self,
        image: &Self::Image,
        dst_rect: impl Into<kurbo::Rect>,
        interp: InterpolationMode,
    ) {
        todo!()
    }

    fn draw_image_area(
        &mut self,
        image: &Self::Image,
        src_rect: impl Into<kurbo::Rect>,
        dst_rect: impl Into<kurbo::Rect>,
        interp: InterpolationMode,
    ) {
        todo!()
    }

    fn capture_image_area(
        &mut self,
        src_rect: impl Into<kurbo::Rect>,
    ) -> Result<Self::Image, Error> {
        todo!()
    }

    fn blurred_rect(&mut self, rect: kurbo::Rect, blur_radius: f64, brush: &impl IntoBrush<Self>) {
        todo!()
    }

    fn current_transform(&self) -> kurbo::Affine {
        todo!()
    }
}
