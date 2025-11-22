use crate::plane::*;
use crate::window::*;

#[allow(dead_code)]
#[derive(Default)]
pub struct Session {
    id: u32,
    planes: Vec<Plane>,
}

#[allow(dead_code)]
impl Session {
    pub fn new() -> Self {
        let planes: Vec<Plane> = Vec::new();
        let id = crate::next_id();
        Session { id, planes }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn planes_is_empty(&self) -> bool {
        self.planes.is_empty()
    }

    pub fn new_plane(&mut self) -> u32 {
        let plane = Plane::new();
        let id = plane.id();
        self.planes.push(Plane::new());
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
