use lyon::lyon_tessellation::FillVertexConstructor;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 2],
    pub prim_index: u32,
    pub _pad: u32,
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

pub struct VertexBuilder {
    pub prim_index: u32,
}

// this one is needed by lyon for inverting construction
impl FillVertexConstructor<Vertex> for VertexBuilder {
    fn new_vertex(&mut self, vertex: lyon::tessellation::FillVertex) -> Vertex {
        Vertex {
            position: [vertex.position().x, vertex.position().y], // z is zero for now
            prim_index: self.prim_index,
            _pad: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Globals {
    pub resolution: [f32; 2],
    pub scale_factor: f32,
    // pub tex_dims: [f32; 4],
    pub _pad: u32,
}

unsafe impl bytemuck::Pod for Globals {}
unsafe impl bytemuck::Zeroable for Globals {}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Primitive {
    pub lower_bound: [f32; 2], // 8
    pub upper_bound: [f32; 2], // 8
    pub color: [f32; 4],       // 16
    pub tex_coords: [f32; 4],  // 16
    pub translate: [f32; 2],   // 8
    pub angle: f32,            // 4
    pub scale: f32,            // 4
    pub z_index: i32,          // 4
    pub _pad: [u32; 3],        // 12
                               // 80
}

impl Primitive {
    const DEFAULT: Self = Primitive {
        lower_bound: [0.0, 0.0],
        upper_bound: [0.0, 0.0],
        color: [0.0, 0.0, 0.0, 1.0],
        tex_coords: [0.0, 0.0, 0.0, 0.0],
        translate: [0.0; 2],
        angle: 0.0,
        scale: 1.0,
        z_index: 0,
        _pad: [0, 0, 0],
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
