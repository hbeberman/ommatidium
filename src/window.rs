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
    border: bool,
    buffer: Vec<Vec<OmmaCell>>,
}

#[allow(dead_code)]
impl Window {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Result<Self, OmmaErr> {
        let id = crate::next_id()?;
        let buffer = vec![vec![OmmaCell::transparent(); height as usize]; width as usize];
        Ok(Window {
            id,
            x,
            y,
            width,
            height,
            border: false,
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
                written +=
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

    pub fn set_window_border(&mut self, cells: Vec<&OmmaCell>) -> Result<u32, OmmaErr> {
        let max_width = self.width as usize - 1;
        let max_height = self.height as usize - 1;
        let (horiz, vert, corner) = match cells.len() {
            1 => (cells[0], cells[0], cells[0]),
            2 => (cells[0], cells[1], cells[0]),
            3 => (cells[0], cells[1], cells[2]),
            _ => {
                return Err(OmmaErr::new(&format!(
                    "invalid set_border vec length {}",
                    cells.len()
                )));
            }
        };
        for x in 1..self.width - 1 {
            self.buffer[x as usize][0_usize] = horiz.clone();
            self.buffer[x as usize][max_height] = horiz.clone();
        }

        for y in 1..self.height - 1 {
            self.buffer[0_usize][y as usize] = vert.clone();
            self.buffer[max_width][y as usize] = vert.clone();
        }
        self.buffer[0_usize][0_usize] = corner.clone();
        self.buffer[0_usize][max_height] = corner.clone();
        self.buffer[max_width][0_usize] = corner.clone();
        self.buffer[max_width][max_height] = corner.clone();

        self.border = true;

        Ok(self.width as u32 * 2 + self.height as u32 * 2 - 4)
    }

    pub fn write_window_string(
        &mut self,
        x: u16,
        y: u16,
        cell: &OmmaCell,
        string: String,
    ) -> Result<u32, OmmaErr> {
        let (mut x, y, max_width, max_height) = if self.border {
            (
                x + 1,
                y + 1,
                self.width as usize - 2,
                self.height as usize - 2,
            )
        } else {
            (x, y, self.width as usize - 1, self.height as usize - 1)
        };

        if x as usize + string.len() > max_width || y as usize > max_height {
            return Err(OmmaErr::new(&format!(
                "window_id {} invalid write_window_string {}:{} (max {}:{}) = {}",
                self.id(),
                x,
                y,
                max_width,
                max_height,
                string
            )));
        }

        let mut cell = cell.clone();
        for ch in string.chars() {
            cell.ch = ch;
            self.set_ommacell(x, y, &cell)?;
            x += 1;
        }

        Ok(string.len() as u32)
    }
}
