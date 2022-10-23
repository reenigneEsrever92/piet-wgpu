use lyon::{
    lyon_tessellation::{BuffersBuilder, FillOptions, FillTessellator, VertexBuffers},
    math::point,
    path::Path,
};
use piet::{kurbo::Rect, IntoBrush};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use wgpu::util::DeviceExt;

use crate::{
    data::{Primitive, Vertex, VertexBuilder},
    PietWgpu,
};

pub struct WgpuImmediateTesselationRenderer {
    instance: wgpu::Instance,
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
    surface_config: wgpu::SurfaceConfiguration,
    geometry_buffer: VertexBuffers<Vertex, u16>,
    primitive_buffer: Vec<Primitive>,
}
static_assertions::assert_impl_all!(WgpuImmediateTesselationRenderer: Send, Sync);

impl WgpuImmediateTesselationRenderer {
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
                limits: wgpu::Limits::default(),
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
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };

        let geometry_buffer = VertexBuffers::new();

        let primitive_buffer = Vec::with_capacity(256); // 256 primitives tops for now

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

        Ok(Self {
            instance,
            surface,
            adapter,
            device,
            queue,
            pipeline,
            surface_config,
            geometry_buffer,
            primitive_buffer,
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

        // tesselates geometries
        fill_tess
            .tessellate(
                &path,
                &FillOptions::tolerance(0.02).with_fill_rule(lyon::tessellation::FillRule::NonZero),
                &mut BuffersBuilder::new(&mut self.geometry_buffer, VertexBuilder),
            )
            .unwrap();
    }

    pub fn finish(&self) -> Result<(), piet::Error> {
        // currently indicies are in clockwise order!? They have to be counterclockwise though!
        // if I got this out this would make finish non mut
        // self.geometry_buffer.indices.reverse();

        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&self.geometry_buffer.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&self.geometry_buffer.indices),
                usage: wgpu::BufferUsages::INDEX,
            });

        let output = self.surface.get_current_texture().unwrap();

        let frame_view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[
                // This is what @location(0) in the fragment shader targets
                Some(wgpu::RenderPassColorAttachment {
                    view: &frame_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: true,
                    },
                }),
            ],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.geometry_buffer.indices.len() as u32, 0, 0..1);
        // render_pass.draw(0..self.geometry_buffer.vertices.len() as u32, 0..1);

        // pass borrows encoder
        drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

// let globals_buffer_byte_size = std::mem::size_of::<Globals>() as u64;
// let prim_buffer_byte_size = (256 * std::mem::size_of::<Primitive>()) as u64;

// let vbo = self
//     .device
//     .create_buffer_init(&wgpu::util::BufferInitDescriptor {
//         label: None,
//         contents: bytemuck::cast_slice(&self.geometry_buffer.vertices),
//         usage: wgpu::BufferUsages::VERTEX,
//     });

// let ibo = self
//     .device
//     .create_buffer_init(&wgpu::util::BufferInitDescriptor {
//         label: None,
//         contents: bytemuck::cast_slice(&self.geometry_buffer.indices),
//         usage: wgpu::BufferUsages::INDEX,
//     });

// let prims_ubo = self.device.create_buffer(&wgpu::BufferDescriptor {
//     label: Some("Prims ubo"),
//     size: prim_buffer_byte_size,
//     usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
//     mapped_at_creation: false,
// });

// let globals_ubo = self.device.create_buffer(&wgpu::BufferDescriptor {
//     label: Some("Globals ubo"),
//     size: globals_buffer_byte_size,
//     usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
//     mapped_at_creation: false,
// });

// let vs_module = &self
//     .device
//     .create_shader_module(wgpu::ShaderModuleDescriptor {
//         label: Some("Geometry vs"),
//         source: wgpu::ShaderSource::Wgsl(
//             include_str!("./../shaders/geometry.vs.wgsl").into(),
//         ),
//     });

// let fs_module = &self
//     .device
//     .create_shader_module(wgpu::ShaderModuleDescriptor {
//         label: Some("Geometry fs"),
//         source: wgpu::ShaderSource::Wgsl(
//             include_str!("./../shaders/geometry.fs.wgsl").into(),
//         ),
//     });

// let bind_group_layout =
//     self.device
//         .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
//             label: Some("Bind group layout"),
//             entries: &[
//                 wgpu::BindGroupLayoutEntry {
//                     binding: 0,
//                     visibility: wgpu::ShaderStages::VERTEX,
//                     ty: wgpu::BindingType::Buffer {
//                         ty: wgpu::BufferBindingType::Uniform,
//                         has_dynamic_offset: false,
//                         min_binding_size: wgpu::BufferSize::new(globals_buffer_byte_size),
//                     },
//                     count: None,
//                 },
//                 wgpu::BindGroupLayoutEntry {
//                     binding: 1,
//                     visibility: wgpu::ShaderStages::VERTEX,
//                     ty: wgpu::BindingType::Buffer {
//                         ty: wgpu::BufferBindingType::Uniform,
//                         has_dynamic_offset: false,
//                         min_binding_size: wgpu::BufferSize::new(prim_buffer_byte_size),
//                     },
//                     count: None,
//                 },
//             ],
//         });

// let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
//     label: Some("Bind group"),
//     layout: &bind_group_layout,
//     entries: &[
//         wgpu::BindGroupEntry {
//             binding: 0,
//             resource: wgpu::BindingResource::Buffer(globals_ubo.as_entire_buffer_binding()),
//         },
//         wgpu::BindGroupEntry {
//             binding: 1,
//             resource: wgpu::BindingResource::Buffer(prims_ubo.as_entire_buffer_binding()),
//         },
//     ],
// });

// let pipeline_layout = self
//     .device
//     .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
//         bind_group_layouts: &[&bind_group_layout],
//         push_constant_ranges: &[],
//         label: None,
//     });

// let depth_stencil_state = Some(wgpu::DepthStencilState {
//     format: wgpu::TextureFormat::Depth32Float,
//     depth_write_enabled: true,
//     depth_compare: wgpu::CompareFunction::Greater,
//     stencil: wgpu::StencilState {
//         front: wgpu::StencilFaceState::IGNORE,
//         back: wgpu::StencilFaceState::IGNORE,
//         read_mask: 0,
//         write_mask: 0,
//     },
//     bias: wgpu::DepthBiasState::default(),
// });

// let mut render_pipeline_descriptor = wgpu::RenderPipelineDescriptor {
//     label: None,
//     layout: Some(&pipeline_layout),
//     vertex: wgpu::VertexState {
//         module: vs_module,
//         entry_point: "main",
//         buffers: &[wgpu::VertexBufferLayout {
//             array_stride: std::mem::size_of::<GpuVertex>() as u64,
//             step_mode: wgpu::VertexStepMode::Vertex,
//             attributes: &[
//                 wgpu::VertexAttribute {
//                     offset: 0,
//                     format: wgpu::VertexFormat::Float32x2,
//                     shader_location: 0,
//                 },
//                 wgpu::VertexAttribute {
//                     offset: 8,
//                     format: wgpu::VertexFormat::Float32x2,
//                     shader_location: 1,
//                 },
//                 wgpu::VertexAttribute {
//                     offset: 16,
//                     format: wgpu::VertexFormat::Uint32,
//                     shader_location: 2,
//                 },
//             ],
//         }],
//     },
//     fragment: Some(wgpu::FragmentState {
//         module: fs_module,
//         entry_point: "main",
//         targets: &[Some(wgpu::ColorTargetState {
//             format: wgpu::TextureFormat::Rgba8UnormSrgb,
//             blend: None,
//             write_mask: wgpu::ColorWrites::ALL,
//         })],
//     }),
//     primitive: wgpu::PrimitiveState {
//         topology: wgpu::PrimitiveTopology::TriangleList,
//         polygon_mode: wgpu::PolygonMode::Fill,
//         front_face: wgpu::FrontFace::Ccw,
//         strip_index_format: None,
//         cull_mode: Some(wgpu::Face::Back),
//         conservative: false,
//         unclipped_depth: false,
//     },
//     depth_stencil: depth_stencil_state,
//     multisample: wgpu::MultisampleState {
//         count: 1,
//         mask: !0,
//         alpha_to_coverage_enabled: false,
//     },
//     multiview: None,
// };

// let render_pipeline = self
//     .device
//     .create_render_pipeline(&render_pipeline_descriptor);
