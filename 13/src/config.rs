// src/config.rs

use std::time::Duration;

pub const FPS: u64 = 60;
pub const FRAME_DURATION: Duration = Duration::from_micros(1_000_000 / FPS);
pub const CELL_SIZE: u32 = 4; // cells size in pixels
pub const DEFAULT_PATTERN_PATH: &str = r"rle/linepuffer.rle";
pub const TITLE: &str = "Step_13: Log";
pub const DEFAULT_BOARD_W: u32 = 178; // 178
pub const DEFAULT_BOARD_H: u32 = 100; // 100

// Window dimensions at startup (in pixels)
pub const WINDOW_WIDTH: u32 = DEFAULT_BOARD_W * CELL_SIZE; // 712 pixels
pub const WINDOW_HEIGHT: u32 = DEFAULT_BOARD_H * CELL_SIZE; // 400 pixels
