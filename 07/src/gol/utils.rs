// src/gol/utils.rs

use crate::Result;
use std::fs;

// Read an RLE file and provide (pattern_cells, pattern_width, pattern_height)
pub fn read_rle(filename: &str) -> Result<(Vec<bool>, u32, u32)> {
    let content = fs::read_to_string(filename)?;

    let mut pattern_width = 0;
    let mut pattern_height = 0;
    let mut pattern_lines = Vec::new();

    // Parse headers and data
    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue; // Ignore empty line and comments
        }

        if line.starts_with('x') {
            // Header line : x = 5, y = 3
            for part in line.split(',') {
                let part = part.trim();
                if part.starts_with("x =") {
                    pattern_width = part[3..].trim().parse()?;
                } else if part.starts_with("y =") {
                    pattern_height = part[3..].trim().parse()?;
                }
                // "rule = ..." is ignored for now
            }
        } else {
            // A line of pattern
            pattern_lines.push(line);
        }
    }

    if pattern_width == 0 || pattern_height == 0 {
        return Err("Pattern size (x or y) not found in RLE file".into());
    }

    // Join pattern's lines (the RLE has $ as EOL)
    let pattern_data = pattern_lines.join("");

    let cells = parse_rle_data(&pattern_data, pattern_width as usize, pattern_height as usize)?;

    Ok((cells, pattern_width, pattern_height))
}

// Parse the RLE pattern as a single string (ex: "3b2o$2o2b2o!")
fn parse_rle_data(pattern_string: &str, pattern_width: usize, pattern_height: usize) -> Result<Vec<bool>> {
    let mut cells = vec![false; pattern_width * pattern_height];
    let mut x = 0;
    let mut y = 0;
    let mut count_str = String::new();

    for c in pattern_string.chars() {
        match c {
            '0'..='9' => {
                count_str.push(c);
            }
            'b' | 'o' => {
                let count = if count_str.is_empty() { 1 } else { count_str.parse::<usize>()? };
                count_str.clear();

                let cell_state = c == 'o'; // 'o' = live, 'b' = dead

                for _ in 0..count {
                    if x < pattern_width && y < pattern_height {
                        cells[y * pattern_width + x] = cell_state;
                    }
                    x += 1;
                }
            }
            '$' => {
                // New line
                let count = if count_str.is_empty() { 1 } else { count_str.parse::<usize>()? };
                count_str.clear();

                y += count;
                x = 0;
            }
            '!' => {
                // End of the pattern string
                break;
            }
            _ => {
                // Ignore other char (paces, etc.)
            }
        }
    }

    Ok(cells)
}

// Show how to use some test patterns
// cargo test -p step_07
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
