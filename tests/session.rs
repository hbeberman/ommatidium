use ommatidium::ommacell::OmmaCell;
use ommatidium::session::Session;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let session = Session::new_for_tests().unwrap();
        assert!(session.planes_is_empty());
    }

    #[test]
    fn windows_is_empty() {
        let mut session = Session::new_for_tests().unwrap();
        let plane_id = session.new_plane().unwrap();
        assert!(session.windows_is_empty(plane_id).unwrap());
        session.new_window(plane_id, 1, 2, 3, 4).unwrap();
        assert!(!session.windows_is_empty(plane_id).unwrap());
    }

    #[test]
    fn find_plane() {
        let mut session = Session::new_for_tests().unwrap();
        let plane_id = session.new_plane().unwrap();
        let plane = session.find_plane(plane_id).unwrap();
        assert!(plane_id == plane.id());
    }

    #[test]
    fn find_window() {
        let mut session = Session::new_for_tests().unwrap();
        let plane_id = session.new_plane().unwrap();
        let window_id = session.new_window(plane_id, 1, 2, 3, 4).unwrap();
        let window = session.find_window(window_id).unwrap();
        assert!(window.x() == 1);
        assert!(window.y() == 2);
        assert!(window.width() == 3);
        assert!(window.height() == 4);
    }

    #[test]
    fn set_ommacell() {
        let mut session = Session::new_for_tests().unwrap();
        let plane_id = session.new_plane().unwrap();
        let window_id = session.new_window(plane_id, 5, 5, 10, 10).unwrap();
        session
            .set_ommacell(
                window_id,
                3,
                3,
                OmmaCell {
                    ch: '@',
                    fg: 0,
                    bg: 0,
                    attrs: 0,
                },
            )
            .unwrap();
    }

    #[test]
    fn get_ommacell() {
        let mut session = Session::new_for_tests().unwrap();
        let plane_id = session.new_plane().unwrap();
        let window_id = session.new_window(plane_id, 5, 5, 10, 10).unwrap();
        session
            .set_ommacell(
                window_id,
                3,
                4,
                OmmaCell {
                    ch: '@',
                    fg: 0,
                    bg: 0,
                    attrs: 0,
                },
            )
            .unwrap();
        let ommacell = session.get_ommacell(window_id, 3, 4).unwrap();
        eprintln!("char: {}", ommacell.ch);
        assert!(ommacell.ch == '@');
    }
}
