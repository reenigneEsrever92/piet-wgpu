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
    error::Result,
    PietWgpu, WgpuBrush,
};

pub trait WgpuRenderer {
    type Renderer: WgpuRenderer;

    fn set_size(&mut self, width: u32, height: u32);
    fn fill_rect(&mut self, rect: kurbo::Rect, brush: &WgpuBrush);
    fn clear_all(&mut self, color: wgpu::Color);
    fn finish(&self) -> Result<()>;
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
