pub mod border;
pub mod error;
pub mod ommacell;
pub mod session;
pub mod term;
pub mod window;

use crate::error::OmmaErr;
use std::sync::atomic::{AtomicU32, Ordering};

static ID: AtomicU32 = AtomicU32::new(0);

pub fn next_id() -> Result<u32, OmmaErr> {
    let current_id = ID.load(Ordering::Relaxed);
    let id = ID.fetch_add(1, Ordering::Relaxed);
    if current_id > id {
        return Err(OmmaErr::new(
            "ommatidium id tracker wrapped u32::MAX. Failed State.",
        ));
    }
    Ok(id)
}

pub fn current_id() -> u32 {
    ID.load(Ordering::Relaxed)
}
