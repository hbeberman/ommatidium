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
        Plane { id: 0, windows }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plane() {
        let plane = Plane::new();
        assert!(plane.id() == 0);
        assert!(plane.windows.is_empty());
    }
}
