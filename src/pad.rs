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

    /// Getters
    pub fn pad_top(&self) -> usize {
        self.pad_top
    }
    pub fn pad_bottom(&self) -> usize {
        self.pad_bottom
    }
    pub fn pad_left(&self) -> usize {
        self.pad_left
    }
    pub fn pad_right(&self) -> usize {
        self.pad_right
    }
}
