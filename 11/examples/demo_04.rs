// cargo run -p step_11 --example bench
//
//! GPU Benchmark - Compare GPU & Backend Performance
// Version 04
// Change these settings to test different configurations
//      1. GPU: Change `power_preference` (HighPerformance = NVIDIA, LowPower = Intel)
//      2. Backend: Uncomment the backend you want to test
//      3. Uncomment the presentation mode (fofo, immediate)

use pixels::{PixelsBuilder, SurfaceTexture, wgpu};
use std::time::{Duration, Instant};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::Window,
};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const BENCHMARK_DURATION: Duration = Duration::from_secs(15);

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

struct App {
    window: Option<&'static Window>,
    pixels: Option<pixels::Pixels<'static>>,
    frame_count: u64,
    start_time: Option<Instant>,
    last_print: Instant,
}

impl Default for App {
    fn default() -> Self {
        Self {
            window: None,
            pixels: None,
            frame_count: 0,
            start_time: None,
            last_print: Instant::now(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes().with_title("Step 11: GPU bench 01, complex animation").with_resizable(false))
            .unwrap();
        // .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None))),
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

        // Init print timer
        self.start_time = Some(Instant::now());

        println!("\nBENCHMARK STARTED");
        println!("Duration: {} seconds", BENCHMARK_DURATION.as_secs());
        println!("Resolution: {}x{}\n", WIDTH, HEIGHT);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: winit::window::WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.print_final_results();
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                // Check if benchmark is complete
                if let Some(start) = self.start_time
                    && start.elapsed() >= BENCHMARK_DURATION
                {
                    self.print_final_results();
                    event_loop.exit();
                    return;
                }

                // Render "complex" animation
                if let Some(pixels) = &mut self.pixels {
                    let frame = pixels.frame_mut();
                    let time = self.frame_count as f32 * 0.05;

                    // Draw animated pattern
                    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                        let x = (i % WIDTH as usize) as f32;
                        let y = (i / WIDTH as usize) as f32;

                        // Animated waves
                        let r = ((x / 10.0 + time).sin() * 127.0 + 128.0) as u8;
                        let g = ((y / 10.0 + time).cos() * 127.0 + 128.0) as u8;
                        let b = ((x / 5.0 + y / 5.0 + time).sin() * 127.0 + 128.0) as u8;

                        pixel[0] = r;
                        pixel[1] = g;
                        pixel[2] = b;
                        pixel[3] = 0xFF;
                    }

                    pixels.render().ok();
                }

                self.frame_count += 1;

                // Print progress every second
                if self.last_print.elapsed() >= Duration::from_secs(1) {
                    if let Some(start) = self.start_time {
                        let elapsed = start.elapsed().as_secs();
                        let fps = self.frame_count as f32 / start.elapsed().as_secs_f32();
                        println!("[{:2}s] Frames: {} | FPS: {:.0}", elapsed, self.frame_count, fps);
                    }
                    self.last_print = Instant::now();
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

impl App {
    fn print_final_results(&self) {
        println!("\n{}", "=".repeat(50));
        println!("BENCHMARK COMPLETE");
        println!("{}", "=".repeat(50));

        if let Some(start) = self.start_time {
            let elapsed = start.elapsed().as_secs_f32();
            let fps = self.frame_count as f32 / elapsed;

            println!("Total Frames:  {}", self.frame_count);
            println!("Total Time:    {:.2}s", elapsed);
            println!("Average FPS:   {:.0}", fps);
            println!("Frame Time:    {:.2}ms", 1000.0 / fps);
        }
    }
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("wgpu_core=info")).init();

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app)?;

    Ok(())
}
