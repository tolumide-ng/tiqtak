#[derive(Debug)]
pub(crate) enum Color {
    White, Black
}

pub(crate) struct Board {
    white: u32,
    black: u32,
    kings: u32,
}


impl Board {
    pub(crate) fn regular(&self, color: Color) {
        
    }
    pub(crate) fn kings(&self, color: Color) {}
}