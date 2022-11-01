struct Globals {
    resolution: vec2<f32>,
    scale_factor: f32,
    // texture_dims: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
    @location(1) prim_index: u32,
};

struct Primitive {
    lower_bound: vec2<f32>,
    upper_bound: vec2<f32>,
    // primitives only have one color for now
    color: vec4<f32>,
    // tex coords point somewhere into texture buffer
    tex_coords: vec4<f32>,
    translate: vec2<f32>,
    angle: f32,
    scale: f32,
    z_index: i32,
    _pad: vec2<u32>,
};

@group(0) @binding(0) var<uniform> globals: Globals;

@group(1) @binding(0) var<uniform> primitives: array<Primitive, 256>;

@group(2) @binding(0) var t_diffuse: texture_2d<f32>;
@group(2) @binding(1) var s_diffuse: sampler;

@vertex
fn vs_main(
    @location(0) position: vec2<f32>,
    @location(1) prim_index: u32,
) -> VertexOutput {
    var prim = primitives[prim_index];
    
    var invert_y = vec2<f32>(1.0, -1.0);
    var offset = vec2<f32>(-1.0, -1.0);
    
    var world_pos = (position / globals.resolution * globals.scale_factor * 2.0 + offset) * invert_y;
    var pos_in_bounds = (position - prim.lower_bound) / (prim.upper_bound - prim.lower_bound);
    var tex_coord = prim.tex_coords.xy + (prim.tex_coords.zw - prim.tex_coords.xy) * pos_in_bounds;

    return VertexOutput(vec4<f32>(world_pos, 1.0, 1.0), tex_coord, prim_index);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var prim = primitives[in.prim_index];
    return textureSample(t_diffuse, s_diffuse, in.tex_coord) + prim.color;
}
