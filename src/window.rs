use crate::error::OmmaErr;
use crate::ommacell::*;
use crate::term::OmmaTerm;

#[allow(dead_code)]
#[derive(Default, Clone)]
pub struct Window {
    id: u32,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    buffer: Vec<Vec<OmmaCell>>,
}

#[allow(dead_code)]
impl Window {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Result<Self, OmmaErr> {
        let id = crate::next_id()?;
        let buffer = vec![vec![OmmaCell::default(); height as usize]; width as usize];
        Ok(Window {
            id,
            x,
            y,
            width,
            height,
            buffer,
        })
    }

    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn x(&self) -> u16 {
        self.x
    }
    pub fn y(&self) -> u16 {
        self.y
    }
    pub fn width(&self) -> u16 {
        self.width
    }
    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn set_ommacell(&mut self, x: u16, y: u16, ommacell: &OmmaCell) -> Result<(), OmmaErr> {
        if x >= self.width || y >= self.height {
            return Err(OmmaErr::new(&format!(
                "window_id {} invalid ommacell write to {}:{} (max {}:{})",
                self.id,
                x,
                y,
                self.width(),
                self.height()
            )));
        };
        self.buffer[x as usize][y as usize] = ommacell.clone();
        Ok(())
    }

    pub fn get_ommacell(&self, x: u16, y: u16) -> Result<OmmaCell, OmmaErr> {
        if x >= self.width || y >= self.height {
            return Err(OmmaErr::new(&format!(
                "window_id {} invalid ommacell read from {}:{} (max {}:{})",
                self.id,
                x,
                y,
                self.width(),
                self.height()
            )));
        };
        Ok(self.buffer[x as usize][y as usize].clone())
    }

    pub fn blit(&self, term: &mut OmmaTerm) -> Result<u32, OmmaErr> {
        let mut written = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                written += 1;
                term.put_cell_at(x + self.x, y + self.y, &self.buffer[x as usize][y as usize])?;
            }
        }
        Ok(written)
    }

    pub fn fill_window(&mut self, cell: &OmmaCell) -> Result<u32, OmmaErr> {
        for x in 0..self.width {
            for y in 0..self.height {
                self.buffer[x as usize][y as usize] = cell.clone();
            }
        }

        Ok(self.width as u32 * self.height as u32)
    }
}
