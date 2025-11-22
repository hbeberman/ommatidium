use crate::window::*;

#[allow(dead_code)]
#[derive(Default)]
pub struct Plane {
    id: u32,
    windows: Vec<Window>,
}

#[allow(dead_code)]
impl Plane {
    pub fn new() -> Self {
        let windows: Vec<Window> = Vec::new();
        let id = crate::next_id();
        Plane { id, windows }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn windows_is_empty(&self) -> bool {
        self.windows.is_empty()
    }
}
