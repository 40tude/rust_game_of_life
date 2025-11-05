// src/config.rs

use std::time::Duration;

pub const FPS: u64 = 60;
pub const FRAME_DURATION: Duration = Duration::from_micros(1_000_000 / FPS);
pub const CELL_SIZE: u32 = 4; // cells size in pixels
pub const DEFAULT_PATTERN_PATH: &str = r"rle/linepuffer.rle";
pub const TITLE: &str = "step_17_winit_030: Performances Measurements";
pub const DEFAULT_BOARD_W: u32 = 712; // 356 712
pub const DEFAULT_BOARD_H: u32 = 400; // 200 400

// Window dimensions at startup (in pixels)
pub const WINDOW_WIDTH: u32 = DEFAULT_BOARD_W * CELL_SIZE; // 712 pixels
pub const WINDOW_HEIGHT: u32 = DEFAULT_BOARD_H * CELL_SIZE; // 400 pixels

// Performances
pub const PERF_SAMPLE_SIZE: usize = 60; // How many frames to average
pub const PERF_LOG_INTERVAL_SECS: u64 = 1; // Display frequency  (seconds)
