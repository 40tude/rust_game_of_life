// src/app/state.rs

// use crate::prelude::*; // see lib.rs
use crate::{Result, config, gol::utils}; // see lib.rs

use pixels::{Pixels, PixelsBuilder, SurfaceTexture, wgpu};
use std::path::{Path, PathBuf};
use std::time::Instant;
use winit::window::Window;

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
}

impl App {
    pub fn try_new(path: &Path) -> Result<Self> {
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
        };

        // Now, do fallible work
        let path = PathBuf::from(&app.pattern_path);
        app.load_pattern(&path)?;

        Ok(app)
    }

    // Called by App::about_to_wait()
    pub fn handle_resize(&mut self, win_w: u32, win_h: u32) -> Result<()> {
        let buffer_w = (win_w / config::CELL_SIZE).max(10);
        let buffer_h = (win_h / config::CELL_SIZE).max(10);

        // Do nothing if no change
        if self.surface_w == win_w && self.surface_h == win_h && self.board_width == buffer_w && self.board_height == buffer_h {
            log::info!("handle_resize(): nothing to do.");
            return Ok(());
        }

        // Save previous state on resize
        let old_cells = std::mem::take(&mut self.board_current);
        let old_width = self.board_width;
        let old_height = self.board_height;

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
            utils::place_pattern_centered(&mut self.board_current, self.board_width, self.board_height, &old_cells, old_width, old_height)?;
        }

        log::info!("handle_resize(): Window new size = {}x{} pixels and buffer new size = {}x{}.", win_w, win_h, buffer_w, buffer_h);
        Ok(())
    }

    // call by WindowEvent::KeyboardInput when  user press `o`
    pub fn load_pattern(&mut self, path: &Path) -> Result<()> {
        // clear the board because a simulation may be in progress
        self.board_current.fill(false);

        let (cells, width, height) = utils::read_rle(path)?;
        log::info!("{} pattern file loaded", path.display());
        utils::place_pattern_centered(&mut self.board_current, self.board_width, self.board_height, &cells, width, height)?;
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
