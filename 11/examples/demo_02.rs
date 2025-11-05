// cargo run -p step_11 --example demo_02
//
//!GPU testing
// Version 02
// Select wgpu_backend & presentation mode
// wgpu::Backends::DX12 or wgpu::Backends::VULKAN
//

use pixels::{Pixels, PixelsBuilder, SurfaceTexture, wgpu};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::Window,
};

//
const WIDTH: u32 = 200;
const HEIGHT: u32 = 150;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Default)]
struct App {
    window: Option<&'static Window>,
    pixels: Option<Pixels<'static>>,
}

//
//
//
//
//
//
//
//
//
//
//

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(Window::default_attributes().with_title("Step_11: GPU, uses wgpu_backend")).unwrap();
        let window_ref: &'static Window = Box::leak(Box::new(window));
        let size = window_ref.inner_size();
        let surface = SurfaceTexture::new(size.width, size.height, window_ref);

        //
        let mut builder = PixelsBuilder::new(WIDTH, HEIGHT, surface);

        // Prefer the high-perf discrete GPU:
        builder = builder.request_adapter_options(wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None, // pixels fills this internally
            force_fallback_adapter: false,
        });

        // Force a backend explicitly. Pick one or the other
        builder = builder.wgpu_backend(wgpu::Backends::DX12);
        // builder = builder.wgpu_backend(wgpu::Backends::VULKAN)

        let mut pixels = builder.build().expect("create pixels");

        // Pick one or the other
        pixels.set_present_mode(wgpu::PresentMode::Fifo); // Fifo = default mode (vsync activated)
        // pixels.set_present_mode(wgpu::PresentMode::Immediate);

        //
        self.window = Some(window_ref);
        self.pixels = Some(pixels);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: winit::window::WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                if let Some(pixels) = &mut self.pixels {
                    let frame = pixels.frame_mut();

                    for pixel in frame.chunks_exact_mut(4) {
                        pixel[0] = 0x20; // R
                        pixel[1] = 0x40; // G
                        pixel[2] = 0xFF; // B
                        pixel[3] = 0xFF; // A
                    }

                    //
                    pixels.render().unwrap();
                }

                //
                //
                //
                //
                //
                //
                //
                //
                //
                //

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        self.window.expect("Bug - Window should exist").request_redraw();
    }
}

fn main() -> Result<()> {
    // Alternative in PowerShell : $env:RUST_LOG='wgpu_core=trace'; cargo run -p step_11 --example demo_02; Remove-Item env:RUST_LOG
    // Recommended
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("wgpu_core=info,wgpu_hal=warn,wgpu=warn")).init();

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app)?;

    Ok(())
}
