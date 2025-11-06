// cargo run -p step_02
//! Full screen avec F11 ou F
//! Le buffer reste WIDTH x HEIGHT, juste la fenêtre change

use pixels::{Pixels, SurfaceTexture};
use std::time::{Duration, Instant};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    window::{Fullscreen, Window},
};

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const FPS: u64 = 60;
const FRAME_DURATION: Duration = Duration::from_micros(1_000_000 / FPS);

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

// #[derive(Default)]
struct App {
    window: Option<&'static Window>,
    pixels: Option<Pixels<'static>>,
    last_frame: Instant,
    frame_count: u32,
    fullscreen: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            window: None,
            pixels: None,
            last_frame: Instant::now(),
            frame_count: 0,
            fullscreen: false,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create a window (its size can be different of WIDTH x HEIGHT)
        let window = event_loop.create_window(Window::default_attributes().with_title("Step_02: Win resize - Buff same size")).unwrap();

        // SurfaceTexture = bridge between buffer and the  window
        // Manage rescaling if the window is not WIDTH x HEIGHT
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

            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    logical_key,
                    state: ElementState::Pressed, // we want that the key is pressed
                    repeat: false, // Not a repetition (key held down)
                    .. // Ignore: text, location, platform_specific
                },
                ..
            } => {
                // F11 or F to toggle fullscreen
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
                // Important: resize the surface when the window's size change
                if let Some(pixels) = &mut self.pixels {
                    pixels.resize_surface(size.width, size.height).unwrap();
                }
            }

            WindowEvent::RedrawRequested => {
                // -------------------------------------------------------------------------
                // Uncomment this block to display buffer/window ratios once per second.
                // It helps visualize why black borders appear when aspect ratios differ.
                // -------------------------------------------------------------------------
                /*
                {
                    use std::sync::OnceLock;
                    use std::time::{Duration, Instant};

                    // A static timestamp that persists between redraws (safe & simple)
                    static LAST_LOG: OnceLock<std::sync::Mutex<Instant>> = OnceLock::new();

                    let now = Instant::now();
                    let last_lock = LAST_LOG.get_or_init(|| std::sync::Mutex::new(Instant::now()));
                    let mut last = last_lock.lock().unwrap();

                    if now.duration_since(*last) >= Duration::from_secs(1) {
                        *last = now;

                        let window = self.window.unwrap();
                        let size = window.inner_size();
                        let win_ratio = size.width as f32 / size.height as f32;
                        let buf_ratio = WIDTH as f32 / HEIGHT as f32;

                        println!(
                            "Buffer = {}x{}, Surface = {}x{}, Window ratio = {:.3}, Buffer ratio = {:.3}",
                            WIDTH, HEIGHT, size.width, size.height, win_ratio, buf_ratio
                        );
                    }
                }
                */

                if let Some(pixels) = &mut self.pixels {
                    let frame = pixels.frame_mut();

                    // Animated gradient
                    for y in 0..HEIGHT {
                        for x in 0..WIDTH {
                            let idx = ((y * WIDTH + x) * 4) as usize;
                            let t = self.frame_count as f32 * 0.05;
                            frame[idx] = ((x as f32 + t).sin() * 127.0 + 128.0) as u8;
                            frame[idx + 1] = ((y as f32 + t).cos() * 127.0 + 128.0) as u8;
                            frame[idx + 2] = 0x40;
                            frame[idx + 3] = 0xFF;
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
            self.frame_count += 1;
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
