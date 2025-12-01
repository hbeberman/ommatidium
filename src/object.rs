use crate::cell::OmmaCell;
use crate::error::OmmaErr;
use crate::session::Session;
use crate::term::OmmaTerm;

#[allow(dead_code)]
#[derive(Default, Clone)]
pub struct Object {
    id: u32,
    parent_id: u32,
    name: String,
    offset_x: usize,
    offset_y: usize,
    hidden: bool,
    cell: Option<OmmaCell>,
}

#[allow(dead_code)]
#[derive(Default)]
pub struct ObjectBuilder {
    parent_id: u32,
    name: Option<String>,
    offset_x: usize,
    offset_y: usize,
    hidden: bool,
    cell: Option<OmmaCell>,
}

impl ObjectBuilder {
    pub fn new() -> Self {
        ObjectBuilder {
            parent_id: 0,
            name: None,
            offset_x: 0,
            offset_y: 0,
            hidden: false,
            cell: None,
        }
    }

    /// parent sets the window that owns this object. Omitting implicitly picks the System Window
    pub fn parent(mut self, parent_id: u32) -> Self {
        self.parent_id = parent_id;
        self
    }

    /// name sets a name for the object
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// offset sets the object offset within the parent
    pub fn offset(mut self, offset_x: usize, offset_y: usize) -> Self {
        self.offset_x = offset_x;
        self.offset_y = offset_y;
        self
    }

    /// hidden marks an object as hidden and it will be skipped during rendering
    pub fn hidden(mut self) -> Self {
        self.hidden = true;
        self
    }

    /// cell sets an object to contain the given OmmaCell
    pub fn cell(mut self, cell: &OmmaCell) -> Self {
        self.cell = Some(cell.clone());
        self
    }

    /// submit adds an ObjectBuilder into the session as a new object, returns object id
    pub fn submit(&self, session: &mut Session) -> Result<u32, OmmaErr> {
        let id = crate::next_object_id()?;
        let name = if let Some(name) = &self.name {
            name
        } else {
            &format!("Unnamed Object #{}", id)
        };
        let object = Object {
            id,
            name: name.to_string(),
            parent_id: self.parent_id,
            offset_x: self.offset_x,
            offset_y: self.offset_y,
            hidden: self.hidden,
            cell: self.cell.clone(),
        };
        let id = session.register_object(object)?;
        Ok(id)
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let visibility = if self.hidden { "H" } else { "V" };
        write!(
            f,
            "{}:{}:{}:{}",
            visibility, self.id, self.name, self.parent_id
        )
    }
}

#[allow(dead_code)]
impl Object {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn window_id(&self) -> u32 {
        self.parent_id
    }

    pub fn offset_x(&self) -> usize {
        self.offset_x
    }

    pub fn offset_y(&self) -> usize {
        self.offset_y
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

    pub fn blit(
        &mut self,
        term: &mut OmmaTerm,
        parent_x: usize,
        parent_y: usize,
    ) -> Result<u32, OmmaErr> {
        if self.hidden {
            return Ok(0);
        }
        let mut written = 0;
        if let Some(cell) = &self.cell {
            written +=
                term.put_cell_at(self.offset_x + parent_x, self.offset_y + parent_y, cell)?;
        }
        Ok(written)
    }
}
