use crate::border::OmmaBorder;
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
    border: Option<OmmaBorder>,
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
    border: Option<OmmaBorder>,
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
            border: None,
            hidden: false,
            virt: false,
            fill: None,
        }
    }

    /// scroll sets the scrolling offset of the window
    pub fn scroll(mut self, scroll_x: usize, scroll_y: usize) -> WindowBuilder {
        self.scroll_x = scroll_x;
        self.scroll_y = scroll_y;
        self
    }

    /// parent sets the window that owns this window. Omitting parent implicitly sets the parent to
    /// the System Window.
    pub fn parent(mut self, parent_id: u32) -> WindowBuilder {
        self.parent_id = parent_id;
        self
    }

    /// offset sets the windows offset within the parent
    pub fn offset(mut self, offset_x: usize, offset_y: usize) -> WindowBuilder {
        self.offset_x = offset_x;
        self.offset_y = offset_y;
        self
    }

    /// fill fills the entire window with a given ommacell
    pub fn fill(mut self, fill: &OmmaCell) -> WindowBuilder {
        self.fill = Some(fill.clone());
        self
    }

    /// name sets a name for the window
    pub fn name(mut self, name: String) -> WindowBuilder {
        self.name = Some(name);
        self
    }

    /// border sets a border for the window
    pub fn border(mut self, border: &OmmaBorder) -> WindowBuilder {
        self.border = Some(border.clone());
        self
    }

    /// border_raw takes 3 ommacells and uses it as a horiz, vert, corner border for the window
    pub fn border_raw(
        mut self,
        horiz: &OmmaCell,
        vert: &OmmaCell,
        corner: &OmmaCell,
    ) -> WindowBuilder {
        self.border = Some(OmmaBorder::new(horiz, vert, corner));
        self
    }

    /// border_mono takes 1 ommacell and uses it for all sides and corner
    pub fn border_mono(mut self, mono: &OmmaCell) -> WindowBuilder {
        self.border = Some(OmmaBorder::new_mono(mono));
        self
    }

    /// border_hidden marks a border as hidden and it will be skipped during rendering
    pub fn border_hidden(mut self) -> WindowBuilder {
        if let Some(border) = &mut self.border {
            border.set_hidden();
        }
        self
    }

    /// hidden marks a window as hidden and it will be skipped during rendering
    pub fn hidden(mut self) -> WindowBuilder {
        self.hidden = true;
        self
    }

    /// virt marks a window as virtual. Virtual windows are not rendered but can be used to group
    /// them as an object
    pub fn virt(mut self) -> WindowBuilder {
        self.virt = true;
        self
    }

    /// submit adds a WindowBuilder into the session as a new window, returns window id
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
            border: self.border.clone(),
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

    pub fn set_border(&mut self, border: &OmmaBorder) {
        self.border = Some(border.clone());
    }

    pub fn is_border_hidden(&self) -> bool {
        if let Some(border) = &self.border {
            border.hidden()
        } else {
            true
        }
    }

    pub fn toggle_border_hidden(&mut self) {
        if let Some(border) = &mut self.border {
            border.toggle_hidden()
        }
    }

    pub fn set_border_hidden(&mut self) {
        if let Some(border) = &mut self.border {
            border.set_hidden()
        }
    }

    pub fn clear_border_hidden(&mut self) {
        if let Some(border) = &mut self.border {
            border.clear_hidden()
        }
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

    /// add_child pushes a child id into the window's children list
    pub(crate) fn add_child(&mut self, child_id: u32) {
        self.children.push(child_id);
    }

    /// remove_child removes a child id from the windsow's children list
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

    /// set_ommacell sets a location within the window to a selected ommacell
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

    /// get_ommacell retrieves the ommacell aa location within the window to a selected ommacell
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

    /// blit submits the window's contents into the terminal backplane
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

            if let Some(border) = &self.border
                && !border.hidden()
            {
                for x in 0..self.view_width {
                    term.put_cell_at(x + offset_x, offset_y, border.border_top())?;
                    term.put_cell_at(
                        x + offset_x,
                        self.view_height + offset_y - 1,
                        border.border_bottom(),
                    )?;
                }
                for y in 0..self.view_height {
                    term.put_cell_at(offset_x, y + offset_y, border.border_left())?;
                    term.put_cell_at(
                        self.view_width + offset_x - 1,
                        y + offset_y,
                        border.border_right(),
                    )?;
                }

                term.put_cell_at(offset_x, offset_y, border.border_corner())?;
                term.put_cell_at(
                    offset_x,
                    self.view_height + offset_y - 1,
                    border.border_corner(),
                )?;
                term.put_cell_at(
                    self.view_width + offset_x - 1,
                    offset_y,
                    border.border_corner(),
                )?;
                term.put_cell_at(
                    self.view_width + offset_x - 1,
                    self.view_height + offset_y - 1,
                    border.border_corner(),
                )?;
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

    /// fill fills the window with a single ommacell
    pub fn fill(&mut self, cell: &OmmaCell) -> Result<u32, OmmaErr> {
        for x in 0..self.width {
            for y in 0..self.height {
                self.buffer[x][y] = cell.clone();
            }
        }

        Ok(self.view_width as u32 * self.view_height as u32)
    }

    /// string_raw prints a string into a window directly, using the properties from ommacell
    pub fn string_raw(
        &mut self,
        x: usize,
        y: usize,
        cell: &OmmaCell,
        string: String,
    ) -> Result<u32, OmmaErr> {
        let (mut x, y, max_width, max_height) = if self.border.is_some() {
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
