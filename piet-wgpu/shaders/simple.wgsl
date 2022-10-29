struct Globals {
    resolution: vec2<f32>,
    scale_factor: f32,
    _pad: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
    @location(1) prim_index: u32,
};

struct Primitive {
    // orimitives only have one color for now
    color: vec4<f32>,
    // tex coords point somewhere into texture buffer
    tex_coords: vec4<f32>,
    translate: vec2<f32>,
    z_index: i32,
    angle: f32,
    scale: f32,
    _pad: i32,
};

struct Primitives {
    primitives: array<Primitive, 256>,
};

@group(0) @binding(0) var<uniform> globals: Globals;

@group(1) @binding(0) var<uniform> primitives: array<Primitive, 256>;

@group(2) @binding(0) var t_diffuse: texture_2d<f32>;
@group(2) @binding(1) var s_diffuse: sampler;

@vertex
fn vs_main(
    @location(0) position: vec2<f32>,
    @location(1) prim_index: u32,
    @builtin(vertex_index) vertex_index: u32,
) -> VertexOutput {
    var prim = primitives[prim_index];
    
    var invert_y = vec2<f32>(1.0, -1.0);
    var offset = vec2<f32>(-1.0, -1.0);
    
    var world_pos = (position / globals.resolution * globals.scale_factor * 2.0 + offset) * invert_y;

    var tex_coords = vec2<f32>(0.0, 0.0);

    if f32(vertex_index) % 4.0 == 0.0 {
        tex_coords = vec2<f32>(prim.tex_coords[0], prim.tex_coords[1]);
    } else if f32(vertex_index) % 3.0 == 0.0 {
        tex_coords = vec2<f32>(prim.tex_coords[2], prim.tex_coords[3]);
    } 

    // var z = f32(prim.z_index) / 4096.0;

    return VertexOutput(vec4<f32>(world_pos, 1.0, 1.0), vec2<f32>(0.0, 0.0), prim_index);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var prim = primitives[in.prim_index];
    return textureSample(t_diffuse, s_diffuse, in.tex_coord) + prim.color;
}
