struct Globals {
    resolution: vec2<f32>,
    scale_factor: f32,
    _pad: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

struct Primitive {
    // orimitives only have one color for now
    color: vec4<f32>,
    // tex coords point somewhere into texture buffer
    tex_cords: vec2<f32>,
    translate: vec2<f32>,
    z_index: i32,
    angle: f32,
    scale: f32,
    _pad: i32,
};

@group(0) @binding(0) var<uniform> globals: Globals;

@group(1) @binding(0) var<uniform> primitives: array<Primitive, 4096>;

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
    var z = f32(prim.z_index) / 4096.0;

    return VertexOutput(vec4<f32>(world_pos, z, 1.0));
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var prim = primitives[prim_index];
    return textureSample(t_diffuse, s_diffuse, in.tex_coords) + in.color;
}
