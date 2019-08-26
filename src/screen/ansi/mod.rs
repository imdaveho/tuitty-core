// ANSI functions for configuring the terminal size and clearing the screen.

use std::io::{Result, Write};
use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};
use crate::{csi, write_cout};
use super::Clear;

mod alternate;
pub use alternate::{
    enable_alt,
    disable_alt,
};


pub fn clear(clr: Clear) -> Result<()> {
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

pub fn size() -> (i16, i16) {
    // Reference source:
    // http://rosettacode.org/wiki/Terminal_control/Dimensions#Library:_BSD_libc
    let mut size = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let r = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ.into(), &mut size) };

    if r == 0 {
        (size.ws_col as i16, size.ws_row as i16)
    } else {
        (0, 0)
    }
}

pub fn resize(w: i16, h: i16) -> Result<()> {
    write_cout!(&format!(csi!("8;{};{}t"), h, w))?;
    Ok(())
}


// NOTE: Native ANSI scroll functions commented out because they do not behave
// as modern implementations of scrolling should. Essentially the scroll also
// clears sections of the screen.

// pub fn _scroll_up(n: i16) -> Result<()> {
//     write_cout!(&format!(csi!("{}S"), n))?;
//     Ok(())
// }

// pub fn _scroll_dn(n: i16) -> Result<()> {
//     write_cout!(&format!(csi!("{}T"), n))?;
//     Ok(())
// }
