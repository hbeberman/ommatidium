use crate::cell::*;

#[derive(Default, Clone)]
#[allow(dead_code)]
pub struct OmmaBorder {
    hidden: bool,
    border_top: OmmaCell,
    border_bottom: OmmaCell,
    border_left: OmmaCell,
    border_right: OmmaCell,
    border_corner_tl: OmmaCell,
    border_corner_tr: OmmaCell,
    border_corner_bl: OmmaCell,
    border_corner_br: OmmaCell,
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
            border_corner_tl: corner.clone(),
            border_corner_tr: corner.clone(),
            border_corner_bl: corner.clone(),
            border_corner_br: corner.clone(),
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
            border_corner_tl: mono.clone(),
            border_corner_tr: mono.clone(),
            border_corner_bl: mono.clone(),
            border_corner_br: mono.clone(),
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
    pub fn border_corner_tl(&self) -> &OmmaCell {
        &self.border_corner_tl
    }
    pub fn border_corner_tr(&self) -> &OmmaCell {
        &self.border_corner_tr
    }
    pub fn border_corner_bl(&self) -> &OmmaCell {
        &self.border_corner_bl
    }
    pub fn border_corner_br(&self) -> &OmmaCell {
        &self.border_corner_br
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

pub const BOX_HOLLOW_BORDER: OmmaBorder = OmmaBorder {
    hidden: false,
    border_top: BOX_HORIZ_HOLLOW_CELL,
    border_bottom: BOX_HORIZ_HOLLOW_CELL,
    border_left: BOX_VERT_HOLLOW_CELL,
    border_right: BOX_VERT_HOLLOW_CELL,
    border_corner_tl: BOX_CORNER_TL_HOLLOW_CELL,
    border_corner_tr: BOX_CORNER_TR_HOLLOW_CELL,
    border_corner_bl: BOX_CORNER_BL_HOLLOW_CELL,
    border_corner_br: BOX_CORNER_BR_HOLLOW_CELL,
};
