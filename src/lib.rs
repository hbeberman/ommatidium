pub mod border;
pub mod cell;
pub mod color;
pub mod error;
pub mod object;
pub mod pad;
pub mod session;
pub mod term;
pub mod window;

use crate::error::OmmaErr;
use std::sync::atomic::{AtomicU32, Ordering};

static WINDOW_ID: AtomicU32 = AtomicU32::new(0);
static OBJECT_ID: AtomicU32 = AtomicU32::new(0);

pub fn next_window_id() -> Result<u32, OmmaErr> {
    let current_id = WINDOW_ID.load(Ordering::Relaxed);
    let id = WINDOW_ID.fetch_add(1, Ordering::Relaxed);
    if current_id > id {
        return Err(OmmaErr::new(
            "ommatidium window_id tracker wrapped u32::MAX. Failed State.",
        ));
    }
    Ok(id)
}

pub fn current_window_id() -> u32 {
    WINDOW_ID.load(Ordering::Relaxed)
}

pub fn next_object_id() -> Result<u32, OmmaErr> {
    let current_id = OBJECT_ID.load(Ordering::Relaxed);
    let id = OBJECT_ID.fetch_add(1, Ordering::Relaxed);
    if current_id > id {
        return Err(OmmaErr::new(
            "ommatidium window_id tracker wrapped u32::MAX. Failed State.",
        ));
    }
    Ok(id)
}

pub fn current_object_id() -> u32 {
    OBJECT_ID.load(Ordering::Relaxed)
}
