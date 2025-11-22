use crate::error::OmmaErr;
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
        y: u16,
        x: u16,
        height: u16,
        width: u16,
    ) -> Result<u32, OmmaErr> {
        let plane = match self.planes.iter_mut().find(|p| p.id() == plane_id) {
            Some(plane) => plane,
            None => return Err(OmmaErr::new(&format!("plane_id {} invalid", plane_id))),
        };

        let id = plane.new_window(y, x, height, width)?;
        Ok(id)
    }

    pub fn find_window(&self, window_id: u32) -> Result<Window, OmmaErr> {
        let mut windows = Vec::new();
        for plane in self.planes.iter() {
            let window = plane.find_window(window_id)?;
            windows.push(window);
        }
        if windows.len() > 1 {
            return Err(OmmaErr::new(&format!(
                "window_id {} parented to multiple planes",
                window_id
            )));
        }
        if windows.is_empty() {
            Err(OmmaErr::new(&format!("window_id {} not found", window_id)))
        } else {
            Ok(windows.into_iter().next().unwrap())
        }
    }

    pub fn windows_is_empty(&self, plane_id: u32) -> Result<bool, OmmaErr> {
        let plane = self.find_plane(plane_id)?;
        Ok(plane.windows_is_empty())
    }
}
