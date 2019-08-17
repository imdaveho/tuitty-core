//! Platform specific functions for the library.
use std::io::Write;
use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};
use crate::{csi, write_cout};
use super::{Clear, TtyResult};

mod alternate;
pub use alternate::*;


pub fn _clear(clr: Clear) -> TtyResult<()> {
    match clr {
        Clear::All => {
            write_cout!(csi!("2J"))?;
        }
        Clear::CursorDn => {
            write_cout!(csi!("J"))?;
        }
        Clear::CursorUp => {
            write_cout!(csi!("1J"))?;
        }
        Clear::CurrentLn => {
            write_cout!(csi!("2K"))?;
        }
        Clear::NewLn => {
            write_cout!(csi!("K"))?;
        }
    };
    Ok(())
}

pub fn _size() -> (i16, i16) {
    // (TimonPost) NOTE: from crossterm_terminal/src/sys/unix.rs
    // http://rosettacode.org/wiki/Terminal_control/Dimensions#Library:_BSD_libc
    let mut size = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let r = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ.into(), &mut size) };

    if r == 0 {
        (size.ws_col, size.ws_row)
    } else {
        (0, 0)
    }
}

pub fn _resize(w: i16, h: i16) -> TtyResult<()> {
    write_cout!(&format!(csi!("8;{};{}t"), h, w))?;
    Ok(())
}


// pub fn _scroll_up(n: i16) -> TtyResult<()> {
//     write_cout!(&format!(csi!("{}S"), n))?;
//     Ok(())
// }

// pub fn _scroll_dn(n: i16) -> TtyResult<()> {
//     write_cout!(&format!(csi!("{}T"), n))?;
//     Ok(())
// }
