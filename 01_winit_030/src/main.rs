// cargo run -p step_01_winit_030
//! Animate 60 FPS

use pixels::{Pixels, SurfaceTexture};
use std::time::{Duration, Instant};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::Window,
};

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const FPS: u64 = 60;
const FRAME_DURATION: Duration = Duration::from_micros(1_000_000 / FPS);

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

struct App {
    window: Option<&'static Window>,
    pixels: Option<Pixels<'static>>,
    last_frame: Instant,
    x_pos: i32,
    x_dir: i32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            window: None,
            pixels: None,
            last_frame: Instant::now(),
            x_pos: 0,
            x_dir: 2,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create a window (its size can be different of WIDTH x HEIGHT)
        let window = event_loop.create_window(Window::default_attributes().with_title("Step_01_winit_030: Animation")).unwrap();

        // SurfaceTexture = bridge between buffer and the  window
        // Managage rescaling if the window is not WIDTH x HEIGHT
        let size = window.inner_size();
        // Leak the window to obtain a &'static Window for the app lifetime
        let window_ref: &'static Window = Box::leak(Box::new(window));

        let surface = SurfaceTexture::new(size.width, size.height, window_ref);

        // Pixels buffer with WIDTH x HEIGHT pixels
        let pixels = Pixels::new(WIDTH, HEIGHT, surface).unwrap();

        self.window = Some(window_ref);
        self.pixels = Some(pixels);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: winit::window::WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // Update the logic
                self.x_pos += self.x_dir;
                if self.x_pos > WIDTH as i32 - 40 || self.x_pos < 0 {
                    self.x_dir = -self.x_dir;
                }

                // Rendering
                if let Some(pixels) = &mut self.pixels {
                    let frame = pixels.frame_mut();

                    // Background is black
                    frame.fill(0);

                    // Moving the shape
                    for y in 100..140 {
                        for x in self.x_pos..(self.x_pos + 40) {
                            if x >= 0 && x < WIDTH as i32 {
                                let idx = ((y * WIDTH as i32 + x) * 4) as usize;
                                frame[idx] = 0xFF;
                                frame[idx + 1] = 0xFF;
                                frame[idx + 2] = 0xFF;
                                frame[idx + 3] = 0xFF;
                            }
                        }
                    }
                    pixels.render().unwrap();
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        let now = Instant::now();
        // Limit to 60 FPS
        if now - self.last_frame >= FRAME_DURATION {
            self.last_frame = now;
            // .window is guaranteed to be Some at this point (created in Event::Resumed)
            self.window.expect("Bug - Window should exist").request_redraw();
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
