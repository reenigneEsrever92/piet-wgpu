mod renderer;
mod text;

use std::collections::VecDeque;

use lyon::{
    geom::point,
    lyon_tessellation::{
        BuffersBuilder, FillOptions, FillTessellator, FillVertexConstructor, VertexBuffers,
    },
    path::Path,
};
pub use piet::kurbo::*;
pub use piet::*;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use renderer::WgpuRenderer;
use text::{WgpuText, WgpuTextLayout};

pub struct PietWgpu {
    pub renderer: WgpuRenderer,
    pub window: WgpuWindow,
}

impl PietWgpu {
    pub fn new<W: HasRawWindowHandle + HasRawDisplayHandle>(
        window: &W,
        width: u32,
        height: u32,
        scale: f64,
    ) -> Self {
        let renderer = WgpuRenderer::new(window, width, height, scale).unwrap();
        let window = WgpuWindow::new(width, height, scale);

        Self { renderer, window }
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

impl IntoBrush<PietWgpu> for WgpuBrush {
    fn make_brush<'a>(
        &'a self,
        piet: &mut PietWgpu,
        bbox: impl FnOnce() -> kurbo::Rect,
    ) -> std::borrow::Cow<'a, <PietWgpu as RenderContext>::Brush> {
        todo!()
    }
}

#[derive(Clone)]
pub struct WgpuImage;

impl piet::Image for WgpuImage {
    fn size(&self) -> Size {
        todo!()
    }
}

impl piet::RenderContext for PietWgpu {
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
        todo!()
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
        if let Some(rect) = shape.as_rect() {
            self.renderer.fill_rect(rect, brush);
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
        todo!()
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
