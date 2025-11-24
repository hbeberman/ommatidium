use crate::error::OmmaErr;
use crate::ommacell::*;
use crate::term::OmmaTerm;

#[allow(dead_code)]
#[derive(Default, Clone)]
pub struct Window {
    id: u32,
    parent_id: u32,
    plane_x: usize,
    plane_y: usize,
    width: usize,
    height: usize,
    view_width: usize,
    view_height: usize,
    scroll_x: usize,
    scroll_y: usize,
    border: bool,
    buffer: Vec<Vec<OmmaCell>>,
}

pub struct WindowBuilder {
    parent_id: u32,
    plane_x: usize,
    plane_y: usize,
    width: usize,
    height: usize,
    view_width: usize,
    view_height: usize,
    scroll_x: usize,
    scroll_y: usize,
    border: bool,
}

impl WindowBuilder {
    pub fn new(width: usize, height: usize) -> WindowBuilder {
        WindowBuilder {
            parent_id: 0,
            plane_x: 0,
            plane_y: 0,
            width,
            height,
            view_width: width,
            view_height: height,
            scroll_x: 0,
            scroll_y: 0,
            border: false,
        }
    }

    pub fn scroll(mut self, scroll_x: usize, scroll_y: usize) -> WindowBuilder {
        self.scroll_x = scroll_x;
        self.scroll_y = scroll_y;
        self
    }

    pub fn plane(mut self, parent_id: u32, plane_x: usize, plane_y: usize) -> WindowBuilder {
        self.parent_id = parent_id;
        self.plane_x = plane_x;
        self.plane_y = plane_y;
        self
    }

    pub fn build(self) -> Result<(u32, Window), OmmaErr> {
        let id = crate::next_id()?;
        let buffer = vec![vec![OmmaCell::transparent(); self.height]; self.width];
        Ok((
            id,
            Window {
                id,
                parent_id: self.parent_id,
                plane_x: self.plane_x,
                plane_y: self.plane_y,
                width: self.width,
                height: self.height,
                view_width: self.view_width,
                view_height: self.view_height,
                scroll_x: self.scroll_x,
                scroll_y: self.scroll_y,
                border: self.border,
                buffer,
            },
        ))
    }
}

#[allow(dead_code)]
impl Window {
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn parent_id(&self) -> u32 {
        self.parent_id
    }
    pub fn plane_x(&self) -> usize {
        self.plane_x
    }
    pub fn plane_y(&self) -> usize {
        self.plane_y
    }
    pub fn view_width(&self) -> usize {
        self.view_width
    }
    pub fn view_height(&self) -> usize {
        self.view_height
    }

    pub fn set_ommacell(&mut self, x: usize, y: usize, ommacell: &OmmaCell) -> Result<(), OmmaErr> {
        if x >= self.width || y >= self.height {
            return Err(OmmaErr::new(&format!(
                "window_id {} invalid ommacell write to {}:{} (max {}:{})",
                self.id,
                x,
                y,
                self.view_width(),
                self.view_height()
            )));
        };
        self.buffer[x][y] = ommacell.clone();
        Ok(())
    }

    pub fn get_ommacell(&self, x: usize, y: usize) -> Result<OmmaCell, OmmaErr> {
        if x >= self.height || y >= self.height {
            return Err(OmmaErr::new(&format!(
                "window_id {} invalid ommacell read from {}:{} (max {}:{})",
                self.id,
                x,
                y,
                self.view_width(),
                self.view_height()
            )));
        };
        Ok(self.buffer[x][y].clone())
    }

    pub fn blit(&self, term: &mut OmmaTerm) -> Result<u32, OmmaErr> {
        let mut written = 0;
        for x in 0..self.view_width {
            for y in 0..self.view_height {
                written +=
                    term.put_cell_at(x + self.plane_x, y + self.plane_y, &self.buffer[x][y])?;
            }
        }
        Ok(written)
    }

    pub fn fill_window(&mut self, cell: &OmmaCell) -> Result<u32, OmmaErr> {
        for x in 0..self.width {
            for y in 0..self.height {
                self.buffer[x][y] = cell.clone();
            }
        }

        Ok(self.view_width as u32 * self.view_height as u32)
    }

    // TODO: Fix this to stash border info into Window then only apply it when blitting
    pub fn set_window_border(&mut self, cells: Vec<&OmmaCell>) -> Result<u32, OmmaErr> {
        let max_width = self.view_width - 1;
        let max_height = self.view_height - 1;
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
        for x in 1..self.view_width - 1 {
            self.buffer[x][0_usize] = horiz.clone();
            self.buffer[x][max_height] = horiz.clone();
        }

        for y in 1..self.view_height - 1 {
            self.buffer[0_usize][y] = vert.clone();
            self.buffer[max_width][y] = vert.clone();
        }
        self.buffer[0_usize][0_usize] = corner.clone();
        self.buffer[0_usize][max_height] = corner.clone();
        self.buffer[max_width][0_usize] = corner.clone();
        self.buffer[max_width][max_height] = corner.clone();

        self.border = true;

        Ok(self.view_width as u32 * 2 + self.view_height as u32 * 2 - 4)
    }

    pub fn window_string_raw(
        &mut self,
        x: usize,
        y: usize,
        cell: &OmmaCell,
        string: String,
    ) -> Result<u32, OmmaErr> {
        let (mut x, y, max_width, max_height) = if self.border {
            (x + 1, y + 1, self.width - 2, self.height - 2)
        } else {
            (x, y, self.width - 1, self.height - 1)
        };

        if x + string.len() > max_width || y > max_height {
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
