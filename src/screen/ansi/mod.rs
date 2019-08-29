// ANSI functions for configuring the terminal size and clearing the screen.

use crate::csi;
use super::Clear;

#[cfg(unix)]
use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};

mod alternate;
pub use alternate::{
    enable_alt,
    disable_alt,
};


pub fn clear(clr: Clear) -> String {
    match clr {
        Clear::All => {
            return csi!("2J").to_string()
        }
        Clear::CursorDn => {
            return csi!("J").to_string()
        }
        Clear::CursorUp => {
            return csi!("1J").to_string()
        }
        Clear::CurrentLn => {
            return csi!("2K").to_string()
        }
        Clear::NewLn => {
            return csi!("K").to_string()
        }
    }
}

#[cfg(unix)]
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

pub fn resize(w: i16, h: i16) -> String {
    format!(csi!("8;{};{}t"), h, w).to_string()
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
