use ommatidium::session::Session;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let session = Session::new().unwrap();
        assert!(session.planes_is_empty());
    }

    #[test]
    fn windows_is_empty() {
        let mut session = Session::new().unwrap();
        let plane_id = session.new_plane().unwrap();
        assert!(session.windows_is_empty(plane_id).unwrap());
        session.new_window(plane_id, 1, 2, 3, 4).unwrap();
        assert!(!session.windows_is_empty(plane_id).unwrap());
    }

    #[test]
    fn find_plane() {
        let mut session = Session::new().unwrap();
        let plane_id = session.new_plane().unwrap();
        let plane = session.find_plane(plane_id).unwrap();
        assert!(plane_id == plane.id());
    }

    #[test]
    fn find_window() {
        let mut session = Session::new().unwrap();
        let plane = session.new_plane().unwrap();
        let window_id = session.new_window(plane, 1, 2, 3, 4).unwrap();
        let window = session.find_window(window_id).unwrap();
        assert!(window.y() == 1);
        assert!(window.x() == 2);
        assert!(window.height() == 3);
        assert!(window.width() == 4);
    }
}
