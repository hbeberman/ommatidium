use ommatidium::plane::Plane;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plane() {
        let plane = Plane::new();
        let plane2 = Plane::new();
        let id = plane.id();
        eprintln!("ID == {}", id);
        let id = plane2.id();
        eprintln!("ID == {}", id);
        assert!(plane.id() == 1);
        assert!(plane2.id() == 2);
        assert!(plane.windows_is_empty());
    }
}
