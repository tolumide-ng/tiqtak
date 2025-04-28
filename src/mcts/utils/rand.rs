use getrandom::Error;

pub(crate) fn getrand(len: usize) -> Result<usize, Error> {
    assert!(len > 0, "len must be greater than 0");

    let mut buf = [0u8; 8]; // max of 8bytes for usize
    getrandom::fill(&mut buf)?;
    let idx = u64::from_le_bytes(buf);
    Ok((idx % len as u64) as usize)
}

pub(crate) fn genrand(min: usize, max: usize) -> usize {
    if max < min {
        println!("the min is {min} and the max is {max}");
        panic!("min must be less than max");
    }

    if min == max {
        return min;
    }
    let range = max - min;
    let mut buf = [0u8; 8];
    getrandom::fill(&mut buf).expect("Failed to generate random bytes");

    let random_value = u64::from_le_bytes(buf);
    let scaled_value = (random_value as usize) % range;

    min + scaled_value
}
