use crate::plane::*;

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
        Session { id: 0, planes }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session() {
        let session = Session::new();
        assert!(session.id() == 0);
        assert!(session.planes.is_empty());
    }
}
