mod context;
mod font;
mod layer;
mod pipeline;
mod svg;
mod text;
mod transformation;

use std::collections::VecDeque;

pub use piet::kurbo::*;
pub use piet::*;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

pub struct PietWgpu {
    pub renderer: WgpuRenderer,
    pub cache: WgpuRenderPrimitives,
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
        let cache = WgpuRenderPrimitives::new();
        let window = WgpuWindow::new(width, height, scale);

        Self {
            renderer,
            cache,
            window,
        }
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.window.width = width;
        self.window.height = height;

        self.renderer.surface_config.width = self.window.width;
        self.renderer.surface_config.height = self.window.height;

        self.renderer
            .surface
            .configure(&self.renderer.device, &self.renderer.surface_config);
    }

    pub fn set_scale(&mut self, scale: f64) {
        self.window.scale = scale;
    }
}

pub struct WgpuTransformer {
    transformations: VecDeque<WgpuTransformation>,
}

impl WgpuTransformer {
    fn new() -> Self {
        Self {
            transformations: VecDeque::new(),
        }
    }

    fn transform<S: Shape>(&mut self, s: S) -> WgpuTransformation<S> {}
}

pub struct WgpuTransformation;

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

#[repr(C)]
pub struct WgpuPrimitive {
    transform: [[f64; 4]; 4],
    color: [f64; 4],
}

pub struct WgpuRenderPrimitives {
    primitives: Vec<WgpuPrimitive>,
}

impl WgpuRenderPrimitives {
    fn new() -> Self {
        Self {
            primitives: Vec::new(),
        }
    }
}

pub struct WgpuRenderer {
    instance: wgpu::Instance,
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_config: wgpu::SurfaceConfiguration,
}
static_assertions::assert_impl_all!(WgpuRenderer: Send, Sync);

impl WgpuRenderer {
    fn new<W: HasRawWindowHandle + HasRawDisplayHandle>(
        window: &W,
        width: u32,
        height: u32,
        scale: f64,
    ) -> Result<Self, piet::Error> {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter =
            futures::executor::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }))
            .unwrap();

        let (device, queue) = futures::executor::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None, // Trace path
        ))
        .unwrap();

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface.get_supported_alpha_modes(&adapter)[0],
        };

        Ok(Self {
            instance,
            surface,
            adapter,
            device,
            queue,
            surface_config,
        })
    }

    pub fn text(&self) -> WgpuText {
        todo!()
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
pub struct WgpuText;

impl piet::Text for WgpuText {
    type TextLayoutBuilder = WgpuTextLayoutBuilder;

    type TextLayout = WgpuTextLayout;

    fn font_family(&mut self, family_name: &str) -> Option<FontFamily> {
        todo!()
    }

    fn load_font(&mut self, data: &[u8]) -> Result<FontFamily, Error> {
        todo!()
    }

    fn new_text_layout(&mut self, text: impl TextStorage) -> Self::TextLayoutBuilder {
        todo!()
    }
}

pub struct WgpuTextLayoutBuilder;

impl piet::TextLayoutBuilder for WgpuTextLayoutBuilder {
    type Out = WgpuTextLayout;

    fn max_width(self, width: f64) -> Self {
        todo!()
    }

    fn alignment(self, alignment: TextAlignment) -> Self {
        todo!()
    }

    fn default_attribute(self, attribute: impl Into<TextAttribute>) -> Self {
        todo!()
    }

    fn range_attribute(
        self,
        range: impl std::ops::RangeBounds<usize>,
        attribute: impl Into<TextAttribute>,
    ) -> Self {
        todo!()
    }

    fn build(self) -> Result<Self::Out, Error> {
        todo!()
    }
}

#[derive(Clone)]
pub struct WgpuTextLayout;

impl piet::TextLayout for WgpuTextLayout {
    fn size(&self) -> Size {
        todo!()
    }

    fn trailing_whitespace_width(&self) -> f64 {
        todo!()
    }

    fn image_bounds(&self) -> kurbo::Rect {
        todo!()
    }

    fn text(&self) -> &str {
        todo!()
    }

    fn line_text(&self, line_number: usize) -> Option<&str> {
        todo!()
    }

    fn line_metric(&self, line_number: usize) -> Option<LineMetric> {
        todo!()
    }

    fn line_count(&self) -> usize {
        todo!()
    }

    fn hit_test_point(&self, point: kurbo::Point) -> HitTestPoint {
        todo!()
    }

    fn hit_test_text_position(&self, idx: usize) -> HitTestPosition {
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

    fn fill(&mut self, shape: impl kurbo::Shape, brush: &impl IntoBrush<Self>) {
        self.cache.primitives.push(WgpuPrimitive {
            transform: [],
            color: (),
        })
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
