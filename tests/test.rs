use winit::{
    dpi::LogicalSize,
    event_loop::{EventLoop, EventLoopBuilder},
    platform::unix::EventLoopBuilderExtUnix,
    window::{Window, WindowBuilder},
};

#[test]
fn test_init() {
    let (window, event_loop) = create_window();

    event_loop.run(|event, _, control_flow| {
        println!("{event:?}");
    });
}

fn create_window() -> (Window, EventLoop<()>) {
    let event_loop = EventLoopBuilder::new().with_any_thread(true).build();

    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(600, 400))
        .build(&event_loop)
        .unwrap();

    (window, event_loop)
}
