use crate::error::OmmaErr;
use crate::ommacell::*;
use crate::session::Session;
use crate::term::OmmaTerm;

#[allow(dead_code)]
#[derive(Default, Clone)]
pub struct Window {
    id: u32,
    parent_id: u32,
    children: Vec<u32>,
    name: String,
    offset_x: usize,
    offset_y: usize,
    width: usize,
    height: usize,
    view_width: usize,
    view_height: usize,
    scroll_x: usize,
    scroll_y: usize,
    border: bool,
    hidden: bool,
    virt: bool,
    buffer: Vec<Vec<OmmaCell>>,
}

pub struct WindowBuilder {
    parent_id: u32,
    name: Option<String>,
    offset_x: usize,
    offset_y: usize,
    width: usize,
    height: usize,
    view_width: usize,
    view_height: usize,
    scroll_x: usize,
    scroll_y: usize,
    border: bool,
    hidden: bool,
    virt: bool,
    fill: Option<OmmaCell>,
}

impl WindowBuilder {
    pub fn new(width: usize, height: usize) -> WindowBuilder {
        WindowBuilder {
            parent_id: 0,
            name: None,
            offset_x: 0,
            offset_y: 0,
            width,
            height,
            view_width: width,
            view_height: height,
            scroll_x: 0,
            scroll_y: 0,
            border: false,
            hidden: false,
            virt: false,
            fill: None,
        }
    }

    pub fn scroll(mut self, scroll_x: usize, scroll_y: usize) -> WindowBuilder {
        self.scroll_x = scroll_x;
        self.scroll_y = scroll_y;
        self
    }

    pub fn parent(mut self, parent_id: u32) -> WindowBuilder {
        self.parent_id = parent_id;
        self
    }

    pub fn offset(mut self, offset_x: usize, offset_y: usize) -> WindowBuilder {
        self.offset_x = offset_x;
        self.offset_y = offset_y;
        self
    }

    pub fn fill(mut self, fill: &OmmaCell) -> WindowBuilder {
        self.fill = Some(fill.clone());
        self
    }

    pub fn name(mut self, name: String) -> WindowBuilder {
        self.name = Some(name);
        self
    }

    pub fn hidden(mut self) -> WindowBuilder {
        self.hidden = true;
        self
    }

    pub fn virt(mut self) -> WindowBuilder {
        self.virt = true;
        self
    }

    /// submit WindowBuilder into the session as a new window, returns window id
    pub fn submit(&self, session: &mut Session) -> Result<u32, OmmaErr> {
        let id = crate::next_id()?;
        let buffer = vec![vec![OmmaCell::transparent(); self.height]; self.width];
        let name = if let Some(name) = &self.name {
            name
        } else {
            &format!("Unnamed Window #{}", id)
        };

        let mut window = Window {
            id,
            name: name.to_string(),
            parent_id: self.parent_id,
            children: Vec::<u32>::new(),
            offset_x: self.offset_x,
            offset_y: self.offset_y,
            width: self.width,
            height: self.height,
            view_width: self.view_width,
            view_height: self.view_height,
            scroll_x: self.scroll_x,
            scroll_y: self.scroll_y,
            border: self.border,
            hidden: self.hidden,
            virt: self.virt,
            buffer,
        };

        if let Some(fill) = &self.fill {
            let _ = window.fill(fill);
        }

        let id = session.register_window(window)?;

        Ok(id)
    }
}

impl std::fmt::Display for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let visibility = if self.hidden { "H" } else { "V" };
        write!(f, "{}:{}:{}", visibility, self.id, self.name)
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

    pub fn offset_x(&self) -> usize {
        self.offset_x
    }

    pub fn offset_y(&self) -> usize {
        self.offset_y
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn view_width(&self) -> usize {
        self.view_width
    }

    pub fn view_height(&self) -> usize {
        self.view_height
    }

    pub fn is_hidden(&self) -> bool {
        self.hidden
    }

    pub fn toggle_hidden(&mut self) {
        self.hidden = !self.hidden;
    }

    pub fn set_hidden(&mut self) {
        self.hidden = true
    }

    pub fn clear_hidden(&mut self) {
        self.hidden = false
    }

    pub(crate) fn add_child(&mut self, child_id: u32) {
        self.children.push(child_id);
    }

    pub(crate) fn remove_child(&mut self, child_id: u32) -> Result<(), OmmaErr> {
        if let Some(index) = self.children.iter().position(|x| *x == child_id) {
            self.children.remove(index);
        } else {
            return Err(OmmaErr::new(&format!(
                "error removing window {} from parent {}, not owned",
                child_id,
                self.id(),
            )));
        }
        Ok(())
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
        if x >= self.width || y >= self.height {
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

    pub fn blit(
        &self,
        windows: &Vec<Window>,
        term: &mut OmmaTerm,
        parent_offset_x: usize,
        parent_offset_y: usize,
    ) -> Result<u32, OmmaErr> {
        if self.hidden {
            return Ok(0);
        }
        let offset_x = self.offset_x + parent_offset_x;
        let offset_y = self.offset_y + parent_offset_y;
        let mut written = 0;
        // Skip drawing virtual window contents
        if !self.virt {
            for x in 0..self.view_width {
                for y in 0..self.view_height {
                    written += term.put_cell_at(x + offset_x, y + offset_y, &self.buffer[x][y])?;
                }
            }
        }
        for window_id in &self.children {
            if &self.id() == window_id {
                if window_id == &0 {
                    continue;
                } else {
                    return Err(OmmaErr::new(&format!(
                        "failed to blit window {} due to self own",
                        self.id(),
                    )));
                }
            }
            written += windows[*window_id as usize].blit(windows, term, offset_x, offset_y)?;
        }
        Ok(written)
    }

    pub fn fill(&mut self, cell: &OmmaCell) -> Result<u32, OmmaErr> {
        for x in 0..self.width {
            for y in 0..self.height {
                self.buffer[x][y] = cell.clone();
            }
        }

        Ok(self.view_width as u32 * self.view_height as u32)
    }

    // TODO: Fix this to stash border info into Window then only apply it when blitting
    pub fn set_border(&mut self, cells: Vec<&OmmaCell>) -> Result<u32, OmmaErr> {
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

    pub fn string_raw(
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
