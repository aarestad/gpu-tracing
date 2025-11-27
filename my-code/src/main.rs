use {
    anyhow::Result,
    winit::application::ApplicationHandler,
    winit::event::WindowEvent,
    winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    winit::window::{Window, WindowId},
};

const SIZE: winit::dpi::PhysicalSize<i32> = winit::dpi::PhysicalSize::new(800, 600);

#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_inner_size(SIZE)
                        .with_resizable(false)
                        .with_title("GPU Path Tracer".to_string()),
                )
                .unwrap(),
        );
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // TODO: draw frame
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app)?;

    Ok(())
}
