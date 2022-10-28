use lyon::lyon_tessellation::FillVertexConstructor;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 2],
    pub z_index: u32,
    pub color: [f32; 4],
    pub tex_coords: [f32; 2],
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TexVertex {
    pub position: [f32; 3],
}

unsafe impl bytemuck::Pod for TexVertex {}
unsafe impl bytemuck::Zeroable for TexVertex {}

// this one is needed by lyon for tessellation
pub struct VertexBuilder;

impl FillVertexConstructor<Vertex> for VertexBuilder {
    fn new_vertex(&mut self, vertex: lyon::tessellation::FillVertex) -> Vertex {
        Vertex {
            position: [vertex.position().x, vertex.position().y], // z is zero for now
            z_index: 1,
            color: [0.0, 0.0, 0.0, 1.0], // make it black
            tex_coords: [0.0, 0.0],      // no texture
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Globals {
    pub resolution: [f32; 2],
    pub scale_factor: f32,
    pub _pad: f32, // required by bind group layout
}

unsafe impl bytemuck::Pod for Globals {}
unsafe impl bytemuck::Zeroable for Globals {}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Primitive {
    pub color: [f32; 4],
    pub translate: [f32; 2],
    pub z_index: i32,
    pub width: f32,
    pub angle: f32,
    pub scale: f32,
    pub _pad1: i32,
    pub _pad2: i32,
}

impl Primitive {
    const DEFAULT: Self = Primitive {
        color: [1.0, 0.0, 0.0, 1.0],
        translate: [0.0; 2],
        z_index: 0,
        width: 0.0,
        angle: 0.0,
        scale: 1.0,
        _pad1: 0,
        _pad2: 0,
    };
}

impl Default for Primitive {
    fn default() -> Self {
        Self::DEFAULT
    }
}

unsafe impl bytemuck::Pod for Primitive {}
unsafe impl bytemuck::Zeroable for Primitive {}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GpuVertex {
    position: [f32; 2],
    normal: [f32; 2],
    prim_id: u32,
}

unsafe impl bytemuck::Pod for GpuVertex {}
unsafe impl bytemuck::Zeroable for GpuVertex {}

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
