// cargo run -p step_03
//! Dynamically sized buffer according to the window size

use pixels::{Pixels, SurfaceTexture};
use std::time::{Duration, Instant};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    window::{Fullscreen, Window},
};

const FPS: u64 = 60;
const FRAME_DURATION: Duration = Duration::from_micros(1_000_000 / FPS);
const CELL_SIZE: u32 = 4; // size of a cell in pixels

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

// #[derive(Default)]
struct App {
    window: Option<&'static Window>,
    pixels: Option<Pixels<'static>>,
    last_frame: Instant,
    buffer_width: u32,
    buffer_height: u32,
    cells: Vec<bool>, // Grid of cells (for Game of Life later)
    fullscreen: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            window: None,
            pixels: None,
            last_frame: Instant::now(),
            buffer_width: 160,
            buffer_height: 120,
            cells: Vec::new(),
            fullscreen: false,
        }
    }
}

impl App {
    fn recreate_buffer(&mut self, window_size: PhysicalSize<u32>) {
        // Calculate the size of the buffer according to the size of the window
        // Cells are of dimension CELL_SIZE x CELL_SIZE pixels
        let buffer_width = (window_size.width / CELL_SIZE).max(10);
        let buffer_height = (window_size.height / CELL_SIZE).max(10);

        // Take the &'static Window (not &&Window)
        if let Some(window) = self.window {
            let surface = SurfaceTexture::new(window_size.width, window_size.height, window);
            let pixels = Pixels::new(buffer_width, buffer_height, surface).unwrap();

            self.pixels = Some(pixels);
            self.buffer_width = buffer_width;
            self.buffer_height = buffer_height;

            // Create the universe (grid of cells)
            let total_cells = (buffer_width * buffer_height) as usize;
            self.cells = vec![false; total_cells];

            // Scatter some cells
            for i in 0..total_cells / 10 {
                let idx = (i * 7) % total_cells;
                self.cells[idx] = true;
            }

            println!("Buffer resized: {}x{} cells ({}x{} pixels)", buffer_width, buffer_height, window_size.width, window_size.height);
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(Window::default_attributes().with_title("Step_03: Resize me")).unwrap();

        let size = window.inner_size();
        let window_ref: &'static Window = Box::leak(Box::new(window));

        self.window = Some(window_ref);
        self.recreate_buffer(size);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: winit::window::WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    logical_key,
                    state: ElementState::Pressed,
                    repeat: false,
                    ..
                },
                ..
            } => {
                let is_fullscreen_key = matches!(logical_key, Key::Named(NamedKey::F11)) || matches!(logical_key.as_ref(), Key::Character(s) if s.eq_ignore_ascii_case("f"));

                if is_fullscreen_key {
                    self.fullscreen = !self.fullscreen;

                    if let Some(window) = &self.window {
                        if self.fullscreen {
                            window.set_fullscreen(Some(Fullscreen::Borderless(None)));
                        } else {
                            window.set_fullscreen(None);
                        }
                    }
                }
            }

            WindowEvent::Resized(size) => {
                // Recreate the buffer and the universe
                self.recreate_buffer(size);
            }

            WindowEvent::RedrawRequested => {
                if let Some(pixels) = &mut self.pixels {
                    let frame = pixels.frame_mut();

                    // Draw the grid of cells
                    for y in 0..self.buffer_height {
                        for x in 0..self.buffer_width {
                            let cell_idx = (y * self.buffer_width + x) as usize;
                            let is_alive = self.cells.get(cell_idx).copied().unwrap_or(false);

                            let pixel_idx = ((y * self.buffer_width + x) * 4) as usize;
                            if is_alive {
                                frame[pixel_idx] = 0xFF;
                                frame[pixel_idx + 1] = 0xFF;
                                frame[pixel_idx + 2] = 0xFF;
                                frame[pixel_idx + 3] = 0xFF;
                            } else {
                                frame[pixel_idx] = 0x10;
                                frame[pixel_idx + 1] = 0x10;
                                frame[pixel_idx + 2] = 0x10;
                                frame[pixel_idx + 3] = 0xFF;
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
        if now - self.last_frame >= FRAME_DURATION {
            self.last_frame = now;
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
