// cargo run -p step_04
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
const CELL_SIZE: u32 = 16; // size of a cell in pixels

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

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
    fn create_buffer(&mut self, window_size: PhysicalSize<u32>) {
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

            println!("Buffer created: {}x{} cells ({}x{} pixels)", buffer_width, buffer_height, window_size.width, window_size.height);
        }
    }

    fn handle_resize(&mut self, w: u32, h: u32) {
        let bw = (w / CELL_SIZE).max(10);
        let bh = (h / CELL_SIZE).max(10);

        if let Some(pixels) = &mut self.pixels {
            // 1) Surface = dimensions of the window
            pixels.resize_surface(w, h).ok();

            // 2) Logical buffer = nombre of cells
            pixels.resize_buffer(bw, bh).ok();
        } else if let Some(window) = self.window {
            // This only happen on creation when self.pixels is not yet Some()
            let surface = SurfaceTexture::new(w, h, window);
            self.pixels = Some(Pixels::new(bw, bh, surface).expect("pixels"));
        }

        self.buffer_width = bw;
        self.buffer_height = bh;

        self.cells = vec![false; (bw * bh) as usize];
        self.init_corner_cells();

        println!("Buffer resized: {}x{} cells ({}x{} pixels)", bw, bh, w, h);
    }

    fn init_corner_cells(&mut self) {
        // Background in black
        self.cells.fill(false);

        // Do nothing if dimensions are 0
        if self.buffer_width == 0 || self.buffer_height == 0 {
            return;
        }

        let w = self.buffer_width as usize;
        let h = self.buffer_height as usize;

        // Corners : (0,0), (w-1,0), (0,h-1), (w-1,h-1)
        let tl = 0; // top-left
        let tr = w - 1; // top-right
        let bl = (h - 1) * w; // bottom-left
        let br = bl + (w - 1); // bottom-right

        self.cells[tl] = true;
        self.cells[tr] = true;
        self.cells[bl] = true;
        self.cells[br] = true;
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(Window::default_attributes().with_title("Step_04: Resize me II")).unwrap();

        let size = window.inner_size();
        let window_ref: &'static Window = Box::leak(Box::new(window));

        self.window = Some(window_ref);
        self.create_buffer(size);
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
                self.handle_resize(size.width, size.height);
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
