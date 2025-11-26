use ommatidium::ommacell::OmmaCell;
use ommatidium::session::Session;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_window() -> Result<(), OmmaErr> {
        let mut session = Session::new_headless(50, 50)?;
        let window_id = session.new_window(1, 2).parent(0).submit(&mut session)?;
        let window = session.window(window_id).unwrap();
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
