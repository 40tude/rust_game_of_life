// src/config.rs

use std::time::Duration;

pub const FPS: u64 = 60;
pub const FRAME_DURATION: Duration = Duration::from_micros(1_000_000 / FPS);
pub const CELL_SIZE: u32 = 4; // cells size in pixels
pub const PATTERN_FILE_PATH: &str = r"rle/gosperglidergun.rle"; // 
pub const TITLE: &str = "Step_08: Modularization++ & Load pattern";
pub const DEFAULT_BOARD_W: u32 = 178; // 178
pub const DEFAULT_BOARD_H: u32 = 100; // 100
