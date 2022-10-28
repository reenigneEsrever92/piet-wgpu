use std::{
    num::{NonZeroU32, NonZeroU64},
    path::PathBuf,
};

use lyon::{
    lyon_tessellation::{
        geometry_builder, BuffersBuilder, FillOptions, FillTessellator, VertexBuffers,
    },
    math::point,
    path::{traits::PathBuilder, Path},
};
use piet::{kurbo::Rect, IntoBrush};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroupDescriptor, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BufferUsages, Extent3d,
};

use crate::{
    config::Config,
    data::{Globals, Vertex, VertexBuilder},
    error::Result,
    renderer::WgpuRenderer,
    PietWgpu, WgpuBrush, WgpuImage,
};

pub type ImmediateRenderer = PietWgpu<WgpuImmediateRenderer>;

pub struct WgpuImmediateRenderer {
    scale: f64,
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
    texture_buffer: wgpu::Texture, // one buffer for all images
    texture_sampler: wgpu::Sampler,
    texture_bind_group_layout: BindGroupLayout,
    globals_buffer: wgpu::Buffer,
    globals_bind_group_layout: BindGroupLayout,
    pipeline: wgpu::RenderPipeline,
    surface_config: wgpu::SurfaceConfiguration,
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

        let globals_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Globals Buffer"),
            size: std::mem::size_of::<Globals>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let globals_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Global Bind Group Layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: NonZeroU64::new(std::mem::size_of::<Globals>() as u64),
                    },
                    count: None,
                }],
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

        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Uint32, 2 => Float32x4, 3 => Float32x2],
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
            mapped_at_creation: false,
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
            mapped_at_creation: false,
        });

        // let index_buffer = self
        //     .device
        //     .create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //         label: Some("Index Buffer"),
        //         contents: bytemuck::cast_slice(&self.geometry_buffer.indices),
        //         usage: wgpu::BufferUsages::INDEX,
        //     });

        let texture_size = wgpu::Extent3d {
            // TODO use config
            width: 4 * 256,
            height: 256,
            depth_or_array_layers: 1,
        };

        let texture_buffer = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("diffuse_texture"),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });

        let texture_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&globals_bind_group_layout, &texture_bind_group_layout],
                push_constant_ranges: &[],
            });

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

        Ok(Self {
            scale,
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
            texture_bind_group_layout,
            texture_buffer,
            texture_sampler,
            globals_buffer,
            globals_bind_group_layout,
            clear_color,
        })
    }

    fn append_geometry(&mut self, geometry: VertexBuffers<Vertex, u16>) {
        let vertecies = self.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Copy Buffer"),
            usage: BufferUsages::COPY_SRC,
            contents: bytemuck::cast_slice(&geometry.vertices),
        });

        let indicies = self.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Copy Buffer"),
            usage: BufferUsages::COPY_SRC,
            contents: bytemuck::cast_slice(
                &geometry
                    .indices
                    .iter()
                    .map(|index| *index + self.num_vertecies as u16)
                    .collect::<Vec<u16>>(),
            ),
        });

        self.encoder.copy_buffer_to_buffer(
            &vertecies,
            0,
            &self.vertex_buffer,
            std::mem::size_of::<Vertex>() as u64 * self.num_vertecies,
            std::mem::size_of::<Vertex>() as u64 * geometry.vertices.len() as u64,
        );

        self.encoder.copy_buffer_to_buffer(
            &indicies,
            0,
            &self.index_buffer,
            std::mem::size_of::<u16>() as u64 * self.num_indecies,
            std::mem::size_of::<u16>() as u64 * geometry.indices.len() as u64,
        );

        self.num_vertecies += geometry.vertices.len() as u64;
        self.num_indecies += geometry.indices.len() as u64;
    }

    fn tesselate_fill(&self, path: Path) -> VertexBuffers<Vertex, u16> {
        let mut tesselation_buffer = VertexBuffers::new();
        let mut fill_tess = FillTessellator::new();

        fill_tess
            .tessellate(
                &path,
                &FillOptions::tolerance(0.02).with_fill_rule(lyon::tessellation::FillRule::NonZero),
                &mut BuffersBuilder::new(&mut tesselation_buffer, VertexBuilder),
            )
            .unwrap();

        tesselation_buffer
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
        let mut builder = Path::builder();

        builder.begin(point(rect.x0 as f32, rect.y0 as f32));
        builder.line_to(point(rect.x0 as f32, rect.y1 as f32));
        builder.line_to(point(rect.x1 as f32, rect.y1 as f32));
        builder.line_to(point(rect.x1 as f32, rect.y0 as f32));

        builder.close();

        let path = builder.build();

        // tesselates geometries
        let geometry = self.tesselate_fill(path);

        self.append_geometry(geometry);
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

        // prepare textures
        let texture_view = self
            .texture_buffer
            .create_view(&wgpu::TextureViewDescriptor::default());

        let texture_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("diffuse_bind_group"),
            layout: &self.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.texture_sampler),
                },
            ],
        });

        // TODO move to set_size or something
        let globals = Globals {
            resolution: [
                self.surface_config.width as f32,
                self.surface_config.height as f32,
            ],
            scale_factor: self.scale as f32,
            _pad: 0.0,
        };

        let globals_bind_group = self.device.create_bind_group(&BindGroupDescriptor {
            label: Some("Globals Bind Group"),
            layout: &self.globals_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(
                    self.globals_buffer.as_entire_buffer_binding(),
                ),
            }],
        });

        // prepare render pass
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
        render_pass.set_bind_group(0, &globals_bind_group, &[]);
        render_pass.set_bind_group(1, &texture_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indecies as u32, 0, 0..1);
        // render_pass.draw(0..self.geometry_buffer.vertices.len() as u32, 0..1);

        // render_pass borrows encoder
        drop(render_pass);

        // create and swap encoders to work around finish() consuming the encoder
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        std::mem::swap(&mut self.encoder, &mut encoder);

        self.queue
            .write_buffer(&self.globals_buffer, 0, bytemuck::cast_slice(&[globals]));
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    fn draw_image(&mut self, rect: kurbo::Rect, image: &WgpuImage) {
        let rgba_image = image.dynamic.as_rgba8().unwrap();

        let texture_size = wgpu::Extent3d {
            width: rgba_image.width(),
            height: rgba_image.height(),
            depth_or_array_layers: 1,
        };

        // copy image data to image buffer
        self.queue.write_texture(
            // Tells wgpu where to copy the pixel data
            wgpu::ImageCopyTexture {
                texture: &self.texture_buffer,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            // The actual pixel data
            rgba_image,
            // The layout of the texture
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * image.dynamic.width()),
                rows_per_image: std::num::NonZeroU32::new(image.dynamic.height()),
            },
            texture_size,
        );

        let mut builder = Path::builder();

        builder.begin(point(rect.x0 as f32, rect.y0 as f32));
        builder.line_to(point(rect.x0 as f32, rect.y1 as f32));
        builder.line_to(point(rect.x1 as f32, rect.y1 as f32));
        builder.line_to(point(rect.x1 as f32, rect.y0 as f32));

        builder.close();

        let path = builder.build();

        let mut geometry = self.tesselate_fill(path);

        // map tex_coords
        let vertecies = geometry
            .vertices
            .iter()
            .enumerate()
            .map(|(index, vertex)| Vertex {
                tex_coords: match index {
                    0 => [0.0, 0.0],
                    1 => [0.0, 1.0],
                    2 => [1.0, 0.0],
                    3 => [1.0, 1.0],
                    _ => panic!(),
                },
                ..*vertex
            })
            .collect::<Vec<Vertex>>();

        geometry.vertices = vertecies;

        self.append_geometry(geometry);
    }

    fn set_scale(&mut self, scale_factor: f64) {
        self.scale = scale_factor;
    }
}
