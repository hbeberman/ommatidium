use ommatidium::error::OmmaErr;
use ommatidium::session::Session;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_window() -> Result<(), OmmaErr> {
        let mut session = Session::new_headless(50, 50)?;
        let window_id = session
            .new_window(3, 4)
            .offset(1, 2)
            .parent(0)
            .submit(&mut session)?;
        let window = session.window(window_id).unwrap();
        assert!(window.offset_x() == 1);
        assert!(window.offset_y() == 2);
        assert!(window.width() == 3);
        assert!(window.height() == 4);
        Ok(())
    }
}
