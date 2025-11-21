use crate::err::OmmaErr;
use std::io::{self, IsTerminal};
use std::os::fd::AsRawFd;

pub struct Terminfo {
    rows: u16,
    cols: u16,
}

impl std::fmt::Display for Terminfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} by {}", self.rows, self.cols)
    }
}

#[cfg(unix)]
pub fn terminfo() -> Result<Terminfo, OmmaErr> {
    let stdout = io::stdout();
    if !stdout.is_terminal() {
        return Err(OmmaErr::new("failed to open stdio, invalid terminal"));
    }
    #[repr(C)]
    struct Winsize {
        ws_row: u16,
        ws_col: u16,
        ws_xpixel: u16,
        ws_ypixel: u16,
    }

    const TIOCGWINSZ: u64 = 0x5413;

    unsafe extern "C" {
        fn ioctl(fd: i32, ioctl: u64, ...) -> i32;
    }

    let fd = stdout.as_raw_fd();
    let mut ws: Winsize = unsafe { std::mem::zeroed() };

    let ret = unsafe { ioctl(fd, TIOCGWINSZ, &mut ws) };
    if ret != 0 {
        return Err(OmmaErr::new(&format!(
            "ioctl(TIOCGWINSZ) failed with {}",
            ret
        )));
    }

    if ws.ws_row == 0 || ws.ws_col == 0 {
        return Err(OmmaErr::new(&format!(
            "zero dimension in terminal {} by {}",
            ws.ws_row, ws.ws_col
        )));
    }
    Ok(Terminfo {
        rows: ws.ws_row,
        cols: ws.ws_col,
    })
}

#[cfg(not(unix))]
pub fn terminfo() -> Result<Terminfo, Err> {
    return Err(OmmaErr::new(
        "terminfo is only compatible with *nix currently",
    ));
}
