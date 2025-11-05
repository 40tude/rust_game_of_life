// gol/utils.rs

use crate::Result;
// use crate::prelude::*; // see lib.rs
use std::fs;
use std::num::ParseIntError;
use std::path::Path; // see lib.rs

// Place a pattern at the center of the board
pub fn place_pattern_centered(board_current: &mut [bool], board_width: u32, board_height: u32, pattern_cells: &[bool], pattern_width: u32, pattern_height: u32) -> Result<()> {
    // Make sure the board is initialized
    if board_current.is_empty() {
        return Err("Board must not be empty".into());
    }

    // Compute offsets to center the pattern
    let offset_x = (board_width as i32 - pattern_width as i32) / 2;
    let offset_y = (board_height as i32 - pattern_height as i32) / 2;

    // Copy the pattern while centering it
    for y in 0..pattern_height {
        for x in 0..pattern_width {
            let pattern_idx = (y * pattern_width + x) as usize;
            let buffer_x = offset_x + x as i32;
            let buffer_y = offset_y + y as i32;

            // Check versus buffer's limits
            if buffer_x >= 0 && buffer_x < board_width as i32 && buffer_y >= 0 && buffer_y < board_height as i32 {
                let buffer_idx = (buffer_y as u32 * board_width + buffer_x as u32) as usize;

                if pattern_idx < pattern_cells.len() && buffer_idx < board_current.len() {
                    board_current[buffer_idx] = pattern_cells[pattern_idx];
                }
            }
        }
    }

    log::info!(
        "place_pattern_centered(): Pattern ({}x{}) centered in buffer ({}x{}).",
        pattern_width,
        pattern_height,
        board_width,
        board_height
    );
    Ok(())
}

// Read an RLE file and provide (pattern_cells, pattern_width, pattern_height)
pub fn read_rle(filename: &Path) -> Result<(Vec<bool>, u32, u32)> {
    let content = fs::read_to_string(filename).map_err(|e| -> crate::Error { format!("Failed to read RLE file '{}': {}", filename.display(), e).into() })?;

    let mut pattern_width: u32 = 0;
    let mut pattern_height: u32 = 0;
    let mut data_lines: Vec<String> = Vec::new();

    // 1) Separate metadata from data; tolerate comments and empty lines
    for raw in content.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with('x') || line.starts_with('X') {
            // Header line: e.g. "x = 19, y = 11, rule = B3/S23"
            for part in line.split(',') {
                let p = part.trim();
                // if let Some(v) = p.strip_prefix("x").and_then(|s| s.strip_prefix(|c: char| c.is_ascii_whitespace() || c == '=')) {
                //     if pattern_width == 0 {
                //         pattern_width = parse_u32_trim(v)?;
                //     }
                if let Some(v) = p.strip_prefix("x").and_then(|s| s.strip_prefix(|c: char| c.is_ascii_whitespace() || c == '='))
                    && pattern_width == 0
                {
                    pattern_width = parse_u32_trim(v)?;
                // } else if let Some(v) = p.strip_prefix("y").and_then(|s| s.strip_prefix(|c: char| c.is_ascii_whitespace() || c == '=')) {
                //     if pattern_height == 0 {
                //         pattern_height = parse_u32_trim(v)?;
                //     }
                // }
                } else if let Some(v) = p.strip_prefix("y").and_then(|s| s.strip_prefix(|c: char| c.is_ascii_whitespace() || c == '='))
                    && pattern_height == 0
                {
                    pattern_height = parse_u32_trim(v)?;
                }
                // "rule" is ignored on purpose
            }
        } else {
            data_lines.push(line.to_string());
        }
    }

    if data_lines.is_empty() {
        return Err("No RLE data found in file.".into());
    }

    // Join the payload (RLE may break across lines; '$' carries EOL semantics)
    let payload = data_lines.join("");

    // 2) If (x,y) not provided, infer them with a cheap first pass
    let (w, h) = if pattern_width == 0 || pattern_height == 0 {
        infer_dims_from_rle(&payload)?
    } else {
        (pattern_width as usize, pattern_height as usize)
    };

    // 3) Second pass: actually decode into a dense Vec<bool> (row-major, y-major)
    let cells = decode_rle(&payload, w, h)?;

    Ok((cells, w as u32, h as u32))
}

// --- helpers ----------------------------------------------------------------

fn parse_u32_trim(s: &str) -> std::result::Result<u32, ParseIntError> {
    s.trim().trim_start_matches('=').trim().parse::<u32>()
}

// Token iterator over the compact RLE stream.
// Produces (count, symbol) where symbol in {'o','b','$','!'}; count defaults to 1.
fn next_token(chars: &mut std::str::Chars<'_>) -> Option<(usize, char)> {
    let mut count: usize = 0;
    while let Some(c) = chars.clone().next() {
        if c.is_ascii_digit() {
            chars.next();
            count = count * 10 + (c as usize - '0' as usize);
        } else {
            break;
        }
    }
    let sym = chars.next()?;
    if sym.is_ascii_whitespace() {
        return next_token(chars);
    }
    Some((if count == 0 { 1 } else { count }, sym))
}

// First pass to compute (width, height) for headerless RLE.
// Width is the max decoded row length before a '$', height is decoded row count.
// Stops at '!' if present, tolerates missing '!'.
fn infer_dims_from_rle(s: &str) -> Result<(usize, usize)> {
    let mut chars = s.chars();
    let mut cur_len = 0usize;
    let mut max_len = 0usize;
    let mut height = 0usize;

    while let Some((n, sym)) = next_token(&mut chars) {
        match sym {
            'o' | 'b' | 'O' | 'B' => {
                cur_len = cur_len.saturating_add(n);
                if cur_len > max_len {
                    max_len = cur_len;
                }
            }
            '$' => {
                height = height.saturating_add(n);
                cur_len = 0;
            }
            '!' => break,
            _ => {
                // Ignore other glyphs/spaces silently to be lenient
            }
        }
    }
    // If there was data on the last line without a trailing '$', count that line
    if cur_len > 0 {
        height = height.saturating_add(1);
        if cur_len > max_len {
            max_len = cur_len;
        }
    }

    if max_len == 0 || height == 0 {
        return Err("Unable to infer dimensions from RLE data.".into());
    }
    Ok((max_len, height))
}

// Second pass: decode into a fixed grid (w,h). Excess decoded cells beyond bounds are ignored.
// Missing cells at end of a short line are left false (dead).
fn decode_rle(s: &str, w: usize, h: usize) -> Result<Vec<bool>> {
    let mut grid = vec![false; w * h];
    let mut chars = s.chars();

    let mut x = 0usize;
    let mut y = 0usize;

    while let Some((n, sym)) = next_token(&mut chars) {
        match sym {
            'o' | 'O' => {
                for _ in 0..n {
                    if y >= h {
                        break;
                    }
                    if x < w {
                        grid[y * w + x] = true;
                    }
                    x = x.saturating_add(1);
                }
            }
            'b' | 'B' => {
                // dead cells: advance cursor, values already false
                x = x.saturating_add(n);
            }
            '$' => {
                // n newlines
                for _ in 0..n {
                    y = y.saturating_add(1);
                    x = 0;
                    if y >= h {
                        break;
                    }
                }
            }
            '!' => break,
            _ => {
                // Be permissive: ignore anything else (spaces, stray commas, etc.)
            }
        }
        if y >= h {
            break;
        }
    }

    Ok(grid)
}

// cargo test -p step_08
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glider() {
        // A 3x3 glider
        let glider = "x = 3, y = 3\nbob$2bo$3o!";
        // Write in temp file for testing
        std::fs::write("test_glider.rle", glider).unwrap();

        let (cells, width, height) = read_rle("test_glider.rle").unwrap();

        assert_eq!(width, 3);
        assert_eq!(height, 3);

        // The glider should look like:
        // . O .
        // . . O
        // O O O
        let expected = vec![
            false, true, false, // line 0
            false, false, true, // line 1
            true, true, true, // line 2
        ];

        assert_eq!(cells, expected);

        // cleanup
        let _ = std::fs::remove_file("test_glider.rle");
    }
}
