#[derive(Clone)]
#[allow(dead_code)]
pub struct OmmaPad {
    pad_top: usize,
    pad_bottom: usize,
    pad_left: usize,
    pad_right: usize,
}

impl Default for OmmaPad {
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

impl OmmaPad {
    pub fn new(pad_top: usize, pad_bottom: usize, pad_left: usize, pad_right: usize) -> Self {
        OmmaPad {
            pad_top,
            pad_bottom,
            pad_left,
            pad_right,
        }
    }
}
