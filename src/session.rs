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
        let term = OmmaTerm::new()?;
        Session::new_inner(term)
    }

    pub fn new_headless(max_row: u16, max_col: u16) -> Result<Self, OmmaErr> {
        let term = OmmaTerm::new_mock(max_row, max_col)?;
        Session::new_inner(term)
    }

    pub fn default_headless() -> Result<Self, OmmaErr> {
        let term = OmmaTerm::new_mock(50, 50)?;
        Session::new_inner(term)
    }

    fn new_inner(term: OmmaTerm) -> Result<Self, OmmaErr> {
        if crate::current_id() != 0 {
            return Err(OmmaErr::new(
                "Session::new() may only be invoked once per executable lifetime",
            ));
        }
        let windows: Vec<Window> = Vec::new();
        let children: Vec<u32> = Vec::new();

        let mut session = Session {
            term,
            windows,
            children,
        };
        // Reserve window id 0 with a valid window
        session
            .new_window(30, 30)
            .name("Backdrop".to_string())
            .virt()
            .submit(&mut session)?;
        Ok(session)
    }

    pub fn new_window(&self, width: usize, height: usize) -> WindowBuilder {
        WindowBuilder::new(width, height)
    }

    /// submit WindowBuilder into the session as a new window, returns window id
    pub fn submit(&mut self, windowbuilder: WindowBuilder) -> Result<u32, OmmaErr> {
        windowbuilder.submit(self)
    }

    // pub fn fn_window(&mut self, window_id: u32) -> Result<(u32, Vec<u32>), OmmaErr> {}

    pub(crate) fn register_window(&mut self, window: Window) -> Result<u32, OmmaErr> {
        let id = window.id();
        let parent = window.parent_id();
        self.windows.push(window);
        let parent_window = self.window(parent)?;
        parent_window.add_child(id);
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

    /// render draws the current state of the session to the terminal
    pub fn render(&mut self) -> Result<u32, OmmaErr> {
        let Self { term, windows, .. } = self;
        let window = &windows[0];
        window.blit(windows, term)?;
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

#[test]
fn new() -> Result<(), OmmaErr> {
    let session = Session::new_headless(50, 50)?;
    assert!(session.windows.len() == 1);
    Ok(())
}
