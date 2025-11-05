// src/app/state.rs

// use crate::prelude::*;
use crate::{Result, app::perfs, app::render, config, gol::life, gol::utils}; // see lib.rs

use pixels::{Pixels, PixelsBuilder, SurfaceTexture, wgpu};
use rfd::FileDialog;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use winit::{
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{Key, NamedKey},
    window::{Fullscreen, Window, WindowBuilder},
};

pub struct App {
    pub window: Option<&'static Window>,
    pub pixels: Option<Pixels<'static>>,
    pub last_frame: Instant,
    pub board_width: u32,
    pub board_height: u32,
    pub board_current: Vec<bool>, // current grid of cells
    pub board_next: Vec<bool>,    // next grid of cells
    pub full_screen: bool,
    pub pending_resize: Option<(u32, u32)>,
    pub surface_w: u32, // size
    pub surface_h: u32,
    pub last_error: Option<String>,           // Error message to display
    pub error_display_until: Option<Instant>, // When to clear the error
    pub pattern_path: PathBuf,                // path to the `.rle` pattern file
    pub perf_metrics: perfs::PerformanceMetrics,
}

impl App {
    pub fn try_new(event_loop: &EventLoop<()>, path: &Path) -> Result<Self> {
        let mut app = Self {
            window: None,
            pixels: None,
            last_frame: Instant::now(),
            board_width: config::DEFAULT_BOARD_W,
            board_height: config::DEFAULT_BOARD_H,
            board_current: vec![false; (config::DEFAULT_BOARD_W * config::DEFAULT_BOARD_H) as usize],
            board_next: vec![false; (config::DEFAULT_BOARD_W * config::DEFAULT_BOARD_H) as usize],
            full_screen: false,
            pending_resize: None,
            surface_w: 0, // size of the window
            surface_h: 0,
            last_error: None,          // error message to overlay
            error_display_until: None, // how long to display the error message
            pattern_path: path.to_path_buf(),
            perf_metrics: perfs::PerformanceMetrics::new(config::PERF_SAMPLE_SIZE), // Average on 60 frames
        };

        // Now, do fallible work
        let path = PathBuf::from(&app.pattern_path);
        app.load_pattern(&path)?;

        let window = WindowBuilder::new()
            .with_title(config::TITLE)
            .with_inner_size(winit::dpi::PhysicalSize::new(config::WINDOW_WIDTH, config::WINDOW_HEIGHT))
            .build(event_loop)?;
        let window_ref: &'static Window = Box::leak(Box::new(window));
        app.window = Some(window_ref);

        // Trigger initial resize to create pixels buffer
        // Note: with_inner_size() will trigger a WindowEvent::Resized automatically
        // let size = window_ref.inner_size();
        // app.pending_resize = Some((size.width, size.height));

        Ok(app)
    }

    pub fn handle_window_event(&mut self, event: WindowEvent) -> bool {
        match event {
            WindowEvent::CloseRequested => true,

            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    logical_key,
                    state: ElementState::Pressed,
                    repeat: false,
                    ..
                },
                ..
            } => {
                // `F11` or `F` to toggle full screen
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
                    return false;
                }

                // `o` : to open .rle file
                if matches!(logical_key.as_ref(), Key::Character(s) if s.eq_ignore_ascii_case("o"))
                    && let Some(path) = FileDialog::new().add_filter("RLE files", &["rle"]).set_directory("rle/").pick_file()
                    && let Err(e) = self.load_pattern(&path)
                {
                    let error_msg = format!("Failed to load pattern: {}", e);
                    log::error!("{}", error_msg);
                    self.set_error(error_msg, 5); // Display error for 5 seconds

                    // Should I clear previous error on success? Something like
                    // match self.load_pattern(&path) {
                    //     Ok(_) => {
                    //         info!("Pattern loaded successfully from {:?}", path);
                    //         self.last_error = None; // Clear any previous error
                    //         self.error_display_until = None;
                    //     }
                    //     Err(e) => {
                    //         let error_msg = format!("Failed to load pattern: {}", e);
                    //         error!("{}", error_msg);
                    //         self.set_error(error_msg, 5); // Display error for 5 seconds
                    //     }
                    // }
                }
                false
            }

            WindowEvent::Resized(size) => {
                self.pending_resize = Some((size.width, size.height));
                log::debug!("WindowEvent::Resized(): pending_resize = {:?}", self.pending_resize);
                false
            }

            WindowEvent::ScaleFactorChanged { .. } => {
                if let Some(w) = self.window {
                    let s = w.inner_size();
                    self.pending_resize = Some((s.width, s.height));
                    log::debug!("WindowEvent::ScaleFactorChanged(): pending_resize = {:?}", self.pending_resize);
                }
                false
            }

            WindowEvent::RedrawRequested => {
                // Update the board & measure
                let step_start = Instant::now();
                life::step_life(&self.board_current, &mut self.board_next, self.board_width, self.board_height);
                std::mem::swap(&mut self.board_current, &mut self.board_next);
                let step_duration = step_start.elapsed();
                self.perf_metrics.record_step(step_duration);

                // Draw the current board & measure
                if let Some(pixels) = &mut self.pixels {
                    let render_start = Instant::now();
                    render::draw_board(pixels, &self.board_current, self.board_width, self.board_height);
                    let render_duration = render_start.elapsed();
                    self.perf_metrics.record_render(render_duration);

                    // TODO: Draw error overlay if there's an error. DO NOT MEASURE ???
                    if let Some(error_msg) = &self.last_error {
                        render::draw_error_overlay(pixels, error_msg, self.board_width, self.board_height);
                    }
                }

                // Display every second
                if self.perf_metrics.should_log(Duration::from_secs(config::PERF_LOG_INTERVAL_SECS))
                    && let (Some(avg_step), Some(avg_render), Some(p95_step)) = (self.perf_metrics.avg_step_time(), self.perf_metrics.avg_render_time(), self.perf_metrics.percentile_95_step())
                {
                    let total = avg_step + avg_render;
                    let fps_theoretical = if total.as_micros() > 0 { 1_000_000 / total.as_micros() } else { 0 };

                    log::info!(
                        "Perf: step={:>6.2}ms (p95={:>6.2}ms) | render={:>6.2}ms | total={:>6.2}ms | theo_fps={:>4} | board={}x{}",
                        avg_step.as_secs_f64() * 1000.0,
                        p95_step.as_secs_f64() * 1000.0,
                        avg_render.as_secs_f64() * 1000.0,
                        total.as_secs_f64() * 1000.0,
                        fps_theoretical,
                        self.board_width,
                        self.board_height
                    );
                }
                false
            }

            _ => false,
        }
    }

    pub fn request_redraw(&self) {
        self.window.expect("REASON").request_redraw();
    }

    // https://docs.rs/winit/latest/winit/application/trait.ApplicationHandler.html#method.about_to_wait
    pub fn about_to_wait(&mut self) {
        let now = Instant::now();
        // Limit to 60 FPS
        if now - self.last_frame >= config::FRAME_DURATION {
            self.last_frame = now;
            // Handle pending resize
            if let Some((w, h)) = self.pending_resize.take() {
                self.handle_resize(w, h)
            }

            // Update error display timer
            self.update_error_display();

            // .window is guaranteed to be Some at this point (created in Event::Resumed)
            self.window.expect("Bug - Window should exist").request_redraw();
        }
    }

    // Called by App::about_to_wait
    pub fn handle_resize(&mut self, win_w: u32, win_h: u32) {
        log::debug!("HEAD handle_resize(): win size = {}x{}", win_w, win_h);
        let buffer_w = (win_w / config::CELL_SIZE).max(10);
        let buffer_h = (win_h / config::CELL_SIZE).max(10);

        // Do nothing if no change
        // TODO : Can we simplify the if below?
        if self.surface_w == win_w && self.surface_h == win_h && self.board_width == buffer_w && self.board_height == buffer_h {
            log::debug!("handle_resize(): nothing to do.");
            return;
        }

        // Save previous state on resize
        let old_cells = std::mem::take(&mut self.board_current);
        let old_width = self.board_width;
        let old_height = self.board_height;

        // Create or resize pixels
        if let Some(pixels) = &mut self.pixels {
            // self.pixels is an Option<T>
            // If self.pixels exists this is a resize
            // let _ = pixels.resize_surface(win_w, win_h);
            // let _ = pixels.resize_buffer(buffer_w, buffer_h);
            // .expect() is OK here because if the GPU fails, the app cannot continue. This is an unrecoverable error.
            pixels.resize_surface(win_w, win_h).expect("GPU resize_surface failed - unrecoverable");
            pixels.resize_buffer(buffer_w, buffer_h).expect("GPU resize_buffer failed - unrecoverable");
            log::debug!("handle_resize(): Window created size = {}x{} pixels and new buffer size = {}x{}.", win_w, win_h, buffer_w, buffer_h);
        } else if let Some(window) = self.window {
            // Create pixels
            // self.window is an Option<T> created in App::resumed() with event_loop.create_window()
            // Create a surface texture attached to the window
            let surface = SurfaceTexture::new(win_w, win_h, window);

            // Create a Pixels with a rendering buffer (buffer_w, buffer_h)
            // Use high-performance GPU adapter (discrete GPU like NVIDIA/AMD instead of integrated)
            let mut pixels = PixelsBuilder::new(buffer_w, buffer_h, surface)
                // Force a backend explicitly:
                .wgpu_backend(wgpu::Backends::DX12) // pick one or the other
                // .wgpu_backend(wgpu::Backends::VULKAN)
                // (Optional) prefer the discrete GPU
                .request_adapter_options(wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance,
                    compatible_surface: None, // pixels fills this internally
                    force_fallback_adapter: false,
                })
                .build() // or .build_async().await
                .expect("Failed to create Pixels with high-performance GPU");

            pixels.set_present_mode(wgpu::PresentMode::Fifo); // Fifo — default mode (vsync activated)

            self.pixels = Some(pixels);
        }

        // Update size fields of the App
        self.surface_w = win_w;
        self.surface_h = win_h;
        self.board_width = buffer_w;
        self.board_height = buffer_h;

        // Create buffers with new size
        self.board_current = vec![false; (buffer_w * buffer_h) as usize];
        self.board_next = vec![false; (buffer_w * buffer_h) as usize];

        // Reposition existing pattern if any
        if !old_cells.is_empty() {
            // utils::place_pattern_centered(&mut self.board_current, self.board_width, self.board_height, &old_cells, old_width, old_height)?;
            log::debug!("handle_resize(): Call place_pattern_centered with buffer size = {}x{}.", buffer_w, buffer_h);
            utils::place_pattern_centered(&mut self.board_current, self.board_width, self.board_height, &old_cells, old_width, old_height);
        }

        log::debug!("TAIL handle_resize(): win size = {}x{} | buffer size = {}x{}", win_w, win_h, buffer_w, buffer_h);
    }

    // call by WindowEvent::KeyboardInput when  user press `o`
    pub fn load_pattern(&mut self, path: &Path) -> Result<()> {
        // clear the board because a simulation may be in progress
        self.board_current.fill(false);

        let (cells, width, height) = utils::read_rle(path)?;
        log::info!("{} pattern file loaded", path.display());

        // utils::place_pattern_centered(&mut self.board_current, self.board_width, self.board_height, &cells, width, height)?;
        utils::place_pattern_centered(&mut self.board_current, self.board_width, self.board_height, &cells, width, height);
        log::debug!("load_pattern(): Call place_pattern_centered with buffer size = {}x{}.", width, height);
        
        Ok(())
    }

    // Set an error message to display for a certain duration
    pub fn set_error(&mut self, message: String, duration_secs: u64) {
        self.last_error = Some(message);
        self.error_display_until = Some(Instant::now() + std::time::Duration::from_secs(duration_secs));
    }

    // Clear the error if the display time has expired
    pub fn update_error_display(&mut self) {
        if let Some(until) = self.error_display_until
            && Instant::now() >= until
        {
            self.last_error = None;
            self.error_display_until = None;
        }
    }

    // Get the current error message if any
    pub fn get_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }
}
