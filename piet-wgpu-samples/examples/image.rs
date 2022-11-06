use piet_wgpu::{kurbo::Rect, RenderContext, WgpuImage};
use piet_wgpu_samples::render;

fn main() {
    render(|renderer| {
        let image = WgpuImage::from_bytes(include_bytes!("../resources/img/happy-tree.png"));
        let darth_vader = WgpuImage::from_bytes(include_bytes!("../resources/img/darth_vader.png"));
        let fire = WgpuImage::from_bytes(include_bytes!("../resources/img/fire.png"));

        renderer.draw_image(
            &image,
            Rect::new(0.0, 0.0, 200.0, 200.0),
            piet::InterpolationMode::NearestNeighbor,
        );
        renderer.draw_image(
            &image,
            Rect::new(200.0, 200.0, 400.0, 400.0),
            piet::InterpolationMode::NearestNeighbor,
        );
        // renderer.draw_image(
        //     &fire,
        //     Rect::new(400.0, 400.0, 600.0, 600.0),
        //     piet::InterpolationMode::NearestNeighbor,
        // );
    });
}
