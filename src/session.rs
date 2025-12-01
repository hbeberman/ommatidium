use crate::error::OmmaErr;
use crate::object::*;
use crate::term::OmmaTerm;
use crate::window::*;

#[allow(dead_code)]
pub struct Session {
    term: OmmaTerm,
    windows: Vec<Window>,
    objects: Vec<Object>,
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

    /// new_inner implements session init shared between headed/headless
    fn new_inner(term: OmmaTerm) -> Result<Self, OmmaErr> {
        if crate::current_window_id() != 0 {
            return Err(OmmaErr::new(
                "Session::new() may only be invoked once per executable lifetime",
            ));
        }

        let mut session = Session {
            term,
            windows: Vec::new(),
            objects: Vec::new(),
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

    /// submit_window adds a WindowBuilder into the session as a new window, returns window id
    pub fn submit_window(&mut self, windowbuilder: WindowBuilder) -> Result<u32, OmmaErr> {
        windowbuilder.submit(self)
    }

    // TODO: implement to run closures on every descendant window
    //pub fn fn_window(&mut self, window_id: u32) -> Result<(u32, Vec<u32>), OmmaErr> {}

    /// register_window adds a window into the session and returns its id
    pub(crate) fn register_window(&mut self, window: Window) -> Result<u32, OmmaErr> {
        let id = window.id();
        let parent = window.parent_id();
        self.windows.push(window);
        let parent_window = self.window(parent)?;
        parent_window.add_child(id);
        Ok(id)
    }

    /// window returns the window corresponding to window_id if available
    pub fn window(&mut self, window_id: u32) -> Result<&mut Window, OmmaErr> {
        Self::window_direct(&mut self.windows, window_id)
    }

    /// window_direct returns the window corresponding to window_id if available
    pub(crate) fn window_direct(
        windows: &mut [Window],
        window_id: u32,
    ) -> Result<&mut Window, OmmaErr> {
        let id = window_id as usize;
        if id < windows.len() {
            Ok(&mut windows[id])
        } else {
            Err(OmmaErr::new(&format!(
                "invalid window_id {}, max is {}",
                window_id,
                windows.len() - 1,
            )))
        }
    }

    /// new_object creates a new ObjectBuilder object for further building
    pub fn new_object(&self) -> ObjectBuilder {
        ObjectBuilder::new()
    }

    /// submit_object adds an ObjectBuilder into the session as a new object, returns object id
    pub fn submit_object(&mut self, objectbuilder: ObjectBuilder) -> Result<u32, OmmaErr> {
        objectbuilder.submit(self)
    }

    /// register_object adds on object into the session and returns its id
    pub(crate) fn register_object(&mut self, object: Object) -> Result<u32, OmmaErr> {
        let id = object.id();
        let window = object.window_id();
        self.objects.push(object);
        let parent_window = self.window(window)?;
        parent_window.add_object(id);
        Ok(id)
    }

    /// object returns the object corresponding to object_id if available
    pub fn object(&mut self, object_id: u32) -> Result<&mut Object, OmmaErr> {
        let id = object_id as usize;
        if id < self.objects.len() {
            Ok(&mut self.objects[id])
        } else {
            Err(OmmaErr::new(&format!(
                "invalid object_id {}, max is {}",
                object_id,
                self.objects.len() - 1,
            )))
        }
    }

    /// object_direct returns the object corresponding to object_id if available
    pub fn object_direct(objects: &mut [Object], object_id: u32) -> Result<&mut Object, OmmaErr> {
        let id = object_id as usize;
        if id < objects.len() {
            Ok(&mut objects[id])
        } else {
            Err(OmmaErr::new(&format!(
                "invalid object_id {}, max is {}",
                object_id,
                objects.len() - 1,
            )))
        }
    }

    /// render draws the current state of the session to the terminal
    pub fn render(&mut self) -> Result<u32, OmmaErr> {
        let Self {
            term,
            objects,
            windows,
            ..
        } = self;
        let window = &windows[0];
        window.blit(windows, objects, term, 0, 0)?;
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
    // Ensure we always have our Session Window
    assert!(session.windows.len() == 1);
    Ok(())
}
