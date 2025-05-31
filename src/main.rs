use tiqtak::convert64bits_to_32bits::c64bits_to_32bits_board;
use tiqtak::{Board, Qmvs};

fn main() {
    // let board = Board::new();
    // println!("{}", board);

    let bits = c64bits_to_32bits_board(0x8040200000000000u64);
    let south = c64bits_to_32bits_board(0x1028000000u64);

    // println!("the bits are >>> {bits:#0x} {south:#0x}");

    // let b = Board::with(bits, south, 0, tiqtak::Player::North, Qmvs::default());

    // println!("{}", b);
}
