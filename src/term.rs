use crate::error::OmmaErr;
use std::io::{self, IsTerminal, Write};
use std::os::fd::AsRawFd;

#[cfg(unix)]
pub fn terminfo() -> Result<(u16, u16), OmmaErr> {
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
    Ok((ws.ws_row, ws.ws_col))
}

#[cfg(not(unix))]
pub fn terminfo() -> Result<Terminfo, Err> {
    return Err(OmmaErr::new(
        "terminfo is only compatible with *nix currently",
    ));
}

pub struct OmmaTerm {
    row: u16,
    col: u16,
    max_row: u16,
    max_col: u16,
    stdout: io::Stdout,
}

impl std::fmt::Display for OmmaTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} by {}", self.max_row, self.max_col)
    }
}

impl OmmaTerm {
    pub fn new() -> Result<Self, OmmaErr> {
        let (max_row, max_col) = terminfo()?;
        Ok(OmmaTerm {
            row: 0,
            col: 0,
            max_row,
            max_col,
            stdout: io::stdout(),
        })
    }

    pub fn move_cursor(&mut self, row: u16, col: u16) -> Result<(), OmmaErr> {
        if row > self.max_row || col > self.max_col {
            return Err(OmmaErr::new(&format!(
                "Invalid cursor move {}:{} (max {}:{})",
                row, col, self.max_row, self.max_col
            )));
        }
        self.row = row;
        self.col = col;
        print!("\x1b[{};{}H", row, col);
        Ok(())
    }

    pub fn put_char_at(&mut self, row: u16, col: u16, ch: char) -> Result<(), OmmaErr> {
        self.move_cursor(row, col)?;
        write!(self.stdout, "{ch}")?;
        self.stdout.flush()?;
        Ok(())
    }
}
