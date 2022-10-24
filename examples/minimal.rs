use log::warn;
use piet_wgpu::{
    immediate::{ImmediateRenderer, WgpuImmediateTesselationRenderer},
    kurbo::Rect,
    Color, PietWgpu, RenderContext,
};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::{Window, WindowBuilder},
};

fn main() {
    render(|renderer| {
        let brush = renderer.solid_brush(Color::rgb(1.0, 0.0, 0.0));
        renderer.fill(Rect::new(-0.5, -0.5, 0.5, 0.5), &brush);
        renderer.fill(Rect::new(-1.0, -1.0, -0.5, -0.5), &brush);
    });
}

fn render<FN: FnMut(&mut PietWgpu<WgpuImmediateTesselationRenderer>) + Sized + 'static>(
    mut fun: FN,
) {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let (window, event_loop, mut piet_wgpu) = create_window();

    fun(&mut piet_wgpu);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            winit::event::Event::WindowEvent { window_id, event } if window_id == window.id() => {
                match event {
                    WindowEvent::Resized(new_size) => {
                        piet_wgpu.set_size(new_size.width, new_size.height);
                        piet_wgpu.finish().unwrap();
                    }
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    _ => warn!("Unhandled window event: {event:?}"),
                }
            }
            winit::event::Event::RedrawRequested(window_id) if window.id() == window_id => {
                piet_wgpu.finish().unwrap();
            }
            winit::event::Event::MainEventsCleared => {}
            _ => warn!("Unhandled event: {event:?}"),
        }
    });
}

fn create_window() -> (Window, EventLoop<()>, ImmediateRenderer) {
    let event_loop = EventLoopBuilder::new().build();

    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(600, 600))
        .build(&event_loop)
        .unwrap();

    let renderer = WgpuImmediateTesselationRenderer::new(
        &window,
        window.inner_size().width,
        window.inner_size().height,
        window.scale_factor(),
    )
    .unwrap();

    let renderer = PietWgpu::new(
        &window,
        renderer,
        window.inner_size().width,
        window.inner_size().height,
        window.scale_factor(),
    );

    (window, event_loop, renderer)
}
