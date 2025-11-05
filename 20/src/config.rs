// src/config.rs

use std::time::Duration;

pub const FPS: u64 = 60;
pub const FRAME_DURATION: Duration = Duration::from_micros(1_000_000 / FPS);
pub const CELL_SIZE: u32 = 4; // cells size in pixels (at zoom 1.0)
pub const DEFAULT_PATTERN_DIR: &str = r"rle/";
pub const DEFAULT_PATTERN_PATH: &str = r"rle/linepuffer.rle";
pub const TITLE: &str = "step_20: Add Panning";

// Board dimensions (FIXED - does not change with zoom or window resize)
// This is the simulation grid size
pub const BOARD_WIDTH: u32 = 1280; //Large enough for most patterns 2560x1600
pub const BOARD_HEIGHT: u32 = 800;

// Window dimensions at startup (in pixels)
pub const WINDOW_WIDTH: u32 = 1280;
pub const WINDOW_HEIGHT: u32 = 800;

// Zoom configuration
pub const ZOOM_FACTOR: f32 = 1.15; // Exponential zoom increment (1.15 = +15% per mouse wheel notch)
pub const ZOOM_MIN: f32 = 0.1; // Minimum zoom level (allows seeing ~10x more cells)
// ZOOM_MAX is calculated dynamically: min(window_w, window_h) / CELL_SIZE

// Panning configuration
pub const PAN_STEP: f32 = 20.0; // Number of cells to move per arrow key press

// Color configuration (RGBA format: 0xRRGGBBAA)
pub const COLOR_CELL_ALIVE: u32 = 0xFFFFFFFF; // White - living cells
pub const COLOR_CELL_DEAD: u32 = 0x101010FF; // Very dark gray - dead cells inside board
pub const COLOR_OUT_OF_BOUNDS: u32 = 0x1A1A2EFF; // Dark blue-gray - area outside board bounds

// Performances
pub const PERF_SAMPLE_SIZE: usize = 60; // How many frames to average
pub const PERF_LOG_INTERVAL_SECS: u64 = 1; // Display frequency  (seconds)
