#[derive(Clone)]
#[allow(dead_code)]
pub struct OmmaColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Default for OmmaColor {
    fn default() -> Self {
        WHITE
    }
}

impl OmmaColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

pub const WHITE: OmmaColor = OmmaColor {
    r: 255,
    g: 255,
    b: 255,
};

pub const LIGHT_GREY: OmmaColor = OmmaColor {
    r: 200,
    g: 200,
    b: 200,
};
pub const DARK_GREY: OmmaColor = OmmaColor {
    r: 100,
    g: 100,
    b: 100,
};

pub const BLACK: OmmaColor = OmmaColor { r: 0, g: 0, b: 0 };

pub const RED: OmmaColor = OmmaColor { r: 255, g: 0, b: 0 };

pub const GREEN: OmmaColor = OmmaColor { r: 0, g: 255, b: 0 };

pub const BLUE: OmmaColor = OmmaColor { r: 0, g: 0, b: 255 };
