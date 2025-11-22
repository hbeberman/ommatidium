use ommatidium::window::Window;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window() {
        let window = Window::new(1, 2, 3, 4);
        let window2 = Window::new(1, 2, 3, 4);
        let id = window.id();
        eprintln!("ID == {}", id);
        let id = window2.id();
        eprintln!("ID == {}", id);
        assert!(window.id() == 1);
        assert!(window2.id() == 2);
        assert!(window.y() == 1);
        assert!(window.x() == 2);
        assert!(window.height() == 3);
        assert!(window.width() == 4);
    }
}
