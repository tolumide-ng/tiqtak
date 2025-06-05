pub(crate) mod board;
pub mod convert64bits_to_32bits;
pub(crate) mod model;
pub(crate) mod traits;
pub(crate) mod utils;

const BOARD_32_TO_48: [u8; 32] = [
    1, 3, 5, 7, //
    8, 10, 12, 14, //
    17, 19, 21, 23, //
    24, 26, 28, 30, //
    33, 35, 37, 39, //
    40, 42, 44, 46, //
    49, 51, 53, 55, //
    56, 58, 60, 62, //
];
