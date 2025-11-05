// src/app/render.rs

use crate::config;
// use crate::prelude::*; // see lib.rs
use pixels::Pixels;

// Draw the visible portion of the board according to camera position and zoom
///
// - `board_*`: Full simulation grid (FIXED size, e.g. 2000x1500)
// - `camera_x/y`: Center of view in board coordinates
// - `zoom_level`: Display scale (1.0 = default, >1 = zoomed in, <1 = zoomed out)
// - Pixels buffer size = board size (rendering buffer stays fixed)
// - Window can be any size (surface texture scales automatically)
// - Colors distinguish between: alive cells, dead cells inside board, and out-of-bounds area
pub fn draw_board_with_camera(pixels: &mut Pixels, board_current: &[bool], board_width: u32, board_height: u32, camera_x: f32, camera_y: f32, zoom_level: f32, window_width: u32, window_height: u32) {
    let frame = pixels.frame_mut();

    // Extract RGB components from config colors (RGBA format: 0xRRGGBBAA)
    let color_alive = [
        ((config::COLOR_CELL_ALIVE >> 24) & 0xFF) as u8,
        ((config::COLOR_CELL_ALIVE >> 16) & 0xFF) as u8,
        ((config::COLOR_CELL_ALIVE >> 8) & 0xFF) as u8,
        (config::COLOR_CELL_ALIVE & 0xFF) as u8,
    ];
    let color_dead = [
        ((config::COLOR_CELL_DEAD >> 24) & 0xFF) as u8,
        ((config::COLOR_CELL_DEAD >> 16) & 0xFF) as u8,
        ((config::COLOR_CELL_DEAD >> 8) & 0xFF) as u8,
        (config::COLOR_CELL_DEAD & 0xFF) as u8,
    ];
    let color_out_of_bounds = [
        ((config::COLOR_OUT_OF_BOUNDS >> 24) & 0xFF) as u8,
        ((config::COLOR_OUT_OF_BOUNDS >> 16) & 0xFF) as u8,
        ((config::COLOR_OUT_OF_BOUNDS >> 8) & 0xFF) as u8,
        (config::COLOR_OUT_OF_BOUNDS & 0xFF) as u8,
    ];

    // Calculate how many board cells fit in the window at current zoom
    let cells_visible_width = window_width as f32 / (config::CELL_SIZE as f32 * zoom_level);
    let cells_visible_height = window_height as f32 / (config::CELL_SIZE as f32 * zoom_level);

    // Calculate the top-left corner of the visible area in board coordinates
    let view_left = camera_x - cells_visible_width / 2.0;
    let view_top = camera_y - cells_visible_height / 2.0;

    // For each pixel in the rendering buffer, determine which board cell to show
    for buffer_y in 0..board_height {
        for buffer_x in 0..board_width {
            // Map buffer pixel to board cell coordinate
            let board_cell_x = view_left + (buffer_x as f32 / board_width as f32) * cells_visible_width;
            let board_cell_y = view_top + (buffer_y as f32 / board_height as f32) * cells_visible_height;

            // Check if this cell is within the board bounds
            let (is_alive, is_out_of_bounds) = if board_cell_x >= 0.0 && board_cell_y >= 0.0 && (board_cell_x as u32) < board_width && (board_cell_y as u32) < board_height {
                let cell_idx = ((board_cell_y as u32) * board_width + (board_cell_x as u32)) as usize;
                (board_current.get(cell_idx).copied().unwrap_or(false), false)
            } else {
                (false, true) // Outside board bounds
            };

            // Choose color based on cell state
            let color = if is_alive {
                &color_alive
            } else if is_out_of_bounds {
                &color_out_of_bounds
            } else {
                &color_dead
            };

            // Draw the pixel
            let pixel_idx = ((buffer_y * board_width + buffer_x) * 4) as usize;
            frame[pixel_idx] = color[0]; // R
            frame[pixel_idx + 1] = color[1]; // G
            frame[pixel_idx + 2] = color[2]; // B
            frame[pixel_idx + 3] = color[3]; // A
        }
    }

    pixels.render().unwrap();
}

// Legacy function for compatibility
pub fn draw_board(pixels: &mut Pixels, board_current: &[bool], buffer_width: u32, buffer_height: u32) {
    // Draw full board centered
    draw_board_with_camera(
        pixels,
        board_current,
        buffer_width,
        buffer_height,
        buffer_width as f32 / 2.0,
        buffer_height as f32 / 2.0,
        1.0,
        buffer_width * config::CELL_SIZE,
        buffer_height * config::CELL_SIZE,
    );
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
    log::error!("Error displayed: {}", error_message);
}
