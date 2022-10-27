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
    var invert_y = vec4<f32>(1.0, -1.0, 1.0, 1.0);
    var position = (model.position / vec4<f32>(globals.resolution, 1.0, 1.0) - vec4<f32>(1.0, 1.0, 0.0, 0.0)) * invert_y * globals.scale_factor;
    // var position = model.position / (0.5 * vec4<f32>(globals.resolution, 1.0, 1.0)) * invert_y;
    
    // vec4<f32>(
    //     model.position.xyz.x + 1.0 * globals.resolution.xyz.x, 
    //     model.position.xyz.y * -1.0 + 1.0 * globals.resolution.xyz.y, 
    //     model.position.xyz.z, 1.0
    // );

    return VertexOutput(position, model.color, model.tex_coords);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords) + in.color;
}
