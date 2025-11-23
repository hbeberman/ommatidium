use crate::error::OmmaErr;
use crate::ommacell::OmmaCell;
use crate::plane::*;
use crate::term::OmmaTerm;
use crate::window::*;

#[allow(dead_code)]
pub struct Session {
    id: u32,
    term: OmmaTerm,
    planes: Vec<Plane>,
}

#[allow(dead_code)]
impl Session {
    pub fn new() -> Result<Self, OmmaErr> {
        let planes: Vec<Plane> = Vec::new();
        let id = crate::next_id()?;
        let term = OmmaTerm::new()?;
        Ok(Session { id, term, planes })
    }

    pub fn new_for_tests() -> Result<Self, OmmaErr> {
        let planes: Vec<Plane> = Vec::new();
        let id = crate::next_id()?;
        let term = OmmaTerm::new_mock(24, 80)?;
        Ok(Session { id, term, planes })
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn planes_is_empty(&self) -> bool {
        self.planes.is_empty()
    }

    pub fn new_plane(&mut self) -> Result<u32, OmmaErr> {
        let plane = Plane::new()?;
        let id = plane.id();
        self.planes.push(plane);
        Ok(id)
    }

    pub fn find_plane(&self, plane_id: u32) -> Result<Plane, OmmaErr> {
        match self.planes.iter().find(|p| p.id() == plane_id) {
            Some(plane) => Ok(plane.clone()),
            None => Err(OmmaErr::new(&format!("plane_id {} invalid", plane_id))),
        }
    }

    pub fn new_window(
        &mut self,
        plane_id: u32,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) -> Result<u32, OmmaErr> {
        let plane = match self.planes.iter_mut().find(|p| p.id() == plane_id) {
            Some(plane) => plane,
            None => return Err(OmmaErr::new(&format!("plane_id {} invalid", plane_id))),
        };

        let id = plane.new_window(x, y, width, height)?;
        Ok(id)
    }

    pub fn find_window(&mut self, window_id: u32) -> Result<&mut Window, OmmaErr> {
        let mut found: Option<&mut Window> = None;
        for plane in self.planes.iter_mut() {
            match plane.find_window(window_id) {
                Ok(window) => {
                    if found.is_some() {
                        return Err(OmmaErr::new(&format!(
                            "window_id {} parented to multiple planes",
                            window_id
                        )));
                    }
                    found = Some(window);
                }
                Err(_) => continue,
            }
        }
        match found {
            Some(window) => Ok(window),
            None => Err(OmmaErr::new(&format!("window_id {} not found", window_id))),
        }
    }

    pub fn find_window_mut(&mut self, window_id: u32) -> Result<&mut Window, OmmaErr> {
        let mut plane_index: Option<usize> = None;

        for (idx, plane) in self.planes.iter_mut().enumerate() {
            if plane.find_window(window_id).is_ok() {
                if plane_index.is_some() {
                    return Err(OmmaErr::new(&format!(
                        "window_id {} parented to multiple planes",
                        window_id
                    )));
                }
                plane_index = Some(idx);
            }
        }

        match plane_index {
            Some(idx) => self.planes[idx].find_window_mut(window_id),
            None => Err(OmmaErr::new(&format!("window_id {} not found", window_id))),
        }
    }

    pub fn windows_is_empty(&self, plane_id: u32) -> Result<bool, OmmaErr> {
        let plane = self.find_plane(plane_id)?;
        Ok(plane.windows_is_empty())
    }

    pub fn set_ommacell(
        &mut self,
        window_id: u32,
        x: u16,
        y: u16,
        ommacell: OmmaCell,
    ) -> Result<(), OmmaErr> {
        let window = self.find_window_mut(window_id)?;
        window.set_ommacell(x, y, &ommacell)?;
        Ok(())
    }

    pub fn get_ommacell(&mut self, window_id: u32, x: u16, y: u16) -> Result<OmmaCell, OmmaErr> {
        let window = self.find_window(window_id)?;
        window.get_ommacell(x, y)
    }

    pub fn blit(&mut self) -> Result<u32, OmmaErr> {
        let mut written = 0;
        for plane in self.planes.iter_mut() {
            written += plane.blit(&mut self.term)?;
        }
        Ok(written)
    }

    pub fn render(&mut self) -> Result<u32, OmmaErr> {
        self.blit()?;
        self.term.render()
    }

    pub fn read_key(&mut self) -> Result<Option<char>, OmmaErr> {
        self.term.read_key()
    }

    pub fn fill_window(&mut self, window_id: u32, cell: &OmmaCell) -> Result<u32, OmmaErr> {
        let window = self.find_window(window_id)?;
        window.fill_window(cell)
    }

    pub fn set_window_border(
        &mut self,
        window_id: u32,
        cell: Vec<&OmmaCell>,
    ) -> Result<u32, OmmaErr> {
        let window = self.find_window(window_id)?;
        window.set_window_border(cell)
    }

    pub fn write_window_string(
        &mut self,
        window_id: u32,
        x: u16,
        y: u16,
        cell: &OmmaCell,
        string: String,
    ) -> Result<u32, OmmaErr> {
        let window = self.find_window(window_id)?;
        window.write_window_string(x, y, cell, string)
    }
}
