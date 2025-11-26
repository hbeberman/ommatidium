use crate::error::OmmaErr;
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
    /// new creates a new session, it may only be invoked once per executable lifetime
    pub fn new() -> Result<Self, OmmaErr> {
        let term = OmmaTerm::new()?;
        Self::new_inner(term)
    }

    /// new_headless creates a session with a fake max_row by max_col term
    pub fn new_headless(max_row: u16, max_col: u16) -> Result<Self, OmmaErr> {
        let term = OmmaTerm::new_mock(max_row, max_col)?;
        Self::new_inner(term)
    }

    /// default_headless creates a session with a fake 50 by 50 term
    pub fn default_headless() -> Result<Self, OmmaErr> {
        let term = OmmaTerm::new_mock(50, 50)?;
        Self::new_inner(term)
    }

    // new_inner implements session init shared between headed/headless
    fn new_inner(term: OmmaTerm) -> Result<Self, OmmaErr> {
        if crate::current_id() != 0 {
            return Err(OmmaErr::new(
                "Session::new() may only be invoked once per executable lifetime",
            ));
        }

        let mut session = Session {
            term,
            windows: Vec::new(),
            children: Vec::new(),
        };

        // Reserve window id 0 with a valid window
        session
            .new_window(1, 1)
            .name("System Window".to_string())
            .virt()
            .submit(&mut session)?;
        Ok(session)
    }

    /// new_window creates a new WindowBuilder object for further building
    pub fn new_window(&self, width: usize, height: usize) -> WindowBuilder {
        WindowBuilder::new(width, height)
    }

    /// submit WindowBuilder into the session as a new window, returns window id
    pub fn submit(&mut self, windowbuilder: WindowBuilder) -> Result<u32, OmmaErr> {
        windowbuilder.submit(self)
    }

    // TODO: implement to run closures on every descendant window
    //pub fn fn_window(&mut self, window_id: u32) -> Result<(u32, Vec<u32>), OmmaErr> {}

    // register_window adds a window into the session and returns its id
    pub(crate) fn register_window(&mut self, window: Window) -> Result<u32, OmmaErr> {
        let id = window.id();
        let parent = window.parent_id();
        self.windows.push(window);
        let parent_window = self.window(parent)?;
        parent_window.add_child(id);
        Ok(id)
    }

    /// window returns the window object corresponding to window_id if available
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

    /// render draws the current state of the session to the terminal
    pub fn render(&mut self) -> Result<u32, OmmaErr> {
        let Self { term, windows, .. } = self;
        let window = &windows[0];
        window.blit(windows, term, 0, 0)?;
        self.term.render()
    }

    /// read_key returns a single keypress from the terminal
    pub fn read_key(&mut self) -> Result<Option<char>, OmmaErr> {
        self.term.read_key()
    }
}

#[test]
fn new() -> Result<(), OmmaErr> {
    let session = Session::new_headless(50, 50)?;
    assert!(session.windows.len() == 1);
    Ok(())
}
