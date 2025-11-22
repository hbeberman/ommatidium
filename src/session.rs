use crate::error::OmmaErr;
use crate::ommacell::OmmaCell;
use crate::plane::*;
use crate::window::*;

#[allow(dead_code)]
#[derive(Default)]
pub struct Session {
    id: u32,
    planes: Vec<Plane>,
}

#[allow(dead_code)]
impl Session {
    pub fn new() -> Result<Self, OmmaErr> {
        let planes: Vec<Plane> = Vec::new();
        let id = crate::next_id()?;
        Ok(Session { id, planes })
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

    pub fn find_window(&self, window_id: u32) -> Result<&Window, OmmaErr> {
        let mut found: Option<&Window> = None;
        for plane in self.planes.iter() {
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

        for (idx, plane) in self.planes.iter().enumerate() {
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
        window.set_ommacell(x, y, ommacell)?;
        Ok(())
    }

    pub fn get_ommacell(&self, window_id: u32, x: u16, y: u16) -> Result<OmmaCell, OmmaErr> {
        let window = self.find_window(window_id)?;
        window.get_ommacell(x, y)
    }
}
