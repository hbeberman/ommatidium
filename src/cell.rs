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
        Self::new(' ', 0, 0, 0)
    }
}

impl OmmaCell {
    pub fn new(ch: char, fg: u8, bg: u8, attrs: u16) -> Self {
        OmmaCell { ch, fg, bg, attrs }
    }

    pub fn transparent() -> Self {
        Self::new(EMPTY, 0, 0, 0)
    }
}

#[test]
fn new() {
    let ommacell = OmmaCell::new(' ', 0, 0, 0);
    assert!(ommacell.ch == ' ');
}

pub const BLANK_CELL: OmmaCell = OmmaCell {
    ch: ' ',
    fg: 0,
    bg: 0,
    attrs: 0,
};

pub const EMPTY_CELL: OmmaCell = OmmaCell {
    ch: EMPTY,
    fg: 0,
    bg: 0,
    attrs: 0,
};

pub const PLAYER_CELL: OmmaCell = OmmaCell {
    ch: '@',
    fg: 0,
    bg: 0,
    attrs: 0,
};

pub const FLOOR_CELL: OmmaCell = OmmaCell {
    ch: '.',
    fg: 0,
    bg: 0,
    attrs: 0,
};

pub const WALL_CELL: OmmaCell = OmmaCell {
    ch: '#',
    fg: 0,
    bg: 0,
    attrs: 0,
};

pub const HORIZ_CELL: OmmaCell = OmmaCell {
    ch: '-',
    fg: 0,
    bg: 0,
    attrs: 0,
};

pub const VERT_CELL: OmmaCell = OmmaCell {
    ch: '|',
    fg: 0,
    bg: 0,
    attrs: 0,
};

pub const CORNER_CELL: OmmaCell = OmmaCell {
    ch: 'O',
    fg: 0,
    bg: 0,
    attrs: 0,
};

pub const SPECIAL_CELL: OmmaCell = OmmaCell {
    ch: '*',
    fg: 0,
    bg: 0,
    attrs: 0,
};
