use getrandom::getrandom;

pub(crate) fn genrand(min: usize, max: usize) -> usize {
    assert!(min < max, "min must be less than max");
    let range = max - min;

    let mut buf = [0u8; std::mem::size_of::<usize>()];
    getrandom(&mut buf).expect("random failed");

    let value = match buf.len() {
        4 => usize::from_le_bytes(buf[..4].try_into().unwrap()),
        8 => usize::from_le_bytes(buf.try_into().unwrap()),
        _ => unreachable!("Unsupported usize size"),
    };

    min + (value % range)
}
