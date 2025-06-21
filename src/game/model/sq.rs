pub(crate) struct Sq(u8);

impl TryFrom<(u8, i8)> for Sq {
    type Error = &'static str;

    fn try_from((base, offset): (u8, i8)) -> Result<Self, Self::Error> {
        let sum = base as i8 + offset as i8;

        if (0..=31).contains(&sum) {
            Ok(Self(sum as u8))
        } else {
            Err("Result out of range for 32bits bitboard")
        }
    }
}

impl AsRef<u8> for Sq {
    fn as_ref(&self) -> &u8 {
        &self.0
    }
}

impl AsMut<u8> for Sq {
    fn as_mut(&mut self) -> &mut u8 {
        &mut self.0
    }
}

impl From<Sq> for u8 {
    fn from(value: Sq) -> Self {
        value.0
    }
}
