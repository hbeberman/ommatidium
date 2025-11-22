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
        let id = crate::next_id();
        Window {
            id,
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
