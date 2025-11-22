#[allow(dead_code)]
#[derive(Default)]
pub struct Window {
    id: u32,
    y: u16,
    x: u16,
    height: u16,
    width: u16,
}

#[allow(dead_code)]
impl Window {
    pub fn new(y: u16, x: u16, height: u16, width: u16) -> Self {
        Window {
            id: 0,
            y,
            x,
            height,
            width,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn y(&self) -> u16 {
        self.y
    }
    pub fn x(&self) -> u16 {
        self.x
    }
    pub fn height(&self) -> u16 {
        self.height
    }
    pub fn width(&self) -> u16 {
        self.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window() {
        let window = Window::new(1, 2, 3, 4);
        assert!(window.id() == 0);
        assert!(window.y() == 1);
        assert!(window.x() == 2);
        assert!(window.height() == 3);
        assert!(window.width() == 4);
    }
}
