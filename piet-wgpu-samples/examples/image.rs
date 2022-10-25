use piet_wgpu::{kurbo::Rect, RenderContext, WgpuImage};
use piet_wgpu_samples::render;

fn main() {
    render(|renderer| {
        let image = WgpuImage::from_bytes(include_bytes!("../resources/img/happy-tree.png"));
        renderer.draw_image(
            &image,
            Rect::new(-0.5, -0.5, 0.5, 0.5),
            piet::InterpolationMode::NearestNeighbor,
        );
    });
}
