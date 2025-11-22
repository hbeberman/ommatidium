use ommatidium::window::Window;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let window = Window::new(1, 2, 3, 4).unwrap();
        assert!(window.y() == 1);
        assert!(window.x() == 2);
        assert!(window.height() == 3);
        assert!(window.width() == 4);
    }
}
