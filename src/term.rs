use crate::cell::{EMPTY, OmmaCell};
use crate::error::OmmaErr;
use std::io::{self, IsTerminal, Read, Write};
use std::mem::MaybeUninit;
use std::os::fd::{AsRawFd, RawFd};

type TcflagT = u32;
type CcT = u8;
type SpeedT = u32;

#[repr(C)]
#[derive(Clone, Copy)]
struct Termios {
    c_iflag: TcflagT,
    c_oflag: TcflagT,
    c_cflag: TcflagT,
    c_lflag: TcflagT,
    c_line: CcT,
    c_cc: [CcT; 32],
    c_ispeed: SpeedT,
    c_ospeed: SpeedT,
}

unsafe extern "C" {
    fn isatty(fd: i32) -> i32;
    fn tcgetattr(fd: i32, termios_p: *mut Termios) -> i32;
    fn tcsetattr(fd: i32, optional_actions: i32, termios_p: *const Termios) -> i32;
    fn cfmakeraw(termios_p: *mut Termios);
}

#[allow(dead_code)]
pub struct RawMode {
    fd: RawFd,
    orig: Option<Termios>,
}

impl RawMode {
    /// set_alt_raw sets the terminal to alternative screen buffer and enables raw mode
    pub(crate) fn set_alt_raw() -> Result<Self, OmmaErr> {
        let stdin = io::stdin();
        let fd = stdin.as_raw_fd();
        unsafe {
            if isatty(fd) == 0 {
                return Err(OmmaErr::new(&format!("stdin fd {} is not a terminal", fd)));
            }

            let mut tio = MaybeUninit::<Termios>::uninit();
            if tcgetattr(fd, tio.as_mut_ptr()) != 0 {
                return Err(OmmaErr::new(&format!(
                    "stdin fd {} unable to tcgetattr",
                    fd
                )));
            }
            let mut tio = tio.assume_init();
            let orig = tio;
            cfmakeraw(&mut tio);

            if tcsetattr(fd, 0, &tio as *const Termios) != 0 {
                return Err(OmmaErr::new(&format!(
                    "stdin fd {} unable to set raw mode",
                    fd
                )));
            }

            // send smcup escape to enter alternative screen buffer.
            print!("\x1b[?1049h");

            Ok(RawMode {
                fd,
                orig: Some(orig),
            })
        }
    }

    #[allow(dead_code)]
    /// noop is used tests to avoid touching the real TTY.
    pub(crate) fn noop() -> Self {
        RawMode { fd: -1, orig: None }
    }
}

impl Drop for RawMode {
    /// drop restores the primary terminal history and disables raw mode
    fn drop(&mut self) {
        if let Some(orig) = &self.orig {
            unsafe {
                // send rmcup escape to return to normal screen buffer.
                print!("\x1b[?1049l");
                let _ = tcsetattr(self.fd, 0, orig as *const Termios);
            }
        }
    }
}

#[cfg(unix)]
/// terminfo reuns the height and width of the terminal using the IOCGWINSZ syscall
pub(crate) fn terminfo() -> Result<(u16, u16), OmmaErr> {
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

#[allow(dead_code)]
pub struct OmmaTerm {
    row: u16,
    col: u16,
    max_row: u16,
    max_col: u16,
    stdout: io::Stdout,
    raw: RawMode,
    front: Vec<Vec<OmmaCell>>,
    back: Vec<Vec<OmmaCell>>,
}

impl std::fmt::Display for OmmaTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} by {}", self.max_row, self.max_col)
    }
}

impl Drop for OmmaTerm {
    fn drop(&mut self) {
        let _ = write!(self.stdout, "\x1b[?25h");
        let _ = self.stdout.flush();
    }
}

impl OmmaTerm {
    pub fn new() -> Result<Self, OmmaErr> {
        let (max_row, max_col) = terminfo()?;
        let raw = RawMode::set_alt_raw()?;
        let front = vec![vec![OmmaCell::default(); max_row as usize]; max_col as usize];
        let back = vec![vec![OmmaCell::default(); max_row as usize]; max_col as usize];
        let mut stdout = io::stdout();
        eprintln!("Terminal dimensions ({max_row}:{max_col})");
        write!(stdout, "\x1b[?25l")?;
        stdout.flush()?;
        Ok(OmmaTerm {
            row: 0,
            col: 0,
            max_row,
            max_col,
            stdout,
            raw,
            front,
            back,
        })
    }

    #[allow(dead_code)]
    /// new_mock initializes a new term object without an actual backing terminal
    pub fn new_mock(max_row: u16, max_col: u16) -> Result<Self, OmmaErr> {
        let raw = RawMode::noop();
        let front = vec![vec![OmmaCell::default(); max_row as usize]; max_col as usize];
        let back = vec![vec![OmmaCell::default(); max_row as usize]; max_col as usize];
        Ok(OmmaTerm {
            row: 0,
            col: 0,
            max_row,
            max_col,
            stdout: io::stdout(),
            raw,
            front,
            back,
        })
    }

    /// move_cursor sets the terminal cursor to a target location
    pub(crate) fn move_cursor(&mut self, col: u16, row: u16) -> Result<(), OmmaErr> {
        if row >= self.max_row || col >= self.max_col {
            return Err(OmmaErr::new(&format!(
                "invalid cursor move {}:{} (max {}:{})",
                row,
                col,
                self.max_row - 1,
                self.max_col - 1
            )));
        }
        self.row = row;
        self.col = col;
        // ANSI escape codes are 1-based; our buffers are 0-based.
        print!("\x1b[{};{}H", row + 1, col + 1);
        Ok(())
    }

    /// put_cell_at sets the cell at a location to be the ommacell argument
    pub(crate) fn put_cell_at(
        &mut self,
        x: usize,
        y: usize,
        cell: &OmmaCell,
    ) -> Result<u32, OmmaErr> {
        let x_max = self.back.len();
        let y_max = self.back[0].len();
        if x >= x_max || y >= y_max {
            return Err(OmmaErr::new(&format!(
                "invalid put cell at {}:{} (max {}:{})",
                x, y, x_max, y_max,
            )));
        }
        match cell.ch {
            EMPTY => Ok(0),
            _ => {
                self.back[x][y] = cell.clone();
                Ok(1)
            }
        }
    }

    /// render_cell_at writes a cell to a location at the ommaterm saved stdout handle
    pub(crate) fn render_cell_at(
        &mut self,
        x: u16,
        y: u16,
        cell: &OmmaCell,
    ) -> Result<(), OmmaErr> {
        self.move_cursor(x, y)?;
        write!(self.stdout, "{}{}{}", cell.fgcode(), cell.bgcode(), cell.ch)?;
        Ok(())
    }

    /// render writes the entire back plane terminal to the screen
    pub(crate) fn render(&mut self) -> Result<u32, OmmaErr> {
        let mut written = 0;
        for x in 0..self.max_col {
            for y in 0..self.max_row {
                written += 1;
                let cell = self.back[x as usize][y as usize].clone();
                self.render_cell_at(x, y, &cell)?;
            }
        }
        Ok(written)
    }

    /// read_key reads a single keypress and returns the char
    pub(crate) fn read_key(&mut self) -> Result<Option<char>, OmmaErr> {
        let mut buf = [0u8; 1];
        let n = io::stdin().read(&mut buf)?;
        if n == 0 {
            Ok(None)
        } else {
            Ok(Some(buf[0] as char))
        }
    }
}
