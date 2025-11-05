// cargo run -p step_11 --example demo_03
//
//!GPU testing - FPS compare
// Version 03
// Edit code and select : GPU, backend and PresentMode

use pixels::{Pixels, PixelsBuilder, SurfaceTexture, wgpu};
use std::time::{Duration, Instant};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::Window,
};

const WIDTH: u32 = 200;
const HEIGHT: u32 = 150;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

struct App {
    window: Option<&'static Window>,
    pixels: Option<Pixels<'static>>,
    fps_last: Instant,
    fps_frames: u32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            window: None,
            pixels: None,
            fps_last: Instant::now(),
            fps_frames: 0,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes().with_title("Step 11: GPU bench 00").with_resizable(false))
            .unwrap();
        let window_ref: &'static Window = Box::leak(Box::new(window));
        let size = window_ref.inner_size();
        let surface = SurfaceTexture::new(size.width, size.height, window_ref);

        let mut builder = PixelsBuilder::new(WIDTH, HEIGHT, surface);
        builder = builder.request_adapter_options(wgpu::RequestAdapterOptions {
            // 1 - GPU: Pick one or the other
            power_preference: wgpu::PowerPreference::LowPower,
            // power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        });

        // 2 - Backend: Pick one or the other
        // builder = builder.wgpu_backend(wgpu::Backends::DX12);
        builder = builder.wgpu_backend(wgpu::Backends::VULKAN);

        let mut pixels = builder.build().expect("create pixels");

        // 3 - PresentationMode: Pick one or the other
        // pixels.set_present_mode(wgpu::PresentMode::Fifo);
        pixels.set_present_mode(wgpu::PresentMode::Immediate);
        println!("Present mode: {:?}", pixels.present_mode());

        self.window = Some(window_ref);
        self.pixels = Some(pixels);

        // Init FPS timer
        self.fps_last = Instant::now();
        self.fps_frames = 0;

        window_ref.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: winit::window::WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::RedrawRequested => {
                if let Some(pixels) = &mut self.pixels {
                    let frame = pixels.frame_mut();

                    for px in frame.chunks_exact_mut(4) {
                        px[0] = 0x20; // R
                        px[1] = 0x40; // G
                        px[2] = 0xFF; // B
                        px[3] = 0xFF; // A
                    }

                    if let Err(e) = pixels.render() {
                        eprintln!("pixels.render() failed: {e}");
                    }

                    // --- FPS counting ---
                    self.fps_frames += 1;
                    let elapsed = self.fps_last.elapsed();
                    if elapsed >= Duration::from_secs(1) {
                        let fps = self.fps_frames as f32 / elapsed.as_secs_f32();
                        println!("FPS (Fifo): {:.1}", fps);
                        self.fps_frames = 0;
                        self.fps_last = Instant::now();
                    }
                }

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
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("wgpu_core=info,wgpu_hal=warn,wgpu=warn")).init();

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app)?;

    Ok(())
}
