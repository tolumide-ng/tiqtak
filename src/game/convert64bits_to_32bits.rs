use crate::Action;

pub fn c64bits_to_32bits_board(input: u64) -> u32 {
    let bits_per_row = 8u64;
    let new_bits_per_row = 4;

    let mut result = 0u32;

    let mut board = input;

    for index in 0..8 {
        let current = ((board & 0b11111111) as u8).reverse_bits(); // this gives us the 8 least significant bits (LSB) of the board
        let starts_with_black_column = index % 2 == 0;

        let mut new_4bits_row = 0u32;

        if starts_with_black_column {
            new_4bits_row = (((current & 0b10000000) >> 4)
                | ((current & 0b00100000) >> 3)
                | ((current & 0b00001000) >> 2)
                | ((current & 0b00000010) >> 1)) as u32;
        } else {
            new_4bits_row = (((current & 0b01000000) >> 3)
                | ((current & 0b00010000) >> 2)
                | ((current & 0b00000100) >> 1)
                | (current & 0b00000001)) as u32;
        }

        result |= new_4bits_row << (new_bits_per_row * index);

        board = board >> bits_per_row;
    }

    result
}

fn convert_64_to_32(idx64: u8) -> Option<u8> {
    if idx64 >= 64 {
        return None; // out of bounds
    }

    let row = idx64 / 8;
    let col = idx64 % 8;

    // Check if the square is dark (playable)
    if (row + col) % 2 == 0 {
        return None; // light square, no corresponding 32-bit index
    }

    // Count how many dark squares before idx64
    // Each row has 4 playable squares

    // Number of dark squares in previous rows
    let dark_squares_before = (row as u8) * 4;

    // So position in current row:
    let position_in_row = match row % 2 {
        0 => (col - 1) / 2, // even row, playable cols start at 1
        1 => col / 2,       // odd row, playable cols start at 0
        _ => unreachable!(),
    };

    Some(dark_squares_before + position_in_row)
}

pub fn getax(a: Action) -> Action {
    let src = convert_64_to_32(a.src).unwrap();
    let tgt = convert_64_to_32(a.tgt).unwrap();

    let xx = Action::from((src, tgt, a.capture, a.promoted, a.scale));

    Action::new(src, tgt, a.capture, a.promoted, !a.scale)
}
