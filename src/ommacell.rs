pub const EMPTY: char = '\0';

#[derive(Clone)]
#[allow(dead_code)]
pub struct OmmaCell {
    pub ch: char,
    pub fg: u8,
    pub bg: u8,
    pub attrs: u16,
}

impl Default for OmmaCell {
    fn default() -> Self {
        OmmaCell::new(' ', 0, 0, 0)
    }
}

impl OmmaCell {
    pub fn new(ch: char, fg: u8, bg: u8, attrs: u16) -> Self {
        OmmaCell { ch, fg, bg, attrs }
    }

    pub fn transparent() -> Self {
        OmmaCell::new(EMPTY, 0, 0, 0)
    }
}
