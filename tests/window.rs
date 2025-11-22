use ommatidium::window::Window;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let window = Window::new(1, 2, 3, 4).unwrap();
        assert!(window.x() == 1);
        assert!(window.y() == 2);
        assert!(window.width() == 3);
        assert!(window.height() == 4);
    }
}
