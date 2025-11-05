// src/app/state.rs

// use crate::prelude::*;
use crate::{Result, app::perfs, config, gol::utils}; // see lib.rs

use pixels::{Pixels, PixelsBuilder, SurfaceTexture, wgpu};
use std::path::{Path, PathBuf};
// use std::time::Duration;
use std::time::Instant;
use winit::window::Window;

pub struct App {
    pub window: Option<&'static Window>,
    pub pixels: Option<Pixels<'static>>,
    pub last_frame: Instant,
    pub board_width: u32,         // FIXED board size (simulation grid)
    pub board_height: u32,        // FIXED board size (simulation grid)
    pub board_current: Vec<bool>, // current grid of cells
    pub board_next: Vec<bool>,    // next grid of cells
    pub full_screen: bool,
    pub pending_resize: Option<(u32, u32)>,
    pub surface_w: u32,                       // window size in pixels
    pub surface_h: u32,                       // window size in pixels
    pub last_error: Option<String>,           // Error message to display
    pub error_display_until: Option<Instant>, // When to clear the error
    pub pattern_path: PathBuf,                // path to the `.rle` pattern file
    pub perf_metrics: perfs::PerformanceMetrics,
    pub zoom_level: f32, // Current zoom level (1.0 = default, affects display only)
    pub zoom_max: f32,   // Maximum zoom level (dynamically calculated)
    pub camera_x: f32,   // Camera position (center of view in board coordinates)
    pub camera_y: f32,   // Camera position (center of view in board coordinates)
}

impl App {
    pub fn try_new(path: &Path) -> Result<Self> {
        let mut app = Self {
            window: None,
            pixels: None,
            last_frame: Instant::now(),
            board_width: config::BOARD_WIDTH,   // FIXED board size
            board_height: config::BOARD_HEIGHT, // FIXED board size
            board_current: vec![false; (config::BOARD_WIDTH * config::BOARD_HEIGHT) as usize],
            board_next: vec![false; (config::BOARD_WIDTH * config::BOARD_HEIGHT) as usize],
            full_screen: false,
            pending_resize: None,
            surface_w: 0, // size of the window
            surface_h: 0,
            last_error: None,          // error message to overlay
            error_display_until: None, // how long to display the error message
            pattern_path: path.to_path_buf(),
            perf_metrics: perfs::PerformanceMetrics::new(config::PERF_SAMPLE_SIZE),                 // Average on 60 frames
            zoom_level: 1.0,                                                                        // Default zoom
            zoom_max: (config::WINDOW_WIDTH.min(config::WINDOW_HEIGHT) / config::CELL_SIZE) as f32, // Initial zoom_max
            camera_x: (config::BOARD_WIDTH / 2) as f32,                                             // Start centered on board
            camera_y: (config::BOARD_HEIGHT / 2) as f32,                                            // Start centered on board
        };

        // Now, do the fallible work
        let path = PathBuf::from(&app.pattern_path);
        app.load_pattern(&path)?;

        Ok(app)
    }

    // Called by App::about_to_wait()
    pub fn handle_resize(&mut self, win_w: u32, win_h: u32) {
        log::debug!("HEAD handle_resize(): win size = {}x{}", win_w, win_h);

        // Update zoom_max based on new window size
        // So with a 1280×800 window and CELL_SIZE=4, we can zoom up to 200x, but not more
        self.zoom_max = (win_w.min(win_h) / config::CELL_SIZE) as f32;

        // Do nothing if window size hasn't changed
        if self.surface_w == win_w && self.surface_h == win_h {
            log::debug!("handle_resize(): nothing to do.");
            return;
        }

        // Buffer size matches the board size (fixed grid) - only surface size changes with window
        let buffer_w = self.board_width;
        let buffer_h = self.board_height;

        // Create or resize pixels
        if let Some(pixels) = &mut self.pixels {
            // self.pixels is an Option<T>
            // If self.pixels exists this is a resize
            let _ = pixels.resize_surface(win_w, win_h);
            let _ = pixels.resize_buffer(buffer_w, buffer_h);
        } else if let Some(window) = self.window {
            // Create pixels
            // self.window is an Option<T> created in App::resumed() with event_loop.create_window()
            // Create a surface texture attached to the window
            let surface = SurfaceTexture::new(win_w, win_h, window);

            // Create a Pixels with a rendering buffer (buffer_w, buffer_h)
            let mut pixels = PixelsBuilder::new(buffer_w, buffer_h, surface)
                .request_adapter_options(wgpu::RequestAdapterOptions {
                    //
                    // 1 - GPU: Pick one or the other
                    power_preference: wgpu::PowerPreference::LowPower,
                    // power_preference: wgpu::PowerPreference::HighPerformance,
                    //
                    compatible_surface: None,
                    force_fallback_adapter: false,
                })
                //
                // 2 - Backend: Pick one or the other
                .wgpu_backend(wgpu::Backends::VULKAN)
                //.wgpu_backend(wgpu::Backends::DX12)
                //
                .build() // or .build_async().await
                .expect("Failed to create Pixels with high-performance GPU");

            // 3 - PresentationMode: Pick one or the other
            pixels.set_present_mode(wgpu::PresentMode::Fifo);
            // pixels.set_present_mode(wgpu::PresentMode::Immediate);

            log::info!("Present mode: {:?}", pixels.present_mode());

            self.pixels = Some(pixels);
        }

        // Update window size
        self.surface_w = win_w;
        self.surface_h = win_h;

        log::debug!("TAIL handle_resize(): win size = {}x{} | board size = {}x{} (fixed)", win_w, win_h, self.board_width, self.board_height);
    }

    // call by WindowEvent::KeyboardInput when  user press `o`
    pub fn load_pattern(&mut self, path: &Path) -> Result<()> {
        // clear the board because a simulation may be in progress
        self.board_current.fill(false);

        let (cells, width, height) = utils::read_rle(path)?;
        log::info!("{} pattern file loaded", path.display());

        log::debug!("load_pattern(): Call place_pattern_centered() with buffer size = {}x{}.", width, height);
        // utils::place_pattern_centered(&mut self.board_current, self.board_width, self.board_height, &cells, width, height)?;
        utils::place_pattern_centered(&mut self.board_current, self.board_width, self.board_height, &cells, width, height);

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

    // Handle zoom changes from mouse wheel
    pub fn handle_zoom(&mut self, delta: f32) {
        let old_zoom = self.zoom_level;

        // Apply exponential zoom increment
        if delta > 0.0 {
            self.zoom_level *= config::ZOOM_FACTOR;
        } else {
            self.zoom_level /= config::ZOOM_FACTOR;
        }

        // Clamp to min/max limits
        self.zoom_level = self.zoom_level.clamp(config::ZOOM_MIN, self.zoom_max);

        // If zoom actually changed, just log it (no board resize!)
        if (self.zoom_level - old_zoom).abs() > f32::EPSILON {
            log::debug!(
                "Zoom changed: {:.2} -> {:.2} (board remains {}x{}, viewport changes)",
                old_zoom,
                self.zoom_level,
                self.board_width,
                self.board_height
            );
            // The render function will automatically show more/less of the board
            // based on zoom_level when drawing
        }
    }
}
