use lyon::{
    lyon_tessellation::{
        BuffersBuilder, FillOptions, FillTessellator, FillVertexConstructor, VertexBuffers,
    },
    math::point,
    path::Path,
};
use piet::{kurbo::Rect, IntoBrush};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

use crate::PietWgpu;

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
    pub fn new<W: HasRawWindowHandle + HasRawDisplayHandle>(
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

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.surface_config.width = width;
        self.surface_config.height = height;

        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn fill_rect(&mut self, rect: Rect, brush: &impl IntoBrush<PietWgpu>) {
        let mut fill_tess = FillTessellator::new();

        let mut builder = Path::builder();

        builder.begin(point(rect.x0 as f32, rect.y0 as f32));
        builder.line_to(point(rect.x0 as f32, rect.y1 as f32));
        builder.line_to(point(rect.x1 as f32, rect.y1 as f32));
        builder.line_to(point(rect.x1 as f32, rect.y0 as f32));

        builder.close();

        let path = builder.build();

        let mut geometry: VertexBuffers<GpuVertex, u16> = VertexBuffers::new();
        let fill_prim_id = 1;

        fill_tess
            .tessellate(
                &path,
                &FillOptions::tolerance(0.02).with_fill_rule(lyon::tessellation::FillRule::NonZero),
                &mut BuffersBuilder::new(&mut geometry, WithId(fill_prim_id as u32)),
            )
            .unwrap();

        let mut cpu_primitives = Vec::with_capacity(100);
        for _ in 0..100 {
            cpu_primitives.push(Primitive {
                color: [1.0, 0.0, 0.0, 1.0],
                z_index: 0,
                width: 0.0,
                translate: [0.0, 0.0],
                angle: 0.0,
                ..Primitive::DEFAULT
            });
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Primitive {
    color: [f32; 4],
    translate: [f32; 2],
    z_index: i32,
    width: f32,
    angle: f32,
    scale: f32,
    _pad1: i32,
    _pad2: i32,
}

impl Primitive {
    const DEFAULT: Self = Primitive {
        color: [0.0; 4],
        translate: [0.0; 2],
        z_index: 0,
        width: 0.0,
        angle: 0.0,
        scale: 1.0,
        _pad1: 0,
        _pad2: 0,
    };
}

#[repr(C)]
#[derive(Copy, Clone)]
struct GpuVertex {
    position: [f32; 2],
    normal: [f32; 2],
    prim_id: u32,
}

pub struct WithId(pub u32);

impl FillVertexConstructor<GpuVertex> for WithId {
    fn new_vertex(&mut self, vertex: lyon::tessellation::FillVertex) -> GpuVertex {
        GpuVertex {
            position: vertex.position().to_array(),
            normal: [0.0, 0.0],
            prim_id: self.0,
        }
    }
}
