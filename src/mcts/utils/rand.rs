use getrandom::Error;

pub(crate) fn getrand(len: usize) -> Result<usize, Error> {
    assert!(len > 0, "len must be greater than 0");

    let mut buf = [0u8; 8]; // max of 8bytes for usize
    getrandom::fill(&mut buf)?;
    let idx = u64::from_le_bytes(buf);
    Ok((idx % len as u64) as usize)
}
