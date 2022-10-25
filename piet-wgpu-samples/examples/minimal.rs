use piet::RenderContext;
use piet_wgpu::{kurbo::Rect, Color};
use piet_wgpu_samples::render;

fn main() {
    render(|renderer| {
        let brush = renderer.solid_brush(Color::rgb(1.0, 0.0, 0.0));
        renderer.fill(Rect::new(-0.5, -0.5, 0.5, 0.5), &brush);
        renderer.fill(Rect::new(1.0, 1.0, 0.5, 0.5), &brush);
        renderer.fill(Rect::new(-1.0, -1.0, -0.5, -0.5), &brush);
        renderer.fill(Rect::new(-1.0, 1.0, -0.5, 0.5), &brush);
        renderer.fill(Rect::new(1.0, -1.0, 0.5, -0.5), &brush);
    });
}
