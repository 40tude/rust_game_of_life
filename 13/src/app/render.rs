// src/app/render.rs

use crate::prelude::*; // see lib.rs
use pixels::Pixels;

pub fn draw_board(pixels: &mut Pixels, board_current: &[bool], buffer_width: u32, buffer_height: u32) {
    let frame = pixels.frame_mut();

    // Drawing => from bool to RGBA
    for y in 0..buffer_height {
        for x in 0..buffer_width {
            let cell_idx = (y * buffer_width + x) as usize;
            let is_alive = board_current.get(cell_idx).copied().unwrap_or(false);

            let pixel_idx = ((y * buffer_width + x) * 4) as usize;
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

// Draw error message overlay on the screen
pub fn draw_error_overlay(pixels: &mut Pixels, error_message: &str, buffer_width: u32, buffer_height: u32) {
    let frame = pixels.frame_mut();

    // Draw a semi-transparent red bar at the top (20 pixels height)
    let bar_height = 20.min(buffer_height);

    for y in 0..bar_height {
        for x in 0..buffer_width {
            let pixel_idx = ((y * buffer_width + x) * 4) as usize;

            // Semi-transparent red background
            frame[pixel_idx] = 0xCC; // R
            frame[pixel_idx + 1] = 0x33; // G
            frame[pixel_idx + 2] = 0x33; // B
            frame[pixel_idx + 3] = 0xDD; // A (semi-transparent)
        }
    }

    // For text rendering we need a font rendering library like `fontdue`
    // For now, we just show a colored bar.
    // The error is also logged
    error!("Error displayed: {}", error_message);
}
