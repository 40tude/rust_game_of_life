// cargo run -p step_06
//! First gol animation

use pixels::{Pixels, SurfaceTexture};
use std::fs;
use std::time::{Duration, Instant};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    window::{Fullscreen, Window},
};

const FPS: u64 = 60;
const FRAME_DURATION: Duration = Duration::from_micros(1_000_000 / FPS);
const CELL_SIZE: u32 = 4; // cells size in pixels
const PATTERN_FILE_PATH: &str = "rle/gosperglidergun.rle";

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

struct App {
    window: Option<&'static Window>,
    pixels: Option<Pixels<'static>>,
    last_frame: Instant,
    buffer_width: u32,
    buffer_height: u32,
    board_current: Vec<bool>, // current grid of cells
    board_next: Vec<bool>,    // next grid of cells
    full_screen: bool,
    pending_resize: Option<(u32, u32)>,
    surface_w: u32, // size
    surface_h: u32,
}

impl Default for App {
    fn default() -> Self {
        // Define initial values here, see why below
        let buffer_width = 178;
        let buffer_height = 100;

        let mut app = Self {
            window: None,
            pixels: None,
            last_frame: Instant::now(),
            buffer_width, // size of the buffer pixels buffer
            buffer_height,
            board_current: vec![false; (buffer_width * buffer_height) as usize], // can use initial values
            board_next: vec![false; (buffer_width * buffer_height) as usize],
            full_screen: false,
            pending_resize: None,
            surface_w: 0, // size of the window
            surface_h: 0,
        };

        // Load and center pattern
        if let Ok((cells, width, height)) = read_rle(PATTERN_FILE_PATH) {
            app.place_pattern_centered(cells, width, height);
        } else {
            app.cells_in_corners(); // fallback
        }

        app
    }
}

// Read an RLE file and provide (pattern_cells, pattern_width, pattern_height)
pub fn read_rle(filename: &str) -> Result<(Vec<bool>, u32, u32)> {
    let content = fs::read_to_string(filename)?;

    let mut pattern_width = 0;
    let mut pattern_height = 0;
    let mut pattern_lines = Vec::new();

    // Parse headers and data
    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue; // Ignore empty line and comments
        }

        if line.starts_with('x') {
            // Header line : x = 5, y = 3
            for part in line.split(',') {
                let part = part.trim();
                if part.starts_with("x =") {
                    pattern_width = part[3..].trim().parse()?;
                } else if part.starts_with("y =") {
                    pattern_height = part[3..].trim().parse()?;
                }
                // "rule = ..." is ignored for now
            }
        } else {
            // A line of pattern
            pattern_lines.push(line);
        }
    }

    if pattern_width == 0 || pattern_height == 0 {
        return Err("Pattern size (x or y) not found in RLE file".into());
    }

    // Join pattern's lines (the RLE has $ as EOL)
    let pattern_data = pattern_lines.join("");

    let cells = parse_rle_data(&pattern_data, pattern_width as usize, pattern_height as usize)?;

    Ok((cells, pattern_width, pattern_height))
}

// Parse the RLE pattern as a single string (ex: "3b2o$2o2b2o!")
fn parse_rle_data(pattern_string: &str, pattern_width: usize, pattern_height: usize) -> Result<Vec<bool>> {
    let mut cells = vec![false; pattern_width * pattern_height];
    let mut x = 0;
    let mut y = 0;
    let mut count_str = String::new();

    for c in pattern_string.chars() {
        match c {
            '0'..='9' => {
                count_str.push(c);
            }
            'b' | 'o' => {
                let count = if count_str.is_empty() { 1 } else { count_str.parse::<usize>()? };
                count_str.clear();

                let cell_state = c == 'o'; // 'o' = live, 'b' = dead

                for _ in 0..count {
                    if x < pattern_width && y < pattern_height {
                        cells[y * pattern_width + x] = cell_state;
                    }
                    x += 1;
                }
            }
            '$' => {
                // New line
                let count = if count_str.is_empty() { 1 } else { count_str.parse::<usize>()? };
                count_str.clear();

                y += count;
                x = 0;
            }
            '!' => {
                // End of the pattern string
                break;
            }
            _ => {
                // Ignore other char (paces, etc.)
            }
        }
    }

    Ok(cells)
}

impl App {
    // Called by App::about_to_wait()
    fn handle_resize(&mut self, win_w: u32, win_h: u32) {
        let buffer_w = (win_w / CELL_SIZE).max(10);
        let buffer_h = (win_h / CELL_SIZE).max(10);

        // Do nothing if no change
        if self.surface_w == win_w && self.surface_h == win_h && self.buffer_width == buffer_w && self.buffer_height == buffer_h {
            println!("handle_resize(): nothing to do.");
            return;
        }

        // Save previous state on resize
        let old_cells = std::mem::take(&mut self.board_current);
        let old_width = self.buffer_width;
        let old_height = self.buffer_height;

        // Create or resize pixels
        if let Some(pixels) = &mut self.pixels {
            // self.pixels is an Option<T>
            // If self.pixels exists this is a resize
            let _ = pixels.resize_surface(win_w, win_h);
            let _ = pixels.resize_buffer(buffer_w, buffer_h);
        } else if let Some(window) = self.window {
            // This is a create
            // self.window is an Option<T> created in App::resumed() with event_loop.create_window()
            // Create a surface texture attached to the window
            let surface = SurfaceTexture::new(win_w, win_h, window);
            // Create a Pixels with a rendering buffer (buffer_w, buffer_h)
            self.pixels = Some(Pixels::new(buffer_w, buffer_h, surface).expect("pixels"));
        }

        // Update size fields of the App
        self.surface_w = win_w;
        self.surface_h = win_h;
        self.buffer_width = buffer_w;
        self.buffer_height = buffer_h;

        // Create buffers with new size
        self.board_current = vec![false; (buffer_w * buffer_h) as usize];
        self.board_next = vec![false; (buffer_w * buffer_h) as usize];

        // Reposition existing pattern if any
        if !old_cells.is_empty() {
            self.place_pattern_centered(old_cells, old_width, old_height);
        } else {
            self.cells_in_corners();
        }

        println!("handle_resize(): Window new size = {}x{} pixels and buffer new size = {}x{}.", win_w, win_h, buffer_w, buffer_h);
    }

    // Call by App::default and App::handle_resize
    // Put 4 living cells in the corners (mostly for debug at the beginning)
    fn cells_in_corners(&mut self) {
        // Clear
        self.board_current.fill(false);

        // Nothing to do if on of the size is 0
        if self.buffer_width == 0 || self.buffer_height == 0 {
            println!("cells_in_corners(): Nothing to do.");
            return;
        }

        let w = self.buffer_width as usize;
        let h = self.buffer_height as usize;

        // Corners: (0,0), (w-1,0), (0,h-1), (w-1,h-1)
        let tl = 0; // top-left
        let tr = w - 1; // top-right
        let bl = (h - 1) * w; // bottom-left
        let br = bl + (w - 1); // bottom-right

        self.board_current[tl] = true;
        self.board_current[tr] = true;
        self.board_current[bl] = true;
        self.board_current[br] = true;
    }

    // Compute one step: current -> next (row-major, no wrapping).
    // Cells outside the board are considered dead.
    pub fn step_life(&mut self) {
        debug_assert_eq!(self.board_current.len(), self.board_next.len());

        let get = |x: isize, y: isize| -> u8 {
            if x < 0 || y < 0 {
                return 0;
            }
            let (x, y) = (x as usize, y as usize);
            if x >= self.buffer_width as usize || y >= self.buffer_height as usize {
                return 0;
            }
            self.board_current[y * self.buffer_width as usize + x] as u8
        };

        for y in 0..self.buffer_height {
            for x in 0..self.buffer_width {
                let xi = x as isize;
                let yi = y as isize;

                let mut n = 0u8;
                n += get(xi - 1, yi - 1);
                n += get(xi, yi - 1);
                n += get(xi + 1, yi - 1);
                n += get(xi - 1, yi);
                n += get(xi + 1, yi);
                n += get(xi - 1, yi + 1);
                n += get(xi, yi + 1);
                n += get(xi + 1, yi + 1);

                let idx: usize = (y * self.buffer_width + x) as usize;
                let alive = self.board_current[idx];

                self.board_next[idx] = match (alive, n) {
                    (true, 2) | (_, 3) => true, // survive with 2; birth/survive with 3
                    _ => false,
                };
            }
        }
    }

    // Place a pattern at the center of the board
    pub fn place_pattern_centered(&mut self, pattern_cells: Vec<bool>, pattern_width: u32, pattern_height: u32) {
        // Make sure the board is initialized
        if self.board_current.is_empty() {
            self.board_current = vec![false; (self.buffer_width * self.buffer_height) as usize];
            println!("place_pattern_centered(): Buffer was not initialized.");
        }

        // Compute offsets to center the pattern
        let offset_x = (self.buffer_width as i32 - pattern_width as i32) / 2;
        let offset_y = (self.buffer_height as i32 - pattern_height as i32) / 2;

        // Copy the pattern while centering it
        for y in 0..pattern_height {
            for x in 0..pattern_width {
                let pattern_idx = (y * pattern_width + x) as usize;
                let buffer_x = offset_x + x as i32;
                let buffer_y = offset_y + y as i32;

                // Check versus buffer's limits
                if buffer_x >= 0 && buffer_x < self.buffer_width as i32 && buffer_y >= 0 && buffer_y < self.buffer_height as i32 {
                    let buffer_idx = (buffer_y as u32 * self.buffer_width + buffer_x as u32) as usize;

                    if pattern_idx < pattern_cells.len() && buffer_idx < self.board_current.len() {
                        self.board_current[buffer_idx] = pattern_cells[pattern_idx];
                    }
                }
            }
        }

        println!(
            "place_pattern_centered(): Pattern ({}x{}) centered in buffer ({}x{}).",
            pattern_width, pattern_height, self.buffer_width, self.buffer_height
        );
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(Window::default_attributes().with_title("Step_06: First live pattern")).unwrap();
        let window_ref: &'static Window = Box::leak(Box::new(window));
        self.window = Some(window_ref);
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
                    self.full_screen = !self.full_screen;

                    if let Some(window) = &self.window {
                        if self.full_screen {
                            window.set_fullscreen(Some(Fullscreen::Borderless(None)));
                        } else {
                            window.set_fullscreen(None);
                        }
                    }
                }
            }

            WindowEvent::Resized(size) => {
                self.pending_resize = Some((size.width, size.height));
            }

            WindowEvent::ScaleFactorChanged { .. } => {
                if let Some(w) = self.window {
                    let s = w.inner_size();
                    self.pending_resize = Some((s.width, s.height));
                }
            }

            WindowEvent::RedrawRequested => {
                // Update the board
                self.step_life();
                std::mem::swap(&mut self.board_current, &mut self.board_next);

                if let Some(pixels) = &mut self.pixels {
                    let frame = pixels.frame_mut();

                    // Draw the current board (from bool to RGBA)
                    for y in 0..self.buffer_height {
                        for x in 0..self.buffer_width {
                            let cell_idx = (y * self.buffer_width + x) as usize;
                            let is_alive = self.board_current.get(cell_idx).copied().unwrap_or(false);

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
            if let Some((w, h)) = self.pending_resize.take() {
                self.handle_resize(w, h);
            }
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
