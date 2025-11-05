// src/app/render.rs

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
