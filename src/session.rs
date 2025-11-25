use crate::error::OmmaErr;
use crate::ommacell::*;
use crate::term::OmmaTerm;
use crate::window::*;

#[allow(dead_code)]
pub struct Session {
    term: OmmaTerm,
    windows: Vec<Window>,
    children: Vec<u32>,
}

#[allow(dead_code)]
impl Session {
    pub fn new() -> Result<Self, OmmaErr> {
        if crate::current_id() != 0 {
            return Err(OmmaErr::new(
                "Session::new() may only be invoked once per executable lifetime",
            ));
        }
        let term = OmmaTerm::new()?;
        let windows: Vec<Window> = Vec::new();
        let children: Vec<u32> = Vec::new();

        let mut session = Session {
            term,
            windows,
            children,
        };
        // TODO: Implement cleaning our backplane better so this isnt necessary
        session
            .new_window(30, 30)
            .name("Backdrop".to_string())
            .fill(&BLANK_CELL)
            .submit(&mut session)?;
        Ok(session)
    }

    pub fn new_window(&self, width: usize, height: usize) -> WindowBuilder {
        WindowBuilder::new(width, height)
    }

    pub(crate) fn push_child(&mut self, child_id: u32) -> Result<(), OmmaErr> {
        // TODO: Make sure child ID has never been used before
        self.children.push(child_id);
        Ok(())
    }

    //    pub fn fn_window(&mut self, window_id: u32) -> Result<(u32, Vec<u32>), OmmaErr> {}

    pub(crate) fn push_window(&mut self, window: Window) -> Result<u32, OmmaErr> {
        let id = window.id();
        self.windows.push(window);
        Ok(id)
    }

    pub fn window(&mut self, window_id: u32) -> Result<&mut Window, OmmaErr> {
        let id = window_id as usize;
        if id < self.windows.len() {
            Ok(&mut self.windows[id])
        } else {
            Err(OmmaErr::new(&format!(
                "invalid window_id {}, max is {}",
                window_id,
                self.windows.len() - 1,
            )))
        }
    }

    pub fn set_ommacell(
        &mut self,
        window_id: u32,
        x: usize,
        y: usize,
        ommacell: OmmaCell,
    ) -> Result<(), OmmaErr> {
        let window = self.window(window_id)?;
        window.set_ommacell(x, y, &ommacell)?;
        Ok(())
    }

    pub fn get_ommacell(
        &mut self,
        window_id: u32,
        x: usize,
        y: usize,
    ) -> Result<OmmaCell, OmmaErr> {
        let window = self.window(window_id)?;
        window.get_ommacell(x, y)
    }

    pub fn blit(&mut self) -> Result<u32, OmmaErr> {
        let mut written = 0;
        let Self {
            children,
            term,
            windows,
            ..
        } = self;
        for &index in children.iter() {
            let window = windows
                .get_mut(index as usize)
                .ok_or_else(|| OmmaErr::new(&format!("invalid child index {}", index)))?;
            written += window.blit(term)?;
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

    pub fn set_window_border(
        &mut self,
        window_id: u32,
        cell: Vec<&OmmaCell>,
    ) -> Result<u32, OmmaErr> {
        let window = self.window(window_id)?;
        window.set_border(cell)
    }

    pub fn window_string_raw(
        &mut self,
        window_id: u32,
        x: usize,
        y: usize,
        cell: &OmmaCell,
        string: String,
    ) -> Result<u32, OmmaErr> {
        let window = self.window(window_id)?;
        window.window_string_raw(x, y, cell, string)
    }
}
