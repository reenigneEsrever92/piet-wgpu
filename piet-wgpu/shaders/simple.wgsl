struct Globals {
    resolution: vec2<f32>,
    scale_factor: f32,
    _pad: f32,
};

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) z_index: u32,
    @location(2) color: vec4<f32>,
    @location(3) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) tex_coords: vec2<f32>,
};

@group(0) @binding(0) var<uniform> globals: Globals;

@group(1) @binding(0) var t_diffuse: texture_2d<f32>;
@group(1) @binding(1) var s_diffuse: sampler;

@vertex
fn vs_main(
    @location(0) position: vec2<f32>,
    @location(1) z_index: u32,
    @location(2) color: vec4<f32>,
    @location(3) tex_coords: vec2<f32>,
) -> VertexOutput {
    var invert_y = vec2<f32>(1.0, -1.0);
    var offset = vec2<f32>(-1.0, -1.0);
    
    var world_pos = (position / globals.resolution * globals.scale_factor * 2.0 + offset) * invert_y;
    var z = f32(z_index) / 4096.0;

    return VertexOutput(vec4<f32>(world_pos, z, 1.0), color, tex_coords);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords) + in.color;
}
