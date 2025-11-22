use crate::error::OmmaErr;
use crate::term::OmmaTerm;
use crate::window::*;

#[allow(dead_code)]
#[derive(Default, Clone)]
pub struct Plane {
    id: u32,
    windows: Vec<Window>,
}

#[allow(dead_code)]
impl Plane {
    pub fn new() -> Result<Self, OmmaErr> {
        let windows: Vec<Window> = Vec::new();
        let id = crate::next_id()?;
        Ok(Plane { id, windows })
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn windows(&self) -> &Vec<Window> {
        &self.windows
    }

    pub fn windows_is_empty(&self) -> bool {
        self.windows.is_empty()
    }

    pub fn new_window(&mut self, x: u16, y: u16, width: u16, height: u16) -> Result<u32, OmmaErr> {
        let window = Window::new(x, y, width, height)?;
        let id = window.id();

        self.windows.push(window);
        Ok(id)
    }

    pub fn find_window(&mut self, window_id: u32) -> Result<&mut Window, OmmaErr> {
        match self.windows.iter_mut().find(|w| w.id() == window_id) {
            Some(window) => Ok(window),
            None => Err(OmmaErr::new(&format!("window_id {} invalid", window_id))),
        }
    }

    pub fn find_window_mut(&mut self, window_id: u32) -> Result<&mut Window, OmmaErr> {
        match self.windows.iter_mut().find(|w| w.id() == window_id) {
            Some(window) => Ok(window),
            None => Err(OmmaErr::new(&format!("window_id {} invalid", window_id))),
        }
    }

    pub fn blit(&mut self, term: &mut OmmaTerm) -> Result<u32, OmmaErr> {
        let mut written = 0;
        for window in self.windows().iter() {
            written += window.blit(term)?;
        }
        Ok(written)
    }
}
