struct Globals {
    resolution: vec2<f32>,
    scale_factor: f32,
    _pad: f32,
};

struct VertexInput {
    @location(0) position: vec4<f32>,
    @location(1) color: vec4<f32>,
    @location(2) tex_coords: vec2<f32>,
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
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.tex_coords = model.tex_coords;
    // invert y - todo matrix multiply
    out.clip_position = vec4<f32>(model.position.xyz.x, model.position.xyz.y * -1.0, model.position.xyz.z, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords) + in.color;
}
