use lyon::{
    lyon_tessellation::{BuffersBuilder, FillOptions, FillTessellator, VertexBuffers},
    math::point,
    path::Path,
};
use piet::{kurbo::Rect, IntoBrush};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BufferUsages,
};

use crate::{
    config::Config,
    data::{Vertex, VertexBuilder},
    error::Result,
    renderer::WgpuRenderer,
    PietWgpu, WgpuBrush, WgpuImage,
};

pub type ImmediateRenderer = PietWgpu<WgpuImmediateRenderer>;

pub struct WgpuImmediateRenderer {
    instance: wgpu::Instance,
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    encoder: wgpu::CommandEncoder,
    vertex_buffer: wgpu::Buffer,
    num_vertecies: u64,
    index_buffer: wgpu::Buffer,
    num_indecies: u64,
    // image_buffer: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
    surface_config: wgpu::SurfaceConfiguration,
    images: Vec<WgpuImage>,
    clear_color: wgpu::Color,
}
static_assertions::assert_impl_all!(WgpuImmediateRenderer: Send, Sync);

impl WgpuImmediateRenderer {
    pub fn new<W: HasRawWindowHandle + HasRawDisplayHandle>(
        window: &W,
        width: u32,
        height: u32,
        scale: f64,
    ) -> Result<Self> {
        Self::from_settings(window, width, height, scale, Default::default())
    }
    pub fn from_settings<W: HasRawWindowHandle + HasRawDisplayHandle>(
        window: &W,
        width: u32,
        height: u32,
        scale: f64,
        config: Config,
    ) -> Result<Self> {
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
                limits: wgpu::Limits::default(),
                label: None,
            },
            None, // Trace path
        ))
        .unwrap();

        let encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };

        let simple_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Simple vs"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./../shaders/simple.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x4],
            // shorthand for:
            // &[
            //     wgpu::VertexAttribute {
            //         offset: 0,
            //         shader_location: 0,
            //         format: wgpu::VertexFormat::Float32x3,
            //     },
            //     wgpu::VertexAttribute {
            //         offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
            //         shader_location: 1,
            //         format: wgpu::VertexFormat::Float32x4,
            //     },
            // ],
        };

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: config.vertex_buffer_size,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: true,
        });

        // let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: Some("Vertex Buffer"),
        //     contents: bytemuck::cast_slice(&[]),
        //     usage: wgpu::BufferUsages::VERTEX,
        // });

        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Index Buffer"),
            size: config.index_buffer_size,
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
            mapped_at_creation: true,
        });

        // let index_buffer = self
        //     .device
        //     .create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //         label: Some("Index Buffer"),
        //         contents: bytemuck::cast_slice(&self.geometry_buffer.indices),
        //         usage: wgpu::BufferUsages::INDEX,
        //     });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &simple_shader,
                entry_point: "vs_main",
                buffers: &[vertex_buffer_layout],
            },
            fragment: Some(wgpu::FragmentState {
                module: &simple_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let clear_color = wgpu::Color::WHITE;

        let images = Vec::new();

        Ok(Self {
            instance,
            surface,
            adapter,
            device,
            queue,
            encoder,
            pipeline,
            surface_config,
            vertex_buffer,
            num_vertecies: 0,
            index_buffer,
            num_indecies: 0,
            images,
            clear_color,
        })
    }

    fn append_geometry(&mut self, geometry: VertexBuffers<Vertex, u16>) {
        // let vertecies = self.device.create_buffer_init(&BufferInitDescriptor {
        //     label: Some("Copy Buffer"),
        //     usage: BufferUsages::COPY_SRC,
        //     contents: bytemuck::cast_slice(&geometry.vertices),
        // });

        // let indicies = self.device.create_buffer_init(&BufferInitDescriptor {
        //     label: Some("Copy Buffer"),
        //     usage: BufferUsages::COPY_SRC,
        //     contents: bytemuck::cast_slice(&geometry.indices),
        // });

        // self.vertex_buffer
        //     .slice(
        //         self.num_vertecies * std::mem::size_of::<Vertex>() as u64
        //             ..(self.num_vertecies + geometry.vertices.len() as u64)
        //                 * std::mem::size_of::<Vertex>() as u64,
        //     )
        //     .get_mapped_range_mut()
        //     .copy_from_slice(bytemuck::cast_slice(&geometry.vertices));

        // self.index_buffer
        //     .slice(
        //         self.num_indecies * std::mem::size_of::<u16>() as u64
        //             ..(self.num_indecies + geometry.indices.len() as u64)
        //                 * std::mem::size_of::<u16>() as u64,
        //     )
        //     .get_mapped_range_mut()
        //     .copy_from_slice(bytemuck::cast_slice(&geometry.indices));

        // self.encoder.copy_buffer_to_buffer(
        //     &vertecies,
        //     0,
        //     &self.vertex_buffer,
        //     std::mem::size_of::<Vertex>() as u64 * self.num_vertecies,
        //     std::mem::size_of::<Vertex>() as u64 * geometry.vertices.len() as u64,
        // );

        // self.encoder.copy_buffer_to_buffer(
        //     &indicies,
        //     0,
        //     &self.index_buffer,
        //     std::mem::size_of::<u16>() as u64 * self.num_indecies,
        //     std::mem::size_of::<u16>() as u64 * geometry.indices.len() as u64,
        // );

        // self.num_vertecies += geometry.vertices.len() as u64;
        // self.num_indecies += geometry.indices.len() as u64;
    }
}

impl WgpuRenderer for WgpuImmediateRenderer {
    type Renderer = WgpuImmediateRenderer;

    fn set_size(&mut self, width: u32, height: u32) {
        self.surface_config.width = width;
        self.surface_config.height = height;

        self.surface.configure(&self.device, &self.surface_config);
    }

    fn fill_rect(&mut self, rect: Rect, brush: &WgpuBrush) {
        let mut tesselation_buffer: VertexBuffers<Vertex, u16> = VertexBuffers::new();

        let mut fill_tess = FillTessellator::new();

        let mut builder = Path::builder();

        builder.begin(point(rect.x0 as f32, rect.y0 as f32));
        builder.line_to(point(rect.x0 as f32, rect.y1 as f32));
        builder.line_to(point(rect.x1 as f32, rect.y1 as f32));
        builder.line_to(point(rect.x1 as f32, rect.y0 as f32));

        builder.close();

        let path = builder.build();

        // tesselates geometries
        fill_tess
            .tessellate(
                &path,
                &FillOptions::tolerance(0.02).with_fill_rule(lyon::tessellation::FillRule::NonZero),
                &mut BuffersBuilder::new(&mut tesselation_buffer, VertexBuilder),
            )
            .unwrap();

        self.append_geometry(tesselation_buffer);
    }

    fn clear_all(&mut self, color: wgpu::Color) {
        self.clear_color = color;
        self.vertex_buffer.destroy();
        self.num_vertecies = 0;
        self.index_buffer.destroy();
        self.num_indecies = 0;
    }

    fn finish(&mut self) -> Result<()> {
        let output = self.surface.get_current_texture().unwrap();

        let frame_view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut render_pass = self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &frame_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(self.clear_color),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indecies as u32, 0, 0..1);
        // render_pass.draw(0..self.geometry_buffer.vertices.len() as u32, 0..1);

        // render_pass borrows encoder
        drop(render_pass);

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // swap encoders to work around finish() consuming the encoder
        // std::mem::swap(&mut self.encoder, &mut encoder);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    fn draw_image(&mut self, rect: kurbo::Rect, image: &WgpuImage) {
        todo!()
        // let texture_size = wgpu::Extent3d {
        //     width: dimensions.0,
        //     height: dimensions.1,
        //     depth_or_array_layers: 1,
        // };

        // let diffuse_texture = device.create_texture(&wgpu::TextureDescriptor {
        //     // All textures are stored as 3D, we represent our 2D texture
        //     // by setting depth to 1.
        //     size: texture_size,
        //     mip_level_count: 1, // We'll talk about this a little later
        //     sample_count: 1,
        //     dimension: wgpu::TextureDimension::D2,
        //     // Most images are stored using sRGB so we need to reflect that here.
        //     format: wgpu::TextureFormat::Rgba8UnormSrgb,
        //     // TEXTURE_BINDING tells wgpu that we want to use this texture in shaders
        //     // COPY_DST means that we want to copy data to this texture
        //     usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        //     label: Some("diffuse_texture"),
        // });
    }
}
