struct WgpuTexture {
    texture_positions: wgpu::TextureDimension,
    texture_buffer: wgpu::Texture,
    texture_sampler: wgpu::Sampler,
    texture_bind_group_layout: BindGroupLayout,
}

impl WgpuTexture {}
