use crate::ommacell::OmmaCell;

#[derive(Default, Clone)]
#[allow(dead_code)]
pub struct OmmaBorder {
    hidden: bool,
    border_top: OmmaCell,
    border_bottom: OmmaCell,
    border_left: OmmaCell,
    border_right: OmmaCell,
    border_corner: OmmaCell,
}

impl OmmaBorder {
    /// new creates an OmmaBorder with distinct sides and corner
    pub fn new(horiz: &OmmaCell, vert: &OmmaCell, corner: &OmmaCell) -> Self {
        OmmaBorder {
            hidden: false,
            border_top: horiz.clone(),
            border_bottom: horiz.clone(),
            border_left: vert.clone(),
            border_right: vert.clone(),
            border_corner: corner.clone(),
        }
    }

    /// new_mono creates an OmmaBorder with a single ommacell for all sides and corner
    pub fn new_mono(mono: &OmmaCell) -> Self {
        OmmaBorder {
            hidden: false,
            border_top: mono.clone(),
            border_bottom: mono.clone(),
            border_left: mono.clone(),
            border_right: mono.clone(),
            border_corner: mono.clone(),
        }
    }

    // Getters
    pub fn hidden(&self) -> bool {
        self.hidden
    }
    pub fn border_bottom(&self) -> &OmmaCell {
        &self.border_bottom
    }
    pub fn border_top(&self) -> &OmmaCell {
        &self.border_top
    }
    pub fn border_left(&self) -> &OmmaCell {
        &self.border_left
    }
    pub fn border_right(&self) -> &OmmaCell {
        &self.border_right
    }
    pub fn border_corner(&self) -> &OmmaCell {
        &self.border_corner
    }

    // Setters
    pub fn toggle_hidden(&mut self) {
        self.hidden = !self.hidden
    }

    pub fn set_hidden(&mut self) {
        self.hidden = true
    }

    pub fn clear_hidden(&mut self) {
        self.hidden = false
    }
}
