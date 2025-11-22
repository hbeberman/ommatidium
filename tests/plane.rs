use ommatidium::plane::Plane;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let plane = Plane::new().unwrap();
        assert!(plane.windows_is_empty());
    }

    #[test]
    fn find_window() {
        let mut plane = Plane::new().unwrap();
        let window_id = plane.new_window(1, 2, 3, 4).unwrap();
        eprintln!("window_id {window_id}");
        let window = plane.find_window(window_id).unwrap();
        assert!(window.y() == 1);
    }
}
