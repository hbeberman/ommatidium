use crate::color::OmmaColor;
pub const EMPTY: char = '\0';

#[derive(Clone)]
#[allow(dead_code)]
pub struct OmmaCell {
    pub ch: char,
    pub fg: Option<OmmaColor>,
    pub bg: Option<OmmaColor>,
    pub attrs: u16,
}

impl Default for OmmaCell {
    fn default() -> Self {
        Self::new(' ', None, None, 0)
    }
}

impl OmmaCell {
    pub fn new(ch: char, fg: Option<OmmaColor>, bg: Option<OmmaColor>, attrs: u16) -> Self {
        OmmaCell { ch, fg, bg, attrs }
    }

    pub fn transparent() -> Self {
        Self::new(EMPTY, None, None, 0)
    }

    /// fgcode returns the ANSI escape code to set the foreground color or reset on None
    pub(crate) fn fgcode(&self) -> String {
        if let Some(fg) = &self.fg {
            format!("\x1b[38;2;{};{};{}m", fg.r, fg.g, fg.b)
        } else {
            "\x1b[39m".to_string()
        }
    }

    /// bgcode returns the ANSI escape code to set the background color or reset on None
    pub(crate) fn bgcode(&self) -> String {
        if let Some(bg) = &self.bg {
            format!("\x1b[48;2;{};{};{}m", bg.r, bg.g, bg.b)
        } else {
            "\x1b[49m".to_string()
        }
    }
}

#[test]
fn new() {
    let ommacell = OmmaCell::new(' ', None, None, 0);
    assert!(ommacell.ch == ' ');
}

pub const DEFAULT_CELL: OmmaCell = OmmaCell {
    ch: ' ',
    fg: None,
    bg: None,
    attrs: 0,
};

pub const BLANK_CELL: OmmaCell = OmmaCell { ..DEFAULT_CELL };

pub const EMPTY_CELL: OmmaCell = OmmaCell {
    ch: EMPTY,
    ..DEFAULT_CELL
};

pub const PLAYER_CELL: OmmaCell = OmmaCell {
    ch: '@',
    ..DEFAULT_CELL
};

pub const FLOOR_CELL: OmmaCell = OmmaCell {
    ch: '.',
    ..DEFAULT_CELL
};

pub const WALL_CELL: OmmaCell = OmmaCell {
    ch: '#',
    ..DEFAULT_CELL
};

pub const HORIZ_CELL: OmmaCell = OmmaCell {
    ch: '-',
    ..DEFAULT_CELL
};

pub const VERT_CELL: OmmaCell = OmmaCell {
    ch: '|',
    ..DEFAULT_CELL
};

pub const CORNER_CELL: OmmaCell = OmmaCell {
    ch: 'O',
    ..DEFAULT_CELL
};

pub const SPECIAL_CELL: OmmaCell = OmmaCell {
    ch: '*',
    ..DEFAULT_CELL
};

pub const BOX_VERT_HOLLOW_CELL: OmmaCell = OmmaCell {
    ch: '║',
    ..DEFAULT_CELL
};

pub const BOX_HORIZ_HOLLOW_CELL: OmmaCell = OmmaCell {
    ch: '═',
    ..DEFAULT_CELL
};

pub const BOX_CORNER_TL_HOLLOW_CELL: OmmaCell = OmmaCell {
    ch: '╔',
    ..DEFAULT_CELL
};

pub const BOX_CORNER_TR_HOLLOW_CELL: OmmaCell = OmmaCell {
    ch: '╗',
    ..DEFAULT_CELL
};

pub const BOX_CORNER_BL_HOLLOW_CELL: OmmaCell = OmmaCell {
    ch: '╚',
    ..DEFAULT_CELL
};

pub const BOX_CORNER_BR_HOLLOW_CELL: OmmaCell = OmmaCell {
    ch: '╝',
    ..DEFAULT_CELL
};

pub const BOX_CORNER_ALL_HOLLOW_CELL: OmmaCell = OmmaCell {
    ch: '╬',
    ..DEFAULT_CELL
};
