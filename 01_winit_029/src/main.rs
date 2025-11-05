// cargo run -p step_01_winit_029
//! Animate 60 FPS

use pixels::{Pixels, SurfaceTexture};
use std::time::{Duration, Instant};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{/*ControlFlow,*/ EventLoop},
    window::{Window, WindowBuilder},
};

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const FPS: u64 = 60;
const FRAME_DURATION: Duration = Duration::from_micros(1_000_000 / FPS);

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

struct App {
    window: &'static Window, // This is not an Option
    pixels: Pixels<'static>, // This is not an Option
    last_frame: Instant,
    x_pos: i32,
    x_dir: i32,
}

impl App {
    fn new(event_loop: &EventLoop<()>) -> Result<Self> {
        let window = WindowBuilder::new().with_title("Step_01_winit_029: Animation").build(event_loop)?;

        let size = window.inner_size();
        let window_ref: &'static Window = Box::leak(Box::new(window));
        let surface = SurfaceTexture::new(size.width, size.height, window_ref);
        let pixels = Pixels::new(WIDTH, HEIGHT, surface)?;

        Ok(Self {
            window: window_ref,
            pixels,
            last_frame: Instant::now(),
            x_pos: 0,
            x_dir: 2,
        })
    }

    fn handle_window_event(&mut self, event: WindowEvent) -> bool {
        match event {
            WindowEvent::CloseRequested => true, // Signal pour quitter
            WindowEvent::RedrawRequested => {
                self.render();
                false
            }
            _ => false,
        }
    }

    fn render(&mut self) {
        // Animation
        self.x_pos += self.x_dir;
        if self.x_pos > WIDTH as i32 - 40 || self.x_pos < 0 {
            self.x_dir = -self.x_dir;
        }

        // Render
        let frame = self.pixels.frame_mut();
        frame.fill(0);
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
        self.pixels.render().unwrap();
    }

    fn request_redraw(&self) {
        self.window.request_redraw();
    }
}

fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;
    let mut app = App::new(&event_loop)?;

    event_loop.run(move |event, elwt| {
        // elwt.set_control_flow(ControlFlow::Poll);

        match event {
            Event::WindowEvent { event, .. } => {
                let should_exit = app.handle_window_event(event);
                if should_exit {
                    elwt.exit();
                }
            }
            Event::AboutToWait => {
                let now = Instant::now();
                // Limit to 60 FPS
                if now - app.last_frame >= FRAME_DURATION {
                    app.last_frame = now;
                    app.request_redraw();
                }
            }
            _ => {}
        }
    })?;

    Ok(())
}
