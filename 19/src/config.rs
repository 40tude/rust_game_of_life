// src/config.rs

use std::time::Duration;

pub const FPS: u64 = 60;
pub const FRAME_DURATION: Duration = Duration::from_micros(1_000_000 / FPS);
pub const CELL_SIZE: u32 = 4; // cells size in pixels (at zoom 1.0)
pub const DEFAULT_PATTERN_PATH: &str = r"rle/linepuffer.rle";
pub const TITLE: &str = "step_19: Add Zoom";

// Board dimensions (FIXED - does not change with zoom or window resize)
// This is the simulation grid size
pub const BOARD_WIDTH: u32 = 1280; // 356 
pub const BOARD_HEIGHT: u32 = 800; // 400

// Window dimensions at startup (in pixels)
pub const WINDOW_WIDTH: u32 = 1280;
pub const WINDOW_HEIGHT: u32 = 800;

// Zoom configuration
pub const ZOOM_FACTOR: f32 = 1.15; // Exponential zoom increment (1.15 = +15% per mouse wheel notch)
pub const ZOOM_MIN: f32 = 0.1; // Minimum zoom level (allows seeing ~10x more cells)
// ZOOM_MAX is calculated dynamically: min(window_w, window_h) / CELL_SIZE

// Performances
pub const PERF_SAMPLE_SIZE: usize = 60; // How many frames to average
pub const PERF_LOG_INTERVAL_SECS: u64 = 1; // Display frequency  (seconds)
