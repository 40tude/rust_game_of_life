// gol/life.rs

// Compute one step: current -> next (row-major, no wrapping).
// Cells outside the board are considered dead.
pub fn step_life(board_current: &[bool], board_next: &mut [bool], buffer_width: u32, buffer_height: u32) {
    debug_assert_eq!(board_current.len(), board_next.len());

    let get = |x: isize, y: isize| -> u8 {
        if x < 0 || y < 0 {
            return 0;
        }
        let (x, y) = (x as usize, y as usize);
        if x >= buffer_width as usize || y >= buffer_height as usize {
            return 0;
        }
        board_current[y * buffer_width as usize + x] as u8
    };

    for y in 0..buffer_height {
        for x in 0..buffer_width {
            let xi = x as isize;
            let yi = y as isize;

            let mut n = 0u8;
            n += get(xi - 1, yi - 1);
            n += get(xi, yi - 1);
            n += get(xi + 1, yi - 1);
            n += get(xi - 1, yi);
            n += get(xi + 1, yi);
            n += get(xi - 1, yi + 1);
            n += get(xi, yi + 1);
            n += get(xi + 1, yi + 1);

            let idx: usize = (y * buffer_width + x) as usize;
            let alive = board_current[idx];

            board_next[idx] = match (alive, n) {
                (true, 2) | (_, 3) => true, // survive with 2; birth/survive with 3
                _ => false,
            };
        }
    }
}
