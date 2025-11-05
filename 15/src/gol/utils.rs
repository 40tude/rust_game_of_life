// gol/utils.rs

use crate::Result;
// use crate::prelude::*;
use std::fs;
use std::num::ParseIntError;
use std::path::Path; // see lib.rs

// Place a pattern at the center of the board
// May receive a pattern bigger than the board when the window is resized
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
                if let Some(v) = p.strip_prefix("x").and_then(|s| s.strip_prefix(|c: char| c.is_ascii_whitespace() || c == '='))
                    && pattern_width == 0
                {
                    pattern_width = parse_u32_trim(v)?;
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

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    // use std::fs;
    // use std::io::Write;
    use std::path::{/*Path,*/ PathBuf};

    // ------------------------------------------------------------------------
    // Helpers ----------------------------------------------------------------
    // ------------------------------------------------------------------------

    // fn tmp_file(name: &str, content: &str) -> PathBuf {
    //     let path = PathBuf::from(name);
    //     let mut f = fs::File::create(&path).expect("create temp file");
    //     f.write_all(content.as_bytes()).expect("write temp file");
    //     path
    // }

    fn bools(rows: &[&[u8]]) -> Vec<bool> {
        // Build a vec<bool> from a small ASCII matrix where 1 = alive, 0 = dead.
        // Example:
        //   bools(&[&[0,1,0], &[1,1,1]])
        // becomes: [false,true,false, true,true,true]
        let mut v = Vec::new();
        for row in rows {
            for &cell in *row {
                v.push(cell != 0);
            }
        }
        v
    }

    // ----------------------------------------------------------------------------
    // place_pattern_centered -----------------------------------------------------
    // ----------------------------------------------------------------------------

    #[test]
    fn place_pattern_centered_empty_board() {
        let mut board: Vec<bool> = vec![];
        let pattern = vec![true];
        let res = place_pattern_centered(&mut board, 0, 0, &pattern, 1, 1);
        assert!(res.is_err(), "Expected Err on empty board");
    }

    #[test]
    fn place_pattern_centered_small_odd_pattern_on_odd_board() {
        // Board: 7x5 (all dead)
        let board_w = 7_u32;
        let board_h = 5_u32;
        let mut board = vec![false; (board_w * board_h) as usize];

        // Pattern: 3x3 glider
        // . O .
        // . . O
        // O O O
        let pat_w = 3_u32;
        let pat_h = 3_u32;
        let glider = bools(&[&[0, 1, 0], &[0, 0, 1], &[1, 1, 1]]);

        // Centered offsets should be:
        // x0 = (7-3)/2 = 2, y0 = (5-3)/2 = 1
        place_pattern_centered(&mut board, board_w, board_h, &glider, pat_w, pat_h).expect("center pattern");

        // Build expected board
        // let mut expected = vec![false; (board_w * board_h) as usize];
        // let x0 = 2;
        // let y0 = 1;
        // let set = |buf: &mut [bool], x: usize, y: usize| {
        //     buf[y * board_w as usize + x] = true;
        // };
        // set(&mut expected, x0 + 1, y0 + 0); // (3,1)
        // set(&mut expected, x0 + 2, y0 + 1); // (4,2)
        // set(&mut expected, x0 + 0, y0 + 2); // (2,3)
        // set(&mut expected, x0 + 1, y0 + 2); // (3,3)
        // set(&mut expected, x0 + 2, y0 + 2); // (4,3)

        // The board should look like:
        // . . . . . . .
        // . . . O . . .
        // . . . . O . .
        // . . O O O . .
        // . . . . . . .
        let expected = vec![
            false, false, false, false, false, false, false, //
            false, false, false, true, false, false, false, //
            false, false, false, false, true, false, false, //
            false, false, true, true, true, false, false, //
            false, false, false, false, false, false, false, //
        ];

        assert_eq!(board, expected);
    }

    #[test]
    fn place_pattern_centered_pattern_bigger_than_board() {
        let board_w = 4;
        let board_h = 4;
        let mut board = vec![false; (board_w * board_h) as usize];

        // 5x1 pattern doesn't fit horizontally
        let pattern = vec![true; 5];
        let res = place_pattern_centered(&mut board, board_w, board_h, &pattern, 5, 1);
        assert!(res.is_ok(), "Expected no Err even if the pattern is wider than the board");
    }

    // ----------------------------------------------------------------------------
    // read_rle -------------------------------------------------------------------
    // ----------------------------------------------------------------------------

    #[test]
    fn read_rle_my_very_first_test_glider() {
        // A 3x3 glider
        let glider = "x = 3, y = 3\nbob$2bo$3o!";
        // Write in temp file for testing
        std::fs::write("test_glider_001.rle", glider).unwrap();

        let (cells, width, height) = read_rle(&PathBuf::from("test_glider_001.rle")).unwrap();

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
        let _ = std::fs::remove_file("test_glider_001.rle");
    }

    //     #[test]
    //     fn read_rle_parses_header_and_data() {
    //         // 3x3 glider in RLE with header
    //         // . O .
    //         // . . O
    //         // O O O
    //         let rle = r#"
    // #N Glider
    // #C Classic 3x3 glider
    // x = 3, y = 3, rule = B3/S23
    // .b.
    // ..o
    // ooo!
    // "#;
    //         let file = tmp_file("test_glider_header.rle", rle);
    //         let (cells, w, h) = read_rle(Path::new(&file)).expect("read rle");
    //         assert_eq!((w, h), (3, 3));

    //         let expected = bools(&[&[0, 1, 0], &[0, 0, 1], &[1, 1, 1]]);
    //         let _ = fs::remove_file(file);

    //         assert_eq!(cells, expected);
    //     }

    // #[test]
    // fn read_rle_ignores_comments_and_whitespace() {
    //     // Same glider but with comments, spaces, and $ newlines
    //     let rle = r#"
    // # Any comments should be ignored
    // x = 3, y = 3
    //   .b. $
    //   ..o $
    //   ooo !
    // "#;
    //     let file = tmp_file("test_glider_ws.rle", rle);
    //     let (cells, w, h) = read_rle(Path::new(&file)).expect("read rle");
    //     assert_eq!((w, h), (3, 3));

    //     let expected = bools(&[&[0, 1, 0], &[0, 0, 1], &[1, 1, 1]]);
    //     assert_eq!(cells, expected);

    //     let _ = fs::remove_file(file);
    // }

    //     #[test]
    //     fn read_rle_handles_run_lengths() {
    //         // 5x3 block with runs:
    //         // ooooo
    //         // bbbbo
    //         // o....
    //         // Encode: 5o$4bo$1o4b!
    //         let rle = r#"
    // x = 5, y = 3
    // 5o$4bo$o4b!
    // "#;
    //         let file = tmp_file("test_runs.rle", rle);
    //         let (cells, w, h) = read_rle(Path::new(&file)).expect("read rle");
    //         assert_eq!((w, h), (5, 3));

    //         let expected = bools(&[&[1, 1, 1, 1, 1], &[0, 0, 0, 0, 1], &[1, 0, 0, 0, 0]]);
    //         assert_eq!(cells, expected);

    //         let _ = fs::remove_file(file);
    //     }

    //     #[test]
    //     fn read_rle_errors_on_invalid_syntax() {
    //         // Missing '!' terminator and illegal char 'x'
    //         let rle = r#"
    // x = 3, y = 1
    // oox
    // "#;
    //         let file = tmp_file("test_invalid.rle", rle);
    //         let res = read_rle(Path::new(&file));
    //         assert!(res.is_err(), "Expected Err on invalid RLE syntax");
    //         let _ = fs::remove_file(file);
    //     }

    //     #[test]
    //     fn read_rle_without_header_is_supported_or_todo() {
    //         // Some collections omit the x/y header. If your implementation already
    //         // supports it, this test should pass; otherwise it will fail and serve
    //         // as a TODO to add that support.
    //         //
    //         // Glider without header:
    //         // .b.$..o$ooo!
    //         let rle = ".b.$..o$ooo!";
    //         let file = tmp_file("test_no_header.rle", rle);
    //         let res = read_rle(Path::new(&file));
    //         match res {
    //             Ok((cells, w, h)) => {
    //                 assert_eq!((w, h), (3, 3));
    //                 let expected = bools(&[&[0, 1, 0], &[0, 0, 1], &[1, 1, 1]]);
    //                 assert_eq!(cells, expected);
    //             }
    //             Err(e) => {
    //                 // Acceptable for now if you haven’t implemented this yet,
    //                 // but you can flip this to assert!(res.is_ok()) once done.
    //                 eprintln!("read_rle without header not yet supported: {e}");
    //             }
    //         }
    //         let _ = fs::remove_file(file);
    //     }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_glider() {
//         // A 3x3 glider
// let glider = "x = 3, y = 3\nbob$2bo$3o!";
//         // Write in temp file for testing
//         std::fs::write("test_glider.rle", glider).unwrap();

//         let (cells, width, height) = read_rle("test_glider.rle").unwrap();

//         assert_eq!(width, 3);
//         assert_eq!(height, 3);

//         // The glider should look like:
//         // . O .
//         // . . O
//         // O O O
//         let expected = vec![
//             false, true, false, // line 0
//             false, false, true, // line 1
//             true, true, true, // line 2
//         ];

//         assert_eq!(cells, expected);

//         // cleanup
//         let _ = std::fs::remove_file("test_glider.rle");
//     }
// }
