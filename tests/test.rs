use log::warn;
use piet_wgpu::{
    kurbo::{Rect, Size},
    Color, PietWgpu, RenderContext, WgpuRenderer,
};
use winit::{
    dpi::LogicalSize,
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    platform::unix::EventLoopBuilderExtUnix,
    window::{Window, WindowBuilder},
};

#[test]
fn test_init() {
    render(|renderer| {
        let brush = renderer.solid_brush(Color::rgb(1.0, 0.0, 0.0));
        renderer.fill(Rect::new(10.0, 10.0, 30.0, 30.0), &brush);
        renderer.finish().unwrap();
    });
}

fn render<FN: FnMut(&mut PietWgpu) + Sized + 'static>(mut fun: FN) {
    let (window, event_loop, mut piet_wgpu) = create_window();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        piet_wgpu.renderer.set_size(Size::new(100.0, 100.0));
        match event {
            winit::event::Event::WindowEvent { window_id, event } if window_id == window.id() => {
                match event {
                    winit::event::WindowEvent::Resized(new_size) => {
                        piet_wgpu
                            .renderer
                            .set_size(Size::new(new_size.width.into(), new_size.height.into()));
                    }
                    _ => warn!("Unhandled window event: {event:?}"),
                }
            }
            winit::event::Event::RedrawRequested(window_id) if window.id() == window_id => {
                fun(&mut piet_wgpu);
            }
            winit::event::Event::RedrawEventsCleared => todo!(),
            winit::event::Event::LoopDestroyed => todo!(),
            _ => warn!("Unhandled event: {event:?}"),
        }
    });
}

fn create_window() -> (Window, EventLoop<()>, PietWgpu) {
    let event_loop = EventLoopBuilder::new().with_any_thread(true).build();

    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(600, 400))
        .build(&event_loop)
        .unwrap();

    let renderer = PietWgpu::new(
        &window,
        Size::new(
            window.inner_size().width.into(),
            window.inner_size().height.into(),
        ),
        window.scale_factor(),
    );

    (window, event_loop, renderer)
}
