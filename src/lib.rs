mod context;
mod font;
mod layer;
mod pipeline;
mod svg;
mod text;
mod transformation;

use log::info;
pub use piet::kurbo;
use piet::kurbo::Size;
pub use piet::*;
pub use svg::Svg;
use svg::SvgStore;
use text::WgpuText;

use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use context::{WgpuImage, WgpuRenderContext};

pub type Piet<'a> = WgpuRenderContext<'a>;

pub type Brush = context::Brush;

pub type PietImage = WgpuImage;

pub struct WgpuRenderer {
    instance: wgpu::Instance,
    device: wgpu::Device,
    surface: wgpu::Surface,
    queue: wgpu::Queue,
    format: wgpu::TextureFormat,
    staging_belt: Rc<RefCell<wgpu::util::StagingBelt>>,
    msaa: wgpu::TextureView,
    size: Size,
    svg_store: SvgStore,

    pipeline: pipeline::Pipeline,
    pub(crate) encoder: Rc<RefCell<Option<wgpu::CommandEncoder>>>,
}

impl WgpuRenderer {
    pub fn new<
        W: raw_window_handle::HasRawWindowHandle + raw_window_handle::HasRawDisplayHandle,
    >(
        window: &W,
    ) -> Result<Self, piet::Error> {
        let backend = wgpu::util::backend_bits_from_env().unwrap_or_else(wgpu::Backends::all);
        let instance = wgpu::Instance::new(backend);
        let surface = unsafe { instance.create_surface(window) };
        let adapter =
            futures::executor::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }))
            .ok_or(piet::Error::NotSupported)?;
        info!("{:?}", adapter.get_info());

        let (device, queue) = futures::executor::block_on(
            adapter.request_device(&wgpu::DeviceDescriptor::default(), None),
        )
        .map_err(|e| piet::Error::BackendError(Box::new(e)))?;

        let format = surface
            .get_supported_formats(&adapter)
            .first()
            .cloned()
            .ok_or(piet::Error::MissingFeature("no supported texture format"))?;

        let staging_belt = wgpu::util::StagingBelt::new(1024);

        let msaa_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Multisampled frame descriptor"),
            size: wgpu::Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 4,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        });
        let msaa = msaa_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let staging_belt = Rc::new(RefCell::new(staging_belt));
        let encoder = Rc::new(RefCell::new(None));
        let device = device;
        let pipeline = pipeline::Pipeline::new(&device, format, Size::ZERO);

        Ok(Self {
            instance,
            device,
            queue,
            surface,
            size: Size::ZERO,
            format,
            staging_belt,
            msaa,
            pipeline,
            svg_store: SvgStore::new(),
            encoder,
        })
    }

    pub fn set_size(&mut self, size: Size) {
        self.size = size;
        let sc_desc = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.format,
            width: size.width as u32,
            height: size.height as u32,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::PreMultiplied,
        };
        self.surface.configure(&self.device, &sc_desc);
        let msaa_texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Multisampled frame descriptor"),
            size: wgpu::Extent3d {
                width: size.width as u32,
                height: size.height as u32,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 4,
            dimension: wgpu::TextureDimension::D2,
            format: self.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        });
        self.msaa = msaa_texture.create_view(&wgpu::TextureViewDescriptor::default());
        self.pipeline.size = size;
    }

    pub fn set_scale(&mut self, scale: f64) {
        self.pipeline.scale = scale;
    }

    pub fn text(&self) -> WgpuText {
        todo!()
    }

    pub(crate) fn ensure_encoder(&mut self) {
        let mut encoder = self.encoder.borrow_mut();
        if encoder.is_none() {
            *encoder = Some(
                self.device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("render"),
                    }),
            );
        }
    }

    pub(crate) fn take_encoder(&mut self) -> wgpu::CommandEncoder {
        self.encoder.take().unwrap()
    }
}

/// A struct provides a `RenderContext` and then can have its bitmap extracted.
pub struct BitmapTarget<'a> {
    phantom: PhantomData<&'a ()>,
}
